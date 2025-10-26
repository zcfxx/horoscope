use std::{collections::HashMap, io, iter::Map};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Month {
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum ZodiacSign {
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
struct Horoscope {
    // Month, day, Zodiac present, Zodiac previous
    months: HashMap<Month, (u8, ZodiacSign, ZodiacSign)>,
}

impl Horoscope {
    fn init(&mut self) {
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

        self.months = init_months;
    }

    fn get(&self, month: &Month, day: u8) -> ZodiacSign {
        let select_month = self.months.get(month);
        let mut select_zodiac_sign = ZodiacSign::Aquarius;

        if day >= select_month.unwrap().0 {
            select_zodiac_sign = select_month.unwrap().1.clone();
        } else {
            select_zodiac_sign = select_month.unwrap().2.clone();
        }

        select_zodiac_sign
    }
}

fn get_horoscope(month: u8, day: u8) -> Option<ZodiacSign> {
    if day == 0 || day > 31 {
        return None;
    }

    match month {
        1 => {
            if day >= 20 {
                Some(ZodiacSign::Aquarius)
            } else {
                Some(ZodiacSign::Capricorn)
            }
        }
        2 => {
            if day > 29 {
                return None;
            }

            if day >= 19 {
                Some(ZodiacSign::Pieces)
            } else {
                Some(ZodiacSign::Aquarius)
            }
        }
        3 => {
            if day >= 21 {
                Some(ZodiacSign::Aries)
            } else {
                Some(ZodiacSign::Pieces)
            }
        }
        4 => {
            if day >= 20 {
                Some(ZodiacSign::Taurus)
            } else {
                Some(ZodiacSign::Aries)
            }
        }
        5 => {
            if day >= 21 {
                Some(ZodiacSign::Gemini)
            } else {
                Some(ZodiacSign::Taurus)
            }
        }
        6 => {
            if day >= 21 {
                Some(ZodiacSign::Cancer)
            } else {
                Some(ZodiacSign::Gemini)
            }
        }
        7 => {
            if day >= 23 {
                Some(ZodiacSign::Leo)
            } else {
                Some(ZodiacSign::Cancer)
            }
        }
        8 => {
            if day >= 23 {
                Some(ZodiacSign::Virgo)
            } else {
                Some(ZodiacSign::Leo)
            }
        }
        9 => {
            if day >= 23 {
                Some(ZodiacSign::Libra)
            } else {
                Some(ZodiacSign::Virgo)
            }
        }
        10 => {
            if day >= 23 {
                Some(ZodiacSign::Scorpio)
            } else {
                Some(ZodiacSign::Libra)
            }
        }
        11 => {
            if day >= 22 {
                Some(ZodiacSign::Sagittarius)
            } else {
                Some(ZodiacSign::Scorpio)
            }
        }
        12 => {
            if day >= 22 {
                Some(ZodiacSign::Capricorn)
            } else {
                Some(ZodiacSign::Sagittarius)
            }
        }

        _ => None,
    }
}

fn main() {
    println!("What is your horoscope?");

    println!("Enter the month: ");
    let mut input_month = String::new();

    io::stdin()
        .read_line(&mut input_month)
        .expect("Failed to read month");

    println!("Enter the day: ");
    let mut input_day = String::new();

    io::stdin()
        .read_line(&mut input_day)
        .expect("Failed to read day");

    let Ok(month) = input_month.trim_end().parse::<u8>() else {
        eprintln!("Invalid 'Month' input!");
        return;
    };

    let Ok(day) = input_day.trim_end().parse::<u8>() else {
        eprintln!("Invalid Day input!");
        return;
    };

    let Some(result) = get_horoscope(month, day) else {
        eprintln!("Horoscope not found!");
        return;
    };

    println!("Horoscope is {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1a() {
        let mut test_horoscope = Horoscope::default();
        test_horoscope.init();

        let result = test_horoscope.get(&Month::January, 20);

        assert_eq!(result, ZodiacSign::Aquarius);

        let result = test_horoscope.get(&Month::January, 19);

        assert_eq!(result, ZodiacSign::Capricorn);
    }

    #[test]
    fn test_1b() {
        let Some(result) = get_horoscope(1, 21) else {
            panic!("Invalid horoscope!");
        };
        assert_eq!(result, ZodiacSign::Aquarius);
    }

    #[test]
    fn test_2() {
        let result = get_horoscope(2, 29);
        assert_eq!(result.unwrap(), ZodiacSign::Pieces);
    }

    #[test]
    fn test_3() {
        let result = get_horoscope(3, 1);
        assert_eq!(result.unwrap(), ZodiacSign::Pieces);
    }

    #[test]
    fn test_feb_greater_29() {
        let result = get_horoscope(2, 30);
        assert_eq!(result, None);
    }

    #[test]
    fn test_4() {
        let result = get_horoscope(11, 23);
        assert_eq!(result.unwrap(), ZodiacSign::Sagittarius);
    }
}
