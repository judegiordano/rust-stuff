#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::hash;

    use super::*;
    use crate::utils::password;

    #[test]
    fn password_should_hash() {
        let password = "ILovePuppies1234*";
        let hashed_password = password::hash(password);
        assert_ne!(hashed_password, password);
    }

    #[test]
    fn password_should_verify() {
        let password = "ILovePuppies1234*";
        let wrong_password = "ILovePuppies1234";
        let hashed_password = password::hash(password);
        let verify = password::verify(password, &hashed_password);
        let wrong_verify = password::verify(wrong_password, &hashed_password);
        assert_eq!(verify, true);
        assert_eq!(wrong_verify, false);
    }
}
