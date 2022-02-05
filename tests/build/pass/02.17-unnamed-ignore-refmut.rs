use ref_split::*;

#[ref_split(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct(i32, #[rs_ignore(ref, mut)] u64);

fn main() {}