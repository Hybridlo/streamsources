pub fn is_default<T: Default + PartialEq>(item: &T) -> bool {
    item == &T::default()
}