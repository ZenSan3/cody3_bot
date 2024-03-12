use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::prelude::*;

use super::file::opfile;

pub async fn errore(error:usize, ctx:&Context, msg:&Message)->String{
    
    let data = match opfile("./json/errors.json".to_owned(), ctx, msg).await {
        Ok(dati)=> dati,
        Err(_err)=>return "Errore apertura file".to_owned(),
    };
    

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

pub async fn ferror(ctx:&Context, msg:&Message){
    let embed = CreateEmbed::new()
        .color(0xff0000)
        .title("Error")
        .description("The follow error has occured:")
        .field(
            "ListAccessError",
            "It was not possible to access the list of the requested command @cmd",
            false
        );

    let builder = CreateMessage::new()
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await{
        println!("Error sending message: {why:?}");
    }
}