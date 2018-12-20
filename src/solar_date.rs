use super::{ChineseVariant, SolarYear, SolarMonth, SolarDay, days_in_a_solar_month};

use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;

use chrono::NaiveDate;

/// 西曆年、月、日。
#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct SolarDate {
    pub(crate) solar_year: SolarYear,
    pub(crate) solar_month: SolarMonth,
    pub(crate) solar_day: SolarDay,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SolarDateParseError {
    /// 超出支援的範圍。
    OutOfRange,
    /// 錯誤的西曆年。
    IncorrectYear,
    /// 錯誤的西曆月。
    IncorrectMonth,
    /// 錯誤的西曆日。
    IncorrectDay,
}

impl SolarDate {
    /// 將無時區的 `Chrono` 日期實體轉成 `SolarDate` 實體。
    pub fn from_naive_date(naive_date: NaiveDate) -> Result<SolarDate, SolarDateParseError> {
        let year = naive_date.year();

        if year < 0 || year > u16::max_value() as i32 {
            Err(SolarDateParseError::OutOfRange)
        } else {
            let solar_year = SolarYear::from_u16(year as u16);
            let solar_month = SolarMonth::from_u8(naive_date.month() as u8).unwrap();
            let solar_day = SolarDay::from_u8(naive_date.day() as u8).unwrap();

            Ok(SolarDate {
                solar_year,
                solar_month,
                solar_day,
            })
        }
    }

    /// 將有時區的 `Chrono` 日期實體，依UTC時區轉成 `SolarDate` 實體。
    pub fn from_date<Tz: TimeZone>(date: Date<Tz>) -> Result<SolarDate, SolarDateParseError> {
        let naive_date = date.naive_utc();

        Self::from_naive_date(naive_date)
    }

    /// 將 `SolarDate` 實體轉成無時區的 `Chrono` 日期實體。
    pub fn to_naive_date(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.solar_year.to_u16() as i32, self.solar_month.to_u8() as u32, self.solar_day.to_u8() as u32)
    }

    /// 將 `SolarDate` 實體轉成UTC時區的 `Chrono` 日期實體。
    pub fn to_date_utc(&self) -> Date<Utc> {
        let naive_date = self.to_naive_date();

        Date::from_utc(naive_date, Utc)
    }

    /// 利用西曆的年、月、日來產生 `SolarDate` 實體。
    pub fn from_solar_year_month_day<Y: Into<SolarYear>>(solar_year: Y, solar_month: SolarMonth, solar_day: SolarDay) -> Result<SolarDate, SolarDateParseError> {
        let solar_year = solar_year.into();

        let days = days_in_a_solar_month(solar_year, solar_month);

        let day = solar_day.to_u8();

        if day > days {
            Err(SolarDateParseError::IncorrectDay)
        } else {
            Ok(SolarDate {
                solar_year,
                solar_month,
                solar_day,
            })
        }
    }

    /// 利用西曆的年、月、日來產生 `SolarDate` 實體。
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Result<SolarDate, SolarDateParseError> {
        let solar_year = SolarYear::from_u16(year);

        let solar_month = match SolarMonth::from_u8(month) {
            Some(solar_month) => solar_month,
            None => return Err(SolarDateParseError::IncorrectMonth)
        };

        let solar_day = match SolarDay::from_u8(day) {
            Some(solar_day) => solar_day,
            None => return Err(SolarDateParseError::IncorrectMonth)
        };


        Self::from_solar_year_month_day(solar_year, solar_month, solar_day)
    }

    /// 以目前的年、月、日來產生 `SolarDate` 實體。
    pub fn now() -> Result<SolarDate, SolarDateParseError> {
        Self::from_date(Utc::now().date())
    }

    pub fn from_str<S: AsRef<str>>(s: S) -> Result<SolarDate, SolarDateParseError> {
        let s = s.as_ref();

        let year_index = {
            match s.find("年") {
                Some(index) => index,
                None => match s.find("　") {
                    Some(index) => index,
                    None => return Err(SolarDateParseError::IncorrectYear)
                }
            }
        };

        let year_str = s[..year_index].trim();

        let solar_year = match SolarYear::from_str(year_str) {
            Some(solar_year) => solar_year,
            None => return Err(SolarDateParseError::IncorrectYear)
        };

        let s = &s[year_index + 3..];

        let month_index = {
            match s.find("月") {
                Some(index) => index,
                None => match s.find("　") {
                    Some(index) => index,
                    None => return Err(SolarDateParseError::IncorrectMonth)
                }
            }
        };

        let month_str = s[..month_index + 3].trim();

        let solar_month = match SolarMonth::from_str(month_str) {
            Some(solar_month) => solar_month,
            None => return Err(SolarDateParseError::IncorrectMonth)
        };

        let mut day_str = s[month_index + 3..].trim();

        if day_str.ends_with("日") {
            day_str = &day_str[..day_str.len() - 3];
        }

        let solar_day = match SolarDay::from_str(day_str) {
            Some(solar_day) => solar_day,
            None => return Err(SolarDateParseError::IncorrectDay)
        };

        Self::from_solar_year_month_day(solar_year, solar_month, solar_day)
    }

    /// 取得 `SolarDate` 實體所代表的中文西曆年、月、日字串。
    pub fn to_chinese_string(&self) -> String {
        let mut s = String::with_capacity(36);

        self.solar_year.write_to_chinese_string(&mut s);
        s.push_str("年");
        s.push_str(self.solar_month.to_str());
        s.push_str(self.solar_day.to_str());
        s.push_str("日");

        s
    }

    /// 取得 `SolarDate` 實體所代表的西曆年、月、日字串(格式：yyyy-mm-dd)。
    pub fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.solar_year.to_u16(), self.solar_month.to_u8(), self.solar_day.to_u8())
    }

    /// 取得西曆年。
    pub fn get_solar_year(&self) -> SolarYear {
        self.solar_year
    }

    /// 取得西曆月。
    pub fn get_solar_month(&self) -> SolarMonth {
        self.solar_month
    }

    /// 取得西曆日。
    pub fn get_solar_day(&self) -> SolarDay {
        self.solar_day
    }
}

impl Display for SolarDate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_chinese_string())
    }
}