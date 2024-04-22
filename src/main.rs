mod commands;

use std::env;

use std::io::{ stdout, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use commands::channel::*;
use commands::err::errore;
use commands::ping::*;
use commands::help::*;
use commands::ratatui::ui;
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

    async fn ready(&self, _: Context, ready:Ready){
        let _ = create_terminal();
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

pub fn create_terminal()->Result<()>{
    
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}