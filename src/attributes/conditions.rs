use crate::{Field, Hasura};
use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub trait Condition: Display {}

#[macro_export]
macro_rules! impl_cond {
    ($t:ident, $n:tt ;$($i:ty),*) => {
        pub struct $t<T: serde::Serialize>(pub T);

        impl<T: serde::Serialize> std::fmt::Display for $t<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($n:{}), crate::to_string(&self.0, true).unwrap())
            }
        }

        $(impl Condition for $t<$i> {})*
    };
}

impl_cond!(Eq,_eq;bool,u64,i64,f64,String,&'_ str);
impl_cond!(Gt,_gt;u64,i64,f64,String,&'_ str);
impl_cond!(Gte,_gte;u64,i64,f64,String,&'_ str);
impl_cond!(Neq,_neq;u64,i64,f64,String,&'_ str);
impl_cond!(Lt,_lt;u64,i64,f64,String,&'_ str);
impl_cond!(Ilike,_ilike;String,&'_ str);
impl_cond!(Like,_like;String,&'_ str);

pub enum Conditions<'a, T: Hasura> {
    And(Box<Conditions<'a, T>>, Box<Conditions<'a, T>>),
    Or(Box<Conditions<'a, T>>, Box<Conditions<'a, T>>),
    Not(Box<Conditions<'a, T>>),
    Field(Field<'a, T>, Vec<Box<dyn Condition>>),
    None
}

impl<'a, T: Hasura> Conditions<'a, T> {
    pub fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    pub fn not(self) -> Self {
        Self::Not(Box::new(self))
    }

    pub fn single(field: Field<'a, T>, condition: impl Condition + 'static) -> Self {
        Self::Field(field, vec![Box::new(condition)])
    }
    pub fn many(field: Field<'a, T>, conditions: Vec<Box<dyn Condition>>) -> Self {
        Self::Field(field, conditions)
    }
}

impl<'a, T: Hasura> Display for Conditions<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And(l, r) => write!(f, "_and{{ {}, {} }}", l, r),
            Self::Or(l, r) => write!(f, "_or{{ {}, {} }}", l, r),
            Self::Not(c) => write!(f, "_not{{ {} }}", c),
            Self::Field(field, cond) => write!(f, "{}: {{ {} }}", field, cond.iter().join(", ")),
            Self::None => write!(f, "")
        }
    }
}
