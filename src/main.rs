use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;

mod queslar;
use queslar::Character;

mod catacomb;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    api: String,
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let settings = load_settings_from_file();
    let player_data = fetch_api(settings.api).await?;
    let player: Character = serde_json::from_str(&player_data).unwrap();
    let cata_params = read_from_clipboard().unwrap();
    let mob_debuff = 2.4;
    let monster = catacomb::create_monster(100.0, cata_params, 2.0, mob_debuff);
    println!("{:#?}", monster);
    Ok(())    
}

pub fn load_settings_from_file() -> Settings {
    let file = File::open("./settings.json").expect("Should be able to read file.");
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect("Should be able to parse settings.");

    return settings;
}

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