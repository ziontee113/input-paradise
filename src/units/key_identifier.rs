use super::keycode::key_code_from_str;

#[macro_export]
macro_rules! ki {
    ($a:ident $b:ident) => {
        $crate::units::key_identifier::KeyIdentifier::new(stringify!($a), stringify!($b))
    };
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct KeyIdentifier {
    device_alias: String,
    code: u16,
    key_name: String,
}

impl KeyIdentifier {
    pub fn new(device_alias: &str, key_name: &str) -> Self {
        Self {
            device_alias: device_alias.to_uppercase(),
            code: key_code_from_str(key_name).unwrap(),
            key_name: key_name.to_uppercase(),
        }
    }

    pub fn device_alias(&self) -> &str {
        self.device_alias.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

impl std::fmt::Display for KeyIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.device_alias, self.key_name)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn can_use_macro_to_create_key_identifier() {
        let key = ki!(L1 D);
        assert_eq!(key.to_string(), "L1 D");

        let key = ki!(r1 j);
        assert_eq!(key.to_string(), "R1 J");
    }
}
