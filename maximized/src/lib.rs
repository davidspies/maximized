use arr_macro::arr;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use std::marker::PhantomData;

pub trait Maximized: Sized + BorshSerialize {
    fn maximized() -> Self;

    fn compute_size() -> usize {
        byte_len(&Self::maximized())
    }
}

pub fn byte_len(x: &impl BorshSerialize) -> usize {
    x.try_to_vec().unwrap().len()
}

pub fn maximum<T: BorshSerialize>(xs: impl IntoIterator<Item = T>) -> T {
    let mut max_val: Option<T> = None;
    let mut nbytes = 0;
    for x in xs {
        let xbytes = byte_len(&x);
        if max_val.is_none() || xbytes > nbytes {
            max_val = Some(x);
            nbytes = xbytes;
        }
    }
    max_val.unwrap()
}

macro_rules! impl_fixed_default {
    ($type: ty, $nbytes: expr) => {
        impl Maximized for $type {
            fn maximized() -> Self {
                Default::default()
            }

            fn compute_size() -> usize {
                $nbytes
            }
        }
    };
}

impl_fixed_default!(bool, 1);
impl_fixed_default!((), 0);
impl_fixed_default!(u8, 1);
impl_fixed_default!(u16, 2);
impl_fixed_default!(u32, 4);
impl_fixed_default!(u64, 8);
impl_fixed_default!(u128, 16);
impl_fixed_default!(i8, 1);
impl_fixed_default!(i16, 2);
impl_fixed_default!(i32, 4);
impl_fixed_default!(i64, 8);
impl_fixed_default!(i128, 16);
impl_fixed_default!(f32, 4);
impl_fixed_default!(f64, 8);
impl_fixed_default!(usize, 8);
impl_fixed_default!(Pubkey, 32);

impl<T: Maximized> Maximized for Option<T> {
    fn maximized() -> Self {
        Some(Maximized::maximized())
    }

    fn compute_size() -> usize {
        1 + T::compute_size()
    }
}

impl<T: ?Sized> Maximized for PhantomData<T> {
    fn maximized() -> Self {
        PhantomData
    }

    fn compute_size() -> usize {
        0
    }
}

impl<T: Maximized, E: Maximized> Maximized for Result<T, E> {
    fn maximized() -> Self {
        maximum([Ok(Maximized::maximized()), Err(Maximized::maximized())])
    }

    fn compute_size() -> usize {
        1 + T::compute_size().max(E::compute_size())
    }
}

macro_rules! impl_tuple {
    ($($name:ident)+) => {
        impl<$($name),+> Maximized for ($($name),+)
        where $($name: Maximized,)+
        {
            fn maximized() -> Self {
                ($({let _ = PhantomData::<$name>; Maximized::maximized()},)+)
            }
            fn compute_size() -> usize {
                [$(<$name as Maximized>::compute_size(),)+].into_iter().sum()
            }
        }
    };
}

impl_tuple!(T0 T1);
impl_tuple!(T0 T1 T2);
impl_tuple!(T0 T1 T2 T3);
impl_tuple!(T0 T1 T2 T3 T4);
impl_tuple!(T0 T1 T2 T3 T4 T5);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);

macro_rules! impl_arrays {
    ($($len:expr)+) => {
        $(
            impl<T: Default + Maximized> Maximized for [T; $len]
            {
                fn maximized() -> Self {
                    arr![T::maximized(); $len]
                }

                fn compute_size() -> usize {
                    $len * T::compute_size()
                }
            }
        )+
    };
}

impl_arrays!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 64 65 128 256 512 1024 2048);
