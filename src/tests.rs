use super::DominanceOrd;
use std::cmp::Ordering;

// Our multiobjective fitness/solution value
struct Tuple(pub usize, pub usize);

struct TupleDominanceOrd;

impl DominanceOrd for TupleDominanceOrd {
    type T = Tuple;

    fn dominance_ord(&self, a: &Self::T, b: &Self::T) -> Ordering {
        if a.0 < b.0 && a.1 <= b.1 {
            Ordering::Less
        } else if a.0 <= b.0 && a.1 < b.1 {
            Ordering::Less
        } else if a.0 > b.0 && a.1 >= b.1 {
            Ordering::Greater
        } else if a.0 >= b.0 && a.1 > b.1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[test]
fn test_non_domination() {
    let a = &Tuple(1, 2);
    let b = &Tuple(2, 1);

    // Non-domination due to reflexitivity
    assert_eq!(Ordering::Equal, TupleDominanceOrd.dominance_ord(a, a));
    assert_eq!(Ordering::Equal, TupleDominanceOrd.dominance_ord(b, b));

    // Non-domination
    assert_eq!(Ordering::Equal, TupleDominanceOrd.dominance_ord(a, b));
    assert_eq!(Ordering::Equal, TupleDominanceOrd.dominance_ord(b, a));
}

#[test]
fn test_domination() {
    let a = &Tuple(1, 2);
    let b = &Tuple(1, 3);
    let c = &Tuple(0, 2);

    // a < b
    assert_eq!(Ordering::Less, TupleDominanceOrd.dominance_ord(a, b));
    // c < a
    assert_eq!(Ordering::Less, TupleDominanceOrd.dominance_ord(c, a));
    // transitivity => c < b
    assert_eq!(Ordering::Less, TupleDominanceOrd.dominance_ord(c, b));

    // Just reverse the relation: forall a, b: a < b => b > a

    // b > a
    assert_eq!(Ordering::Greater, TupleDominanceOrd.dominance_ord(b, a));
    // a > c
    assert_eq!(Ordering::Greater, TupleDominanceOrd.dominance_ord(a, c));
    // transitivity => b > c
    assert_eq!(Ordering::Greater, TupleDominanceOrd.dominance_ord(b, c));
}
