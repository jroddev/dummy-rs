use dummy_rs::Dummy;

#[derive (Debug, PartialEq)]
struct Custom {
    x: i32,
    y: i32
}
impl Dummy for Custom {
    fn dummy() -> Self {
        Custom {
            x: i32::dummy(),
            y: i32::dummy()
        }
    }
}

#[test]
fn test_custom_type() {
    let a = Custom::dummy();
    let b = Custom::dummy();
    assert_ne!(a, b);
}

#[test]
fn test_partial() {
    let a = Custom{ y: 5555, ..Custom::dummy() };
    assert_eq!(a.y, 5555);
    assert_ne!(a.x, 5555);
}

#[derive(PartialEq, Debug)]
struct Nested {
    custom: Custom
}

impl Dummy for Nested {
    fn dummy() -> Self {
        Nested {
            custom: Custom::dummy()
        }
    }
}

#[test]
fn test_nested_type() {
    let a = Nested::dummy();
    let b = Nested::dummy();
    assert_ne!(a, b);
}


#[derive (Debug)]
struct ComplexType<'a> {
    customs: Vec<Custom>,
    label: &'a str,
    label_owned: String
}


impl Dummy  for ComplexType<'_> {
    fn dummy() -> Self {
        ComplexType {
            customs: Vec::<Custom>::dummy(),
            label: <&str>::dummy(),
            label_owned: String::dummy()
        }
    }
}

#[test]
fn complex_type() {
    let a = ComplexType::dummy();
    let mut b = ComplexType::dummy();

    while a.customs.len() == b.customs.len() {
        // Reroll if random Vec end up being the same length
        b = ComplexType::dummy();
    }

    assert_ne!(a.customs.len(), b.customs.len());
    assert_ne!(a.label, b.label);
    assert_ne!(a.label_owned, b.label_owned);
}
