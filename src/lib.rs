pub mod utils;
use sqlx::postgres::PgPool;

extern crate derive_more;
use teloxide::RequestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from SQLx: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("error from std::env: {0}")]
    EnvError(#[from] std::env::VarError),
    #[error("error from reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("error from chrono: {0}")]
    ChronoError(#[from] chrono::ParseError),
}

pub fn east_coast_date_today() -> Result<chrono::NaiveDate, Error> {
    let today_east_coast_delayed_format = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::hours(5))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    Ok(chrono::NaiveDate::parse_from_str(
        &today_east_coast_delayed_format,
        "%Y-%m-%d",
    )?)
}

/// past: describes if you want the day x days in the past (true) or in the future (false)
pub fn east_coast_date_in_x_days(days: i64, past: bool) -> Result<chrono::NaiveDate, Error> {
    let east_coast_datetime = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::hours(5))
        .unwrap();
    let east_coast_delayed_format = match past {
        true => east_coast_datetime
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string(),
        false => east_coast_datetime
            .checked_add_signed(chrono::Duration::days(days))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string(),
    };

    Ok(chrono::NaiveDate::parse_from_str(
        &east_coast_delayed_format,
        "%Y-%m-%d",
    )?)
}

pub async fn get_active_chat_status(pool: &PgPool, chat_id: i64) -> Result<bool, Error> {
    Ok(
        sqlx::query!("SELECT is_active FROM chats WHERE id = $1", chat_id)
            .fetch_one(pool)
            .await?
            .is_active
            .unwrap_or(false),
    )
}
