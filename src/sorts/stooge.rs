use crate::consts::Num;
use crate::tools::checked_list::CheckedListMut;

pub async fn stooge_sort(list: &mut CheckedListMut<'_, Num>) {
    // println!("Starting!");
    match list.len() {
        0 | 1 => (),
        2 => {
            if list[0].cmp(&list[1]).await.is_gt() {
                list.swap(0, 1).await;
            }
        }
        len => {
            Box::pin(stooge_sort(&mut list.slice(..len - len / 3))).await;
            Box::pin(stooge_sort(&mut list.slice(len / 3..))).await;
            Box::pin(stooge_sort(&mut list.slice(..len - len / 3))).await;
        }
    }
}
