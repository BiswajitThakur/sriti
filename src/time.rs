use std::fmt;
use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, TimeParseError> {
    if input.is_empty() {
        return Err(TimeParseError::Empty);
    }
    if input.trim() == "0" {
        return Ok(Duration::ZERO);
    }

    let mut total = Duration::ZERO;
    let mut num = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            num.push(c);
            continue;
        }

        if num.is_empty() {
            return Err(TimeParseError::InvalidFormat);
        }

        let value: u64 = num.parse().map_err(|_| TimeParseError::InvalidNumber)?;
        num.clear();

        let unit = match c {
            'n' if chars.peek() == Some(&'s') => {
                chars.next();
                Duration::from_nanos(value)
            }
            'u' if chars.peek() == Some(&'s') => {
                chars.next();
                Duration::from_micros(value)
            }
            'm' if chars.peek() == Some(&'s') => {
                chars.next();
                Duration::from_millis(value)
            }
            's' => Duration::from_secs(value),
            'm' => Duration::from_secs(value * 60),
            'h' => Duration::from_secs(value * 60 * 60),
            'd' => Duration::from_secs(value * 60 * 60 * 24),
            _ => return Err(TimeParseError::UnknownUnit(c.to_string())),
        };

        total += unit;
    }

    if !num.is_empty() {
        return Err(TimeParseError::InvalidFormat);
    }

    Ok(total)
}

#[derive(Debug)]
pub enum TimeParseError {
    Empty,
    InvalidFormat,
    InvalidNumber,
    UnknownUnit(String),
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeParseError::Empty => write!(f, "empty time string"),
            TimeParseError::InvalidFormat => write!(f, "invalid time format"),
            TimeParseError::InvalidNumber => write!(f, "invalid number"),
            TimeParseError::UnknownUnit(u) => write!(f, "unknown unit: {}", u),
        }
    }
}

impl std::error::Error for TimeParseError {}
