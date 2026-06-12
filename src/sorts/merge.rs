use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::{cmp, CVal};

pub async fn merge_sort<'a>(list: &'a mut CheckedListMut<'a, Num>) {
    let len = list.len();
    let mut aux_buffer = Vec::with_capacity(len);
    if len < 2 {
        return;
    }

    let (mut fst, mut snd) = list.slice(..).split_at_mut(len / 2);

    Box::pin(merge_sort(&mut fst)).await;
    Box::pin(merge_sort(&mut snd)).await;

    merge_sublists(list, len/2, &mut aux_buffer).await;
}

pub async fn merge_sublists(list: &mut CheckedListMut<'_, Num>, split: usize, aux_buffer: &mut Vec<CVal<Num>>) {
    let len = list.len();

    let mut a = 0;
    let mut b = split;
    loop {
        if a >= split {
            break
        }
        if b >= len {
            while a < split {
                aux_buffer.push(list.read(a).await);
                a += 1;
            }
            break
        }
        if cmp(&list[a], &list[b]).await.is_le() {
            aux_buffer.push(list.read(a).await);
            a += 1;
        } else {
            aux_buffer.push(list.read(b).await);
            b += 1;
        }
    }
    for (i, x) in aux_buffer.drain(..).enumerate() {
        list.replace(i, x).await;
    }
}