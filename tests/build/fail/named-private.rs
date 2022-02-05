mod inner {
    use ref_split::*;

    #[ref_split(ref(MyStructRef))]
    struct MyStruct {
        pub x: i32,
    }
}

fn main() {
    use ref_split::*;
    let a = inner::MyStruct{ x: 1i32 };
    let _b: inner::MyStructRef = (&a).split();
}