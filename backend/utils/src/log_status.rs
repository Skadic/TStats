use std::{fmt::Display, future::Future};

use poem::http::StatusCode;

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

impl<T, E: Display + Send + Sync, Msg: Display> LogPoemError<T, Msg> for Result<T, E> {
    #[inline]
    fn log_poem_error_with_client_msg(
        self,
        server_msg: Msg,
        client_msg: Option<Msg>,
        status: StatusCode,
    ) -> poem::Result<T> {
        self.inspect_err(|error| tracing::error!(%error, "{server_msg}"))
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
