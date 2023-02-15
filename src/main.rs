use reqwest::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

mod queslar;
use queslar::Character;

mod catacomb;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    api: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = load_settings_from_file();
    let player_data = fetch_api(settings.api).await?;
    let player: Character = serde_json::from_str(&player_data).unwrap();
    println!("{:?}", player);
    let monster = catacomb::create_monster(100.0, 1.32, 1.30, 0.03);
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

pub async fn fetch_api(api: String) -> Result<String> {
    let url = format!("https://queslar.com/api/player/full/{}", api);
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}
