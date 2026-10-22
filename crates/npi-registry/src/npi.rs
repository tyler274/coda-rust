//! The National Provider Identifier and its check digit.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// A 10-digit NPI whose last digit is a valid Luhn check digit.
///
/// The check digit is computed over the nine leading digits prefixed with the
/// `80840` health-industry issuer code, as CMS specifies. Holding an `Npi`
/// proves only the format; whether NPPES lists it is a registry question.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Npi(String);

/// Why a string is not an NPI.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NpiError {
    /// The value is not exactly ten ASCII digits.
    #[error("an NPI is exactly 10 digits")]
    Format,
    /// The tenth digit does not match the Luhn check digit.
    #[error("the NPI check digit does not match")]
    CheckDigit,
}

impl Npi {
    /// Parses and validates an NPI. Surrounding whitespace is ignored.
    pub fn parse(value: &str) -> Result<Self, NpiError> {
        let value = value.trim();
        if value.len() != 10 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(NpiError::Format);
        }
        let digits: Vec<u32> = value.bytes().map(|byte| u32::from(byte - b'0')).collect();
        if check_digit(&digits[..9]) != digits[9] {
            return Err(NpiError::CheckDigit);
        }
        Ok(Self(value.to_owned()))
    }

    /// The ten digits.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The Luhn check digit for nine NPI digits, with the `80840` prefix.
fn check_digit(nine: &[u32]) -> u32 {
    // Doubling the `80840` prefix positions contributes a constant 24.
    let mut sum = 24;
    for (index, digit) in nine.iter().rev().enumerate() {
        sum += if index % 2 == 0 {
            let doubled = digit * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            *digit
        };
    }
    (10 - sum % 10) % 10
}

impl fmt::Display for Npi {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for Npi {
    type Err = NpiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Npi {
    type Error = NpiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<Npi> for String {
    fn from(npi: Npi) -> Self {
        npi.0
    }
}

impl AsRef<str> for Npi {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prefix_constant_matches_the_full_luhn_sum() {
        // Full Luhn over "80840" + nine digits, computed the long way.
        for nine in ["123456789", "198765432", "000000000", "999999999"] {
            let full = format!("80840{nine}");
            let mut sum = 0;
            for (index, ch) in full.chars().rev().enumerate() {
                let digit = ch.to_digit(10).unwrap();
                sum += if index % 2 == 0 {
                    let doubled = digit * 2;
                    if doubled > 9 { doubled - 9 } else { doubled }
                } else {
                    digit
                };
            }
            let digits: Vec<u32> = nine.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
            assert_eq!(check_digit(&digits), (10 - sum % 10) % 10, "{nine}");
        }
    }
}
