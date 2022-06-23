/// A trait to differentiate between `BTreeMap` and `BTreeSet` values.
/// Returns `true` only for type `()`, `false` for all other types (blanket implementation).
/// `TypeId` requires a `'static` lifetime, use of this trait avoids that restriction.
///
/// [`TypeId`]: std::any::TypeId
pub trait IsSetVal {
    fn is_set_val() -> bool;
}

// Blanket implementation
impl<V> IsSetVal for V {
    default fn is_set_val() -> bool {
        false
    }
}

// Specialization
impl IsSetVal for () {
    fn is_set_val() -> bool {
        true
    }
}
