#[macro_export]
macro_rules! timer {
    ($b:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        $b;
        start.elapsed()
    }};
}
