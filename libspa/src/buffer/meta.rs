use crate::param::video::VideoFormat;
use crate::utils::{Point, Rectangle, Region};

use std::fmt::Debug;

pub trait Metadata {
    const META_TYPE: u32;
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct MetaHeaderFlags: u32 {
        /// Buffer is discontinuous
        const DISCONT = spa_sys::SPA_META_HEADER_FLAG_DISCONT;
        /// Buffer data is corrupted
        const CORRUPTED = spa_sys::SPA_META_HEADER_FLAG_CORRUPTED;
        /// Buffer contains a codec config marker
        const MARKER = spa_sys::SPA_META_HEADER_FLAG_MARKER;
        /// Buffer contains header data
        const HEADER = spa_sys::SPA_META_HEADER_FLAG_HEADER;
        /// Buffer represents a gap in the stream
        const GAP = spa_sys::SPA_META_HEADER_FLAG_GAP;
        /// Buffer is a delta unit (not self-contained)
        const DELTA_UNIT = spa_sys::SPA_META_HEADER_FLAG_DELTA_UNIT;
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct MetaHeader(spa_sys::spa_meta_header);

impl MetaHeader {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_header {
        &self.0
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn flags(&self) -> MetaHeaderFlags {
        MetaHeaderFlags::from_bits_retain(self.0.flags)
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
}

impl Debug for MetaHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaHeader")
            .field("flags", &self.flags())
            .field("offset", &self.offset())
            .field("pts", &self.pts())
            .field("dts_offset", &self.dts_offset())
            .field("seq", &self.seq())
            .finish()
    }
}

impl Metadata for MetaHeader {
    const META_TYPE: u32 = spa_sys::SPA_META_Header;
}

#[derive(Clone)]
#[repr(transparent)]
pub struct MetaRegion(spa_sys::spa_meta_region);

impl MetaRegion {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_region {
        &self.0
    }

    pub fn region(&self) -> &Region {
        &self.0.region
    }

    pub fn position(&self) -> Point {
        self.0.region.position
    }

    pub fn size(&self) -> Rectangle {
        self.0.region.size
    }

    pub fn is_valid(&self) -> bool {
        unsafe { spa_sys::spa_meta_region_is_valid(self.as_raw()) }
    }
}

impl Debug for MetaRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaRegion")
            .field("region", &self.region())
            .finish()
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct MetaVideoCrop(MetaRegion);

impl MetaVideoCrop {
    pub fn meta_region(&self) -> &MetaRegion {
        &self.0
    }
}

impl Metadata for MetaVideoCrop {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoCrop;
}

#[repr(transparent)]
pub struct MetaVideoDamage(spa_sys::spa_meta);

impl MetaVideoDamage {
    pub fn as_raw(&self) -> &spa_sys::spa_meta {
        &self.0
    }

    pub fn iter(&self) -> VideoDamageIter<'_> {
        VideoDamageIter::new(self)
    }
}

pub struct VideoDamageIter<'a> {
    video_damage: &'a MetaVideoDamage,
    pos: *const spa_sys::spa_meta_region,
}

impl<'a> VideoDamageIter<'a> {
    fn new(video_damage: &'a MetaVideoDamage) -> Self {
        Self {
            video_damage,
            pos: unsafe { spa_sys::spa_meta_first(video_damage.as_raw()) }.cast(),
        }
    }
}

impl<'a> Iterator for VideoDamageIter<'a> {
    type Item = &'a MetaRegion;

    fn next(&mut self) -> Option<Self::Item> {
        if !unsafe { spa_sys::spa_meta_check(self.pos.cast(), self.video_damage.as_raw()) } {
            return None;
        }

        let region = unsafe { self.pos.cast::<MetaRegion>().as_ref()? };
        if !region.is_valid() {
            return None;
        }

        self.pos = unsafe { self.pos.add(1) };

        Some(region)
    }
}

impl Metadata for MetaVideoDamage {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoDamage;
}

#[repr(transparent)]
pub struct MetaBitmap(spa_sys::spa_meta_bitmap);

impl MetaBitmap {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_bitmap {
        &self.0
    }

    pub fn format(&self) -> VideoFormat {
        VideoFormat(self.0.format)
    }

    pub fn size(&self) -> Rectangle {
        self.0.size
    }

    pub fn stride(&self) -> i32 {
        self.0.stride
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn is_valid(&self) -> bool {
        unsafe { spa_sys::spa_meta_bitmap_is_valid(self.as_raw()) }
    }

    pub fn bitmap_data(&self) -> Option<&[u8]> {
        if !self.is_valid()
            || (self.0.offset as usize) < std::mem::size_of::<spa_sys::spa_meta_bitmap>()
        {
            return None;
        }

        let height = self.0.size.height as usize;
        let stride = self.0.stride.unsigned_abs() as usize;
        let data_size = height * stride;

        unsafe {
            let base_ptr = self as *const _ as *const u8;
            let data_ptr = base_ptr.add(self.0.offset as usize);
            Some(std::slice::from_raw_parts(data_ptr, data_size))
        }
    }
}

impl Debug for MetaBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaBitmap")
            .field("format", &self.format())
            .field("size", &self.size())
            .field("stride", &self.stride())
            .field("offset", &self.offset())
            .finish()
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

    pub fn position(&self) -> Point {
        self.0.position
    }

    /// This field has no meaning  when there is no valid bitmap.
    pub fn hotspot(&self) -> Point {
        self.0.hotspot
    }

    pub fn bitmap_offset(&self) -> u32 {
        self.0.bitmap_offset
    }

