#[derive(Clone)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> darling::FromMeta for Either<A, B>
    where
        A: darling::FromMeta,
        B: darling::FromMeta {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        let mut errors = Vec::new();

        match A::from_meta(item) {
            Ok(a) => return Ok(Self::A(a)),
            Err(err) => errors.push(err),
        }
        match B::from_meta(item) {
            Ok(b) => return Ok(Self::B(b)),
            Err(err) => errors.push(err),
        }

        errors.push(darling::Error::custom("expected either a bool or an identifier"));
        Err(darling::Error::multiple(errors))
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let mut errors = Vec::new();

        match A::from_value(value) {
            Ok(a) => return Ok(Self::A(a)),
            Err(err) => errors.push(err),
        }
        match B::from_value(value) {
            Ok(b) => return Ok(Self::B(b)),
            Err(err) => errors.push(err),
        }

        errors.push(darling::Error::custom("expected either a bool or an identifier"));
        Err(darling::Error::multiple(errors))
    }
}
