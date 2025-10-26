mod lib;

use std::io;
use std::io::Write;

use crate::lib::horoscope::Horoscope;
use crate::lib::horoscope::Month;
use crate::lib::horoscope::ZodiacSign;

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

    print!("Enter the month: ");
    io::stdout().flush().expect("Could not flush!");

    let mut input_month = String::new();
    io::stdin()
        .read_line(&mut input_month)
        .expect("Failed to read month");

    print!("Enter the day: ");
    io::stdout().flush().expect("Could not flush!");

    let mut input_day = String::new();
    io::stdin()
        .read_line(&mut input_day)
        .expect("Failed to read day");

    // let Ok(month) = input_month.trim_end().parse::<u8>() else {
    //     eprintln!("Invalid 'Month' input!");
    //     return;
    // };

    let Ok(day) = input_day.trim_end().parse::<u8>() else {
        eprintln!("Invalid Day input!");
        return;
    };

    // let Some(result) = get_horoscope(month, day) else {
    //     eprintln!("Horoscope not found!");
    //     return;
    // };

    // println!("1. Horoscope is {:?}", result);

    let mut horoscope = Horoscope::default();
    horoscope.init();

    let Ok(month_value) = input_month.parse::<Month>() else {
        panic!("Failed to parse month!");
    };

    let Some(result) = horoscope.get(&month_value, day) else {
        eprintln!("Horoscope not found!");
        return;
    };

    println!("\nHoroscope is {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

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
