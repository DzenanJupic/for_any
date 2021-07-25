// use for_any::ForAny;
//
// #[derive(ForAny)]
// #[for_any(generic = "std::fmt::Display")]
// enum MyResult<T, E> {
//     Ok { a: T, b: T },
//     Err(E, E),
// }
//
// impl<T, E> MyResult<T, E> {
//     fn for_any<RET, F: FnOnce(dyn std::any::Any, dyn std::any::Any) -> RET>(self, func: F) -> RET {
//         match self {
//             MyResult::Ok { a, b } => func(a, b),
//             MyResult::Err(a, b) => func(a, b),
//         }
//     }
// }
//
//
// #[test]
// fn t() {
//     let res = MyResult::<i32, String>::Ok{ a: 3, b: 4};
//     let ret = res.for_any_ref(|val, val2| println!("{}, {}", val, val2));
//     assert_eq!(ret, ());
//     // let mut res =  MyResult::<i32, &str>::Err("test");
//     // let ret = res.for_any_mut(|val| println!("{}", val));
//     // assert_eq!(ret, ());
// }
