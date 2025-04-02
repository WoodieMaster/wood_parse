#[macro_export]
macro_rules! tee {
    ($e:expr) => {{
        let e = $e;
        dbg!(&e);
        e
    }};
}

#[derive(Debug)]
pub struct END();

impl std::fmt::Display for END {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EOF")
    }
}

impl std::error::Error for END {}
