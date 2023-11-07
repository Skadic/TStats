//! This module contains utilities for cacheing values using Redis.

use std::borrow::BorrowMut;
use std::sync::Arc;
use std::{convert::Infallible, future::Future};

use redis::{AsyncCommands, FromRedisValue};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tracing::info;

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

pub type CacheResult<T> = Result<T, CacheError>;

/// An error that can occur during caching
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("error during (de)serialization")]
    Serde(#[from] serde_json::Error),
    #[error("error interacting with redis")]
    Redis(#[from] redis::RedisError),
    #[error("error in request")]
    Request(Box<dyn std::error::Error + Send + Sync>),
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
pub async fn cache<V>(
    mut redis: impl BorrowMut<redis::aio::MultiplexedConnection>,
    v: &V,
    expiry_time: Option<usize>,
) -> Result<(), CacheError>
where
    V: Cacheable,
{
    let serialized = serde_json::to_string(v)?;

    let resp = if let Some(time) = expiry_time {
        redis
            .borrow_mut()
            .set_ex::<String, String, String>(v.full_key(), serialized, time)
    } else {
        redis
            .borrow_mut()
            .set::<String, String, String>(v.full_key(), serialized)
    }
    .await?;

    info!("stored value in cache: {resp}");

    Ok(())
}

/// Gets a value from redis. Or `Ok(None)` if it doesn't exist
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
pub async fn get_cached<V>(
    mut redis: impl BorrowMut<redis::aio::MultiplexedConnection>,
    key: &V::KeyType,
) -> Result<Option<V>, CacheError>
where
    V: Cacheable,
{
    // Try to find the value in the cache
    let Some(s) = redis
        .borrow_mut()
        .get::<String, redis::Value>(V::full_key_with(key))
        .await
        .and_then(|v| match v {
            // If it doesn't exist, we just return "None"
            redis::Value::Nil => Ok(None),
            // Otherwise we try to convert it to a string to parse later
            v => String::from_redis_value(&v).map(Some),
        })?
    else {
        // If it's not found, just return
        return Ok(None);
    };

    // Try to parse it to the output value
    serde_json::from_str::<V>(&s)
        .map(Some)
        .map_err(CacheError::from)
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
pub async fn get_cached_or_infallible<V, Fut>(
    redis: impl BorrowMut<redis::aio::MultiplexedConnection>,
    key: &V::KeyType,
    expiry_time: Option<usize>,
    get_fn: impl FnOnce() -> Fut,
) -> Result<V, CacheError>
where
    V: Cacheable,
    Fut: Future<Output = V>,
{
    get_cached_or::<V, Infallible, _>(redis, key, expiry_time, || async { Ok(get_fn().await) })
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
pub async fn get_cached_or<V, E, Fut>(
    mut redis: impl BorrowMut<redis::aio::MultiplexedConnection>,
    key: &V::KeyType,
    expiry_time: Option<usize>,
    get_fn: impl FnOnce() -> Fut,
) -> Result<V, CacheError>
where
    V: Cacheable,
    E: 'static + std::error::Error + Send + Sync,
    Fut: Future<Output = Result<V, E>>,
{
    // Try to find the value in the cache
    if let Ok(Some(v)) = get_cached::<V>(redis.borrow_mut(), key).await {
        // If found, just return it
        return Ok(v);
    }

    // Otherwise, try to get it from the function
    let v = get_fn()
        .await
        .map_err(|e| CacheError::Request(Box::new(e)))?;

    // Cache the value and return it
    cache::<V>(redis, &v, expiry_time).await?;
    Ok(v)
}
