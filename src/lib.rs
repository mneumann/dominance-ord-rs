use std::cmp::Ordering;

/// Trait for comparisons according to [dominance order][1].
///
/// The dominance relation is a partial order. Given solutions `a` and
/// `b`, a dominance comparison has three possible outcomes:
///
/// - Either solution `a` dominates solution `b` ("a < b"),
///
/// - or solution `b` dominates solution `a` ("a > b"),
///
/// - or neither solution `a` nor `b` dominates each other ("a == b").
///
/// The dominance relation is for example used in non-dominated sort
/// algorithms to obtain the Pareto fronts.
///
/// [1]: https://en.wikipedia.org/wiki/Dominance_order
///
pub trait DominanceOrd {
    /// The type on which the dominance relation is defined.
    type T;

    /// Returns the dominance order.
    fn dominance_ord(&self, a: &Self::T, b: &Self::T) -> Ordering {
        if self.dominates(a, b) {
            Ordering::Less
        } else if self.dominates(b, a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Returns true if `a` dominates `b` ("a < b").
    fn dominates(&self, a: &Self::T, b: &Self::T) -> bool {
        match self.dominance_ord(a, b) {
            Ordering::Less => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests;
