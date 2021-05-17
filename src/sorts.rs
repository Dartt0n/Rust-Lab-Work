use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;

pub fn selection_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;
    drop(file);

    for i in 0..length - 1 {
        let mut min_index = i;
        for j in i + 1..length {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(min_index, i);
    }
    let mut file = OpenOptions::new().truncate(true).write(true).open(target)?;
    file.write_array(array)?;
    Ok(())
}

pub fn bubble_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;
    drop(file);
    for i in 0..length {
        for j in 0..length {
            if array[i] < array[j] {
                array.swap(i, j);
            }
        }
    }
    let mut file = OpenOptions::new().truncate(true).write(true).open(target)?;
    file.write_array(array)?;
    Ok(())
}

pub fn limited_bubble_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(target)?;
    let mut i = 0;
    let length = file.get_count()?;
    let mut array = file.read_as_array()?;
    drop(file);
    let mut flag = true;
    while flag {
        flag = false;
        for j in 0..length - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                flag = true;
            }
        }
        i += 1;
    }
    let mut file = OpenOptions::new().truncate(true).write(true).open(target)?;
    file.write_array(array)?;
    Ok(())
}

pub fn shaker_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;
    drop(file);
    loop {
        let mut swapped = false;

        for i in 0..length - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        for i in (0..length - 1).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    let mut file = OpenOptions::new().truncate(true).write(true).open(target)?;
    file.write_array(array)?;
    Ok(())
}

fn quick_sort_rec(mut array: &mut Vec<i32>, low: usize, high: usize) -> Result<()> {
    let mut i = low as i32;
    let mut j = high as i32;
    let pivot = array[(i+j) as usize/2];

    while i <= j {
        while array[i as usize] < pivot {
            i += 1;
        }
        while array[j as usize] > pivot {
            j -= 1;
        }

        if i <= j {
            array.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
        if j > low as i32 {
            quick_sort_rec(&mut array, low, j as usize)?;
        }
        if i < high as i32 {
            quick_sort_rec(&mut array, i as usize, high)?;
        }
    }

    Ok(())
}

pub fn quick_sort(target: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(target)?;
    let mut array = file.read_as_array()?;
    let length = file.get_count()?;
    drop(file);
    quick_sort_rec(&mut array, 0, length-1)?;
    let mut file = OpenOptions::new().truncate(true).write(true).open(target)?;
    file.write_array(array)?;
    Ok(())
}

