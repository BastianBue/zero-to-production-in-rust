use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if ValidateEmail::validate_email(&s) == false {
            return Err(format!("Invalid subscriber email {}", s));
        }
        Ok(SubscriberEmail(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary(_: &mut Gen) -> Self {
            let email = SafeEmail().fake();
            ValidEmailFixture(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn a_valid_subscriber_email_can_be_constructed(valide_email: ValidEmailFixture) {
        let subscriber_email = SubscriberEmail::parse(valide_email.0);
        claims::assert_ok!(subscriber_email);
    }

    #[test]
    fn a_subscriber_email_must_have_an_at_sign() {
        let subscriber_email = SubscriberEmail::parse("rogergmail.com".into());
        claims::assert_err!(subscriber_email);
    }

    #[test]
    fn a_subscriber_email_must_have_a_valid_domain() {
        let subscriber_email = SubscriberEmail::parse("roger@.com".into());
        claims::assert_err!(subscriber_email);
    }

    #[test]
    fn must_have_a_valid_subject() {
        let subscriber_email = SubscriberEmail::parse("@gmail.com".into());
        claims::assert_err!(subscriber_email);
    }
}
