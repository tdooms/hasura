use crate::attributes::Fields;
use crate::{Fields, Object};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub struct Braced<T: Display> {
    inner: T,
}

impl<T: Display> Display for Braced<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {} }}", self.inner)
    }
}

#[derive(Default)]
pub struct Builder<'a, T> {
    name: Option<String>,

    params: Vec<(&'a str, &'a dyn Display)>,
    returning: &'a Fields<'a, T>,

    affected: bool,
    explicit: bool,
}

impl<'a, T> Builder<'a, T> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn param(mut self, key: &'a str, value: &'a dyn Display) -> Self {
        self.params.push((key, value));
        self
    }

    pub fn maybe(mut self, key: &'a str, value: Option<&'a dyn Display>) -> Self {
        value.map(|value| self.param(key, value));
        self
    }
    pub fn returning(mut self, returning: &'a Fields<'a, T>) -> Self {
        self.returning = returning;
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
        write!(f, self.name)?;

        if !self.params.is_empty() {
            let fmt = |(k, v), f| f(&format_args!("{}: {}", k, v));
            write!(f, "({})", self.params.iter().format_with(", ", fmt))?;
        }

        match (self.affected, self.explicit) {
            (false, false) => write!(f, "{{ {} }}", self.returning),
            (false, true) => write!(f, "{{ returning {{ {} }}", self.returning),
            (true, false) => write!(f, "{{ {} affected_rows }}", self.returning),
            (true, true) => write!(f, "{{ returning {{ {} }} affected_rows }}", self.returning),
        }
    }
}
