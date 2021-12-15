use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<'a, 'b: 'a> {
    x: &'a i32,
    y: &'b u32,
}

fn main() {}