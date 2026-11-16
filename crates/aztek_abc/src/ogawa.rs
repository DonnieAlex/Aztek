//! Ogawa, the container under Alembic: a tree of groups and data blocks in one file. A file starts with "Ogawa", a
//! byte that is 0xff once the file is complete, a u16 version and the position of the root group. A group is a u64
//! count and that many u64 children: a child with the top bit set is a data block (its position, or 0 for empty
//! data), otherwise a group (0 for an empty group). A data block is a u64 size and that many bytes. Little endian.

pub const EMPTY_GROUP: u64 = 0;
pub const DATA_BIT: u64 = 0x8000_0000_0000_0000;
const MAGIC: &[u8; 5] = b"Ogawa";

/// A tree to write.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Group(Vec<Node>),
    /// Empty bytes write as empty data.
    Data(Vec<u8>),
}

fn emit(n: &Node, out: &mut Vec<u8>) -> u64 {
    match n {
        Node::Data(b) if b.is_empty() => DATA_BIT,
        Node::Data(b) => {
            let pos = out.len() as u64;
            out.extend_from_slice(&(b.len() as u64).to_le_bytes());
            out.extend_from_slice(b);
            pos | DATA_BIT
        }
        Node::Group(kids) if kids.is_empty() => EMPTY_GROUP,
        Node::Group(kids) => {
            // Children first, as the reference's library writes them: a group's table comes after what it holds.
            let entries: Vec<u64> = kids.iter().map(|k| emit(k, out)).collect();
            let pos = out.len() as u64;
            out.extend_from_slice(&(entries.len() as u64).to_le_bytes());
            for e in entries {
                out.extend_from_slice(&e.to_le_bytes());
            }
            pos
        }
    }
}

/// The file's bytes.
pub fn write(root: &Node) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 << 16);
    out.extend_from_slice(MAGIC);
    out.push(0xff);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    let root_pos = emit(root, &mut out);
    out[8..16].copy_from_slice(&root_pos.to_le_bytes());
    out
}

/// Reads an Ogawa file held in memory.
pub struct Reader {
    buf: Vec<u8>,
}

#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn err<T>(s: impl Into<String>) -> Result<T> {
    Err(Error(s.into()))
}

impl Reader {
    pub fn new(buf: Vec<u8>) -> Result<Self> {
        if buf.len() < 16 || &buf[0..5] != MAGIC {
            return err("not an Ogawa file (an HDF5 Alembic file is not read)");
        }
        if buf[5] != 0xff {
            return err("the file was not finished when it was written");
        }
        Ok(Reader { buf })
    }

    fn u64_at(&self, pos: u64) -> Result<u64> {
        let p = pos as usize;
        let b = self.buf.get(p..p + 8).ok_or_else(|| Error(format!("position {pos} is past the end of the file")))?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
    }

    /// The root group's entry.
    pub fn root(&self) -> Result<u64> {
        self.u64_at(8)
    }

    /// A group's children; an empty group has none.
    pub fn group(&self, entry: u64) -> Result<Vec<u64>> {
        if entry & DATA_BIT != 0 {
            return err("a data block where a group was expected");
        }
        if entry == EMPTY_GROUP {
            return Ok(Vec::new());
        }
        let n = self.u64_at(entry)?;
        if n > (self.buf.len() as u64) / 8 {
            return err("a group larger than the file");
        }
        (0..n).map(|k| self.u64_at(entry + 8 + 8 * k)).collect()
    }

    /// A data block's bytes; empty data has none.
    pub fn data(&self, entry: u64) -> Result<&[u8]> {
        if entry & DATA_BIT == 0 {
            return err("a group where a data block was expected");
        }
        let pos = entry & !DATA_BIT;
        if pos == 0 {
            return Ok(&[]);
        }
        let n = self.u64_at(pos)? as usize;
        let start = pos as usize + 8;
        self.buf.get(start..start + n).ok_or_else(|| Error("a data block past the end of the file".into()))
    }

    pub fn is_data(entry: u64) -> bool {
        entry & DATA_BIT != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_round_trips() {
        let t = Node::Group(vec![Node::Data(vec![1, 2, 3]), Node::Group(vec![]), Node::Data(vec![]), Node::Group(vec![Node::Data(vec![9])])]);
        let r = Reader::new(write(&t)).unwrap();
        let kids = r.group(r.root().unwrap()).unwrap();
        assert_eq!(kids.len(), 4);
        assert_eq!(r.data(kids[0]).unwrap(), &[1, 2, 3]);
        assert!(r.group(kids[1]).unwrap().is_empty());
        assert!(r.data(kids[2]).unwrap().is_empty());
        let inner = r.group(kids[3]).unwrap();
        assert_eq!(r.data(inner[0]).unwrap(), &[9]);
    }
}
