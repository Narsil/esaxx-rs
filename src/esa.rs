use crate::sais::saisxx;
use crate::types::{SArray, StringT, SuffixError};
use std::convert::TryInto;

fn suffixtree(
    string: &StringT,
    suffix_array: &mut SArray,
    left: &mut SArray,
    right: &mut SArray,
    depth: &mut SArray,
    n: u32,
) -> u32 {
    if n == 0 {
        return 0;
    }

    // Psi = l
    left[suffix_array[0] as usize] = suffix_array[n as usize - 1];
    for i in 1..n as usize {
        left[suffix_array[i] as usize] = suffix_array[i - 1];
    }
    // Compare at most 2n log n charcters. Practically fastest
    // "Permuted Longest-Common-Prefix Array", Juha Karkkainen, CPM 09
    // PLCP = r
    let mut h = 0;
    for i in 0..n as usize {
        let j = left[i] as usize;
        while i + h < n as usize && j + h < n as usize && string[i + h] == string[j + h] {
            h += 1;
        }
        right[i] = h as u32;
        if h > 0 {
            h -= 1;
        }
    }

    // H = l
    for i in 0..n as usize {
        left[i] = right[suffix_array[i] as usize];
    }
    // TODO XXX: i32 necessary
    // l[0] = -1;

    let mut s: Vec<(i32, i32)> = vec![(-1, -1)];
    let mut node_num = 0;
    let mut i: usize = 0;
    loop {
        let mut cur: (i32, i32) = (i as i32, if i == n as usize { -1 } else { left[i] as i32 });
        let mut cand = s[s.len() - 1];
        while cand.1 > cur.1 {
            if (i as i32) - cand.0 > 1 {
                left[node_num] = cand.0.try_into().unwrap();
                right[node_num] = i as u32;
                depth[node_num] = cand.1.try_into().unwrap();
                node_num += 1;
                if node_num >= n as usize {
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
        if i == n as usize {
            break;
        }
        s.push((
            i.try_into().unwrap(),
            (n - suffix_array[i] + 1).try_into().unwrap(),
        ));
        i += 1;
    }
    node_num as u32
}

pub(crate) fn esaxx_rs(
    string: &StringT,
    suffix_array: &mut SArray,
    left: &mut SArray,
    right: &mut SArray,
    depth: &mut SArray,
    k: u32,
) -> Result<u32, SuffixError> {
    let n = string.len() as u32;
    saisxx(string, suffix_array, n, k)?;
    let node_num = suffixtree(string, suffix_array, left, right, depth, n);
    Ok(node_num)
}
