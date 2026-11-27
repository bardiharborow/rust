#[repr(i32)] //~ ERROR: E0084
enum Foo {}

#[repr(Rust)] //~ ERROR: E0084
enum Bar {}

#[repr(align(8))] //~ ERROR: E0084
enum Baz {}

fn main() {
}
