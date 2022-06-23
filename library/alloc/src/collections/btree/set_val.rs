pub trait IsSetVal {
    fn is_set_val(&self) -> bool;
}

impl<V> IsSetVal for V {
    default fn is_set_val(&self) -> bool {
        false
    }
}

impl IsSetVal for () {
    fn is_set_val(&self) -> bool {
        true
    }
}
