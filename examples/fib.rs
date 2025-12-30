#![recursion_limit = "1024"]

use rust_type_gyms::nat::*;

trait Fib {
    type Output: Nat;
}

impl Fib for N0 {
    type Output = N1;
}

impl Fib for N1 {
    type Output = N1;
}

impl<N> Fib for Succ<Succ<N>>
where
    N: Nat + Fib,
    Succ<N>: Nat + Fib,
    <N as Fib>::Output: Add<<Succ<N> as Fib>::Output>,
{
    type Output = <<N as Fib>::Output as Add<<Succ<N> as Fib>::Output>>::Output;
}

fn main() {
    println!("Fib(0) = {}", <N0 as Fib>::Output::VALUE);
    println!("Fib(1) = {}", <N1 as Fib>::Output::VALUE);
    println!("Fib(2) = {}", <N2 as Fib>::Output::VALUE);
    println!("Fib(3) = {}", <N3 as Fib>::Output::VALUE);
    println!("Fib(4) = {}", <N4 as Fib>::Output::VALUE);
    println!("Fib(5) = {}", <N5 as Fib>::Output::VALUE);
    println!("Fib(6) = {}", <N6 as Fib>::Output::VALUE);
    println!("Fib(7) = {}", <N7 as Fib>::Output::VALUE);
    println!("Fib(8) = {}", <N8 as Fib>::Output::VALUE);
}
