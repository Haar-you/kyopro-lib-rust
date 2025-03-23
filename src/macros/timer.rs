//! `timer!`

/// block内のコードを実行して、実行時間を計測表示し、実行結果を返す。
#[macro_export]
macro_rules! timer {
    ($b:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        let ret = $b;
        eprintln!("{:?}", start.elapsed());
        ret
    }};
}
