use crate::utils::{code_to_key_name::code_to_key_name, for_testing::key_code_from_str};

#[cfg(test)]
mod test;

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
            code: key_code_from_str(key_name).unwrap(),
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
        write!(f, "{}|{}", self.device_alias, code_to_key_name(self.code))
    }
}
