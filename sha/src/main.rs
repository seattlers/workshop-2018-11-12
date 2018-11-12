extern crate ring;

use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use ring::digest;


/// Read a file at `path` and compute its SHA-256 digest.
fn print_file_sha256_digest(path: &Path) -> io::Result<()> {
    let mut buf = [0u8; 4096];
    let f = fs::File::open(path)?;
    let mut r = io::BufReader::new(f);

    let mut ctx = digest::Context::new(&digest::SHA256);

    loop {
        // Read some more bytes from the file at `path`, storing them in `buf`.
        // Remember: `count` may be strictly less than `buf.len()`!
        let count = r.read(&mut buf)?;

        // Check if we read zero bytes. If so, we hit EOF. We didn't read any
        // data, so we don't have anything to feed to to the digest context, so
        // we're all done.
        if count == 0 {
            break;
        }

        // Update the digest by feeding it onl ythe part of `buf` that contains
        // new bytes.
        ctx.update(&buf[..count]);
    }

    let d = ctx.finish();

    println!("{:?}", d);

    Ok(())
}


fn main() {
    // Skip arg[0], which is the program name.
    let args: Vec<_> = env::args().skip(1).collect();

    for path in args {
        // NOTE: we are silently ignoring failures, e.g. bad paths.
        let _ = print_file_sha256_digest(Path::new(&path));
    }
}
