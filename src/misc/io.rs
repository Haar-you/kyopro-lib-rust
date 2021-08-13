use std::io::{ Read, Write };


#[allow(unused_macros)]
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

#[allow(unused_macros)]
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


pub trait JoinStr {
    fn join_str(&self, _: &str) -> String;
}

impl<T: std::fmt::Display> JoinStr for Vec<T> {
    fn join_str(&self, s: &str) -> String {
        (&self.iter().map(|x| format!("{}", x)).collect::<Vec<_>>()).join(s).to_string()
    }
}

macro_rules! impl_join_str_tuple {
    ( $head:ident ) => {
        impl<$head: std::fmt::Display> JoinStr for ($head,) {
            #[allow(non_snake_case, redundant_semicolons)]
            fn join_str(&self, _: &str) -> String {
                let (ref $head,) = *self;
                format!("{}", $head)
            }
        }
    };
    ( $head:ident, $($tail:ident),+ ) => {
        impl<$head: std::fmt::Display, $($tail: std::fmt::Display),+> JoinStr for ($head, $($tail),*) {
            #[allow(non_snake_case, redundant_semicolons)]
            fn join_str(&self, s: &str) -> String {
                let mut ret = vec![];
                let (ref $head, $(ref $tail,)+) = *self;
                ret.push(format!("{}", $head));
                $(
                    ret.push(format!("{}", $tail));
                )+;
                ret.join_str(s)
            }
        }

        impl_join_str_tuple!($($tail),+);
    }
}

impl_join_str_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

#[allow(unused_macros)]
macro_rules! io {
    ( $in:ident, $out:ident ) => {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        let mut $in = s.split_ascii_whitespace();

        let $out = std::io::stdout();
        let mut $out = std::io::BufWriter::new($out.lock());
    }
}
