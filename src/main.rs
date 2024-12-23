use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};
use flux_rs::*;

fn main() {
    let num = Rational::new(1, 7 * 7 * 7);
    let num2 = Rational::new(5, 8);
    let num3 = num - num2;
    println!("{}", num.get_decimal());
    println!("{}", num);
    println!("{}{}/{}", if num3.sign { "-" } else { "" }, num3.get_numerator(), num3.get_denominator());
    println!("{}", num3);
}

// An `assert` function, whose precondition expects only `true`
#[sig(fn(bool[true]))]
pub fn assert(_: bool) {}

/// A struct that represents a rational number.
#[derive(Clone, Copy)]
struct Rational {
    /// The sign of the rational number.
    /// False is positive, true is negative.
    sign: bool,
    /// The numerator of the rational number.
    numerator: u32,
    /// The denominator of the rational number.
    denominator: u32,
}

impl Rational {
    /// Builds a rational number from a numerator and denominator.
    #[sig(fn(numerator: i32, denominator: i32{denominator != 0}) -> Rational)]
    pub fn new(numerator: i32, denominator: i32) -> Rational {
        let mut rational = Rational {
            sign: numerator.signum() * denominator.signum() == -1,
            numerator: numerator.abs() as u32,
            denominator: denominator.abs() as u32,
        };
        rational.reduce();
        rational
    }

    /// Returns an approximation to the rational number as f64.
    pub fn get_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64 * self.signum() as f64
    }

    /// Returns the sign of the rational number.
    /// Returns 1 if positive, 0 if zero and -1 if negative.
    #[sig(fn(&Rational) -> i32{value: -1 <= value && value <= 1})]
    pub fn signum(&self) -> i32 {
        if self.numerator == 0 {
            return 0;
        }
        if self.sign { -1 } else { 1 }
    }

    /// Returns the numerator of the rational number.
    #[sig(fn(&Rational) -> u32)]
    pub fn get_numerator(&self) -> u32 {
        self.numerator
    }

    /// Returns the denominator of the rational number.
    #[sig(fn(&Rational) -> u32{value: value != 0})]
    #[trusted]
    pub fn get_denominator(&self) -> u32 {
        self.denominator
    }

    /// Returns a decimal iterator over the rational number.
    /// Can be used to get arbitrary precision.
    fn get_decimal_iterator(&self) -> Decimal {
        Decimal::build(self.numerator, self.get_denominator())
    }

    /// Reduces the fraction to a canonical form.
    /// Should be executed whenever the rational number might go out of its canonical form.
    /// E.g. when constructing the rational or involved in calculations.
    fn reduce(&mut self) {
        let mut remainder;
        let mut a = self.numerator;
        let mut b = self.get_denominator();

        loop {
            remainder = a % b;
            a = b;
            b = remainder;
            if b == 0 {
                break;
            }
        }

        self.numerator /= a;
        self.denominator /= a;
    }

    /// Builds the reciprocal of the rational number.
    /// Returns an error when rational number is zero, the reciprocal otherwise.
    #[sig(fn(&Rational) -> Result<Rational, String>)]
    fn reciprocal(&self) -> Result<Rational, String> {
        let numerator = self.get_numerator() as i32;
        if numerator == 0 {
            return Err("Denominator must not be zero.".to_string());
        }
        Ok(Self::new(
            self.get_denominator() as i32 * self.signum(),
            numerator
        ))
    }
}

impl Display for Rational {
    /// Displays the rational number as (repeating) decimal.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = self.get_decimal_iterator().get_repeating();
        write!(
            f,
            "{}{}.{}\x1b[53m{}\x1b[0m", 
            if self.sign { "-" } else { "" },
            digits.0[0], 
            digits.0[1..].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""), 
            digits.1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
        )
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        let denominator = (self.get_denominator() * rhs.get_denominator()) as i32;
        assert!(denominator != 0);
        let mut res = Rational::new(
            (self.numerator * rhs.get_denominator()) as i32 * self.signum() 
            + (rhs.numerator * self.get_denominator()) as i32 * rhs.signum(),
            denominator
        );
        res.reduce();
        res
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        let denominator = (self.get_denominator() * rhs.get_denominator()) as i32;
        assert!(denominator != 0);
        let mut res = Rational::new(
            (self.numerator * rhs.numerator) as i32 * self.signum() * rhs.signum(),
            denominator
        );
        res.reduce();
        res
    }
}

impl Div for Rational {
    type Output = Result<Rational, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match rhs.reciprocal() {
            Ok(reciprocal_rhs) => Ok(self * reciprocal_rhs),
            Err(e) => Err(e)
        }
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        let denominator = self.get_denominator() as i32;
        assert!(denominator != 0);
        Rational::new(
            self.numerator as i32 * self.signum() * -1,
            denominator
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
    /// Returns an error when denominator is zero, the decimal otherwise.
    #[sig(fn(u32, u32{v: v > 0}) -> Decimal)]
    fn build(numerator: u32, denominator: u32) -> Decimal {
        Decimal {
            numerator, 
            denominator
        }
    }

    #[sig(fn(&Decimal) -> u32{value: value != 0})]
    #[trusted]
    fn get_denominator(&self) -> u32 {
        self.denominator
    }

    /// Returns the digits of the rational number.
    /// The first value of the tuple consists of:
    /// The first value which is the whole part of the number.
    /// The remaining values are the digits of the non-repeating part of the fraction.
    /// The second value is a slice of the repeating digits.
    #[sig(fn(&mut Decimal) -> (Box<[u32]{v: v > 0}>, Box<[u32]>))]
    #[trusted]
    fn get_repeating(&mut self) -> (Box<[u32]>, Box<[u32]>) {
        let mut remainders = vec![];
        let mut digits = vec![];
        let mut remainder = self.numerator % self.get_denominator();

        while 
            self.denominator != 0 && 
            self.numerator != 0 && 
            remainder != 0 &&
            !remainders.contains(&remainder) {
            
            digits.push(self.numerator / self.get_denominator());
            remainders.push(remainder);
            self.numerator = remainder * 10;
            remainder = self.numerator % self.get_denominator();
        }
        digits.push(self.numerator / self.get_denominator());
        
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
    #[trusted]
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

#[extern_spec]
impl i32 {
    #[sig(fn(i32) -> i32{value: -1 <= value && value <= 1})]
    fn signum(self) -> Self;
}