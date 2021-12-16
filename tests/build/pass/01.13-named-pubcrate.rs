mod inner {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    pub(crate) struct MyStruct {
        pub(crate) x: i32,
    }
}

fn main() {
    use ref_destruct::*;
    let a = inner::MyStruct{ x: 1i32 };
    let _b: inner::MyStructRef = (&a).destruct();
}