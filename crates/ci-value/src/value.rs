use std::fmt;

/// The items of a value, in row-major order.
#[derive(Clone, Debug, PartialEq)]
pub enum Items {
    Num(Vec<f64>),
    Char(Vec<char>),
}

/// An array: its shape (empty for a scalar) and its items.
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub shape: Vec<usize>,
    pub items: Items,
}

impl Value {
    pub fn nums(shape: Vec<usize>, items: Vec<f64>) -> Self {
        Value {
            shape,
            items: Items::Num(items),
        }
    }

    pub fn chars(shape: Vec<usize>, text: &str) -> Self {
        Value {
            shape,
            items: Items::Char(text.chars().collect()),
        }
    }

    pub fn is_char(&self) -> bool {
        matches!(self.items, Items::Char(_))
    }

    /// True when the number of items is what the shape says.
    pub fn is_consistent(&self) -> bool {
        let n = match &self.items {
            Items::Num(v) => v.len(),
            Items::Char(v) => v.len(),
        };
        n == self.shape.iter().product::<usize>()
    }
}

/// The canonical form: `num [2 3] 1 2 3 4 5 6`, `char [5] "EDCBA"`,
/// `num [] 10` for a scalar.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shape: Vec<String> = self.shape.iter().map(|n| n.to_string()).collect();
        let shape = shape.join(" ");
        match &self.items {
            Items::Num(v) => {
                let items: Vec<String> = v.iter().map(|x| number(*x)).collect();
                write!(f, "num [{shape}] {}", items.join(" "))
            }
            Items::Char(v) => write!(f, "char [{shape}] {:?}", v.iter().collect::<String>()),
        }
    }
}

/// A whole number prints without a decimal point; others as Rust prints them.
fn number(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x}")
    }
}
