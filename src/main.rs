use std::{error::Error, time::Instant};

use day_query::DayQuery;
use dotenv::dotenv;

use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod day_query;
mod endpoint;
mod price_query;

use endpoint::Endpoint;
use price_query::get_price;

#[derive(BotCommands, Clone, Debug)]
#[command(rename = "lowercase", description = "Comandos disponibles:")]
enum Command {
    Luz,
    Dia,
    Help,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dotenv().ok();

    info!("Creating bot...");
    let bot = Bot::from_env().auto_send();

    info!("Starting bot...");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let sender = if message.chat.is_group() {
        message.chat.title().unwrap()
    } else {
        message.chat.username().unwrap()
    };
    info!("Request from {} -> [{:?}]", sender, command);
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Luz => {
            let start = Instant::now();
            let response = if let Ok(price) = get_price(Endpoint::Now).await {
                bot.send_message(message.chat.id, format!("{price}"))
                    .await?
            } else {
                bot.send_message(message.chat.id, format!("Se ha producido un error"))
                    .await?
            };
            info!("Price request attended in {:?}", (Instant::now() - start));
            response
        }
        Command::Dia => {
            let start = Instant::now();
            let response = match DayQuery::new().await {
                Ok(day) => bot.send_message(message.chat.id, format!("{day}")).await?,
                Err(er) => {
                    error!("{er}");
                    bot.send_message(message.chat.id, format!("Se ha producido un error"))
                        .await?
                }
            };
            info!("Day request attended in {:?}", (Instant::now() - start));
            response
        }
    };

    Ok(())
}
