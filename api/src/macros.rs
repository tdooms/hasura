use crate::error::{Error, Result};
use crate::request::request;
use crate::Object;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub trait Queryable<P: Object>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}

pub trait Mutation<P: Object>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}

pub struct Fetch<O> {
    pub body: String,
    pub extract: Box<dyn FnOnce(Value) -> Result<O>>,

    pub token: Option<String>,
    pub admin: Option<String>,
}

impl<O> Debug for Fetch<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.body)
    }
}

impl<O> Fetch<O> {
    pub fn new<T: Display, Fn: FnOnce(Value) -> Result<O> + 'static>(body: T, extract: Fn) -> Self {
        let body = body.to_string().replace('"', "\\\"");
        Self {
            body: format!("{{\"query\": \"{}\"}}", body),
            extract: Box::new(extract),
            token: None,
            admin: None,
        }
    }

    pub fn token(mut self, token: Option<String>) -> Self {
        self.token = token;
        self
    }

    pub fn admin(mut self, admin: Option<String>) -> Self {
        self.admin = admin;
        self
    }

    pub async fn send(self, url: &str) -> Result<O> {
        let val = request(url, self.body, self.token, self.admin).await?;
        (self.extract)(val)
    }
}

fn decode<O: DeserializeOwned>(value: &Value, op: &str, returning: bool) -> Result<O> {
    let mut entry = value.get(op).ok_or(Error::Empty)?;

    if let (true, Some(new)) = (returning, entry.get("returning")) {
        entry = new
    }

    Ok(serde_json::from_value(entry.clone())?)
}

fn dec_query<P: Object, T: Queryable<P>>(val: &Value) -> Result<T::Out> {
    decode(val, P::name(), false)
}

fn dec_mut<P: Object, T: Mutation<P>>(val: &Value) -> Result<T::Out> {
    decode(val, &T::name(), true)
}

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} }}", _0)]
pub struct Query1<'a, P1: Object, T1: Queryable<P1>>(pub &'a T1, pub PhantomData<P1>);

impl<'a, P1: Object, T1: Queryable<P1>> Query1<'a, P1, T1> {
    pub fn build(self) -> Fetch<T1::Out> {
        Fetch::new(self, |val| dec_query::<_, T1>(&val))
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} {} }}", _0, _1)]
pub struct Query2<'a, P1: Object, P2: Object, T1: Queryable<P1>, T2: Queryable<P2>>(
    pub &'a T1,
    pub &'a T2,
    pub PhantomData<(P1, P2)>,
);

impl<'a, P1: Object, P2: Object, T1: Queryable<P1>, T2: Queryable<P2>> Query2<'a, P1, P2, T1, T2> {
    pub fn build(self) -> Fetch<(T1::Out, T2::Out)> {
        let func = |val| Ok((dec_query::<_, T1>(&val)?, dec_query::<_, T2>(&val)?));
        Fetch::new(self, func)
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} }}", _0)]
pub struct Mutation1<'a, P1: Object, T1: Mutation<P1>>(pub &'a T1, pub PhantomData<P1>);

impl<'a, P1: Object, T1: Mutation<P1>> Mutation1<'a, P1, T1> {
    pub fn build(self) -> Fetch<T1::Out> {
        Fetch::new(self, |val| dec_mut::<_, T1>(&val))
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} {} }}", _0, _1)]
pub struct Mutation2<'a, P1: Object, P2: Object, T1: Mutation<P1>, T2: Mutation<P2>>(
    pub &'a T1,
    pub &'a T2,
    pub PhantomData<(P1, P2)>,
);

impl<'a, P1: Object, P2: Object, T1: Mutation<P1>, T2: Mutation<P2>> Mutation2<'a, P1, P2, T1, T2> {
    pub fn build(self) -> Fetch<(T1::Out, T2::Out)> {
        let func = |val| Ok((dec_mut::<_, T1>(&val)?, dec_mut::<_, T2>(&val)?));
        Fetch::new(self, func)
    }
}

#[macro_export]
macro_rules! query {
    ($a:ident) => {
        Query1(&$a, std::marker::PhantomData::default()).build()
    };
    ($a:ident, $b:ident) => {
        Query2(&$a, &$b, std::marker::PhantomData::default()).build()
    };
}

#[macro_export]
macro_rules! mutation {
    ($a:ident) => {
        Mutation1(&$a, std::marker::PhantomData::default()).build()
    };
    ($a:ident, $b:ident) => {
        Mutation2(&$a, &$b, std::marker::PhantomData::default()).build()
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
