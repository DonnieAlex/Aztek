//! Attributes: kinds, properties with the reference's defaults, and the specs node types and dynamic
//! attributes are built from.

use crate::data::{MatrixData, TypedData, IDENTITY};
use crate::node::NodeId;
use crate::registry::ClassId;
use crate::types::{DataType, DgError, DisconnectBehavior, MatrixType, NumericType, Result, UnitType};
use crate::value::Value;
use std::fmt;
use std::sync::Arc;

/// An attribute, unique across every node type and dynamic attribute in a registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttrId(pub u32);

impl fmt::Display for AttrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attr#{}", self.0)
    }
}

/// Attribute properties. `Default` gives the reference's defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct Props {
    /// Can be the source of a connection.
    pub readable: bool,
    /// Can be the destination of a connection.
    pub writable: bool,
    pub connectable: bool,
    /// Saved with the scene.
    pub storable: bool,
    /// Kept in the node's datablock.
    pub cached: bool,
    pub array: bool,
    /// Whether an index must be given when connecting to the array.
    pub index_matters: bool,
    pub keyable: bool,
    pub channel_box: bool,
    pub hidden: bool,
    pub used_as_color: bool,
    pub indeterminant: bool,
    pub render_source: bool,
    pub world_space: bool,
    pub affects_world_space: bool,
    pub affects_appearance: bool,
    pub used_as_filename: bool,
    /// The node keeps the value itself, through its internal value hooks.
    pub internal: bool,
    pub uses_array_data_builder: bool,
    pub disconnect_behavior: DisconnectBehavior,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            readable: true,
            writable: true,
            connectable: true,
            storable: true,
            cached: true,
            array: false,
            index_matters: true,
            keyable: false,
            channel_box: false,
            hidden: false,
            used_as_color: false,
            indeterminant: false,
            render_source: false,
            world_space: false,
            affects_world_space: false,
            affects_appearance: false,
            used_as_filename: false,
            internal: false,
            uses_array_data_builder: false,
            disconnect_behavior: DisconnectBehavior::Nothing,
        }
    }
}

/// Hard and soft limits, one value per component.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Limits {
    pub min: Option<Vec<f64>>,
    pub max: Option<Vec<f64>>,
    pub soft_min: Option<Vec<f64>>,
    pub soft_max: Option<Vec<f64>>,
}

#[derive(Clone, Debug)]
pub enum AttrKind {
    Numeric { ty: NumericType, default: Value },
    Unit { unit: UnitType, default: f64 },
    Enum { fields: Vec<(String, i16)>, default: i16 },
    Typed { ty: DataType, default: Option<Arc<dyn TypedData>> },
    Matrix { ty: MatrixType, default: [f64; 16] },
    Message,
    /// `numeric` is set for a numeric compound (made from numeric children, like a translate).
    Compound { children: Vec<AttrId>, numeric: Option<NumericType> },
    /// Accepts any of the listed numeric and data types.
    Generic { numeric: Vec<NumericType>, data: Vec<DataType> },
}

/// Who an attribute belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Class(ClassId),
    Dynamic(NodeId),
}

#[derive(Clone, Debug)]
pub struct AttrDef {
    pub id: AttrId,
    pub name: String,
    pub short: String,
    pub nice: Option<String>,
    pub kind: AttrKind,
    pub props: Props,
    pub limits: Limits,
    pub parent: Option<AttrId>,
    pub owner: Owner,
    /// Added to a node type after it was registered.
    pub extension: bool,
}

impl AttrDef {
    pub fn children(&self) -> &[AttrId] {
        match &self.kind {
            AttrKind::Compound { children, .. } => children,
            _ => &[],
        }
    }

    pub fn is_compound(&self) -> bool {
        matches!(self.kind, AttrKind::Compound { .. })
    }
}

/// An attribute to be added, with its children. Built with the functions below and the setters,
/// the way the reference's attribute function sets build one.
#[derive(Clone, Debug)]
pub struct AttrSpec {
    pub name: String,
    pub short: String,
    pub nice: Option<String>,
    pub kind: SpecKind,
    pub props: Props,
    pub limits: Limits,
}

#[derive(Clone, Debug)]
pub enum SpecKind {
    Leaf(AttrKind),
    Compound { children: Vec<AttrSpec>, numeric: Option<NumericType> },
}

fn spec(name: &str, short: &str, kind: SpecKind) -> AttrSpec {
    AttrSpec { name: name.into(), short: short.into(), nice: None, kind, props: Props::default(), limits: Limits::default() }
}

fn zero(ty: NumericType) -> Value {
    if ty.arity() > 1 {
        Value::Tuple(vec![0.0; ty.arity()])
    } else if ty == NumericType::Boolean {
        Value::Bool(false)
    } else if ty.is_integer() {
        Value::Int(0)
    } else {
        Value::Double(0.0)
    }
}

/// A numeric attribute; its default is zero until set with [`AttrSpec::default_value`].
pub fn numeric(name: &str, short: &str, ty: NumericType) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Numeric { ty, default: zero(ty) }))
}

