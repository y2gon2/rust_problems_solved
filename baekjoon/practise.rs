use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let _g = File::open("hello.txt")?;
    let mut write_buf = std::io::BufWriter::new(std::io::stdout().lock());
    let a = "abc".to_string();

    writeln!(write_buf, "{}", a)?;
    Ok(())
}