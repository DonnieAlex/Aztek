//! Typed data: what typed attributes hold. Geometry types (mesh, curves, surfaces, lattice) implement
//! [`TypedData`] in the crates that own them.

use crate::types::{DataType, NumericType};
use std::any::Any;
use std::fmt;

pub trait TypedData: Any + Send + Sync + fmt::Debug {
    fn data_type(&self) -> DataType;
    fn as_any(&self) -> &dyn Any;
    /// Equal content; data of another Rust type is never equal.
    fn eq_data(&self, other: &dyn TypedData) -> bool;
}

macro_rules! data {
    ($(#[$m:meta])* $name:ident, $ty:ty, $dt:expr) => {
        $(#[$m])*
        #[derive(Clone, Debug, PartialEq, Default)]
        pub struct $name(pub $ty);
        impl TypedData for $name {
            fn data_type(&self) -> DataType {
                $dt
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn eq_data(&self, other: &dyn TypedData) -> bool {
                other.as_any().downcast_ref::<Self>().is_some_and(|o| o == self)
            }
        }
    };
}

data!(StringData, String, DataType::String);
data!(StringArrayData, Vec<String>, DataType::StringArray);
data!(DoubleArrayData, Vec<f64>, DataType::DoubleArray);
data!(FloatArrayData, Vec<f32>, DataType::FloatArray);
data!(IntArrayData, Vec<i32>, DataType::IntArray);
data!(
    /// Homogeneous points (x, y, z, w).
    PointArrayData,
    Vec<[f64; 4]>,
    DataType::PointArray
);
data!(VectorArrayData, Vec<[f64; 3]>, DataType::VectorArray);
data!(
    /// Row-major 4x4 matrices, as the reference lays them out.
    MatrixArrayData,
    Vec<[f64; 16]>,
    DataType::MatrixArray
);
data!(ComponentListData, Vec<Component>, DataType::ComponentList);

/// Row-major 4x4 identity.
pub const IDENTITY: [f64; 16] = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

/// A matrix held as typed data (the reference's matrix data).
#[derive(Clone, Debug, PartialEq)]
pub struct MatrixData(pub [f64; 16]);

impl Default for MatrixData {
    fn default() -> Self {
        MatrixData(IDENTITY)
    }
}

impl TypedData for MatrixData {
    fn data_type(&self) -> DataType {
        DataType::Matrix
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq_data(&self, other: &dyn TypedData) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|o| o == self)
    }
}

/// Numeric values held as typed data (the reference's numeric data).
#[derive(Clone, Debug, PartialEq)]
pub struct NumericData {
    pub ty: NumericType,
    pub values: Vec<f64>,
}

impl TypedData for NumericData {
    fn data_type(&self) -> DataType {
        DataType::Numeric
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq_data(&self, other: &dyn TypedData) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|o| o == self)
    }
}

/// One component group: a component kind and its indices, one, two or three per element.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Component {
    pub kind: String,
    pub dimensions: u8,
    pub elements: Vec<[i32; 3]>,
}
