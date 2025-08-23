//! `get!`, `input!`

/// [`crate::io::fastio::FastIO`]を第一引数に、型を第二引数にとって、入力のパースを行う。
#[macro_export]
macro_rules! get {
    ( $in:expr; [$a:tt $(as $to:ty)*; $num:expr] ) => {
        (0..$num).map(|_| get!($in; $a $(as $to)*)).collect::<Vec<_>>()
    };

    ( $in:expr; ($($type:tt $(as $to:ty)*),*) ) => {
        ($(get!($in; $type $(as $to)*)),*)
    };

    ( $in:expr; i8 ) => { $in.read_i64() as i8 };
    ( $in:expr; i16 ) => { $in.read_i64() as i16 };
    ( $in:expr; i32 ) => { $in.read_i64() as i32 };
    ( $in:expr; i64 ) => { $in.read_i64() };
    ( $in:expr; isize ) => { $in.read_i64() as isize };

    ( $in:expr; u8 ) => { $in.read_u64() as u8 };
    ( $in:expr; u16 ) => { $in.read_u64() as u16 };
    ( $in:expr; u32 ) => { $in.read_u64() as u32 };
    ( $in:expr; u64 ) => { $in.read_u64() };
    ( $in:expr; usize ) => { $in.read_u64() as usize };

    ( $in:expr; char) => { $in.read_char() };

    ( $in:expr; [u8]) => { $in.read_bytes() };
    ( $in:expr; [char] ) => { $in.read_chars() };
    ( $in:expr; String ) => { $in.read_string() };

    ( $in:expr; $from:tt as $to:ty ) => { <$to>::from(get!($in; $from)) };
}

/// [`crate::io::fastio::FastIO`]を第一引数にとり、第二引数以降に`変数名: 型`を連ねる。
#[macro_export]
macro_rules! input {
    ( @inner $in:expr, $name:pat, $type:tt ) => {
        let $name = get!($in; $type);
    };
    ( @inner $in:expr, $name:pat, $type:tt as $to:ty ) => {
        let $name = get!($in; $type as $to);
    };

    ( $in:expr; $( $names:pat = $type:tt $(as $to:ty)? ),* ) => {
        $(input!(@inner $in, $names, $type $(as $to)?);)*
    }
}

/// [`crate::io::fastio::FastIO`]を第一引数にとり、第二引数以降を空白区切りで出力する。
#[macro_export]
macro_rules! output {
    ( @one $io:expr, $a:expr ) => {
        $io.write($a);
    };

    ( $io:expr; $a:expr, $($rest:expr),* ) => {
        output!(@one $io, $a);
        $(
            $io.write(" ");
            $io.write($rest);
        )*
        $io.writeln("");
    };
}
