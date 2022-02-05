use ref_split::*;

#[ref_split(ref(MyEnumRef))]
enum MyEnum {
    A,
    B,
}

fn main() {}