use std::fmt::Display;

use chrono::Datelike;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct Date(chrono::NaiveDate);

impl From<chrono::NaiveDate> for Date {
    fn from(value: chrono::NaiveDate) -> Self {
        Self(value)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}-{}-{}",
            self.0.year(),
            self.0.month(),
            self.0.day()
        ))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct DateVisitor;

impl Visitor<'_> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting a date in %Y-%m-%d format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Date(
            match chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d") {
                Ok(x) => x,
                Err(e) => return Err(E::custom(e.to_string())),
            },
        ))
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateVisitor)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State(pub Date, pub u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct StateMap(pub Vec<State>);

impl<'a> Default for StateMap {
    fn default() -> Self {
        let mut v = Vec::new();
        for _ in 0..crate::consts::GRID_SIZE {
            v.push(State::default())
        }

        Self(v)
    }
}
