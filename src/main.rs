#[macro_use]
extern crate serde_derive;
extern crate serde;

use clap::{Parser, ArgEnum, Subcommand};

const URL: &str = "https://www.dota2.com/webapi/ILeaderboard/GetDivisionLeaderboard/v0001";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Leaderboard {
        /// Division to get statistics about
        #[clap(arg_enum, value_parser)]
        division: Division,

        /// To show all player information from the leaderboard statistics
        #[clap(short, long, action)]
        showall: bool,
    },
}

#[derive(Parser, Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Division {
    Europe,
    Asia,
    China,
    Americas
}

#[derive(Deserialize, Debug)]
pub struct DotaApi {
    pub time_posted: Option<u64>,
    pub next_scheduled_post_time: Option<u64>,
    pub server_time: Option<u64>,
    pub leaderboard: Vec<Leaderboard>,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd,Clone)]
pub struct Leaderboard {
    pub rank: Option<u64>,
    pub name: Option<String>,
    pub team_id: Option<u64>,
    pub team_tag: Option<String>,
    pub country: Option<String>,
    pub sponsor: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Leaderboard { division, showall } => {
            match division {
                Division::Asia => {
                    doreq("se_asia", *showall).await?;
                }
                Division::Europe => {
                    doreq("europe", *showall).await?;
                }
                Division::China => {
                    doreq("china", *showall).await?;
                }
                Division::Americas => {
                    doreq("americas", *showall).await?;
                }
            }

        }
    }
    Ok(())
}

async fn doreq(divisionargs: &str, show_leader_board: bool) -> Result<(), Box<dyn std::error::Error>> {

    let mut url = reqwest::Url::parse(URL).unwrap();

    url.query_pairs_mut()
    .clear()
    .append_pair("division", divisionargs)
    .append_pair("leaderboard", "0");

    let resp = reqwest::get(url)
                .await?
                .json::<DotaApi>()
                .await?;

    println!("Time posted: {:#?}", resp.time_posted.unwrap());
    println!("Next schedule: {:#?}", resp.next_scheduled_post_time.unwrap());
    println!("Server time: {:#?}", resp.server_time.unwrap());
    println!("Total players: {:#?}", resp.leaderboard.len());
    if show_leader_board {
        println!("Leaderboard: {:#?}", resp.leaderboard);
    }

    Ok(())
}
