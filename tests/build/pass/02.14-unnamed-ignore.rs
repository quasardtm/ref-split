use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct(i32, #[rs_ignore] u64);

fn main() {}