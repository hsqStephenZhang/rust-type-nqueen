#![recursion_limit = "1024"]
#![allow(unused)]
use std::{fmt::Display, marker::PhantomData};

trait Nat {}

struct Zero;
impl Nat for Zero {}

struct Succ<N: Nat>(PhantomData<N>);

impl<N: Nat> Nat for Succ<N> {}

type N0 = Zero;
type N1 = Succ<N0>;
type N2 = Succ<N1>;
type N3 = Succ<N2>;
type N4 = Succ<N3>;
type N5 = Succ<N4>;
type N6 = Succ<N5>;
type N7 = Succ<N6>;
type N8 = Succ<N7>;

trait Bool {}
struct True;
struct False;
impl Bool for True {}
impl Bool for False {}

trait And<Rhs: Bool> {
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

trait Or<Rhs: Bool> {
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

trait Val: Nat {
    const VALUE: usize;
}

impl Val for Zero {
    const VALUE: usize = 0;
}

impl<N: Val> Val for Succ<N> {
    const VALUE: usize = <N as Val>::VALUE + 1;
}

trait NatEq<Rhs: Nat> {
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

trait NatLessThan<Rhs: Nat> {
    type Output: Bool;
}

impl NatLessThan<Zero> for Zero {
    type Output = False;
}

impl<R: Nat> NatLessThan<Succ<R>> for Zero {
    type Output = True;
}

impl<R: Nat> NatLessThan<Zero> for Succ<R> {
    type Output = False;
}

impl<L: Nat + NatLessThan<R>, R: Nat> NatLessThan<Succ<R>> for Succ<L> {
    type Output = <L as NatLessThan<R>>::Output;
}

trait Add<Rhs: Nat> {
    type Output: Nat;
}

impl<R: Nat> Add<R> for Zero {
    type Output = R;
}

impl<L: Nat + Add<R>, R: Nat> Add<R> for Succ<L> {
    type Output = Succ<<L as Add<R>>::Output>;
}

trait AbsOfSub<R: Nat> {
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

trait Mul<Rhs: Nat> {
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

trait List {}

struct Nil;
struct Cons<H, T: List>(PhantomData<(H, T)>);

impl List for Nil {}

impl<H, T: List> List for Cons<H, T> {}

trait ListElementAt<Index: Nat> {
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

trait ListEq<R: List> {
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

trait Concat<Rhs> {
    type Output: List;
}

impl<R: List> Concat<R> for Nil {
    type Output = R;
}

// Cons(h, l) ++ r = Cons(h, l ++ r)
impl<H, L: List + Concat<R>, R: List> Concat<R> for Cons<H, L> {
    type Output = Cons<H, <L as Concat<R>>::Output>;
}

trait ListLen: List {
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

trait Contains<T> {
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

trait Pred<X> {
    type Output: Bool;
}

trait Not {
    type Output: Bool;
}

impl Not for False {
    type Output = True;
}

impl Not for True {
    type Output = False;
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

trait Filter<P>: List {
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

trait IfThenElse<Then, Else> {
    type Output;
}

impl<Then, Else> IfThenElse<Then, Else> for True {
    type Output = Then;
}

impl<Then, Else> IfThenElse<Then, Else> for False {
    type Output = Else;
}

// N: base
trait NextBoard<N: Nat> {
    type Output: List;
}

impl<N: Nat> NextBoard<N> for Nil {
    type Output = Nil;
}

impl<H, T, N: Nat> NextBoard<N> for Cons<H, T>
where
    H: Nat,
    Succ<H>: NatLessThan<N>,
    T: List + NextBoard<N>,
    <Succ<H> as NatLessThan<N>>::Output:
        IfThenElse<Cons<Succ<H>, T>, Cons<N0, <T as NextBoard<N>>::Output>>,
    <<Succ<H> as NatLessThan<N>>::Output as IfThenElse<
        Cons<Succ<H>, T>,
        Cons<N0, <T as NextBoard<N>>::Output>,
    >>::Output: List,
{
    // output = condition ? TrueValue : FalseValue
    // condition: <Succ<H> as NatEq<N>>::Output;
    // TrueValue: Cons<Succ<H>, T>
    // FalseValue: Cons<N0, <T as NextBoard<N>>::Output>
    type Output = <<Succ<H> as NatLessThan<N>>::Output as IfThenElse<
        Cons<Succ<H>, T>,
        Cons<N0, <T as NextBoard<N>>::Output>,
    >>::Output;
}

// my solution, not working due to unbounded recursion
// trait AllBoardsN4: List {
//     type Output: List;
// }

// type StartState = Cons<N0, Cons<N0, Cons<N0, Cons<N0, Nil>>>>;

// impl<Board> AllBoardsN4 for Board
// where
//     Board: List + NextBoard<N4> + ListEq<Cons<N0, Cons<N0, Cons<N0, Cons<N0, Nil>>>>>,
//     <Board as NextBoard<N4>>::Output:
//         NextBoard<N4>,
//     <Board as NextBoard<N4>>::Output: AllBoardsN4,
//     <Board as ListEq<StartState>>::Output: IfThenElse<
//             Nil,
//             Nil,
//         >,
//     <<Board as ListEq<Cons<Zero, Cons<Zero, Cons<Zero, Cons<Zero, Nil>>>>>>::Output as IfThenElse<Nil, Nil>>::Output: List
// {
//     // output = condition ? TrueValue : FalseValue
//     // condition: <Board as ListEq<List0000>>::Output
//     // TrueValue: Nil
//     // FalseValue: <NextBoard<Board> as AllBoardsN4>::Output
//     type Output =
//         <<Self as ListEq<StartState>>::Output as IfThenElse<
//             Nil,
//             Nil,
//         >>::Output;
// }

type List0000 = Cons<N0, Cons<N0, Cons<N0, Cons<N0, Nil>>>>;

trait AllBoardsN4: List {
    type Output: List;
}

trait AllBoardsHelper<IsEnd: Bool>: List {
    type Output: List;
}

impl<Board> AllBoardsN4 for Board
where
    Board: NextBoard<N4>,
    // 计算 Next
    <Board as NextBoard<N4>>::Output: ListEq<List0000>,
    // 将 (Next == StartState) 的结果传给 Helper
    Board: AllBoardsHelper<<<Board as NextBoard<N4>>::Output as ListEq<List0000>>::Output>,
{
    type Output = <Board as AllBoardsHelper<
        <<Board as NextBoard<N4>>::Output as ListEq<List0000>>::Output,
    >>::Output;
}

// COVEAT: solution generated by gemini
// IsEnd = True
impl<Board: List> AllBoardsHelper<True> for Board {
    type Output = Cons<Board, Nil>;
}

// IsEnd = False
impl<Board> AllBoardsHelper<False> for Board
where
    Board: List + NextBoard<N4>,
    <Board as NextBoard<N4>>::Output: AllBoardsN4,
{
    type Output = Cons<Board, <<Board as NextBoard<N4>>::Output as AllBoardsN4>::Output>;
}

// nqueen conditions

struct ListUnique;

impl Pred<Nil> for ListUnique {
    type Output = True;
}

impl<H, T> Pred<Cons<H, T>> for ListUnique
where
    H: Nat,
    T: List + Contains<H>,
    <T as Contains<H>>::Output: Not,
    ListUnique: Pred<T>,
    <<T as Contains<H>>::Output as Not>::Output: And<<ListUnique as Pred<T>>::Output>,
{
    // cond: all elements are unique
    // cond1: ! T.contains(H)
    // cond2: ListUnique(T)
    // type Output = <<T as Contains<H>>::Output as Not>::Output;
    type Output = <<<T as Contains<H>>::Output as Not>::Output as And<
        <ListUnique as Pred<T>>::Output,
    >>::Output;
}

struct NqueenDiagonalCheck;

impl Pred<Nil> for NqueenDiagonalCheck {
    type Output = True;
}

impl<H, T> Pred<Cons<H, T>> for NqueenDiagonalCheck
where
    H: Nat,
    T: List,
    T: CheckDiagSafe<H, N1>,
    NqueenDiagonalCheck: Pred<T>,
    <T as CheckDiagSafe<H, N1>>::Output: And<<NqueenDiagonalCheck as Pred<T>>::Output>,
{
    type Output = <<T as CheckDiagSafe<H, N1>>::Output as And<
        <NqueenDiagonalCheck as Pred<T>>::Output,
    >>::Output;
}

trait CheckDiagSafe<TargetRow: Nat, Dist: Nat> {
    type Output: Bool;
}

impl<TargetRow: Nat, Dist: Nat> CheckDiagSafe<TargetRow, Dist> for Nil {
    type Output = True;
}

impl<H, T, TargetRow, Dist> CheckDiagSafe<TargetRow, Dist> for Cons<H, T>
where
    H: Nat,
    TargetRow: Nat,
    Dist: Nat,
    T: List + CheckDiagSafe<TargetRow, Succ<Dist>>,
    H: AbsOfSub<TargetRow>,
    <H as AbsOfSub<TargetRow>>::Output: NatEq<Dist>,
    <<H as AbsOfSub<TargetRow>>::Output as NatEq<Dist>>::Output: Not,
    <<<H as AbsOfSub<TargetRow>>::Output as NatEq<Dist>>::Output as Not>::Output:
        And<<T as CheckDiagSafe<TargetRow, Succ<Dist>>>::Output>,
{
    type Output =
        <<<<H as AbsOfSub<TargetRow>>::Output as NatEq<Dist>>::Output as Not>::Output as And<
            <T as CheckDiagSafe<TargetRow, Succ<Dist>>>::Output,
        >>::Output;
}

// will overflow rustc's stack in `rustc_trait_selection`
// struct SatisfyNqueen;

// impl<Board> Pred<Board> for SatisfyNqueen
// where
//     Board: List,
//     ListUnique: Pred<Board>,
//     NqueenDiagonalCheck: Pred<Board>,
// {
//     // cond1: all elements are unique
//     // cond2: all |vec[i] - vec[]| != |i - j|
//     type Output = <<ListUnique as Pred<Board>>::Output as And<
//         <NqueenDiagonalCheck as Pred<Board>>::Output,
//     >>::Output;
// }

mod fmt {

