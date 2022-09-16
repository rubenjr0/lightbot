use std::{error::Error, sync::Arc, time::Instant};

use cache::Cache;
use chrono::{Datelike, NaiveDate, Timelike, DateTime, Local};
use day_query::DayQuery;
use dotenv::dotenv;

use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::{RwLock, RwLockWriteGuard};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod cache;
mod day_query;
mod endpoint;
mod price_query;

use price_query::{get_price, PriceQuery};

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

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(answer);

    let price_cache: Cache<PriceQuery> = Arc::new(RwLock::new(None));
    let day_cache: Cache<DayQuery> = Arc::new(RwLock::new(None));

    info!("Starting bot...");
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![price_cache, day_cache])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn report_duration(start: Instant) {
    info!("Price request attended in {:.2?}", (Instant::now() - start));
}

async fn update_price_cache(lock: &mut RwLockWriteGuard<'_, Option<PriceQuery>>) -> String {
    match get_price(endpoint::Endpoint::Now).await {
        Ok(price) => {
            **lock = Some(price.clone());
            info!("Price cache updated");
            price.to_string()
        }
        Err(error) => {
            error!("{error}");
            String::from("Se ha producido un error")
        }
    }
}

async fn update_day_cache(lock: &mut RwLockWriteGuard<'_, Option<DayQuery>>) -> String {
    match DayQuery::new().await {
        Ok(day) => {
            **lock = Some(day.clone());
            info!("Day cache updated");
            day.to_string()
        }
        Err(error) => {
            error!("{error}");
            String::from("Se ha producido un error")
        }
    }
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
    price_cache: Cache<PriceQuery>,
    day_cache: Cache<DayQuery>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let start = Instant::now();
    let message_dt: DateTime<Local> = DateTime::from(message.date);
    dbg!(&message_dt);

    if let Some(sender) = if message.chat.is_group() {
        message.chat.title()
    } else {
        message.chat.username()
    } {
        info!("Request from {} -> [{:?}]", sender, command);
    } else {
        info!("Request from unknown sender -> [{:?}]", command);
    }
    let reply = match command {
        Command::Help => Command::descriptions().to_string(),
        Command::Luz => {
            let mut lock = price_cache.write().await;
            if lock.is_none() {
                update_price_cache(&mut lock).await
            } else {
                let price_query = lock.as_ref().unwrap();
                let (_, end) = price_query.hour().unwrap();
                info!(
                    "Cache end's time is {end}, and the current message was sent at {}",
                    message_dt.time().hour()
                );
                if message_dt.time().hour() >= end as u32 {
                    update_price_cache(&mut lock).await
                } else {
                    info!("Cache hit on price data");
                    price_query.to_string()
                }
            }
        }
        Command::Dia => {
            let mut lock = day_cache.write().await;
            if lock.is_none() {
                update_day_cache(&mut lock).await
            } else {
                let day_query = lock.as_ref().unwrap();
                let day_date = NaiveDate::parse_from_str(day_query.date(), "%d-%m-%Y")
                    .unwrap()
                    .day();
                if message_dt.day() > day_date {
                    update_day_cache(&mut lock).await
                } else {
                    info!("Cache hit on day data");
                    day_query.to_string()
                }
            }
        }
    };

    bot.send_message(message.chat.id, reply)
        .reply_to_message_id(message.id)
        .disable_notification(true)
        .await?;

    report_duration(start);

    Ok(())
}
