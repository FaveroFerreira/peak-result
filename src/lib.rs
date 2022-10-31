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
    /// Runs a function on `std::result::Result` if it contains an Err variant.
    fn if_err(self, f: fn(&E) -> ()) -> Self {
        if let Err(e) = &self {
            f(e);
        }

        self
    }

    /// Runs a function on `std::result::Result` if it contains an Ok variant.
    fn if_ok(self, f: fn(&T) -> ()) -> Self {
        if let Ok(t) = &self {
            f(t);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Peak;
    // =========================== if_ok() tests ============================

    #[test]
    #[should_panic]
    fn test_if_ok_is_called() {
        Ok::<(), ()>(()).if_ok(|_| panic!("Nice")).unwrap();
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_if_ok_should_not_be_called() {
        Err::<(), ()>(()).if_ok(|_| panic!("if_ok should not be called"));
    }

    #[test]
    fn test_if_ok_uses_result_value() {
        Ok::<&str, ()>("OK")
            .if_ok(|e| assert_eq!(e, &"OK"))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_if_ok_uses_result_value_wrong_assert() {
        Ok::<&str, ()>("OK").if_ok(|e| assert_eq!(e, &"")).unwrap();
    }

    // =========================== if_err() tests ===========================

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_if_err_is_called() {
        Err::<(), ()>(()).if_err(|_| panic!("Nice"));
    }

    #[test]
    fn test_if_err_should_not_be_called() {
        Ok::<(), ()>(())
            .if_err(|_| panic!("if_err should not be called"))
            .unwrap();
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_if_err_uses_result_value() {
        Err::<(), &str>("error").if_err(|e| assert_eq!(e, &"error"));
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_if_err_uses_result_value_wrong_assert() {
        Err::<(), &str>("error").if_err(|e| assert_eq!(e, &""));
    }
}
