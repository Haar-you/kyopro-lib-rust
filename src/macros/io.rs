#[macro_export]
macro_rules! get {
    ( $in:ident, [$a:tt; $num:expr] ) => {
        {
            let n = $num;
            (0 .. n).map(|_| get!($in, $a)).collect::<Vec<_>>()
        }
    };

    ( $in:ident, ($($type:ty),*) ) => {
        ($(get!($in, $type)),*)
    };

    ( $in:ident, $type:ty ) => {
        {
            let token = $in.next().unwrap();

            token.parse::<$type>().expect(
                format!("cannot convert \"{}\" into {}", token, stringify!($type)).as_str()
            )
         }
    };
}

#[macro_export]
macro_rules! input {
    ( @inner $in:ident, mut $name:ident : $type:tt ) => {
        let mut $name = get!($in, $type);
    };

    ( @inner $in:ident, $name:ident : $type:tt ) => {
        let $name = get!($in, $type);
    };

    ( $in:ident, $($($names:ident)* : $type:tt),* ) => {
        $(
            input!(@inner $in, $($names)* : $type);
        )*
    }
}

#[macro_export]
macro_rules! io {
    ( $in:ident, $out:ident ) => {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        let mut $in = s.split_ascii_whitespace();

        let $out = std::io::stdout();
        let mut $out = std::io::BufWriter::new($out.lock());
    };
}
