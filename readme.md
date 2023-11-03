# Stream sources

## Prerequisites

Running this in debug requires ngrok (or any other proxy that allows for HTTPS tunneling, but then you'll need to adjust the ngrok module in `server/src/twitch_api/subscribe.rs`)

The application uses a PostgreSQL as a database and Redis for PubSub

Server to client communication is managed thanks to [paperclip](https://github.com/paperclip-rs/paperclip) and [openapi generator](https://github.com/OpenAPITools/openapi-generator-cli) on client (needs to be installed)

Needs rust wasm compilation target installed

Also requires [diesel_cli](https://crates.io/crates/diesel_cli), [cargo watch](https://crates.io/crates/cargo-watch) and [trunk](https://crates.io/crates/trunk)

And, of course, you'll need a third party application created on https://dev.twitch.tv

## Setup

Setup DB and Redis access, then fill the `.env` file with these key-value pairs:

DATABASE_URL=&lt;your DB connection string&gt;  
REDIS_URL=&lt;your Redis connection string&gt;  
SECRET=&lt;Random string to be used for signing session tokens&gt;  
TWITCH_KEY=&lt;Client ID of your twitch application&gt;  
TWITCH_SECRET=&lt;Secret of your twitch application&gt;

Run `diesel migration run` to create your DB tables, then launch the tunnel with `ngrok http 80` (or whatever port you decide to use in server/src/main.rs).

Now you can run the start.bat (or you can create your own .sh if you need). That immediately will test if your setup is working - a request is sent to create a subscription of users access revocation (in server/src/main.rs before server is built). If the server processes the subscription fine - we're good!