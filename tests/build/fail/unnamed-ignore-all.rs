use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct(#[rd_ignore] i32, #[rd_ignore] u32);

fn main() {}