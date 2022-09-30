use crate::{Fields, Hasura};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

pub trait Maybe<T: Display> {
    fn maybe(&self) -> Option<&T>;
}

pub struct Braced<'a, T: Display>(pub &'a T);
pub struct Serialized<'a, T: serde::Serialize>(pub &'a T);
pub struct Flattened<'a, T: serde::Serialize>(pub &'a T);

pub struct Separated<'a, T: Display>(pub &'a [T]);
pub struct Separalized<'a, T: serde::Serialize>(pub &'a [T]);

impl<'a, T: Display> Display for Braced<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ {} }}", self.0)
    }
}

impl<'a, T: serde::Serialize> Display for Serialized<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", crate::to_string(self.0, true).unwrap())
    }
}

impl<'a, T: serde::Serialize> Display for Flattened<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", crate::to_string(self.0, false).unwrap())
    }
}

impl<'a, T: Display> Display for Separated<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.iter().join(", "))
    }
}

impl<'a, T: Display> Maybe<Self> for Separated<'a, T> {
    fn maybe(&self) -> Option<&Self> {
        (!self.0.is_empty()).then(|| self)
    }
}

impl<'a, T: serde::Serialize> Display for Separalized<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}]", self.0.iter().map(Serialized).join(", "))
    }
}

impl<'a, T: serde::Serialize> Maybe<Self> for Separalized<'a, T> {
    fn maybe(&self) -> Option<&Self> {
        (!self.0.is_empty()).then(|| self)
    }
}

impl<T: Display> Maybe<T> for Option<T> {
    fn maybe(&self) -> Option<&T> {
        self.as_ref()
    }
}

pub struct Builder<'a, T: Hasura> {
    name: String,
    returning: &'a Fields<'a, T>,

    params: Vec<(&'a str, &'a dyn Display)>,
    pk: Option<&'a dyn Display>,

    affected: bool,
    explicit: bool,
}

impl<'a, T: Hasura> Builder<'a, T> {
    pub fn new(name: String, returning: &'a Fields<'a, T>) -> Self {
        Self {
            name,
            returning,
            params: vec![],
            pk: None,
            affected: false,
            explicit: false,
        }
    }

    pub fn param<D: Display>(mut self, key: &'a str, value: &'a D) -> Self {
        self.params.push((key, value));
        self
    }
    pub fn maybe<I: Display + 'a, D: Maybe<I>>(self, key: &'a str, value: &'a D) -> Self {
        match value.maybe() {
            Some(value) => self.param(key, value),
            None => self,
        }
    }
    pub fn affected(mut self, affected: bool) -> Self {
        self.affected = affected;
        self
    }
    pub fn pk(mut self, pk: &'a impl Display) -> Self {
        self.pk = Some(pk);
        self
    }
    pub fn explicit(mut self, explicit: bool) -> Self {
        self.explicit = explicit;
        self
    }

    pub fn write(self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)?;

        let params = self
            .params
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .join(", ");

        match (params, self.pk) {
            (params, Some(pk)) => write!(f, "({pk}, {params})")?,
            (params, None) if params.is_empty() => (),
            (params, None) => write!(f, "({params})")?,
        };

        match (self.affected, self.explicit) {
            (false, false) => write!(f, " {{ {} }}", self.returning),
            (false, true) => write!(f, " {{ returning {{ {} }} }}", self.returning),
            (true, false) => write!(f, " {{ {} affected_rows }}", self.returning),
            (true, true) => write!(f, " {{ returning {{ {} }} affected_rows }}", self.returning),
        }
    }
}
