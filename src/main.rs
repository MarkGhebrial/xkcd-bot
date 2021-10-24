mod xkcd;
use xkcd::XkcdComic;

mod discord;
use discord::DiscordWebhook;

fn main() {
    let url = String::from("https://discord.com/api/webhooks/894361486141583390/eCmb2isBsGokLoeizzZI3AKDlkPSxdI3Bfb_Asf6b5CuTVcIsKygCGvpWkLIx-PzQR9D");
    let webhook = DiscordWebhook::new(&url);
    let comic = XkcdComic::get_latest_comic().unwrap();

    webhook.post_xkcd_comic(&comic).unwrap();
    webhook.post_xkcd_comic(&comic).unwrap();
}