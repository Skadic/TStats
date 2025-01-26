use futures::{FutureExt, TryFutureExt};
use miette::{Context, IntoDiagnostic};
use poem::http::StatusCode;
use std::{
    fmt::Display,
    future::Future,
};

pub trait LogError<T, Msg>: Sized {
    fn log_error(self, msg: Msg) -> miette::Result<T>;
}

pub trait LogErrorDiagnostic<T, Msg>: Sized {
    fn log_error(self, msg: Msg) -> miette::Result<T>;
}

pub trait LogErrorFuture<T, Msg>: Sized + Send {
    fn log_error(self, msg: Msg) -> impl Future<Output = miette::Result<T>> + Send;
}

pub trait LogErrorDiagnosticFuture<T, Msg>: Sized + Send {
    fn log_error(self, msg: Msg) -> impl Future<Output = miette::Result<T>> + Send;
}

impl<T, E, Msg> LogError<T, Msg> for Result<T, E>
where
    E: 'static + std::error::Error + Send + Sync,
    Msg: 'static + Display + Send + Sync,
{
    #[inline]
    fn log_error(self, server_msg: Msg) -> miette::Result<T> {
        self.into_diagnostic()
            .wrap_err(server_msg)
            .inspect_err(|e| tracing::error!("{e:?}"))
    }
}

impl<T, Msg> LogError<T, Msg> for Option<T>
where
    Msg: 'static + Display + Send + Sync,
{
    #[inline]
    fn log_error(self, server_msg: Msg) -> miette::Result<T> {
        self.ok_or_else(|| miette::miette!(server_msg.to_string()))
            .inspect_err(|e| tracing::error!("{e:?}"))
    }
}

impl<T, Msg> LogErrorDiagnostic<T, Msg> for miette::Result<T>
where
    Msg: 'static + Display + Send + Sync,
{
    #[inline]
    fn log_error(self, server_msg: Msg) -> miette::Result<T> {
        self.wrap_err(server_msg)
            .inspect_err(|e| tracing::error!("{e:?}"))
    }
}

impl<T, Msg, Fut> LogErrorFuture<T, Msg> for Fut
where
    Msg: 'static + Display + Send + Sync,
    Fut: Future<Output: LogError<T, Msg>> + Send,
{
    async fn log_error(self, msg: Msg) -> miette::Result<T> {
        self.map(move |res| res.log_error(msg)).await
    }
}

impl<T, Msg, Fut> LogErrorDiagnosticFuture<T, Msg> for Fut
where
    Msg: 'static + Display + Send + Sync,
    Fut: Future<Output: LogErrorDiagnostic<T, Msg>> + Send,
{
    async fn log_error(self, msg: Msg) -> miette::Result<T> {
        self.map(move |res| res.log_error(msg)).await
    }
}

pub trait ToPoemError<T>: Sized {
    fn with_status(self, code: StatusCode) -> poem::Result<T>;

    #[inline]
    fn internal_server_error(self) -> poem::Result<T> {
        self.with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub trait ToPoemErrorFuture<T>: Sized {
    fn with_status(self, code: StatusCode) -> impl Future<Output = poem::Result<T>> + Send;

    #[inline]
    fn internal_server_error(self) -> impl Future<Output = poem::Result<T>> + Send {
        self.with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl<T> ToPoemError<T> for miette::Result<T> {
    fn with_status(self, code: StatusCode) -> poem::Result<T> {
        self.map_err(|e| poem::Error::from_string(e.to_string(), code))
    }
}

impl<T, Fut> ToPoemErrorFuture<T> for Fut where Fut: Future<Output = miette::Result<T>> + Send   {
    async fn with_status(self, code: StatusCode) -> poem::Result<T> {
        self.map_err(|e| poem::Error::from_string(e.to_string(), code)).await
    }
}
/*
pub trait LogPoemErrorFuture<T, Msg>: Sized + Send {
    #[inline]
    fn log_internal_server_error(
        self,
        server_msg: Msg,
    ) -> impl Future<Output = poem::Result<T>> + Send {
        self.log_poem_error_with_client_msg(server_msg, None, StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[inline]
    fn log_error(
        self,
        server_msg: Msg,
        status: StatusCode,
    ) -> impl Future<Output = poem::Result<T>> + Send {
        self.log_poem_error_with_client_msg(server_msg, None, status)
    }

    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> impl Future<Output = poem::Result<T>> + Send;
}

pub trait LogPoemError<T, Msg>: Sized {
    #[inline]
    fn log_internal_server_error(self, server_msg: Msg) -> poem::Result<T> {
        self.log_poem_error_with_client_msg(server_msg, None, StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[inline]
    fn log_error(self, server_msg: Msg, status: StatusCode) -> poem::Result<T> {
        self.log_poem_error_with_client_msg(server_msg, None, status)
    }

    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> poem::Result<T>;
}

pub trait LogPoemErrorFuture<T, Msg>: Sized + Send {
    #[inline]
    fn log_internal_server_error(
        self,
        server_msg: Msg,
    ) -> impl Future<Output = poem::Result<T>> + Send {
        self.log_poem_error_with_client_msg(server_msg, None, StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[inline]
    fn log_error(
        self,
        server_msg: Msg,
        status: StatusCode,
    ) -> impl Future<Output = poem::Result<T>> + Send {
        self.log_poem_error_with_client_msg(server_msg, None, status)
    }

    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> impl Future<Output = poem::Result<T>> + Send;
}

impl<T, Msg: Display> LogPoemError<T, Msg> for miette::Result<T> {
    #[inline]
    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> poem::Result<T> {
        self.inspect_err(|error| tracing::error!("{server_msg}: {error:?}"))
            .map_err(|_| match client_msg {
                Some(msg) => poem::Error::from_string(msg.to_string(), status),
                None => poem::Error::from_status(status),
            })
    }
}

impl<T, Msg: Display> LogPoemError<T, Msg> for Option<T> {
    #[inline]
    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> poem::Result<T> {
        self.ok_or_else(|| {
            tracing::error!("{server_msg}");
            return match client_msg {
                Some(msg) => poem::Error::from_string(msg.to_string(), status),
                None => poem::Error::from_status(status),
            };
        })
    }
}

impl<T, F, Msg> LogPoemErrorFuture<T, Msg> for F
where
    F: Future<Output: LogPoemError<T, Msg>> + Send,
    Msg: Display + Send,
{
    #[inline]
    async fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> poem::Result<T> {
        let res = self.await;
        res.log_poem_error_with_client_msg(server_msg, client_msg, status)
    }
}

*/