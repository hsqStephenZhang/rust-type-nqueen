use crate::bool::*;
use crate::list::*;
use crate::nat::*;
use std::marker::PhantomData;

pub trait Pred<X> {
    type Output: Bool;
}

pub trait IfThenElse<Then, Else> {
    type Output;
}

impl<Then, Else> IfThenElse<Then, Else> for True {
    type Output = Then;
}

impl<Then, Else> IfThenElse<Then, Else> for False {
    type Output = Else;
}

struct IsZero;

impl Pred<Zero> for IsZero {
    type Output = True;
}

impl<N: Nat> Pred<Succ<N>> for IsZero {
    type Output = False;
}

struct Is<V: Nat>(PhantomData<V>);

impl<N: Nat + NatEq<V>, V: Nat> Pred<N> for Is<V> {
    type Output = <N as NatEq<V>>::Output;
}

struct IsNot<V: Nat>(PhantomData<V>);

impl<N: Nat + NatEq<V>, V: Nat> Pred<N> for IsNot<V>
where
    <N as NatEq<V>>::Output: Not,
{
    type Output = <<N as NatEq<V>>::Output as Not>::Output;
}

struct ListEqPred<L: List>(PhantomData<L>);

impl<L: List, Cmp: List> Pred<Cmp> for ListEqPred<L>
where
    L: ListEq<Cmp>,
{
    type Output = <L as ListEq<Cmp>>::Output;
}

pub trait Filter<P>: List {
    type Output: List;
}

impl<P> Filter<P> for Nil {
    type Output = Nil;
}

impl<P: Pred<H>, H, L: List + Filter<P>> Filter<P> for Cons<H, L>
where
    <P as Pred<H>>::Output: IfThenElse<Cons<H, <L as Filter<P>>::Output>, <L as Filter<P>>::Output>,
    <<P as Pred<H>>::Output as IfThenElse<
        Cons<H, <L as Filter<P>>::Output>,
        <L as Filter<P>>::Output,
    >>::Output: List,
{
    // cond: <P as Pred<H>>::Output
    // then: Cons<H, <L as Filter<P>>::Output>;
    // else: <L as Filter<P>>::Output;
    type Output = <<P as Pred<H>>::Output as IfThenElse<
        Cons<H, <L as Filter<P>>::Output>,
        <L as Filter<P>>::Output,
    >>::Output;
}

#[test]
fn test_list_predicate() {
    type L1 = Cons<N0, Cons<N2, Cons<N3, Nil>>>;
    type L2 = <L1 as Filter<IsZero>>::Output;
    println!("{}", std::any::type_name::<L2>());

    type L3 = <L1 as Filter<Is<N2>>>::Output;
    println!("{}", std::any::type_name::<L3>());

    type L4 = <L1 as Filter<Is<N7>>>::Output;
    println!("{}", std::any::type_name::<L4>());

    type L5 = <L1 as Filter<IsNot<N3>>>::Output;
    println!("{}", std::any::type_name::<L5>());
}
