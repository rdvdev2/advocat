use curl::easy;

#[derive(Clone)]
pub struct Credentials {
    email: Vec<u8>,
    password: Vec<u8>,
}

impl Credentials {
    pub fn new(email: &[u8], password: &[u8]) -> Credentials {
        Credentials {
            email: Vec::from(email),
            password: Vec::from(password),
        }
    }

    pub fn build_form(&self) -> Option<easy::Form> {
        let mut form = easy::Form::new();

        form.part("email")
            .contents(self.email.as_slice())
            .add()
            .ok()?;
        form.part("password")
            .contents(self.password.as_slice())
            .add()
            .ok()?;
        form.part("submit").contents(b"").add().ok()?;

        Some(form)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn credentials_build_form_test() {
        let credentials = Credentials::new(b"me@example.com", b"1234");
        assert!(credentials.build_form().is_some());
    }
}
