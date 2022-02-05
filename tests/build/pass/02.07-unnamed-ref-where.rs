use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<T>(T)
where T: core::cmp::Eq + core::cmp::Ord;

fn main() {}