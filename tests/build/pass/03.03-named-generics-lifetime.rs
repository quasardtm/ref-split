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
    struct MyStruct<'a, 'b, X, Y, Z> {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a Y,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: &'b Z,
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
    struct MyStruct<'a, 'b, X, Y, Z> {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a Y,
        z: &'b Z,
    }
}
fn main() {}
