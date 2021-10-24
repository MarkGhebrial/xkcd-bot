struct xkcd_bot {
    title: String,
    img_url: String,
    hovertext: String,
}

impl xkcd_bot {
    pub fn new() -> Result<xkcd_bot, Box<dyn std::error::Error>> {
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
        
        Ok(xkcd_bot {
            title: String::from(title),
            img_url: parsed_description.0,
            hovertext: parsed_description.1,
        })
    }
}

fn parse_description(description: &String) -> (String, String) {
    use regex::Regex;
    let regex = Regex::new(r#"<img src="(https://[\w\W]*)"\s*?title="([\w\W]*)"\s*?alt="([\w\W]*)"\s*?/>"#).unwrap();

    let captures = regex.captures(description).unwrap();

    let img_url = String::from(captures.get(1).unwrap().as_str());
    let hovertext = String::from(captures.get(2).unwrap().as_str());
    (img_url, hovertext)
}