//! What the Density and Place tools change on a description's base: splines taken away (by id) and splines placed
//! (by face, triangle and weights, so they follow the mesh as grown ones do). Kept on the base as plug-in data and
//! written in scene files as text.

use std::any::Any;
use std::collections::BTreeSet;
use std::sync::Arc;

use aztek_geo::Mesh;
use serde_json::{json, Value};

use super::SplineData;
use crate::plug::{flags, AttrDef, PlugType, PlugValue};

pub const KIND: &str = "xjenEditData";

/// The bit that marks a placed spline's id.
pub const PLACED: u64 = 1 << 31;

#[derive(Clone, Debug, PartialEq)]
pub struct Placed {
    pub face: u32,
    pub corners: [u32; 3],
    pub w: [f64; 3],
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditData {
    pub removed: BTreeSet<u64>,
    pub placed: Vec<Placed>,
}

impl EditData {
    pub fn placed_id(face: u32, k: usize) -> u64 {
        (face as u64) << 32 | PLACED | k as u64
    }

    /// Takes away the removed splines and adds the placed ones, straight along the normal for length.
    pub fn apply(&self, mesh: &Mesh, s: &mut SplineData, length: f64, cv: usize) {
        if !self.removed.is_empty() {
            s.splines.retain(|x| !self.removed.contains(&x.id));
        }
        if self.placed.is_empty() {
            return;
        }
        let normals = aztek_geo::normals::point_normals(mesh, aztek_geo::normals::NormalWeight::Angle);
        let n = mesh.point_capacity() as usize;
        for (k, p) in self.placed.iter().enumerate() {
            let id = Self::placed_id(p.face, k);
            let ok = mesh.face_valid(p.face as _) && p.corners.iter().all(|&c| (mesh.corner_point(c) as usize) < n);
            if !ok || self.removed.contains(&id) {
                continue;
            }
            s.splines.push(super::place(mesh, &normals, p.face, id, p.corners, p.w, length, cv.max(2)));
        }
    }

    /// Words: removed count and ids, placed count and (face, three corners, three weights) each.
    pub fn text(&self) -> String {
        let mut out = vec![self.removed.len().to_string()];
        out.extend(self.removed.iter().map(u64::to_string));
        out.push(self.placed.len().to_string());
        for p in &self.placed {
            out.push(p.face.to_string());
            out.extend(p.corners.iter().map(u32::to_string));
            out.extend(p.w.iter().map(|x| format!("{x}")));
        }
        out.join(" ")
    }

    pub fn parse(text: &str) -> Result<Self, String> {
        let mut it = text.split_whitespace();
        let mut next = || it.next().ok_or_else(|| "edit data ends early".to_string());
        let mut d = EditData::default();
        let n: usize = next()?.parse().map_err(|_| "removed count")?;
        for _ in 0..n {
            d.removed.insert(next()?.parse().map_err(|_| "removed id")?);
        }
        let n: usize = next()?.parse().map_err(|_| "placed count")?;
        for _ in 0..n {
            let face = next()?.parse().map_err(|_| "face")?;
            let mut corners = [0u32; 3];
            for c in &mut corners {
                *c = next()?.parse().map_err(|_| "corner")?;
            }
            let mut w = [0f64; 3];
            for x in &mut w {
                *x = next()?.parse().map_err(|_| "weight")?;
            }
            d.placed.push(Placed { face, corners, w });
        }
        Ok(d)
    }
}

impl crate::plug::Data for EditData {
    fn kind(&self) -> &str {
        KIND
    }
    fn to_json(&self) -> Value {
        json!({"kind": KIND, "pluginData": true, "text": self.text()})
    }
    fn bytes(&self) -> usize {
        64 + self.removed.len() * 8 + self.placed.len() * 40
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_kind() {
    crate::plug::data::register_kind(
        KIND,
        Arc::new(|j: &Value| {
            let text = j.get("text").and_then(Value::as_str).unwrap_or("0 0");
            EditData::parse(text).map(|d| Arc::new(d) as crate::plug::DataPtr).map_err(crate::error::GraphError::invalid)
        }),
    );
}

pub fn edit_data(v: &PlugValue) -> Option<&EditData> {
    match v {
        PlugValue::Data(Some(d)) => d.as_any().downcast_ref::<EditData>(),
        _ => None,
    }
}

pub fn attr() -> AttrDef {
    AttrDef {
        name: "editData".into(),
        short_name: "edd".into(),
        ty: PlugType::Data,
        default_value: PlugValue::Data(None),
        flags: flags::INPUT | flags::STORABLE | flags::HIDDEN,
        category: format!("{}{KIND}", crate::scripted::PLUGIN_DATA),
        doc: "Splines the Density and Place tools took away or added.".into(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_round_trips() {
        let mut d = EditData::default();
        d.removed.insert(5);
        d.removed.insert(1 << 33);
        d.placed.push(Placed { face: 3, corners: [1, 2, 3], w: [0.25, 0.5, 0.25] });
        assert_eq!(EditData::parse(&d.text()).unwrap(), d);
    }
}
