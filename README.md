# DUMMY-RS

Implementation of Dummy for Rust
https://blog.jroddev.com/smaller-clearer-tests/

## Basic Usage
The derive macro will recursively build randomised/dummy constructors
```rs
#[derive (Dummy)
struct Point {
    x: i32,
    y: i32
}

let a = Point::dummy();
let b = Point{
    y: 0,
    ..Point::dummy()
};
```


## Macros Expanded
If you need to customise how the dummy is created you simply need to 
implement the Dummy trait with one function `dummy()->Self`.
By default all fields will be created by `T::dummy()`.
```rs
struct Point {
    x: i32,
    y: i32
}

impl Dummy for Point {
    fn dummy() -> Self {
        Point{
            x: i32::dummy(),
            y: i32::dummy()
        }
    }
}
```

## Complex Type 1
The library supports nested custom structs (as long as the whole chain implements Dummy).
Values can also be collections. By default I have implemented Vec, HashSet, and HashMap.
```rs
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
```

## Complex Type 2
&str is also supported by referencing into a global/static string literal with the library.
Other references and slices can be implemented in a similar way, but are not supported by Derive
```rs
static GLOBAL_STRING: &str = r#"
    f:G&+(r3Gm_-9TtwvJy)WY@WGZ8]p?m!)gHn79_rmP5k4}]k,KD}a,R!2+#+vh.q}TD}2WQARtCcCJU]};#NLR[+dF;J2Ff+!f#$D%N2D!m2U(L:@i9E:ZNmQ#L,k9@&?!?4c6*Y;&)a]/58K(6hw2Jg]d%5]+V}b2caMmf+na6p+k4!E)*+3A}=VDp[q&3v$df*Pv]i}(WxZiCx3+YTyY&d(6LMQV2$8#@)*X/%!gB.R@3;+@Y{#*K;aZJ2FS()Hbza#HucQQ[fP};Nvpmd7,Z=nY=?_MX&7]T49tt;bY#Sbr$tZnPX4%;zLqCMpDu_h/iCGicg?,?K*K=(:5K&{@QaEHVYL/Bqy=KJjWWA{&d}]9yu=6YR!#?==%k5#SJH,y/e)}MbSef,x4A_mP]yL7Edd}]DWh-M4BTFtq]@$NL$ptQ$L&.:QHgz=[yu:z,-bU)}W?Ba$WQig@n.dQj%Bf*t]_%;*Z?F[N[BMi,/67H%$kUDzSba;..Vj;XW]v,.
"#;
impl Dummy for &str {
    fn dummy() -> Self {
        let length = thread_rng().gen_range(3..32);
        let start = thread_rng().gen_range(0..GLOBAL_STRING.len()-length);
        let end = start+length;
        &GLOBAL_STRING[start..end]
    }
}
```

## Complex Type 3
Implementation of Dummy for HashMap shows a heavier use of generics.
But the key type (K) and the value type (V) need to implement the Dummy train.
Key type (K) also needs to support Eq and Hash to be used as the HashMap key
```rs
impl<K: Dummy + std::cmp::Eq + std::hash::Hash, V: Dummy> Dummy for HashMap<K, V> {
    fn dummy() -> Self {
        let mut result = HashMap::<K,V>::new();
        (0..5).for_each(|_|{
            result.insert(K::dummy(), V::dummy());
        });
        result
    }
}
```
