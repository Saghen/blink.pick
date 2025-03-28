use nvim_oxi::api;
use nvim_oxi::{Dictionary, Function};
use pickers::test::TestPicker;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

mod list;
mod picker;
mod pickers;

use picker::Picker;

#[nvim_oxi::plugin]
fn blink_pick() -> nvim_oxi::Result<Dictionary> {
    let picker: Rc<RefCell<Option<Picker<TestPicker>>>> = Rc::default();

    let picker_rc = Rc::clone(&picker);

    let open_window = Function::from_fn(move |()| {
        if picker_rc.borrow().is_none() {
            match Picker::new(Rc::new(Mutex::new(TestPicker::new()))) {
                Ok(picker) => {
                    *picker_rc.borrow_mut() = Some(picker);
                }
                Err(err) => {
                    api::err_writeln(&format!("Failed to create layout: {err}"));
                }
            }
        }
    });

    let close_window = Function::from_fn(move |()| {
        let mut picker = picker.borrow_mut();
        *picker = None;
    });

    let api = Dictionary::from_iter([("open_window", open_window), ("close_window", close_window)]);

    Ok(api)
}
