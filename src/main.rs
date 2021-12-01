mod xkcd;
mod discord;

use xkcd::*;
use discord::*;
use std::env;

fn main() {
    let url = env::var("DISCORD_WEBHOOK_URL")
        .expect("Please set the url for the webhook in the DISCORD_WEBHOOK_URL enviroment variable");

    let webhook = DiscordWebhook::new(&url);
    let comic = XkcdComic::get_latest_comic().unwrap();

    webhook.post_message(&comic).unwrap();
}