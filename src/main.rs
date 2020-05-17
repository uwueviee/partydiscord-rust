use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot == true { return };

        /**
        Only allow certain commands to run if the author has MANAGE_EMOJIS
        There should def be a better way of doing this, can't find it in the documentation.
        Maybe a macro somewhere? I'm not sure.
        This is based off of the official Discord API documentation so it *should* be fine
        FYI 0x40000000 is MANAGE_EMOJIS on Discord's API (That's why I'm looking for it)
        **/
        if  (msg.member(&ctx.cache).unwrap().permissions(&ctx.cache)
                .expect("permissions").bits & 0x40000000) == 0x40000000 {

                    // Upload default emoji pack
                    if msg.content == "+start" {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "<a:ultrafastparrot:405266489218826241> Uploading the emojis! <a:ultrafastparrot:405266489218826241>") {
                            println!("Error sending message: {:?}", why);
                        }
                    }

                    // Upload chosen emojis
                    if msg.content == "+choose" {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "<a:ultrafastparrot:405266489218826241> Uploading the emojis! <a:ultrafastparrot:405266489218826241>") {
                            println!("Error sending message: {:?}", why);
                        }
                    }

                    // Show available emojis
                    if msg.content == "+showemojis" {
                        // Dynamically load all filenames in /emoji/ and print them here
                        if let Err(why) = msg.channel_id.say(&ctx.http, "") {
                            println!("Error sending message: {:?}", why);
                        }
                    }
        }

        if msg.content == "+ping" {
            // To be removed later, just for testing shards
            println!("Shard {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} shard {} is connected!", ready.user.name, ctx.shard_id);
    }
}


fn main() {
    // Download the new parrots *BEFORE* the bot or API starts

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start_shards(12) {
        println!("Client error: {:?}", why);
    }
}