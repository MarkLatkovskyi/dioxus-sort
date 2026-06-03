use crate::checked_list::ListAction;
use futures::channel::mpsc::Sender;
use futures::SinkExt;
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct CheckedValue<T: Ord> {
    pub(crate) index: Option<usize>,
    pub(crate) value: T,
    sender: RefCell<Sender<ListAction>>,
}

impl<T: Ord> CheckedValue<T> {
    pub fn new(value: T, index: Option<usize>, sender: Sender<ListAction>) -> Self {
        Self {
            index,
            value,
            sender: RefCell::new(sender),
        }
    }
}

impl<T: Ord> CheckedValue<T> {
    pub async fn cmp(&self, other: &Self) -> Ordering {
        _ = self.sender
            .borrow_mut()
            .send(ListAction::Compare(self.index, other.index))
            .await;
        self.value.cmp(&other.value)
    }
}