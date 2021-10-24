use std::collections::HashMap;
use super::xkcd::XkcdComic;

pub struct DiscordWebhook {
    url: String,
    client: reqwest::blocking::Client,
}

impl DiscordWebhook {
    pub fn new(url: &String) -> DiscordWebhook {
        DiscordWebhook { 
            url: String::from(url),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn post_xkcd_comic(&self, comic: &XkcdComic) -> Result<(), Box<dyn std::error::Error>> {
        //use reqwest::blocking::multipart;

        let mut message = HashMap::new();
        message.insert(String::from("content"), format_message(&comic.title, &comic.img_url, &comic.hovertext));

        /*let img_bytes = self.client.get(&comic.img_url).send()?.bytes()?;
        let multipart = multipart::Form::new()
            .part("file", multipart::Part::bytes(&img_bytes)?);*/

        self.client.post(&self.url)
            .json(&message)
            //.multipart(multipart)
            .send()?;

        Ok(())
    }
}

fn format_message (title: &String, img_url: &String, hovertext: &String) -> String {
    format!("**{}**\n{}\n||{}||", title, img_url, hovertext)
}