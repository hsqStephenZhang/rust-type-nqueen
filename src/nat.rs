use crate::bool::{Bool, False, True};
use std::marker::PhantomData;

pub trait Nat {}

pub struct Zero;

impl Nat for Zero {}

pub struct Succ<N: Nat>(PhantomData<N>);

impl<N: Nat> Nat for Succ<N> {}

pub type N0 = Zero;
pub type N1 = Succ<N0>;
pub type N2 = Succ<N1>;
pub type N3 = Succ<N2>;
pub type N4 = Succ<N3>;
pub type N5 = Succ<N4>;
pub type N6 = Succ<N5>;
pub type N7 = Succ<N6>;
pub type N8 = Succ<N7>;

pub trait Val: Nat {
    const VALUE: usize;
}

impl Val for Zero {
    const VALUE: usize = 0;
}

impl<N: Val> Val for Succ<N> {
    const VALUE: usize = <N as Val>::VALUE + 1;
}

pub trait NatEq<Rhs: Nat> {
    type Output: Bool;
}

impl NatEq<Zero> for Zero {
    type Output = True;
}

impl<R: Nat> NatEq<Succ<R>> for Zero {
    type Output = False;
}

impl<L: Nat> NatEq<Zero> for Succ<L> {
    type Output = False;
}

impl<L: Nat + NatEq<R>, R: Nat> NatEq<Succ<R>> for Succ<L> {
    type Output = <L as NatEq<R>>::Output;
}

pub trait NatLessThan<Rhs: Nat> {
    type Output: Bool;
}

impl NatLessThan<Zero> for Zero {
    type Output = False;
}

impl<R: Nat> NatLessThan<Succ<R>> for Zero {
    type Output = True;
}

impl<L: Nat> NatLessThan<Zero> for Succ<L> {
    type Output = False;
}

impl<L: Nat + NatLessThan<R>, R: Nat> NatLessThan<Succ<R>> for Succ<L> {
    type Output = <L as NatLessThan<R>>::Output;
}

pub trait Add<Rhs: Nat> {
    type Output: Nat;
}

impl<R: Nat> Add<R> for Zero {
    type Output = R;
}

impl<L: Nat + Add<R>, R: Nat> Add<R> for Succ<L> {
    type Output = Succ<<L as Add<R>>::Output>;
}

pub trait AbsOfSub<R: Nat> {
    type Output: Nat;
}

// Case 1: |0 - R| = R
impl<R: Nat> AbsOfSub<R> for Zero {
    type Output = R;
}

// Case 2: |Succ<L> - 0| = Succ<L>
impl<L: Nat> AbsOfSub<Zero> for Succ<L> {
    type Output = Succ<L>;
}

// Case 3: |Succ<L> - Succ<R>| = |L - R|
impl<L: Nat + AbsOfSub<R>, R: Nat> AbsOfSub<Succ<R>> for Succ<L> {
    type Output = <L as AbsOfSub<R>>::Output;
}

pub trait Mul<Rhs: Nat> {
    type Output: Nat;
}

// 0 * 0 = 0
impl Mul<Zero> for Zero {
    type Output = Zero;
}

// succ(l) * 0 = 0
impl<L: Nat> Mul<Zero> for Succ<L> {
    type Output = Zero;
}

// mul(l, succ(r)) = mul(l, r) + l
impl<L: Nat + Mul<R>, R: Nat> Mul<Succ<R>> for L
where
    <L as Mul<R>>::Output: Add<L>, // hint by compiler, not by us
{
    type Output = <<L as Mul<R>>::Output as Add<L>>::Output;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nateq() {
        type Res1 = <N2 as NatEq<N2>>::Output;
        println!("{}", std::any::type_name::<Res1>());

        type Res2 = <N1 as Add<N2>>::Output;
        println!("{}", std::any::type_name::<Res2>());

        type Res3 = <N2 as Mul<N3>>::Output;
        println!("{}", std::any::type_name::<Res3>());
    }

    #[test]
    fn test_nat_lessthan() {
        type Res1 = <N2 as NatLessThan<N3>>::Output;
        type Res2 = <N4 as NatLessThan<N3>>::Output;
        println!("{}", std::any::type_name::<Res1>());
        println!("{}", std::any::type_name::<Res2>());
    }
}
