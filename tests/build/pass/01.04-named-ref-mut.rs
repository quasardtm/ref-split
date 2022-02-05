use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<'a> {
    x: &'a mut i32,
}

fn main() {}