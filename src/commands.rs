use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::config::*;
use crate::helpers::*;

#[command]
#[usage("[address] [port]")]
#[example("mc.example.com 25566")]
#[description("Gets server info from server")]
#[max_args(2)]
async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let addr = args.single::<String>().unwrap_or_else(|_| ADDR.to_string());
    let port = args.single::<u16>().unwrap_or(*PORT);

    // Get status response from server
    let res = get_status(&addr, port).await;
    // println!("Res: {:?}", res);
    // If successful, send response info to user
    msg.channel_id
        .say(&ctx.http, status_or_error_message(res))
        .await?;
    Ok(())
}
