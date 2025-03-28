use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use nvim_oxi::{
    Array, Result,
    api::{
        self, Buffer, Window,
        opts::{BufDeleteOpts, CreateAutocmdOpts, OptionOpts, OptionScope},
        types::{AutocmdCallbackArgs, SplitDirection, WindowConfig},
    },
    libuv::AsyncHandle,
};
use tokio::sync::mpsc;

use crate::list::{List as ListTrait, ListItem};

pub struct Picker<List: ListTrait + 'static> {
    win: Window,
    buf: Buffer,
    list: Rc<Mutex<List>>,
    items: Arc<Mutex<Vec<List::Item>>>,
    on_input_autocmd: u32,
    on_cursor_moved_autocmd: u32,
}

impl<List: ListTrait> Picker<List> {
    pub fn new(list: Rc<Mutex<List>>) -> Result<Self> {
        let buf = Self::make_buf()?;
        let win = Self::make_win(&buf)?;
        let on_input_autocmd = Self::make_on_input_autocmd(&win, list.clone())?;
        let on_cursor_moved_autocmd = Self::make_cursor_moved_autocmd(&win)?;

        let items = Arc::new(Mutex::new(vec![]));
        let cloned_items = items.clone();

        Ok(Self {
            buf,
            win,
            list,
            items: cloned_items,
            on_input_autocmd,
            on_cursor_moved_autocmd,
        })
    }

    // Internal API

    fn make_buf() -> Result<Buffer> {
        let mut buf = api::create_buf(false, true)?;

        let opts = OptionOpts::builder().buffer(buf.clone()).build();

        buf.set_var("completion", false)?;
        api::set_option_value("filetype", "blink_pick_input", &opts)?;
        api::set_option_value("modified", false, &opts)?;
        api::set_option_value("buflisted", false, &opts)?;
        api::set_option_value("bufhidden", "hide", &opts)?;

        Ok(buf)
    }

    fn make_win(buf: &Buffer) -> Result<Window> {
        let win_config = WindowConfig::builder().split(SplitDirection::Below).build();

        let win = api::open_win(buf, true, &win_config)?;

        let opts = OptionOpts::builder()
            .scope(OptionScope::Local)
            .win(win.clone())
            .build();
        api::set_option_value("cursorline", false, &opts)?;
        api::set_option_value("number", false, &opts)?;
        api::set_option_value("relativenumber", false, &opts)?;
        api::set_option_value("fillchars", "horiz:─", &opts)?;
        api::set_option_value("winhighlight", "WinSeparator:Normal", &opts)?;
        api::set_option_value("signcolumn", "no", &opts)?;

        Ok(win)
    }

    fn make_on_input_autocmd(win: &Window, list: Rc<Mutex<List>>) -> Result<u32> {
        let win = win.clone();

        let input_listener = CreateAutocmdOpts::builder()
            .buffer(win.get_buf()?)
            .callback(move |args: AutocmdCallbackArgs| -> Result<bool> {
                let input = args
                    .buffer
                    .get_lines(0..1, true)?
                    .next()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                // TODO: don't run if input didn't change

                let (sender, mut receiver) = mpsc::unbounded_channel::<List::Item>();

                let handle = AsyncHandle::new(move || {
                    let item = receiver.blocking_recv().unwrap();
                    let item_2 = receiver.blocking_recv().unwrap();
                    let item_3 = receiver.blocking_recv().unwrap();
                    nvim_oxi::schedule(move |_| {
                        nvim_oxi::print!("Received item {} from backround thread", item.text());
                        nvim_oxi::print!("Received item {} from backround thread", item_2.text());
                        nvim_oxi::print!("Received item {} from backround thread", item_3.text());
                    });
                })?;

                let _ = list.lock().unwrap().items(input, handle, sender);

                Ok(false)
            })
            .build();

        Ok(api::create_autocmd(
            ["TextChanged", "TextChangedI"],
            &input_listener,
        )?)
    }

    /// Ensures the cursor never leaves the first line of the buffer
    fn make_cursor_moved_autocmd(win: &Window) -> Result<u32> {
        let win = win.clone();
        let cursor_moved_listener = CreateAutocmdOpts::builder()
            .buffer(win.get_buf()?)
            .callback(move |args: AutocmdCallbackArgs| -> Result<bool> {
                let cursor = win.get_cursor()?;
                if cursor.0 > 1 {
                    win.clone().set_cursor(1, cursor.1)?;
                }
                Ok(false)
            })
            .build();

        Ok(api::create_autocmd(
            ["CursorMoved", "CursorMovedI"],
            &cursor_moved_listener,
        )?)
    }
}

impl<List: ListTrait> Drop for Picker<List> {
    fn drop(&mut self) {
        if self.on_input_autocmd != 0 {
            let _ = api::del_autocmd(self.on_input_autocmd);
        }
        if self.on_cursor_moved_autocmd != 0 {
            let _ = api::del_autocmd(self.on_cursor_moved_autocmd);
        }
        if self.win.is_valid() {
            let _ = self.win.clone().close(true);
        }
        if self.buf.is_valid() {
            let _ = self
                .buf
                .clone()
                .delete(&BufDeleteOpts::builder().force(true).build());
        }
    }
}
