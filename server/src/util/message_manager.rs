use actix::AsyncContext;
use futures::Stream;
use anyhow::Result;
use redis::AsyncCommands;
use redis::Msg;
use redis::aio::Connection;

use actix::{Actor, StreamHandler, WrapFuture, Message, Handler};
use futures::StreamExt;
use actix_web_actors::ws;

use crate::util::get_redis_connection;

pub async fn send_message(conn: &mut Connection, user_id: i64, topic: &str, message: &[u8]) -> Result<()> {
    conn.publish(user_id.to_string() + ":" + topic, message).await?;

    Ok(())
}

// all the stuff for websocket
async fn subscribe(user_id: i64, topic: &str) -> Result<impl Stream<Item = Msg>> {
    let conn = get_redis_connection().await?;
    let mut pub_sub = conn.into_pubsub();

    pub_sub.subscribe(user_id.to_string() + ":" + topic).await?;

    Ok(pub_sub.into_on_message())
}

#[derive(Message)]
#[rtype(result="()")]
pub struct PubsubMsg {
    payload: String
}

#[derive(Message)]
#[rtype(result="()")]
pub struct PubsubErr {
    err: anyhow::Error
}

pub struct GenericPassthroughWs {
    user_id: i64,
    pubsub_topic: String
}

impl GenericPassthroughWs {
    pub fn new(user_id: i64, topic: &str) -> Self {
        Self {
            user_id,
            pubsub_topic: topic.to_string()
        }
    }
}

impl Actor for GenericPassthroughWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<PubsubMsg> for GenericPassthroughWs {
    type Result = ();

    fn handle(&mut self, msg: PubsubMsg, ctx: &mut Self::Context) -> Self::Result {
        // pass it through the websocket
        ctx.text(msg.payload);
    }
}

impl Handler<PubsubErr> for GenericPassthroughWs {
    type Result = ();

    fn handle(&mut self, msg: PubsubErr, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(Some((
            ws::CloseCode::Error,
            msg.err.to_string()
        ).into()))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GenericPassthroughWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(close)) => ctx.close(close),
            _ => ()
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        // makes a pubsub listen
        let user_id = self.user_id;
        let topic = self.pubsub_topic.clone();
        let addr = ctx.address();
        let fut = Box::pin(
            
            async move {
                match subscribe(user_id, &topic).await {
                    Ok(mut sub) => {
                        while let Some(msg) = sub.next().await {
                            match msg.get_payload::<String>() {
                                Ok(payload) => {
                                    addr.do_send(PubsubMsg { payload })
                                },
                                Err(err) => addr.do_send(PubsubErr { err: err.into() }),
                            }
                        }
                    },
                    Err(err) => {
                        addr.do_send(PubsubErr { err })
                    },
                }
            }

        );

        ctx.spawn(fut.into_actor(self));
    }    
}