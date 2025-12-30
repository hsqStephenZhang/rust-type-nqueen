pub trait Bool {}

pub struct True;
pub struct False;

impl Bool for True {}

impl Bool for False {}

pub trait And<Rhs: Bool> {
    type Output: Bool;
}

impl And<True> for True {
    type Output = True;
}

impl And<False> for True {
    type Output = False;
}

impl<B: Bool> And<B> for False {
    type Output = False;
}

pub trait Or<Rhs: Bool> {
    type Output: Bool;
}

impl Or<False> for False {
    type Output = False;
}

impl Or<True> for False {
    type Output = True;
}

impl<B: Bool> Or<B> for True {
    type Output = True;
}

pub trait Not {
    type Output: Bool;
}

impl Not for False {
    type Output = True;
}

impl Not for True {
    type Output = False;
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::IfThenElse,
        nat::{N1, N2},
    };

    use super::*;

    #[test]
    fn test_bool() {
        type T1 = <True as IfThenElse<N1, N2>>::Output;
        type T2 = <False as IfThenElse<T1, N2>>::Output;
        println!("{}", std::any::type_name::<T1>());
        println!("{}", std::any::type_name::<T2>());
    }
}
