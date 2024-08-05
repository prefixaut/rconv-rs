use std::collections::HashMap;
use std::fmt;

use anyhow::Result;

use super::common::*;

#[derive(Debug, Default)]
pub struct StepmaniaInstrumentTrack {
    pub instrument: String,
    pub file: String,
}

#[derive(Debug, Default)]
pub struct StepmaniaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Default)]
pub struct StepmaniaVisualChange {
    /// Beat of the change in ms
    pub beat: i64,
    /// Path to the file for the change
    pub path: String,
    /// The update rate of the change, f32 as i64
    pub update_rate: i64,
    /// If it should cross fade between the previous and this change
    pub crossfade: bool,
    pub stretch_rewind: bool,
    pub stretch_no_loop: bool,
    /// The effect to apply
    pub effect: String,
    /// Second file for changes
    pub file2: String,
    /// The transition effect between previous and this change
    pub transition: String,
    pub color1: StepmaniaColor,
    pub color2: StepmaniaColor,
}

#[derive(Debug, Default)]
pub struct StepmaniaStop {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// Duration of the stop in ms
    pub duration: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaNumberRange {
    min: i64,
    max: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaFile {
    pub title: Option<String>,
    pub title_translit: Option<String>,
    pub subtitle: Option<String>,
    pub subtitle_translit: Option<String>,
    pub artist: Option<String>,
    pub artist_translit: Option<String>,
    pub genre: Option<String>,
    pub credit: Option<String>,
    pub banner: Option<String>,
    pub background: Option<String>,
    pub lyricspath: Option<String>,
    pub cdtitle: Option<String>,
    pub music: Option<String>,
    pub sample_start: Option<i64>,
    pub sample_length: Option<i64>,
    pub display_bpm: Option<StepmaniaNumberRange>,
    pub instrument_tracks: Option<Vec<StepmaniaInstrumentTrack>>,
    pub background_changes: Option<Vec<StepmaniaVisualChange>>,
    pub background_changes2: Option<Vec<StepmaniaVisualChange>>,
    pub background_changes3: Option<Vec<StepmaniaVisualChange>>,
    pub animations: Option<Vec<StepmaniaVisualChange>>,
    pub foreground_changes: Option<Vec<StepmaniaVisualChange>>,
    pub offset: Option<i64>,
    pub keysounds: Option<Vec<String>>,
    pub stops: Option<Vec<StepmaniaStop>>,
}

#[derive(Debug, Default)]
struct UnparsedPropertyValue {
    pub raw: String,
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

impl fmt::Display for UnparsedPropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\" ({}:{} - {})",
            self.raw, self.line, self.column, self.len
        )
    }
}

#[derive(Debug, Default)]
pub struct StepmaniaParser {
    // The file name from where the source came from
    file_name: Option<String>,
    // The calculcated line we're currently on
    line: usize,
    // The calculcated column we're currently on
    col: usize,
    // The start-position in the buffer
    start_pos: usize,
    // All errors for the Parse-Result
    pub errors: Vec<ParseError>,
    // A map of the latest errors to update the length. Index is the error-code and gets cleaned on next valid char
    latest_errors: HashMap<ParseErrorCode, ParseError>,
    // The latest name/key we have to parse before hand.
    latest_name: String,
}

#[derive(Debug)]
enum ParserState {
    Clean,
    Name,
    Value,
    DoubleQuouteValue,
    SingleQouoteValue,
}

const CHAR_LINE_BREAK: char = '\n';
const CHAR_PROPERTY_START: char = '#';
const CHAR_VALUE_START: char = ':';
const CHAR_VALUE_END: char = ';';
const CHAR_OBJ_VAL_SEPARATOR: char = '=';
const CHAR_OBJ_SEPARATOR: char = ',';

const PRECISION_TIME: u8 = 3;
const PRECISION_COLOR: u8 = 2;

impl StepmaniaParser {
    pub fn new() -> StepmaniaParser {
        StepmaniaParser {
            line: 1,
            ..Default::default()
        }
    }

