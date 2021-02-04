#[macro_export]
macro_rules! anyhowize {
    ($e:expr) => {
        Error::msg($e.to_string())
    };
}
