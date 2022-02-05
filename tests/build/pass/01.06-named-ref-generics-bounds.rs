use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<T, U: core::cmp::Eq + core::cmp::Ord> {
    x: T,
    y: U,
}

fn main() {}