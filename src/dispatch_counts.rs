use crate::*;

/// Defines the ranges to use within the vertex/instance/index buffers for rendering operations.
///
/// This is not a required parameter for rendering. If omitted, the complete buffer ranges will be used.
///
/// Example usage:
/// ```
/// use pgfx::RenderRanges;
/// // Use the first 6 indices, all instances, and a vertex offset of 0.
/// let ranges = RenderRanges::indexed(0..6u32, ..);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RenderRanges {
    vertex_index_range: RenderRange,
    instance_range: RenderRange,
    vertex_offset: i32,
    is_indexed: bool,
}
impl RenderRanges {
    pub fn indexed(
        index_range: impl Into<RenderRange>,
        instance_range: impl Into<RenderRange>,
    ) -> Self {
        RenderRanges {
            vertex_index_range: index_range.into(),
            instance_range: instance_range.into(),
            vertex_offset: 0,
            is_indexed: true,
        }
    }

    pub fn indexed_with_offset(
        index_range: impl Into<RenderRange>,
        instance_range: impl Into<RenderRange>,
        vertex_offset: i32,
    ) -> Self {
        RenderRanges {
            vertex_index_range: index_range.into(),
            instance_range: instance_range.into(),
            vertex_offset: vertex_offset,
            is_indexed: true,
        }
    }

    pub fn non_indexed(vertex_range: impl Into<RenderRange>) -> Self {
        RenderRanges {
            vertex_index_range: vertex_range.into(),
            instance_range: RenderRange::full(),
            vertex_offset: 0,
            is_indexed: false,
        }
    }

    pub fn non_indexed_instances(
        vertex_range: impl Into<RenderRange>,
        instance_range: impl Into<RenderRange>,
    ) -> Self {
        RenderRanges {
            vertex_index_range: vertex_range.into(),
            instance_range: instance_range.into(),
            vertex_offset: 0,
            is_indexed: false,
        }
    }

    pub fn is_indexed(&self) -> bool {
        self.is_indexed
    }

