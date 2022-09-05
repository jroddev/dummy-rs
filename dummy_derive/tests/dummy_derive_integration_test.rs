use std::collections::HashMap;
use rand::random;
use dummy::Dummy;
use dummy_derive::Dummy;

#[derive(Dummy, Default, Debug, PartialEq)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

#[test]
fn test_struct() {
    let a: Vector3 = Vector3::dummy();
    assert_ne!(a.x, f32::default());
    assert_ne!(a.y, f32::default());
    assert_ne!(a.z, f32::default());
    let b: Vector3 = Vector3{ z: 0.0, ..Vector3::dummy() };
    assert_eq!(b.z, 0.0);
}

#[derive(Dummy, Debug)]
struct Tuple0();

#[test]
fn test_unit_type() {
    let a: Tuple0 = Tuple0::dummy();
    assert!(!format!("{:?}", a).is_empty());
}

#[derive(Dummy)]
struct Tuple3(i32, f64, String);

#[test]
fn test_tuple_type() {
    let a: Tuple3 = Tuple3::dummy();
    assert_ne!(a.0, i32::default());
    assert_ne!(a.1, f64::default());
    assert_ne!(a.2, String::default());
}


#[derive(Dummy)]
enum MyEnum {
    VariantA,
    VariantB(i32),
    VariantC{x: i32, y: i32, z:i32},
    VariantD(f32, f32)
}

#[test]
fn test_enum() {
    let mut a_found = false;
    let mut b_found = false;
    let mut c_found = false;
    let mut d_found = false;

    for _i in 0..100 {
        let obj = MyEnum::dummy();
        match obj {
            MyEnum::VariantA => a_found = true,
            MyEnum::VariantB(b) => {
                assert_ne!(b, i32::default());
                b_found = true;
            }
            MyEnum::VariantC { x, y, z } => {
                assert_ne!(x, i32::default());
                assert_ne!(y, i32::default());
                assert_ne!(z, i32::default());
                c_found = true;
            }
            MyEnum::VariantD(a, b) => {
                assert_ne!(a, f32::default());
                assert_ne!(b, f32::default());
                d_found = true;
            }
        }
    }

    assert!(a_found);
    assert!(b_found);
    assert!(c_found);
    assert!(d_found);
}

#[derive(Dummy)]
struct NestedStruct {
    v: Vector3,
    list: Vec<u32>,
    dict: HashMap<String, u32>
}

#[test]
fn test_nested_strut() {
    let a: NestedStruct = NestedStruct::dummy();
    assert_ne!(a.v, Vector3::default());
    assert!(!a.list.is_empty());
    assert!(!a.dict.is_empty());

}


// TODO: derive to support lifetime annotations
// #[derive(Dummy)]
// struct LifeTimeStruct<'a> {
//     str_ref: &'a str
// }

