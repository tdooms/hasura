use crate::error::Error;
use crate::request::request;
use crate::Object;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Display;
use std::marker::PhantomData;

pub trait Queryable<P: Object>: Display {
    type Out: DeserializeOwned;
}

pub trait Mutation<P: Object>: Display {
    type Out: DeserializeOwned;
}

pub struct Fetch<O, T: FnOnce(Value) -> Result<O, Error>> {
    pub extract: T,
    pub token: Option<String>,
}

impl<O, T: FnOnce(Value) -> Result<O, Error>> Fetch<O, T> {
    pub fn new(extract: T) -> Self {
        Self {
            extract,
            token: None,
        }
    }
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());
        self
    }

    pub async fn send(&self, url: &str) -> Result<O, Error> {
        let val = request(url, self.to_string(), self.token.map(|x| x.as_str())).await?;
        self.extract(val)
    }
}

fn enc(obj: &impl ToString) -> String {
    obj.to_string().replace('"', "\\\"")
}

pub fn decode<O: DeserializeOwned>(value: &Value, op: &str, ret: bool) -> Result<O, Error> {
    let mut entry = value.get(op).ok_or(Error::Empty)?;

    if ret {
        entry = entry.get("returning").ok_or(Error::Empty)?;
    }

    Ok(serde_json::from_value(entry.clone())?)
}

pub fn dec_query<P: Object, T: Queryable<P>>(val: &Value) -> Result<T::Out, Error> {
    decode(val, P::name(), false)
}

pub fn dec_mut<P: Object, T: Mutation<P>>(val: &Value) -> Result<T::Out, Error> {
    decode(val, P::name(), false)
}

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} }}", "enc(_0)")]
pub struct Query1<'a, P1: Object, T1: Queryable<P1>>(pub &'a T1, pub PhantomData<P1>);

impl<'a, P1: Object, T1: Queryable<P1>> Query1<'a, P1, T1> {
    pub async fn build(self) -> Fetch<P1, _> {
        Fetch::new(|val| dec_query::<_, T1>(&val))
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "query {{ {} {} }}", "enc(_0)", "enc(_1)")]
pub struct Query2<'a, P1: Object, P2: Object, T1: Queryable<P1>, T2: Queryable<P2>>(
    pub &'a T1,
    pub &'a T2,
    PhantomData<(P1, P2)>,
);

impl<'a, P1: Object, P2: Object, T1: Queryable<P1>, T2: Queryable<P2>> Query2<'a, P1, P2, T1, T2> {
    pub async fn send(self, url: &str, token: Option<&str>) -> Result<(T1::Out, T2::Out), Error> {
        let val = request(url, self.to_string(), token).await?;
        Ok((dec_query::<_, T1>(&val)?, dec_query::<_, T2>(&val)?))
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} }}", "enc(_0)")]
pub struct Mutation1<'a, P1: Object, T1: Mutation<P1>>(pub &'a T1, PhantomData<P1>);

impl<'a, P1: Object, T1: Mutation<P1>> Mutation1<'a, P1, T1> {
    pub async fn send(self, url: &str, token: Option<&str>) -> Result<T1::Out, Error> {
        let val = request(url, self.to_string(), token).await?;
        Ok(dec_mut::<_, T1>(&val)?)
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "mutation {{ {} {} }}", "enc(_0)", "enc(_1)")]
pub struct Mutation2<'a, P1: Object, P2: Object, T1: Mutation<P1>, T2: Mutation<P2>>(
    pub &'a T1,
    pub &'a T2,
    pub PhantomData<(P1, P2)>,
);

impl<'a, P1: Object, P2: Object, T1: Mutation<P1>, T2: Mutation<P2>> Mutation2<'a, P1, P2, T1, T2> {
    pub async fn send(self, url: &str, token: Option<&str>) -> Result<(T1::Out, T2::Out), Error> {
        let val = request(url, self.to_string(), token).await?;
        Ok((dec_mut::<_, T1>(&val)?, dec_mut::<_, T2>(&val)?))
    }
}

#[macro_export]
macro_rules! query {
    ($a:ident) => {
        Query1(&$a, std::marker::PhantomData::default())
    };
    ($a:ident, $b:ident) => {
        Query2(&$a, &$b, std::marker::PhantomData::default())
    };
}

#[macro_export]
macro_rules! mutation {
    ($a:ident) => {
        Mutation1(&$a, std::marker::PhantomData::default())
    };
    ($a:ident, $b:ident) => {
        Mutation2(&$a, &$b, std::marker::PhantomData::default())
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
