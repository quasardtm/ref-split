use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<T>(T)
where T: core::cmp::Eq + core::cmp::Ord;

fn main() {}