mod typed_files;
use std::fs::OpenOptions;
use crate::typed_files::IntegerFile;

fn main() {
    let mut file = OpenOptions::new().write(true).create_new(true).open("data/test.dat").unwrap();
    file.fill_shuffled(10);
}
