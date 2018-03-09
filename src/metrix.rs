extern crate reqwest;

static METRIX_API: &'static str = "https://discgolfmetrix.com/api.php?content=bagtag_list&id=6";

#[derive(Deserialize, Debug)]
pub struct Players {
  players: Vec<Player>,
  #[serde(rename = "Errors")]
  errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  #[serde(rename = "Number")]
  number: String,
  #[serde(rename = "Name")]
  name: Option<String>,
  #[serde(rename = "Rating")]
  rating: Option<String>,
  #[serde(rename = "LastNumber")]
  last_number: Option<String>,
}

pub fn get_player_data() -> reqwest::Result<Vec<Player>> {
  let mut response = reqwest::get(METRIX_API)?;
  let players: Players = response.json()?;
  Ok(players.players)
}