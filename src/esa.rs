use crate::sais::saisxx;
use crate::types::{SArray, StringT, SuffixError};
use std::convert::TryInto;

fn suffixtree(
    t: &StringT,
    sa: &mut SArray,
    l: &mut SArray,
    r: &mut SArray,
    d: &mut SArray,
    n: usize,
) -> usize {
    if n == 0 {
        return 0;
    }

    println!("-------------");
    println!("sa {:?}", sa);
    // Psi = l
    l[sa[0]] = sa[n - 1];
    for i in 1..n {
        l[sa[i]] = sa[i - 1];
    }
    // Compare at most 2n log n charcters. Practically fastest
    // "Permuted Longest-Common-Prefix Array", Juha Karkkainen, CPM 09
    // PLCP = r
    let mut h = 0;
    for i in 0..n {
        let j = l[i];
        while i + h < n && j + h < n && t[i + h] == t[j + h] {
            h += 1;
        }
        r[i] = h;
        if h > 0 {
            h -= 1;
        }
    }

    // H = l
    for i in 0..n {
        l[i] = r[sa[i]];
    }
    // TODO XXX: i32 necessary
    // l[0] = -1;

    let mut s: Vec<(i32, i32)> = vec![(-1, -1)];
    let mut node_num = 0;
    let mut i: usize = 0;
    loop {
        let mut cur: (i32, i32) = (i as i32, if i == n { -1 } else { l[i] as i32 });
        let mut cand = s[s.len() - 1];
        while cand.1 > cur.1 {
            if (i as i32) - cand.0 > 1 {
                l[node_num] = cand.0.try_into().unwrap();
                r[node_num] = i;
                d[node_num] = cand.1.try_into().unwrap();
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
        s.push((i.try_into().unwrap(), (n - sa[i] + 1).try_into().unwrap()));
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
