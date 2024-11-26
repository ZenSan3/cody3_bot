mod commands;

use std::env;

use commands::channel::*;
use commands::err::errore;
use commands::ping::*;
use commands::help::*;
use commands::ratatui::refresh_terminal;
use commands::ship::ship_maker;
use commands::ver::version;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, context:Context, msg: Message){
        if msg.content.starts_with('!') {
            if msg.content.starts_with("!version"){
                match msg.content.to_lowercase().as_str(){
                    "!version baffo"=> version(&context, &msg, "baffo").await,
                    _=> println!("{}", errore(3, &context, &msg).await),
                }
            }else{
                match msg.content.as_str(){
                    "!help" => helpreq(&context, &msg).await,
                    "!ping" => pinger(&context, &msg).await,
                    "!channel" => channelmsg(&context, &msg).await,
                    "!ship" => ship_maker(&context, &msg).await,
                    _ => println!("{}", errore(0, &context, &msg).await),
                }
            }
        }
    }

    async fn ready(&self, _: Context, _ready:Ready){
        let _ = refresh_terminal();
    }
}

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    
    //creo il token per connettermi al bot
    let token = env::var("DISCORD_TOKEN").expect("Token");
    
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");
    

    if let Err(why) = client.start().await{
        println!("Client error: {why:?}");
    }
}