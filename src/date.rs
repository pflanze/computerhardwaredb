use anyhow::{Result, bail, Context, anyhow};
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use chrono::{NaiveDate, Datelike};


// and again, too ? chrono is pretty unbelievable, this required GPT.
pub fn unixtime_from_naivedate(nd: NaiveDate) -> i64 {
    nd.and_hms_opt(0, 0, 0).unwrap().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_unixtime_from_naivedate() {
        let t = |y, m, d| {
            unixtime_from_naivedate(NaiveDate::from_ymd_opt(y, m, d).unwrap())
        };
        assert_eq!(t(2024, 04, 01), 1711929600);
    }
}


#[derive(Debug, Clone)]
pub struct Date(NaiveDate);
impl Date {
    pub fn new(
        year: u16,
        month: u8,
        mday: u8,
    ) -> Result<Self> {
        if year < 1900 { bail!("year too small") }
        if year > 2200 { bail!("year too large") }
        if month > 12 { bail!("month too large") }
        if mday > 31 { bail!("mday too large") }
        if let Some(nd) = NaiveDate::from_ymd_opt(year as i32, month as u32, mday as u32) {
            Ok(Self(nd))
        } else {
            bail!("invalid mday {mday} for {year}/{month}")
        }
    }
}

impl From<NaiveDate> for Date {
    fn from(nd: NaiveDate) -> Self {
        Self(nd)
    }
}

// impl Deref for Date {
//     type Target = NaiveDate;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl Date {
    pub fn unixtime(&self) -> i64 {
        unixtime_from_naivedate(self.0)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let y: i32 = self.0.year();
        let m: u32 = self.0.month0() + 1;
        let mday: u32 = self.0.day0() + 1;

        let mut state = serializer.serialize_struct("Date", 3)?;
        state.serialize_field("year", &y)?;
        state.serialize_field("month", &m)?;
        state.serialize_field("mday", &mday)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Default implementation just delegates to `deserialize` impl.
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}

impl TryFrom<&str> for Date {
    type Error = anyhow::Error;

    /// "6/13/2023", "Q1'20", "August 7, 2019"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        (|| -> Result<Self, Self::Error> {
            // "August 7, 2019"
            match NaiveDate::parse_from_str(value, "%B %d, %Y") {
                Ok(v) => {
                    return Ok(v.into())
                }
                Err(_) => ()
            }
            // "6/13/2023"
            let vals: Vec<_> = value.split('/').collect();
            if vals.len() == 3 {
                let month = vals[0];
                let mday = vals[1];
                let year = vals[2];
                Ok(Date::new(
                    year.parse().with_context(|| anyhow!("year {year}"))?,
                    month.parse().with_context(|| anyhow!("month {month}"))?,
                    mday.parse().with_context(|| anyhow!("mday {mday}"))?
                )?)
            } else {
                // "Q1'20"
                let vals: Vec<_> = value.split('\'').collect();
                if vals.len() == 2 {
                    if let [ quarterstr, yearstr ] = &*vals {
                        let year: u16 =
                            match yearstr.len() {
                                2 => {
                                    let y: u16 = yearstr.parse()?;
                                    y + 2000
                                }
                                4 => yearstr.parse()?,
                                _ => bail!("2 or 4 characters representing year expected after '\''")
                            };
                        Ok(match *quarterstr {
                            "Q1" => Date::new(year, 2, 15)?,
                            "Q2" => Date::new(year, 5, 15)?,
                            "Q3" => Date::new(year, 8, 15)?,
                            "Q4" => Date::new(year, 11, 15)?,
                            _ => bail!("expecting Q{{1,2,3,4}} before '\''")
                        }.into())
                    } else {
                        panic!()
                    }
                } else {
                    bail!("need exactly two '/' in US style date or \"Q1\'20\" style format")
                }
            }
        })().with_context(|| anyhow!("invalid date {value:?}"))
    }
}
