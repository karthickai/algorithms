use std::io;


fn string_to_int32(value: String) -> Result<i32, ()> {
    let value_trimmed = value.trim();
    match value_trimmed.parse::<i32>(){
    Ok(i) => Ok(i),
    Err(..) => Err(()),
    }
}

fn main() -> io::Result<()> {
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("failed to read from stdin");
    let size_trimmed = size.trim();
    match size_trimmed.parse::<u32>() {
        Ok(i) => {
            for _iter in 0..i {
                let mut values = String::new();
                io::stdin()
                    .read_line(&mut values)
                    .expect("failed to read from stdin");
                let mut vec_values: Vec<_> = values.split_ascii_whitespace().collect();
                let mut final_vec: Vec<i32> = Vec::with_capacity(50);
                for vec in vec_values {
                    let val = string_to_int32(String::from(vec)).expect("unable to convert string to int");
                    final_vec.push(val);
                }
                println!("{:?}", final_vec);
            }
        }
        Err(..) => println!("this was not an integer: {}", size_trimmed),
    }
    Ok(())
}
