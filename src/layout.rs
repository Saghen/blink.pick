use crate::view::View;
use nvim_oxi::api::{
    Error, Window,
    types::{SplitDirection, WindowConfig},
};

pub struct Layout {
    preview: Window,
    input: View,
    list: View,
}

impl Layout {
    pub fn new(preview: Window) -> Result<Self, Error> {
        let input_config = WindowConfig::builder()
            .height(1)
            .split(SplitDirection::Below)
            .focusable(true)
            .build();

        let list_config = WindowConfig::builder()
            .height(16)
            .split(SplitDirection::Below)
            .build();

        Ok(Self {
            preview,
            input: View::new(input_config)?,
            list: View::new(list_config)?,
        })
    }

    pub fn open(&mut self) -> Result<(), Error> {
        self.list.ensure_open(false)?;
        self.input.ensure_open(true)?;
        Ok(())
    }

    pub fn close(&mut self, force: bool) -> Result<(), Error> {
        self.input.close(force)?;
        self.list.close(force)?;
        Ok(())
    }
}
