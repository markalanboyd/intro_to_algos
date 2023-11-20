pub fn vec_is_sorted<T, F>(v: &[T], mut is_ordered: F) -> bool
where
    T: PartialOrd, // PartialOrd for comparison
    F: FnMut(&T, &T) -> bool,
{
    if v.is_empty() {
        return true;
    }

    for (i, item) in v.iter().enumerate() {
        if i > 0 && is_ordered(&v[i - 1], item) {
            return false;
        }
    }
    true
}
