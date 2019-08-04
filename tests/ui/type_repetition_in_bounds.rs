#[deny(clippy::type_repetition_in_bounds)]

pub fn foo<T>(_t: T)
where
    T: Copy,
    T: Clone,
{
    unimplemented!();
}

pub fn bar<T, U>(_t: T, _u: U)
where
    T: Copy,
    U: Clone,
{
    unimplemented!();
}

pub struct FooBar<T>(T);

pub struct TwoFooBars<A, B> {
    a: FooBar<A>,
    b: FooBar<B>,
}

impl<A, B> Unpin for TwoFooBars<A, B>
where
    FooBar<A>: Unpin,
    FooBar<B>: Unpin,
{
}

fn main() {}
