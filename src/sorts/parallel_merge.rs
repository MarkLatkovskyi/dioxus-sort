use crate::consts::Num;
use crate::sorts::merge::merge_sublists;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::{cmp, CVal};

pub async fn parallel_merge_sort<'a>(list: &'a mut CheckedListMut<'a, Num>) {
    let len = list.len();
    let mut aux_buffer = Vec::with_capacity(len);
    if len < 2 {
        return;
    }

    let (mut fst, mut snd) = list.slice(..).split_at_mut(len / 2);

    let fst = Box::pin(parallel_merge_sort(&mut fst));
    let snd = Box::pin(parallel_merge_sort(&mut snd));

    futures::join!(fst, snd);

    merge_sublists(list, len/2, &mut aux_buffer).await;
}