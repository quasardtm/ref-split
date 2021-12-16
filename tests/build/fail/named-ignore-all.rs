use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct {
    #[rd_ignore]
    x: i32,
    #[rd_ignore]
    y: u32,
}

fn main() {}