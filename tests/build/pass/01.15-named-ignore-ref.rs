use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct {
    x: i32,
    #[rd_ignore(ref)]
    y: u32,
}

fn main() {}