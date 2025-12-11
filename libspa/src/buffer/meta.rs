use spa_sys::spa_meta_busy;

use crate::param::video::VideoFormat;
use crate::utils::Rectangle;

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

pub struct DamageRegion<'a>(&'a spa_sys::spa_meta_region);

impl<'a> DamageRegion<'a> {
    pub fn position(&self) -> (i32, i32) {
        (self.0.region.position.x, self.0.region.position.y)
    }
    pub fn size(&self) -> (u32, u32) {
        (self.0.region.size.width, self.0.region.size.height)
    }
}

#[repr(transparent)]
pub struct VideoDamage<'a>(&'a [spa_sys::spa_meta_region]);

impl<'a> VideoDamage<'a> {
    pub fn as_raw(&self) -> &[spa_sys::spa_meta_region] {
        &self.0
    }

    /// Returns the number of damage regions
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the damage regions
    pub fn iter(&self) -> impl Iterator<Item = DamageRegion<'a>> + '_ {
        self.0.iter().map(|region| DamageRegion(region))
    }

    /// Returns the coordinates and size of the region at the given index
    pub fn region(&self, index: usize) -> Option<DamageRegion<'a>> {
        self.0.get(index).map(|r| DamageRegion(r))
    }
}

impl Metadata for VideoDamage<'_> {
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
        Rectangle {
            width: self.0.size.width,
            height: self.0.size.height,
        }
    }

    pub fn stride(&self) -> i32 {
        self.0.stride
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn is_valid(&self) -> bool {
        self.0.format != 0
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
pub struct MetaBusy(spa_meta_busy);

impl MetaBusy {
    pub fn as_raw(&self) -> &spa_meta_busy {
        &self.0
    }

    pub fn flag(&self) -> u32 {
        self.0.flags
    }

    pub fn count(&self) -> u32 {
        self.0.count
    }
}

impl Metadata for MetaBusy {
    const META_TYPE: u32 = spa_sys::SPA_META_Busy;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum VideoTransform {
    Identity = 0,
    Rot90 = 1,
    Rot180 = 2,
    Rot270 = 3,
    FlipX = 4,
    FlipY = 5,
    FlipXY = 6,
}

impl VideoTransform {
    pub fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Identity),
            1 => Some(Self::Rot90),
            2 => Some(Self::Rot180),
            3 => Some(Self::Rot270),
            4 => Some(Self::FlipX),
            5 => Some(Self::FlipY),
            6 => Some(Self::FlipXY),
            _ => None,
        }
    }
}

#[repr(transparent)]
pub struct MetaVideoTransform(spa_sys::spa_meta_videotransform);

impl MetaVideoTransform {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_videotransform {
        &self.0
    }

    pub fn transform(&self) -> Option<VideoTransform> {
        VideoTransform::from_raw(self.0.transform)
    }
}

impl Metadata for MetaVideoTransform {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoTransform;
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct SyncTimelineFlags: u32 {
        /// This flag is set by the producer and cleared by the consumer
        /// when it promises to signal the release point
        const UNSCHEDULED_RELEASE = 1 << 0;
    }
}

#[repr(transparent)]
pub struct MetaSyncTimeline(spa_sys::spa_meta_sync_timeline);

impl MetaSyncTimeline {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_sync_timeline {
        &self.0
    }

    pub fn flags(&self) -> SyncTimelineFlags {
        SyncTimelineFlags::from_bits_truncate(self.0.flags)
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
