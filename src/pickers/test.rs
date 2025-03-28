use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::{mpsc::UnboundedSender, oneshot};

use nvim_oxi::libuv::AsyncHandle;
use nvim_oxi::{Result, api::Buffer};

use ignore::{WalkBuilder, WalkState};

use crate::list::{List, ListItem};

pub struct TestPickerItem(String);

#[allow(unused_variables)]
impl ListItem for TestPickerItem {
    fn text(&self) -> String {
        self.0.clone()
    }
    fn highlight(&self, buf: &Buffer, line: u32) -> Result<()> {
        Ok(())
    }
}

pub struct TestPicker {
    run_id: Arc<AtomicUsize>,
}

impl TestPicker {
    pub fn new() -> Self {
        Self {
            run_id: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[allow(unused_variables)]
impl List for TestPicker {
    type Item = TestPickerItem;

    fn items(
        &mut self,
        prompt: String,
        handle: AsyncHandle,
        sender: UnboundedSender<Self::Item>,
    ) -> Result<()> {
        let run_id = self.run_id.fetch_add(1, Ordering::SeqCst);

        WalkBuilder::new(".")
            .hidden(false) // include hidden
            .threads(8)
            .build_parallel()
            .run(|| {
                Box::new(|result| {
                    match result {
                        Ok(entry) => {
                            sender
                                .send(TestPickerItem(entry.path().display().to_string()))
                                .unwrap();
                            handle.send().unwrap();
                        }
                        Err(e) => {}
                    };

                    WalkState::Continue
                })
            });

        Ok(())
    }

    fn preview(&self, item: &Self::Item) -> Result<()> {
        Ok(())
    }

    fn select(&self, item: &Self::Item) -> Result<()> {
        Ok(())
    }
}
