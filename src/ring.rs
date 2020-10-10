use crate::operators::{AbelianBinaryOperator, BinaryOperator};
use std::marker::PhantomData;

struct RingElement<R, A: AbelianBinaryOperator<R>, M: BinaryOperator<R>> {
    element: R,
    additive_operator: PhantomData<A>,
    multiplicative_operator: PhantomData<M>,
}