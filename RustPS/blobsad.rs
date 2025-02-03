Rust 2021

#![no_main]

use std::{
    fs::File,
    io::{BufWriter, Write},
    os::unix::io::FromRawFd,
};

fn solve(stdin: &str, stdout: &mut impl Write) {
    let mut blob = stdin.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = blob.next().unwrap();
    let k = blob.next().unwrap();
    let m = k / 2 + 1;

    let mut mid = 0; let mut cnt = 0; let mut ans = 0;

    for i in 1..n+1 {
        let mut x = blob.next().unwrap();
        if cnt + x < k {
            cnt += x;
            if mid==0 && cnt>=m {
                mid = i;
            }

            if mid>0 {
                ans += (i - mid) * x;
            }
            else {
                ans += cnt;
            }
        }
        else {
            if mid>0 {
              ans += (i - mid) * (k - cnt);
              x -= k-cnt;
              cnt=0;
              mid=0;
            }
            else {
              x -= k-cnt;
              cnt=0;
            }
            
            
            cnt = x%k;
            if cnt >= m {
              mid = i;
            }
            else {
              ans += cnt;
            }
        }
    }
    
    if cnt>0 {
      writeln!(stdout, "blobsad").unwrap();
    }
    else {
      writeln!(stdout, "{ans}", ans = ans).unwrap();
    }
}

#[no_mangle]
unsafe fn main() -> usize {
    let mut stat = [0; 20];
    fstat(0, stat.as_mut_ptr());
    let stdin = mmap(std::ptr::null(), stat[6], 3, 2, 0, 0);
    let mut stdout = BufWriter::with_capacity(1 << 17, { File::from_raw_fd(1) });
    solve(
        std::str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(stdin, stat[6])),
        &mut stdout,
    );
    stdout.flush().ok();
    0
}

#[link(name = "c")]
extern "C" {
    fn fstat(fd: i32, stat: *mut usize);
    fn mmap(addr: *const u8, len: usize, prot: i32, flags: i32, fd: i32, off: isize) -> *mut u8;
}
