// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::{convert::TryFrom, fmt::Debug, os::fd::RawFd};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct DataType(spa_sys::spa_data_type);

#[allow(non_upper_case_globals)]
impl DataType {
    pub const Invalid: Self = Self(spa_sys::SPA_DATA_Invalid);
    /// Pointer to memory, the data field in struct [`Data`] is set.
    pub const MemPtr: Self = Self(spa_sys::SPA_DATA_MemPtr);
    /// Generic fd, `mmap` to get to memory
    pub const MemFd: Self = Self(spa_sys::SPA_DATA_MemFd);
    /// Fd to `dmabuf` memory
    pub const DmaBuf: Self = Self(spa_sys::SPA_DATA_DmaBuf);
    /// Memory is identified with an id
    pub const MemId: Self = Self(spa_sys::SPA_DATA_MemId);

    pub fn from_raw(raw: spa_sys::spa_data_type) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> spa_sys::spa_data_type {
        self.0
    }
}

impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!(
            "DataType::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::MemPtr => "MemPtr",
                Self::MemFd => "MemFd",
                Self::DmaBuf => "DmaBuf",
                Self::MemId => "MemId",
                _ => "Unknown",
            }
        );
        f.write_str(&name)
    }
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct DataFlags: u32 {
        /// Data is readable
        const READABLE = 1<<0;
        /// Data is writable
        const WRITABLE = 1<<1;
        /// Data pointer can be changed
        const DYNAMIC = 1<<2;
        const READWRITE = Self::READABLE.bits() | Self::WRITABLE.bits();
    }
}

#[repr(transparent)]
pub struct Data(spa_sys::spa_data);

impl Data {
    pub fn as_raw(&self) -> &spa_sys::spa_data {
        &self.0
    }

    pub fn type_(&self) -> DataType {
        DataType::from_raw(self.0.type_)
    }

    pub fn flags(&self) -> DataFlags {
        DataFlags::from_bits_retain(self.0.flags)
    }

    pub fn fd(&self) -> RawFd {
        // We don't have a reliable way of checking if the fd is invalid or uninitialized, so we just return it as a RawFd.
        // The client side will need to use unsafe if they want to manipulate the file descriptor.
        self.0.fd as RawFd
    }

    pub fn data(&mut self) -> Option<&mut [u8]> {
        // FIXME: For safety, perhaps only return a non-mut slice when DataFlags::WRITABLE is not set?
        if self.0.data.is_null() {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts_mut(
                    self.0.data as *mut u8,
                    usize::try_from(self.0.maxsize).unwrap(),
                ))
            }
        }
    }

    pub fn chunk(&self) -> &Chunk {
        assert_ne!(self.0.chunk, std::ptr::null_mut());
        unsafe {
            let chunk: *const spa_sys::spa_chunk = self.0.chunk;
            &*(chunk as *const Chunk)
        }
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        assert_ne!(self.0.chunk, std::ptr::null_mut());
        unsafe {
            let chunk: *mut spa_sys::spa_chunk = self.0.chunk;
            &mut *(chunk as *mut Chunk)
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("type", &self.type_())
            .field("flags", &self.flags())
            .field("fd", &self.fd())
            .field("data", &self.0.data) // Only print the pointer here, as we don't want to print a (potentially very big) slice.
            .field("chunk", &self.chunk())
            .finish()
    }
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ChunkFlags: i32 {
        /// Chunk data is corrupted in some way
        const CORRUPTED = 1<<0;
    }
}

#[repr(transparent)]
pub struct Chunk(spa_sys::spa_chunk);

impl Chunk {
    pub fn as_raw(&self) -> &spa_sys::spa_chunk {
        &self.0
    }

    pub fn size(&self) -> u32 {
        self.0.size
    }

    pub fn size_mut(&mut self) -> &mut u32 {
        &mut self.0.size
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.0.offset
    }

    pub fn stride(&self) -> i32 {
        self.0.stride
    }

    pub fn stride_mut(&mut self) -> &mut i32 {
        &mut self.0.stride
    }

    pub fn flags(&self) -> ChunkFlags {
        ChunkFlags::from_bits_retain(self.0.flags)
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("stride", &self.stride())
            .field("flags", &self.flags())
            .finish()
    }
}

pub trait Metadata {
    const META_TYPE: u32;
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct MetaHeaderFlags: u32 {
        /// Buffer is discontinuous
        const DISCONT = 1 << 0;
        /// Buffer data is corrupted
        const CORRUPTED = 1 << 1;
        /// Buffer contains a codec config marker
        const MARKER = 1 << 2;
        /// Buffer contains header data
        const HEADER = 1 << 3;
        /// Buffer represents a gap in the stream
        const GAP = 1 << 4;
        /// Buffer is a delta unit (not self-contained)
        const DELTA_UNIT = 1 << 5;
    }
}

#[repr(transparent)]
pub struct MetaHeader(spa_sys::spa_meta_header);

impl MetaHeader {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_header {
        &self.0
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn pts(&self) -> i64 {
        self.0.pts
    }

    pub fn dts_offset(&self) -> i64 {
        self.0.dts_offset
    }

