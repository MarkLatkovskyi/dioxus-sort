use crate::tools::checked_list::ListAction;
use futures::SinkExt;
use futures::channel::mpsc::Sender;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::hint;
pub use CheckedValue as CVal;

#[derive(Clone)]
pub struct CheckedValue<T: Ord> {
    pub(crate) index: Option<usize>,
    pub(crate) value: T,
    sender: RefCell<Sender<ListAction<T>>>,
}

impl<T: Ord> CVal<T> {
    pub fn new(value: T, index: Option<usize>, sender: Sender<ListAction<T>>) -> Self {
        Self {
            index,
            value,
            sender: RefCell::new(sender),
        }
    }
}

impl<T: Ord> CVal<T> {
    pub async fn cmp(&self, other: &Self) -> Ordering {
        _ = self
            .sender
            .borrow_mut()
            .send(ListAction::Compare(self.index, other.index))
            .await;
        self.value.cmp(&other.value)
    }
}


pub async fn cmp<T: Ord>(a: &CVal<T>, b: &CVal<T>) -> Ordering {
    a.cmp(b).await
}

pub trait CValListExt<T: Ord> {
    async fn cval_binary_search(&self, item: &CVal<T>) -> Result<usize, usize> {
        self.cval_binary_search_by(|x| x.cmp(item)).await
    }
    async fn cval_binary_search_by<'a, F, G>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a CVal<T>) -> G,
        G: Future<Output = Ordering>, T: 'a;

    async fn cval_is_sorted_by<'a, F, G>(&'a self, f: F) -> bool
    where
        F: FnMut(&'a CheckedValue<T>, &'a CheckedValue<T>) -> G,
        G: Future<Output = Ordering>, T: 'a;

    async fn cval_is_sorted<'a>(&'a self) -> bool {
        self.cval_is_sorted_by(cmp).await
    }

    async fn cmp(&self, fst: usize, snd: usize) -> Ordering;
}

impl<T: Ord> CValListExt<T> for [CVal<T>] {

    // Yes, this is copy-pasted from `std`
    // I modified it to work with `async`, which I hope it does...
    #[inline]
    async fn cval_binary_search_by<'a, F, G>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a CVal<T>) -> G,
        G: Future<Output = Ordering>,
    {
        let mut size = self.len();
        if size == 0 {
            return Err(0);
        }
        let mut base = 0usize;

        // This loop intentionally doesn't have an early exit if the comparison
        // returns Equal. We want the number of loop iterations to depend *only*
        // on the size of the input slice so that the CPU can reliably predict
        // the loop count.
        while size > 1 {
            let half = size / 2;
            let mid = base + half;

            // SAFETY: the call is made safe by the following invariants:
            // - `mid >= 0`: by definition
            // - `mid < size`: `mid = size / 2 + size / 4 + size / 8 ...`
            let cmp = f(unsafe { self.get_unchecked(mid) }).await;

            // Binary search interacts poorly with branch prediction, so force
            // the compiler to use conditional moves if supported by the target
            // architecture.
            base = hint::select_unpredictable(cmp == Greater, base, mid);

            // This is imprecise in the case where `size` is odd and the
            // comparison returns Greater: the mid element still gets included
            // by `size` even though it's known to be larger than the element
            // being searched for.
            //
            // This is fine though: we gain more performance by keeping the
            // loop iteration count invariant (and thus predictable) than we
            // lose from considering one additional element.
            size -= half;
        }

        // SAFETY: base is always in [0, size) because base <= mid.
        let cmp = f(unsafe { self.get_unchecked(base) }).await;
        if cmp == Equal {
            // SAFETY: same as the `get_unchecked` above.
            unsafe { hint::assert_unchecked(base < self.len()) };
            Ok(base)
        } else {
            let result = base + (cmp == Less) as usize;
            // SAFETY: same as the `get_unchecked` above.
            // Note that this is `<=`, unlike the assume in the `Ok` path.
            unsafe { hint::assert_unchecked(result <= self.len()) };
            Err(result)
        }
    }

    async fn cval_is_sorted_by<'a, F, G>(&'a self, mut f: F) -> bool
    where
        F: FnMut(&'a CheckedValue<T>, &'a CheckedValue<T>) -> G,
        G: Future<Output=Ordering>,
        T: 'a
    {
        for i in 1..self.len() {
            if f(&self[i-1], &self[i]).await.is_gt() {
                return false
            }
        }
        true
    }

    async fn cmp(&self, fst: usize, snd: usize) -> Ordering {
        cmp(&self[fst], &self[snd]).await
    }
}