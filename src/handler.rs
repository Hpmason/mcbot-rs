use std::collections::HashSet;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::UserId,
        user::OnlineStatus,
    },
};

use anyhow::Result;

use async_minecraft_ping::{ConnectionConfig, StatusResponse};

use tokio::time::{delay_for, Duration};


use crate::config::*;

async fn get_status(server_addr: &str, port: u16) -> Result<StatusResponse> {
    let mut config = ConnectionConfig::build(server_addr.to_string());
    config = config.with_port(port);

    let mut connection = config.connect().await?;
    Ok(connection.status().await?)
}

#[command]
async fn mc_info(ctx: &Context, msg: &Message) -> CommandResult {
    let status = get_status(&ADDR, *PORT).await?;

    println!("{} of  {}", status.players.online, status.players.max);

    let mut players_info = format!("Server info: {} players online", status.players.online);
    if let Some(players) = status.players.sample {
        let name_vec: Vec<_> = players.into_iter().map(|i| i.name).collect();
        players_info = format!("{}\nPlayers: {:?}", players_info, name_vec)
    }
    msg.channel_id.say(&ctx.http, players_info).await?;
    Ok(())
}

#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
fn get_activity(status: StatusResponse) -> Activity {
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

#[group]
#[commands(mc_info)]
struct General;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        loop {
            let result = get_status(&ADDR, *PORT).await;
            let act = match result {
                Ok(status) => get_activity(status),
                Err(_e) => Activity::playing("with some errors"),
            };
            ctx.set_presence(Some(act), OnlineStatus::Online).await;
            delay_for(Duration::from_millis(REFRESH_TIME)).await;
        }
    }
}
