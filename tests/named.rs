use ref_destruct::*;

#[derive(Default)]
#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct {
    x: u32,
    y: i32,
}

#[test]
fn into_ref_struct() {
    let a = MyStruct { x: 1, y: 2 };
    let ref_struct: MyStructRef = (&a).into();
    assert_eq!(&a.x, ref_struct.x);
}

#[test]
fn destruct_to_ref_struct() {
    let a = MyStruct { x: 1, y: 2 };
    let ref_struct = a.destruct();
    assert_eq!(&a.x, ref_struct.x);
}

#[test]
fn into_mut_struct() {
    let mut a = MyStruct { x: 1, y: 2 };
    let ref_struct: MyStructMut = (&mut a).into();
    assert_eq!(*ref_struct.x, 1);
    *ref_struct.x += 10;
    assert_eq!(a.x, 11);
}

#[test]
fn destruct_to_mut_struct() {
    let mut a = MyStruct { x: 1, y: 2 };
    let ref_struct = (&mut a).destruct();
    assert_eq!(*ref_struct.x, 1);
    *ref_struct.x += 10;
    assert_eq!(a.x, 11);
}