//! `get_time!`

/// ブロック内のコードの実行時間を返す。
#[macro_export]
macro_rules! get_time {
    ($b:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        $b;
        start.elapsed()
    }};
}
