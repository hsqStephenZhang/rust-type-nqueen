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

pub struct PIsZero;

impl Pred<Zero> for PIsZero {
    type Output = True;
}

impl<N: Nat> Pred<Succ<N>> for PIsZero {
    type Output = False;
}

pub struct PIs<V: Nat>(PhantomData<V>);

impl<N: Nat + NatEq<V>, V: Nat> Pred<N> for PIs<V> {
    type Output = <N as NatEq<V>>::Output;
}

pub struct PIsNot<V: Nat>(PhantomData<V>);

impl<N: Nat + NatEq<V>, V: Nat> Pred<N> for PIsNot<V>
where
    <N as NatEq<V>>::Output: Not,
{
    type Output = <<N as NatEq<V>>::Output as Not>::Output;
}

pub struct PListEq<L: List>(PhantomData<L>);

impl<L: List, Cmp: List> Pred<Cmp> for PListEq<L>
where
    L: ListEq<Cmp>,
{
    type Output = <L as ListEq<Cmp>>::Output;
}

pub struct PLessThan<N: Nat>(PhantomData<N>);

impl<N: Nat, Cmp: Nat> Pred<Cmp> for PLessThan<N>
where
    Cmp: NatLessThan<N>,
{
    type Output = <Cmp as NatLessThan<N>>::Output;
}

pub struct PGreaterEqThan<N: Nat>(PhantomData<N>);

impl<N: Nat, Cmp: Nat> Pred<Cmp> for PGreaterEqThan<N>
where
    Cmp: NatLessThan<N>,
    <Cmp as NatLessThan<N>>::Output: Not,
{
    type Output = <<Cmp as NatLessThan<N>>::Output as Not>::Output;
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
    type L2 = <L1 as Filter<PIsZero>>::Output;
    println!("{}", std::any::type_name::<L2>());

    type L3 = <L1 as Filter<PIs<N2>>>::Output;
    println!("{}", std::any::type_name::<L3>());

    type L4 = <L1 as Filter<PIs<N7>>>::Output;
    println!("{}", std::any::type_name::<L4>());

    type L5 = <L1 as Filter<PIsNot<N3>>>::Output;
    println!("{}", std::any::type_name::<L5>());
}

#[test]
fn test_list_predicate2() {
    // 0, 1, 2, 3
    type L1 = Cons<N0, Cons<N1, Cons<N2, Cons<N3, Nil>>>>;

    type LessThan2 = <L1 as Filter<PLessThan<N1>>>::Output;
    type GreaterEq2 = <L1 as Filter<PGreaterEqThan<N1>>>::Output;

    assert_eq!(LessThan2::to_vec(), vec![0]);
    assert_eq!(GreaterEq2::to_vec(), vec![1, 2, 3]);
}
