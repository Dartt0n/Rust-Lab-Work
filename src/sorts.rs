use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;


pub fn selection_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;

    for i in 0..length-1 {
        let mut min_index = i;
        for j in i+1..length {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(min_index, i);
    }

    file.write_array(array)?;
    Ok(())
}

pub fn bubble_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(target)?;
    let mut array =file.read_as_array()?;
    let length = file.get_count()?;

    for i in 0..length {
        for j in 0..length {
            if array[i] < array[j] {
                array.swap(i, j);
            }
        }
    }

    file.write_array(array)?;
    Ok(())
}

pub fn limited_bubble_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(target)?;
    let mut i = 0;
    let length = file.get_count()?;
    let mut array = file.read_as_array()?;
    let mut flag = true;
    while flag {
        flag = false;
        for j in 0..length-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
                flag = true;
            }
        }
        i += 1;
    }

    file.write_array(array)?;
    Ok(())
}

pub fn shaker_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;

    loop {
        let mut swapped = false;

        for i in 0..length-1 {
            if array[i] > array[i+1] {
                array.swap(i, i+1);
                swapped = true;
            }
        }

        if !swapped {
            break
        }

        for i in (0..length-1).rev() {
            if array[i] > array[i+1] {
                array.swap(i, i+1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    file.write_array(array)?;
    Ok(())
}