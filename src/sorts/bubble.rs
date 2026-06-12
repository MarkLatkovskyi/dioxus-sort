use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;
use crate::tools::checked_value::cmp;

pub async fn bubble_sort(list: &mut CheckedListMut<'_, Num>) {
    for i in (0..list.len()).rev() {
        for j in 0..i {
            if cmp(&list[j], &list[j+1]).await.is_gt() {
                list.swap(j, j+1).await;
            }
        }
    }
}