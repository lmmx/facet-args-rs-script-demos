// #!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! facet =    { version = "0.*", default-features = false, features = ["alloc"] }
//! facet-args = { version = "0.*", default-features = false, features = [] }
//! facet-pretty = { version = "0.*", default-features = false, features = ["alloc"] }
//! ```
//! 
#![no_std]
#![no_main]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::{ffi::c_int, slice, str, arch::asm, fmt::Write};
use facet::Facet;
use facet_args::from_slice;
use facet_pretty::FacetPretty;

#[derive(Facet)]
struct HelloArgs {
    // A positional argument for the name to greet
    #[facet(positional)]
    name: String,
    
    // An optional verbose flag
    #[facet(named, short = 'v')]
    verbose: bool,
    
    // Add help flag
    #[facet(named, short = 'h')]
    help: bool,
}


#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *const *const u8) -> c_int {
    // 1) Collect &str slices from the raw argv
    let mut slices: Vec<&str> = Vec::with_capacity((argc - 1) as usize);
    unsafe {
        for i in 1..argc as isize {
            let p = *argv.offset(i) as *const u8;
            let mut len = 0;
            while *p.add(len) != 0 { len += 1; }
            let bytes = slice::from_raw_parts(p, len);
            // SAFETY: the kernel provides UTF-8 argv
            let s = str::from_utf8_unchecked(bytes);
            slices.push(s);
        }
    }

    // 2) Parse into the `HelloArgs` type
    let args: HelloArgs = match from_slice::<HelloArgs>(&slices) {
        Ok(a) => a,
        Err(err) => {
            // serialize the error so we can see it on stderr
            let mut serr = String::new();
            write!(&mut serr, "ERROR: {:?}\n", err).unwrap();
            let b = serr.as_bytes();
            // write to fd=2 (stderr), ignore the return value
            unsafe { syscall_write(2, b.as_ptr(), b.len()); }
            unsafe { syscall_exit(1); }
        }
    };

    // 3) Pretty-print and append a newline
    let mut out = String::new();
    write!(&mut out, "RESULT:{}\n", args.pretty()).unwrap();

    // 4) Write it out via the Linux `write` syscall, then exit
    let bytes = out.as_bytes();
    unsafe {
        syscall_write(1, bytes.as_ptr(), bytes.len()); // fd=1 → stdout
        syscall_exit(0);
    }
}

#[inline(always)]
unsafe fn syscall_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") 1,      // __NR_write
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") len,
        lateout("rax") ret,
        lateout("rcx") _, // clobbered by syscall
        lateout("r11") _, // clobbered by syscall
    );
    ret
}

#[inline(always)]
unsafe fn syscall_exit(code: i32) -> ! {
    asm!(
        "syscall",
        in("rax") 60,     // __NR_exit
        in("rdi") code,
        options(noreturn),
    );
}
