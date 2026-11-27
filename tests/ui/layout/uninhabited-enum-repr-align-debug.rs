//@ normalize-stderr: "randomization_seed: \d+" -> "randomization_seed: $$SEED"
//! Pins the full layout of enums whose variants are all absent but that carry
//! `repr(align(N))`: `align` and `max_repr_align` are N, while
//! `unadjusted_abi_align` excludes the `repr(align)` part.
#![feature(rustc_attrs)]
#![crate_type = "lib"]

enum Never {}

#[rustc_dump_layout(debug)]
#[repr(align(4))]
enum OneVariant { //~ ERROR: layout_of
    V(Never),
}
