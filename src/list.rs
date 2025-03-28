use std::sync::Arc;

use nvim_oxi::{Result, api::Buffer, libuv::AsyncHandle};
use tokio::sync::{mpsc::UnboundedSender, oneshot};

#[allow(dead_code)]
pub trait ListItem {
    fn text(&self) -> String;
    fn highlight(&self, buf: &Buffer, line: u32) -> Result<()>;
}

#[allow(dead_code)]
pub trait List {
    type Item: ListItem + Sync + Send;

    fn items(
        &mut self,
        prompt: String,
        handle: AsyncHandle,
        sender: UnboundedSender<Self::Item>,
    ) -> Result<()>;
    fn preview(&self, item: &Self::Item) -> Result<()>;
    fn select(&self, item: &Self::Item) -> Result<()>;
}
