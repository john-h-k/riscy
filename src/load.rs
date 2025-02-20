use anyhow::anyhow;
use elf::{abi, endian::AnyEndian, ElfBytes};
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Segment {
    pub offset: u64, // relative address
    pub vaddr: u64,
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct LoadedElf {
    pub base: u64,
    pub entrypoint: u64,
    pub segments: Vec<Segment>,

    pub wk_memmove: u32,
    pub wk_memcpy: u32,
    pub wk_memset: u32,
    pub wk_cos: u32,
    pub wk_sin: u32,
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

        let mut symbols = Vec::new();
        // iterate over each symbol entry
        if let Some((symbol_table, string_table)) = elf.symbol_table()? {
            for sym in symbol_table {
                if sym.st_name != 0 {
                    symbols.push((
                        string_table.get(sym.st_name as usize)?.to_string(),
                        sym.st_value,
                    ));
                }
            }
        }

        let mut wk_memmove = 0;
        let mut wk_memcpy = 0;
        let mut wk_memset = 0;
        let mut wk_cos = 0;
        let mut wk_sin = 0;
        for (sym, offset) in symbols {
            match sym.as_str() {
                "memset" => wk_memset = offset as u32,
                "memmove" => wk_memmove = offset as u32,
                "memcpy" => wk_memcpy = offset as u32,
                "cos" => wk_cos = offset as u32,
                "sin" => wk_sin = offset as u32,
                _ => {}
            }
        }

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
                size: ph.p_memsz,
                data: seg_data,
            });
        }
        Ok(LoadedElf {
            base,
            entrypoint: elf.ehdr.e_entry,
            wk_memmove,
            wk_memset,
            wk_memcpy,
            wk_cos,
            wk_sin,
            segments: loaded_segments,
        })
    }

    pub fn find_segment(&self, vaddr: u64) -> Option<(&Segment, usize, usize)> {
        if vaddr < self.base {
            return None;
        }

        for seg in &self.segments {
            let seg_start = seg.vaddr;
            let seg_end = seg.vaddr + seg.size;

            if vaddr >= seg_start && vaddr < seg_end {
                return Some((seg, seg_start as usize, (vaddr - seg_start) as usize));
            }
        }
        None
    }
}
