use ref_split::*;

#[ref_split(ref(MyStructRef), mut(MyStructMut))]
struct MyStruct<'a, 'b, T, U, V: core::fmt::Display + core::fmt::Debug>(&'a T, &'b mut U, &'static V)
where
    U: Default + core::fmt::Debug,
    V: 'static + ?Sized;

#[test]
fn into_ref_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let a = MyStruct(&x, &mut y, z);
    let ref_struct: MyStructRef<_, _, _> = (&a).into();
    assert_eq!(&a.0, ref_struct.0);
}

#[test]
fn split_to_ref_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let a = MyStruct(&x, &mut y, z);
    let ref_struct = a.split();
    assert_eq!(&a.0, ref_struct.0);
}

#[test]
fn into_mut_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let mut a = MyStruct(&x, &mut y, z);
    let ref_struct: MyStructMut<_, _, _> = (&mut a).into();
    assert_eq!(**ref_struct.1, 2);
    **ref_struct.1 += 10;
    assert_eq!(*a.1, 12);
}

#[test]
fn split_to_mut_struct() {
    let x = 1i32;
    let mut y = 2u32;
    let z = "test";

    let mut a = MyStruct(&x, &mut y, z);
    let ref_struct = (&mut a).split();
    assert_eq!(**ref_struct.1, 2);
    **ref_struct.1 += 10;
    assert_eq!(*a.1, 12);
}