/// function signatures:
///   QuickSort: List -> List
///   Partition: Nat -> List -> (List, Nat, List)
///   Concat: List -> List -> List
use rust_type_gyms::list::*;
use rust_type_gyms::logic::*;
use rust_type_gyms::nat::*;

trait QuickSort {
    type Output: List;
}

impl QuickSort for Nil {
    type Output = Nil;
}

impl<Head, Tail> QuickSort for Cons<Head, Tail>
where
    Head: Nat,
    Tail: List + QuickSort + Partition<Head>,
    <Tail as Partition<Head>>::Less: QuickSort,
    <Tail as Partition<Head>>::GreaterOrEqual: QuickSort,
    <Tail as Partition<Head>>::Less:
        Concat<Cons<Head, <<Tail as Partition<Head>>::GreaterOrEqual as QuickSort>::Output>>,
    <<Tail as Partition<Head>>::Less as QuickSort>::Output:
        Concat<Cons<Head, <<Tail as Partition<Head>>::GreaterOrEqual as QuickSort>::Output>>,
{
    // left:
    // type Output = <<Tail as Partition<Head>>::Less as QuickSort>::Output;
    // right:
    // type Output = <<Tail as Partition<Head>>::GreaterOrEqual as QuickSort>::Output;
    // combined:
    type Output = <<<Tail as Partition<Head>>::Less as QuickSort>::Output as Concat<
        Cons<Head, <<Tail as Partition<Head>>::GreaterOrEqual as QuickSort>::Output>,
    >>::Output;
}

trait Partition<Pivot: Nat> {
    type Less: List;
    type GreaterOrEqual: List;
}

impl<Pivot: Nat> Partition<Pivot> for Nil {
    type Less = Nil;
    type GreaterOrEqual = Nil;
}

impl<Head: Nat, Tail, Pivot: Nat> Partition<Pivot> for Cons<Head, Tail>
where
    <Tail as Filter<PLessThan<Pivot>>>::Output: List,
    <Tail as Filter<PGreaterEqThan<Pivot>>>::Output: List,
    Tail: List + Filter<PGreaterEqThan<Pivot>> + Filter<PLessThan<Pivot>>,
    Cons<Head, Tail>: List + Filter<PGreaterEqThan<Pivot>> + Filter<PLessThan<Pivot>>,
{
    type Less = <Cons<Head, Tail> as Filter<PLessThan<Pivot>>>::Output;
    type GreaterOrEqual = <Cons<Head, Tail> as Filter<PGreaterEqThan<Pivot>>>::Output;
}

#[test]
fn test_partition() {
    {
        type L1 = Cons<N3, Nil>;
        type Pivot = N3;
        type Part = <L1 as Partition<Pivot>>::Less;
        type PartGE = <L1 as Partition<Pivot>>::GreaterOrEqual;
        assert_eq!(Part::to_vec(), vec![]);
        assert_eq!(PartGE::to_vec(), vec![]);
    }

    {
        // 3,1
        // pivot = 3, less = 1; mid = 3; greater = []
        type L = Cons<N3, Cons<N1, Nil>>;
        type Pivot = N3;
        type Part = <L as Partition<Pivot>>::Less;
        type PartGE = <L as Partition<Pivot>>::GreaterOrEqual;
        assert_eq!(Part::to_vec(), vec![1]);
        assert_eq!(PartGE::to_vec(), vec![]);

        type Merged = <Part as Concat<Cons<Pivot, PartGE>>>::Output;
        assert_eq!(Merged::to_vec(), vec![1, 3]);
    }

    {
        // 3,4,5,2,1
        // pivot = 3, less = 2,1; mid = 3; greater = 4,5
        type L = Cons<N3, Cons<N4, Cons<N5, Cons<N2, Cons<N1, Nil>>>>>;
        type Pivot = N3;
        type Part = <L as Partition<Pivot>>::Less;
        type PartGE = <L as Partition<Pivot>>::GreaterOrEqual;
        assert_eq!(Part::to_vec(), vec![2, 1]);
        assert_eq!(PartGE::to_vec(), vec![4, 5]);

        type Merged = <Part as Concat<Cons<Pivot, PartGE>>>::Output;
        assert_eq!(Merged::to_vec(), vec![2, 1, 3, 4, 5]);
    }
}

fn main() {
    // 3
    type L = Cons<N3, Nil>;
    type Sorted = <L as QuickSort>::Output;
    println!("Original: {:?}", L::to_vec());
    println!("Sorted: {:?}", Sorted::to_vec());

    // 3, 1
    type L2 = Cons<N3, Cons<N1, Nil>>;
    type Sorted2 = <L2 as QuickSort>::Output;
    println!("Original: {:?}", L2::to_vec());
    println!("Sorted: {:?}", Sorted2::to_vec());
    assert_eq!(Sorted2::to_vec(), vec![1, 3]);

    // 3, 4, 5, 2, 1
    type L3 = Cons<N3, Cons<N4, Cons<N5, Cons<N2, Cons<N1, Nil>>>>>;
    type Sorted3 = <L3 as QuickSort>::Output;
    println!("Original: {:?}", L3::to_vec());
    println!("Sorted: {:?}", Sorted3::to_vec());
    assert_eq!(Sorted3::to_vec(), vec![1, 2, 3, 4, 5]);

    // 8, 7, 6, 5, 4, 3, 2, 1
    type L4 = Cons<N8, Cons<N7, Cons<N6, Cons<N5, Cons<N4, Cons<N3, Cons<N2, Cons<N1, Nil>>>>>>>>;
    type Sorted4 = <L4 as QuickSort>::Output;
    println!("Original: {:?}", L4::to_vec());
    println!("Sorted: {:?}", Sorted4::to_vec());
    assert_eq!(Sorted4::to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
