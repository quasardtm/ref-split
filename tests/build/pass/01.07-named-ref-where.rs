use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<T>
where T: core::cmp::Eq + core::cmp::Ord,
{
    x: T,
}

fn main() {}