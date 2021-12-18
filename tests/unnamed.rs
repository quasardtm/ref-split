use ref_destruct::*;

#[derive(Default)]
#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct(u32, i32);

#[test]
fn into_ref_struct() {
    let a = MyStruct(1, 2);
    let ref_struct: MyStructRef = (&a).into();
    assert_eq!(&a.0, ref_struct.0);
}

#[test]
fn destruct_to_ref_struct() {
    let a = MyStruct(1, 2);
    let ref_struct = a.destruct();
    assert_eq!(&a.0, ref_struct.0);
}

#[test]
fn into_mut_struct() {
    let mut a = MyStruct(1, 2);
    let ref_struct: MyStructMut = (&mut a).into();
    assert_eq!(*ref_struct.0, 1);
    *ref_struct.0 += 10;
    assert_eq!(a.0, 11);
}

#[test]
fn destruct_to_mut_struct() {
    let mut a = MyStruct(1, 2);
    let ref_struct = (&mut a).destruct();
    assert_eq!(*ref_struct.0, 1);
    *ref_struct.0 += 10;
    assert_eq!(a.0, 11);
}