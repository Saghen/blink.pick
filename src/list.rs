use nvim_oxi::{Result, api::Buffer};

#[allow(dead_code)]
pub trait ListItem {
    fn text(&self) -> String;
    fn highlight(&self, buf: &Buffer, line: u32) -> Result<()>;
}

#[allow(dead_code)]
pub trait List {
    type Item: ListItem;

    fn items(&mut self, prompt: String) -> Result<Vec<Self::Item>>;
    fn preview(&self, item: &Self::Item) -> Result<()>;
    fn select(&self, item: &Self::Item) -> Result<()>;
}