/// A numeric compound from two or three numeric children of one type, like a translate.
pub fn numeric_compound(name: &str, short: &str, children: Vec<AttrSpec>) -> Result<AttrSpec> {
    let mut scalar = None;
    for c in &children {
        match &c.kind {
            SpecKind::Leaf(AttrKind::Numeric { ty, .. }) if ty.arity() == 1 => {
                if scalar.is_some_and(|s| s != *ty) {
                    return Err(DgError::Invalid(format!("{name}: children of one numeric type are needed")));
                }
                scalar = Some(*ty);
            }
            SpecKind::Leaf(AttrKind::Unit { .. }) => scalar = scalar.or(Some(NumericType::Double)),
            _ => return Err(DgError::Invalid(format!("{name}: numeric children are needed"))),
        }
    }
    let ty = scalar
        .filter(|_| (2..=3).contains(&children.len()))
        .and_then(|s| NumericType::with_arity(s, children.len()))
        .ok_or_else(|| DgError::Invalid(format!("{name}: two or three children are needed")))?;
    Ok(spec(name, short, SpecKind::Compound { children, numeric: Some(ty) }))
}

pub fn unit(name: &str, short: &str, unit: UnitType) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Unit { unit, default: 0.0 }))
}

pub fn enumeration(name: &str, short: &str, default: i16) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Enum { fields: Vec::new(), default }))
}

pub fn typed(name: &str, short: &str, ty: DataType) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Typed { ty, default: None }))
}

pub fn matrix(name: &str, short: &str, ty: MatrixType) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Matrix { ty, default: IDENTITY }))
}

pub fn message(name: &str, short: &str) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Message))
}

pub fn compound(name: &str, short: &str, children: Vec<AttrSpec>) -> AttrSpec {
    spec(name, short, SpecKind::Compound { children, numeric: None })
}

pub fn generic(name: &str, short: &str) -> AttrSpec {
    spec(name, short, SpecKind::Leaf(AttrKind::Generic { numeric: Vec::new(), data: Vec::new() }))
}

/// A light data attribute: a compound of direction (float3), intensity (float3), ambient, diffuse
/// and specular (booleans), shadow fraction and pre-shadow intensity (floats) and blind data (address),
/// in that order, named by the caller as in the reference.
pub fn light_data(name: &str, short: &str, children: [AttrSpec; 8]) -> Result<AttrSpec> {
    use NumericType::*;
    let want = [Float3, Float3, Boolean, Boolean, Boolean, Float, Float, Addr];
    for (c, w) in children.iter().zip(want) {
        let ty = match &c.kind {
            SpecKind::Leaf(AttrKind::Numeric { ty, .. }) => Some(*ty),
            SpecKind::Compound { numeric, .. } => *numeric,
            _ => None,
        };
        if ty != Some(w) {
            return Err(DgError::Invalid(format!("{name}: child {} should be {w:?}", c.name)));
        }
    }
    Ok(spec(name, short, SpecKind::Compound { children: children.into(), numeric: None }))
}

macro_rules! setters {
    ($($f:ident),*) => {
        $(pub fn $f(mut self, on: bool) -> Self {
            self.props.$f = on;
            self
        })*
    };
}

impl AttrSpec {
    setters!(
        readable,
        writable,
        connectable,
        storable,
        cached,
        array,
        index_matters,
        keyable,
        channel_box,
        hidden,
        used_as_color,
        indeterminant,
        render_source,
        world_space,
        affects_world_space,
        affects_appearance,
        used_as_filename,
        internal,
        uses_array_data_builder
    );

    pub fn disconnect_behavior(mut self, b: DisconnectBehavior) -> Self {
        self.props.disconnect_behavior = b;
        self
    }

    pub fn nice_name(mut self, n: &str) -> Self {
        self.nice = Some(n.into());
        self
    }

    pub fn min(mut self, v: &[f64]) -> Self {
        self.limits.min = Some(v.to_vec());
        self
    }

    pub fn max(mut self, v: &[f64]) -> Self {
        self.limits.max = Some(v.to_vec());
        self
    }

    pub fn soft_min(mut self, v: &[f64]) -> Self {
        self.limits.soft_min = Some(v.to_vec());
        self
    }

    pub fn soft_max(mut self, v: &[f64]) -> Self {
        self.limits.soft_max = Some(v.to_vec());
        self
    }

    /// Sets the default. Numeric, unit, enum, matrix and typed attributes have one.
    pub fn default_value(mut self, v: Value) -> Result<Self> {
        let name = self.name.clone();
        let bad = || DgError::WrongValue(name.clone());
        match &mut self.kind {
            SpecKind::Leaf(AttrKind::Numeric { ty, default }) => *default = coerce_numeric(*ty, v).ok_or_else(bad)?,
            SpecKind::Leaf(AttrKind::Unit { default, .. }) => *default = v.as_f64().ok_or_else(bad)?,
            SpecKind::Leaf(AttrKind::Enum { default, .. }) => *default = v.as_i64().ok_or_else(bad)? as i16,
            SpecKind::Leaf(AttrKind::Matrix { default, .. }) => *default = v.as_matrix().ok_or_else(bad)?,
            SpecKind::Leaf(AttrKind::Typed { ty, default }) => match v {
                Value::None => *default = None,
                Value::Data(d) if ty.accepts(d.data_type()) => *default = Some(d),
                _ => return Err(bad()),
            },
            _ => return Err(bad()),
        }
        Ok(self)
    }

