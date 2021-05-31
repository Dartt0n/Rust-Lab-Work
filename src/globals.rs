/*
    Глобальные переменные используемые в многих местах кода находятся в этом файле
*/

// сортирока
use crate::sorts::{bubble_sort, limited_bubble_sort, quick_sort, selection_sort, shaker_sort};
// типизированные файлы
use crate::typed_files::IntegerFile;
// операции с файлами
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
// правильное вычисление длительности
use std::time::Duration;

// структура описывающая сортировку, содержит имя сортировка, и указатель на саму функцию сортировка
pub struct Sort {
    pub name: &'static str,
    pub run: fn(&str) -> Result<Duration>,
}
// массив всех сортировок
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

// запускает все сортировки
pub fn run_sorts() {
    let sizes = [100, 500, 1_000, 5_000, 10_000, 50_000]; // размеры файлов
    let mut result_file = OpenOptions::new() // открываем файл
        .write(true) // с возможнотью записи
        .create(true) // создания в случае отсуствия
        .truncate(true) // переписывания
        .open("result.csv") // путь до файла
        .unwrap();
    for sort in &SORTS { // для каждой сортировки
        let mut files = Vec::<String>::new(); // массив названий файлов
        for size in &sizes { // для каждого размера блока
            let name = format!("{}_integers.dat", size); // имя файла
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&name)
                .unwrap();
            file.fill_shuffled(*size).unwrap(); // заполняем массив случайно перемещанным набором чисел от 0 до size
            files.push(name); // сохраняем имя файла
        }
        let mut scv_line = String::from(sort.name); // генерируем строку в csv формате
        for size in &sizes { // для каждого файла
            match (sort.run)(&format!("{}_integers.dat", size)) { // вызываем сортировку
                Ok(time) => scv_line += &format!(":{}", time.as_micros()), // если не было ошибок, сохраняем время выполнения в микросекундах
                Err(e) => println!("{}", e), // если возникла ошибка - выводим ее
            };
        }
        scv_line += "\n"; // добавляем симфол перенома строки
        result_file.write(scv_line.as_bytes()).unwrap(); // записываем в файл
    }
}
