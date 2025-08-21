pub mod main {
    use super::*;

    #[allow(unused_imports)]
    use haar_lib::{get, input, io::fastio::*, iter::join_str::*, output};

    #[allow(unused_imports)]
    use std::cell::{Cell, RefCell};
    #[allow(unused_imports)]
    use std::cmp::{max, min, Reverse};
    #[allow(unused_imports)]
    use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
    #[allow(unused_imports)]
    use std::io::Write;
    #[allow(unused_imports)]
    use std::mem::swap;
    #[allow(unused_imports)]
    use std::rc::Rc;

    pub struct Problem {
        io: FastIO,
    }

    impl Problem {
        pub fn init() -> Self {
            Self { io: FastIO::new() }
        }

        pub fn main(&mut self) {}
    }
}

fn main() {
    //    const STACK_SIZE: usize = 1024 * 1024 * 1024;
    std::thread::Builder::new()
        //        .stack_size(STACK_SIZE)
        .spawn(|| main::Problem::init().main())
        .unwrap()
        .join()
        .unwrap()
}
