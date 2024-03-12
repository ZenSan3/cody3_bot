use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use serenity::model::channel::Message;
use serenity::prelude::*;

use super::err::ferror;

pub async fn opfile(path:String, ctx:&Context, msg:&Message)->Result<Value, String>{

    if let Ok(mut file) = File::open(path){
        let mut contents = String::new();
        
        if let Err(_err) = file.read_to_string(&mut contents){
            ferror(ctx, msg).await;
        }

        if let Ok(data) = serde_json::from_str(&contents){
            return Ok(data);
        }else{
            return Err("".to_owned());
        }
    }else{
        return Err("".to_owned());
    }
}
    