    pub fn seq(&self) -> u64 {
        self.0.seq
    }

    pub fn flags(&self) -> MetaHeaderFlags {
        MetaHeaderFlags::from_bits_retain(self.0.flags)
    }
}

impl Metadata for MetaHeader {
    const META_TYPE: u32 = spa_sys::SPA_META_Header;
}

#[repr(transparent)]
pub struct VideoCrop(spa_sys::spa_meta_region);

impl VideoCrop {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_region {
        &self.0
    }

    /// Returns the coordinates of the region as (x, y)
    pub fn position(&self) -> (i32, i32) {
        (self.0.region.position.x, self.0.region.position.y)
    }

    /// Returns the size of the region as (width, height)
    pub fn size(&self) -> (u32, u32) {
        (self.0.region.size.width, self.0.region.size.height)
    }
}

impl Metadata for VideoCrop {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoCrop;
}

#[repr(transparent)]
pub struct VideoDamage(spa_sys::spa_meta_region);

impl VideoDamage {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_region {
        &self.0
    }

    /// Returns the coordinates of the region as (x, y)
    pub fn position(&self) -> (i32, i32) {
        (self.0.region.position.x, self.0.region.position.y)
    }

    /// Returns the size of the region as (width, height)
    pub fn size(&self) -> (u32, u32) {
        (self.0.region.size.width, self.0.region.size.height)
    }
}

impl Metadata for VideoDamage {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoDamage;
}

#[repr(transparent)]
pub struct MetaBitmap(spa_sys::spa_meta_bitmap);

impl MetaBitmap {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_bitmap {
        &self.0
    }

    pub fn format(&self) -> u32 {
        self.0.format
    }

    /// Returns the size of the bitmap as (width, height)
    pub fn size(&self) -> (u32, u32) {
        (self.0.size.width, self.0.size.height)
    }

    pub fn stride(&self) -> i32 {
        self.0.stride
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }
}

impl Metadata for MetaBitmap {
    const META_TYPE: u32 = spa_sys::SPA_META_Bitmap;
}

#[repr(transparent)]
pub struct MetaCursor(spa_sys::spa_meta_cursor);

impl MetaCursor {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_cursor {
        &self.0
    }

    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn flags(&self) -> u32 {
        self.0.flags
    }

    /// Returns the position, on screen, of the cursor as (x, y)
    pub fn position(&self) -> (i32, i32) {
        (self.0.position.x, self.0.position.y)
    }

    /// offsets for hotspot in bitmap as (x, y).
    /// This field has no meaning  when there is no valid bitmap.
    pub fn hotspot(&self) -> (i32, i32) {
        (self.0.hotspot.x, self.0.hotspot.y)
    }

    pub fn bitmap_offset(&self) -> u32 {
        self.0.bitmap_offset
    }
}

impl Metadata for MetaCursor {
    const META_TYPE: u32 = spa_sys::SPA_META_Cursor;
}

#[repr(transparent)]
pub struct MetaControl(spa_sys::spa_meta_control);

impl MetaControl {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_control {
        &self.0
    }

    pub fn sequence(&self) -> &spa_sys::spa_pod_sequence {
        &self.0.sequence
    }
}

impl Metadata for MetaControl {
    const META_TYPE: u32 = spa_sys::SPA_META_Control;
}

#[repr(transparent)]
pub struct MetaBusy(u32);

impl MetaBusy {
    pub fn as_raw(&self) -> &u32 {
        &self.0
    }

    pub fn is_busy(&self) -> bool {
        self.0 != 0
    }
}

impl Metadata for MetaBusy {
    const META_TYPE: u32 = spa_sys::SPA_META_Busy;
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct VideoTransform: u32 {
        const IDENTITY = 0;
        const ROT_90 = 1;
        const ROT_180 = 2;
        const ROT_270 = 3;
        const FLIP_X = 4;
        const FLIP_Y = 5;
        const FLIP_XY = 6;
    }
}

#[repr(transparent)]
pub struct MetaVideoTransform(spa_sys::spa_meta_videotransform);

impl MetaVideoTransform {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_videotransform {
        &self.0
    }

    pub fn transform(&self) -> VideoTransform {
        VideoTransform::from_bits_retain(self.0.transform)
    }
}

impl Metadata for MetaVideoTransform {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoTransform;
}

#[repr(transparent)]
pub struct MetaSyncTimeline(spa_sys::spa_meta_sync_timeline);

impl MetaSyncTimeline {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_sync_timeline {
        &self.0
    }

    pub fn flags(&self) -> u32 {
        self.0.flags
    }

    pub fn padding(&self) -> u32 {
        self.0.padding
    }

    /// The timeline acquire point - when the data can be accessed
    pub fn acquire_point(&self) -> u64 {
        self.0.acquire_point
    }

    /// The timeline release point - should be signaled when data is no longer accessed
    pub fn release_point(&self) -> u64 {
        self.0.release_point
    }
}

impl Metadata for MetaSyncTimeline {
    const META_TYPE: u32 = spa_sys::SPA_META_SyncTimeline;
}
