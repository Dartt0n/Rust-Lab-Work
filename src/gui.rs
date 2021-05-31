extern crate gtk;

use crate::globals::{run_sorts, SORTS};
use gtk::prelude::*;
use std::fs::OpenOptions;
use std::io::Read;

pub fn init_gui(app: &gtk::Application) {
    // создаем главное окно размером 920х850
    let main_window = gtk::ApplicationWindowBuilder::new()
        .width_request(920)
        .height_request(850)
        .build();
    // создаем обьект для размешения остальных виджетов по координатам
    let field = gtk::FixedBuilder::new()
        .width_request(920)
        .height_request(850)
        .build();
    // и добавляем его на основное окно
    main_window.add(&field);
    // создаем выпадающее меню размером 445х50
    let sort_chooser = gtk::ComboBoxTextBuilder::new()
        .width_request(445)
        .height_request(50)
        .build();
    // добавляем вариант "Выберите сортировку" с id "Default"
    sort_chooser.append(Some("Default"), "Выберите сортировку");
    for (index, sort) in SORTS.iter().enumerate() {
        // добавляем как вариант каждую сортировку, id - индекс в глобальном массиве
        sort_chooser.append(Some(&format!("{}", index)), sort.name)
    }
    // по умолчанию сортировка не выбрана
    sort_chooser.set_active_id(Some("Default"));
    // располгаем меню на окне
    field.put(&sort_chooser, 10, 10);
    // добавляем кнопку перерасчета
    let recalculate_button = gtk::ButtonBuilder::new()
        .width_request(445)
        .height_request(50)
        .build();
    // на ней надпись
    recalculate_button.add(&gtk::Label::new(Some("Пересчитать все")));
    // при нажатии на кнопку
    recalculate_button.connect_button_press_event(|_btn, _event| {
        run_sorts(); // запускаем все сортировка
        gtk::Inhibit(false)
    });
    // размешаем на окне
    field.put(&recalculate_button, 465, 10);
    // длина и ширина поля для рисования
    const DRAWING_WIDTH: f64 = 900.0;
    const DRAWING_HEIGHT: f64 = 760.0;
    // создаем облать для рисовния
    let drawing_area = gtk::DrawingAreaBuilder::new()
        .width_request(DRAWING_WIDTH as i32)
        .height_request(DRAWING_HEIGHT as i32)
        .build();
    let cloned_drawing_area = drawing_area.clone();
    
    // когда мы выбираем сортировку в выпадающем меню мы перерисовываем график
    sort_chooser.connect_changed(move |_cmbx| {cloned_drawing_area.queue_draw()});
    // перерисовывание графки
    drawing_area.connect_draw(move |_x, c| {
        // ставим черный цвет
        c.set_source_rgb(0.12, 0.12, 0.12);
        // заполняем все поле черным
        c.rectangle(0.0, 0.0, DRAWING_WIDTH, DRAWING_HEIGHT);
        c.fill();

        // ставим белый цвет
        c.set_source_rgb(1.0, 1.0, 1.0);
        // ширина линии 0.5
        c.set_line_width(0.5);
        // рисуем х
        c.move_to(10.0, 10.0);
        c.line_to(10.0, DRAWING_HEIGHT - 10.0);
        // рисуем y
        c.move_to(10.0, DRAWING_HEIGHT - 10.0);
        c.line_to(DRAWING_WIDTH - 10.0, DRAWING_HEIGHT - 10.0);
        // сохраням все изменения
        c.stroke();
        // изменяем размер шрифта
        c.set_font_size(10.0);

        // подписываем ось х
        let x_step = (DRAWING_WIDTH - 40.0) / 6.0;
        let sizes = [100.0, 500.0, 1000.0, 5000.0, 10000.0, 50000.0];
        for i in 0..6 {
            c.move_to(x_step * (i as f64 + 1.0) - 10.0, DRAWING_HEIGHT - 13.0);
            c.show_text(&format!("{}", sizes[i]));
            c.stroke();
        }
        c.set_line_width(2.0);

        // открываем файл с данными о сортировках
        let mut file = OpenOptions::new().read(true).open("result.csv").unwrap();
        // считываем весь файл в 1 строку
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        // закрываем файл и освобождаем память
        drop(file);
        // для каждой линии в файле
        for line in data.split("\n") {
            // получаем значения
            let values = line.split(":").collect::<Vec<&str>>();
            // имя - первое
            let name = values[0];
            // если оно не совпадает с выбранной сортировкой идем дальше
            if name != sort_chooser.get_active_text().unwrap() {
                continue;
            }
            // переводим данные из файла в массив чисел
            let mut times = Vec::new();
            for i in 1..values.len() {
                times.push(values[i].parse::<i32>().unwrap());
            }
            // высчитываем шаг для маштаба
            let max_time = times.iter().max().unwrap();
            let min_time = times.iter().min().unwrap();
            let delta = max_time - min_time;
            let y_step = (DRAWING_HEIGHT - 20.0) / (delta as f64);

            // ставим рыжий цвет
            c.set_source_rgb(1.0, 1.0, 1.0);

            let mut last_point = (10.0, DRAWING_HEIGHT-10.0);
            
            for (i, time) in times.iter().enumerate() {
                // русуем график
                let x = x_step * (i as f64 + 1.0) - 10.0;
                let y = DRAWING_HEIGHT - 10.0 - y_step * f64::from(*time);
                c.move_to(last_point.0, last_point.1);
                c.line_to(x, y);
                c.move_to(20.0, y);
                // и подписываем точки
                c.set_source_rgb(1.0, 1.0, 1.0);
                c.show_text(&format!("{}", time));
                c.set_source_rgb(1.0, 0.3, 0.0);
                last_point = (x, y);
            }
            // сохраняем изменения и выходим
            c.stroke();
            break;
        }

        gtk::Inhibit(false)
    });
    // добавляем поля на окно
    field.put(&drawing_area, 10, 70);
    // информация для ОС
    main_window.set_application(Some(app));
    // показываем все виджеты
    main_window.show_all();
}
