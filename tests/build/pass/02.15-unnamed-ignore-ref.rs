use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct(i32, #[rs_ignore(ref)] u64);

fn main() {}