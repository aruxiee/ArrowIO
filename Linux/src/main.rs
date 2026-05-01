#![no_std]
#![no_main]

use core::panic::PanicInfo;

const AT_FDCWD: i32 = -100;
const SYS_OPENAT: usize = 257;
const SYS_WRITE: usize = 1;
const SYS_CLOSE: usize = 3;
const SYS_EXIT: usize = 60;
const FLAGS: usize = 577; 
const MODE: usize = 0o644;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    syscall_exit(1);
}

fn syscall_write(fd: i64, buf: &[u8]) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _,
            out("r11") _,
        );
    }
}

fn syscall_exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let path = b"call_res.txt\0";
    let content = b"Log Entry: Evasion check passed via direct syscall.\n";

    let fd: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_OPENAT,
            in("rdi") AT_FDCWD,
            in("rsi") path.as_ptr(),
            in("rdx") FLAGS,
            in("r10") MODE,
            lateout("rax") fd,
            out("rcx") _,
            out("r11") _,
        );
    }

    if fd < 0 {
        syscall_exit(1);
    }

    syscall_write(fd, content);

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_CLOSE,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
        );
    }

    syscall_exit(0);
}
