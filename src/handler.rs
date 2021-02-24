use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        user::OnlineStatus,
        channel::Message, 
        gateway::{
            Ready,
            Activity,
        },
    },
    framework::standard::{
        macros::{command, group},
        CommandResult,
    }
};

use anyhow::Result;

use async_minecraft_ping::{
    ConnectionConfig,
    StatusResponse,
};

use tokio::time::{delay_for, Duration};

const ADDR: &str = "mc.hpmason.com";
const PORT: u16 = 25565;

async fn get_status(server_addr: &str, port: u16) -> Result<StatusResponse> {
    let mut config = ConnectionConfig::build(server_addr.to_string());
    config = config.with_port(port);

    let mut connection = config.connect().await?;
    Ok(connection.status().await?)
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
async fn mc_info(ctx: &Context, msg: &Message) -> CommandResult {
    let status = get_status(&self.addr, self.port).await?;
    
    println!("{} of  {}", status.players.online, status.players.max);
    let mut players_info = format!("Server info: {} players online", status.players.online);
    if let Some(players) = status.players.sample {
        let name_vec: Vec<_> = players.into_iter().map(|i| {
            i.name
        }).collect();
        players_info = format!("{}\nPlayers: {:?}", players_info, name_vec)
    } 
    msg.channel_id.say(&ctx.http, players_info).await?;
    Ok(())
}

#[group]
#[commands(ping, mc_info)]
struct General;
pub struct Handler {
    addr: String,
    port: u16,
}
impl Handler {
    pub fn new(addr: String, port: Option<u16>) -> Self {
        Self {
            addr,
            port: match port {
                Some(p) => p,
                None => 25565,
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        
        loop {
            let result = get_status(&self.addr, self.port).await;
            match result {
                Ok(status) => {
                    let act = Activity::playing(format!("Minecraft w/{} players", status.players.online).as_str());
                    ctx.set_presence(Some(act), OnlineStatus::Online).await;
                },
                Err(_) => println!("Error getting status")
            }
            
            delay_for(Duration::from_millis(3000)).await;
            
        }
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
