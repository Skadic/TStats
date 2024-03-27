//! This module contains utilities for cacheing values using Redis.

use std::fmt::Display;
use std::{convert::Infallible, future::Future};

use deadpool_redis::redis::{AsyncCommands, FromRedisValue};
use miette::{Context, IntoDiagnostic};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tracing::info;

/// A trait for structs cached in the redis store
#[async_trait::async_trait]
pub trait Cacheable: Serialize + DeserializeOwned + Send + Sync {
    type KeyType: ?Sized + Display + Send + Sync;

    /// Returns a unique string for this type with which all entries in the redis store are prefixed.
    fn type_key() -> &'static str;

    /// Returns a key that identifies the specific object among the entries in the redis store.
    fn key(&self) -> &Self::KeyType;

    /// Returns the key that the current object would have in the redis store
    fn full_key(&self) -> String {
        format!("{}:{}", Self::type_key(), self.key())
    }

    /// Returns the full redis store key for an object with the given key type
    fn full_key_with(key: &Self::KeyType) -> String {
        format!("{}:{}", Self::type_key(), key)
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
    async fn cache(
        &self,
        redis: &deadpool_redis::Pool,
        expiry_time: Option<usize>,
    ) -> Result<(), CacheError> {
        cache(redis, self, expiry_time).await
    }

    /// Removes a value from redis.
    ///
    /// # Arguments
    ///
    /// * `redis` - A connection to the redis instance
    /// * `key` - The key to remove.
    ///
    /// # Errors
    ///
    /// An error can occur when deserialization fails, or the del command in the redis store fails.
    ///
    async fn uncache(
        redis: &deadpool_redis::Pool,
        key: &Self::KeyType,
    ) -> Result<Option<Self>, CacheError> {
        uncache(redis, key).await
    }

    /// Gets a value from redis. Or `Ok(None)` if it doesn't exist
    ///
    /// # Arguments
    ///
    /// * `key` - The value's ksy.
    /// * `redis` - A connection to the redis instance
    ///
    /// # Errors
    ///
    /// An error can occur when deserialization fails, or the get command in the redis store fails.
    ///
    async fn get_cached(
        key: &Self::KeyType,
        redis: &deadpool_redis::Pool,
    ) -> Result<Option<Self>, CacheError> {
        get_cached(redis, key).await
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
    async fn get_cached_or_infallible<Fut>(
        redis: &deadpool_redis::Pool,
        key: &Self::KeyType,
        expiry_time: Option<usize>,
        get_fn: impl FnOnce() -> Fut + Send,
    ) -> Result<Self, CacheError>
    where
        Fut: Future<Output = Self> + Send,
    {
        get_cached_or_infallible(redis, key, expiry_time, get_fn).await
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
    async fn get_cached_or<E, Fut>(
        redis: &deadpool_redis::Pool,
        key: &Self::KeyType,
        expiry_time: Option<usize>,
        get_fn: impl FnOnce() -> Fut + Send,
    ) -> Result<Self, CacheError>
    where
        E: 'static + std::error::Error + Send + Sync,
        Fut: Future<Output = Result<Self, E>> + Send,
    {
        get_cached_or(redis, key, expiry_time, get_fn).await
    }
}

pub type CacheResult<T> = Result<T, CacheError>;

/// An error that can occur during caching
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("error during (de)serialization: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("error interacting with redis: {0}")]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error("error in redis connection pool: {0}")]
    Pool(#[from] deadpool_redis::PoolError),
    #[error("error in request: {0}")]
    Request(miette::Error),
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
pub async fn cache<V: Cacheable>(
    redis: &deadpool_redis::Pool,
    v: &V,
    expiry_time: Option<usize>,
) -> Result<(), CacheError> {
    let serialized = serde_json::to_string(v)?;
    let mut conn = redis.get().await?;

    if let Some(time) = expiry_time {
        conn.set_ex::<String, String, String>(v.full_key(), serialized, time as u64)
    } else {
        conn.set::<String, String, String>(v.full_key(), serialized)
    }
    .await?;

    Ok(())
}

/// Removes a value from redis.
///
/// # Arguments
///
/// * `redis` - A connection to the redis instance
/// * `key` - The key to remove.
///
/// # Errors
///
/// An error can occur when deserialization fails, or the del command in the redis store fails.
///
pub async fn uncache<V: Cacheable>(
    redis: &deadpool_redis::Pool,
    key: &V::KeyType,
) -> CacheResult<Option<V>> {
    let mut conn = redis.get().await?;

    let Some(s) = conn
        .get_del::<_, deadpool_redis::redis::Value>(V::full_key_with(key))
        .await
        .and_then(|v| match v {
            // If it doesn't exist, we just return "None"
            deadpool_redis::redis::Value::Nil => Ok(None),
            // Otherwise we try to convert it to a string to parse later
            v => String::from_redis_value(&v).map(Some),
        })?
    else {
        return Ok(None);
    };

    // Try to parse it to the output value
    serde_json::from_str::<V>(&s)
        .map(Some)
        .map_err(CacheError::from)
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
pub async fn get_cached<V: Cacheable>(
    redis: &deadpool_redis::Pool,
    key: &V::KeyType,
) -> Result<Option<V>, CacheError> {
    let mut conn = redis.get().await?;
    // Try to find the value in the cache
    let Some(s) = conn
        .get::<String, deadpool_redis::redis::Value>(V::full_key_with(key))
        .await
        .and_then(|v| match v {
            // If it doesn't exist, we just return "None"
            deadpool_redis::redis::Value::Nil => Ok(None),
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
    redis: &deadpool_redis::Pool,
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
    redis: &deadpool_redis::Pool,
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
    if let Ok(Some(v)) = get_cached::<V>(redis, key).await {
        // If found, just return it
        return Ok(v);
    }

    // Otherwise, try to get it from the function
    let v = get_fn()
        .await
        .into_diagnostic()
        .wrap_err("error requesting value")
        .map_err(CacheError::Request)?;

    // Cache the value and return it
    cache(redis, &v, expiry_time).await?;
    Ok(v)
}
