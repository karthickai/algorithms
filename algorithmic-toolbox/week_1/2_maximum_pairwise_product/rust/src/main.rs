use std::io;

fn main() -> io::Result<()> {
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("failed to read from stdin");
    let mut size_trimmed = size.trim();
    match size_trimmed.parse::<u32>() {
        Ok(i) => {
            for iter in 0..i {
                println!("{}", iter);
            }
        }
        Err(..) => println!("this was not an integer: {}", size_trimmed),
    }
    Ok(())
}
