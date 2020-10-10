use crate::operators::{BinaryOperator, InvertibleAbelianBinaryOperator};
use std::marker::PhantomData;

struct RingElement<R, A: InvertibleAbelianBinaryOperator<R>, M: BinaryOperator<R>> {
    element: R,
    additive_operator: PhantomData<A>,
    multiplicative_operator: PhantomData<M>,
}
