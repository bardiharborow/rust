//@ check-pass
//! `repr(align(N))` must be respected even if every variant of the enum is absent
//! (uninhabited with only 1-ZST fields).
#![allow(dead_code)]

use std::convert::Infallible;
use std::mem::{align_of, size_of};

enum Never {}

#[repr(align(4))]
enum OneVariant {
    V(Never),
}

#[repr(align(8))]
enum TwoVariants {
    A(Never),
    B(Never, [u8; 0]),
}

enum OneVariantUnaligned {
    V(Never),
}

#[repr(align(4))]
struct OneVariantNewtype(OneVariantUnaligned);

enum TwoVariantsUnaligned {
    A(Never),
    B(Never, [u8; 0]),
}

#[repr(align(8))]
struct TwoVariantsNewtype(TwoVariantsUnaligned);

#[repr(align(4))]
enum OneVariantPlusUnit {
    U,
    V(Never),
}

#[repr(align(1))]
enum AlignOne {
    V(Never),
}

enum NoRepr {
    V(Never),
}

// ---------------------------------------------------------------------------
// Guaranteed by the reference.
// ---------------------------------------------------------------------------

// [layout.repr.alignment.enum]: `align` on an enum has the same effect on its alignment as
// wrapping the enum (without the modifier) in a newtype struct with the same `align` modifier.
const _: () = assert!(align_of::<OneVariant>() == align_of::<OneVariantNewtype>());
const _: () = assert!(align_of::<TwoVariants>() == align_of::<TwoVariantsNewtype>());

// [layout.repr.alignment.align]: `align` raises the alignment to the requested value.
const _: () = assert!(align_of::<OneVariant>() == 4);
const _: () = assert!(align_of::<TwoVariants>() == 8);

// [layout.repr.rust.enum-struct-like-zst]: an enum with a single variant whose fields are all
// zero sized is itself zero sized (`align` is not a primitive representation).
const _: () = assert!(size_of::<OneVariant>() == 0);
const _: () = assert!(size_of::<OneVariant>() == size_of::<OneVariantNewtype>());

// [layout.repr.rust.layout]: a type is at least as aligned as its fields, so the over-aligned
// enum raises the alignment of an enum containing it. The size is then a nonzero multiple of
// that alignment, since the `Ok(u8)` variant needs at least one byte.
const _: () = assert!(align_of::<Result<u8, OneVariant>>() >= 4);
const _: () = assert!(size_of::<Result<u8, OneVariant>>() >= 4);

// ---------------------------------------------------------------------------
// Current rustc behavior, not guaranteed by the reference. These pin that absent variants
// still contribute nothing to the layout, and that enums without `repr(align(N > 1))` are
// unaffected by the fix.
// ---------------------------------------------------------------------------

// Multi-variant enums whose variants are all absent are zero sized.
const _: () = assert!(size_of::<TwoVariants>() == 0);
const _: () = assert!(size_of::<TwoVariants>() == size_of::<TwoVariantsNewtype>());

// Adding an inhabited unit variant does not change the size or alignment.
const _: () = assert!(align_of::<OneVariant>() == align_of::<OneVariantPlusUnit>());
const _: () = assert!(size_of::<OneVariant>() == size_of::<OneVariantPlusUnit>());

// Without `repr(align)`, or with `repr(align(1))`, uninhabited enums remain 1-ZSTs...
const _: () = assert!(align_of::<Never>() == 1);
const _: () = assert!(size_of::<Never>() == 0);
const _: () = assert!(align_of::<Infallible>() == 1);
const _: () = assert!(size_of::<Infallible>() == 0);
const _: () = assert!(align_of::<NoRepr>() == 1);
const _: () = assert!(size_of::<NoRepr>() == 0);
const _: () = assert!(align_of::<AlignOne>() == 1);
const _: () = assert!(size_of::<AlignOne>() == 0);

// ...so they can still be trivial fields of `repr(transparent)` types...
#[repr(transparent)]
struct Wrapper(u32, Never, Infallible, NoRepr, AlignOne);

// ...and variants containing them are still absent.
const _: () = assert!(align_of::<Result<u8, AlignOne>>() == 1);
const _: () = assert!(size_of::<Result<u8, AlignOne>>() == 1);
const _: () = assert!(size_of::<Result<u8, Infallible>>() == 1);

fn main() {}
