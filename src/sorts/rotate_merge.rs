use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::CValListExt;

pub async fn rotate_merge_sort(list: &mut CheckedListMut<'_, Num>) {
    let len = list.len();

    for i in 1.. {
        let step = 1 << i;
        for j in (0..len).step_by(step) {
            let mut slice = list.slice(j..(j + step).min(len));
            if slice.len() < step / 2 {
                continue
            }
            rotate_merge_sublists(&mut slice, step / 2).await;
        }
        if step > len {
            break
        }
    }
}

pub async fn rotate_merge_sublists<'a>(list: &'a mut CheckedListMut<'a, Num>, mid: usize) {
    if list.len() < 2 || list.len() <= mid || mid == 0 {
        return;
    }
    // debug_assert!((**list)[..mid].cval_is_sorted().await, "first half not sorted");
    // debug_assert!((**list)[mid..].cval_is_sorted().await, "first half not sorted");

    let index1 = mid/2;

    let index2 = (**list)[mid..].cval_binary_search(&list[index1]).await
        .unwrap_or_else(|x| x) + mid;


    list.slice(index1..index2).rotate_left(mid-index1).await;

    let search_index = if index2 == list.len() {
        index2 - 1
    } else {
        index2
    };
    let split_index = (**list)[index1..index2].cval_binary_search(&list[search_index]).await
        .unwrap_or_else(|x| x);

    let (mut fst, mut snd) = list.slice(..).split_at_mut(split_index);

    futures::join!(
        Box::pin(rotate_merge_sublists(&mut fst, index1)),
        Box::pin(rotate_merge_sublists(&mut snd, index2)),
    );
    println!("Done! ({})", list.len());
}