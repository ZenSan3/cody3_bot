use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::prelude::*;

use super::err::errore;

pub async fn version(ctx: &Context, msg: &Message, game: &str){
    
    let mut campo: Vec<(String, String, bool)> = Vec::new();
    let mut file = File::open("./json/version.json").expect(&errore(1, ctx, msg).await);
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&errore(1, ctx, msg).await);

    let data: Value = serde_json::from_str(&contents).expect(&errore(1, ctx, msg).await);
    
    match game {
        "baffo"=> baffo(data, &mut campo).await,
        _ => println!("{}", errore(5, ctx, msg).await),
    }

    let embed = CreateEmbed::new()
        .title("Version")
        .description(
            "Information of the game requested by the command @cmd".replace("@cmd", &msg.content)
        )
        .color(0xba21c1)
        .fields(campo);

    let builder = CreateMessage::new()
        .embed(embed);
    
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await{
        println!("Error sending message: {why:?}");
    }

}

pub (self) async fn baffo(data: Value, campo: &mut Vec<(String, String, bool)>){

    let titoli:[&str; 3] = ["Title", "Version", "Released"];
    let mut i = 0;
    while !(data["baffo"][i].is_null()) {
        campo.push((titoli[i].to_owned().replace('"', ""), data["baffo"][i].to_string(), false));
        i+=1;
    }
}