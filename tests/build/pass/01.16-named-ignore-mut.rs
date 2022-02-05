use ref_split::*;

#[ref_split(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct {
    x: i32,
    #[rs_ignore(mut)]
    y: u32,
}

fn main() {}