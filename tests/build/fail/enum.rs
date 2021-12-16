use ref_destruct::*;

#[ref_destruct(ref(MyEnumRef))]
enum MyEnum {
    A,
    B,
}

fn main() {}