    fn update_read(&mut self, c: char) -> () {
        if c == CHAR_LINE_BREAK {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }

    fn create_error(&self, code: ParseErrorCode, pos: usize) -> ParseError {
        ParseError {
            code,
            column: self.col,
            line: self.line,
            len: pos,
        }
    }

    // Converts a stepmania color value to a hex value, where max is 255.
    fn color_value_to_hex(color: i32) -> f32 {
        return 255.0 * color as f32;
    }

    fn create_and_push_error(&mut self, code: ParseErrorCode, pos: usize) -> () {
        // Create the appropiate error if it doesn't exist yet
        if !self.latest_errors.contains_key(&code) {
            let err = self.create_error(code, pos);
            self.latest_errors.insert(code, err);
        }
    }

    fn cleanup_error(&mut self, code: ParseErrorCode, pos: usize) -> () {
        // If there has been an error before this, then update the length of the error and clear it from the map
        if self.latest_errors.contains_key(&code) {
            // Pops the error from the map
            let mut err = self.latest_errors.remove(&code).unwrap();
            // The previous current_pos was saved in err, therefore set it correctly here
            err.len = pos - err.len;
            // Now that the error is finished, push it into the errors vec
            self.errors.push(err);
        }
    }

    fn parse_to_property_map(
        &mut self,
        input: &String,
    ) -> Result<HashMap<String, UnparsedPropertyValue>> {
        // The map which will hold the unparsed values indexed by their keys
        let mut map: HashMap<String, UnparsedPropertyValue> = HashMap::new();

        // Parsing state "maschine"
        let mut state = ParserState::Clean;

        for (current_pos, c) in input.chars().enumerate() {
            match state {
                ParserState::Clean => {
                    // Ignore whitespaces/new lines before the actual file contents
                    if c.is_whitespace() {
                        self.update_read(c);
                        continue;
                    }

                    if c != CHAR_PROPERTY_START {
                        self.create_and_push_error(
                            ParseErrorCode::StepmaniaExpectedPropertyStart,
                            current_pos,
                        );
                        self.update_read(c);
                        continue;
                    }

                    self.cleanup_error(ParseErrorCode::StepmaniaExpectedPropertyStart, current_pos);
                    state = ParserState::Name;
                    self.start_pos = current_pos + 1;
                    self.update_read(c);
                    continue;
                }

                ParserState::Name => {
                    if c.is_whitespace() {
                        self.create_and_push_error(
                            ParseErrorCode::StepmaniaInvalidPropertyName,
                            current_pos,
                        );
                        self.update_read(c);
                        continue;
                    }

                    if c != CHAR_VALUE_START {
                        self.update_read(c);
                        continue;
                    }

                    // Copy the name of the property into latest_name, since we need it later
                    self.latest_name = input
                        .chars()
                        .skip(self.start_pos)
                        .take(current_pos - self.start_pos)
                        .collect::<String>()
                        .to_lowercase();

                    // Check if this property is a duplicate here, since this is the only place where we have proper
                    // line/col info.
                    if map.contains_key(&self.latest_name) {
                        let mut err = self.create_error(
                            ParseErrorCode::StepmaniaDuplicatePropertyName,
                            self.start_pos,
                        );
                        err.len = self.latest_name.len();
                        self.errors.push(err);
                    }

                    self.cleanup_error(ParseErrorCode::StepmaniaExpectedPropertyStart, current_pos);
                    state = ParserState::Value;
                    self.start_pos = current_pos + 1;
                    self.update_read(c);
                    continue;
                }

                ParserState::Value => {
                    if c == CHAR_VALUE_START {
                        self.create_and_push_error(
                            ParseErrorCode::StepmaniaExpectedValueEnd,
                            current_pos,
                        );
                        self.update_read(c);
                        continue;
                    }

                    if c != CHAR_VALUE_END {
                        self.update_read(c);
                        continue;
                    }

                    let len = current_pos - self.start_pos;
                    let value = input.chars().skip(self.start_pos).take(len).collect();
                    map.insert(
                        self.latest_name.to_owned(),
                        UnparsedPropertyValue {
                            raw: value,
                            line: self.line,
                            column: self.col,
                            len,
                        },
                    );

                    state = ParserState::Clean;
                    self.update_read(c);
                    continue;
                }

                _ => {
                    // TODO: Add quoute handling
                }
            }
        }

        if matches!(state, ParserState::Value) {
            self.create_and_push_error(ParseErrorCode::StepmaniaExpectedValueEnd, input.len());
        }

        Ok(map)
    }

    /// The numbers that we handle in SM files are typically always timings or time related.
    /// Converts them to ms times, and all precision after 3 is lost (we dont do ns timings here)
    fn parse_to_number(&mut self, value: UnparsedPropertyValue, precision: u8) -> Option<i64> {
        let mut str_val = value.raw;
        let idx = str_val.find(".");
        // Always make sure we have at least the required precision
        if (precision > 0) {
            for _ in [0..(precision - 1)] {
                str_val.push('0');
            }
        }

        if let Some(idx_val) = idx {
            str_val.remove(idx_val);
            let tmp: usize = precision.into();
            str_val = str_val.chars().take(idx_val + tmp).collect();
        }

        match str_val.parse::<i64>() {
            Ok(val) => Some(val),
            Err(err) => {
                println!("{:?}", err);
                self.errors.push(ParseError {
                    code: ParseErrorCode::StepmaniaInvalidNumber,
                    line: value.line,
                    column: value.column,
                    len: value.len,
                });
                None
            }
        }
    }

    fn parse_to_number_range(
        &mut self,
        value: UnparsedPropertyValue,
        precision: u8,
    ) -> Option<StepmaniaNumberRange> {
        let split_idx = value.raw.find("-");
        if let Some(idx) = split_idx {
            let min: String = value.raw.chars().take(idx).collect();
            let max: String = value.raw.chars().skip(idx + 1).collect();

            let min_val = self.parse_to_number(
                UnparsedPropertyValue {
                    len: min.len(),
                    raw: min,
                    line: value.line,
                    column: value.column,
                },
                precision,
            );
            let max_val = self.parse_to_number(
                UnparsedPropertyValue {
                    len: max.len(),
                    raw: max,
                    line: value.line,
                    column: value.column,
                },
                precision,
            );

            if min_val.is_some() && max_val.is_some() {
                return Some(StepmaniaNumberRange {
                    min: min_val.unwrap(),
                    max: max_val.unwrap(),
                });
            }

            None
        } else {
            return self
                .parse_to_number(value, precision)
                .map(|v| StepmaniaNumberRange { min: v, max: v });
        }
    }

    /// Parses "a=b=c=d=e=f,1=2=3=4=5=6" to "[[a,b,c,d,e,f], [1,2,3,4,5,6]]"
    fn parse_to_value_entries(
        &mut self,
        value: UnparsedPropertyValue,
    ) -> Vec<Vec<UnparsedPropertyValue>> {
        let mut list: Vec<Vec<UnparsedPropertyValue>> = vec![];
        let mut latest_obj: Vec<UnparsedPropertyValue> = vec![];
        let mut has_latest = false;

        let mut line = value.line;
        let mut column = value.column;
        let mut start_pos: usize = 0;

        for (current_pos, c) in value.raw.chars().enumerate() {
            if c == CHAR_OBJ_VAL_SEPARATOR {
                let len = current_pos - start_pos;
                latest_obj.push(UnparsedPropertyValue {
                    raw: value.raw.chars().skip(start_pos).take(len).collect(),
                    line,
                    column,
                    len,
                });
                column += 1;
                has_latest = true;
                start_pos = current_pos + 1;
                continue;
            }

            if c == CHAR_OBJ_SEPARATOR {
                list.push(latest_obj);
                latest_obj = vec![];
                has_latest = false;
                column += 1;
                start_pos = current_pos + 1;
                continue;
            }

            if c == CHAR_LINE_BREAK {
                line += 1;
                column = 0;
                continue;
            }

            column += 1;
        }

        if has_latest {
            // Finish any potentially lingering objects
            let len = value.raw.len() - start_pos;
            if len > 0 {
                latest_obj.push(UnparsedPropertyValue {
                    raw: value.raw.chars().skip(start_pos).take(len).collect(),
                    line,
                    column,
                    len,
                });
            }
            list.push(latest_obj);
        }

        list
    }

    fn parse_to_bool(&mut self, val: UnparsedPropertyValue) -> bool {
        return match val.raw.as_str() {
            "0" => false,
            "1" => true,
            _ => {
                self.errors.push(ParseError {
                    code: ParseErrorCode::StepmaniaInvalidBoolean,
                    line: val.line,
                    column: val.column,
                    len: val.len,
                });
                false
            }
        };
    }

    fn parse_to_single_visual_change(
        &mut self,
        mut parts: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaVisualChange> {
        let plen = parts.len();
        if plen == 0 {
            return None;
        }

        let first = parts.remove(0);
        let mut total_len = plen - 1;

        if plen != 11 {
            for p in parts.iter() {
                total_len += p.len;
            }
            self.errors.push(ParseError {
                code: ParseErrorCode::StepmaniaInvalidValueCount,
                line: first.line,
                column: first.column,
                len: total_len,
            });
        }

        let mut bg = StepmaniaVisualChange {
            beat: 0,
            path: String::new(),
            update_rate: 0,
            crossfade: false,
            stretch_rewind: false,
            stretch_no_loop: false,
            effect: String::new(),
            file2: String::new(),
            transition: String::new(),
            color1: StepmaniaColor::default(),
            color2: StepmaniaColor::default(),
        };

        if plen > 0 {
            if let Some(beat) = self.parse_to_number(first, PRECISION_TIME) {
                bg.beat = beat;
            }
        }
        if plen > 1 {
            bg.path = parts.remove(0).raw;
        }
        if plen > 2 {
            let fp = parts.remove(0);
            match fp.raw.parse::<f32>() {
                Ok(float) => bg.update_rate = float as i64,
                Err(_) => self.errors.push(ParseError {
                    code: ParseErrorCode::StepmaniaInvalidNumber,
                    line: fp.line,
                    column: fp.column,
                    len: fp.len,
                }),
            }
        }
        if plen > 3 {
            bg.crossfade = self.parse_to_bool(parts.remove(0));
        }
        if plen > 4 {
            bg.stretch_rewind = self.parse_to_bool(parts.remove(0));
        }
        if plen > 5 {
            bg.stretch_no_loop = self.parse_to_bool(parts.remove(0));
        }
        if plen > 6 {
            bg.effect = parts.remove(0).raw;
        }
        if plen > 7 {
            bg.file2 = parts.remove(0).raw;
        }
        if plen > 8 {
            bg.transition = parts.remove(0).raw;
        }

        Some(bg)
    }

    fn parse_to_visual_changes(
        &mut self,
        value: UnparsedPropertyValue,
    ) -> Option<Vec<StepmaniaVisualChange>> {
        let parts = self.parse_to_value_entries(value);
        if parts.len() == 0 {
            return None;
        }

        let mut list: Vec<StepmaniaVisualChange> = vec![];

        for group in parts {
            if let Some(bg) = self.parse_to_single_visual_change(group) {
                list.push(bg);
            }
        }

        Some(list)
    }

    pub fn parse_from_string(&mut self, input: &String) -> Result<StepmaniaFile> {
        let mut step: StepmaniaFile = StepmaniaFile::default();

        let result = self.parse_to_property_map(input);

        for (name, value) in result.unwrap() {
            match name.as_str() {
                // Simple string values
                "title" => step.title = Some(value.raw),
                "titletranslit" => step.title_translit = Some(value.raw),
                "subtitle" => step.subtitle = Some(value.raw),
                "artist" => step.artist = Some(value.raw),
                "artisttranslist" => step.artist_translit = Some(value.raw),
                "genre" => step.genre = Some(value.raw),
                "credit" => step.credit = Some(value.raw),
                "banner" => step.banner = Some(value.raw),
                "background" => step.background = Some(value.raw),
                "lyricspath" => step.lyricspath = Some(value.raw),
                "cdtitle" => step.cdtitle = Some(value.raw),
                "music" => step.music = Some(value.raw),

                // Number values
                "samplestart" => step.sample_start = self.parse_to_number(value, PRECISION_TIME),
                "samplelength" => step.sample_length = self.parse_to_number(value, PRECISION_TIME),
                "offset" => step.offset = self.parse_to_number(value, PRECISION_TIME),
                "displaybpm" => {
                    step.display_bpm = self.parse_to_number_range(value, PRECISION_TIME)
                }

                // background changes
                "bgchanges" => step.background_changes = self.parse_to_visual_changes(value),
                "bgchanges2" => step.background_changes2 = self.parse_to_visual_changes(value),
                "bgchanges3" => step.background_changes3 = self.parse_to_visual_changes(value),
                "fgchanges" => step.foreground_changes = self.parse_to_visual_changes(value),
                "animations" => step.animations = self.parse_to_visual_changes(value),

                // Unhandled keys are not recognised, and should be marked as correct warning/error
                _ => {
                    //     self.errors.push(ParseError {
                    //     code: ERROR_STEPMANIA_UNKNOWN_PROPERTY_NAME,
                    //     line: value.line,
                    //     column: value.column,
                    //     len: value.len,
                    // })
                }
            }
        }

        Ok(step)
    }
}
