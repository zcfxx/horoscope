use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug)]
pub struct MonthError;

impl FromStr for Month {
    type Err = MonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("value {}", s);

        match s.trim_end() {
            "1" => Ok(Month::January),
            "2" => Ok(Month::February),
            "3" => Ok(Month::March),
            "4" => Ok(Month::April),
            "5" => Ok(Month::May),
            "6" => Ok(Month::June),
            "7" => Ok(Month::July),
            "8" => Ok(Month::August),
            "9" => Ok(Month::September),
            "10" => Ok(Month::October),
            "11" => Ok(Month::November),
            "12" => Ok(Month::December),
            _ => Err(MonthError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ZodiacSign {
    Aquarius,
    Pieces,
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
}

#[derive(Default)]
pub struct Horoscope {
    // Month, day, Zodiac present, Zodiac previous
    months: HashMap<Month, (u8, ZodiacSign, ZodiacSign)>,
}

impl Horoscope {
    // initialize HashMap
    pub fn init(&mut self) {
        let mut init_months: HashMap<Month, (u8, ZodiacSign, ZodiacSign)> = HashMap::new();
        init_months.insert(
            Month::January,
            (20, ZodiacSign::Aquarius, ZodiacSign::Capricorn),
        );
        init_months.insert(
            Month::February,
            (19, ZodiacSign::Pieces, ZodiacSign::Aquarius),
        );
        init_months.insert(Month::March, (21, ZodiacSign::Aries, ZodiacSign::Pieces));
        init_months.insert(Month::April, (20, ZodiacSign::Taurus, ZodiacSign::Aries));
        init_months.insert(Month::May, (21, ZodiacSign::Gemini, ZodiacSign::Taurus));
        init_months.insert(Month::June, (21, ZodiacSign::Cancer, ZodiacSign::Gemini));
        init_months.insert(Month::July, (23, ZodiacSign::Leo, ZodiacSign::Cancer));
        init_months.insert(Month::August, (23, ZodiacSign::Virgo, ZodiacSign::Leo));
        init_months.insert(Month::September, (23, ZodiacSign::Libra, ZodiacSign::Virgo));
        init_months.insert(Month::October, (23, ZodiacSign::Scorpio, ZodiacSign::Libra));
        init_months.insert(
            Month::November,
            (22, ZodiacSign::Sagittarius, ZodiacSign::Scorpio),
        );
        init_months.insert(
            Month::December,
            (22, ZodiacSign::Capricorn, ZodiacSign::Sagittarius),
        );

        self.months = init_months;
    }

    // return ZodiacSign from Month and Day
    pub fn get(&self, month: &Month, day: u8) -> Option<ZodiacSign> {
        let select_month = self.months.get(month);

        if day >= select_month.unwrap().0 {
            Some(select_month.unwrap().1.clone())
        } else {
            Some(select_month.unwrap().2.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut test_horoscope = Horoscope::default();
        test_horoscope.init();

        let Some(result) = test_horoscope.get(&Month::January, 20) else {
            panic!("Failed to retrieve horoscope!");
        };

        assert_eq!(result, ZodiacSign::Aquarius);

        let Some(result) = test_horoscope.get(&Month::January, 19) else {
            panic!("Failed to retrieve horoscope!");
        };

        assert_eq!(result, ZodiacSign::Capricorn);
    }

    #[test]
    fn test_parse() {
        let Ok(month) = "1".parse::<Month>() else {
            panic!("Failed to parse!");
        };

        assert_eq!(month, Month::January);
    }

    #[test]
    #[should_panic]
    fn test_parse_panic() {
        let Ok(_) = "13".parse::<Month>() else {
            panic!("Failed to parse!");
        };
    }
}
