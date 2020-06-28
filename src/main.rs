use std::env;
use std::path::Path;
use std::time::Duration;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tokio::time::delay_for;
use warp::Filter;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot == true { return; };

        /*
        Only allow certain commands to run if the author has MANAGE_EMOJIS
        There should def be a better way of doing this, can't find it in the documentation.
        Maybe a macro somewhere? I'm not sure.
        This is based off of the official Discord API documentation so it *should* be fine
        FYI 0x40000000 is MANAGE_EMOJIS on Discord's API (That's why I'm looking for it)
        */
        if (msg.member(&ctx.cache).await.unwrap().permissions(&ctx.cache).await
            .expect("permissions").bits & 0x40000000) == 0x40000000 {

            // Upload default emoji pack
            if msg.content == "+start" {
                // Default emoji list.
                let emoji_list = vec!["partyparrot", "middleparrot", "reverseparrot",
                                      "congaparrot", "shuffleparrot", "fastparrot", "ultrafastparrot",
                                      "christmasparrot", "wave1parrot", "wave2parrot", "wave3parrot",
                                      "wave4parrot", "wave5parrot", "wave6parrot", "wave7parrot",
                                      "confusedparrot", "dealwithitparrot", "gothparrot", "loveparrot",
                                      "explodyparrot", "boredparrot", "coffeeparrot", "fidgetparrot",
                                      "hamburgerparrot", "luckyparrot", "matrixparrot", "discoparrot",
                                      "angryparrot", "aussiecongaparrot", "aussieparrot", "aussiereversecongaparrot",
                                      "dadparrot", "rotatingparrot", "sadparrot", "stableparrot",
                                      "sleepingparrot", "covid19parrot", "sassyparrot", "slowparrot", "hmmparrot"];

                if let Err(why) = msg.channel_id.say(&ctx.http, "<a:ultrafastparrot:405266489218826241> Uploading the emojis! <a:ultrafastparrot:405266489218826241>").await {
                    println!("Error sending message: {:?}", why);
                }

                let mut upload_errors = 0;

                for emoji in emoji_list {
                    if Path::new(&format!("parrots/hd/{}.gif", emoji)).exists() {
                        if let Err(why) = msg.guild_id.unwrap().create_emoji(&ctx.http, emoji, &serenity::utils::read_image(format!("parrots/hd/{}.gif", emoji)).unwrap()).await {
                            println!("Error uploading emoji: {:?}", why);
                        };
                    } else if Path::new(&format!("parrots/{}.gif", emoji)).exists() {
                        if let Err(why) = msg.guild_id.unwrap().create_emoji(&ctx.http, emoji, &serenity::utils::read_image(format!("parrots/{}.gif", emoji)).unwrap()).await {
                            println!("Error uploading emoji: {:?}", why);
                        };
                    } else {
                        upload_errors += 1;
                        println!("Parrot \"{}\" does not exist.", emoji);
                        if let Err(why) = msg.channel_id.say(&ctx.http, format!("Parrot \"{}\" does not exist.", emoji)).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }

                println!("{} errors detected", upload_errors);

                if let Err(why) = msg.channel_id.say(&ctx.http, "<a:ultrafastparrot:405266489218826241> Finished! <a:ultrafastparrot:405266489218826241>").await {
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