#[derive(Debug, Clone, PartialOrd)]
pub enum YarnValue {
    STRING(String),
    NUMBER(f64),
    BOOL(bool)
}

impl PartialEq for YarnValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::STRING(l0), Self::STRING(r0)) => l0 == r0,
            (Self::NUMBER(l0), Self::NUMBER(r0)) => (l0 - r0) < 1e-10,
            (Self::BOOL(l0), Self::BOOL(r0)) => l0 == r0,
            _ => false
        }
    }
}

impl YarnValue {
    
    pub fn get_type_as_string(&self) -> &str {
        match self {
            YarnValue::STRING(_) => "STRING",
            YarnValue::NUMBER(_) => "NUMBER",
            YarnValue::BOOL(_) => "BOOL",
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            YarnValue::NUMBER(_) => true,
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            YarnValue::NUMBER(num) => Some(*num),
            _ => None,
        }
    }

    pub fn is_equal(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => Some(YarnValue::BOOL(s1 == s2)),
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 == n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(b2) => Some(YarnValue::BOOL(b1 == b2)),
                }
            },
        }
    }

    pub fn is_not_equal(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => Some(YarnValue::BOOL(s1 != s2)),
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 != n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(b2) => Some(YarnValue::BOOL(b1 != b2)),
                }
            },
        }
    }

    pub fn is_less_than(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 < n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            _ => None
        }
    }

    pub fn is_less_than_eq(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 <= n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            _ => None
        }
    }

    pub fn is_greater_than(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 > n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            _ => None
        }
    }

    pub fn is_greater_than_eq(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(_) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::BOOL(n1 >= n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            _ => None
        }
    }

    pub fn add(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => Some(YarnValue::STRING(format!("{}{}", s1, s2))),
                    YarnValue::NUMBER(n2) => Some(YarnValue::STRING(format!("{}{}", s1, n2))),
                    YarnValue::BOOL(b2) => Some(YarnValue::STRING(format!("{}{}", s1, b2))),
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(s2) => Some(YarnValue::STRING(format!("{}{}", n1, s2))),
                    YarnValue::NUMBER(n2) => Some(YarnValue::NUMBER(n1 + n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(s2) => Some(YarnValue::STRING(format!("{}{}", b1, s2))),
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            },
        }
    }

    pub fn mult(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => None,
                    YarnValue::BOOL(b2) => None,
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::NUMBER(n1 * n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            },
        }
    }

    pub fn sub(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => None,
                    YarnValue::BOOL(b2) => None,
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::NUMBER(n1 - n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            }
        }
    }

    pub fn div(&self, other : &YarnValue) -> Option<YarnValue> {
        match self {
            YarnValue::STRING(s1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => None,
                    YarnValue::BOOL(b2) => None,
                }
            },
            YarnValue::NUMBER(n1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(n2) => Some(YarnValue::NUMBER(n1 / n2)),
                    YarnValue::BOOL(_) => None,
                }
            },
            YarnValue::BOOL(b1) => {
                match other {
                    YarnValue::STRING(s2) => None,
                    YarnValue::NUMBER(_) => None,
                    YarnValue::BOOL(_) => None,
                }
            },
        }
    }
}

impl From<&str> for YarnValue {
    fn from(value : &str) -> Self {
        let number_value = value.parse::<f64>();
        if value == "true" {
            YarnValue::BOOL(true)
        } else if value == "false" {
            YarnValue::BOOL(false)
        } else if number_value.is_ok() {
            YarnValue::NUMBER(number_value.unwrap())
        } else {
            YarnValue::STRING(value.to_string())
        }
    }
}