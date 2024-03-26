use tonic::Status;
use tracing_log::log::{log, Level};

pub trait LogStatus: Sized {
    fn log_status(self, level: Level) -> Self;
    fn trace_status(self) -> Self {
        self.log_status(Level::Info)
    }
    fn debug_status(self) -> Self {
        self.log_status(Level::Debug)
    }
    fn info_status(self) -> Self {
        self.log_status(Level::Info)
    }
    fn warn_status(self) -> Self {
        self.log_status(Level::Warn)
    }
    fn error_status(self) -> Self {
        self.log_status(Level::Error)
    }
}

impl<T> LogStatus for Result<T, Status> {
    fn log_status(self, level: Level) -> Self {
        self.map_err(|status| {
            log!(level, "{}", status.message());
            status
        })
    }
}
