#[macro_use]
extern crate enum_primitive_derive;
extern crate enum_primitive;

use enum_primitive::FromPrimitive;

#[derive(EnumPrimitive, PartialEq, Debug)]
enum Foo {
    A,
    B,
    C,
    D,
}

#[derive(EnumPrimitive, PartialEq, Debug)]
enum Bar {
    A = 10,
    B = 0b11,
    C = -0,
    D = 0x14,
}

#[derive(EnumPrimitive, PartialEq, Debug)]
enum Empty {

}

// Fails to compile
// #[derive(EnumPrimitive, PartialEq, Debug)]
// enum Bar2 {
//     A(u32),
//     B,
//     C,
// }

// Fails to compile
// #[derive(EnumPrimitive, PartialEq, Debug)]
// enum Barr {
//     A,
//     B = 0b11,
//     C = -0,
//     D = 0x14,
// }

#[test]
fn test() {
    assert_eq!(Foo::from_isize(-1), None);
    assert_eq!(Foo::from_usize(0), Some(Foo::A));
    assert_eq!(Foo::from_usize(4), None);

    assert_eq!(Bar::from_usize(10), Some(Bar::A));
    assert_eq!(Bar::from_usize(3), Some(Bar::B));
    assert_eq!(Bar::from_usize(0), Some(Bar::C));
    assert_eq!(Bar::from_u32(0x14), Some(Bar::D));

    assert_eq!(Bar::from_isize(-1), None);
    assert_eq!(Bar::from_u32(22), None);

    assert_eq!(Empty::from_u32(0), None);
}