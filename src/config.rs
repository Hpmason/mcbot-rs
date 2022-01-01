use std::env;

use lazy_static::lazy_static;

pub const REFRESH_TIME: u64 = 3000;

lazy_static! {
    // Configure the client with your Discord bot token in the environment.
    pub static ref TOKEN: String = env::var("MC_BOT_TOK")
        .expect("Expected a token in the environment");
    // Get mc server address
    pub static ref ADDR: String = env::var("MC_BOT_ADDR")
        .expect("Expected a MC_BOT_ADDR in the environment");
    // Get mc server port number, or set to 25565
    pub static ref PORT: u16 = match env::var("MC_BOT_PORT") {
        Ok(s) => s.parse().or::<u16>(Ok(25565)).unwrap(),
        Err(_) => 25565
    };
}
