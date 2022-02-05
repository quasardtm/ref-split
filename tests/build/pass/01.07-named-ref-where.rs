use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<T>
where T: core::cmp::Eq + core::cmp::Ord,
{
    x: T,
}

fn main() {}