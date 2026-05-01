#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    unsafe {
        core::arch::asm!(
            "mov r10, rcx",
            "syscall",
            in("rax") 0x2cu32,
            in("rcx") -1i64,
            in("rdx") 0u64,
        );
    }

    loop {}
}
