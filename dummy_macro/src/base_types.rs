use rand::prelude::*;
use crate::dummy::Dummy;
use std::collections::HashSet;
use std::collections::HashMap;

impl Dummy for bool { fn dummy() -> Self { random() }}

impl Dummy for u8   { fn dummy() -> Self { random() }}
impl Dummy for u16  { fn dummy() -> Self { random() }}
impl Dummy for u32  { fn dummy() -> Self { random() }}
impl Dummy for u64  { fn dummy() -> Self { random() }}
impl Dummy for u128 { fn dummy() -> Self { random() }}

impl Dummy for i8   { fn dummy() -> Self { random() }}
impl Dummy for i16  { fn dummy() -> Self { random() }}
impl Dummy for i32  { fn dummy() -> Self { random() }}
impl Dummy for i64  { fn dummy() -> Self { random() }}
impl Dummy for i128 { fn dummy() -> Self { random() }}

impl Dummy for f32  { fn dummy() -> Self { random() }}
impl Dummy for f64  { fn dummy() -> Self { random() }}


impl Dummy for String  {
    fn dummy() -> Self {
        let count = thread_rng().gen_range(3..32);
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(count)
            .map(char::from)
            .collect::<String>()
}}

impl<T: Dummy> Dummy for Vec<T> {
    fn dummy() -> Self {
        let mut result = Vec::<T>::new();
        (0..5).for_each(|_|{
            result.push(T::dummy());
        });
        result
    }
}

impl<T: Dummy + std::cmp::Eq + std::hash::Hash> Dummy for HashSet<T> {
    fn dummy() -> Self {
        let mut result = HashSet::<T>::new();
        (0..5).for_each(|_|{
            result.insert(T::dummy());
        });
        result
    }
}


impl<K: Dummy + std::cmp::Eq + std::hash::Hash, V: Dummy> Dummy for HashMap<K, V> {
    fn dummy() -> Self {
        let mut result = HashMap::<K,V>::new();
        (0..5).for_each(|_|{
            result.insert(K::dummy(), V::dummy());
        });
        result
    }
}

// In order to create a dummy slice we need a static sample in which to reference
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


impl<T: Dummy> Dummy for Box<T> {
    fn dummy() -> Self {
        Box::<T>::new(T::dummy())
    }
}

