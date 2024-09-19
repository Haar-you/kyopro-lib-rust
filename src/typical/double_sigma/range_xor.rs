/// ΣΣ 範囲のxor総和
///
/// Σ{i = 1 ~ N}Σ{j = i ~ N} aᵢ ⊕ aᵢ ₊ ₁ ⊕ ... ⊕ aⱼ
///
/// **Time complexity O(64 * |a|)**
///
/// # Problems
/// - <https://atcoder.jp/contests/abc365/tasks/abc365_e>
pub fn sum_of_sum_of_range_xor(a: Vec<u64>) -> u128 {
    let mut ret = 0;

    for b in 0..64 {
        let mut count = [0, 0];
        let mut sum = 0;

        for &a in &a {
            if a & (1 << b) == 0 {
                count[0] += 1;
            } else {
                count.swap(0, 1);
                count[1] += 1;
            }

            sum += count[1];
        }

        ret += sum << b;
    }

    ret
}
