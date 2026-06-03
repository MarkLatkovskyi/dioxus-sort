use crate::checked_value::CheckedValue;
use std::ops::{Deref, Index};
use futures::channel::mpsc::Sender;
use futures::SinkExt;

pub struct CheckedList<T: Ord> {
    inner: Vec<CheckedValue<T>>,
    sender: Sender<ListAction>,
    lo: usize,
    hi: usize,
}

impl<T: Ord> Deref for CheckedList<T> {
    type Target = [CheckedValue<T>];

    fn deref(&self) -> &Self::Target {
        &self.inner[self.lo..self.hi]
    }
}

impl<T: Ord> CheckedList<T> {
    pub async fn swap(&mut self, a: usize, b: usize) {
        let l = &mut self.inner[self.lo..self.hi];
        l.swap(a, b);
        (l[a].index, l[b].index) = (l[b].index, l[a].index);
        _ = self.sender.send(ListAction::Swap(a, b)).await;
    }

    pub fn new(list: Vec<T>, sender: Sender<ListAction>) -> Self {
        let len = list.len();
        let inner: Vec<CheckedValue<_>> = list.into_iter().enumerate().map(|(i, x)| {
            CheckedValue::new(x, Some(i), sender.clone())
        }).collect();
        Self {
            lo: 0,
            hi: len,
            inner,
            sender,
        }
    }
}

impl<T: Ord> Index<usize> for CheckedList<T> {
    type Output = CheckedValue<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner[self.lo..self.hi].index(index)
    }
}

#[derive(Copy, Clone)]
pub enum ListAction {
    Swap(usize, usize),
    Compare(Option<usize>, Option<usize>),
}

impl ListAction {}
