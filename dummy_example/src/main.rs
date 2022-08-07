use rand::random;
use dummy_macro::Dummy;
use dummy_macro_derive::Dummy;

struct D<T>(T);
impl Dummy for D<i32> {
    fn dummy() -> D<i32> {
        D(random())
    }
}

// pub fn dummy<T> -> T {
//     D::<T>::value()
// }
//
// impl Dummy for i32 {
//     fn dummy() -> i32 {
//         random()
//     }
// }
// impl Dummy for i64{ fn dummy() -> i64 { random() } }
// impl Dummy for f32{ fn dummy() -> f32 { random() } }



#[derive(Dummy, Debug)]
struct Pancakes {
    x: i32,
    y: i32
}

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
}
