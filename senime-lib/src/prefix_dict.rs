use std::collections::BTreeMap;

use bincode::{Decode, Encode};

use crate::dict::{Candidate, StringArena};

// Dict 结构
#[derive(Debug)]
pub struct PrefixDict {
    pub(crate) arena: StringArena,
    pub(crate) prism: Prism,
    pub(crate) candidates: Vec<Candidate>,
}
// Dict 节点结构 — 使用连续字节存储所有前缀，避免每个前缀独立 Vec<char> 的堆分配开销。
#[derive(Debug)]
pub(crate) struct Prism {
    // 所有前缀的字节连续存放（code 都是 ASCII，每个字符 1 字节 vs char 的 4 字节）
    keys: Vec<u8>,
    // 每个前缀在 keys 中的 (offset, len)
    key_meta: Vec<(u32, u16)>,
    // candidates 的索引范围，与 key_meta 一一对应: [start, end)
    indices: Vec<(usize, usize)>,
}
impl Default for PrefixDict {
    fn default() -> Self {
        Self {
            arena: StringArena::new(),
            prism: Prism {
                keys: vec![],
                key_meta: vec![],
                indices: vec![],
            },
            candidates: vec![],
        }
    }
}

impl PrefixDict {
    pub fn reachable(&self, chars: &[char]) -> bool {
        self.prism.lookup(chars).is_some()
    }

    pub fn search(&self, chars: &[char]) -> Option<&[Candidate]> {
        if let Some(range) = self.prism.lookup(chars) {
            Some(&self.candidates[range.0..range.1])
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.candidates.len()
    }

    pub fn get_str(&self, range: (u32, u16)) -> &str {
        self.arena.get(range.0, range.1)
    }
}

impl Prism {
    pub(crate) fn new_with_arena(candidates: &[Candidate], arena: &StringArena) -> Self {
        let mut map: BTreeMap<Vec<u8>, (usize, usize)> = BTreeMap::new();
        for (i, cand) in candidates.iter().enumerate() {
            let code_bytes = arena.get(cand.code.0, cand.code.1).as_bytes();
            for len in 1..=code_bytes.len() {
                let prefix = &code_bytes[..len];
                map.entry(prefix.to_vec())
                    .and_modify(|r| r.1 = i + 1)
                    .or_insert((i, i + 1));
            }
        }
        let mut keys = Vec::new();
        let mut key_meta = Vec::with_capacity(map.len());
        let mut indices = Vec::with_capacity(map.len());
        for (prefix, range) in map {
            let offset = keys.len() as u32;
            let len = prefix.len() as u16;
            keys.extend_from_slice(&prefix);
            key_meta.push((offset, len));
            indices.push(range);
        }
        keys.shrink_to_fit();
        Self {
            keys,
            key_meta,
            indices,
        }
    }

    fn lookup(&self, code: &[char]) -> Option<&(usize, usize)> {
        // code 都是 ASCII lowercase，直接转为字节比较
        let code_bytes: Vec<u8> = code.iter().map(|c| *c as u8).collect();
        let idx = self
            .key_meta
            .binary_search_by(|&(offset, len)| {
                let start = offset as usize;
                let end = start + len as usize;
                self.keys[start..end].cmp(&code_bytes)
            })
            .ok()?;
        self.indices.get(idx)
    }
}

impl Encode for Prism {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.keys, encoder)?;
        bincode::Encode::encode(&self.key_meta, encoder)?;
        bincode::Encode::encode(&self.indices, encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Prism {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let keys: Vec<u8> = bincode::Decode::decode(decoder)?;
        let key_meta: Vec<(u32, u16)> = bincode::Decode::decode(decoder)?;
        let indices: Vec<(usize, usize)> = bincode::Decode::decode(decoder)?;
        Ok(Self {
            keys,
            key_meta,
            indices,
        })
    }
}