    pub fn vertex_range(&self, vertex_buffer_len: u32) -> RenderRange {
        if self.is_indexed() {
            // Does not apply to indexed rendering
            RenderRange::start_and_end(0, vertex_buffer_len)
        } else {
            self.vertex_index_range.range_for_buffer(vertex_buffer_len)
        }
    }
    pub fn unbound_vertex_range(&self) -> Option<RenderRange> {
        if self.vertex_index_range == RenderRange::full() {
            None
        } else {
            Some(self.vertex_index_range)
        }
    }
    pub fn index_range(&self, index_buffer_len: u32) -> RenderRange {
        if !self.is_indexed() {
            // Does not apply to non-indexed rendering
            RenderRange::start_and_end(0, index_buffer_len)
        } else {
            self.vertex_index_range.range_for_buffer(index_buffer_len)
        }
    }
    pub fn instance_range(&self, instance_buffer_len: u32) -> RenderRange {
        self.instance_range.range_for_buffer(instance_buffer_len)
    }
    pub fn vertex_offset(&self) -> i32 {
        self.vertex_offset
    }
}
impl<B: Backend> DeviceLoad<B> for RenderRanges {
    type LoadAs = Self;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_render_ranges(data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            d.reload_render_ranges(data[0], dest.stored_data())
        }
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_render_ranges(data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RenderRange {
    start: u32,
    len: u32,
}
impl RenderRange {
    pub const fn full() -> Self {
        RenderRange {
            start: 0,
            len: u32::MAX,
        }
    }

    pub const fn start_and_len(start: u32, len: u32) -> Self {
        RenderRange { start, len }
    }

    pub const fn start_and_end(start: u32, end: u32) -> Self {
        RenderRange {
            start,
            len: end - start,
        }
    }

    pub fn range_for_buffer(&self, buffer_len: u32) -> Self {
        let expected_len = buffer_len.saturating_sub(self.start);
        RenderRange {
            start: self.start,
            len: self.len.min(expected_len),
        }
    }

    pub fn start(&self) -> u32 {
        self.start
    }
    pub fn end(&self) -> u32 {
        self.start + self.len
    }
    pub fn len(&self) -> u32 {
        self.len
    }
}

/// ComputeWorkgroup is a structure that defines the size of a compute workgroup.
/// It is used to specify how many threads will be used in a compute shader for processing
/// a given quantity of data. The `x`, `y`, and `z` fields represent the dimensions of the
/// workgroup in 3D space.
///
/// Choosing the right workgroup size is crucial for performance, but is beyond the scope
/// of this documentation.
#[derive(Debug, Clone, Copy)]
pub struct ComputeWorkgroup {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl ComputeWorkgroup {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn for_quantity(total_quantity: u32, quantity_per_group: u32) -> Self {
        let work_group_count =
            ((total_quantity as f64) / (quantity_per_group as f64)).ceil() as u32;

        Self {
            x: work_group_count,
            y: 1,
            z: 1,
        }
    }
}
impl<B: Backend> DeviceLoad<B> for ComputeWorkgroup {
    type LoadAs = Self;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_compute_workgroup(data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            d.reload_compute_workgroup(data[0], dest.stored_data())
        }
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_compute_workgroup(data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }
}

/*
================================================================================
The `From` trait implementations for `RenderRange`.
================================================================================
*/
impl From<std::ops::RangeFull> for RenderRange {
    fn from(_: std::ops::RangeFull) -> Self {
        RenderRange {
            start: 0,
            len: u32::MAX,
        }
    }
}
impl From<std::ops::RangeTo<u16>> for RenderRange {
    fn from(range: std::ops::RangeTo<u16>) -> Self {
        RenderRange {
            start: 0,
            len: range.end as u32,
        }
    }
}
impl From<std::ops::RangeTo<i32>> for RenderRange {
    fn from(range: std::ops::RangeTo<i32>) -> Self {
        assert!(range.end >= 0, "Range end must not be negative");
        RenderRange {
            start: 0,
            len: range.end as u32,
        }
    }
}
impl From<std::ops::RangeTo<u32>> for RenderRange {
    fn from(range: std::ops::RangeTo<u32>) -> Self {
        RenderRange {
            start: 0,
            len: range.end,
        }
    }
}
impl From<std::ops::RangeTo<usize>> for RenderRange {
    fn from(range: std::ops::RangeTo<usize>) -> Self {
        RenderRange {
            start: 0,
            len: range.end as u32,
        }
    }
}
impl From<std::ops::RangeFrom<u16>> for RenderRange {
    fn from(range: std::ops::RangeFrom<u16>) -> Self {
        RenderRange {
            start: range.start as u32,
            len: u32::MAX,
        }
    }
}
impl From<std::ops::RangeFrom<i32>> for RenderRange {
    fn from(range: std::ops::RangeFrom<i32>) -> Self {
        assert!(range.start >= 0, "Range start must not be negative");
        RenderRange {
            start: range.start as u32,
            len: u32::MAX,
        }
    }
}
impl From<std::ops::RangeFrom<u32>> for RenderRange {
    fn from(range: std::ops::RangeFrom<u32>) -> Self {
        RenderRange {
            start: range.start,
            len: u32::MAX,
        }
    }
}
impl From<std::ops::RangeFrom<usize>> for RenderRange {
    fn from(range: std::ops::RangeFrom<usize>) -> Self {
        RenderRange {
            start: range.start as u32,
            len: u32::MAX,
        }
    }
}
impl From<std::ops::Range<u16>> for RenderRange {
    fn from(range: std::ops::Range<u16>) -> Self {
        RenderRange {
            start: range.start as u32,
            len: (range.end - range.start) as u32,
        }
    }
}
impl From<std::ops::Range<i32>> for RenderRange {
    fn from(range: std::ops::Range<i32>) -> Self {
        assert!(range.start >= 0, "Range start must not be negative");
        assert!(range.end >= 0, "Range end must not be negative");
        RenderRange {
            start: range.start as u32,
            len: (range.end - range.start) as u32,
        }
    }
}
impl From<std::ops::Range<u32>> for RenderRange {
    fn from(range: std::ops::Range<u32>) -> Self {
        RenderRange {
            start: range.start,
            len: range.end - range.start,
        }
    }
}
impl From<std::ops::Range<usize>> for RenderRange {
    fn from(range: std::ops::Range<usize>) -> Self {
        RenderRange {
            start: range.start as u32,
            len: (range.end - range.start) as u32,
        }
    }
}
