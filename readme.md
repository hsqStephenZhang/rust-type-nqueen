# rust type system exercises

solving problem like nqueen, quicksort, fibonacci in rust type system.

## Goal

1. practice and deep my understanding of rust's type system
2. have fun
3. explore what rust can do and what rust cannot do in type system

the 3rd is only partly achieved, projects like [lisp-in-types](https://github.com/playX18/lisp-in-types) did make a solid exploration on the boundary of rust's type system's expressiveness.

## Solutions

### Nqueen

1. iterate all possible combinations.
2. filter the valid ones.

### QuickSort

1. partition the list at some point.
2. sort the partitioned left and right part recursively.
3. merge the final results.

### Fibonacci

1. simple recursive function encoding.

## Limitations

1. the trait bound looks like a mess, it is indeed unreadable. But thankfully, you don't have to worry about that much, since our compiler are smart enough to tell you how to write the correct one.
2. we did not inplemented data structures like map, which could be useful if you want to write more advanced applications like `expression evaluator`

## Similar Projects

1. [lisp-in-types](https://github.com/playX18/lisp-in-types)
2. [type-exercise-in-rust](https://github.com/skyzh/type-exercise-in-rust)
