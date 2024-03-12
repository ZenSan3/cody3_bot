use serenity::json::NULL;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::prelude::*;

use super::file::opfile;

pub async fn helpreq(ctx: &Context, msg: &Message){

    let mut campi:Vec<(String, String, bool)> = Vec::new();
    
    let data = match opfile("./json/help.json".to_owned(), ctx, msg).await {
        Ok(dati)=> dati,
        Err(_err)=>return,
    };

    let mut i = 0;
    while !(data[i].is_null()) {
        campi.push(
            (data[i]["name"].to_string().replace('"', ""),
            data[i]["value"].to_string().replace('"', ""),
            data[i]["inline"].as_bool().expect("is not a boolean"))
        );
        i+=1;
    }

    //creo l'embed
    let embed = CreateEmbed::new()
        .title("Help section")
        .description("Sotto sono elencati i comadi presenti al momento")
        .color(0xba21c1)
        .fields(campi);

    //"appendo" il file al 
    let builder = CreateMessage::new()
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await{
        println!("Error sending message: {why:?}");
    }
}