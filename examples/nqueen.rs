#![recursion_limit = "65536"]

use rust_type_gyms::bool::*;
use rust_type_gyms::list::*;
use rust_type_gyms::logic::*;
use rust_type_gyms::nat::*;

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

// Generic helper traits for board generation
trait AllBoards<N: Nat, StartState: List>: List {
    type Output: List;
}

trait AllBoardsHelper<N: Nat, StartState: List, IsEnd: Bool>: List {
    type Output: List;
}

// Check if a board is valid for N-queens
trait IsValidBoard: List {
    type Output: Bool;
}

impl<Board, N: Nat, StartState: List> AllBoards<N, StartState> for Board
where
    Board: NextBoard<N>,
    <Board as NextBoard<N>>::Output: ListEq<StartState>,
    Board: AllBoardsHelper<
            N,
            StartState,
            <<Board as NextBoard<N>>::Output as ListEq<StartState>>::Output,
        >,
{
    type Output = <Board as AllBoardsHelper<
        N,
        StartState,
        <<Board as NextBoard<N>>::Output as ListEq<StartState>>::Output,
    >>::Output;
}

impl<Board> IsValidBoard for Board
where
    Board: List,
    ListUnique: Pred<Board>,
    NqueenDiagonalCheck: Pred<Board>,
    <ListUnique as Pred<Board>>::Output: And<<NqueenDiagonalCheck as Pred<Board>>::Output>,
{
    type Output = <<ListUnique as Pred<Board>>::Output as And<
        <NqueenDiagonalCheck as Pred<Board>>::Output,
    >>::Output;
}

// IsEnd = True
impl<Board: List, N: Nat, StartState: List> AllBoardsHelper<N, StartState, True> for Board
where
    Board: IsValidBoard,
    <Board as IsValidBoard>::Output: IfThenElse<Cons<Board, Nil>, Nil>,
    <<Board as IsValidBoard>::Output as rust_type_gyms::logic::IfThenElse<
        rust_type_gyms::list::Cons<Board, rust_type_gyms::list::Nil>,
        rust_type_gyms::list::Nil,
    >>::Output: rust_type_gyms::list::List,
{
    type Output = <<Board as IsValidBoard>::Output as IfThenElse<Cons<Board, Nil>, Nil>>::Output;
}

// IsEnd = False
impl<Board, N: Nat, StartState: List> AllBoardsHelper<N, StartState, False> for Board
where
    Board: List + NextBoard<N> + IsValidBoard,
    <Board as NextBoard<N>>::Output: AllBoards<N, StartState>,
    <Board as IsValidBoard>::Output: IfThenElse<
            Cons<Board, <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output>,
            <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output,
        >,
    <<Board as IsValidBoard>::Output as rust_type_gyms::logic::IfThenElse<
        rust_type_gyms::list::Cons<
            Board,
            <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output,
        >,
        <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output,
    >>::Output: rust_type_gyms::list::List,
{
    type Output = <<Board as IsValidBoard>::Output as IfThenElse<
        Cons<Board, <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output>,
        <<Board as NextBoard<N>>::Output as AllBoards<N, StartState>>::Output,
    >>::Output;
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

// Solution trait for different N values
trait Solution {
    type Output: List;
}

// Macro to generate N-queens solution for any N
macro_rules! impl_nqueen_solution {
    ($n:ty, [$($zero:tt)*]) => {
        paste::paste! {
            type [<StartState $n>] = impl_nqueen_solution!(@cons $($zero)*);
            impl Solution for $n {
                type Output = <[<StartState $n>] as AllBoards<$n, [<StartState $n>]>>::Output;
            }
        }
    };

    // Helper to build nested Cons types
    (@cons N0 $($rest:tt)*) => {
        Cons<N0, impl_nqueen_solution!(@cons $($rest)*)>
    };
    (@cons) => {
        Nil
    };
}

impl_nqueen_solution!(N4, [N0 N0 N0 N0]);
impl_nqueen_solution!(N5, [N0 N0 N0 N0 N0]);
// impl_nqueen_solution!(N6, [N0 N0 N0 N0 N0 N0]);
// impl_nqueen_solution!(N7, [N0 N0 N0 N0 N0 N0 N0]);
// impl_nqueen_solution!(N8, [N0 N0 N0 N0 N0 N0 N0 N0]);

fn print_board(board_solutions: Vec<Vec<usize>>) {
    println!("Found {} solution(s):", board_solutions.len());
    for (i, board) in board_solutions.iter().enumerate() {
        println!("Solution {}: {:?}", i + 1, board);
    }
}

// comment out `impl_nqueen_solution!(N5, [N0 N0 N0 N0 N0]);` and match arm for `5` for faster compile time
fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    let _program = args.iter().skip(1);
    let num: u8 = args.pop().expect("Enter a number").parse().expect("Enter a number");

    // try other N values, but its extremely slow and may overflow rustc stack
    match num {
        4 => {
            let solutions = <N4 as Solution>::Output::to_matrix();
            print_board(solutions);
        }
        5 => {
            let solutions = <N5 as Solution>::Output::to_matrix();
            print_board(solutions);
        }
        _ => unimplemented!()
    };
}
