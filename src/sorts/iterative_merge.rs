use crate::consts::Num;
use crate::sorts::merge::merge_sublists;
use crate::tools::checked_list::CheckedListMut;

pub async fn iterative_merge_sort(list: &mut CheckedListMut<'_, Num>) {
    let len = list.len();
    let mut aux_buffer = Vec::with_capacity(len);

    for i in 1.. {
        let step = 1 << i;
        for j in (0..len).step_by(step) {
            let mut slice = list.slice(j..(j + step).min(len));
            if slice.len() < step / 2 {
                continue
            }
            merge_sublists(&mut slice, step / 2, &mut aux_buffer).await;
        }
        if step > len {
            break
        }
    }
}