//@ needs-asm-support
#![feature(repr_simd)]

use std::arch::asm;

#[repr(simd)]
//~^ ERROR attribute cannot be used on
//~| ERROR zero-variant enums do not currently support `#[repr]`
enum Es {}

fn main() {
    unsafe {
        let mut x: Es;
        asm!("{}", out(reg) x);
    }
}
