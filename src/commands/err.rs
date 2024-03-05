use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::prelude::*;

pub async fn errore(error:usize, ctx:&Context, msg:&Message)->String{
    
    let mut file = File::open("./json/errors.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: Value = serde_json::from_str(&contents).unwrap();

    let embed = CreateEmbed::new()
        .color(0xff0000)
        .title("Error")
        .description("The follow error has occured:")
        .field(
            data[error]["name"].to_string().replace('"', ""),
            data[error]["value"].to_string().replace('"', "").replace("@cmd", &msg.content),
            false
        );

    let builder = CreateMessage::new()
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await{
        return ("Error sending message: ").to_owned() + &why.to_string();

    }

    return " ".to_owned();

}