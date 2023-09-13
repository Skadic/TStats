use std::{convert::Infallible, future::Future};

use redis::{aio::ConnectionLike, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tracing::{info, warn};

/// A trait for structs cached in the redis store
pub trait Cacheable: Serialize + DeserializeOwned {
    type KeyType: ToString;

    /// Returns a unique string for this type with which all entries in the redis store are prefixed.
    fn type_key() -> &'static str;

    /// Returns a key that identifies the specific object among the entries in the redis store.
    fn key(&self) -> Self::KeyType;

    /// Returns the key that the current object would have in the redis store
    fn full_key(&self) -> String {
        format!("{}:{}", Self::type_key(), self.key().to_string())
    }

    /// Returns the full redis store key for an object with the given key type
    fn full_key_with(key: &Self::KeyType) -> String {
        format!("{}:{}", Self::type_key(), key.to_string())
    }
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("error during (de)serialization")]
    Serde(#[from] serde_json::Error),
    #[error("error interacting with redis")]
    Redis(#[from] redis::RedisError),
    #[error("error in request")]
    Request(Box<dyn std::error::Error>),
}

/// Stores a value in redis.
///
/// # Arguments
///
/// * `redis` - A connection to the redis instance
/// * `v` - The value to store.
/// * `expiry_time` - An optional number of seconds until the entry expires.
///
/// # Errors
///
/// An error can occur when serialization fails, or the set command in the redis store fails.
///
pub async fn cache<V, Conn>(
    redis: &mut Conn,
    v: &V,
    expiry_time: Option<usize>,
) -> Result<(), CacheError>
where
    V: Cacheable,
    Conn: ConnectionLike + Send + Sync,
{
    let serialized = serde_json::to_string(v)?;

    let resp = if let Some(time) = expiry_time {
        redis.set_ex::<String, String, String>(v.full_key(), serialized, time)
    } else {
        redis.set::<String, String, String>(v.full_key(), serialized)
    }
    .await?;

    info!("stored value in cache: {resp}");

    Ok(())
}

/// Gets a value from redis.
///
/// # Arguments
///
/// * `redis` - A connection to the redis instance
/// * `v` - The value's key.
///
/// # Errors
///
/// An error can occur when deserialization fails, or the get command in the redis store fails.
///
pub async fn get_cached<V, Conn>(
    redis: &mut redis::aio::MultiplexedConnection,
    key: &V::KeyType,
) -> Result<Option<V>, CacheError>
where
    V: Cacheable,
    Conn: ConnectionLike + Send + Sync,
{
    // Try to find the value in the cache
    Ok(
        match redis.get::<String, String>(V::full_key_with(key)).await {
            // If found, just return it
            Ok(v) => Some(serde_json::from_str(&v)?),
            // If not found, try getting it from the lambda and cache it
            Err(e) => match e.kind() {
                _ => {
                    warn!("error retrieving from cache: {e}");
                    None
                }
            },
        },
    )
}

/// Tries to get a value from the cache and returns it, if it exist.
/// If it does not exists, calls a function to get a value (e.g. from an API), caches it and
/// returns it.
///
/// # Arguments
///
/// * `redis` - A connection to the redis instance
/// * `key` - The value's key.
/// * `expiry_time` - An optional number of seconds until the entry expires.
/// * `get_fn` - A function that gets an instance of the value.
///
/// # Errors
///
/// An error can occur during (de-)seriaization or if the redis set command fails.
///
pub async fn get_cached_or_else<V, Conn, Fut>(
    redis: &mut Conn,
    key: &V::KeyType,
    expiry_time: Option<usize>,
    get_fn: impl FnOnce() -> Fut,
) -> Result<V, CacheError>
where
    V: Cacheable,
    Conn: ConnectionLike + Send + Sync,
    Fut: Future<Output = V>,
{
    get_cached_or_else_fallible::<V, Infallible, Conn, _>(redis, key, expiry_time, || async {
        Ok(get_fn().await)
    })
    .await
}

/// Tries to get a value from the cache and returns it, if it exist.
/// If it does not exists, calls a *fallible* function to get a value (e.g. from an API),
/// caches it and returns it.
///
/// # Arguments
///
/// * `redis` - A connection to the redis instance
/// * `key` - The value's key.
/// * `expiry_time` - An optional number of seconds until the entry expires.
/// * `get_fn` - A function that gets an instance of the value but might return an error.
///
/// # Errors
///
/// An error can occur during (de-)seriaization, if the redis set command fails or if the `get_fn`
/// fails.
///
pub async fn get_cached_or_else_fallible<V, E, Conn, Fut>(
    redis: &mut Conn,
    key: &V::KeyType,
    expiry_time: Option<usize>,
    get_fn: impl FnOnce() -> Fut,
) -> Result<V, CacheError>
where
    V: Cacheable,
    E: 'static + std::error::Error,
    Conn: ConnectionLike + Send + Sync,
    Fut: Future<Output = Result<V, E>>,
{
    // Try to find the value in the cache
    let resp = match redis.get::<String, String>(V::full_key_with(key)).await {
        // If found, just return it
        Ok(v) => {
            info!("found value in cache: {v}");
            serde_json::from_str(&v)?
        }
        // If not found, try getting it from the get function and cache it
        Err(e) => {
            warn!("error retrieving from cache: {e}");
            let v = get_fn()
                .await
                .map_err(|e| CacheError::Request(Box::new(e)))?;

            cache(redis, &v, expiry_time).await?;

            v
        }
    };

    Ok(resp)
}
