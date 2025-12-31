use std::marker::PhantomData;

use crate::bool::*;
use crate::nat::*;

pub trait List {}

pub struct Nil;

pub struct Cons<H, T: List>(PhantomData<(H, T)>);

impl List for Nil {}

impl<H, T: List> List for Cons<H, T> {}

pub trait ListElementAt<Index: Nat> {
    type Output;
}

impl<H, T> ListElementAt<Zero> for Cons<H, T>
where
    H: Nat,
    T: List,
{
    type Output = H;
}

impl<H, T: List + ListElementAt<N>, N: Nat> ListElementAt<Succ<N>> for Cons<H, T> {
    type Output = <T as ListElementAt<N>>::Output;
}

pub trait ListEq<R: List> {
    type Output: Bool;
}

impl ListEq<Nil> for Nil {
    type Output = True;
}

impl<H: Nat, R: List> ListEq<Cons<H, R>> for Nil {
    type Output = False;
}

impl<H: Nat, L: List> ListEq<Nil> for Cons<H, L> {
    type Output = False;
}

impl<Hl: Nat + NatEq<Hr>, Hr: Nat, L: List + ListEq<R>, R: List> ListEq<Cons<Hr, R>> for Cons<Hl, L>
where
    <L as ListEq<R>>::Output: And<<Hl as NatEq<Hr>>::Output>, // hint by compiler, not by us
{
    type Output = <<L as ListEq<R>>::Output as And<<Hl as NatEq<Hr>>::Output>>::Output;
}

pub trait Concat<Rhs> {
    type Output: List;
}

impl<R: List> Concat<R> for Nil {
    type Output = R;
}

// Cons(h, l) ++ r = Cons(h, l ++ r)
impl<H, L: List + Concat<R>, R: List> Concat<R> for Cons<H, L> {
    type Output = Cons<H, <L as Concat<R>>::Output>;
}

pub trait ListLen: List {
    type Output: Nat;
}

impl ListLen for Nil {
    type Output = Zero;
}

impl<H, T: List + ListLen> ListLen for Cons<H, T> {
    type Output = Succ<<T as ListLen>::Output>;
}

// for fun
// trait Rev: List {
//     type Output: List;
// }

// impl Rev for Nil {
//     type Output = Nil;
// }

// // rev(cons(h, t)) = concat(rev(t), cons(h, nil))
// impl<H, T: List + Rev> Rev for Cons<H, T>
// where
//     <T as Rev>::Output: Concat<Cons<H, Nil>>,
// {
//     type Output = <<T as Rev>::Output as Concat<Cons<H, Nil>>>::Output;
// }

pub trait Contains<T> {
    type Output: Bool;
}

impl<T> Contains<T> for Nil {
    type Output = False;
}

// not a very general implementation for Contains
// but enough for us
impl<H: Nat + NatEq<T>, L: List + Contains<T>, T: Nat> Contains<T> for Cons<H, L>
where
    <H as NatEq<T>>::Output: Or<<L as Contains<T>>::Output>,
{
    type Output = <<H as NatEq<T>>::Output as Or<<L as Contains<T>>::Output>>::Output;
}

pub trait ToVec {
    type Output;

    fn to_vec() -> Vec<Self::Output>;
}

impl ToVec for Nil {
    type Output = usize;

    fn to_vec() -> Vec<Self::Output> {
        vec![]
    }
}

impl<H: Val, T: List + ToVec> ToVec for Cons<H, T>
where
    Vec<usize>: Extend<<T as ToVec>::Output>,
{
    type Output = usize;

    fn to_vec() -> Vec<Self::Output> {
        let mut v = vec![H::VALUE];
        v.extend(T::to_vec());
        v
    }
}

pub trait ToMatrix {
    fn to_matrix() -> Vec<Vec<usize>>;
}

impl ToMatrix for Nil {
    fn to_matrix() -> Vec<Vec<usize>> {
        vec![]
    }
}

impl<H: List + ToVec<Output = usize>, T: List + ToMatrix> ToMatrix for Cons<H, T> {
    fn to_matrix() -> Vec<Vec<usize>> {
        let mut v = vec![H::to_vec()];
        v.extend(T::to_matrix());
        v
    }
}

#[test]
fn test_contains() {
    type L1 = Cons<N1, Cons<N2, Cons<N3, Nil>>>;
    type L2 = Cons<N1, Cons<N2, Cons<N4, Nil>>>;
    type L3 = Cons<N1, Cons<N2, Cons<N3, Nil>>>;

    println!("{:?}", L1::to_vec());

    println!("{}", std::any::type_name::<<L1 as ListEq<L2>>::Output>());
    println!("{}", std::any::type_name::<<L1 as ListEq<L3>>::Output>());

    type L11 = Cons<N1, Nil>;
    type L12 = Cons<N1, Nil>;
    type L13 = <L11 as Concat<L12>>::Output;
    println!("{}", std::any::type_name::<L13>());

    type L13Len = <L13 as ListLen>::Output;
    println!("{}", std::any::type_name::<L13Len>());

    // type Revl1 = <L1 as Rev>::Output;
    // println!("{}", std::any::type_name::<Revl1>());

    type ContainsN1 = <L1 as Contains<N1>>::Output;
    type ContainsN2 = <L1 as Contains<N2>>::Output;
    type NotContainsN5 = <L1 as Contains<N5>>::Output;
    println!("contains n1: {}", std::any::type_name::<ContainsN1>());
    println!("contains n2: {}", std::any::type_name::<ContainsN2>());
    println!("contains n5: {}", std::any::type_name::<NotContainsN5>());
}

#[test]
fn test_concats() {
    {
        type L1 = Nil;
        type L2 = Cons<N1, Cons<N2, Nil>>;
        type L3 = <L1 as Concat<L2>>::Output;
        println!("{:?}", L3::to_vec());
    }

    {
        type L1 = Cons<N1, Cons<N2, Nil>>;
        type L2 = Nil;
        type L3 = <L1 as Concat<L2>>::Output;
        println!("{:?}", L3::to_vec());
    }

    {
        type L1 = Cons<N1, Cons<N2, Nil>>;
        type L2 = Cons<N3, Cons<N4, Nil>>;
        type L3 = <L1 as Concat<L2>>::Output;
        println!("{:?}", L3::to_vec());
    }
}
