use std::fs::File;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::io::Read;


fn main() {
    if !cfg!(target_os = "windows") {
        let mut f: File = unsafe { File::from_raw_fd(0) };
    }
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("I read: \n{}", input);
}
