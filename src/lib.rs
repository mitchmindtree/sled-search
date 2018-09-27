pub extern crate sled;

use std::cmp::Ordering;

/// Search the key space of the tree for an entry using the `guide` function.
///
/// The `guide` function will return a `bool` alongside an `Ordering` where
///
/// - The `bool` indicates whether or not the search should be updated with the yielded entry.
/// - The `Ordering` indicates whether or not the given entry is greater than or less than the
///   target. If `Greater` is returned, the search will continue through lesser values. If `Less`
///   is returned, the search will continue through greater values.
///
/// Returns the last entry that caused the `guide` function to return `true`.
pub fn search<F>(tree: &sled::Tree, mut guide: F) -> sled::DbResult<Option<(Vec<u8>, Vec<u8>)>, ()>
where
    F: FnMut(&[u8], &[u8]) -> (bool, Ordering),
{
    let mut key = vec![];
    let mut last = None;
    loop {
        // Push a byte.
        let ix = key.len();
        key.push(0x00);

        // Search this index of the key.
        let mut attempt = 0;
        let mut last_update_attempt = None;
        let mut step = std::u8::MAX / 2 + 1;
        let mut some_ix = false;
        loop {
            key[ix] = attempt;
            match tree.scan(&key).next() {
                Some(Err(err)) => return Err(err),
                Some(Ok((k, v))) => {
                    if k.len() >= key.len() {
                        some_ix = true;
                    }
                    let (update, ord) = guide(&k, &v);
                    match ord {
                        Ordering::Less => attempt += step,
                        Ordering::Greater if key[ix] == 0 => break,
                        Ordering::Greater => attempt -= step,
                        Ordering::Equal => return Ok(Some((k, v))),
                    }
                    if update {
                        last = Some((k, v));
                        last_update_attempt = Some(key[ix]);
                    }
                }
                None if key[ix] == 0 => break,
                None => attempt -= step,
            }
            if step == 0 {
                break;
            }
            step /= 2;
        }

        // If there is no valid value for this index, pop and we're done!
        if !some_ix {
            key.pop();
            break;
        }

        if let Some(k) = last_update_attempt {
            key[ix] = k;
        }
    }

    Ok(last)
}

/// Find the maximum entry within the given `Tree` using a binary search.
///
/// 1. Repeatedly appending `0xFF` until scan next returns `None`.
/// 2. Binary search the final byte to find the greatest value that causes `scan.next` to return
///    `Some`.
/// 3. GOTO 1.
///
/// The key has been found when the binary search in step 2. finds no keys that return `Some`.
pub fn max(tree: &sled::Tree) -> sled::DbResult<Option<(Vec<u8>, Vec<u8>)>, ()> {
    search(tree, |_k, _v| (true, Ordering::Less))
}

/// Find the greatest entry that precedes the given key.
pub fn pred(tree: &sled::Tree, key: &[u8]) -> sled::DbResult<Option<(Vec<u8>, Vec<u8>)>, ()> {
    search(tree, |k, _v| match k >= key {
        true => (false, Ordering::Greater),
        false => (true, Ordering::Less),
    })
}

/// Find the entry at the given key or the greatest that precedes it if no entry for the key exists.
pub fn pred_incl(tree: &sled::Tree, key: &[u8]) -> sled::DbResult<Option<(Vec<u8>, Vec<u8>)>, ()> {
    search(tree, |k, _v| match k > key {
        true => (false, Ordering::Greater),
        false => (true, Ordering::Less),
    })
}
