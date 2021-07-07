use serenity::{client::Context, framework::standard::{Args, CommandResult, macros::command}, model::channel::Message};

use crate::helpers::*;
use crate::config::*;

#[command]
#[description("Gets server info from server")]
#[max_args(0)]
async fn info_default(ctx: &Context, msg: &Message) -> CommandResult {
    let status = get_status(&ADDR, *PORT).await?;
    println!("{} of  {}", status.players.online, status.players.max);

    let mut players_info = format!("Server info: {} players online", status.players.online);
    if let Some(players) = status.players.sample {
        let name_vec: Vec<_> = players.into_iter().map(|i| i.name).collect();
        players_info = format!("{}\nPlayers: {:?}", players_info, name_vec);
    }
    msg.channel_id.say(&ctx.http, players_info).await?;
    Ok(())
}

#[command]
#[usage("[address]")]
#[description("Gets server info from server")]
#[num_args(1)]
async fn info_by_address(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get address from user input
    let addr = args.single::<String>();

    if addr.is_err() {
        msg.channel_id.say(&ctx.http, "Address must be a valid address").await?;
        return Ok(());
    }
    // Get status response from server
    let status = get_status(&ADDR, *PORT).await;
    // If successful, send response info to user
    if let Ok(status) = status {
        println!("{} of  {}", status.players.online, status.players.max);

        let mut players_info = format!("Server info: {} players online", status.players.online);
        if let Some(players) = status.players.sample {
            let name_vec: Vec<_> = players.into_iter().map(|i| i.name).collect();
            players_info = format!("{}\nPlayers: {:?}", players_info, name_vec);
        }
        msg.channel_id.say(&ctx.http, players_info).await?;
    }
    // If cannont 
    else if let Err(e) = status {
        msg.channel_id.say(&ctx.http, format!("Error getting server info: {}", e)).await?;
    }
    
    Ok(())
}

#[command]
#[usage("[address] [port]")]
#[description("Gets server info from server")]
#[max_args(2)]
async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let addr = args.single::<String>().unwrap_or(ADDR.to_string());
    let port = args.single::<u16>().unwrap_or(*PORT);
    
    // Get status response from server
    let res = get_status(&addr, port).await;
    // If successful, send response info to user
    if let Ok(status) = res {
        println!("{} of  {}", status.players.online, status.players.max);

        let mut players_info = format!("Server info \nAddress:{}\nPort{}\n{} players online", addr, port, status.players.online);
        if let Some(players) = status.players.sample {
            let name_vec: Vec<_> = players.into_iter().map(|i| i.name).collect();
            players_info = format!("{}\nPlayers: {:?}", players_info, name_vec);
        }
        msg.channel_id.say(&ctx.http, players_info).await?; 
    }
    // If cannont 
    else if let Err(e) = res {
        msg.channel_id.say(&ctx.http, format!("Error getting server info: {}", e)).await?;
    }
    Ok(())
}
