mod inner {
    use ref_destruct::*;

    #[ref_destruct(MyStructRef)]
    pub struct MyStruct(pub i32);
}

fn main() {
    use ref_destruct::*;
    let a = inner::MyStruct(1i32);
    let _b: inner::MyStructRef = (&a).destruct();
}