use reqwest;
mod queslar;
use queslar::Character;

mod catacomb;

use queslar_rs;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let settings = queslar_rs::load_settings_from_file();
    let player_data = queslar_rs::fetch_api(settings.api).await?;
    let player: Character = serde_json::from_str(&player_data).unwrap();
    let cata_params = queslar_rs::read_from_clipboard().unwrap();
    let mob_debuff = 2.4;
    let monster = catacomb::create_monster(100.0, cata_params, 2.0, mob_debuff);
    println!("{:#?}", monster);
    Ok(())    
}