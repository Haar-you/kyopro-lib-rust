use crate::utils::lower_bound::*;

#[derive(Clone)]
pub struct Compressor<T> {
    data: Vec<T>
}

#[derive(Clone)]
pub struct CompressorBuilder<T> {
    data: Vec<T>
}

impl<T> Compressor<T>
where
    T: Clone + Ord + PartialEq
{
    pub fn index(&self, value: T) -> usize {
        lower_bound(&self.data, value)
    }

    pub fn get(&self, i: usize) -> T {
        self.data[i].clone()
    }

    pub fn compress(&self, values: Vec<T>) -> Vec<usize> {
        values.iter().map(|x| self.index(x.clone())).collect()
    }

    pub fn decompress(&self, indices: Vec<usize>) -> Vec<T> {
        indices.into_iter().map(|i| self.get(i)).collect()
    }
}

impl<T> CompressorBuilder<T>
where
    T: Clone + Ord + PartialEq
{
    pub fn new() -> Self {
        CompressorBuilder {
            data: vec![]
        }
    }

    pub fn add(&mut self, value: T) -> &Self{
        self.data.push(value);
        self
    }

    pub fn add_vec(&mut self, values: Vec<T>) -> &Self {
        self.data.extend(values);
        self
    }

    pub fn build(&mut self) -> Compressor<T> {
        self.data.sort();
        self.data.dedup();
        Compressor {
            data: self.data.clone()
        }
    }
}


#[cfg(test)]
mod tests {
}
