use for_any::ForAny;

#[derive(ForAny)]
enum EmptyEnum {}

#[derive(ForAny)]
enum EmptyVariants {
    A,
    B,
    C,
}

#[derive(ForAny)]
enum EmptyTupleVariants {
    A(),
    B(),
    C(),
}

#[derive(ForAny)]
enum SingleTypeTupleVariants {
    A(&'static str),
    B(&'static str),
    C(&'static str),
}

#[derive(ForAny)]
enum MultiTypeTupleVariants {
    A(&'static str, i32, &'static str),
    B(&'static str, i32, &'static str),
    C(&'static str, i32, &'static str),
}

#[derive(ForAny)]
enum SingleGenericTupleVariants<T> {
    A(T),
    B(T),
    C(T),
}

#[derive(ForAny)]
enum MultiGenericTupleVariants<T, E> {
    A(T, E, T),
    B(T, E, T),
    C(T, E, T),
}

#[derive(ForAny)]
enum EmptyStructVariants {
    A {},
    B {},
    C {},
}

#[derive(ForAny)]
enum SingleTypeStructVariants {
    A { a: &'static str },
    B { b: &'static str },
    C { c: &'static str },
}

#[derive(ForAny)]
enum MultiTypeStructVariants {
    A { a: &'static str, b: i32, c: &'static str },
    B { b: &'static str, c: i32, a: &'static str },
    C { c: &'static str, a: i32, b: &'static str },
}

#[derive(ForAny)]
enum SingleGenericStructVariants<T> {
    A { a: T },
    B { b: T },
    C { c: T },
}

#[derive(ForAny)]
enum MultiGenericStructVariants<T, E> {
    A { a: T, b: E, c: T },
    B { b: T, c: E, a: T },
    C { c: T, a: E, b: T },
}

fn main() {
    /* EmptyEnum */

    let _func = EmptyEnum::for_any::<(), fn() -> ()>;
    let _func = EmptyEnum::for_any_ref::<(), fn() -> ()>;
    let _func = EmptyEnum::for_any_mut::<(), fn() -> ()>;

    let _func = EmptyEnum::for_any::<bool, fn() -> bool>;
    let _func = EmptyEnum::for_any_ref::<bool, fn() -> bool>;
    let _func = EmptyEnum::for_any_mut::<bool, fn() -> bool>;

    /* EmptyVariants */

    let _: () = EmptyVariants::A.for_any(|| {});
    let _: bool = EmptyVariants::A.for_any(|| true);
    let _: () = EmptyVariants::B.for_any(|| {});
    let _: bool = EmptyVariants::B.for_any(|| true);
    let _: () = EmptyVariants::C.for_any(|| {});
    let _: bool = EmptyVariants::C.for_any(|| true);

    let _: () = EmptyVariants::A.for_any_ref(|| {});
    let _: bool = EmptyVariants::A.for_any_ref(|| true);
    let _: () = EmptyVariants::B.for_any_ref(|| {});
    let _: bool = EmptyVariants::B.for_any_ref(|| true);
    let _: () = EmptyVariants::C.for_any_ref(|| {});
    let _: bool = EmptyVariants::C.for_any_ref(|| true);

    let _: () = EmptyVariants::A.for_any_mut(|| {});
    let _: bool = EmptyVariants::A.for_any_mut(|| true);
    let _: () = EmptyVariants::B.for_any_mut(|| {});
    let _: bool = EmptyVariants::B.for_any_mut(|| true);
    let _: () = EmptyVariants::C.for_any_mut(|| {});
    let _: bool = EmptyVariants::C.for_any_mut(|| true);

    /* EmptyTupleVariants */

    let _: () = EmptyTupleVariants::A().for_any(|| {});
    let _: bool = EmptyTupleVariants::A().for_any(|| true);
    let _: () = EmptyTupleVariants::B().for_any(|| {});
    let _: bool = EmptyTupleVariants::B().for_any(|| true);
    let _: () = EmptyTupleVariants::C().for_any(|| {});
    let _: bool = EmptyTupleVariants::C().for_any(|| true);

    let _: () = EmptyTupleVariants::A().for_any_ref(|| {});
    let _: bool = EmptyTupleVariants::A().for_any_ref(|| true);
    let _: () = EmptyTupleVariants::B().for_any_ref(|| {});
    let _: bool = EmptyTupleVariants::B().for_any_ref(|| true);
    let _: () = EmptyTupleVariants::C().for_any_ref(|| {});
    let _: bool = EmptyTupleVariants::C().for_any_ref(|| true);

    let _: () = EmptyTupleVariants::A().for_any_mut(|| {});
    let _: bool = EmptyTupleVariants::A().for_any_mut(|| true);
    let _: () = EmptyTupleVariants::B().for_any_mut(|| {});
    let _: bool = EmptyTupleVariants::B().for_any_mut(|| true);
    let _: () = EmptyTupleVariants::C().for_any_mut(|| {});
    let _: bool = EmptyTupleVariants::C().for_any_mut(|| true);

    /* SingleTypeTupleVariants */

    let _: () = SingleTypeTupleVariants::A("test").for_any(|_: &'static str| {});
    let _: bool = SingleTypeTupleVariants::A("test").for_any(|_: &'static str| true);
    let _: () = SingleTypeTupleVariants::B("test").for_any(|_: &'static str| {});
    let _: bool = SingleTypeTupleVariants::B("test").for_any(|_: &'static str| true);
    let _: () = SingleTypeTupleVariants::C("test").for_any(|_: &'static str| {});
    let _: bool = SingleTypeTupleVariants::C("test").for_any(|_: &'static str| true);

    let _: () = SingleTypeTupleVariants::A("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeTupleVariants::A("test").for_any_ref(|_: &&'static str| true);
    let _: () = SingleTypeTupleVariants::B("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeTupleVariants::B("test").for_any_ref(|_: &&'static str| true);
    let _: () = SingleTypeTupleVariants::C("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeTupleVariants::C("test").for_any_ref(|_: &&'static str| true);

    let _: () = SingleTypeTupleVariants::A("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeTupleVariants::A("test").for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleTypeTupleVariants::B("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeTupleVariants::B("test").for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleTypeTupleVariants::C("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeTupleVariants::C("test").for_any_mut(|_: &mut &'static str| true);

    /* MultiTypeTupleVariants */

    let _: () = MultiTypeTupleVariants::A("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeTupleVariants::A("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiTypeTupleVariants::B("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeTupleVariants::B("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiTypeTupleVariants::C("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeTupleVariants::C("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);

    let _: () = MultiTypeTupleVariants::A("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeTupleVariants::A("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiTypeTupleVariants::B("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeTupleVariants::B("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiTypeTupleVariants::C("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeTupleVariants::C("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);

    let _: () = MultiTypeTupleVariants::A("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeTupleVariants::A("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiTypeTupleVariants::B("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeTupleVariants::B("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiTypeTupleVariants::C("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeTupleVariants::C("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);

    /* SingleGenericTupleVariants */

    let _: () = SingleGenericTupleVariants::A("test").for_any(|_: &'static str| {});
    let _: bool = SingleGenericTupleVariants::A("test").for_any(|_: &'static str| true);
    let _: () = SingleGenericTupleVariants::B("test").for_any(|_: &'static str| {});
    let _: bool = SingleGenericTupleVariants::B("test").for_any(|_: &'static str| true);
    let _: () = SingleGenericTupleVariants::C("test").for_any(|_: &'static str| {});
    let _: bool = SingleGenericTupleVariants::C("test").for_any(|_: &'static str| true);

    let _: () = SingleGenericTupleVariants::A("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericTupleVariants::A("test").for_any_ref(|_: &&'static str| true);
    let _: () = SingleGenericTupleVariants::B("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericTupleVariants::B("test").for_any_ref(|_: &&'static str| true);
    let _: () = SingleGenericTupleVariants::C("test").for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericTupleVariants::C("test").for_any_ref(|_: &&'static str| true);

    let _: () = SingleGenericTupleVariants::A("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericTupleVariants::A("test").for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleGenericTupleVariants::B("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericTupleVariants::B("test").for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleGenericTupleVariants::C("test").for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericTupleVariants::C("test").for_any_mut(|_: &mut &'static str| true);

    /* MultiGenericTupleVariants */

    let _: () = MultiGenericTupleVariants::A("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericTupleVariants::A("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiGenericTupleVariants::B("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericTupleVariants::B("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiGenericTupleVariants::C("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericTupleVariants::C("test", 42, "test").for_any(|_: &'static str, _: i32, _: &'static str| true);

    let _: () = MultiGenericTupleVariants::A("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericTupleVariants::A("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiGenericTupleVariants::B("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericTupleVariants::B("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiGenericTupleVariants::C("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericTupleVariants::C("test", 42, "test").for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);

    let _: () = MultiGenericTupleVariants::A("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericTupleVariants::A("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiGenericTupleVariants::B("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericTupleVariants::B("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiGenericTupleVariants::C("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericTupleVariants::C("test", 42, "test").for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);

    /* EmptyStructVariants */

    let _: () = EmptyStructVariants::A {}.for_any(|| {});
    let _: bool = EmptyStructVariants::A {}.for_any(|| true);
    let _: () = EmptyStructVariants::B {}.for_any(|| {});
    let _: bool = EmptyStructVariants::B {}.for_any(|| true);
    let _: () = EmptyStructVariants::C {}.for_any(|| {});
    let _: bool = EmptyStructVariants::C {}.for_any(|| true);

    let _: () = EmptyStructVariants::A {}.for_any_ref(|| {});
    let _: bool = EmptyStructVariants::A {}.for_any_ref(|| true);
    let _: () = EmptyStructVariants::B {}.for_any_ref(|| {});
    let _: bool = EmptyStructVariants::B {}.for_any_ref(|| true);
    let _: () = EmptyStructVariants::C {}.for_any_ref(|| {});
    let _: bool = EmptyStructVariants::C {}.for_any_ref(|| true);

    let _: () = EmptyStructVariants::A {}.for_any_mut(|| {});
    let _: bool = EmptyStructVariants::A {}.for_any_mut(|| true);
    let _: () = EmptyStructVariants::B {}.for_any_mut(|| {});
    let _: bool = EmptyStructVariants::B {}.for_any_mut(|| true);
    let _: () = EmptyStructVariants::C {}.for_any_mut(|| {});
    let _: bool = EmptyStructVariants::C {}.for_any_mut(|| true);

    /* SingleTypeStructVariants */

    let _: () = SingleTypeStructVariants::A { a: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleTypeStructVariants::A { a: "test" }.for_any(|_: &'static str| true);
    let _: () = SingleTypeStructVariants::B { b: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleTypeStructVariants::B { b: "test" }.for_any(|_: &'static str| true);
    let _: () = SingleTypeStructVariants::C { c: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleTypeStructVariants::C { c: "test" }.for_any(|_: &'static str| true);

    let _: () = SingleTypeStructVariants::A { a: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeStructVariants::A { a: "test" }.for_any_ref(|_: &&'static str| true);
    let _: () = SingleTypeStructVariants::B { b: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeStructVariants::B { b: "test" }.for_any_ref(|_: &&'static str| true);
    let _: () = SingleTypeStructVariants::C { c: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleTypeStructVariants::C { c: "test" }.for_any_ref(|_: &&'static str| true);

    let _: () = SingleTypeStructVariants::A { a: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeStructVariants::A { a: "test" }.for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleTypeStructVariants::B { b: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeStructVariants::B { b: "test" }.for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleTypeStructVariants::C { c: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleTypeStructVariants::C { c: "test" }.for_any_mut(|_: &mut &'static str| true);

    /* MultiTypeStructVariants */

    let _: () = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);

    let _: () = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);

    let _: () = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeStructVariants::A { a: "test", b: 42, c: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeStructVariants::B { b: "test", c: 42, a: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiTypeStructVariants::C { c: "test", a: 42, b: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);

    /* SingleGenericStructVariants */

    let _: () = SingleGenericStructVariants::A { a: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleGenericStructVariants::A { a: "test" }.for_any(|_: &'static str| true);
    let _: () = SingleGenericStructVariants::B { b: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleGenericStructVariants::B { b: "test" }.for_any(|_: &'static str| true);
    let _: () = SingleGenericStructVariants::C { c: "test" }.for_any(|_: &'static str| {});
    let _: bool = SingleGenericStructVariants::C { c: "test" }.for_any(|_: &'static str| true);

    let _: () = SingleGenericStructVariants::A { a: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericStructVariants::A { a: "test" }.for_any_ref(|_: &&'static str| true);
    let _: () = SingleGenericStructVariants::B { b: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericStructVariants::B { b: "test" }.for_any_ref(|_: &&'static str| true);
    let _: () = SingleGenericStructVariants::C { c: "test" }.for_any_ref(|_: &&'static str| {});
    let _: bool = SingleGenericStructVariants::C { c: "test" }.for_any_ref(|_: &&'static str| true);

    let _: () = SingleGenericStructVariants::A { a: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericStructVariants::A { a: "test" }.for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleGenericStructVariants::B { b: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericStructVariants::B { b: "test" }.for_any_mut(|_: &mut &'static str| true);
    let _: () = SingleGenericStructVariants::C { c: "test" }.for_any_mut(|_: &mut &'static str| {});
    let _: bool = SingleGenericStructVariants::C { c: "test" }.for_any_mut(|_: &mut &'static str| true);

    /* MultiGenericStructVariants */

    let _: () = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);
    let _: () = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| {});
    let _: bool = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any(|_: &'static str, _: i32, _: &'static str| true);

    let _: () = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);
    let _: () = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| {});
    let _: bool = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any_ref(|_: &&'static str, _: &i32, _: &&'static str| true);

    let _: () = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericStructVariants::A { a: "test", b: 42, c: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericStructVariants::B { b: "test", c: 42, a: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
    let _: () = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| {});
    let _: bool = MultiGenericStructVariants::C { c: "test", a: 42, b: "test" }.for_any_mut(|_: &mut &'static str, _: &mut i32, _: &mut &'static str| true);
}
