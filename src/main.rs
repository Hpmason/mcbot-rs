use serenity::framework::standard::StandardFramework;
use serenity::Client;

use mcbot_rs::config::*;
use mcbot_rs::handler::*;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&MY_HELP)
        // The `#[group]` (and similarly, `#[command]`) macro generates static instances
        // containing any options you gave it. For instance, the group `name` and its `commands`.
        // Their identifiers, names you can use to refer to these instances in code, are an
        // all-uppercased version of the `name` with a `_GROUP` suffix appended at the end.
        .group(&INFO_GROUP);

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(TOKEN.as_str())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
