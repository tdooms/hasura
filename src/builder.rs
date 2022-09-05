use crate::{Fields, Hasura};
use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct Braced<'a, T: Display> (pub &'a T);
pub struct Separated<'a, T: Display>(pub &'a [T]);

impl<'a, T: Display> Display for Braced<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {} }}", self.0)
    }
}

impl<'a, T: Display> Display for Separated<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(" "))
    }
}

pub struct Builder<'a, T: Hasura> {
    name: Option<String>,

    params: Vec<(&'a str, &'a dyn Display)>,
    returning: Option<&'a Fields<'a, T>>,

    affected: bool,
    explicit: bool,
}

impl<T: Hasura> Default for Builder<'_, T> {
    fn default() -> Self {
        Builder {
            name: None,
            params: vec![],
            returning: None,
            affected: false,
            explicit: false,
        }
    }
}

impl<'a, T: Hasura> Builder<'a, T> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn param<D: Display>(mut self, key: &'a str, value: &'a D) -> Self {
        self.params.push((key, value));
        self
    }

    pub fn vector<D: Display>(mut self, key: &'a str, value: &'a Separated<'a, D>) -> Self {
        if !value.0.is_empty() {
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
    pub fn returning(mut self, returning: &'a Fields<'a, T>) -> Self {
        self.returning = Some(returning);
        self
    }

    pub fn affected(mut self, affected: bool) -> Self {
        self.affected = affected;
        self
    }

    pub fn explicit(mut self, explicit: bool) -> Self {
        self.explicit = explicit;
        self
    }

    pub fn build(self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.unwrap())?;

        if !self.params.is_empty() {
            let params = self.params.iter().map(|(k, v)| format!("{}: {}", k, v)).join(", ");
            write!(f, "({params})")?;
        }

        let returning = self.returning.unwrap();

        match (self.affected, self.explicit) {
            (false, false) => write!(f, " {{ {} }}", returning),
            (false, true) => write!(f, " {{ returning {{ {} }}", returning),
            (true, false) => write!(f, " {{ {} affected_rows }}", returning),
            (true, true) => write!(f, " {{ returning {{ {} }} affected_rows }}", returning),
        }
    }
}
