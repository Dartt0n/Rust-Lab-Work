mod sorts;
mod typed_files;

use crate::sorts::{bubble_sort, limited_bubble_sort, selection_sort, shaker_sort};
use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;
use std::time::{Duration, Instant};

fn run_with_timer(function: fn(&str) -> Result<()>, target: &str) -> Result<Duration> {
    let start = Instant::now();
    function(target)?;
    Ok(start.elapsed())
}

struct Sort {
    name: &'static str,
    run: fn(&str) -> Result<()>,
}

fn run_sorts() {
    let sorts = [
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
    ];
    let sizes = [100, 500, 1_000, 5_000, 10_000, 50_000];

    for sort in &sorts {
        let mut files = Vec::<String>::new();
        for size in &sizes {
            let name = format!("data/d{}.dat", size);
            let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&name).unwrap();
            file.fill_shuffled(*size).unwrap();
            files.push(name);
        }
        for file in files {
            match run_with_timer(sort.run, &file) {
                Ok(time) => println!("Сортировка: \"{}\" Файл: \"{}\" Время: {} мс", sort.name, file, time.as_millis()),

                Err(e) => println!("Сортировка: \"{}\" Файл: \"{}\" Ошибка: \"{}\"", sort.name, file, e),
            };
        }
    }
}

fn main() {
    run_sorts();
}
