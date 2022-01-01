use std::collections::HashSet;

use serenity::framework::standard::macros::*;
use serenity::framework::standard::*;
use serenity::model::prelude::*;
use serenity::{async_trait, prelude::*};

use tokio::time::{sleep, Duration};

use crate::commands::*;
use crate::config::*;
use crate::helpers::*;

#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut new_options = help_options.clone();
    new_options.strikethrough_commands_tip_in_dm = None;
    new_options.strikethrough_commands_tip_in_guild = None;

    let _help = help_commands::with_embeds(ctx, msg, args, &new_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(info)]
/// Get server info
struct Info;
/// Discord handler
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("Using addr: {}:{}", *ADDR, *PORT);
        loop {
            let result = get_status(&ADDR, *PORT).await;
            let act = match result {
                Ok(status) => get_activity(status),
                Err(e) => {
                    println!("Error: {}", e);
                    Activity::playing("with some errors")
                }
            };
            ctx.set_presence(Some(act), OnlineStatus::Online).await;
            sleep(Duration::from_millis(REFRESH_TIME)).await;
        }
    }
}
