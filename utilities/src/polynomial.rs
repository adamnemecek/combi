use std::fmt;

mod unimode;

#[derive(Clone, Debug)]
pub struct Polynomial {
    coefs: Vec<i64>,
    var_name: String,
}

pub enum Modality {
    Unimodal(f64),
    Zero,
    Constant,
    Nonmodal,
    Multimodal
}

impl Polynomial {
    pub fn new() -> Self {
        Self { coefs: vec![], var_name: "p".to_owned() }
    }

    pub fn of_vec(coefs: &Vec<i64>) -> Self {
        let mut new_coefs: Vec<i64> = vec![];
        for x in coefs {
            new_coefs.push(*x);
        }
        Self { coefs: new_coefs, var_name: "p".to_owned() }
    }

    pub fn monomial(coef: i64, power: usize) -> Self {
        let mut coefs = vec![0; power];
        coefs.push(coef);
        Self { coefs, var_name: "p".to_owned() }
    }

    pub fn pow(&self, exp: usize) -> Self {
        if exp == 0 {
            Self::of_vec(&vec![1])
        } else if exp == 1 {
            self.to_owned()
        } else {
            self.to_owned().mul(&self.pow(exp - 1))
        }
    }

    fn add_vec_inplace(coefs: &mut Vec<i64>, rhs: &Self) {
        for (pos, y) in rhs.coefs.iter().enumerate() {
            if pos >= coefs.len() {
                coefs.push(*y);
            } else {
                coefs[pos] += *y;
            }
        }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let mut coefs: Vec<i64> = vec![];
        for x in self.coefs.iter() {
            coefs.push(*x);
        }
        Self::add_vec_inplace(&mut coefs, rhs);
        Self::of_vec(&coefs)
    }

    pub fn add_inplace(&mut self, rhs: &Self) {
        Self::add_vec_inplace(&mut self.coefs, rhs)
    }

    fn sub_vec_inplace(coefs: &mut Vec<i64>, rhs: &Self) {
        for (pos, y) in rhs.coefs.iter().enumerate() {
            if pos >= coefs.len() {
                coefs.push(-*y);
            } else {
                coefs[pos] -= *y;
            }
        }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        let mut coefs: Vec<i64> = vec![];
        for x in self.coefs.iter() {
            coefs.push(*x);
        }
        Self::sub_vec_inplace(&mut coefs, rhs);
        Self::of_vec(&coefs)
    }

    pub fn sub_inplace(&mut self, rhs: &Self) {
        Self::sub_vec_inplace(&mut self.coefs, rhs)
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut coefs: Vec<i64> = vec![];
        for (i, xi) in self.coefs.iter().enumerate() {
            for (j, yj) in rhs.coefs.iter().enumerate() {
                if i + j >= coefs.len() {
                    coefs.push(xi * yj);
                } else {
                    coefs[i + j] += xi * yj;
                }
            }
        }
        Self::of_vec(&coefs)
    }

    pub fn apply(&self, g: &Self) -> Self {
        let mut out = Self::new();
        for (i, xi) in self.coefs.iter().enumerate() {
            // Highly efficient, as always
            out.add_inplace(&g.pow(i).mul(&Self::of_vec(&vec![*xi])));
        }
        out
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut out = 0.0;
        for (i, xi) in self.coefs.iter().enumerate() {
            out += x.powf(i as f64) * (*xi as f64);
        }
        out
    }

    pub fn differentiate(&self) -> Self {
        let mut coefs: Vec<i64> = vec![];
        for (i, xi) in self.coefs.iter().enumerate() {
            if i > 0 {
                coefs.push(*xi * (i as i64));
            }
        }
        Self::of_vec(&coefs)
    }

    pub fn is_zero(&self) -> bool {
        self.coefs.iter().all(|x| *x == 0)
    }

    pub fn find_prob_unimode(&self) -> Modality {
        unimode::find_unimode(&self, 0.0, 1.0)
    }

    pub fn with_var_name(&self, var_name: &str) -> Self {
        Self { coefs: self.coefs.to_owned(), var_name: var_name.to_owned() }
    }
}

impl Modality {
    pub fn four_letter_code(&self) -> &str {
        use Modality::*;
        match *self {
            Unimodal(_mode) => " :) ",
            Zero => "zero",
            Constant => "cons",
            Nonmodal => "none",
            Multimodal => "mult",
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coefs.len() == 0 {
            write!(f, "0")
        } else {
            let pars: Vec<String> = self.coefs
                .iter()
                .enumerate()
                .map(|(exp, c)| format!("{}{}^{}", c, self.var_name, exp)).collect();
            write!(f, "{}", pars.join(" + "))
        }
    }
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Modality::*;
        let sta;
        let text = match *self {
            Unimodal(mode) => {
                sta = format!("Unimodal({})", mode);
                &sta
            }
            Zero => "Identically zero",
            Constant => "Constant",
            Nonmodal => "Without extrema",
            Multimodal => "Multiple extrema",
        };
        write!(f, "{}", text)
    }
}