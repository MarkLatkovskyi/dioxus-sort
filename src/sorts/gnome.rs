use crate::checked_list::CheckedList;
use crate::Num;

pub async fn gnome_sort(list: &mut CheckedList<Num>) {
    let mut ptr = 0;
    while ptr + 1 < list.len() {
        if list[ptr].cmp(&list[ptr + 1]).await.is_gt() {
            list.swap(ptr, ptr + 1).await;
            ptr = ptr.saturating_sub(1);
        } else {
            ptr += 1;
        }
    }
}
