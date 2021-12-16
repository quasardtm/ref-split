use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<T>(T)
where T: core::cmp::Eq + core::cmp::Ord;

fn main() {}