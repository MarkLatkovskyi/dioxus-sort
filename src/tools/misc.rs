use std::ops::Rem;

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Rem<Output = T> + From<u8> + PartialEq + Clone,
{
    let zero = 0.into();
    while b != zero {
        (a, b) = (b.clone(), a % b)
    }
    a
}