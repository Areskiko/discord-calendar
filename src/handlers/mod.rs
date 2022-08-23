use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;

use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

mod commands;
use commands::{ping, sleep};

pub struct MainHandler;

#[async_trait]
impl EventHandler for MainHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => ping().await,
                "sleep" => sleep().await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("ping").description("A ping command")
            });

            commands.create_application_command(|command| {
                command.name("sleep").description("A sleep command")
            })
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("wonderful_command")
                .description("An amazing command")
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}
