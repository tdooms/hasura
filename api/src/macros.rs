#[macro_export]
macro_rules! query {
    ($a:ident) => {
        hasura::Query1(&$a, std::marker::PhantomData::default()).build()
    };
    ($a:ident, $b:ident) => {
        hasura::Query2(&$a, &$b, std::marker::PhantomData::default()).build()
    };
}

#[macro_export]
macro_rules! mutation {
    ($a:ident) => {
        hasura::Mutation1(&$a, std::marker::PhantomData::default()).build()
    };
    ($a:ident, $b:ident) => {
        hasura::Mutation2(&$a, &$b, std::marker::PhantomData::default()).build()
    };
}

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
