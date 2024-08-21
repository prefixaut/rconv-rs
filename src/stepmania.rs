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

impl StepmaniaColor {
    fn default() -> Self {
        StepmaniaColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedVisualChange {
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
pub struct StepmaniaTimedDuration {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// Duration of the stop in ms
    pub duration: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedBPM {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// Duration of the stop in ms
    pub bpm: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedTimeSignature {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// Numerator the signature
    pub numerator: u8,
    /// Denominator of the signature
    pub denominator: u8,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedNumber {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// The value/number
    pub value: i32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedComboChange {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// How much a single hit is worth for the combo
    pub hit: u32,
    /// How much a single miss will deal damage
    pub miss: u32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedSpeedChange {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// The ratio to be applied
    pub ratio: f32,
    /// How long the change should be applied for in ms or in s if `in_seconds` is true
    pub duration: u32,
    /// If the `duration` should be timed in seconds instead of milli-seconds
    pub in_seconds: bool,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedScrollSpeedChange {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// The factor to apply
    pub factor: f32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedLabel {
    /// Beat when the stop starts in ms
    pub beat: i64,
    /// Label content to display
    pub label: String,
}

#[derive(Debug, Default)]
pub struct StepmaniaNumberRange {
    /// Lower bounds
    pub min: i64,
    /// Upper bounds
    pub max: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaRadioValues {
    stream: f32,
    voltage: f32,
    air: f32,
    freeze: f32,
    chaos: f32,
}

#[derive(Debug, Default)]
pub struct StepmaniaAttackModifier {
    /// Name of the Modifier
    name: String,
    /// The name of the player the modifier is applied to
    player: String,
    /// Approach rate how to ease the modifier
    approach_rate: i32,
    /// The magnitude of the modifier
    magnitude: f32,
    /// If the magnitude should be interpreted as percent
    is_percent: bool,
}

#[derive(Debug, Default)]
pub struct StepmaniaAttack {
    duration: i64,
    modifiers: Vec<String>,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedAttack {
    beat: i64,
    duration: i64,
    modifiers: Vec<StepmaniaAttackModifier>,
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
    pub instrument_tracks: Vec<StepmaniaInstrumentTrack>,
    pub background_changes: Vec<StepmaniaTimedVisualChange>,
    pub background_changes2: Vec<StepmaniaTimedVisualChange>,
    pub background_changes3: Vec<StepmaniaTimedVisualChange>,
    pub animations: Vec<StepmaniaTimedVisualChange>,
    pub foreground_changes: Vec<StepmaniaTimedVisualChange>,
    pub offset: Option<i64>,
    pub keysounds: Vec<String>,
    pub stops: Vec<StepmaniaTimedDuration>,
    pub delays: Vec<StepmaniaTimedDuration>,
    pub fakes: Vec<StepmaniaTimedDuration>,
    pub bpms: Vec<StepmaniaTimedBPM>,
    pub time_signatures: Vec<StepmaniaTimedTimeSignature>,
    pub attacks: Vec<StepmaniaTimedAttack>,
    pub tick_counts: Vec<StepmaniaTimedNumber>,
    pub combos: Vec<StepmaniaTimedComboChange>,
    pub speeds: Vec<StepmaniaTimedSpeedChange>,
    pub scrolls: Vec<StepmaniaTimedScrollSpeedChange>,
    pub labels: Vec<StepmaniaTimedLabel>,
}

#[derive(Debug, Default, Clone)]
struct UnparsedPropertyValue {
    pub raw: String,
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

#[derive(Debug, Default)]
pub struct StepmaniaParser {
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
const CHAR_COLOR_SEPARATOR: char = '^';

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
        if precision > 0 {
            for _ in [0..(precision - 1)] {
                str_val.push('0');
            }
        }

        if let Some(idx_val) = idx {
            str_val.remove(idx_val);
            let tmp: usize = precision.into();
            str_val = str_val.chars().take(idx_val + tmp).collect();
        }

        match str_val.trim().parse::<i64>() {
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
        value: &UnparsedPropertyValue,
        groups: bool,
    ) -> Vec<Vec<UnparsedPropertyValue>> {
        let mut list: Vec<Vec<UnparsedPropertyValue>> = vec![];
        let mut latest_obj: Vec<UnparsedPropertyValue> = vec![];
        let mut has_latest = false;

        let mut line = value.line;
        let mut column = value.column;
        let mut start_pos: usize = 0;

        for (current_pos, c) in value.raw.chars().enumerate() {
            if c == CHAR_OBJ_VAL_SEPARATOR && groups {
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
                let len = current_pos - start_pos;

                if len > 0 {
                    latest_obj.push(UnparsedPropertyValue {
                        raw: value.raw.chars().skip(start_pos).take(len).collect(),
                        line,
                        column,
                        len,
                    });
                }

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

    fn parse_to_color_channel(&mut self, value: UnparsedPropertyValue) -> Option<u8> {
        match value.raw.trim().parse::<f32>() {
            Ok(float) => {
                let parsed = (255.0 * float.clamp(0.0, 1.0)) as u8;
                Some(parsed)
            }
            Err(_) => {
                self.errors.push(ParseError {
                    code: ParseErrorCode::StepmaniaInvalidColorValue,
                    line: value.line,
                    column: value.column,
                    len: value.len,
                });
                None
            }
        }
    }

    fn parse_to_color(&mut self, value: UnparsedPropertyValue) -> StepmaniaColor {
        let mut start_pos = 0;
        let mut line = value.line;
        let mut column = value.column;
        let mut color = StepmaniaColor::default();
        let mut color_pos = 0;

        for (current_pos, c) in value.raw.chars().enumerate() {
            if c != CHAR_COLOR_SEPARATOR {
                if c == CHAR_LINE_BREAK {
                    line += 1;
                    column = 0;
                    continue;
                }
                column += 1;
                continue;
            }

            let len = current_pos - start_pos;
            if let Some(cv) = self.parse_to_color_channel(UnparsedPropertyValue {
                raw: value.raw.chars().skip(start_pos).take(len).collect(),
                line,
                column,
                len,
            }) {
                match color_pos {
                    0 => color.red = cv,
                    1 => color.green = cv,
                    2 => color.blue = cv,
                    3 => color.alpha = cv,
                    _ => {}
                }
            }

            start_pos = current_pos + 1;
            color_pos += 1;
            column += 1;
        }

        if start_pos > 0 {
            let len = value.raw.len() - start_pos + 1;
            if let Some(cv) = self.parse_to_color_channel(UnparsedPropertyValue {
                raw: value.raw.chars().skip(start_pos).take(len).collect(),
                line,
                column,
                len,
            }) {
                match color_pos {
                    0 => color.red = cv,
                    1 => color.green = cv,
                    2 => color.blue = cv,
                    3 => color.alpha = cv,
                    _ => {}
                }
            }
        }

        color
    }

    fn add_value_count_error(&mut self, entry: &Vec<UnparsedPropertyValue>) {
        let first = entry.get(0).unwrap();
        let mut total_len = entry.len() - 1;
        for p in entry.iter() {
            total_len += p.len;
        }
        self.errors.push(ParseError {
            code: ParseErrorCode::StepmaniaInvalidValueCount,
            line: first.line,
            column: first.column,
            len: total_len,
        });
    }

    fn parse_value_group<T, F>(
        &mut self,
        value: &UnparsedPropertyValue,
        min: usize,
        max: usize,
        mut mapper_fn: F,
    ) -> Vec<T>
    where
        F: FnMut(&mut Self, Vec<UnparsedPropertyValue>) -> Option<T>,
    {
        let mut list: Vec<T> = vec![];

        for group in self.parse_to_value_entries(&value, true) {
            let len = group.len();
            if len < min {
                continue;
            }

            if len < max {
                self.add_value_count_error(&group);
            }

            if let Some(val) = mapper_fn(self, group) {
                list.push(val);
            }
        }

        list
    }

    fn parse_to_visual_change(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedVisualChange> {
        let len = entry.len();
        let mut bg = StepmaniaTimedVisualChange {
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

        if len > 0 {
            if let Some(beat) = self.parse_to_number(entry.remove(0), PRECISION_TIME) {
                bg.beat = beat;
            }
        }
        if len > 1 {
            bg.path = entry.remove(0).raw;
        }
        if len > 2 {
            let fp = entry.remove(0);
            match fp.raw.trim().parse::<f32>() {
                Ok(float) => bg.update_rate = float as i64,
                Err(_) => self.errors.push(ParseError {
                    code: ParseErrorCode::StepmaniaInvalidNumber,
                    line: fp.line,
                    column: fp.column,
                    len: fp.len,
                }),
            }
        }
        if len > 3 {
            bg.crossfade = self.parse_to_bool(entry.remove(0));
        }
        if len > 4 {
            bg.stretch_rewind = self.parse_to_bool(entry.remove(0));
        }
        if len > 5 {
            bg.stretch_no_loop = self.parse_to_bool(entry.remove(0));
        }
        if len > 6 {
            bg.effect = entry.remove(0).raw;
        }
        if len > 7 {
            bg.file2 = entry.remove(0).raw;
        }
        if len > 8 {
            bg.transition = entry.remove(0).raw;
        }
        if len > 9 {
            bg.color1 = self.parse_to_color(entry.remove(0));
        }
        if len > 10 {
            bg.color2 = self.parse_to_color(entry.remove(0));
        }

        Some(bg)
    }

    fn parse_to_string_list(&mut self, value: UnparsedPropertyValue) -> Vec<String> {
        self.parse_to_value_entries(&value, false)
            .iter()
            .filter_map(|entry| entry.get(0).map(|v| v.raw.clone()))
            .collect()
    }

    fn parse_to_instrument_track(
        &mut self,
        group: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaInstrumentTrack> {
        Some(StepmaniaInstrumentTrack {
            instrument: group.get(0).unwrap().raw.clone(),
            file: group.get(1).unwrap().raw.clone(),
        })
    }

    fn parse_to_timed_duration(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedDuration> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let duration = self.parse_to_number(entry.remove(0), PRECISION_TIME);

        if beat.is_none() || duration.is_none() {
            return None;
        }

        return Some(StepmaniaTimedDuration {
            beat: beat.unwrap(),
            duration: duration.unwrap(),
        });
    }

    fn parse_to_timed_bpm(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedBPM> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let bpm = self.parse_to_number(entry.remove(0), PRECISION_TIME);

        if beat.is_none() || bpm.is_none() {
            return None;
        }

        return Some(StepmaniaTimedBPM {
            beat: beat.unwrap(),
            bpm: bpm.unwrap(),
        });
    }

    fn parse_to_timed_time_signature(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedTimeSignature> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let numerator = entry.remove(0).raw.trim().parse::<u8>();
        let denominator = entry.remove(0).raw.trim().parse::<u8>();

        if beat.is_none() || numerator.is_err() || denominator.is_err() {
            return None;
        }

        return Some(StepmaniaTimedTimeSignature {
            beat: beat.unwrap(),
            numerator: numerator.unwrap(),
            denominator: denominator.unwrap(),
        });
    }

    fn parse_to_timed_number(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedNumber> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let value = entry.remove(0).raw.trim().parse::<i32>();

        if beat.is_none() || value.is_err() {
            return None;
        }

        return Some(StepmaniaTimedNumber {
            beat: beat.unwrap(),
            value: value.unwrap(),
        });
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
                "subtitletranslit" => step.subtitle_translit = Some(value.raw),
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

                // visual changes
                "bgchanges" => {
                    step.background_changes = self.parse_value_group(&value, 1, 11, |tmp, group| {
                        tmp.parse_to_visual_change(group)
                    })
                }

                "bgchanges2" => {
                    step.background_changes2 =
                        self.parse_value_group(&value, 1, 11, |tmp, group| {
                            tmp.parse_to_visual_change(group)
                        })
                }
                "bgchanges3" => {
                    step.background_changes3 =
                        self.parse_value_group(&value, 1, 11, |tmp, group| {
                            tmp.parse_to_visual_change(group)
                        })
                }
                "fgchanges" => {
                    step.foreground_changes = self.parse_value_group(&value, 1, 11, |tmp, group| {
                        tmp.parse_to_visual_change(group)
                    })
                }
                "animations" => {
                    step.animations = self.parse_value_group(&value, 1, 11, |tmp, group| {
                        tmp.parse_to_visual_change(group)
                    })
                }

                // Keysounds
                "keysounds" => step.keysounds = self.parse_to_string_list(value),

                // Instrument Tracks
                "instrumenttracks" => {
                    step.instrument_tracks = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_instrument_track(group)
                    })
                }

                // Timed durations
                "stops" => {
                    step.stops = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_duration(group)
                    })
                }
                "delays" => {
                    step.delays = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_duration(group)
                    })
                }
                "fakes" => {
                    step.fakes = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_duration(group)
                    })
                }

                // Timed BPMs
                "bpms" => {
                    step.bpms = self
                        .parse_value_group(&value, 2, 2, |tmp, group| tmp.parse_to_timed_bpm(group))
                }

                // Time signatures
                "timesignatures" => {
                    step.time_signatures = self.parse_value_group(&value, 3, 3, |tmp, group| {
                        tmp.parse_to_timed_time_signature(group)
                    })
                }

                // Numbers
                "tickcounts" => {
                    step.tick_counts = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_number(group)
                    })
                }

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
