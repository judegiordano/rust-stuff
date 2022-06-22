use std::any::type_name;

pub fn type_of<T>(_: &T) -> &'static str {
    return type_name::<T>();
}
