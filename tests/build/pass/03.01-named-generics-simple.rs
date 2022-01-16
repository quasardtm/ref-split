mod gen1 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<X, Y, Z> {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: Y,
        z: Z,
    }
}
mod gen2 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<X, Y, Z> {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: Y,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: Z,
    }
}
fn main() {}
