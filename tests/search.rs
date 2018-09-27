extern crate sled_search;

use sled_search::{max, pred, pred_incl, sled};

#[test]
fn test_max() {
    let config = sled::ConfigBuilder::new().temporary(true).build();
    let tree = sled::Tree::start(config).unwrap();
    assert!(max(&tree).unwrap().is_none());
    let entries = vec![
        (vec![1, 2, 3, 4], vec![2]),
        (vec![0, 0, 0, 0, 0, 9, 9], vec![1]),
        (vec![0], vec![0]),
        (vec![8, 255, 9], vec![3]),
        (vec![9], vec![4]),
    ];
    for (k, v) in entries {
        tree.set(k, v).unwrap();
    }
    assert_eq!(max(&tree).unwrap(), Some((vec![9], vec![4])));
}

#[test]
fn test_pred() {
    let config = sled::ConfigBuilder::new().temporary(true).build();
    let tree = sled::Tree::start(config).unwrap();
    assert!(pred(&tree, &[0]).unwrap().is_none());
    let entries = vec![
        (vec![1, 2, 3, 4], vec![2]),
        (vec![0, 0, 0, 0, 0, 9, 9], vec![1]),
        (vec![0], vec![0]),
        (vec![8, 255, 9], vec![3]),
        (vec![9], vec![4]),
        (vec![255, 255], vec![5]),
    ];
    for (k, v) in entries {
        tree.set(k, v).unwrap();
    }
    assert_eq!(
        pred(&tree, &[8, 255, 9]).unwrap(),
        Some((vec![1, 2, 3, 4], vec![2]))
    );
    assert_eq!(
        pred(&tree, &[1, 2, 3, 4]).unwrap(),
        Some((vec![0, 0, 0, 0, 0, 9, 9], vec![1]))
    );
    assert_eq!(pred(&tree, &[10]).unwrap(), Some((vec![9], vec![4])));
    assert_eq!(
        pred(&tree, &[0, 0, 0, 0, 0, 9, 9]).unwrap(),
        Some((vec![0], vec![0]))
    );
    assert_eq!(
        pred(&tree, &[255, 255, 1]).unwrap(),
        Some((vec![255, 255], vec![5]))
    );
    assert_eq!(pred(&tree, &[0]).unwrap(), None);
}

#[test]
fn test_pred_incl() {
    let config = sled::ConfigBuilder::new().temporary(true).build();
    let tree = sled::Tree::start(config).unwrap();
    assert!(pred(&tree, &[0]).unwrap().is_none());
    let entries = vec![
        (vec![1, 2, 3, 4], vec![2]),
        (vec![0, 0, 0, 0, 0, 9, 9], vec![1]),
        (vec![0], vec![0]),
        (vec![8, 255, 9], vec![3]),
        (vec![9], vec![4]),
        (vec![255, 255], vec![5]),
    ];
    for (k, v) in entries {
        tree.set(k, v).unwrap();
    }
    assert_eq!(
        pred_incl(&tree, &[8, 255, 9]).unwrap(),
        Some((vec![8, 255, 9], vec![3]))
    );
    assert_eq!(
        pred_incl(&tree, &[8, 255, 8]).unwrap(),
        Some((vec![1, 2, 3, 4], vec![2]))
    );
    assert_eq!(
        pred_incl(&tree, &[1, 2, 3, 4]).unwrap(),
        Some((vec![1, 2, 3, 4], vec![2]))
    );
    assert_eq!(pred_incl(&tree, &[10]).unwrap(), Some((vec![9], vec![4])));
    assert_eq!(
        pred_incl(&tree, &[0, 0, 0, 0, 0, 9, 8]).unwrap(),
        Some((vec![0], vec![0]))
    );
    assert_eq!(
        pred_incl(&tree, &[255, 255, 1]).unwrap(),
        Some((vec![255, 255], vec![5]))
    );
    assert_eq!(pred_incl(&tree, &[0]).unwrap(), Some((vec![0], vec![0])));
}
