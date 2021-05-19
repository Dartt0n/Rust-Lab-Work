use crate::sorts::{bubble_sort, limited_bubble_sort, quick_sort, selection_sort, shaker_sort};
use std::io::Result;
use std::time::Duration;

pub struct Sort {
    pub name: &'static str,
    pub run: fn(&str) -> Result<Duration>,
}

pub const SORTS: [Sort; 5] = [
    Sort {
        name: "Сортировка пузырьком",
        run: bubble_sort,
    },
    Sort {
        name: "Сортировка простым выбором",
        run: selection_sort,
    },
    Sort {
        name: "Сортировка пузырьком с ограничением",
        run: limited_bubble_sort,
    },
    Sort {
        name: "Сортировка перемешиванием",
        run: shaker_sort,
    },
    Sort {
        name: "Быстрая сортировка",
        run: quick_sort,
    },
];
