use crate::utils;

#[macro_export]
macro_rules! ki {
    ($a:ident $b:ident) => {
        $crate::units::key_identifier::KeyIdentifier::from_str(stringify!($a), stringify!($b))
    };
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct KeyIdentifier {
    device_alias: String,
    code: u16,
}

impl KeyIdentifier {
    pub fn new(device_alias: &str, code: u16) -> Self {
        Self {
            device_alias: device_alias.to_string(),
            code,
        }
    }

    pub fn from_str(device_alias: &str, key_name: &str) -> Self {
        Self {
            device_alias: device_alias.to_uppercase(),
            code: utils::test_utils::key_code_from_str(key_name).unwrap(),
        }
    }

    pub fn device_alias(&self) -> &str {
        self.device_alias.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

#[cfg(test)]
mod test {
    use crate::units::key_identifier::KeyIdentifier;

    #[test]
    fn can_construct_key_identifier_from_device_alias_and_code() {
        let key = KeyIdentifier::new("L1", 32);
        assert_eq!(key.device_alias(), "L1");
        assert_eq!(key.code(), 32);
    }

    #[test]
    fn can_use_macro_to_create_key_identifier_for_testing_purposes() {
        let key = ki!(L1 D);
        assert_eq!(key.device_alias(), "L1");
        assert_eq!(key.code(), 32);
    }
}
