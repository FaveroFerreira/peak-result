pub trait Peak<T, E>
where
    E: std::fmt::Debug,
{
    fn if_err(self, f: fn(&E) -> ()) -> Self;
    fn if_ok(self, f: fn(&T) -> ()) -> Self;
}

impl<T, E> Peak<T, E> for std::result::Result<T, E>
where
    E: std::fmt::Debug,
{
    fn if_err(self, f: fn(&E) -> ()) -> Self {
        if let Err(e) = &self {
            f(e);
        }

        self
    }

    fn if_ok(self, f: fn(&T) -> ()) -> Self {
        if let Ok(t) = &self {
            f(t);
        }

        self
    }
}
