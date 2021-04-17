
#[derive(Debug, Clone)]
pub struct CumulativeSum1D<T> {
    data: Vec<T>
}

pub struct CumulativeSum1DBuilder<T> {
    n: usize,
    data: Vec<T>,
    zero: T
}

impl<T> CumulativeSum1D<T>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Clone
{
    pub fn fold(&self, l: usize, r: usize) -> T {
        self.data[r].clone() - self.data[l].clone()
    }
}

impl<T> CumulativeSum1DBuilder<T>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Clone
{
    pub fn new(n: usize, zero: T) -> Self {
        CumulativeSum1DBuilder {
            n: n,
            data: vec![zero.clone(); n],
            zero: zero
        }
    }

    pub fn update(&mut self, i: usize, value: T) -> &Self {
        self.data[i] = value;
        self
    }

    pub fn build(&self) -> CumulativeSum1D<T> {
        let mut data = vec![self.zero.clone(); self.n + 1];
        for i in 0 .. self.n {
            data[i + 1] = data[i].clone() + self.data[i].clone();
        }

        CumulativeSum1D {
            data: data
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
    }
}
