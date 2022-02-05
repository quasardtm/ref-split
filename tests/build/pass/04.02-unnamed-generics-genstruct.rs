mod gen1 {
    use ref_split::*;
    use std::collections::HashMap;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<X, Y, Z> (
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        HashMap<Y, Z>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        Z,
    );
}
mod gen2 {
    use ref_split::*;
    use std::collections::HashMap;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<X, Y, Z> (
        HashMap<X, Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        HashMap<Y, Z>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        HashMap<Z, X>,
    );
}
mod gen3 {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<X, Y, Z> (
        [X; 3],
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        [(Y, Z); 3],
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        [Z; 3],
    );
}
mod gen4 {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<X, Y, Z> {
        x: [(X, Y); 3],
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: [(Y, Z); 3],
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: [(Z, X); 3],
    }
}
fn main() {}