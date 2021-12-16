use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<T>
where T: core::cmp::Eq + core::cmp::Ord,
{
    x: T,
}

fn main() {}