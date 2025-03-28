use ignore::WalkBuilder;
use nvim_oxi::{Result, api::Buffer};

use crate::list::{List, ListItem};

pub struct FilePickerItem(String);

#[allow(unused_variables)]
impl ListItem for FilePickerItem {
    fn text(&self) -> String {
        self.0.clone()
    }
    fn highlight(&self, buf: &Buffer, line: u32) -> Result<()> {
        Ok(())
    }
}

pub struct FilePicker {}

impl FilePicker {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(unused_variables)]
impl List for FilePicker {
    type Item = FilePickerItem;

    fn items(&mut self, _prompt: String) -> Result<Vec<Self::Item>> {
        let mut items = vec![];
        for entry in WalkBuilder::new(".")
            .hidden(true)
            .add_custom_ignore_filename(".git")
            .build()
        {
            match entry {
                Ok(entry) => {
                    let path = entry.path().display().to_string();
                    if path == "." {
                        continue;
                    }
                    items.push(FilePickerItem(path[2..].to_string()));
                }
                Err(e) => {}
            }
        }

        Ok(items)
    }

    fn preview(&self, item: &Self::Item) -> Result<()> {
        Ok(())
    }

    fn select(&self, item: &Self::Item) -> Result<()> {
        Ok(())
    }
}
