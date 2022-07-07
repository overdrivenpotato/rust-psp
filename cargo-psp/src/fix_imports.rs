use goblin::elf::Elf;
use std::{collections::HashMap, mem, path::Path};

/// A stub library entry like the one found in the `psp` crate, but with types
/// changed to be cross-platform.
///
/// In particular, pointers are swapped to `u32`.
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
struct SceStubLibraryEntry {
    name: u32,
    version: [u8; 2],
    flags: u16,
    len: u8,
    v_stub_count: u8,
    stub_count: u16,
    nid_table: u32,
    stub_table: u32,
}

/// Fixes the stub count for all imported modules.
///
/// When linking, sometimes it is possible for imported functions to be
/// completely stripped. This can happen with LTO, for example. Because of this,
/// we need a way to write the stub count post-linking. This function does
/// exactly this.
pub fn fix<T: AsRef<Path>>(path: T) {
    let mut bytes = std::fs::read(&path).unwrap();
    let elf = Elf::parse(&bytes).unwrap();

    let shstrtab = {
        let sh = &elf.section_headers[elf.header.e_shstrndx as usize];
        let start = sh.sh_offset as usize;
        let end = (sh.sh_offset + sh.sh_size) as usize;

        &bytes[start..end]
    };

    // Map of name -> section header
    let sections = elf
        .section_headers
        .iter()
        .map(|sh| {
            let name = shstrtab[sh.sh_name..]
                .iter()
                .take_while(|b| **b != 0)
                .map(|b| *b as char)
                .collect::<String>();

            (name, sh)
        })
        .collect::<HashMap<_, _>>();

    let lib_stub = match sections.get(".lib.stub") {
        Some(s) => s,

        // The binary might not import any functions, in some rare cases.
        None => return,
    };

    // TODO: Use module info instead of these sections, as they can technically
    // be stripped.

    // If we have .lib.stub, then .lib.stub.btm must exist.
    let lib_stub_btm = sections
        .get(".lib.stub.btm")
        .expect("could not find .lib.stub.btm section");

    let rodata_sce_nid = sections
        .get(".rodata.sceNid")
        .expect("Could not find .rodata.sceNid section");

    let start = lib_stub.sh_offset as usize;
    let end = lib_stub_btm.sh_offset as usize;

    // Rough check for the length.
    assert_eq!((end - start) % mem::size_of::<SceStubLibraryEntry>(), 0);

    // List of (stub index in .lib.stub, stub entry). Ordered by appearance in
    // .rodata.sceNid section.
    let stubs_nid_sorted = {
        let mut entries = bytes[start..end]
            .chunks(mem::size_of::<SceStubLibraryEntry>())
            .map(bincode::deserialize)
            .map(Result::unwrap)
            .enumerate()
            .collect::<Vec<(_, SceStubLibraryEntry)>>();

        // Sort by order of appearance in .rodata.sceNid section.
        entries.sort_unstable_by_key(|(_, s)| s.nid_table);

        entries
    };

    // Iterator of mutable byte slices (of stub lib entries) in raw ELF binary.
    let stub_entry_bufs = bytes[start..end].chunks_mut(mem::size_of::<SceStubLibraryEntry>());

    for (i, stub_entry_buf) in stub_entry_bufs.enumerate() {
        // A NID is a 32-bit value.
        const NID_SIZE: u32 = 4;

        let mut stub: SceStubLibraryEntry = bincode::deserialize(stub_entry_buf).unwrap();

        let nid_end = stubs_nid_sorted
            .get(1 + stubs_nid_sorted.iter().position(|&(j, _)| i == j).unwrap())
            .map(|(_, s)| s.nid_table)
            .unwrap_or(rodata_sce_nid.sh_addr as u32 + rodata_sce_nid.sh_size as u32);

        stub.stub_count = ((nid_end - stub.nid_table) / NID_SIZE) as u16;

        // Re-serialize the stub and save.
        let serialized = bincode::serialize(&stub).unwrap();
        stub_entry_buf.copy_from_slice(&serialized);
    }

    std::fs::write(path, bytes).unwrap();
}
