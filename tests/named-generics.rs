use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct<'a, 'b, T, U, V: core::fmt::Display + core::fmt::Debug>
where
    U: Default + core::fmt::Debug,
    V: 'static + ?Sized,
{
    x: &'a T,
    y: &'b mut U,
    z: &'static V,
}

#[test]
fn into_ref_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let a = MyStruct { x: &x, y: &mut y, z };
    let ref_struct: MyStructRef<_, _, _> = (&a).into();
    assert_eq!(&a.x, ref_struct.x);
}

#[test]
fn destruct_to_ref_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let a = MyStruct { x: &x, y: &mut y, z };
    let ref_struct = a.destruct();
    assert_eq!(&a.x, ref_struct.x);
}

#[test]
fn into_mut_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let mut a = MyStruct { x: &x, y: &mut y, z };
    let ref_struct: MyStructMut<_, _, _> = (&mut a).into();
    assert_eq!(**ref_struct.y, 2);
    **ref_struct.y += 10;
    assert_eq!(*a.y, 12);
}

#[test]
fn destruct_to_mut_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let mut a = MyStruct { x: &x, y: &mut y, z };
    let ref_struct = (&mut a).destruct();
    assert_eq!(**ref_struct.y, 2);
    **ref_struct.y += 10;
    assert_eq!(*a.y, 12);
}