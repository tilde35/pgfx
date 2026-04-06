use crate::*;

pub trait BackendLoadOptions<B: Backend> {
    fn create_load_options(self) -> Result<B::LoadOptions, B::Error>;
}

/// Flags that describe how a buffer will be used in rendering or compute operations.
///
/// These usage flags are intended as general-purpose flags that can be interpreted by most any backend. For example, a DirectX
/// backend might interpret `UsageFlags::RENDER_WRITE` for a color texture as `D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UsageFlags(pub u32);
impl UsageFlags {
    pub const NONE: Self = UsageFlags(0);

    /// Indicates that the buffer will be written to during rendering.
    ///
    /// This flag is typically used for buffers that are used as render targets or depth/stencil buffers.
    pub const RENDER_WRITE: Self = UsageFlags(1 << 0);
    /// Indicates that the buffer will be read from during rendering.
    pub const RENDER_READ: Self = UsageFlags(1 << 1);

    /// Indicates that the buffer will be written to during compute operations.
    pub const COMPUTE_WRITE: Self = UsageFlags(1 << 2);
    /// Indicates that the buffer will be read from during compute operations.
    pub const COMPUTE_READ: Self = UsageFlags(1 << 3);

    /// Indicates that the buffer will be used as a source for copy operations.
    pub const COPY_SRC: Self = UsageFlags(1 << 4);
    /// Indicates that the buffer will be used as a destination for copy operations.
    pub const COPY_DST: Self = UsageFlags(1 << 5);

    pub const fn new(
        render_read: bool,
        render_write: bool,
        compute_read: bool,
        compute_write: bool,
    ) -> Self {
        let mut flags = UsageFlags(0);
        if render_read {
            flags.0 |= Self::RENDER_READ.0;
        }
        if render_write {
            flags.0 |= Self::RENDER_WRITE.0;
        }
        if compute_read {
            flags.0 |= Self::COMPUTE_READ.0;
        }
        if compute_write {
            flags.0 |= Self::COMPUTE_WRITE.0;
        }
        flags
    }

    pub const fn contains_any(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
    pub const fn contains_all(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}
impl std::ops::BitOr for UsageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        UsageFlags(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd for UsageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        UsageFlags(self.0 & rhs.0)
    }
}
impl std::ops::BitOrAssign for UsageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl std::ops::BitAndAssign for UsageFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl<B: Backend> BackendLoadOptions<B> for UsageFlags {
    fn create_load_options(self) -> Result<B::LoadOptions, B::Error> {
        B::usage_flags_as_options(self)
    }
}
