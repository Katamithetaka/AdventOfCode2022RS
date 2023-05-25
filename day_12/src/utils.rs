use std::{cmp::Ordering, path::Display};


#[derive(Clone, Debug)]
pub enum PacketValue {
    List(Vec<PacketValue>),
    Number(i64)
}

impl ToString for PacketValue {
    fn to_string(&self) -> String {
        match self {
            Self::List(v) => {
                    String::from("[") + &v.iter().fold(String::new(), |a, b| {
                    a + &b.to_string() + ", "
                }) + "]"
            }
            Self::Number(v) => {
                 v.to_string() 
            }
        }
    }
}


impl From<serde_json::Value> for PacketValue {
    fn from(value: serde_json::Value) -> Self {
        if value.is_i64() {
            Self::Number(value.as_i64().unwrap())
        }
        else if value.is_array() {
            let mut v = vec![];
            for value in value.as_array().unwrap() {
                v.push(Self::from(value.clone()));
            }
            Self::List(v)
        }
        else {
            panic!("invalid PacketValue");
        }
    }
}



impl From<&str> for PacketValue {


    fn from(s: &str) -> Self {
        let v = serde_json::from_str::<serde_json::Value>(s).unwrap();
        Self::from(v)
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PacketValue::List(left) => {
                match other {

                    Self::List(right) => {
                            if left.len() != right.len() {
                                return false
                            }
                        else {
                            for i in 0..left.len() {
                                if !left[i].eq(&right[i]) {
                                    return false
                                }
                            }
                            return true;
                        }
                    },
                    Self::Number(right) => {
                        self.eq(&Self::List(vec![Self::Number(*right)]))
                    }
                    
                } 

            },
            PacketValue::Number(left) => {
                match other {
                    Self::List(_) => {
                        Self::List(vec![Self::Number(*left)]).eq(other)
                    }
                    Self::Number(right) => {
                        left.eq(right)
                    }
                }
            },
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            PacketValue::List(left) => {
                match other {
                    PacketValue::List(right) => {
                        for i in 0..left.len() {
                            if i >= right.len() {
                                return Some(Ordering::Greater);
                            }

                            let Some(result) = left[i].partial_cmp(&right[i]) else {
                                return None;
                            };
                            
                            if result == Ordering::Less {
                                return Some(Ordering::Less);
                            }
                            else if result == Ordering::Greater {
                                return Some(Ordering::Greater);
                            }
                        }

                        return Some(left.len().cmp(&right.len()));
                    },
                    PacketValue::Number(right) => {
                        self.partial_cmp(&Self::List(vec![Self::Number(*right)]))
                    },
                }
            },
            PacketValue::Number(left) => {
                match other {
                    PacketValue::Number(right) => {
                        Some(left.cmp(right))
                    },
                    PacketValue::List(_) => {
                        Self::List(vec![Self::Number(*left)]).partial_cmp(other)
                    }
                }
            },
        }
    }
}