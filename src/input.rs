use crate::error::BagResult;
use memmap2::Mmap;
use std::fs::File;
use std::io::Cursor;
use std::ops::Deref;
use std::path::Path;
use std::str::from_utf8;
use zip::ZipArchive;

/// Mmap File Loader for fast repetitive reads
///
/// Memory-Mapped File loader provider fast read for constrained memory environments
/// The Base LVBAG extract is 3.18 GB compressed and > 100 GB Uncompressed
/// Ideally we want to process this data without needing to fist decompress
///
/// The main Input Struct as a safe wrapper around Unsafe Mmap
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
    pub fn new<P: AsRef<Path>>(path: P) -> BagResult<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }

    pub fn to_file(&self) -> BagResult<&str> {
        let file_data = from_utf8(&*self.mmap.deref())?;
        Ok(file_data)
    }
}

pub fn to_archive(bytes: &[u8]) -> BagResult<ZipArchive<Cursor<&[u8]>>> {
    Ok(zip::ZipArchive::new(Cursor::new(&bytes[..]))?)
}

pub fn archive_info(bytes: &[u8]) -> BagResult<Vec<FileInfo>> {
    let mut archive = to_archive(bytes)?;
    let mut info: Vec<FileInfo> = Vec::with_capacity(archive.len());

    for i in 0..archive.len() {
        let inner_file = archive.by_index(i)?;
        info.push(FileInfo {
            start: inner_file.data_start() as usize,
            end: (inner_file.data_start() + inner_file.compressed_size()) as usize,
            inflated_size: inner_file.size() as usize,
            name: inner_file.name().to_owned(),
        });
    }
    Ok(info)
}

pub fn _inflate(
    bytes: &[u8],
    buffer: &mut Vec<u8>,
    s_idx: usize,
    e_idx: usize,
) -> BagResult<usize> {
    let decompress = libdeflater::Decompressor::new()
        .deflate_decompress(&bytes[s_idx..e_idx], &mut buffer[0..])?;
    Ok(decompress)
}

pub fn _should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
}

/// The [`IsArchive`] trait and associated `is_archive()` checks whether
/// any type T that can be [`AsRef`] into `[u8]` is a archive (aka zip).
///
/// This is done by checking the first 4 bytes, know as the magic or header bytes.
/// The magic bytes for a zip archive is equal to `[0x50, 0x4B, 0x03, 0x04]`
pub trait IsArchive {
    fn _is_archive(&self) -> bool;
}

impl<T: AsRef<[u8]>> IsArchive for T {
    fn _is_archive(&self) -> bool {
        if self.as_ref().len() >= 4 && self.as_ref()[0..4] == [0x50, 0x4B, 0x03, 0x04] {
            return true;
        }
        false
    }
}
