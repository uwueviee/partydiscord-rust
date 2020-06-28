use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tokio::time::delay_for;
use std::time::Duration;

use warp::Filter;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot == true { return };

        /*
        Only allow certain commands to run if the author has MANAGE_EMOJIS
        There should def be a better way of doing this, can't find it in the documentation.
        Maybe a macro somewhere? I'm not sure.
        This is based off of the official Discord API documentation so it *should* be fine
        FYI 0x40000000 is MANAGE_EMOJIS on Discord's API (That's why I'm looking for it)
        */
        if  (msg.member(&ctx.cache).await.unwrap().permissions(&ctx.cache).await
                .expect("permissions").bits & 0x40000000) == 0x40000000 {

                    // Upload default emoji pack
                    if msg.content == "+start" {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "<a:ultrafastparrot:405266489218826241> Uploading the emojis! <a:ultrafastparrot:405266489218826241>").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }

                    // Upload chosen emojis
                    if msg.content == "+choose" {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Coming Soon!").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }

                    // Show available emojis
                    if msg.content == "+showemojis" {
                        // Dynamically load all filenames in /emoji/ and print them here
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Coming Soon!").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
        }

        if msg.content == "+ping" {
            // To be removed later, just for testing shards
            println!("Shard {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            println!(
                "{} is connected on shard {}/{}!",
                ready.user.name,
                shard[0] + 1,
                shard[1],
            );
        }
    }
}

#[tokio::main]
async fn main() {
    // Download the new parrots *BEFORE* the bot or API starts
    download_parrots().await;
    // Start the API and Discord bot
    futures::join!(start_api(), start_bot());
}

async fn download_parrots() {
    println!("DOWNLOADING PARROTS");

    println!("DOWNLOADED PARROTS");
}

async fn start_api() {
    println!("API STARTING");
    let status = warp::path!("status")
        .map(|| "OK");

    warp::serve(status)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn start_bot() {
    println!("STARTING DISCORD BOT");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let shards = env::var("SHARDS")
        .expect("Expected a shard amount in the environment");

    let mut client = Client::new(&token).event_handler(Handler).await.expect("Err creating client");

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(30)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id,
                    runner.stage,
                    runner.latency,
                );
            }
        }
    });

    if let Err(why) = client.start_shards(shards.parse().unwrap()).await {
        println!("Client error: {:?}", why);
    }
}