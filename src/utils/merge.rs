pub fn inplace_merge<T: Ord + Copy>(a: &mut [T], k: usize) {
    let fst = &a[0..k].to_vec();
    let snd = &a[k..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    loop {
        if i >= fst.len() {
            if j >= snd.len() {
                break;
            } else {
                a[k] = snd[j];
                j += 1;
            }
        } else {
            if j >= snd.len() {
                a[k] = fst[i];
                i += 1;
            } else {
                if fst[i] < snd[j] {
                    a[k] = fst[i];
                    i += 1;
                } else {
                    a[k] = snd[j];
                    j += 1;
                }
            }
        }
        k += 1;
    }
}

pub fn merge<T: Ord + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    let mut ret = vec![];
    ret.reserve(a.len() + b.len());
    ret.extend_from_slice(a);
    ret.extend_from_slice(b);
    inplace_merge(&mut ret, a.len());
    ret
}
