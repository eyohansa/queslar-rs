use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};


pub async fn fetch_api(api: String) -> reqwest::Result<String> {
    let url = format!("https://queslar.com/api/player/full/{}", api);
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}

pub fn read_from_clipboard() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let contents = ctx.get_contents()?;

    let mut monster_params = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        monster_params.insert(parts[0].to_string(), parts[1].trim().to_string());
    }
    Ok(monster_params)
}


#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub api: String,
}

pub fn load_settings_from_file() -> Settings {
    let file = File::open("./settings.json").expect("Should be able to read file.");
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect("Should be able to parse settings.");

    return settings;
}