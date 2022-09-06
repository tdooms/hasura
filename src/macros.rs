#[macro_export]
macro_rules! query {
    ($a:ident) => {
        hasura::query1(&$a)
    };
    ($a:ident, $b:ident) => {
        hasura::query2(&$a, &$b)
    };
        ($a:ident, $b:ident, $c:ident) => {
        hasura::query3(&$a, &$b, &$c)
    };
}

#[macro_export]
macro_rules! mutation {
    ($a:ident) => {
        hasura::mutation1(&$a)
    };
    ($a:ident, $b:ident) => {
        hasura::mutation2(&$a, &$b)
    };
    ($a:ident, $b:ident, $c:ident) => {
        hasura::mutation3(&$a, &$b, &$c)
    };
}

// #[macro_export]
// macro_rules! condition {
//     ($($a:stmt $b:stmt $c:stmt),+) => {
//         let map: std::collections::HashMap<Field<'a, T>, (String, String)> = [$($a $b $c),+].iter().map(|(a, b, c)| (*a, (b.to_string(), c.to_string()))).collect();
//         hasura::Conditions::Field()
//     }
// }

// let condition1 = Condition {
//     op: "_eq",
//     value: "true",
// };
// let condition2 = Condition {
//     op: "_ilike",
//     value: "cit",
// };
// let conditions = Conditions::Field(Quiz::title(), vec![condition1, condition2]);

// conditions!(Quiz::title() _ilike "cit", Quiz::public() == false);
// TODO: make this macro
