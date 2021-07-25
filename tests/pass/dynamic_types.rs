use for_any::ForAny;
use std::any::Any;

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum EmptyEnum {}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum EmptyVariants {
    A,
    B,
    C,
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum EmptyTupleVariants {
    A(),
    B(),
    C(),
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum SingleTypeTupleVariants {
    A(String),
    B(String),
    C(String),
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum MultiTypeTupleVariants {
    A(String, i32, String),
    B(String, i32, String),
    C(String, i32, String),
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum SingleGenericTupleVariants<T> {
    A(T),
    B(T),
    C(T),
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum MultiGenericTupleVariants<T, E> {
    A(T, E, T),
    B(T, E, T),
    C(T, E, T),
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum EmptyStructVariants {
    A {},
    B {},
    C {},
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum SingleTypeStructVariants {
    A { a: String },
    B { b: String },
    C { c: String },
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum MultiTypeStructVariants {
    A { a: String, b: i32, c: String },
    B { b: String, c: i32, a: String },
    C { c: String, a: i32, b: String },
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum SingleGenericStructVariants<T> {
    A { a: T },
    B { b: T },
    C { c: T },
}

#[derive(ForAny)]
#[for_any(generic = "Any")]
enum MultiGenericStructVariants<T, E> {
    A { a: T, b: E, c: T },
    B { b: T, c: E, a: T },
    C { c: T, a: E, b: T },
}

fn main() {}
