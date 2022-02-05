use ref_split::*;

#[ref_split(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct(#[rs_ignore] i32, #[rs_ignore] u32);

fn main() {}