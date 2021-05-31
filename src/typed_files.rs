use rand::{prelude::*, thread_rng};
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom, Write};

const I32_SIZE: usize = 4;

pub trait IntegerFile {
    fn read_int(&mut self) -> Result<i32>;
    fn write_int(&mut self, value: i32) -> Result<()>;
    fn goto(&mut self, index: u32) -> Result<()>;
    fn get_count(&self) -> Result<usize>;

    fn read_as_array(&mut self) -> Result<Vec<i32>>;
    fn write_array(&mut self, values: Vec<i32>) -> Result<()>;
    fn fill_shuffled(&mut self, capacity: usize) -> Result<()>;
}

impl IntegerFile for File {
    fn read_int(&mut self) -> Result<i32> {
        /*
            Читаем 1 число из файла и переводит курсор
        */
        let mut buffer = [0u8; I32_SIZE];
        self.read_exact(&mut buffer)
            .and_then(|_| Ok(i32::from_le_bytes(buffer)))
    }

    fn write_int(&mut self, value: i32) -> Result<()> {
        /*
            Записывает 1 число и переводит курсор
        */
        self.write_all(&value.to_le_bytes())
    }

    fn goto(&mut self, index: u32) -> Result<()> {
        /*
            Переходит на указынный индекс
        */
        self.seek(SeekFrom::Start(index as u64 * I32_SIZE as u64))
            .and_then(|_| Ok(()))
    }

    fn get_count(&self) -> Result<usize> {
        /*
            Возвращает количество чисел
        */
        self.metadata()
            .and_then(|data| Ok((data.len() / I32_SIZE as u64) as usize))
    }

    fn read_as_array(&mut self) -> Result<Vec<i32>> {
        /*
            Читает все числа в файла и возвращем массив из них
        */
        let length = self.get_count()?;
        let mut array: Vec<i32> = Vec::new();
        for _ in 0..length {
            array.push(self.read_int()?);
        }
        Ok(array)
    }

    fn write_array(&mut self, values: Vec<i32>) -> Result<()> {
        /*
            Записывает весь массив чисел в файл
        */
        for value in values {
            self.write_int(value)?;
        }

        Ok(())
    }

    fn fill_shuffled(&mut self, capacity: usize) -> Result<()> {
        /*
            Заполняет файл рандомным набором чисел
        */
        let mut data: Vec<i32> = (0..capacity as i32).collect();
        let mut rng = thread_rng();
        data.shuffle(&mut rng);
        self.write_array(data)?;
        Ok(())
    }
}
