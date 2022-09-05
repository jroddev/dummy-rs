use dummy::Dummy;


// struct Nested {
//     custom: Custom
// }
//
// impl Dummy for Nested {
//     fn dummy() -> Self {
//         Nested {
//             custom: Custom::dummy()
//         }
//     }
// }
//
//
// #[derive (Debug)]
// struct ComplexType<'a> {
//     customs: Vec<Custom>,
//     label: &'a str,
//     label_owned: String
// }
//
//
// impl Dummy  for ComplexType<'_> {
//     fn dummy() -> Self {
//         ComplexType {
//             customs: Vec::<Custom>::dummy(),
//             label: <&str>::dummy(),
//             label_owned: String::dummy()
//         }
//     }
// }

fn main() {
//     println!("Wrappers: {:?}", Nested::dummy().custom.x);
//     println!("Wrappers: {:?}", Custom{x: 123, ..Custom::dummy()}.x);
//     println!("D2: {:?}", Vec::<i32>::dummy());
//     println!("ComplexType: {:?}", ComplexType::dummy());
//     println!("String: {}", String::dummy());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive (Debug)]
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
    fn complex_type() {

    }




    #[test]
    fn it_works() {
        println!("Wrappers: {:?}", Nested::dummy().custom.x);
        println!("Wrappers: {:?}", Custom{x: 123, ..Custom::dummy()}.x);
        println!("D2: {:?}", Vec::<i32>::dummy());
        println!("ComplexType: {:?}", ComplexType::dummy());
        println!("String: {}", String::dummy());

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

