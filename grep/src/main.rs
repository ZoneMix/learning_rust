use std::fs::File;
use std::io::Read;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};

#[cfg(windows)]
use { 
    kernel32::GetStdHandle,
    std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle},
    winapi::*,
};

fn main() {
    #[cfg(unix)]
    let mut f: File = get_linux_stdin();

    #[cfg(windows)]
    let mut f: File = get_win_stdin();

    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    println!("I read: \n{}", input);
}

#[cfg(windows)]
fn get_win_stdin() -> File {
    let handle: RawHandle = unsafe { GetStdHandle(winapi::winbase::STD_INPUT_HANDLE) };
    unsafe { File::from_raw_handle(handle) }
}

#[cfg(unix)]
fn get_linux_stdin() -> File {
    unsafe { File::from_raw_fd(0) }
}