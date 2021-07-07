use async_minecraft_ping::{ConnectionConfig, StatusResponse};
use serenity::model::prelude::Activity;

use anyhow::Result;

pub async fn get_status(server_addr: &str, port: u16) -> Result<StatusResponse> {
    let mut config = ConnectionConfig::build(server_addr.to_string());
    config = config.with_port(port);

    let result = config
        .connect()
        .await?
        .status()
        .await?;
    Ok(result)
}

pub fn get_activity(status: StatusResponse) -> Activity {
    // If there is a player list in response
    if let Some(players) = status.players.sample {
        // If no players online
        if players.len() == 0 {
            return Activity::playing("alone with 0 players");
        }
        // If there are 3 or less players, display player names
        if players.len() <= 3 {
            let comma_players: String = players
                .into_iter()
                .map(|a| a.name + ", ")
                .collect();
            
            let presence = format!("w/ {}", &comma_players);
            return Activity::playing(&presence);
        }
        // If more than 3 players, just display number of players
        else {
            return Activity::playing(&format!("w/ {} players", players.len()));
        }
    }
    // If no players found in response, expect there to be just 0 players
    return Activity::playing("alone with 0 players");
}