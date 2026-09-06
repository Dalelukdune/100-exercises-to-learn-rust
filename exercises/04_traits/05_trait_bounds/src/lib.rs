// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.
pub fn min<T>(left: T, right: T) -> T
where
    T: PartialOrd
{
    if left <= right {
        left
    } else {
        right
    }
}
