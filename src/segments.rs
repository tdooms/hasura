use crate::error::{Error, Result};
use crate::traits::{Mutation, Queryable};
use crate::{Fetcher, Hasura};
use serde::de::DeserializeOwned;
use serde_json::Value;

fn decode<O: DeserializeOwned>(value: &Value, operation: &str, returning: bool) -> Result<O> {
    let mut entry = value.get(operation).ok_or(Error::Empty)?;

    if let (true, Some(new)) = (returning, entry.get("returning")) {
        entry = new
    }

    Ok(serde_json::from_value(entry.clone())?)
}

fn dec_query<P: Hasura, T: Queryable<P>>(val: &Value) -> Result<T::Out> {
    decode(val, &T::name(), false)
}

fn dec_mut<P: Hasura, T: Mutation<P>>(val: &Value) -> Result<T::Out> {
    decode(val, &T::name(), true)
}

pub fn query1<P1: Hasura, T1: Queryable<P1>>(t1: &T1) -> Fetcher<T1::Out> {
    let extract = move |val: Value| dec_query::<_, T1>(&val);
    Fetcher::new(format!("query {{ {t1} }}"), extract)
}

pub fn query2<P1: Hasura, P2: Hasura, T1: Queryable<P1>, T2: Queryable<P2>>(
    t1: &T1,
    t2: &T2,
) -> Fetcher<(T1::Out, T2::Out)> {
    let extract = |val| Ok((dec_query::<_, T1>(&val)?, dec_query::<_, T2>(&val)?));
    Fetcher::new(format!("query {{ {t1} {t2} }}"), extract)
}

pub fn query3<
    P1: Hasura,
    P2: Hasura,
    P3: Hasura,
    T1: Queryable<P1>,
    T2: Queryable<P2>,
    T3: Queryable<P3>,
>(
    t1: &T1,
    t2: &T2,
    t3: &T3,
) -> Fetcher<(T1::Out, T2::Out, T3::Out)> {
    let extract = |val| {
        Ok((
            dec_query::<_, T1>(&val)?,
            dec_query::<_, T2>(&val)?,
            dec_query::<_, T3>(&val)?,
        ))
    };
    Fetcher::new(format!("query {{ {t1} {t2} {t3} }}"), extract)
}

pub fn mutation1<P1: Hasura, T1: Mutation<P1>>(t1: &T1) -> Fetcher<T1::Out> {
    let extract = move |val: Value| dec_mut::<_, T1>(&val);
    Fetcher::new(format!("mutation {{ {t1} }}"), extract)
}

pub fn mutation2<P1: Hasura, P2: Hasura, T1: Mutation<P1>, T2: Mutation<P2>>(
    t1: &T1,
    t2: &T2,
) -> Fetcher<(T1::Out, T2::Out)> {
    let extract = |val| Ok((dec_mut::<_, T1>(&val)?, dec_mut::<_, T2>(&val)?));
    Fetcher::new(format!("mutation {{ {t1} {t2} }}"), extract)
}

pub fn mutation3<
    P1: Hasura,
    P2: Hasura,
    P3: Hasura,
    T1: Mutation<P1>,
    T2: Mutation<P2>,
    T3: Mutation<P3>,
>(
    t1: &T1,
    t2: &T2,
    t3: &T3,
) -> Fetcher<(T1::Out, T2::Out, T3::Out)> {
    let extract = |val| {
        Ok((
            dec_mut::<_, T1>(&val)?,
            dec_mut::<_, T2>(&val)?,
            dec_mut::<_, T3>(&val)?,
        ))
    };
    Fetcher::new(format!("mutation {{ {t1} {t2} {t3} }}"), extract)
}
