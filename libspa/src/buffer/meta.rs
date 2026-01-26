use crate::param::video::VideoFormat;
use crate::utils::{Point, Rectangle};

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
pub struct MetaRegion(spa_sys::spa_meta_region);

impl MetaRegion {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_region {
        &self.0
    }

    pub fn region(&self) -> &spa_sys::spa_region {
        &self.0.region
    }

    pub fn position(&self) -> Point {
        self.0.region.position
    }

    pub fn size(&self) -> Rectangle {
        self.0.region.size
    }
}

#[repr(transparent)]
pub struct VideoCrop(MetaRegion);

impl VideoCrop {
    pub fn meta_region(&self) -> &MetaRegion {
        &self.0
    }
}

impl Metadata for VideoCrop {
    const META_TYPE: u32 = spa_sys::SPA_META_VideoCrop;
}

pub struct DamageRegion<'a>(&'a spa_sys::spa_meta_region);

impl<'a> DamageRegion<'a> {
    pub fn position(&self) -> Point {
        self.0.region.position
    }

    pub fn size(&self) -> Rectangle {
        Rectangle {
            width: self.0.region.size.width,
            height: self.0.region.size.height,
        }
    }
}

#[repr(transparent)]
pub struct VideoDamage(spa_sys::spa_meta);

impl Metadata for VideoDamage {
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
        self.0.id != 0
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
pub struct MetaBusy(spa_sys::spa_meta_busy);

impl MetaBusy {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_busy {
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

mod sys {
    pub type VideoTransform = u32;

    pub const SPA_VIDEO_TRANSFORM_IDENTITY: VideoTransform = 0;
    pub const SPA_VIDEO_TRANSFORM_ROT90: VideoTransform = 1;
    pub const SPA_VIDEO_TRANSFORM_ROT180: VideoTransform = 2;
    pub const SPA_VIDEO_TRANSFORM_ROT270: VideoTransform = 3;
    pub const SPA_VIDEO_TRANSFORM_FLIP_X: VideoTransform = 4;
    pub const SPA_VIDEO_TRANSFORM_FLIP_Y: VideoTransform = 5;
    pub const SPA_VIDEO_TRANSFORM_FLIP_XY: VideoTransform = 6;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VideoTransform(sys::VideoTransform);

impl VideoTransform {
    pub const IDENTITY: Self = Self(sys::SPA_VIDEO_TRANSFORM_IDENTITY);
    pub const ROT90: Self = Self(sys::SPA_VIDEO_TRANSFORM_ROT90);
    pub const ROT180: Self = Self(sys::SPA_VIDEO_TRANSFORM_ROT180);
    pub const ROT270: Self = Self(sys::SPA_VIDEO_TRANSFORM_ROT270);
    pub const FLIP_X: Self = Self(sys::SPA_VIDEO_TRANSFORM_FLIP_X);
    pub const FLIP_Y: Self = Self(sys::SPA_VIDEO_TRANSFORM_FLIP_Y);
    pub const FLIP_XY: Self = Self(sys::SPA_VIDEO_TRANSFORM_FLIP_XY);

    pub fn from_raw(raw: sys::VideoTransform) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> sys::VideoTransform {
        self.0
    }
}

impl std::fmt::Debug for VideoTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IDENTITY => "IDENTITY",
            Self::ROT90 => "ROT90",
            Self::ROT180 => "ROT180",
            Self::ROT270 => "ROT270",
            Self::FLIP_X => "FLIP_X",
            Self::FLIP_Y => "FLIP_Y",
            Self::FLIP_XY => "FLIP_XY",
            _ => "UNKNOWN",
        };
        write!(f, "VideoTransform::{}", name)
    }
}

#[repr(transparent)]
pub struct MetaVideoTransform(spa_sys::spa_meta_videotransform);

impl MetaVideoTransform {
    pub fn as_raw(&self) -> &spa_sys::spa_meta_videotransform {
        &self.0
    }

    pub fn transform(&self) -> VideoTransform {
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
