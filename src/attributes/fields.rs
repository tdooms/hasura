#[derive(Clone)]
pub struct Field<'a, T: Object + ?Sized> {
    pub name: &'a str,
    pub inner: Vec<String>,
    pub phantom: PhantomData<T>,
}

impl<'a, T: Object> PartialEq for Field<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<'a, T: Object> Field<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Field {
            name,
            inner: vec![],
            phantom: PhantomData::default(),
        }
    }
    pub fn recursive<S: Object>(name: &'a str, keys: Fields<'a, S>) -> Self {
        let inner = keys.inner.into_iter().map(|k| k.to_string()).collect();
        Field {
            name,
            inner,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: Object> Display for Field<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.is_empty() {
            true => f.write_str(self.name),
            false => write!(f, "{} {{ {} }}", self.name, self.inner.iter().join(" ")),
        }
    }
}

pub struct Fields<'a, T: Object + Sized> {
    pub inner: Vec<Field<'a, T>>,
}

impl<'a, T: Object> Display for Fields<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.iter().join(" "))
    }
}

impl<'a, T: Object> Default for Fields<'a, T> {
    fn default() -> Self {
        T::all()
    }
}
