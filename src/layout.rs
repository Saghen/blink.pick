use nvim_oxi::api::Window;

struct View {
    pub win: Window,
    pub buf: Window,
}

struct Layout {
    preview: Window,
    input: View,
    list: View,
}

impl Layout {
    fn new(preview: Window) -> Self {
        Self {
            preview,
            input: View {
                win: Window::new(),
                buf: Window::new(),
            },
            list: View {
                win: Window::new(),
                buf: Window::new(),
            },
        }
    }
}
