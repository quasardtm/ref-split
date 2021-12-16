use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<T, U: core::cmp::Eq + core::cmp::Ord> {
    x: T,
    y: U,
}

fn main() {}