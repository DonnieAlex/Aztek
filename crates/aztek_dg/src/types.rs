//! The kinds of attribute, data and status the graph knows, one for one with the reference's.

use thiserror::Error;

/// Numeric attribute and numeric data types, in the reference's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumericType {
    Boolean,
    Byte,
    Char,
    Short,
    Short2,
    Short3,
    Int,
    Int2,
    Int3,
    Int64,
    Float,
    Float2,
    Float3,
    Double,
    Double2,
    Double3,
    Double4,
    Addr,
}

impl NumericType {
    /// How many values one item holds.
    pub fn arity(self) -> usize {
        use NumericType::*;
        match self {
            Short2 | Int2 | Float2 | Double2 => 2,
            Short3 | Int3 | Float3 | Double3 => 3,
            Double4 => 4,
            _ => 1,
        }
    }

    /// The type of one component.
    pub fn scalar(self) -> NumericType {
        use NumericType::*;
        match self {
            Short2 | Short3 => Short,
            Int2 | Int3 => Int,
            Float2 | Float3 => Float,
            Double2 | Double3 | Double4 => Double,
            t => t,
        }
    }

    /// The type holding `n` components of `scalar`, where the reference has one.
    pub fn with_arity(scalar: NumericType, n: usize) -> Option<NumericType> {
        use NumericType::*;
        match (scalar.scalar(), n) {
            (s, 1) => Some(s),
            (Short, 2) => Some(Short2),
            (Short, 3) => Some(Short3),
            (Int, 2) => Some(Int2),
            (Int, 3) => Some(Int3),
            (Float, 2) => Some(Float2),
            (Float, 3) => Some(Float3),
            (Double, 2) => Some(Double2),
            (Double, 3) => Some(Double3),
            (Double, 4) => Some(Double4),
            _ => None,
        }
    }

    pub fn is_integer(self) -> bool {
        use NumericType::*;
        matches!(self.scalar(), Byte | Char | Short | Int | Int64 | Addr)
    }
}

/// Unit attribute types. Values are held in internal units: centimetres, radians and seconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitType {
    Angle,
    Distance,
    Time,
}

/// Matrix attribute precision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatrixType {
    Double,
    Float,
}

/// Typed attribute data types, in the reference's order. Plug-in data carries its type id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    Numeric,
    Plugin(u32),
    PluginGeometry(u32),
    String,
    Matrix,
    StringArray,
    DoubleArray,
    FloatArray,
    IntArray,
    PointArray,
    VectorArray,
    MatrixArray,
    ComponentList,
    Mesh,
    Lattice,
    NurbsCurve,
    NurbsSurface,
    Sphere,
    DynArrayAttrs,
    DynSweptGeometry,
    SubdSurface,
    NObject,
    NId,
    Any,
    FalloffFunction,
}

impl DataType {
    /// Whether data of type `other` can go where this type is asked for.
    pub fn accepts(self, other: DataType) -> bool {
        self == DataType::Any || other == DataType::Any || self == other
    }
}

/// What happens to a destination when its incoming connection is broken.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DisconnectBehavior {
    /// Delete the array element (array attributes only).
    Delete,
    /// Reset the attribute to its default.
    Reset,
    /// Keep the attribute's value.
    Nothing,
}

/// What a node's compute returns.
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Success,
    Failure(String),
    /// The plug is not this class's to compute: the class it derives from is asked next.
    UnknownParameter,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum DgError {
    #[error("no such node")]
    NoNode,
    #[error("no node type {0}")]
    NoClass(String),
    #[error("node type {0} is abstract")]
    Abstract(String),
    #[error("no attribute {0}")]
    NoAttribute(String),
    #[error("bad plug: {0}")]
    BadPlug(String),
    #[error("{0} is not readable")]
    NotReadable(String),
    #[error("{0} is not writable")]
    NotWritable(String),
    #[error("{0} is not connectable")]
    NotConnectable(String),
    #[error("{0} is already connected")]
    AlreadyConnected(String),
    #[error("{0} and {1} are not connected")]
    NotConnected(String, String),
    #[error("{0} cannot connect to {1}")]
    Incompatible(String, String),
    #[error("{0} is locked")]
    Locked(String),
    #[error("{0} has an incoming connection")]
    ConnectedDestination(String),
    #[error("wrong value for {0}")]
    WrongValue(String),
    #[error("{0} is already in use")]
    NameClash(String),
    #[error("{0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, DgError>;
