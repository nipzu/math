pub trait BinaryOperator<S> {
    fn get_identity() -> S;
    fn apply(left: &S, right: &S) -> S;
}

pub trait InvertibleBinaryOperator<S>: BinaryOperator<S> {
    fn get_inverse(element: &S) -> S;
}
pub trait AbelianBinaryOperator<S>: BinaryOperator<S> {}

pub trait InvertibleAbelianBinaryOperator<S> = InvertibleBinaryOperator<S> + AbelianBinaryOperator<S>;