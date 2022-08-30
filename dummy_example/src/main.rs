use rand::random;
use dummy_macro::Dummy;
use dummy_macro_derive::Dummy;

struct D<T>(T);
impl Dummy for D<i32> {
    fn dummy() -> D<i32> {
        D(random())
    }
}

#[derive(Dummy, Debug)]
struct Pancakes {
    x: i32,
    y: i32
}

#[derive(Dummy, Debug)]
struct SSS {
    s: String
}

#[derive(Dummy, Debug)]
struct Tuple3(i32, f64, String);


#[derive(Dummy, Debug)]
struct Tuple0();



#[derive(Debug, Dummy)]
enum MyEnum {
    VariantA,
    VariantB(i32),
    VariantC{x: i32, y: i32, z:i32},
    VariantD(f32, f32)
}

// impl Dummy for MyEnum {
//     fn dummy() -> Self {
//         let variant = random::<u32>()%3;
//         match variant {
//             0 => Self::VariantA{},
//             1 => Self::VariantB(i32::dummy()),
//             2 => Self::VariantC{ x: i32::dummy(), y: i32::dummy(), z: i32::dummy() },
//             _ => panic!("Dummy Enum Variant Out of Bounds: {}", variant)
//         }
//     }
// }


// #[derive(Dummy, Debug)]
// struct PancakeStack {
//     pancakes: Vec<Pancakes>,
//     ready: bool,
// }

fn main() {
    let a: i32 = random();
    for i in 0..5 {
        println!("created a dummy:{:?}", Pancakes::dummy());
    }
    println!("partial: {:?}",
             Pancakes{
                 x: 7,
                ..Pancakes::dummy()
             });

    println!("other: {}", D::<i32>::dummy().0);
    println!("Unit type: {:?}", Tuple0::dummy());
    println!("Tuple type: {:?}", Tuple3::dummy());
    println!("Enum: {:?}", MyEnum::dummy());
}
