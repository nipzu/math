use crate::operators::{BinaryOperator, AbelianBinaryOperator, InvertibleAbelianBinaryOperator, InvertibleBinaryOperator};
use std::fmt;
use std::marker::PhantomData;
use std::ops;

/// A struct representing an element of a group.
/// The group is determined by the set of possible elements which can be
/// either S or a subset of it, and the binary operator acting on elements of S.
pub struct GroupElement<S: PartialEq, O: InvertibleBinaryOperator<S>> {
    element: S,
    operator: PhantomData<O>,
}

pub trait AbelianGroup {}

impl<S: PartialEq, O: InvertibleAbelianBinaryOperator<S>> AbelianGroup for GroupElement<S, O> {}

impl<S: PartialEq, O: InvertibleBinaryOperator<S>> ops::Mul<&GroupElement<S, O>> for &GroupElement<S, O> {
    type Output = GroupElement<S, O>;
    fn mul(self, other: &GroupElement<S, O>) -> Self::Output {
        GroupElement {
            element: O::apply(&self.element, &other.element),
            operator: PhantomData,
        }
    }
}

impl<S: PartialEq, O: InvertibleBinaryOperator<S>> PartialEq for GroupElement<S,O> {
    fn eq(&self,  other: &GroupElement<S,O>) -> bool {
        self.element == other.element
    }
}

pub type CyclicGroupElement<const N: u64> = GroupElement<u64, CyclicOperator<{ N }>>;

impl<const N: u64> CyclicGroupElement<{ N }> {
    pub fn new(val: u64) -> Self {
        CyclicGroupElement {
            element: val % N,
            operator: PhantomData,
        }
    }
}

impl<const N: u64> fmt::Display for CyclicGroupElement<{ N }> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl<const N: u64> fmt::Debug for CyclicGroupElement<{ N }> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.element)
    }
}

#[derive(Debug)]
pub struct CyclicOperator<const N: u64>;

impl<const N: u64> AbelianBinaryOperator<u64> for CyclicOperator<{ N }> {}

impl<const N: u64> AbelianGroup for CyclicOperator<{N}> {}

impl<const N: u64> BinaryOperator<u64> for CyclicOperator<{ N }> {
    fn get_identity() -> u64 {
        0
    }
    fn apply(left: &u64, right: &u64) -> u64 {
        let r = left + right;
        if r >= N {
            r - N
        } else {
            r
        }
    }
}

impl<const N: u64> InvertibleBinaryOperator<u64> for CyclicOperator<{ N }> {
    fn get_inverse(element: &u64) -> u64 {
        N - element
    }
}
