pub trait Queryable {}
pub trait Mutation {}

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} }}", _0)]
pub struct Query1<T1: Queryable>(pub T1);

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} {} }}", _0, _1)]
pub struct Query2<T1: Queryable, T2: Queryable>(pub T1, pub T2);

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} }}", _0)]
pub struct Mutation1<T1: Mutation>(pub T1);

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} {} }}", _0, _1)]
pub struct Mutation2<T1: Mutation, T2: Mutation>(pub T1, pub T2);

#[macro_export]
macro_rules! query {
    ($a:ident) => {
        format!("{{ \"query\": \"{}\" }}", Query1($a));
    };
    ($a:ident, $b:ident) => {
        format!("{{ \"query\": \"{}\" }}", Query2($a, $b));
    };
}

#[macro_export]
macro_rules! mutation {
    ($a:ident) => {
        format!("{{ \"mutation\": \"{}\" }}", Mutation1($a))
    };
    ($a:ident, $b:ident) => {
        format!("{{ \"mutation\": \"{}\" }}", Mutation2($a, $b))
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
