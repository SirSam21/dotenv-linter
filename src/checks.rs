use crate::LineEntry;
use std::fmt;

mod incorrect_delimiter;
mod key_without_value;
mod leading_space;
mod lowercase_key;
mod spaces_around_equal;

#[derive(Debug, Clone, PartialEq)]
pub struct Warning {
    message: String,
}

impl Warning {
    fn new(message: String) -> Self {
        Self { message }
    }

    #[allow(dead_code)]
    fn from(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

trait Check {
    fn run(&self, line: &LineEntry) -> Option<Warning>;
}

fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(leading_space::LeadingSpaceChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(spaces_around_equal::SpacesAroundEqualChecker::default()),
    ]
}

pub fn run(line: &LineEntry) -> Vec<Warning> {
    checklist().iter().filter_map(|c| c.run(line)).collect()
}
