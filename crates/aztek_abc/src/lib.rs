//! Alembic archives (Ogawa files): written and read with no outside library, laid out as the reference's own Alembic
//! library lays them out. Used by AbcExport and AbcImport, the AlembicNode and XJEN's caches.

pub mod abc;
pub mod geom;
pub mod ogawa;

pub use abc::{read, write, Archive, Kind, Object, Pod, Property, TimeSampling};
