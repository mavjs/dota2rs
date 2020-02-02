#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;

use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Players {
    rank: i64,
    name: String,

    // the below elements may not always be present when "deserializing" json
    // elements, therefore, tell "serde" to skip that.
    #[serde(skip_deserializing)]
    team_id: i64,
    #[serde(skip_deserializing)]
    team_tag: String,
    #[serde(skip_deserializing)]
    country: String,
    #[serde(skip_deserializing)]
    sponsor: String,
}

#[derive(Deserialize, Debug)]
struct API {
    time_posted: i64,
    next_scheduled_post_time: i64,
    server_time: i64,
    leaderboard: Vec<Players>,
}

const URL: &str = "https://www.dota2.com/webapi/ILeaderboard/GetDivisionLeaderboard/v0001";

fn main() -> Result<(), Error> {
    let europe = format!("{}?division={}", URL, "europe");
    //println!("{}", europe);
    let mut resp = reqwest::get(&europe)?;
    let api: API = resp.json()?;
    println!("{:?}", api);
    Ok(())
}
