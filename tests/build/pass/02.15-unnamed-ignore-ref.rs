use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct(i32, #[rd_ignore(ref)] u64);

fn main() {}