use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct(i32, #[rd_ignore(ref, mut)] u64);

fn main() {}