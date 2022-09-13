use std::future::Future;

use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::error::CacheError;

use redis::AsyncCommands;

/// This attempts to find a value with the given key in the Redis cache.
/// If it is not found, the supplier function is called to produce the value.
/// That value cached under the given key as json and then returned.
///
/// # Errors
///
/// This function will return:
/// A Redis variant of CacheError, if the communication or caching with Redis fails.
/// A Json variant of CacheError, if serialization or deserialization fails.
/// A different variant of CacheError depending on the errors returned by the supplier.
///
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
    // Try to find the value in the cache
    let val = if let Ok(cached) = connection.get::<_, String>(key).await {
        // If found, deserialize and return it
        serde_json::from_str(&cached)?
    } else {
        // If not found, get the value from the supplier, serialize and cache it.
        let supplied = supplier().await?;
        let json_str = serde_json::to_string(&supplied)?;
        connection
            .set::<&str, String, String>(key, json_str)
            .await?;
        supplied
    };

    Ok(val)
}

/// This attempts to find a value with the given key in the Redis cache.
/// If it is not found, the supplier function is called which may produce a value.
/// If a value is returned, that value is cached under the given key as json and then returned.
/// If no value is returned by the supplier, Ok(None) is returned.
///
/// # Errors
///
/// This function will return:
/// A Redis variant of CacheError, if the communication or caching with Redis fails.
/// A Json variant of CacheError, if serialization or deserialization fails.
/// A different variant of CacheError depending on the errors returned by the supplier.
///
pub async fn get_cached_opt<Out, Err, Supplier, Fut>(
    connection: &mut redis::aio::Connection,
    key: &str,
    supplier: Supplier,
) -> Result<Option<Out>, CacheError>
where
    Fut: Future<Output = Result<Option<Out>, Err>>,
    Supplier: FnOnce() -> Fut,
    Out: Serialize + DeserializeOwned,
    CacheError: From<Err>,
{
    // Try to find the value in the cache
    let val = if let Ok(cached) = connection.get::<_, String>(key).await {
        // If found, deserialize and return it
        serde_json::from_str(&cached)?
    } else {
        // If not found, get the value from the supplier, serialize and cache it.
        let supplied = supplier().await?;
        if supplied.is_some() {
            let json_str = serde_json::to_string(&supplied)?;
            connection
                .set::<&str, String, String>(key, json_str)
                .await?;
        }
        supplied
    };

    Ok(val)
}