use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::cmp;

pub async fn heapsort(list: &mut CheckedListMut<'_, Num>) {
    let len = list.len();
    let mut start = len /2;
    let mut end = len;

    while end > 1 {
        if start > 0 {
            start -= 1;
        } else {
            end -= 1;
            list.swap(end, 0).await;
        }

        let mut root = start;

        while 2*root + 1 < end {
            let mut child = 2*root + 1;
            if child + 1 < end && cmp(&list[child], &list[child+1]).await.is_lt() {
                child += 1;
            }
            if cmp(&list[root], &list[child]).await.is_lt() {
                list.swap(root, child).await;
                root = child
            } else {
                break
            }
        }
    }
}