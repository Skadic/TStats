use std::future::Future;

use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::error::CacheError;

use redis::AsyncCommands;

pub async fn get_cached<Out, Err, Supplier, Fut>(
    connection: &mut redis::aio::Connection,
    key: &str,
    supplier: Supplier,
) -> Result<Out, CacheError>
where
    Fut: Future<Output = Result<Out, Err>>,
    Supplier: FnOnce() -> Fut,
    Out: Serialize + DeserializeOwned,
    CacheError: From<Err>,
{
    let val = if let Ok(cached) = connection.get::<_, String>(key).await {
        serde_json::from_str(&cached)?
    } else {
        let calculated = supplier().await?;
        let json_str = serde_json::to_string(&calculated)?;
        connection
            .set::<&str, String, String>(key, json_str)
            .await?;
        calculated
    };

    Ok(val)
}
