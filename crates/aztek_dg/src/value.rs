//! Plug values. A compound plug's value lists its children in order; an array plug's value lists its
//! elements by logical index.

use crate::data::{MatrixData, StringData, TypedData};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Value {
    /// No value: message attributes, and typed attributes holding no data.
    None,
    Bool(bool),
    /// Every integer numeric type, enums included.
    Int(i64),
    /// Float and double numeric types, and unit attributes in internal units.
    Double(f64),
    /// A numeric attribute with two to four components and no child attributes.
    Tuple(Vec<f64>),
    /// Row-major 4x4.
    Matrix(Box<[f64; 16]>),
    Data(Arc<dyn TypedData>),
    Compound(Vec<Value>),
    Array(Vec<(u32, Value)>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (None, None) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Double(a), Double(b)) => a == b,
            (Tuple(a), Tuple(b)) => a == b,
            (Matrix(a), Matrix(b)) => a == b,
            (Data(a), Data(b)) => Arc::ptr_eq(a, b) || a.eq_data(b.as_ref()),
            (Compound(a), Compound(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    pub fn string(s: impl Into<String>) -> Value {
        Value::Data(Arc::new(StringData(s.into())))
    }

    pub fn data<T: TypedData>(d: T) -> Value {
        Value::Data(Arc::new(d))
    }

    pub fn matrix(m: [f64; 16]) -> Value {
        Value::Matrix(Box::new(m))
    }

    /// Booleans, integers and doubles as a double.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Int(i) => Some(*i as f64),
            Value::Double(d) => Some(*d),
            _ => None,
        }
    }

    /// Booleans, integers and doubles as an integer; doubles truncate toward zero.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Bool(b) => Some(*b as i64),
            Value::Int(i) => Some(*i),
            Value::Double(d) => Some(*d as i64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.as_f64().map(|v| v != 0.0)
    }

    /// A tuple, or a compound of numbers, as its components.
    pub fn as_tuple(&self) -> Option<Vec<f64>> {
        match self {
            Value::Tuple(t) => Some(t.clone()),
            Value::Compound(c) => c.iter().map(Value::as_f64).collect(),
            _ => None,
        }
    }

    pub fn as_matrix(&self) -> Option<[f64; 16]> {
        match self {
            Value::Matrix(m) => Some(**m),
            Value::Data(d) => d.as_any().downcast_ref::<MatrixData>().map(|m| m.0),
            _ => None,
        }
    }

    pub fn as_data<T: 'static>(&self) -> Option<&T> {
        match self {
            Value::Data(d) => d.as_any().downcast_ref::<T>(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        self.as_data::<StringData>().map(|s| s.0.as_str())
    }

    pub fn as_array(&self) -> Option<&[(u32, Value)]> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_compound(&self) -> Option<&[Value]> {
        match self {
            Value::Compound(c) => Some(c),
            _ => None,
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::string(v)
    }
}
