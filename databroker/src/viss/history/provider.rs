use async_trait::async_trait;
use std::time::SystemTime;
use crate::broker::Datapoint;

#[derive(Debug)]
pub enum HistoryError {
    NotFound,
    BackendUnavailable,
    InternalError,
}

#[async_trait]
pub trait HistoryProvider: Send + Sync {
    async fn get_history(
        &self,
        _path: &str,
        _start: SystemTime,
        _end: SystemTime,
    ) -> Result<Vec<Datapoint>, HistoryError> {
        Err(HistoryError::BackendUnavailable)
    }
}