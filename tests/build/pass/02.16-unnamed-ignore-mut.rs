use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct(i32, #[rd_ignore(mut)] u64);

fn main() {}