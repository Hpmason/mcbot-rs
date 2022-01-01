use async_minecraft_ping::{ConnectionConfig, ServerError, StatusResponse};
use serenity::model::prelude::Activity;

/// Get status using server_addr and port
pub async fn get_status(server_addr: &str, port: u16) -> Result<StatusResponse, ServerError> {
    let config = ConnectionConfig::build(server_addr.to_string()).with_port(port);

    let conn = config.connect().await?;

    let ping_conn = conn.status().await?;
    Ok(ping_conn.status)
}
/// Get discord activity from StatusResponse
pub fn get_activity(status: StatusResponse) -> Activity {
    // If there is a player list in response
    if let Some(players) = status.players.sample {
        // If no players online
        if players.len() == 0 {
            return Activity::playing("alone with 0 players");
        }
        // If there are 3 or less players, display player names
        if players.len() <= 3 {
            let comma_players: String = players.into_iter().map(|a| a.name + ", ").collect();

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

pub fn generate_message(status: StatusResponse) -> String {
    println!("{} of  {}", status.players.online, status.players.max);

    let mut msg = format!("Server info: {} players online", status.players.online);
    if let Some(players) = status.players.sample {
        let name_vec: Vec<_> = players.into_iter().map(|i| i.name).collect();
        msg = format!("{}\nPlayers: {:?}", msg, name_vec);
    }
    msg
}

pub fn status_or_error_message(res: Result<StatusResponse, ServerError>) -> String {
    match res {
        Ok(status) => generate_message(status),
        Err(e) => format!("Error getting server info: {}", e),
    }
}
