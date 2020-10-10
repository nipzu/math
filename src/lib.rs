#![feature(min_const_generics)]
#![feature(trait_alias)]

mod field;
mod group;
mod operators;
mod polynomial;
mod ring;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        type Z11 = crate::group::CyclicGroupElement<11>;
        let a = Z11::new(18);
        let b = Z11::new(8);
        assert_eq!(&a*&b, Z11::new(4));
    }
}
