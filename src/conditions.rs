use crate::{Field, Object};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub trait Condition: Display {}

#[macro_export]
macro_rules! impl_cond {
    ($t:ident, $n:tt ;$($i:ty),*) => {
        pub struct $t<T: serde::Serialize>(pub T);

        impl<T: serde::Serialize> std::fmt::Display for $t<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($n:{}), crate::serializer::to_string(&self.0, true).unwrap())
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

pub enum Conditions<'a, T: Object> {
    And(Vec<Conditions<'a, T>>),
    Or(Vec<Conditions<'a, T>>),
    Not(Vec<Conditions<'a, T>>),
    Field(Field<'a, T>, Vec<Box<dyn Condition>>),
}

impl<'a, T: Object> Conditions<'a, T> {
    pub fn and(self, other: Self) -> Self {
        Self::And(vec![self, other])
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(vec![self, other])
    }

    pub fn not(self, other: Self) -> Self {
        Self::Not(vec![self, other])
    }

    pub fn single(field: Field<'a, T>, condition: impl Condition + 'static) -> Self {
        Self::Field(field, vec![Box::new(condition)])
    }
    pub fn many(field: Field<'a, T>, conditions: Vec<Box<dyn Condition>>) -> Self {
        Self::Field(field, conditions)
    }
}

impl<'a, T: Object> Display for Conditions<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And(v) => write!(f, "_and{{ {} }}", v.iter().format(", ")),
            Self::Or(v) => write!(f, "_or{{ {} }}", v.iter().format(", ")),
            Self::Not(v) => write!(f, "_not{{ {} }}", v.iter().format(", ")),
            Self::Field(field, cond) => write!(f, "{}: {{ {} }}", field, cond.iter().format(", ")),
        }
    }
}
