# mcbot-rs
Discord bot for displaying
## Usage
mcbot-rs supports the following commands:
- !mc_info
  - Replies with info about the server, such as players logged on
## Setup
mc-rs requires the environmental variable `MC_BOT_TOK`, which is a discord bot API token.

#### Setting environment variables
##### Windows Powershell
```
$Env:TOKEN_NAME="TOKEN_KEY"
```
##### Linux/Unix/Mac
```
export TOKEN_NAME="TOKEN_KEY"
```
#### MC_BOT_TOK
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

### Instalation
You can run the bot 3 ways:
- Install via `cargo install --git https://github.com/Hpmason/mcbot-rs` and then run with `mcbot-rs` command
- Download release executable
- Build from source
  - git clone this repo
  - run `cargo run` or build then run with `cargo build`

# Contributing
 I'm always open to pull requests and suggestions
