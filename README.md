# mcbot-rs
Discord bot for displaying info of a minecraft server
Docker image can be found on my [GitHub Container Registry](https://github.com/Hpmason/mcbot-rs/pkgs/container/mcbot-rs)
```
docker pull ghcr.io/hpmason/mcbot-rs:latest
```
## Usage
The main feature of the bot is displaying player count in the bot's discord presence. 
mcbot-rs also supports the following command:
- !mc_info
  - Replies with info about the server, such as players logged on

## Setup
mcbot-rs requires 2 environmental variables tokens/keys.
- `MC_BOT_TOK`
  - Discord API bot token. See steps below on obtain this token
- `MC_BOT_ADDR`
  - The minecraft server address to link the bot to.
- Optional `MC_BOT_PORT`
  - Port of the minecraft server, if the env variable is not set, it will use the default minecraft port of 25565

Once you have these variables, update the `.env` file with the values.
### Running



If you want to build the image manually, you can do so by running the following commands:
```
docker build -t mcbot .
```
Run built image
```
docker run --rm --env-file .env --name mcbot mcbot
```

### MC_BOT_TOK
This token is required in order for the program to communicate with discord via a bot.

You can get this token from https://discord.com/developers/applications.
Steps are:
1. Create a discord app
2. In your new app, click the `Bot` tab on the left hand side
3. Click `Add Bot`
4. Copy the token provided
5. Add this as an environment variable under the name `MC_BOT_TOK`
6. Under `OAuth2` create an OAuth url with the scope of `bot` and the permission to send messages
    - In order to add the bot to your disord server, open this link and discord will walk you through adding the bot

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.