use crate::sais::saisxx;
use crate::types::{SArray, StringT, SuffixError};
use std::convert::TryInto;

fn suffixtree(
    string: &StringT,
    suffix_array: &mut SArray,
    left: &mut SArray,
    right: &mut SArray,
    depth: &mut SArray,
    n: usize,
) -> usize {
    if n == 0 {
        return 0;
    }

    // Psi = l
    left[suffix_array[0]] = suffix_array[n - 1];
    for i in 1..n {
        left[suffix_array[i]] = suffix_array[i - 1];
    }
    // Compare at most 2n log n charcters. Practically fastest
    // "Permuted Longest-Common-Prefix Array", Juha Karkkainen, CPM 09
    // PLCP = r
    let mut h = 0;
    for i in 0..n {
        let j = left[i];
        while i + h < n && j + h < n && string[i + h] == string[j + h] {
            h += 1;
        }
        right[i] = h;
        if h > 0 {
            h -= 1;
        }
    }

    // H = l
    for i in 0..n {
        left[i] = right[suffix_array[i]];
    }
    // TODO XXX: i32 necessary
    // l[0] = -1;

    let mut s: Vec<(i32, i32)> = vec![(-1, -1)];
    let mut node_num = 0;
    let mut i: usize = 0;
    loop {
        let mut cur: (i32, i32) = (i as i32, if i == n { -1 } else { left[i] as i32 });
        let mut cand = s[s.len() - 1];
        while cand.1 > cur.1 {
            if (i as i32) - cand.0 > 1 {
                left[node_num] = cand.0.try_into().unwrap();
                right[node_num] = i;
                depth[node_num] = cand.1.try_into().unwrap();
                node_num += 1;
                if node_num >= n {
                    break;
                }
            }
            cur.0 = cand.0;
            s.pop();
            cand = s[s.len() - 1];
        }
        if cand.1 < cur.1 {
            s.push(cur);
        }
        if i == n {
            break;
        }
        s.push((i.try_into().unwrap(), (n - suffix_array[i] + 1).try_into().unwrap()));
        i += 1;
    }
    node_num
}

pub(crate) fn esaxx_rs(
    t: &StringT,
    sa: &mut SArray,
    l: &mut SArray,
    r: &mut SArray,
    d: &mut SArray,
    k: usize,
) -> Result<usize, SuffixError> {
    let n = t.len();
    if k <= 0 {
        return Err(SuffixError::InvalidLength);
    }
    saisxx(t, sa, n, k)?;
    let node_num = suffixtree(t, sa, l, r, d, n);
    Ok(node_num)
}
