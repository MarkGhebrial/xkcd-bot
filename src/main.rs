mod xkcd;
use xkcd::XkcdComic;

mod discord;
use discord::DiscordWebhook;

fn main() {
    let url = String::from("https://discord.com/api/webhooks/902015970128826408/G2I4nfh6Ztgze2qy7uA9gmIAgovjf8uccDrmSm-8C2yUnlWYlBk0UQBdM1zY12A-n9RJ");
    let webhook = DiscordWebhook::new(&url);
    let comic = XkcdComic::get_latest_comic().unwrap();

    webhook.post_xkcd_comic(&comic).unwrap();
    webhook.post_xkcd_comic(&comic).unwrap();
}