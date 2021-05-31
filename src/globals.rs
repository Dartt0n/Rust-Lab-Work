use crate::sorts::{bubble_sort, limited_bubble_sort, quick_sort, selection_sort, shaker_sort};
use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
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

pub fn run_sorts() {
    let sizes = [100, 500, 1_000, 5_000, 10_000, 50_000];
    let mut result_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("result.csv")
        .unwrap();
    for sort in &SORTS {
        let mut files = Vec::<String>::new();
        for size in &sizes {
            let name = format!("{}_integers.dat", size);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&name)
                .unwrap();
            file.fill_shuffled(*size).unwrap();
            files.push(name);
        }
        let mut scv_line = String::from(sort.name);
        for size in &sizes {
            match (sort.run)(&format!("{}_integers.dat", size)) {
                Ok(time) => scv_line += &format!(":{}", time.as_micros()),
                Err(e) => println!("{}", e),
            };
        }
        scv_line += "\n";
        result_file.write(scv_line.as_bytes()).unwrap();
    }
}
