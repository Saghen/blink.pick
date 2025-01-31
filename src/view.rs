use nvim_oxi::api::{self, Buffer, Error, Window, types::WindowConfig};

pub struct View {
    win_config: WindowConfig,
    win: Option<Window>,
    buf: Buffer,
}

impl View {
    pub fn new(win_config: WindowConfig) -> Result<Self, Error> {
        Ok(Self {
            win_config,
            win: None,
            buf: api::create_buf(false, true)?,
        })
    }
}

impl View {
    pub fn get_buf(&mut self) -> Result<Buffer, Error> {
        if !self.buf.is_valid() {
            self.buf = api::create_buf(false, true)?;
        }
        Ok(self.buf.clone())
    }

    pub fn ensure_open(&mut self, focus: bool) -> Result<(), Error> {
        let buf = self.get_buf()?;

        let win_config = self.win_config.clone();
        let win = api::open_win(&buf, focus, &win_config)?;
        self.win = Some(win);

        Ok(())
    }

    pub fn close(&mut self, force: bool) -> Result<(), Error> {
        if let Some(win) = self.win.as_mut() {
            if win.is_valid() {
                win.clone().close(force)?;
            }
        }
        self.win = None;
        Ok(())
    }
}