    pub fn is_valid(&self) -> bool {
        unsafe { spa_sys::spa_meta_cursor_is_valid(self.as_raw()) }
    }

    pub fn bitmap(&self) -> Option<&MetaBitmap> {
        if !self.is_valid()
            || (self.0.bitmap_offset as usize) < std::mem::size_of::<spa_sys::spa_meta_cursor>()
        {
            return None;
        }

        unsafe {
            let base_ptr = self as *const _ as *const u8;
            let bitmap_ptr = base_ptr.add(self.0.bitmap_offset as usize);
            let bitmap_ptr = bitmap_ptr as *const MetaBitmap;
            bitmap_ptr.as_ref()
        }
    }
}

impl Debug for MetaCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaCursor")
            .field("id", &self.id())
            .field("flags", &self.flags())
            .field("position", &self.position())
            .field("hotspot", &self.hotspot())
            .field("bitmap_offset", &self.bitmap_offset())
            .finish()
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

#[cfg(feature = "v0_3_21")]
#[derive(Clone)]
#[repr(transparent)]
pub struct MetaBusy(spa_sys::spa_meta_busy);

#[cfg(feature = "v0_3_21")]
impl MetaBusy {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_busy {
        &self.0
    }

    pub fn flags(&self) -> u32 {
        self.0.flags
    }

    pub fn count(&self) -> u32 {
        self.0.count
    }
}

#[cfg(feature = "v0_3_21")]
impl Debug for MetaBusy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaBusy")
            .field("flags", &self.flags())
            .field("count", &self.count())
            .finish()
    }
}

#[cfg(feature = "v0_3_21")]
impl Metadata for MetaBusy {
    const META_TYPE: u32 = spa_sys::SPA_META_Busy;
}

#[cfg(feature = "v0_3_62")]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct MetaVideoTransformValue(spa_sys::spa_meta_videotransform_value);

#[cfg(feature = "v0_3_62")]
impl MetaVideoTransformValue {
    pub const NONE: Self = Self(spa_sys::SPA_META_TRANSFORMATION_None);
    pub const ROTATED90: Self = Self(spa_sys::SPA_META_TRANSFORMATION_90);
    pub const ROTATED180: Self = Self(spa_sys::SPA_META_TRANSFORMATION_180);
    pub const ROTATED270: Self = Self(spa_sys::SPA_META_TRANSFORMATION_270);
    pub const FLIPPED: Self = Self(spa_sys::SPA_META_TRANSFORMATION_Flipped);
    pub const FLIPPED90: Self = Self(spa_sys::SPA_META_TRANSFORMATION_Flipped90);
    pub const FLIPPED180: Self = Self(spa_sys::SPA_META_TRANSFORMATION_Flipped180);
    pub const FLIPPED270: Self = Self(spa_sys::SPA_META_TRANSFORMATION_Flipped270);

    pub fn from_raw(raw: spa_sys::spa_meta_videotransform_value) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> spa_sys::spa_meta_videotransform_value {
        self.0
    }
}

#[cfg(feature = "v0_3_62")]
impl Debug for MetaVideoTransformValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MetaVideoTransformValue::{}",
            match *self {
                Self::NONE => "None",
                Self::ROTATED90 => "ROTATED90",
                Self::ROTATED180 => "ROTATED180",
                Self::ROTATED270 => "ROTATED270",
                Self::FLIPPED => "FLIPPED",
                Self::FLIPPED90 => "FLIPPED90",
                Self::FLIPPED180 => "FLIPPED180",
                Self::FLIPPED270 => "FLIPPED270",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "v0_3_62")]
#[repr(transparent)]
pub struct MetaVideoTransform(spa_sys::spa_meta_videotransform);

#[cfg(feature = "v0_3_62")]
impl MetaVideoTransform {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_videotransform {
        &self.0
    }

    pub fn transform(&self) -> MetaVideoTransformValue {
        MetaVideoTransformValue::from_raw(self.0.transform)
    }
}

#[cfg(feature = "v0_3_62")]
impl Debug for MetaVideoTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaVideoTransform")
            .field("transform", &self.transform())
            .finish()
    }
}

#[cfg(feature = "v0_3_62")]
impl Metadata for MetaVideoTransform {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoTransform;
}

#[cfg(feature = "v1_2_0")]
bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct SyncTimelineFlags: u32 {
        /// This flag is set by the producer and cleared by the consumer
        /// when it promises to signal the release point
        #[cfg(feature = "v1_6_0")]
        const UNSCHEDULED_RELEASE = spa_sys::SPA_META_SYNC_TIMELINE_UNSCHEDULED_RELEASE;

    }
}

#[cfg(feature = "v1_2_0")]
#[repr(transparent)]
pub struct MetaSyncTimeline(spa_sys::spa_meta_sync_timeline);

#[cfg(feature = "v1_2_0")]
impl MetaSyncTimeline {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_sync_timeline {
        &self.0
    }

    pub fn flags(&self) -> SyncTimelineFlags {
        SyncTimelineFlags::from_bits_retain(self.0.flags)
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

#[cfg(feature = "v1_2_0")]
impl Debug for MetaSyncTimeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaSyncTimeline")
            .field("flags", &self.flags())
            .field("padding", &self.padding())
            .field("acquire_point", &self.acquire_point())
            .field("release_point", &self.release_point())
            .finish()
    }
}

#[cfg(feature = "v1_2_0")]
impl Metadata for MetaSyncTimeline {
    const META_TYPE: u32 = spa_sys::SPA_META_SyncTimeline;
}
