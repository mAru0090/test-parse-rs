use crate::types::DataValue;
use std::ops::{Add, Div, Mul, Sub};
impl Add for DataValue {
    type Output = DataValue;

    fn add(self, other: DataValue) -> DataValue {
        match (self, other) {
            (DataValue::Int64(a), DataValue::Int64(b)) => DataValue::Int64(a + b),
            (DataValue::Int32(a), DataValue::Int32(b)) => DataValue::Int32(a + b),
            (DataValue::Int16(a), DataValue::Int16(b)) => DataValue::Int16(a + b),
            (DataValue::Int8(a), DataValue::Int8(b)) => DataValue::Int8(a + b),
            (DataValue::String(a), DataValue::String(b)) => DataValue::String(a + &b),
            _ => panic!("Unsupported addition operation for different types"),
        }
    }
}

impl Sub for DataValue {
    type Output = DataValue;

    fn sub(self, other: DataValue) -> DataValue {
        match (self, other) {
            (DataValue::Int64(a), DataValue::Int64(b)) => DataValue::Int64(a - b),
            (DataValue::Int32(a), DataValue::Int32(b)) => DataValue::Int32(a - b),
            (DataValue::Int16(a), DataValue::Int16(b)) => DataValue::Int16(a - b),
            (DataValue::Int8(a), DataValue::Int8(b)) => DataValue::Int8(a - b),
            _ => panic!("Unsupported subtraction operation for different types"),
        }
    }
}

impl Mul for DataValue {
    type Output = DataValue;

    fn mul(self, other: DataValue) -> DataValue {
        match (self, other) {
            (DataValue::Int64(a), DataValue::Int64(b)) => DataValue::Int64(a * b),
            (DataValue::Int32(a), DataValue::Int32(b)) => DataValue::Int32(a * b),
            (DataValue::Int16(a), DataValue::Int16(b)) => DataValue::Int16(a * b),
            (DataValue::Int8(a), DataValue::Int8(b)) => DataValue::Int8(a * b),
            _ => panic!("Unsupported multiplication operation for different types"),
        }
    }
}

impl Div for DataValue {
    type Output = DataValue;

    fn div(self, other: DataValue) -> DataValue {
        match (self, other) {
            (DataValue::Int64(a), DataValue::Int64(b)) => {
                if b == 0 {
                    panic!("Division by zero");
                }
                DataValue::Int64(a / b)
            }
            (DataValue::Int32(a), DataValue::Int32(b)) => {
                if b == 0 {
                    panic!("Division by zero");
                }
                DataValue::Int32(a / b)
            }
            (DataValue::Int16(a), DataValue::Int16(b)) => {
                if b == 0 {
                    panic!("Division by zero");
                }
                DataValue::Int16(a / b)
            }
            (DataValue::Int8(a), DataValue::Int8(b)) => {
                if b == 0 {
                    panic!("Division by zero");
                }
                DataValue::Int8(a / b)
            }
            _ => panic!("Unsupported division operation for different types"),
        }
    }
}
