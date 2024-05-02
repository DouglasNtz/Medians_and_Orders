use super::{return_min, return_max, return_min_max, return_min_max_fast, randomized_select,
return_min_proof_stability, return_max_proof_stability, return_min_max_proof_stability,
return_min_max_fast_proof_stability, randomized_select_proof_stability};

#[test]
fn test_min() {

    let v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    let vmin = return_min(&v);
    assert_eq!(vmin, 0);
}
#[test]
fn test_max() {

    let v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    let vmax = return_max(&v);
    assert_eq!(vmax, 10);
}

#[test]
fn test_min_max() {

    let v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    let (vmin, vmax) = return_min_max(&v);
    assert_eq!((vmin, vmax), (0,10));
}
#[test]
fn test_min_max_fast() {

    let v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    let (vmin, vmax) = return_min_max_fast(&v);
    assert_eq!((vmin, vmax), (0,10));
}


#[test]
fn test_nsimo() {

    let v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    let setimo = randomized_select(&v,7);
    assert_eq!(setimo, 3);
    let segundo = randomized_select(&v,2);
    assert_eq!(segundo, 0);
    let quatorze = randomized_select(&v,14);
    assert_eq!(quatorze, 10);
}

#[test]
fn test_min_proof_stability() {

    let v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    let vmin = return_min_proof_stability(&v);
    assert_eq!(vmin, (0,2));
}

#[test]
fn test_max_proof_stability() {

    let v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    let vmax = return_max_proof_stability(&v);
    assert_eq!(vmax, (10,2));
}

#[test]
fn test_min_max_proof_stability() {

    let v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    let (vmin, vmax) = return_min_max_proof_stability(&v);
    assert_eq!((vmin, vmax), ((0,2), (10,2)));
}

#[test]
fn test_min_max_fast_proof_stability() {

    let v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    let (vmin, vmax) = return_min_max_fast_proof_stability(&v);
    assert_eq!((vmin, vmax), ((0,2), (10,2)));
}

#[test]
fn test_nsimo_proof_stability() {

    let v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    let setimo = randomized_select_proof_stability(&v,7);
    assert_eq!(setimo, (3,1));
    let segundo = randomized_select_proof_stability(&v,2);
    assert_eq!(segundo, (0,3));
    let quatorze = randomized_select_proof_stability(&v,14);
    assert_eq!(quatorze, (10,1));
}
