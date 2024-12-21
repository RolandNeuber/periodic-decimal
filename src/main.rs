use std::fmt::Display;

fn main() {
    let num = Rational::build(23, 99).unwrap();
    println!("{}", num.get_decimal());
    println!("{}", num);
}

/// A struct that represents a rational number.
struct Rational {
    /// The numerator of the rational number.
    numerator: u32,
    /// The denominator of the rational number.
    denominator: u32,
}

impl Rational {
    /// Builds a rational number from a numerator and denominator.
    /// Returns an Error when denominator is zero, the rational number otherwise.
    pub fn build(numerator: u32, denominator: u32) -> Result<Rational, String> {
        if denominator == 0 {
            return Err("Denominator must not be zero.".to_string())
        }
        Ok(Rational {
            numerator,
            denominator
        })
    }

    /// Returns an approximation to the rational number as f64.
    pub fn get_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    /// Returns a decimal iterator over the rational number.
    /// Can be used to get arbitrary precision.
    fn get_decimal_iterator(&self) -> Decimal {
        Decimal::build(self.numerator, self.denominator).unwrap()
    }

    /// Reduces the fraction to a canonical form.
    /// Should be executed whenever the rational number might go out of its canonical form.
    /// E.g. when constructing the rational or involved in calculations.
    fn reduce(&mut self) {

    }
}

impl Display for Rational {
    /// Displays the rational number as (repeating) decimal.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = self.get_decimal_iterator().get_repeating();
        write!(
            f,
            "{}.{}\x1b[53m{}\x1b[0m", 
            digits.0[0], 
            digits.0[1..].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""), 
            digits.1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
        )
    }
}

/// A struct that represents the decimal places of a rational number.
/// Is consumed by the implemented iterator.
struct Decimal {
    numerator: u32,
    denominator: u32,
}

impl Decimal {
    /// Builds a decimal struct from a numerator and denominator.
    /// Returns an Error when denominator is zero, the decimal otherwise.
    fn build(numerator: u32, denominator: u32) -> Result<Decimal, String> {
        if denominator == 0 {
            return Err("Denominator must not be zero.".to_string())
        }
        Ok(Decimal {
            numerator, 
            denominator
        })
    }

    /// Returns the digits of the rational number.
    /// The first value of the tuple consists of:
    /// The first value which is the whole part of the number.
    /// The remaining values are the digits of the non-repeating part of the fraction.
    /// The second value is a slice of the repeating digits.
    fn get_repeating(&mut self) -> (Box<[u32]>, Box<[u32]>) {
        let mut remainders = vec![];
        let mut digits = vec![];
        let mut remainder = self.numerator % self.denominator;

        while 
            self.denominator != 0 && 
            self.numerator != 0 && 
            remainder != 0 &&
            !remainders.contains(&remainder) {
            
            digits.push(self.numerator / self.denominator);
            remainders.push(remainder);
            self.numerator = remainder * 10;
            remainder = self.numerator % self.denominator;
        }
        digits.push(self.numerator / self.denominator);
        
        let index = remainders.iter().position(|&rem| rem == remainder);

        match index {
            Some(index) => {
                (
                    digits[..=index].to_vec().into_boxed_slice(), 
                    digits[index+1..].to_vec().into_boxed_slice()
                )
            }
            None => {
                (
                    digits.into_boxed_slice(),
                    vec![].into_boxed_slice()
                )
            }
        }        
        
    }
}

impl Iterator for Decimal {
    type Item = u32;

    /// Returns the next digit of the decimal number.
    fn next(&mut self) -> Option<Self::Item> {
        let mut res = 0;
        while self.numerator > self.denominator { 
            res += 1;
            self.numerator -= self.denominator;
        }
        self.numerator *= 10;
        Some(res)
    }
}

