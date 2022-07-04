#[cfg(not(target_os = "windows"))]
compile_error!("This crate doesn't support any other target operating systems than Windows");

#[macro_use] extern crate litcrypt;
#[macro_use] extern crate lazy_static;

mod handler;

use handler::{Handler, options::TOKEN};
use serenity::prelude::*;
use crate::handler::utils::{ENGINE, UUID};

use_litcrypt!();

pub async fn main() {
    init();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(TOKEN.as_ref(), intents)
            .event_handler(Handler)
            .await
            .unwrap();

    let _ = client.start().await;
}

fn init() {
    let _ = &ENGINE;
    let _ = &UUID;
}
