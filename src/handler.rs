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
            match result {
                Ok(status) => {
                    let act = Activity::playing(
                        format!("Minecraft w/{} players", status.players.online).as_str(),
                    );
                    ctx.set_presence(Some(act), OnlineStatus::Online).await;
                }
                Err(e) => println!("Error getting status: {}", e.to_string()),
            }

            delay_for(Duration::from_millis(REFRESH_TIME)).await;
        }
    }
}
