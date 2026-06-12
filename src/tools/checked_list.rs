use crate::tools::checked_value::{CVal, CheckedValue};
use crate::tools::misc::gcd;
use futures::channel::mpsc::Sender;
use futures::SinkExt;
use std::ops::{Deref, Index};
use std::slice::SliceIndex;
use std::mem;

pub struct CheckedList<T: Ord> {
    inner: Box<[CVal<T>]>,
    sender: Sender<ListAction<T>>,
}

impl<T: Ord> CheckedList<T> {
    pub fn new(list: impl IntoIterator<Item = T>, sender: Sender<ListAction<T>>) -> Self {
        let inner: Vec<CVal<_>> = list
            .into_iter()
            .enumerate()
            .map(|(i, x)| CVal::new(x, Some(i), sender.clone()))
            .collect();
        let inner = inner.into_boxed_slice();
        Self { inner, sender }
    }
    
    pub fn as_mut(&mut self) -> CheckedListMut<'_, T> {
        CheckedListMut {
            inner: &mut self.inner,
            sender: self.sender.clone(),
        }
    }
}

pub struct CheckedListMut<'a, T: Ord> {
    inner: &'a mut [CVal<T>],
    sender: Sender<ListAction<T>>,
}

impl<T: Ord> Deref for CheckedListMut<'_, T> {
    type Target = [CVal<T>];

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T: Ord> CheckedListMut<'_, T> {
    pub async fn swap(&mut self, a: usize, b: usize) {
        let l = &mut *self.inner;
        l.swap(a, b);
        (l[a].index, l[b].index) = (l[b].index, l[a].index);
        _ = self
            .sender
            .send(ListAction::Swap(l[a].index.unwrap(), l[b].index.unwrap()))
            .await;
    }
    pub async fn replace(&mut self, idx: usize, val: CheckedValue<T>) -> CheckedValue<T> where T: Clone {
        let ret = mem::replace(&mut self.inner[idx].value, val.value.clone());
        _ = self.sender.send(ListAction::Write(self.inner[idx].index.unwrap(), val.value)).await;
        CheckedValue::new(ret, None, self.sender.clone())
    }

    pub async fn read(&mut self, idx: usize) -> CheckedValue<T> where T: Clone {
        CheckedValue::new(self.inner[idx].value.clone(), None, self.sender.clone())
    }

    pub fn slice(
        &mut self,
        slice: impl SliceIndex<[CVal<T>], Output = [CVal<T>]>,
    ) -> CheckedListMut<'_, T> {
        CheckedListMut {
            inner: &mut self.inner[slice],
            sender: self.sender.clone(),
        }
    }

    pub fn split_at_mut(self, mid: usize) -> (Self, Self) {
        let (fst, snd) = self.inner.split_at_mut(mid);
        let fst = Self {
            inner: fst,
            sender: self.sender.clone(),
        };
        let snd=  Self {
            inner: snd,
            sender: self.sender.clone(),
        };
        (fst, snd)
    }

    pub async fn rotate_left(&mut self, mid: usize) {
        let len = self.len();
        let gcd = gcd(len, mid);
        for starting_i in 0..gcd {
            let mut i = starting_i;
            loop {
                let new_i = (i + mid) % len;
                if new_i == starting_i {
                    break
                }

                self.swap(i, new_i).await;
                i = new_i;
            }
        }
    }
}

impl<T: Ord> Index<usize> for CheckedListMut<'_, T> {
    type Output = CVal<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

#[derive(Copy, Clone)]
pub enum ListAction<T> {
    Swap(usize, usize),
    Compare(Option<usize>, Option<usize>),
    Write(usize, T)
}
