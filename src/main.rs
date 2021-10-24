mod xkcd;
use xkcd::XkcdBot;

fn main() {
    let url = String::from("https://discord.com/api/webhooks/894361486141583390/eCmb2isBsGokLoeizzZI3AKDlkPSxdI3Bfb_Asf6b5CuTVcIsKygCGvpWkLIx-PzQR9D");

    let bot = XkcdBot::new().unwrap();
    bot.generate_request(&url).send().unwrap();
}