use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;


fn count_in_file(path: &Path) -> io::Result<()> {
    let mut counter = 0;

    let f = File::open(path)?;
    let r = io::BufReader::new(f);

    for b in r.bytes() {
        if b? == b'e' {
            counter += 1;
        }
    }

    println!("count = {}", counter);

    Ok(())
}


fn main() {
    // Skip arg[0], which is the program name.
    let args: Vec<_> = env::args().skip(1).collect();

    let p = Path::new(&args[0]);
    count_in_file(p).ok();
}
