/// Maximum Pairwise Product
/// Input
/// 3
/// 1 2 3
/// Output
/// 6
use std::io;
use std::convert::TryInto;

fn string_to_int32(value: String) -> Result<i32, ()> {
    let value_trimmed = value.trim();
    match value_trimmed.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(..) => Err(()),
    }
}

fn read_line() -> Result<String, ()> {
    let mut read = String::new();
    io::stdin()
        .read_line(&mut read)
        .expect("failed to read from stdin");
    Ok(read)
}

fn main() -> io::Result<()> {
    let size_string = read_line().expect("unable to read string");
    let size_int = string_to_int32(String::from(size_string)).expect("unable to parse string to int");
    let size = size_int.try_into().unwrap();
    let values = read_line().expect("unable to read string");
    let vec_values: Vec<_> = values.split_ascii_whitespace().collect();
    let mut final_vec: Vec<i32> = Vec::with_capacity(50);

    for i in 0..size {
        let val = string_to_int32(String::from(vec_values[i])).expect("unable to convert string to int");
        final_vec.push(val);
    }

    final_vec.sort();
    let first = final_vec.last().unwrap().clone();
    final_vec.pop();
    let second = final_vec.last().unwrap().clone();
    println!("{:?}", first * second);

    Ok(())
}
