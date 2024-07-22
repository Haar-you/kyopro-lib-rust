pub fn inplace_merge_by<T: Copy>(a: &mut [T], k: usize, cmp: fn(&T, &T) -> bool) {
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
        } else if j >= snd.len() || cmp(&fst[i], &snd[j]) {
            a[k] = fst[i];
            i += 1;
        } else {
            a[k] = snd[j];
            j += 1;
        }
        k += 1;
    }
}

pub fn inplace_merge<T: Ord + Copy>(a: &mut [T], k: usize) {
    inplace_merge_by(a, k, |x, y| x < y);
}

pub fn merge<T: Ord + Copy>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    let n = a.len();
    a.append(&mut b);
    inplace_merge(&mut a, n);
    a
}