    /// Adds an enum field.
    pub fn field(mut self, name: &str, value: i16) -> Self {
        if let SpecKind::Leaf(AttrKind::Enum { fields, .. }) = &mut self.kind {
            fields.push((name.into(), value));
        }
        self
    }

    /// Adds a numeric type a generic attribute accepts.
    pub fn accept_numeric(mut self, t: NumericType) -> Self {
        if let SpecKind::Leaf(AttrKind::Generic { numeric, .. }) = &mut self.kind {
            numeric.push(t);
        }
        self
    }

    /// Adds a data type a generic attribute accepts.
    pub fn accept_data(mut self, t: DataType) -> Self {
        if let SpecKind::Leaf(AttrKind::Generic { data, .. }) = &mut self.kind {
            data.push(t);
        }
        self
    }
}

fn scalar_to(ty: NumericType, v: &Value) -> Option<Value> {
    use NumericType::*;
    Some(match ty {
        Boolean => Value::Bool(v.as_bool()?),
        Byte => Value::Int(v.as_i64()? as u8 as i64),
        Char => Value::Int(v.as_i64()? as i8 as i64),
        Short | Short2 | Short3 => Value::Int(v.as_i64()? as i16 as i64),
        Int | Int2 | Int3 => Value::Int(v.as_i64()? as i32 as i64),
        Int64 | Addr => Value::Int(v.as_i64()?),
        Float | Float2 | Float3 => Value::Double(v.as_f64()? as f32 as f64),
        Double | Double2 | Double3 | Double4 => Value::Double(v.as_f64()?),
    })
}

/// A value converted to a numeric type, the way the reference's data handles convert.
pub fn coerce_numeric(ty: NumericType, v: Value) -> Option<Value> {
    if ty.arity() == 1 {
        return scalar_to(ty, &v);
    }
    let t = v.as_tuple()?;
    if t.len() != ty.arity() {
        return None;
    }
    t.into_iter()
        .map(|x| scalar_to(ty.scalar(), &Value::Double(x)).and_then(|v| v.as_f64()))
        .collect::<Option<Vec<_>>>()
        .map(Value::Tuple)
}

/// The value a leaf attribute holds before anything sets it.
pub fn leaf_default(def: &AttrDef) -> Value {
    match &def.kind {
        AttrKind::Numeric { default, .. } => default.clone(),
        AttrKind::Unit { default, .. } => Value::Double(*default),
        AttrKind::Enum { default, .. } => Value::Int(*default as i64),
        AttrKind::Typed { default, .. } => default.clone().map(Value::Data).unwrap_or(Value::None),
        AttrKind::Matrix { default, .. } => Value::matrix(*default),
        AttrKind::Message | AttrKind::Generic { .. } | AttrKind::Compound { .. } => Value::None,
    }
}

/// A leaf value converted for `def`, or `None` when it cannot go there.
pub fn coerce_leaf(def: &AttrDef, v: Value) -> Option<Value> {
    match &def.kind {
        AttrKind::Numeric { ty, .. } => coerce_numeric(*ty, v),
        AttrKind::Unit { .. } => v.as_f64().map(Value::Double),
        AttrKind::Enum { .. } => v.as_i64().map(|i| Value::Int(i as i16 as i64)),
        AttrKind::Typed { ty, .. } => match v {
            Value::None => Some(Value::None),
            Value::Data(d) if ty.accepts(d.data_type()) => Some(Value::Data(d)),
            Value::Matrix(m) if *ty == DataType::Matrix => Some(Value::data(MatrixData(*m))),
            _ => None,
        },
        AttrKind::Matrix { ty, .. } => v.as_matrix().map(|m| match ty {
            MatrixType::Double => Value::matrix(m),
            MatrixType::Float => Value::matrix(m.map(|x| x as f32 as f64)),
        }),
        AttrKind::Message => matches!(v, Value::None).then_some(Value::None),
        AttrKind::Generic { numeric, data } => match v {
            Value::None => Some(Value::None),
            Value::Data(d) => data.iter().any(|t| t.accepts(d.data_type())).then_some(Value::Data(d)),
            Value::Matrix(m) => data.contains(&DataType::Matrix).then_some(Value::Matrix(m)),
            other => {
                let n = match &other {
                    Value::Tuple(t) => t.len(),
                    Value::Compound(c) => c.len(),
                    _ => 1,
                };
                let ty = numeric.iter().find(|t| t.arity() == n)?;
                coerce_numeric(*ty, other)
            }
        },
        AttrKind::Compound { .. } => None,
    }
}
