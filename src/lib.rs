use nvim_oxi::api::{self, Window, opts::*, types::*};
use nvim_oxi::{Dictionary, Function, print};

mod layout;

#[nvim_oxi::plugin]
fn blink_pick() -> nvim_oxi::Result<Dictionary> {
    // Create a new `Greetings` command.
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let greetings = |args: CommandArgs| {
        let who = args.args.unwrap_or("from Rust".to_owned());
        let bang = if args.bang { "!" } else { "" };
        print!("Hello {}{}", who, bang);
    };

    api::create_user_command("BlinkPick", greetings, &opts)?;

    // Remaps `hi` to `hello` in insert mode.
    api::set_keymap(Mode::Insert, "hi", "hello", &Default::default())?;

    // Creates two functions `{open,close}_window` to open and close a
    // floating window.

    use std::cell::RefCell;
    use std::rc::Rc;

    let win: Rc<RefCell<Option<Window>>> = Rc::default();

    let w = Rc::clone(&win);

    let open_window: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
        if w.borrow().is_some() {
            api::err_writeln("Window is already open");
            return Ok(());
        }

        let config = WindowConfig::builder()
            .height(20)
            .split(SplitDirection::Below)
            .build();

        let mut win = w.borrow_mut();
        let buf = api::create_buf(false, true)?;

        *win = Some(api::open_win(&buf, true, &config)?);

        Ok(())
    });

    let close_window = Function::from_fn(move |()| {
        if win.borrow().is_none() {
            api::err_writeln("Window is already closed");
            return Ok(());
        }

        let win = win.borrow_mut().take().unwrap();
        win.close(false)
    });

    let api = Dictionary::from_iter([("open_window", open_window), ("close_window", close_window)]);

    Ok(api)
}
