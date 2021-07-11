// https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm
// NOTE: the impl only works for ascii string
pub fn rabin_karp(src: &str, pat: &str) {
    let base = 256;
    let modulo = 1_000_000_009;

    let src = src.as_bytes();
    let pat = pat.as_bytes();
    let m = src.len();
    let n = pat.len();
    if n > m {
        return;
    }
    let base_pow_n = {
        let mut res = 1;
        let mut n = n;
        while n > 1 {
            res = (res * base) % modulo;
            n -= 1;
        }
        res
    };
    let (mut src_hash, pat_hash) = {
        let mut src_hash = 0;
        let mut pat_hash = 0;
        for i in 0..n {
            src_hash = (src_hash * base + src[i] as isize) % modulo;
            pat_hash = (pat_hash * base + pat[i] as isize) % modulo;
        }
        (src_hash, pat_hash)
    };
    for i in 0..=(m - n) {
        if src_hash == pat_hash {
            let matched = (0..n).all(|j| src[i + j] == pat[j]);
            if matched {
                println!("found match at idx: {}", i);
            }
        }

        if i < (m - n) {
            src_hash -= (src[i] as isize) * base_pow_n;
            src_hash *= base;
            src_hash += src[i + n] as isize;
            src_hash %= modulo;
            if src_hash < 0 {
                src_hash += modulo;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        rabin_karp("aabbccrrccccrrrccrr", "ccrr");
    }
}
