pub fn vec_is_sorted<T, F>(v: &[T], mut is_ordered: F) -> bool
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    if v.is_empty() {
        return true;
    }

    for window in v.windows(2) {
        if is_ordered(&window[0], &window[1]) {
            return false;
        }
    }
    true
}
