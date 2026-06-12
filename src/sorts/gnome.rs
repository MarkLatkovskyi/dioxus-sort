use crate::Num;
use crate::tools::checked_list::CheckedListMut;

pub async fn gnome_sort(list: &mut CheckedListMut<'_, Num>) {
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
