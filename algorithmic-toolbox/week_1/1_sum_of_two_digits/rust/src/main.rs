/// Sum of two digits
/// input 5 6
/// output 11
use std::io;

fn main() -> io::Result<()> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut iter = input_text.split_ascii_whitespace();
    let mut sum = 0;
    loop {
        match iter.next() {
            Some(x) => {
                let trimmed = x.trim();
                match trimmed.parse::<u32>() {
                    Ok(i) => sum = sum + i,
                    Err(..) => println!("this was not an integer: {}", trimmed),
                };
            }
            None => break,
        }
    }
    println!("{}", sum);

    Ok(())
}
