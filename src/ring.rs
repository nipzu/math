use crate::operators::{BinaryOperator, AbelianBinaryOperator,InvertibleBinaryOperator, InvertibleAbelianBinaryOperator};
use std::marker::PhantomData;
use std::ops;

#[derive(Debug)]
pub struct CommutativeRingElement<S: PartialEq, A: InvertibleAbelianBinaryOperator<S>, M: AbelianBinaryOperator<S>> {
    element: S,
    additive_operator: PhantomData<A>,
    multiplicative_operator: PhantomData<M>,
}

impl<S: PartialEq, A: InvertibleAbelianBinaryOperator<S>, M: AbelianBinaryOperator<S>> ops::Mul<&CommutativeRingElement<S,A,M>> for &CommutativeRingElement<S, A,M> {
    type Output = CommutativeRingElement<S, A, M>;
    fn mul(self,  other: &CommutativeRingElement<S, A, M>) -> Self::Output {
        CommutativeRingElement {
            element: M::apply(&self.element, &other.element),
            additive_operator: PhantomData,
            multiplicative_operator: PhantomData,
        }
    }
}

impl<S: PartialEq, A: InvertibleAbelianBinaryOperator<S>, M: AbelianBinaryOperator<S>> PartialEq for CommutativeRingElement<S, A,M> {
    fn eq(&self,  other: &CommutativeRingElement<S, A,M>) -> bool {
        self.element == other.element
    }
}


pub type GaussianIntegers = CommutativeRingElement<(i64, i64), ComplexAddition, ComplexMultiplication>;

impl GaussianIntegers {
    fn new(a: i64, b: i64) -> Self {
        GaussianIntegers {
            element: (a, b),
            additive_operator: PhantomData::<ComplexAddition>,
            multiplicative_operator: PhantomData::<ComplexMultiplication>,
        }
    }
}
#[derive(Debug)]
pub struct ComplexAddition {
}

impl BinaryOperator<(i64,i64)> for ComplexAddition {
    fn get_identity() -> (i64,i64) {(0,0)}
    fn apply(left: &(i64,i64), right: &(i64,i64)) -> (i64,i64) {
        (left.0+right.0, left.1+right.1)
    }
}

impl AbelianBinaryOperator<(i64,i64)> for ComplexAddition {}

impl InvertibleBinaryOperator<(i64,i64)> for ComplexAddition {
    fn get_inverse(val: &(i64,i64)) -> (i64,i64) {
        (-val.0,-val.1)
    }
}


#[derive(Debug)]
pub struct ComplexMultiplication {
}

impl BinaryOperator<(i64,i64)> for ComplexMultiplication {
    fn get_identity() -> (i64,i64) {(1,0)}
    fn apply(left: &(i64,i64), right: &(i64,i64)) -> (i64,i64) {
        (left.0*right.0-left.1*right.1, left.0*right.1+left.1*right.0)
    }
}

impl AbelianBinaryOperator<(i64,i64)> for ComplexMultiplication {}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::ring::GaussianIntegers;
        type G = GaussianIntegers;
        let a = G::new(18, 2);
        let b = G::new(8, -3);
        assert_eq!(&a*&b, G::new(150, -38));
    }
}
