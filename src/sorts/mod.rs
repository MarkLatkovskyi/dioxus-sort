mod gnome;
mod stooge;
mod bubble;
mod quick;
mod merge;
mod heap;
mod iterative_merge;
pub mod rotate_merge;
mod parallel_merge;
mod parallel_quick;
// TODO
// mod smooth;

use crate::consts::Num;
use crate::sort_type;
use crate::sorts::bubble::bubble_sort;
use crate::sorts::gnome::gnome_sort;
use crate::sorts::heap::heapsort;
use crate::sorts::iterative_merge::iterative_merge_sort;
use crate::sorts::merge::merge_sort;
use crate::sorts::parallel_merge::parallel_merge_sort;
use crate::sorts::quick::quicksort;
use crate::sorts::parallel_quick::parallel_quicksort;
use crate::sorts::rotate_merge::rotate_merge_sort;
use crate::sorts::stooge::stooge_sort;
use crate::tools::checked_list::CheckedListMut;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use dioxus::core::{AttributeValue, IntoAttributeValue};
use thiserror::Error;

sort_type! {
    Gnome => gnome_sort,
    Stooge => stooge_sort,
    Bubble => bubble_sort,
    Quick => quicksort,
    ParallelQuick => parallel_quicksort,
    Merge => merge_sort,
    Heap => heapsort,
    ParallelMerge => parallel_merge_sort,
    IterativeMerge => iterative_merge_sort,
    RotateMerge => rotate_merge_sort,
}

#[derive(Error, Debug)]
pub enum SortParseError {
    #[error("Couldn't parse as isize")]
    IntParsing(#[from] ParseIntError),
    #[error(transparent)]
    NoSuchSort(#[from] NoSuchSort),
}

impl FromStr for SortType {
    type Err = SortParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<isize>()?.try_into()?)
    }
}

impl Display for SortType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SortType::Gnome => "Gnome sort",
            SortType::Stooge => "Stooge sort",
            SortType::Bubble => "Bubble sort",
            SortType::Quick => "Quicksort",
            SortType::ParallelQuick => "Parallel Quicksort",
            SortType::Merge => "Merge sort",
            SortType::Heap => "Heapsort",
            SortType::ParallelMerge => "Parallel Merge sort",
            SortType::IterativeMerge => "Iterative Merge sort",
            SortType::RotateMerge => "Rotate Merge sort",
        };
        f.write_str(s)
    }
}


#[derive(Error, Debug)]
#[error("No sort with id {id}")]
pub struct NoSuchSort {
    id: isize,
}

impl From<isize> for NoSuchSort {
    fn from(value: isize) -> Self {
        Self {
            id: value,
        }
    }
}

#[macro_export]
macro_rules! sort_type {
    ($($variant:ident => $function:ident,)*) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum SortType {
            $($variant),*
        }

        impl ::std::convert::TryFrom<isize> for SortType {
            type Error = NoSuchSort;

            fn try_from(value: isize) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == Self::$variant as isize => Ok(Self::$variant),)*
                    _ => Err(value.into())
                }
            }
        }

        impl SortType {
            pub async fn start(self, mut list: CheckedListMut<'_, Num>) {
                match self {
                    $(Self::$variant => $function(&mut list).await),*
                }
            }
        }
    }
}

impl IntoAttributeValue for SortType {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Int(self as isize as i64)
    }
}

