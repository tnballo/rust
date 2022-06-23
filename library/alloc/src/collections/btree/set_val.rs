pub trait IsSetVal<V> {
    fn is_set_val() -> bool;
}

impl<V> IsSetVal<V> for V {
    default fn is_set_val() -> bool {
        false
    }
}

impl<V> IsSetVal<V> for () {
    fn is_set_val() -> bool {
        true
    }
}
