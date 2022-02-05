mod inner {
    use ref_split::*;

    #[ref_split(ref(MyStructRef))]
    struct MyStruct(pub i32);
}

fn main() {
    use ref_split::*;
    let a = inner::MyStruct(1i32);
    let _b: inner::MyStructRef = (&a).split();
}