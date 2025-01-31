use layout::Layout;
use nvim_oxi::api;
use nvim_oxi::{Dictionary, Function};
use std::cell::RefCell;
use std::rc::Rc;

mod layout;
mod view;

#[nvim_oxi::plugin]
fn blink_pick() -> nvim_oxi::Result<Dictionary> {
    let layout: Rc<RefCell<Option<Layout>>> = Rc::default();

    let layout_rc = Rc::clone(&layout);

    let open_window = Function::from_fn(move |()| {
        if layout_rc.borrow().is_none() {
            match Layout::new(api::get_current_win()) {
                Ok(layout) => {
                    *layout_rc.borrow_mut() = Some(layout);
                }
                Err(err) => {
                    api::err_writeln(&format!("Failed to create layout: {err}"));
                    return;
                }
            }
        }

        if let Err(err) = layout_rc.borrow_mut().as_mut().unwrap().open() {
            api::err_writeln(&format!("Failed to open window: {err}"));
        }
    });

    let close_window = Function::from_fn(move |()| {
        if layout.borrow().is_none() {
            api::err_writeln("Window is already closed");
            return;
        }

        if let Err(err) = layout.borrow_mut().as_mut().unwrap().close(false) {
            api::err_writeln(&format!("Failed to close window: {err}"));
        }
    });

    let api = Dictionary::from_iter([("open_window", open_window), ("close_window", close_window)]);

    Ok(api)
}
