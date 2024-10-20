/// 0以上n以下の自然数について、ビット毎にそのビットが立っている数の個数を数える。
///
/// # Problems
/// - <https://atcoder.jp/contests/abc356/tasks/abc356_d>
/// - <https://yukicoder.me/problems/no/2939>
pub fn bitwise_sum_popcount(n: u64) -> [u64; 64] {
    let mut ans = [0; 64];

    for i in 0..64 {
        if n < (1 << i) {
            break;
        }

        let n = n as u128 + 1;
        let c = 1 << i;
        let dc = 1 << (i + 1);

        ans[i] = ((n / dc) * c) as u64;

        if n % dc >= c && n % dc < dc {
            ans[i] += (n % dc - c) as u64;
        }
    }

    ans
}
