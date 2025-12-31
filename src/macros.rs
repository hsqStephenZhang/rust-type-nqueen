#[macro_export]
macro_rules! list {
    () => { crate::list::Nil };

    ($head:ty $(,)?) => {
        crate::list::Cons<$head, crate::list::Nil>
    };

    ($head:ty, $($tail:ty),* $(,)?) => {
        crate::list::Cons<$head, list!($($tail),*)>
    };
}
