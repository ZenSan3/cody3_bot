use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
use serenity::prelude::*;

use crate::commands::err::errore;

pub async fn channelmsg(ctx: &Context, msg: &Message){
    let channel = match msg.channel_id.to_channel(&ctx).await{
        Ok(channel)=>channel,
        Err(why)=>{
            println!("Error getting channel: {why:?}");
            println!("{}", errore(2, ctx, msg).await);

            return;
        },
    };

    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" used the 'ping' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await{
        println!("Error sending message: {why:?}");
    }
}