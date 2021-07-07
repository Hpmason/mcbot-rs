use std::collections::HashSet;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::UserId,
        user::OnlineStatus,
    },
};

use tokio::time::{sleep, Duration};


use crate::config::*;
use crate::helpers::*;
use crate::commands::*;



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
#[commands(info)]
/// Get server info
struct General;
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
                },
            };
            ctx.set_presence(Some(act), OnlineStatus::Online).await;
            sleep(Duration::from_millis(REFRESH_TIME)).await;
        }
    }
}
