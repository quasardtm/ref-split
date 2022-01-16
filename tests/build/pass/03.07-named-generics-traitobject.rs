mod gen1 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        z: Box<dyn Add<Z, Output = X>>,
    }
}

mod gen2 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        z: Box<dyn Add<Z, Output = X>>,
    }
}

mod gen3 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: Box<dyn Add<Z, Output = X>>,
    }
}

mod gen4 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        z: Box<dyn Add<Z, Output = X>>,
    }
}

mod gen5 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: Box<dyn Add<Z, Output = X>>,
    }
}

mod gen6 {
    use std::ops::Add;

    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef))]
    struct MyStruct<'a, X, Y, Z> {
        x: Box<dyn for<'b> Fn(&'b X, &'a Y) -> &'b X + Send + Sync>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: Box<dyn Iterator<Item = (Y, Z)> + 'a>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: Box<dyn Add<Z, Output = X>>,
    }
}

fn main() {}
