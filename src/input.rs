use memmap2::Mmap;
use std::fs::File;
use std::io::Cursor;
use std::ops::Deref;
use std::str::from_utf8;
use zip::ZipArchive;

#[derive(Debug)]
pub struct Input {
    pub mmap: Mmap,
}

#[derive(Debug, Default)]
pub struct FileInfo {
    pub start: usize,
    pub end: usize,
    pub inflated_size: usize,
    pub name: String,
}

impl Input {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mmap = unsafe { Mmap::map(&file).expect("Failed to memory map file") };

        Self { mmap }
    }

    pub fn to_file(&self) -> &str {
        let file_data = from_utf8(&*self.mmap.deref());
        match file_data {
            Ok(data) => data,
            Err(_) => "error",
        }
    }
}

pub fn to_archive(bytes: &[u8]) -> ZipArchive<Cursor<&[u8]>> {
    zip::ZipArchive::new(Cursor::new(&bytes[..])).unwrap()
}

pub fn archive_info(bytes: &[u8]) -> Vec<FileInfo> {
    let mut archive = to_archive(bytes);
    let mut info: Vec<FileInfo> = Vec::with_capacity(archive.len());

    for i in 0..archive.len() {
        let inner_file = archive.by_index(i).unwrap();
        info.push(FileInfo {
            start: inner_file.data_start() as usize,
            end: (inner_file.data_start() + inner_file.compressed_size()) as usize,
            inflated_size: inner_file.size() as usize,
            name: inner_file.name().to_owned(),
        });
    }
    info
}

pub fn inflate(bytes: &[u8], buffer: &mut Vec<u8>, start_idx: usize, end_idx: usize) {
    let _ = libdeflater::Decompressor::new()
        .deflate_decompress(&bytes[start_idx..end_idx], &mut buffer[0..])
        .unwrap();
}

pub fn should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
}

pub trait IsArchive {
    fn is_zipfile_from_bytes(&self) -> bool;
}

impl IsArchive for &[u8] {
    fn is_zipfile_from_bytes(&self) -> bool {
        if self.len() >= 4 && self[0..4] == [0x50, 0x4B, 0x03, 0x04] {
            return true;
        }
        false
    }
}

impl IsArchive for Vec<u8> {
    fn is_zipfile_from_bytes(&self) -> bool {
        if self.len() >= 4 && self[0..4] == [0x50, 0x4B, 0x03, 0x04] {
            return true;
        }
        false
    }
}
