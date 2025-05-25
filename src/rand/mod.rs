//! 乱数
use std::arch::asm;

/// `u64`型の範囲で乱数を返す。
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn rand() -> u64 {
    let b = [0_u8; 8];

    unsafe {
        asm!(
            "syscall",
            in("rax") 0x13e,
            in("rdi") b.as_ptr(),
            in("rsi") b.len(),
            in("rdx") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
            options(nostack),
        );
    }

    u64::from_be_bytes(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s: u64 = 0;

        for _ in 0..1000000 {
            let a = rand();
            s = s.wrapping_add(a);
        }

        dbg!(s);
    }
}
