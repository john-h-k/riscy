use anyhow::anyhow;
use elf::{abi, endian::AnyEndian, ElfBytes};
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Segment {
    pub offset: u64, // relative address
    pub vaddr: u64,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct LoadedElf {
    pub base: u64,
    pub entrypoint: u64,
    pub segments: Vec<Segment>,
}

impl LoadedElf {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let data = fs::read(path)?;
        let elf = ElfBytes::<AnyEndian>::minimal_parse(&data)?;

        let segments = elf.segments().ok_or(anyhow!("no segments in ELF"))?;

        let base = segments
            .iter()
            .filter(|ph| ph.p_type == abi::PT_LOAD)
            .map(|ph| ph.p_vaddr)
            .min()
            .unwrap_or(0);

        let mut loaded_segments = Vec::new();

        for ph in segments.iter() {
            if ph.p_type != abi::PT_LOAD {
                continue;
            }
            let file_size = ph.p_filesz as usize;
            let mem_size = ph.p_memsz as usize;
            let offset_in_file = ph.p_offset as usize;
            let rel_offset = ph.p_vaddr - base;
            let mut seg_data = vec![0u8; mem_size];
            seg_data[..file_size]
                .copy_from_slice(&data[offset_in_file..offset_in_file + file_size]);
            loaded_segments.push(Segment {
                offset: rel_offset,
                vaddr: ph.p_vaddr,
                data: seg_data,
            });
        }
        Ok(LoadedElf {
            base,
            entrypoint: elf.ehdr.e_entry,
            segments: loaded_segments,
        })
    }

    pub fn find_segment(&self, vaddr: u64) -> Option<(&Segment, usize, usize)> {
        if vaddr < self.base {
            return None;
        }
        let rel_addr = vaddr - self.base;
        for seg in &self.segments {
            let seg_start = seg.offset;
            let seg_end = seg.offset + seg.data.len() as u64;
            if rel_addr >= seg_start && rel_addr < seg_end {
                return Some((seg, seg_start as usize, (rel_addr - seg_start) as usize));
            }
        }
        None
    }
}
