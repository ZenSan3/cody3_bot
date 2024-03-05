use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
use serenity::prelude::*;

pub async fn pinger(ctx:&Context, msg:&Message){
    let response = MessageBuilder::new()
        .push("Pong!")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await{
        println!("Error sending message: {why:?}");
    }

}