pub trait BinaryOperator<S> {
    fn get_identity() -> S;
    fn apply(left: &S, right: &S) -> S;
}

pub unsafe trait AbelianBinaryOperator<S> : BinaryOperator<S> {

}