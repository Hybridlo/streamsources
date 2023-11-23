use actix::AsyncContext;
use futures::Stream;
use anyhow::Result;
use futures_util::future::join_all;
use redis::Msg;

use actix::{Actor, StreamHandler, WrapFuture, Message, Handler};
use futures::StreamExt;
use actix_web_actors::ws;

use crate::{util::get_redis_connection, db::Repository, domain::subscription::Subscription};

// all the stuff for websocket
async fn subscribe(user_id: i64, topic: &str) -> Result<impl Stream<Item = Msg>> {
    let conn = get_redis_connection().await?;
    let mut pub_sub = conn.into_pubsub();

    pub_sub.subscribe(user_id.to_string() + ":" + topic).await?;

    Ok(pub_sub.into_on_message())
}

async fn pre_start_ws(db: Repository, sub_ids: Vec<String>) {
    join_all(sub_ids.iter().map(|sub_id| Subscription::update_connect_time_by_id(&db, sub_id))).await;
}

async fn pre_end_ws(db: Repository, sub_ids: Vec<String>) {
    join_all(sub_ids.iter().map(|sub_id| Subscription::update_disconnect_time_by_id(&db, sub_id))).await;
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
    pubsub_topic: String,
    sub_ids: Vec<String>,
    db: Repository
}

impl GenericPassthroughWs {
    pub fn new(user_id: i64, topic: &str, sub_ids: Vec<String>, db: Repository) -> Self {
        Self {
            user_id,
            pubsub_topic: topic.to_string(),
            sub_ids,
            db
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
        ctx.wait(pre_end_ws(self.db.clone(), self.sub_ids.clone()).into_actor(self));
        
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
            Ok(ws::Message::Close(close)) => {
                ctx.wait(pre_end_ws(self.db.clone(), self.sub_ids.clone()).into_actor(self));
                
                ctx.close(close)
            },
            _ => ()
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        // makes a pubsub listen
        let user_id = self.user_id;
        let topic = self.pubsub_topic.clone();
        let addr = ctx.address();

        let db = self.db.clone();
        let sub_ids = self.sub_ids.clone();

        let fut = Box::pin(
            
            async move {
                pre_start_ws(db, sub_ids).await;

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