    use super::*;

    // can be generalized, but i'm not intended to do that
    pub trait ToVec {
        fn to_vec() -> Vec<usize>;
    }

    impl ToVec for Nil {
        fn to_vec() -> Vec<usize> {
            vec![]
        }
    }

    impl<H: Val, T: List + ToVec> ToVec for Cons<H, T> {
        fn to_vec() -> Vec<usize> {
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

    impl<H: List + ToVec, T: List + ToMatrix> ToMatrix for Cons<H, T> {
        fn to_matrix() -> Vec<Vec<usize>> {
            let mut v = vec![H::to_vec()];
            v.extend(T::to_matrix());
            v
        }
    }
}

use fmt::{ToMatrix, ToVec};

#[test]
fn t2() {
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
fn test_bool() {
    type T1 = <True as IfThenElse<N1, N2>>::Output;
    type T2 = <False as IfThenElse<N1, N2>>::Output;
    println!("{}", std::any::type_name::<T1>());
    println!("{}", std::any::type_name::<T2>());
}

#[test]
fn test_list() {
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

#[test]
fn test_nextboard() {
    type L0000 = Cons<N0, Cons<N0, Cons<N0, Cons<N0, Nil>>>>;
    // [0, 0, 0, 0] -> [1, 0, 0, 0]
    type L1000 = <L0000 as NextBoard<N4>>::Output;
    // [1, 0, 0, 0] -> [2, 0, 0, 0]
    type L2000 = <L1000 as NextBoard<N4>>::Output;
    // [2, 0, 0, 0] -> [3, 0, 0, 0]
    type L3000 = <L2000 as NextBoard<N4>>::Output;
    // [3, 0, 0, 0] -> [0, 1, 0, 0]
    type L0100 = <L3000 as NextBoard<N4>>::Output;
    type L1100 = <L0100 as NextBoard<N4>>::Output;
    println!("{:?}", L0000::to_vec());
    println!("{:?}", L1000::to_vec());
    println!("{:?}", L2000::to_vec());
    println!("{:?}", L3000::to_vec());
    println!("{:?}", L0100::to_vec());
    println!("{:?}", L1100::to_vec());

    type L2333 = Cons<N2, Cons<N3, Cons<N3, Cons<N3, Nil>>>>;
    type L3333 = <L2333 as NextBoard<N4>>::Output;
    type L0000Again = <L3333 as NextBoard<N4>>::Output;
    println!("{:?}", L2333::to_vec());
    println!("{:?}", L3333::to_vec());
    println!("{:?}", L0000Again::to_vec());
}

fn main() {
    type L0000 = Cons<N0, Cons<N0, Cons<N0, Cons<N0, Nil>>>>;
    type AllBoards = <L0000 as AllBoardsN4>::Output;
    type CountBoards = <AllBoards as ListLen>::Output;

    let matrix = AllBoards::to_matrix();
    for row in matrix {
        println!("{:?}", row);
    }
    println!("Total boards: {}", CountBoards::VALUE);
    println!("---------------\n");

    type UniqueBoards = <AllBoards as Filter<ListUnique>>::Output;
    println!("Unique Boards:");
    let unique_matrix = UniqueBoards::to_matrix();
    for row in unique_matrix {
        println!("{:?}", row);
    }
    println!("---------------\n");

    type NqueenSolutions = <UniqueBoards as Filter<NqueenDiagonalCheck>>::Output;
    println!("N-Queen Solutions:");
    let nqueen_matrix = NqueenSolutions::to_matrix();
    for row in nqueen_matrix {
        println!("{:?}", row);
    }
    println!("---------------\n");
}
