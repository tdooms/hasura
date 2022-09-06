use crate::{Fields, Hasura};
use std::fmt::{Display, Formatter, Result};
use itertools::Itertools;
use crate::serializer::to_string;


pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub struct Braced<'a, T: Display> (pub &'a T);
pub struct Serialized<'a, T: serde::Serialize>(pub &'a T);

pub struct Separated<'a, T: Display>(pub &'a [T]);
pub struct Separalized<'a, T: serde::Serialize>(pub &'a [T]);

impl<'a, T: Display> Display for Braced<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ {} }}", self.0)
    }
}

impl<'a, T: serde::Serialize> Display for Serialized<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", to_string(self.0, true).unwrap()) }
}


impl<'a, T: Display> Display for Separated<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.0.iter().join(", ")) }
}

impl <'a, T: Display> IsEmpty for Separated<'a, T> {
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl<'a, T: serde::Serialize> Display for Separalized<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "[{}]", self.0.iter().map(Serialized).join(", ")) }
}

impl <'a, T: serde::Serialize> IsEmpty for Separalized<'a, T> {
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

pub struct Builder<'a, T: Hasura> {
    name: String,
    returning: &'a Fields<'a, T>,

    params: Vec<(&'a str, &'a dyn Display)>,

    affected: bool,
    explicit: bool,
}

impl<'a, T: Hasura> Builder<'a, T> {
    pub fn new(name: String, returning: &'a Fields<'a, T>) -> Self {
        Self {
            name,
            returning,
            params: vec![],
            affected: false,
            explicit: false,
        }
    }

    pub fn param<D: Display>(mut self, key: &'a str, value: &'a D) -> Self {
        self.params.push((key, value));
        self
    }

    pub fn vector<D: Display + IsEmpty>(mut self, key: &'a str, value: &'a D) -> Self {
        if !value.is_empty() {
            self.params.push((key, value))
        }
        self
    }

    pub fn maybe<D: Display>(self, key: &'a str, value: Option<&'a D>) -> Self {
        match value {
            Some(value) => self.param(key, value),
            None => self,
        }
    }

    pub fn affected(mut self, affected: bool) -> Self {
        self.affected = affected;
        self
    }

    pub fn explicit(mut self, explicit: bool) -> Self {
        self.explicit = explicit;
        self
    }

    pub fn write(self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)?;

        if !self.params.is_empty() {
            let params = self.params.iter().map(|(k, v)| format!("{}: {}", k, v)).join(", ");
            write!(f, "({params})")?;
        }

        match (self.affected, self.explicit) {
            (false, false) => write!(f, " {{ {} }}", self.returning),
            (false, true) => write!(f, " {{ returning {{ {} }}", self.returning),
            (true, false) => write!(f, " {{ {} affected_rows }}", self.returning),
            (true, true) => write!(f, " {{ returning {{ {} }} affected_rows }}", self.returning),
        }
    }
}
