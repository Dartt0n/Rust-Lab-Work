use std::fs::File;
use std::io::{read, Seek,SeekFrom, Write, Result};

const INT16: usize = 2;

pub trait IntegerFile {
    fn read(&mut self) -> Result<i16>;
    fn write(&mut self, value: i16) -> Result<()>;
    fn goto(&mut self, index: u32) -> Result<()>;
    fn get_count(&self) -> Result<u32>;

    fn read_all(&mut self) -> Result<Vec<i16>>;
    fn write_all(&mut self, values: Vec<i16>) -> Result<()>;
}