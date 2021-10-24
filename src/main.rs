use std::collections::HashMap;
//use std::io;

mod xkcd;

fn main() {
    post_latest_comic().unwrap();
}

fn send_message(message: &HashMap<String, String>, webhook_url: &str) {
    let client = reqwest::blocking::Client::new();
    let res = client.post(webhook_url)
        .json(&message);
    
    res.send().expect("Failed to send mesaage");
}

fn post_latest_comic() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://xkcd.com/rss.xml")?;
    let resp_text = resp.text()?;
    let resp_bytes = resp_text.as_bytes();
    let rss_channel = rss::Channel::read_from(resp_bytes)?;

    let item = match rss_channel.items.get(0) {
        Some(i) => i,
        None => {panic!("No item");}
    };

    let title = match &item.title {
        Some(s) => s,
        None => {panic!("No title")}
    };

    let description = match &item.description {
        Some(s) => s,
        None => {panic!("No title")}
    };
    let parsed_description = parse_description(description);

    let mut message = HashMap::new();
    message.insert(String::from("content"), format_message(&title, &parsed_description.0, &parsed_description.1));

    send_message(&message, "https://discord.com/api/webhooks/894361486141583390/eCmb2isBsGokLoeizzZI3AKDlkPSxdI3Bfb_Asf6b5CuTVcIsKygCGvpWkLIx-PzQR9D");

    Ok(())
}

fn parse_description(description: &String) -> (String, String) {
    use regex::Regex;
    let regex = Regex::new(r#"<img src="(https://[\w\W]*)"\s*?title="([\w\W]*)"\s*?alt="([\w\W]*)"\s*?/>"#).unwrap();

    let captures = regex.captures(description).unwrap();

    let img_url = String::from(captures.get(1).unwrap().as_str());
    let hovertext = String::from(captures.get(2).unwrap().as_str());
    (img_url, hovertext)
}

fn format_message (title: &String, img_url: &String, hovertext: &String) -> String {
    format!("{}\n{}\n||{}||", title, img_url, hovertext)
}