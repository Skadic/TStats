use tonic::Status;
use tracing::{debug, error, info, trace, warn, Level};

pub trait LogStatus: Sized {
    fn log_status(self, level: Level) -> Self;
    fn trace_status(self) -> Self {
        self.log_status(Level::TRACE)
    }
    fn debug_status(self) -> Self {
        self.log_status(Level::DEBUG)
    }
    fn info_status(self) -> Self {
        self.log_status(Level::INFO)
    }
    fn warn_status(self) -> Self {
        self.log_status(Level::WARN)
    }
    fn error_status(self) -> Self {
        self.log_status(Level::ERROR)
    }
}

impl<T> LogStatus for Result<T, Status> {
    fn log_status(self, level: Level) -> Self {
        self.map_err(|status| {
            match level {
                Level::TRACE => {
                    trace!("{}", status.message());
                }
                Level::DEBUG => {
                    debug!("{}", status.message());
                }
                Level::INFO => {
                    info!("{}", status.message());
                }
                Level::WARN => {
                    warn!("{}", status.message());
                }
                Level::ERROR => {
                    error!("{}", status.message());
                }
            }

            status
        })
    }
}
