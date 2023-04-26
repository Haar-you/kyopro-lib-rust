#[macro_export]
macro_rules! for_loop {
    ($init:stmt;  $end:expr; $update:stmt; $b:block) => {
        #[allow(redundant_semicolons)]
        {
            $init;
            while $end {
                $b;
                $update;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut x = 0;
        for_loop!(x += 1; x < 100; x += 1; {
            println!("{}", x);
        });
    }
}
