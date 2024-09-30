use std::collections::HashMap;

use anyhow::Result;

use super::common::*;

#[derive(Debug, Default)]
pub struct StepmaniaInstrumentTrack {
    /// The instrument name
    pub instrument: String,
    /// Relative path to the instrument track
    pub file: String,
}

#[derive(Debug)]
pub struct StepmaniaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Default for StepmaniaColor {
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
    /// At which beat the visual change should apply
    pub beat: i64,
    /// Path to the file for the change
    pub path: String,
    /// The rate of how fast the image/video should be played
    pub play_rate: i64,
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
    /// At which beat the duration should apply
    pub beat: i64,
    /// Duration of the stop in ms
    pub duration: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedBPM {
    /// At which beat the bpm change should apply
    pub beat: i64,
    /// Duration of the stop in ms
    pub bpm: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedTimeSignature {
    /// At which beat the time signature should apply
    pub beat: i64,
    /// Numerator the signature
    pub numerator: u8,
    /// Denominator of the signature
    pub denominator: u8,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedNumber {
    /// At which beat the value should be applied
    pub beat: i64,
    /// The value/number
    pub value: i32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedComboChange {
    /// At which beat the combo change should apply
    pub beat: i64,
    /// How much a single hit is worth for the combo
    pub hit: u32,
    /// How much a single miss will deal damage
    pub miss: u32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedSpeedChange {
    /// At which beat the time-speed change should apply
    pub beat: i64,
    /// The ratio to be applied
    pub ratio: f32,
    /// How long the change should be applied for in ms or in s if `in_seconds` is true
    pub duration: i64,
    /// If the `duration` should be timed in seconds instead of milli-seconds
    pub in_seconds: bool,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedScrollSpeedChange {
    /// At which beat the scroll-speed change should apply
    pub beat: i64,
    /// The factor to apply
    pub factor: f32,
}

#[derive(Debug, Default)]
pub struct StepmaniaTimedLabel {
    /// At which beat the label should appear
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

#[derive(Debug)]
pub enum StepmaniaBPMRange {
    /// A single BPM value
    Single(i64),
    /// A range of BPM (from - to)
    Range(i64, i64),
    /// A randomly updated/displayed BPM
    Random,
}

#[derive(Debug, Default)]
pub struct StepmaniaWarp {
    /// At which beat the warp starts
    pub beat: i64,
    /// At which beat the warp ends
    pub end_beat: i64,
}

#[derive(Debug, Default)]
pub struct StepmaniaRadarValues {
    pub stream: f32,
    pub voltage: f32,
    pub air: f32,
    pub freeze: f32,
    pub chaos: f32,
}

#[derive(Debug)]
pub enum StepmaniaMagnitude {
    /// The amount in %
    Percent(u16),
    /// The amount in a 1000s parsed integer (i.E 1 = 1000 like the regular ms parsing)
    Amount(i64),
}

impl Default for StepmaniaMagnitude {
    fn default() -> Self {
        StepmaniaMagnitude::Percent(0)
    }
}

#[derive(Debug, Default)]
pub struct StepmaniaAttackModifier {
    /// Name of the Modifier
    pub name: String,
    /// The name of the player the modifier is applied to.
    /// Left empty to target all players.
    pub player: Option<String>,
    /// Approach rate how to ease the modifier
    pub approach_rate: Option<u8>,
    /// The magnitude of the modifier.
    pub magnitude: StepmaniaMagnitude,
}

#[derive(Debug, Default)]
pub struct StepmaniaNoteAttack {
    /// Duration of the attack in seconds (NOT BEATS)
    pub duration: i64,
    /// The modifiers to apply during the Attack
    pub modifiers: Vec<StepmaniaAttackModifier>,
}

#[derive(Debug, Default)]
pub struct StepmaniaAttack {
    /// The start of the Attack in seconds (NOT BEATS)
    pub start: i64,
    /// The duration for how long the Attack lasts in seconds (NOT BEATS)
    pub duration: i64,
    /// The modifiers to apply during the Attack
    pub modifiers: Vec<StepmaniaAttackModifier>,
}

#[derive(Debug, Clone)]
pub enum StepmaniaNoteType {
    Empty,
    Tap,
    HoldHead,
    RollHead,
    Tail,
    Mine,
    Keysound,
    Lift,
    Fake,
}

impl Default for StepmaniaNoteType {
    fn default() -> Self {
        StepmaniaNoteType::Empty
    }
}

impl StepmaniaNoteType {
    pub fn from_char(c: char) -> Self {
        match c {
            NOTE_EMPTY => StepmaniaNoteType::Empty,
            NOTE_TAP => StepmaniaNoteType::Tap,
            NOTE_HOLD_HEAD => StepmaniaNoteType::HoldHead,
            NOTE_ROLL_HEAD => StepmaniaNoteType::RollHead,
            NOTE_TAIL => StepmaniaNoteType::Tail,
            NOTE_MINE => StepmaniaNoteType::Mine,
            NOTE_KEYSOUND => StepmaniaNoteType::Keysound,
            NOTE_LIFT => StepmaniaNoteType::Lift,
            NOTE_FAKE => StepmaniaNoteType::Fake,
            _ => StepmaniaNoteType::Empty,
        }
    }
}

#[derive(Debug)]
pub enum StepmaniaDifficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Challenge,
    Edit,
}

impl Default for StepmaniaDifficulty {
    fn default() -> Self {
        StepmaniaDifficulty::Edit
    }
}

impl StepmaniaDifficulty {
    pub fn from_str(str: &str) -> Self {
        match str.to_lowercase().as_str() {
            "beginner" => StepmaniaDifficulty::Beginner,
            "easy" => StepmaniaDifficulty::Easy,
            "medium" => StepmaniaDifficulty::Medium,
            "hard" => StepmaniaDifficulty::Hard,
            "challange" => StepmaniaDifficulty::Challenge,
            _ => StepmaniaDifficulty::Edit,
        }
    }
}

#[derive(Debug, Default)]
pub struct StepmaniaChart {
    /// Name of the chart (SSC)
    pub name: Option<String>,
    /// The type of the game mode (i.E. dance-single, dance-double, ...)
    pub step_style: String,
    // Custom name for the step-style
    pub chart_style: Option<String>,
    /// The charter/creator
    pub credit: String,
    /// Difficulty of the chart
    pub difficulty: StepmaniaDifficulty,
    /// The difficulty/rating of the chart as number
    pub meter: u16,
    /// Radar-Values which describe the skillsets
    pub radar_values: StepmaniaRadarValues,
    /// The note-data
    pub data: StepmaniaNoteData,
}

#[derive(Debug, Default)]
pub struct StepmaniaNoteData {
    pub column_count: u8,
    pub notes: Vec<Vec<StepmaniaNote>>,
}

#[derive(Debug, Default)]
pub struct StepmaniaNote {
    pub note_type: StepmaniaNoteType,
    pub keysound: Option<u32>,
    pub actions: Vec<StepmaniaNoteAttack>,
}

#[derive(Debug, Default, Clone)]
pub struct UnparsedPropertyValue {
    pub raw: String,
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

#[derive(Debug, Default)]
pub struct StepmaniaFile {
    /// Version of the SSC format
    pub version: Option<String>,
    /// Title of the song, in the original language
    pub title: Option<String>,
    /// Title of the song, translated to english (if title isn't english already)
    pub title_translit: Option<String>,
    /// Subtitle of the song, in the original language
    pub subtitle: Option<String>,
    /// Subtitle of the song, translated to english (if subtitle isn't english already)
    pub subtitle_translit: Option<String>,
    /// The Artist of the song, in the original language
    pub artist: Option<String>,
    /// The Artist of the song, "translated" to english (usually only romanji versions)
    pub artist_translit: Option<String>,
    /// The genre of the song
    pub genre: Option<String>,
    /// The author or the pack/mix of the chart
    pub credit: Option<String>,
    /// Relative path to the banner image
    pub banner: Option<String>,
    /// Relative path to the background image
    pub background: Option<String>,
    /// Relative path to the lyrics file (.lrc)
    pub lyrics_path: Option<String>,
    /// Relative path to the cd-title image
    pub cd_title: Option<String>,
    /// Relative path to the music file
    pub music: Option<String>,
    /// Where the song came from if it's a crossover from an existing game.
    pub origin: Option<String>,
    /// Relative path to the jacket image
    pub jacket: Option<String>,
    /// Relative path to the CD image
    pub cd_image: Option<String>,
    /// Relative path to the disk image
    pub disk_image: Option<String>,
    /// Relative path to the preview song/file.
    /// If provided, will be used instead of `sample_start` and `sample_length`.
    pub preview: Option<String>,
    /// Starting time for the sample/preview in ms
    pub sample_start: Option<i64>,
    /// Duration/length of the sample/preview in ms
    pub sample_length: Option<i64>,
    /// A float value. Tells StepMania when to end the song if your longest chart is shorter than this value.
    /// Normally song length is determined by the longest chart.
    /// Required if your chart has only EDIT difficulties, as EDITs are not factored into song length calculation.
    pub last_second_hint: Option<i64>,
    /// The range of BPM the song has
    // TODO: Change to StepmaniaBPMRange
    pub display_bpm: Option<StepmaniaNumberRange>,
    /// If the chart is selectable/should be hidden
    pub selectable: bool,
    /// The different assignments of instruments and their audio file
    pub instrument_tracks: Vec<StepmaniaInstrumentTrack>,
    /// Transitions/Changes to the background layer 1
    pub background_changes: Vec<StepmaniaTimedVisualChange>,
    /// Transitions/Changes to the background layer 2
    pub background_changes2: Vec<StepmaniaTimedVisualChange>,
    /// Transitions/Changes to the background layer 3
    pub background_changes3: Vec<StepmaniaTimedVisualChange>,
    /// Transitions/Changes to the animations layer
    pub animations: Vec<StepmaniaTimedVisualChange>,
    /// Transitions/Changes to the foreground layer
    pub foreground_changes: Vec<StepmaniaTimedVisualChange>,
    /// The offset between the beginning of the song and the start of the note data in ms
    pub offset: Option<i64>,
    /// Keysound files which are referenced in the note-data
    pub keysounds: Vec<String>,
    /// The stops to apply at specific times
    pub stops: Vec<StepmaniaTimedDuration>,
    /// The delays to apply at specific times
    pub delays: Vec<StepmaniaTimedDuration>,
    /// The fake sections to apply at specific times
    pub fakes: Vec<StepmaniaTimedDuration>,
    /// BPM changes to apply at specific times
    pub bpms: Vec<StepmaniaTimedBPM>,
    /// Time signature changes to apply at specific times
    pub time_signatures: Vec<StepmaniaTimedTimeSignature>,
    /// Attacks to apply at specific times
    pub attacks: Vec<StepmaniaAttack>,
    /// The checkpoint-hold tick rate count to apply at specific times
    pub tick_counts: Vec<StepmaniaTimedNumber>,
    /// The combo changes to apply at specific times
    pub combos: Vec<StepmaniaTimedComboChange>,
    /// The speed changes to apply at specific times
    pub speeds: Vec<StepmaniaTimedSpeedChange>,
    /// The scroll-speed changes to apply at specific times
    pub scrolls: Vec<StepmaniaTimedScrollSpeedChange>,
    /// The warps to be applied at specific times
    pub warps: Vec<StepmaniaWarp>,
    /// The labels to display at specific times
    pub labels: Vec<StepmaniaTimedLabel>,
    /// The SM chart content
    // TODO: Change to vec, since SSC can define multiple charts in a file
    pub notes: Option<StepmaniaChart>,
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

#[derive(Debug, PartialEq, PartialOrd)]
enum ChartParserState {
    Type,
    Credits,
    Difficulty,
    Rating,
    RadioValues,
    Notes,
    InlineKeysound,
    InlineAttack,
}

impl ChartParserState {
    pub fn next(&self) -> ChartParserState {
        match self {
            ChartParserState::Type => ChartParserState::Credits,
            ChartParserState::Credits => ChartParserState::Difficulty,
            ChartParserState::Difficulty => ChartParserState::Rating,
            ChartParserState::Rating => ChartParserState::RadioValues,
            _ => ChartParserState::Notes,
        }
    }
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
const CHAR_ATTACK_VALUE_SEPARATOR: char = ':';
const CHAR_ATTACK_KEY_SEPARATOR: char = '=';
const CHAR_NOTE_PROP_SEPARATOR: char = ':';
const CHAR_INLINE_ATTACK_START: char = '{';
const CHAR_INLINE_ATTACK_END: char = '}';
const CHAR_INLINE_KEYSOUND_START: char = '[';
const CHAR_INLINE_KEYSOUND_END: char = ']';
const CHAR_BEAT_SEPARATOR: char = ',';

const NOTE_EMPTY: char = '0';
const NOTE_TAP: char = '1';
const NOTE_HOLD_HEAD: char = '2';
const NOTE_ROLL_HEAD: char = '4';
const NOTE_TAIL: char = '3';
const NOTE_MINE: char = 'M';
const NOTE_KEYSOUND: char = 'K';
const NOTE_LIFT: char = 'L';
const NOTE_FAKE: char = 'F';

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
        let mut str_val = value.raw.trim().to_string();
        let idx = str_val.find(".");
        // Always make sure we have at least the required precision
        if precision > 0 {
            for _ in 0..precision {
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
            play_rate: 0,
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
                Ok(float) => bg.play_rate = float as i64,
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
            .filter_map(|entry| entry.get(0).map(|v| v.raw.clone().trim().to_string()))
            .collect()
    }

    fn parse_to_instrument_track(
        &mut self,
        group: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaInstrumentTrack> {
        Some(StepmaniaInstrumentTrack {
            instrument: group.get(0).unwrap().raw.clone().trim().to_string(),
            file: group.get(1).unwrap().raw.clone().trim().to_string(),
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

    fn parse_to_timed_combo_change(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedComboChange> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let hit = entry.remove(0).raw.trim().parse::<u32>();
        let miss = entry.remove(0).raw.trim().parse::<u32>();

        if beat.is_none() || hit.is_err() || miss.is_err() {
            return None;
        }

        return Some(StepmaniaTimedComboChange {
            beat: beat.unwrap(),
            hit: hit.unwrap(),
            miss: miss.unwrap(),
        });
    }

    fn parse_to_timed_speed_change(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedSpeedChange> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let ratio = entry.remove(0).raw.trim().parse::<f32>();
        let duration = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let in_seconds = entry.remove(0).raw.trim().parse::<u32>();

        if beat.is_none() || ratio.is_err() || duration.is_none() || in_seconds.is_err() {
            return None;
        }

        return Some(StepmaniaTimedSpeedChange {
            beat: beat.unwrap(),
            ratio: ratio.unwrap(),
            duration: duration.unwrap(),
            in_seconds: in_seconds.unwrap() == 1,
        });
    }

    fn parse_to_timed_scroll_speed_change(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedScrollSpeedChange> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);
        let factor = entry.remove(0).raw.trim().parse::<f32>();

        if beat.is_none() || factor.is_err() {
            return None;
        }

        return Some(StepmaniaTimedScrollSpeedChange {
            beat: beat.unwrap(),
            factor: factor.unwrap(),
        });
    }

    fn parse_to_timed_label(
        &mut self,
        mut entry: Vec<UnparsedPropertyValue>,
    ) -> Option<StepmaniaTimedLabel> {
        let beat = self.parse_to_number(entry.remove(0), PRECISION_TIME);

        if beat.is_none() {
            return None;
        }

        return Some(StepmaniaTimedLabel {
            beat: beat.unwrap(),
            label: entry.remove(0).raw.trim().to_string(),
        });
    }

    fn parse_attack_modifiers(
        &mut self,
        value: UnparsedPropertyValue,
    ) -> Vec<StepmaniaAttackModifier> {
        vec![]
    }

    fn parse_attacks(&mut self, value: UnparsedPropertyValue) -> Vec<StepmaniaAttack> {
        // I have it so much that attacks have this abomination of a format
        // Absolute ass to parse and is different from all other properties for no reason.
        let mut list: Vec<StepmaniaAttack> = vec![];

        // In progress elements
        let mut segment_name: String = String::new();
        let mut start_val: i64 = 0;
        let mut len_val: i64 = 0;
        let mut element_idx = 0;

        // Positions
        let mut current_line = value.line;
        let mut current_pos = 0;
        let mut start_line = value.line;
        let mut start_pos = 0;

        for c in value.raw.chars() {
            if c == CHAR_ATTACK_KEY_SEPARATOR {
                let len: usize = current_pos - start_pos;
                segment_name = value
                    .raw
                    .chars()
                    .skip(start_pos)
                    .take(len)
                    .collect::<String>()
                    .trim()
                    .to_lowercase();
                start_line = current_line;
                start_pos = current_pos + 1;
            } else if c == CHAR_ATTACK_VALUE_SEPARATOR {
                let len: usize = current_pos - start_pos;
                let tmp_value = value
                    .raw
                    .chars()
                    .skip(start_pos)
                    .take(len)
                    .collect::<String>();
                let tmp_unparsed = UnparsedPropertyValue {
                    line: start_line,
                    column: start_pos,
                    len,
                    raw: tmp_value,
                };

                match (element_idx, segment_name.as_str()) {
                    (_, "time") => {
                        if element_idx != 0 {
                            self.errors.push(ParseError {
                                code: ParseErrorCode::StepmaniaInvalidAttackValueOrder,
                                line: start_line,
                                column: start_pos,
                                len,
                            });
                            // Reset to make the next steps not screw up completely.
                            element_idx = 0;
                        }

                        if let Some(time) = self.parse_to_number(tmp_unparsed, PRECISION_TIME) {
                            start_val = time;
                        }
                    }
                    (1, "end") | (1, "len") => {
                        if let Some(val) = self.parse_to_number(tmp_unparsed, PRECISION_TIME) {
                            len_val = if segment_name.as_str() == "len" {
                                val
                            } else {
                                val - start_val
                            };
                        }
                    }
                    (2, "mods") => {
                        list.push(StepmaniaAttack {
                            start: start_val,
                            duration: len_val,
                            modifiers: self.parse_attack_modifiers(tmp_unparsed),
                        });
                        start_val = 0;
                        len_val = 0;
                    }
                    _ => self.errors.push(ParseError {
                        code: ParseErrorCode::StepmaniaInvalidAttackValue,
                        line: start_line,
                        column: start_pos,
                        len,
                    }),
                }

                start_line = current_line;
                start_pos = current_pos + 1;
                element_idx = (element_idx + 1) % 3;
            }

            if c == CHAR_LINE_BREAK {
                current_line += 1;
            }

            current_pos += 1;
        }

        // TODO: Find a way to prevent copy-paste here
        let len: usize = value.raw.len() - start_pos;
        let tmp_value = value
            .raw
            .chars()
            .skip(start_pos)
            .take(len)
            .collect::<String>();
        let tmp_unparsed = UnparsedPropertyValue {
            line: start_line,
            column: start_pos,
            len,
            raw: tmp_value,
        };

        match (element_idx, segment_name.as_str()) {
            (_, "time") => {
                if element_idx != 0 {
                    self.errors.push(ParseError {
                        code: ParseErrorCode::StepmaniaInvalidAttackValueOrder,
                        line: start_line,
                        column: start_pos,
                        len,
                    });
                    // Reset to make the next steps not screw up completely.
                    element_idx = 0;
                }

                if let Some(time) = self.parse_to_number(tmp_unparsed, PRECISION_TIME) {
                    start_val = time;
                }
            }
            (1, "end") | (1, "len") => {
                if let Some(val) = self.parse_to_number(tmp_unparsed, PRECISION_TIME) {
                    len_val = if segment_name.as_str() == "len" {
                        val
                    } else {
                        val - start_val
                    };
                }
            }
            (2, "mods") => {
                list.push(StepmaniaAttack {
                    start: start_val,
                    duration: len_val,
                    modifiers: self.parse_attack_modifiers(tmp_unparsed),
                });
            }
            _ => self.errors.push(ParseError {
                code: ParseErrorCode::StepmaniaInvalidAttackValue,
                line: start_line,
                column: start_pos,
                len,
            }),
        }

        return list;
    }

    fn parse_to_radio_values(
        &mut self,
        input: UnparsedPropertyValue,
    ) -> Option<StepmaniaRadarValues> {
        Some(StepmaniaRadarValues {
            ..Default::default()
        })
    }

    fn parse_to_chart(&mut self, input: UnparsedPropertyValue) -> Option<StepmaniaChart> {
        let mut state = ChartParserState::Type;
        let mut start_idx: usize = 0;
        let mut line = input.line;
        let mut col = input.column;
        let mut chart = StepmaniaChart {
            ..Default::default()
        };
        let mut current_beat_notes: Vec<StepmaniaNote> = vec![];

        for (idx, c) in input.raw.chars().enumerate() {
            match state {
                ChartParserState::Type
                | ChartParserState::Credits
                | ChartParserState::Difficulty
                | ChartParserState::Rating
                | ChartParserState::RadioValues => {
                    if c != CHAR_NOTE_PROP_SEPARATOR {
                        if c == CHAR_LINE_BREAK {
                            col = 1;
                            line += 1;
                        } else {
                            col += 1;
                        }
                        continue;
                    }

                    let str = input
                        .raw
                        .chars()
                        .skip(start_idx)
                        .take(idx)
                        .collect::<String>()
                        .trim()
                        .to_owned();
                    match state {
                        ChartParserState::Type => chart.step_style = str,
                        ChartParserState::Credits => chart.credit = str,
                        ChartParserState::Difficulty => {
                            chart.difficulty = StepmaniaDifficulty::from_str(&str)
                        }
                        ChartParserState::Rating => match str.parse::<u16>() {
                            Ok(rating) => chart.meter = rating,
                            Err(_) => self.errors.push(ParseError {
                                code: ParseErrorCode::StepmaniaInvalidNumber,
                                column: col,
                                line: line,
                                len: str.len(),
                            }),
                        },
                        ChartParserState::RadioValues => {
                            if let Some(val) = self.parse_to_radio_values(UnparsedPropertyValue {
                                len: str.len(),
                                raw: str,
                                column: col,
                                line: line,
                            }) {
                                chart.radar_values = val;
                            }
                        }
                        _ => {
                            // Never happens
                        }
                    }

                    col += 1;
                    start_idx = idx + 1;
                    state = state.next();
                }
                ChartParserState::InlineAttack => {
                    if c == CHAR_INLINE_ATTACK_END {
                        state = ChartParserState::Notes;
                        // TODO: Do parsing
                        continue;
                    }
                }
                ChartParserState::InlineKeysound => {
                    if c == CHAR_INLINE_KEYSOUND_END {
                        state = ChartParserState::Notes;
                        col += 1;
                        // TOOD: Do parsing
                        continue;
                    }
                }
                ChartParserState::Notes => match c {
                    CHAR_INLINE_ATTACK_START => {
                        state = ChartParserState::InlineAttack;
                        col += 1;
                        continue;
                    }
                    CHAR_INLINE_KEYSOUND_START => {
                        state = ChartParserState::InlineKeysound;
                        col += 1;
                        continue;
                    }
                    CHAR_BEAT_SEPARATOR => {
                        chart.data.notes.push(current_beat_notes);
                        current_beat_notes = vec![];
                        col += 1;
                    }
                    // All valid note types
                    NOTE_EMPTY | NOTE_TAP | NOTE_HOLD_HEAD | NOTE_ROLL_HEAD | NOTE_TAIL
                    | NOTE_MINE | NOTE_KEYSOUND | NOTE_LIFT | NOTE_FAKE => {
                        current_beat_notes.push(StepmaniaNote {
                            note_type: StepmaniaNoteType::from_char(c),
                            actions: vec![],
                            keysound: None,
                        });
                        col += 1;
                    }
                    // We need to find the column count and we do this not based on the type for now,
                    // but based on the note count in the first line.
                    CHAR_LINE_BREAK => {
                        if chart.data.column_count == 0 && current_beat_notes.len() > 0 {
                            match u8::try_from(current_beat_notes.len()) {
                                Ok(col) => chart.data.column_count = col,
                                Err(_) => {
                                    // TODO: Handle error
                                }
                            }
                        }
                        col = 1;
                        line += 1;
                    }
                    _ => {
                        col += 1;
                        if !c.is_whitespace() {
                            // TODO: Throw error
                            continue;
                        }
                    }
                },
            }
        }

        if current_beat_notes.len() > 0 {
            chart.data.notes.push(current_beat_notes)
        }

        return Some(chart);
    }

    pub fn parse_from_string(&mut self, input: &String) -> Result<StepmaniaFile> {
        let mut step: StepmaniaFile = StepmaniaFile::default();

        let result = self.parse_to_property_map(input);

        for (name, value) in result.unwrap() {
            match name.as_str() {
                // Simple string values
                "version" => step.version = Some(value.raw.trim().to_string()),
                "title" => step.title = Some(value.raw.trim().to_string()),
                "titletranslit" => step.title_translit = Some(value.raw.trim().to_string()),
                "subtitle" => step.subtitle = Some(value.raw.trim().to_string()),
                "subtitletranslit" => step.subtitle_translit = Some(value.raw.trim().to_string()),
                "artist" => step.artist = Some(value.raw.trim().to_string()),
                "artisttranslist" => step.artist_translit = Some(value.raw.trim().to_string()),
                "genre" => step.genre = Some(value.raw.trim().to_string()),
                "credit" => step.credit = Some(value.raw.trim().to_string()),
                "banner" => step.banner = Some(value.raw.trim().to_string()),
                "background" => step.background = Some(value.raw.trim().to_string()),
                "lyricspath" => step.lyrics_path = Some(value.raw.trim().to_string()),
                "cdtitle" => step.cd_title = Some(value.raw.trim().to_string()),
                "music" => step.music = Some(value.raw.trim().to_string()),
                "origin" => step.origin = Some(value.raw.trim().to_string()),
                "jacket" => step.jacket = Some(value.raw.trim().to_string()),
                "cdimage" => step.cd_image = Some(value.raw.trim().to_string()),
                "diskimage" => step.disk_image = Some(value.raw.trim().to_string()),
                "preview" => step.preview = Some(value.raw.trim().to_string()),

                // Simple inline match
                "selectable" => {
                    match value.raw.to_lowercase().trim() {
                        "yes"
                        // backwards compatibility
                        | "roulette" | "es" | "omes" | "1" => {

                        }
                        _ => step.selectable = false,
                    }
                }

                // Number values
                "samplestart" => step.sample_start = self.parse_to_number(value, PRECISION_TIME),
                "samplelength" => step.sample_length = self.parse_to_number(value, PRECISION_TIME),
                "offset" => step.offset = self.parse_to_number(value, PRECISION_TIME),
                "displaybpm" => {
                    step.display_bpm = self.parse_to_number_range(value, PRECISION_TIME)
                }
                "lastsecondhint" => step.last_second_hint = self.parse_to_number(value, PRECISION_TIME),

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
                "freezes" | "stops" => {
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

                // Attacks
                "attacks" => step.attacks = self.parse_attacks(value),

                // Combo changes
                "combos" => {
                    step.combos = self.parse_value_group(&value, 3, 3, |tmp, group| {
                        tmp.parse_to_timed_combo_change(group)
                    })
                }

                // Speed changes
                "speeds" => {
                    step.speeds = self.parse_value_group(&value, 4, 4, |tmp, group| {
                        tmp.parse_to_timed_speed_change(group)
                    })
                }

                // Scroll speed changes
                "scrolls" => {
                    step.scrolls = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_scroll_speed_change(group)
                    })
                }

                // Labels
                "labels" => {
                    step.labels = self.parse_value_group(&value, 2, 2, |tmp, group| {
                        tmp.parse_to_timed_label(group)
                    })
                }

                // Notes
                "notes" => {
                    step.notes = self.parse_to_chart(value);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_title() {
        let mut parser = StepmaniaParser::new();
        let data = "
#TITLE:ゾンビー・サーカス;
";
        let res = parser.parse_from_string(&data.to_string());
        assert!(res.is_ok());
        assert_eq!(parser.errors.len(), 0);

        let chart = res.unwrap();
        assert!(chart.title.is_some());
        assert_eq!(chart.title.unwrap(), "ゾンビー・サーカス");
    }

    #[test]
    fn it_should_parse_title_translit() {
        let mut parser = StepmaniaParser::new();
        let data = "
#TITLETRANSLIT:  hello world! ;
";
        let res = parser.parse_from_string(&data.to_string());
        assert!(res.is_ok());
        assert_eq!(parser.errors.len(), 0);

        let chart = res.unwrap();
        assert!(chart.title_translit.is_some());
        assert_eq!(chart.title_translit.unwrap(), "hello world!");
    }

    #[test]
    fn it_should_parse_sample() {
        let mut parser = StepmaniaParser::new();
        let data = "
#SAMPLESTART:1.333  ;
#SAMPLELENGTH: 83;
";
        let res = parser.parse_from_string(&data.to_string());
        assert!(res.is_ok());
        assert_eq!(parser.errors.len(), 0);

        let chart = res.unwrap();
        assert!(chart.sample_start.is_some());
        assert!(chart.sample_length.is_some());
        assert_eq!(chart.sample_start.unwrap(), 1333);
        assert_eq!(chart.sample_length.unwrap(), 83000);
    }

    #[test]
    fn it_should_parse_display_bpm() {
        let mut parser = StepmaniaParser::new();
        let data = "
#DISPLAYBPM:66.6668423 -240;
";
        let res = parser.parse_from_string(&data.to_string());
        assert!(res.is_ok());
        assert_eq!(parser.errors.len(), 0);

        let chart = res.unwrap();
        assert!(chart.display_bpm.is_some());
        let bpm = chart.display_bpm.unwrap();
        assert_eq!(bpm.min, 66666);
        assert_eq!(bpm.max, 240000);
    }

    #[test]
    fn it_should_parse_instrument_tracks() {
        let mut parser = StepmaniaParser::new();
        let data = "
#INSTRUMENTTRACKS:guitar=guiatarrr.ogg,
    drums= drums.mp3, vocal =yer.mp3;
";
        let res = parser.parse_from_string(&data.to_string());
        assert!(res.is_ok());
        assert_eq!(parser.errors.len(), 0);

        let chart = res.unwrap();
        assert_eq!(chart.instrument_tracks.len(), 3);

        let guitar = chart.instrument_tracks.get(0).unwrap();
        let drums = chart.instrument_tracks.get(1).unwrap();
        let vocals = chart.instrument_tracks.get(2).unwrap();

        assert_eq!(guitar.instrument, "guitar");
        assert_eq!(guitar.file, "guiatarrr.ogg");

        assert_eq!(drums.instrument, "drums");
        assert_eq!(drums.file, "drums.mp3");

        assert_eq!(vocals.instrument, "vocal");
        assert_eq!(vocals.file, "yer.mp3");
    }
}
