use std::fmt::Display;

fn main() {
    let num = Rational::build(10, 3).unwrap();
    println!("{}", num.get_decimal());

    let mut decimal = num.get_decimal_iterator();
    let mut i = 20;
    let mut res = String::new();
    res.push_str(decimal.next().unwrap().to_string().as_str());
    res.push_str(".");
    for digit in decimal {
        res.push(digit.to_string().chars().nth(0).unwrap());

        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("{}", res);

    println!("{}", num);
}

struct Rational {
    numerator: u32,
    denominator: u32,
}

impl Rational {
    pub fn build(numerator: u32, denominator: u32) -> Result<Rational, String> {
        if denominator == 0 {
            return Err("Denominator must not be zero.".to_string())
        }
        Ok(Rational {
            numerator,
            denominator
        })
    }

    pub fn get_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn get_decimal_iterator(&self) -> Decimal{
        Decimal::build(self.numerator, self.denominator).unwrap()
    }
}

impl Display for Rational {
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

#[derive(Clone, Copy)]
struct Decimal {
    numerator: u32,
    denominator: u32,
}

impl Decimal {
    fn build(numerator: u32, denominator: u32) -> Result<Decimal, String> {
        if denominator == 0 {
            return Err("Denominator must not be zero.".to_string())
        }
        Ok(Decimal {
            numerator, 
            denominator
        })
    }

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

