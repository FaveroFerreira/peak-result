// Trait for peaking Results
pub trait Peak<T, E>
where
    E: std::fmt::Debug,
{
    fn peak_err(self, f: fn(&E) -> ()) -> Self;
    fn peak_ok(self, f: fn(&T) -> ()) -> Self;
}

impl<T, E> Peak<T, E> for std::result::Result<T, E>
where
    E: std::fmt::Debug,
{
    /// Runs a function on `std::result::Result` if it contains an Err variant.
    fn peak_err(self, f: fn(&E) -> ()) -> Self {
        if let Err(e) = &self {
            f(e);
        }

        self
    }

    /// Runs a function on `std::result::Result` if it contains an Ok variant.
    fn peak_ok(self, f: fn(&T) -> ()) -> Self {
        if let Ok(t) = &self {
            f(t);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Peak;

    #[test]
    #[should_panic]
    fn peak_ok_should_be_called_when_result_is_ok() {
        Ok::<(), ()>(()).peak_ok(|_| panic!("Nice")).unwrap();
    }

    #[test]
    #[allow(unused_must_use)]
    fn peak_ok_should_not_be_called_when_its_err() {
        Err::<(), ()>(()).peak_ok(|_| panic!("peak_ok should not be called"));
    }

    #[test]
    fn peak_ok_should_use_result_value() {
        Ok::<&str, ()>("OK")
            .peak_ok(|e| assert_eq!(e, &"OK"))
            .unwrap();
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn peak_err_is_called_when_result_is_err() {
        Err::<(), ()>(()).peak_err(|_| panic!("Nice"));
    }

    #[test]
    fn peak_err_should_not_be_called_on_ok_result() {
        Ok::<(), ()>(())
            .peak_err(|_| panic!("peak_err should not be called"))
            .unwrap();
    }

    #[test]
    #[allow(unused_must_use)]
    fn peak_err_should_use_result_value() {
        Err::<(), &str>("error").peak_err(|e| assert_eq!(e, &"error"));
    }
}
