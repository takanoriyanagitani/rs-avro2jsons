#[macro_export]
macro_rules! bind {
    ($io: expr, $f: expr) => {
        move || {
            let t = $io()?;
            $f(t)()
        }
    };
}
