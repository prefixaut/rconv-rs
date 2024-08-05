use std::fmt::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ParseErrorCode {
    /// When there's free floating content which isn't associated with any property.
    StepmaniaExpectedPropertyStart,
    /// When inside the property-name definition, and an invalid character/amount of characters is encountered.
    StepmaniaInvalidPropertyName,
    /// When a property-name is not recognised and therefore will not be parsed.
    StepmaniaUnknownPropertyName,
    /// When a property-name is duplicated
    StepmaniaDuplicatePropertyName,
    /// When a property-end ";" was expected, but hasn't been found.
    StepmaniaExpectedValueEnd,
    /// When an EOF is reached unexpectedly
    StepmaniaUnexpectedEOF,
    /// When the property-value is an invalid number
    StepmaniaInvalidNumber,
    /// When the property-value is an invalid string
    StepmaniaInvalidString,
    /// When the property-value is an invalid number-range
    StepmaniaInvalidNumberRange,
    /// When the property-value is an invalid boolean
    StepmaniaInvalidBoolean,
    /// When the property-value is an invalid color-value (i.E. "1.1", "255", whatever)
    StepmaniaInvalidColorValue,
    /// When the property count is not valid for the property
    StepmaniaInvalidValueCount,
}

#[derive(Debug)]
pub struct ParseError {
    pub code: ParseErrorCode,
    pub line: usize,
    pub column: usize,
    pub len: usize,
}
