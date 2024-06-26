use super::{counting_sort, counting_sort_proof_stability, radix_sort, radix_sort_limit_digits,
            selection_sort, selection_sort_proof_stability};

#[test]
fn test_counting_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    counting_sort(&mut v, 10);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}

#[test]
fn test_radix_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    radix_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}

#[test]
fn test_radix_sort_limit_digits() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    radix_sort_limit_digits(&mut v, 2);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}

#[test]
fn test_counting_sort_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    counting_sort_proof_stability(&mut v, 10);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])

}

#[test]
fn test_selection_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    selection_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}

#[test]
fn test_selection_sort_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    selection_sort_proof_stability(&mut v);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])

}