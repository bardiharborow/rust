//@ compile-flags: -W repeated-reprs
// issue #100631, make sure `TyCtxt::get_attr` only called by case that compiler
// can reasonably deal with multiple attributes.
// `repr` will use `TyCtxt::get_attrs` since it's `DuplicatesOk`.
#[repr(C)] //~ ERROR: zero-variant enums do not currently support `#[repr]` [E0084]
//~^ WARN `#[repr(..)]` attribute is specified more than once [repeated_reprs]
#[repr(C)]
enum Foo {}

fn main() {}
