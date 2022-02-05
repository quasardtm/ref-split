use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<'a, 'b: 'a> {
    x: &'a i32,
    y: &'b u32,
}

fn main() {}