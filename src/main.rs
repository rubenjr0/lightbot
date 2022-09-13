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
#[command(
    rename = "lowercase",
    description = "Comandos disponibles. Todos los precios estan en € por kWh."
)]
enum Command {
    Luz,
    Dia,
    Help,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Creating bot...");
    dotenv().ok();
    let bot = Bot::from_env().auto_send();

    info!("Starting bot...");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

fn report_duration(start: Instant) {
    info!("Price request attended in {:.2?}", (Instant::now() - start));
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let start = Instant::now();
    if let Some(sender) = if message.chat.is_group() {
        message.chat.title()
    } else {
        message.chat.username()
    } {
        info!("Request from {} -> [{:?}]", sender, command);
    }
    let reply = match command {
        Command::Help => Command::descriptions().to_string(),
        Command::Luz => match get_price(Endpoint::Now).await {
            Ok(price) => price.to_string(),
            Err(error) => {
                error!("{error}");
                String::from("Se ha producido un error")
            }
        },
        Command::Dia => match DayQuery::new().await {
            Ok(day) => day.to_string(),
            Err(error) => {
                error!("{error}");
                String::from("Se ha producido un error")
            }
        },
    };

    bot.send_message(message.chat.id, reply)
        .reply_to_message_id(message.id)
        .disable_notification(true)
        .await?;

    report_duration(start);

    Ok(())
}
