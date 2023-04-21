pub mod main {
    use super::*;
    use haar_lib::{
        get,
        input,
        io,
        //chmin, chmax,
        //mul_vec,
        utils::join_str::*,
    };

    use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
    use std::io::Write;

    #[derive(Clone, Default)]
    pub struct Problem {/* write variables here */}

    impl Problem {
        pub fn main(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            io!(cin, cout);

            Ok(())
        }
        /* write functions here */
    }
}

fn main() {
    main::Problem::default().main().unwrap();
}
