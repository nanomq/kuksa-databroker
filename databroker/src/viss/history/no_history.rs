use async_trait::async_trait;
use crate::viss::history::HistoryProvider;

pub struct NoHistory;

#[async_trait]
impl HistoryProvider for NoHistory {}
