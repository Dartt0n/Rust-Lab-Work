// подключаем все используемые модули
mod globals;
mod gui;
mod sorts;
mod typed_files;

// графическая библиотека
extern crate gtk;
use gio::prelude::*;

// импортируем функцию инициализации интерфейса
use crate::gui::init_gui;

fn main() {
    // Создаем обьект приложения с флагами по умолчанию
    let app = gtk::Application::new(None, Default::default())
        // при возникновении ошибки сообщаем об этом пользователю и выходим
        .expect("Failed to start application"); 
    
        // при старте приложение запустит функцию init_gui
    app.connect_activate(init_gui);
    // запускаем приложения с переданными аргуменатми
    app.run(&std::env::args().collect::<Vec<_>>());  
}
