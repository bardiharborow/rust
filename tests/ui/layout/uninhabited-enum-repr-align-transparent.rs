//! An uninhabited enum with `repr(align(N > 1))` is not a 1-ZST, even if every
//! variant is absent, so it cannot be a trivial field of a `repr(transparent)` type
//! ([layout.repr.transparent.constraint-field] requires size 0 *and* alignment 1).

enum Never {}

#[repr(align(4))]
enum OneVariant {
    V(Never),
}

#[repr(transparent)]
struct Wrapper(u32, OneVariant);
//~^ ERROR transparent struct needs at most one non-trivial field, but has 2

fn main() {}
