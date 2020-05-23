use std::convert::TryFrom;

/// 6 digit number
#[derive(Clone, PartialEq, PartialOrd)]
struct Number([u8; 6]);

impl TryFrom<u32> for Number {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let d1 = match u8::try_from(value / 100_000) {
            Ok(0) => return Err("Number is less than 6 digits!"),
            Ok(10..=255) | Err(_) => return Err("Number can not be more than 6 digits!"),
            Ok(d1) => d1,
        };

        let d2 = ((value / 10_000) % 10) as u8;
        let d3 = ((value / 1_000) % 10) as u8;
        let d4 = ((value / 100) % 10) as u8;
        let d5 = ((value / 10) % 10) as u8;
        let d6 = (value % 10) as u8;

        Ok(Self([d1, d2, d3, d4, d5, d6]))
    }
}

impl Number {
    fn increment(&mut self) {
        for i in (0..6).rev() {
            self.0[i] += 1;
            if self.0[i] != 10 {
                return;
            }

            self.0[i] = 0;
        }
    }
}

/// Given a range, determines how many numbers follow a particular pattern
pub struct PatternCounter {
    min_number: Number,
    max_number: Number,
}
impl TryFrom<(u32, u32)> for PatternCounter {
    type Error = &'static str;

    fn try_from((start, end): (u32, u32)) -> Result<Self, Self::Error> {
        let min_number = Number::try_from(start)?;
        let max_number = Number::try_from(end)?;

        Ok(Self {
            min_number,
            max_number,
        })
    }
}

impl PatternCounter {
    fn is_increasing(number: &Number) -> bool {
        (0..5).all(|i| number.0[i] <= number.0[i + 1])
    }

    fn has_repeating(number: &Number) -> bool {
        (0..5).any(|i| number.0[i] == number.0[i + 1])
    }

    fn repeats_twice(number: &Number) -> bool {
        (0..5).any(|i| match i {
            0 => number.0[0] == number.0[1] && number.0[0] != number.0[2],
            4 => number.0[4] == number.0[5] && number.0[4] != number.0[3],
            _ => {
                number.0[i] != number.0[i - 1]
                    && number.0[i] == number.0[i + 1]
                    && number.0[i] != number.0[i + 2]
            }
        })
    }
    
    /// Main Pattern, takes in a function to check second pattern
    /// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    fn pattern<F>(&self, check_2: F) -> u16
    where
        F: Fn(&Number) -> bool,
    {
        let mut count = 0;
        let mut number = self.min_number.clone();
        loop {
            if Self::is_increasing(&number) {
                if check_2(&number) {
                    count += 1;
                }
            }

            if number >= self.max_number {
                break;
            }
            number.increment();
        }
        count
    }

    /// Pattern 1:
    /// Follows main pattern
    /// Additional Rule:
    /// Two adjacent digits are the same (like 22 in 122345).
    pub fn pattern_1(&self) -> u16 {
        self.pattern(Self::has_repeating)
    }

    /// Pattern 2:
    /// Follows main pattern
    /// Additional Rule:
    /// Two adjacent digits are the same (like 22 in 122345).
    /// But can not be part of a larger group, has to be extactly 2
    /// i.e. 123444 is not valid
    pub fn pattern_2(&self) -> u16 {
        self.pattern(Self::repeats_twice)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_increment() {
        let mut number = Number([1, 9, 9, 9, 9, 9]);
        let expected = [2, 0, 0, 0, 0, 0];
        number.increment();

        assert_eq!(number.0, expected);
    }

    #[test]
    fn test_pattern_1() {
        let number = Number([1, 1, 1, 1, 1, 1]);
        assert!(PatternCounter::is_increasing(&number));
        assert!(PatternCounter::has_repeating(&number));

        let number = Number([2, 2, 3, 4, 5, 0]);
        assert_eq!(PatternCounter::is_increasing(&number), false);
        assert!(PatternCounter::has_repeating(&number));

        let number = Number([1, 2, 3, 7, 8, 9]);
        assert!(PatternCounter::is_increasing(&number));
        assert_eq!(PatternCounter::has_repeating(&number), false);
    }

    #[test]
    fn test_pattern_2() {
        let number = Number([1, 1, 2, 2, 3, 3]);
        assert!(PatternCounter::is_increasing(&number));
        assert!(PatternCounter::repeats_twice(&number));

        let number = Number([1, 2, 3, 4, 4, 4]);
        assert!(PatternCounter::is_increasing(&number));
        assert_eq!(PatternCounter::repeats_twice(&number), false);

        let number = Number([1, 1, 1, 1, 2, 2]);
        assert!(PatternCounter::is_increasing(&number));
        assert!(PatternCounter::repeats_twice(&number));
    }
}
