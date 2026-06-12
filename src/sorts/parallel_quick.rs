use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::cmp;

pub async fn parallel_quicksort<'a>(list: &'a mut CheckedListMut<'a, Num>) {
    let len = list.len();
    if len < 2 {
        return;
    }
    if len == 2 && cmp(&list[0], &list[1]).await.is_gt() {
        list.swap(0, 1).await;
        return;
    }
    let mut a = 0;
    let mut b = len;
    loop {
        a += 1;
        while a < len && cmp(&list[a], &list[0]).await.is_lt() {
            a += 1;
        }
        b -= 1;
        while b > 0 && cmp(&list[b], &list[0]).await.is_gt() {
            b -= 1;
        }
        if a >= b {
            break
        }
        list.swap(a, b).await;
    };
    list.swap(b, 0).await;
    let (mut fst, mut snd) = list.slice(..).split_at_mut(b+1);
    futures::join!(
        Box::pin(parallel_quicksort(&mut fst)),
        Box::pin(parallel_quicksort(&mut snd)),
    );
}