//! `get!`, `input!`

/// [`crate::io::fastio::FastIO`]を第一引数に、型を第二引数にとって、入力のパースを行う。
#[macro_export]
macro_rules! get {
    ( $in:ident, [$a:tt $(as $to:ty)*; $num:expr] ) => {
        {
            let n = $num;
            (0 .. n).map(|_| get!($in, $a $(as $to)*)).collect::<Vec<_>>()
        }
    };

    ( $in:ident, ($($type:tt $(as $to:ty)*),*) ) => {
        ($(get!($in, $type $(as $to)*)),*)
    };

    ( $in:ident, i8 ) => { $in.read_i64() as i8 };
    ( $in:ident, i16 ) => { $in.read_i64() as i16 };
    ( $in:ident, i32 ) => { $in.read_i64() as i32 };
    ( $in:ident, i64 ) => { $in.read_i64() };
    ( $in:ident, isize ) => { $in.read_i64() as isize };

    ( $in:ident, u8 ) => { $in.read_u64() as u8 };
    ( $in:ident, u16 ) => { $in.read_u64() as u16 };
    ( $in:ident, u32 ) => { $in.read_u64() as u32 };
    ( $in:ident, u64 ) => { $in.read_u64() };
    ( $in:ident, usize ) => { $in.read_u64() as usize };

    ( $in:ident, [char] ) => { $in.read_chars() };

    ( $in:ident, $from:tt as $to:ty ) => { <$to>::from(get!($in, $from)) };
}

/// [`crate::io::fastio::FastIO`]を第一引数にとり、第二引数以降に`変数名: 型`を連ねる。
#[macro_export]
macro_rules! input {
    ( @inner $in:ident, mut $name:ident : $type:tt ) => {
        let mut $name = get!($in, $type);
    };

    ( @inner $in:ident, mut $name:ident : $type:tt as $to:ty ) => {
        let mut $name = get!($in, $type as $to);
    };

    ( @inner $in:ident, $name:ident : $type:tt ) => {
        let $name = get!($in, $type);
    };

    ( @inner $in:ident, $name:ident : $type:tt as $to:ty ) => {
        let $name = get!($in, $type as $to);
    };

    ( $in:ident >> $($($names:ident)* : $type:tt $(as $to:ty)*),* ) => {
        $(input!(@inner $in, $($names)* : $type $(as $to)*);)*
    }
}
