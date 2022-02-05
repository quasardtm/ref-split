use ref_split::*;

#[ref_split(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct {
    #[rs_ignore]
    x: i32,
    #[rs_ignore]
    y: u32,
}

fn main() {}