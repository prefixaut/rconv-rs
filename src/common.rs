use std::fmt::*;

use std::fmt;

#[derive(Debug, Default)]
pub struct ParseError {
    /** The error code for */
    pub code: u32,
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} on {}:{} - {})", self.code, self.line, self.column, self.len)
    }
}

/* Error are defined in here so that we don't run into potential duplicate error codes. */

/// Indicator that this error is a severe one, where the parsing would normally stop or potentially break upcoming parsing.
pub const ERROR_SEVERE: u32 = 0b1100_0000_0000_0000;
/// Indicator that this error is a schemantic violation, but can technically be ignored, depending on the warning.
pub const ERROR_WARNING: u32 = 0b0100_0000_0000_0000;

/* Stepmania Errors */

/// When there's free floating content which isn't associated with any property.
pub const ERROR_STEPMANIA_EXPECTED_PROPERTY_START: u32 = ERROR_WARNING | 64;
/// When inside the property-name definition, and an invalid character/amount of characters is encountered.
pub const ERROR_STEPMANIA_INVALID_PROPERTY_NAME: u32 = ERROR_SEVERE | 65;
/// When a property-name is not recognised and therefore will not be parsed.
pub const ERROR_STEPMANIA_UNKNOWN_PROPERTY_NAME: u32 = ERROR_WARNING | 66;
/// When a property-name is duplicated
pub const ERROR_STEPMANIA_DUPLICATE_PROPERTY_NAME: u32 = ERROR_WARNING | 67;
/// When a property-end ";" was expected, but hasn't been found.
pub const ERROR_STEPMANIA_EXPECTED_VALUE_END: u32 = ERROR_WARNING | 68;
/// When an EOF is reached unexpectedly
pub const ERROR_STEPMANIA_UNEXPECTED_EOF: u32 = ERROR_SEVERE | 69;
/// When the property-value is an invalid number
pub const ERROR_STEPMANIA_INVALID_NUMBER: u32 = ERROR_WARNING | 70;
/// When the property-value is an invalid string
pub const ERROR_STEPMANIA_INVALID_STRING: u32 = ERROR_WARNING | 71;
/// When the property-value is an invalid number-range
pub const ERROR_STEPMANIA_INVALID_NUMBER_RANGE: u32 = ERROR_WARNING | 72;
/// When the property-value is an invalid boolean
pub const ERROR_STEPMANIA_INVALID_BOOLEAN: u32 = ERROR_WARNING | 73;
/// When the property-value is an invalid color-value (i.E. "1.1", "255", whatever)
pub const ERROR_STEPMANIA_INVALID_COLOR_VALUE: u32 = ERROR_WARNING | 74;
/// When the property count is not valid for the property
pub const ERROR_STEPMANIA_INVALID_VALUE_COUNT: u32 = ERROR_WARNING | 75;
