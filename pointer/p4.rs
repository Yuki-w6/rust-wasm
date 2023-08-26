fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}