pub use crate::*;

pub trait PixelType<B: Backend>: Sized {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum PixelFormat {
    /// Standard 8-bit RGBA format. This is the most common format for images and will be automatically gamma-corrected into
    /// linear color space when read.
    ///
    /// If linear color space is needed, then use `PixelFormat::V4(PixelChannel::NormU8)` instead.
    ///
    /// Corresponding type: `pixel::Srgba`
    Srgba,
    /// Standard 8-bit RGB format. This is the most common format for images and will be automatically gamma-corrected into
    /// linear color space when read.
    ///
    /// If linear color space is needed (ex. for normal maps), then use `PixelFormat::V3(PixelChannel::NormU8)` instead.
    ///
    /// Corresponding type: `pixel::Srgb`
    Srgb,

    /// This is used to represent the preferred depth format for the backend device. In general, this will be 24-bits or more.
    ///
    /// Corresponding type: `pixel::Depth`
    Depth,
    /// This is used to represent the preferred depth-stencil format for the backend device. In general, this will be 24-bits or
    /// more for depth and 8-bits for stencil.
    ///
    /// Corresponding type: `pixel::DepthStencil`
    DepthStencil,
    /// This represents an 8-bit stencil format.
    ///
    /// Corresponding type: `pixel::Stencil8`
    Stencil8,

    /// A single channel format using the specified channel type. For example, `PixelFormat::V1(PixelChannel::NormU8)` would be
    /// a single 8-bit normalized value.
    ///
    /// Corresponding type: `ChannelType` (see `PixelChannel`)
    V1(PixelChannel),
    /// A two channel format using the specified channel type.
    ///
    /// Corresponding type: `[ChannelType; 2]` (see `PixelChannel`)
    V2(PixelChannel),
    /// A three channel format using the specified channel type.
    ///
    /// Corresponding type: `[ChannelType; 3]` (see `PixelChannel`)
    V3(PixelChannel),
    /// A four channel format using the specified channel type.
    ///
    /// Corresponding type: `[ChannelType; 4]` (see `PixelChannel`)
    V4(PixelChannel),
}
impl PixelFormat {
    pub const fn get_width(&self) -> u32 {
        match self {
            Self::Srgba => 4,
            Self::Srgb => 3,
            Self::Depth => 4,
            Self::DepthStencil => 4,
            Self::Stencil8 => 1,
            Self::V1(c) => c.get_width(),
            Self::V2(c) => c.get_width() * 2,
            Self::V3(c) => c.get_width() * 3,
            Self::V4(c) => c.get_width() * 4,
        }
    }

    pub const fn is_depth_or_stencil(&self) -> bool {
        match self {
            PixelFormat::Depth | PixelFormat::DepthStencil | PixelFormat::Stencil8 => true,
            PixelFormat::Srgba
            | PixelFormat::Srgb
            | PixelFormat::V1(..)
            | PixelFormat::V2(..)
            | PixelFormat::V3(..)
            | PixelFormat::V4(..) => false,
        }
    }
    pub const fn has_depth(&self) -> bool {
        match self {
            PixelFormat::Depth | PixelFormat::DepthStencil => true,
            PixelFormat::Stencil8
            | PixelFormat::Srgba
            | PixelFormat::Srgb
            | PixelFormat::V1(..)
            | PixelFormat::V2(..)
            | PixelFormat::V3(..)
            | PixelFormat::V4(..) => false,
        }
    }
    pub const fn has_stencil(&self) -> bool {
        match self {
            PixelFormat::Stencil8 | PixelFormat::DepthStencil => true,
            PixelFormat::Depth
            | PixelFormat::Srgba
            | PixelFormat::Srgb
            | PixelFormat::V1(..)
            | PixelFormat::V2(..)
            | PixelFormat::V3(..)
            | PixelFormat::V4(..) => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum PixelChannel {
    /// U8 value which converted from 0 to 255 into 0.0 to 1.0.
    ///
    /// Corresponding type: `pixel::UNorm8`
    UNorm8,
    /// I8 value which converted from -127 to 127 into -1.0 to 1.0. Note that -128 and -127 both map to -1.0.
    ///
    /// Corresponding type: `pixel::SNorm8`
    SNorm8,
    /// U16 value which converted from 0 to 65535 into 0.0 to 1.0.
    ///
    /// Corresponding type: `pixel::UNorm16`
    UNorm16,
    /// I16 value which converted from -32767 to 32767 into -1.0 to 1.0. Note that -32768 and -32767 both map to -1.0.
    ///
    /// Corresponding type: `pixel::SNorm16`
    SNorm16,

    /// A 16-bit floating point value.
    ///
    /// Corresponding type: `pixel::Float16`
    F16,
    /// A 32-bit floating point value.
    ///
    /// Corresponding type: `f32`
    F32,
}
impl PixelChannel {
    pub const fn get_width(&self) -> u32 {
        match self {
            Self::UNorm8 => 1,
            Self::SNorm8 => 1,
            Self::UNorm16 => 2,
            Self::SNorm16 => 2,
            Self::F16 => 2,
            Self::F32 => 4,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Srgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Srgb {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        LrgbF32::from_hsl(hue, saturation, lightness).to_srgb()
    }
    pub fn to_hsl(&self) -> [f32; 3] {
        self.to_lrgb().to_hsl()
    }

    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        LrgbF32::from_hsv(hue, saturation, value).to_srgb()
    }
    pub fn to_hsv(&self) -> [f32; 3] {
        self.to_lrgb().to_hsv()
    }

    pub const fn to_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub const fn to_srgba(&self) -> Srgba {
        Srgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 255,
        }
    }

    pub fn to_lrgb(&self) -> LrgbF32 {
        LrgbF32 {
            r: convert_std_channel_to_linear(self.r),
            g: convert_std_channel_to_linear(self.g),
            b: convert_std_channel_to_linear(self.b),
        }
    }

    pub fn to_lrgba(&self) -> LrgbaF32 {
        LrgbaF32 {
            r: convert_std_channel_to_linear(self.r),
            g: convert_std_channel_to_linear(self.g),
            b: convert_std_channel_to_linear(self.b),
            a: 1.0,
        }
    }
}
impl<B: Backend> PixelType<B> for Srgb {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::Srgb)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Srgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Srgba {
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    pub fn from_hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        LrgbaF32::from_hsla(hue, saturation, lightness, alpha).to_srgba()
    }
    pub fn to_hsla(&self) -> [f32; 4] {
        self.to_lrgba().to_hsla()
    }

    pub fn from_hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        LrgbaF32::from_hsva(hue, saturation, value, alpha).to_srgba()
    }
    pub fn to_hsva(&self) -> [f32; 4] {
        self.to_lrgba().to_hsva()
    }

    pub fn from_linear_rgba(color: [UNorm8; 4]) -> Self {
        LrgbaF32::from_linear_rgba(color).to_srgba()
    }

    pub const fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub const fn to_srgb(&self) -> Srgb {
        Srgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub fn to_lrgb(&self) -> LrgbF32 {
        LrgbF32 {
            r: convert_std_channel_to_linear(self.r),
            g: convert_std_channel_to_linear(self.g),
            b: convert_std_channel_to_linear(self.b),
        }
    }

    pub fn to_lrgba(&self) -> LrgbaF32 {
        LrgbaF32 {
            r: convert_std_channel_to_linear(self.r),
            g: convert_std_channel_to_linear(self.g),
            b: convert_std_channel_to_linear(self.b),
            a: self.a as f32 / 255.0,
        }
    }
}
impl<B: Backend> PixelType<B> for Srgba {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::Srgba)
    }
}
impl From<[u8; 4]> for Srgba {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}

/// Linear-space RGB color with 32-bit floating point channels. For standard RGB colors, use `Srgb` instead.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct LrgbF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl LrgbF32 {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub const fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let h = hue / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let (r1, g1, b1) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let m = lightness - c / 2.0;
        Self {
            r: r1 + m,
            g: g1 + m,
            b: b1 + m,
        }
    }
    pub const fn to_hsl(&self) -> [f32; 3] {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (((self.g - self.b) / delta) % 6.0)
        } else if max == self.g {
            60.0 * (((self.b - self.r) / delta) + 2.0)
        } else {
            60.0 * (((self.r - self.g) / delta) + 4.0)
        };

        let lightness = (max + min) / 2.0;

        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        [hue, saturation, lightness]
    }

    pub const fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let c = value * saturation;
        let h = hue / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let (r1, g1, b1) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let m = value - c;
        Self {
            r: r1 + m,
            g: g1 + m,
            b: b1 + m,
        }
    }
    pub const fn to_hsv(&self) -> [f32; 3] {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (((self.g - self.b) / delta) % 6.0)
        } else if max == self.g {
            60.0 * (((self.b - self.r) / delta) + 2.0)
        } else {
            60.0 * (((self.r - self.g) / delta) + 4.0)
        };

        let value = max;

        let saturation = if max == 0.0 { 0.0 } else { delta / max };

        [hue, saturation, value]
    }

    pub const fn to_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn to_srgb(&self) -> Srgb {
        Srgb {
            r: convert_linear_channel_to_std(self.r),
            g: convert_linear_channel_to_std(self.g),
            b: convert_linear_channel_to_std(self.b),
        }
    }

    pub fn to_srgba(&self) -> Srgba {
        Srgba {
            r: convert_linear_channel_to_std(self.r),
            g: convert_linear_channel_to_std(self.g),
            b: convert_linear_channel_to_std(self.b),
            a: 255,
        }
    }

    pub const fn to_lrgba(&self) -> LrgbaF32 {
        LrgbaF32 {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 1.0,
        }
    }

    pub const fn to_unorm8(&self) -> [UNorm8; 3] {
        [
            UNorm8::new(self.r),
            UNorm8::new(self.g),
            UNorm8::new(self.b),
        ]
    }
}
impl<B: Backend> PixelType<B> for LrgbF32 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::F32))
    }
}
impl Into<[f32; 3]> for LrgbF32 {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}
impl Into<LrgbF32> for [f32; 3] {
    fn into(self) -> LrgbF32 {
        LrgbF32 {
            r: self[0],
            g: self[1],
            b: self[2],
        }
    }
}
impl DeviceValueType for LrgbF32 {
    fn device_value_type() -> ValueType {
        ValueType::V3(PrimitiveType::F32)
    }
}

/// Linear-space RGBA color with 32-bit floating point channels. For standard RGBA colors, use `Srgba` instead.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct LrgbaF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl LrgbaF32 {
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        let c = LrgbF32::from_hsl(hue, saturation, lightness);
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: alpha,
        }
    }
    pub const fn to_hsla(&self) -> [f32; 4] {
        let hsl = self.to_lrgb().to_hsl();
        [hsl[0], hsl[1], hsl[2], self.a]
    }

    pub const fn from_hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        let c = LrgbF32::from_hsv(hue, saturation, value);
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: alpha,
        }
    }
    pub const fn to_hsva(&self) -> [f32; 4] {
        let hsv = self.to_lrgb().to_hsv();
        [hsv[0], hsv[1], hsv[2], self.a]
    }

    pub const fn from_linear_rgba(color: [UNorm8; 4]) -> Self {
        Self {
            r: color[0].get(),
            g: color[1].get(),
            b: color[2].get(),
            a: color[3].get(),
        }
    }

    pub const fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_srgb(&self) -> Srgb {
        Srgb {
            r: convert_linear_channel_to_std(self.r),
            g: convert_linear_channel_to_std(self.g),
            b: convert_linear_channel_to_std(self.b),
        }
    }

    pub fn to_srgba(&self) -> Srgba {
        Srgba {
            r: convert_linear_channel_to_std(self.r),
            g: convert_linear_channel_to_std(self.g),
            b: convert_linear_channel_to_std(self.b),
            a: (self.a * 255.0).round().clamp(0.0, 255.0) as u8,
        }
    }

    pub const fn to_lrgb(&self) -> LrgbF32 {
        LrgbF32 {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub const fn to_unorm8(&self) -> [UNorm8; 4] {
        [
            UNorm8::new(self.r),
            UNorm8::new(self.g),
            UNorm8::new(self.b),
            UNorm8::new(self.a),
        ]
    }
}
impl<B: Backend> PixelType<B> for LrgbaF32 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::F32))
    }
}
impl Into<[f32; 4]> for LrgbaF32 {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
impl Into<LrgbaF32> for [f32; 4] {
    fn into(self) -> LrgbaF32 {
        LrgbaF32 {
            r: self[0],
            g: self[1],
            b: self[2],
            a: self[3],
        }
    }
}
impl DeviceValueType for LrgbaF32 {
    fn device_value_type() -> ValueType {
        ValueType::V4(PrimitiveType::F32)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Float16(pub u16);
impl Float16 {
    // IMPORTANT: The conversion from f32 to f16 was LLM-generated and has not yet been tested.

    pub const fn new(val: f32) -> Self {
        let bits = val.to_bits(); // Get IEEE 754 binary representation of f32
        let sign = (bits >> 16) & 0x8000; // Extract sign bit
        let exp = ((bits >> 23) & 0xFF) as i32; // Extract exponent
        let mantissa = bits & 0x7FFFFF; // Extract mantissa

        if exp == 0xFF {
            // Handle NaN and infinity
            if mantissa != 0 {
                return Self((sign | 0x7E00) as u16); // Convert NaN to half-float NaN
            }
            return Self((sign | 0x7C00) as u16); // Convert infinity
        }

        let new_exp = exp - 127 + 15; // Convert exponent bias (127 for f32, 15 for f16)
        if new_exp >= 0x1F {
            // Overflow: return infinity
            return Self((sign | 0x7C00) as u16);
        }
        if new_exp <= 0 {
            // Handle subnormals and underflow
            if new_exp < -10 {
                return Self(sign as u16); // Too small, return zero
            }
            let mantissa = (mantissa | 0x800000) >> (1 - new_exp);
            return Self((sign | (mantissa >> 13)) as u16);
        }

        // Normal case: just shift mantissa and pack the bits
        let new_mantissa = mantissa >> 13;
        Self((sign as u16) | ((new_exp as u16) << 10) | (new_mantissa as u16))
    }

    pub const fn get(&self) -> f32 {
        let val = self.0;
        let sign = ((val & 0x8000) as u32) << 16;
        let exp = ((val & 0x7C00) >> 10) as i32;
        let mantissa = (val & 0x03FF) as u32;

        if exp == 0 {
            if mantissa == 0 {
                return f32::from_bits(sign);
            }
            let exp = 127 - 15 + 1;
            let mantissa = mantissa << 13;
            return f32::from_bits(sign | (exp << 23) as u32 | mantissa);
        } else if exp == 0x1F {
            return f32::from_bits(sign | 0x7F800000 | (mantissa << 13));
        }

        let exp = exp + (127 - 15);
        let mantissa = mantissa << 13;
        f32::from_bits(sign | ((exp as u32) << 23) | mantissa)
    }
}

/// Standard 32-bit float format.
impl<B: Backend> PixelType<B> for f32 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V1(PixelChannel::F32))
    }
}
impl<B: Backend> PixelType<B> for [f32; 2] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V2(PixelChannel::F32))
    }
}
impl<B: Backend> PixelType<B> for [f32; 3] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::F32))
    }
}
impl<B: Backend> PixelType<B> for [f32; 4] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::F32))
    }
}

/// Unsigned normalized 8-bit integer.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct UNorm8(pub u8);
impl UNorm8 {
    pub const fn new(f: f32) -> Self {
        Self((f * 255.0).round().clamp(0.0, 255.0) as u8)
    }
    pub const fn get(&self) -> f32 {
        self.0 as f32 / 255.0
    }
}
impl<B: Backend> PixelType<B> for UNorm8 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V1(PixelChannel::UNorm8))
    }
}
impl<B: Backend> PixelType<B> for [UNorm8; 2] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V2(PixelChannel::UNorm8))
    }
}
impl<B: Backend> PixelType<B> for [UNorm8; 3] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::UNorm8))
    }
}
impl<B: Backend> PixelType<B> for [UNorm8; 4] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::UNorm8))
    }
}

/// Signed normalized 8-bit integer.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct SNorm8(pub i8);
impl SNorm8 {
    pub const fn new(f: f32) -> Self {
        Self((f * 127.0).round().clamp(-127.0, 127.0) as i8)
    }
    pub const fn get(&self) -> f32 {
        // Note: The range of i8 is -128 to 127, so we need to convert it to the range -1.0 to 1.0.
        // The `max` is required due to the fact that `-128.0 / 127.0` is less than `-1.0`.
        (self.0 as f32 / 127.0).max(-1.0)
    }
}
impl<B: Backend> PixelType<B> for SNorm8 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V1(PixelChannel::SNorm8))
    }
}
impl<B: Backend> PixelType<B> for [SNorm8; 2] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V2(PixelChannel::SNorm8))
    }
}
impl<B: Backend> PixelType<B> for [SNorm8; 3] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::SNorm8))
    }
}
impl<B: Backend> PixelType<B> for [SNorm8; 4] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::SNorm8))
    }
}

/// Unsigned normalized 16-bit integer.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct UNorm16(pub u16);
impl UNorm16 {
    pub const fn new(f: f32) -> Self {
        Self((f * 65535.0).round().clamp(0.0, 65535.0) as u16)
    }
    pub const fn get(&self) -> f32 {
        self.0 as f32 / 65535.0
    }
}
impl<B: Backend> PixelType<B> for UNorm16 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V1(PixelChannel::UNorm16))
    }
}
impl<B: Backend> PixelType<B> for [UNorm16; 2] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V2(PixelChannel::UNorm16))
    }
}
impl<B: Backend> PixelType<B> for [UNorm16; 3] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::UNorm16))
    }
}
impl<B: Backend> PixelType<B> for [UNorm16; 4] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::UNorm16))
    }
}

/// Signed normalized 16-bit integer.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct SNorm16(pub i16);
impl SNorm16 {
    pub const fn new(f: f32) -> Self {
        Self((f * 32767.0).round().clamp(-32767.0, 32767.0) as i16)
    }
    pub const fn get(&self) -> f32 {
        // Note: The range of i16 is -32768 to 32767, so we need to convert it to the range -1.0 to 1.0.
        // The `max` is required due to the fact that `-32768.0 / 32767.0` is less than `-1.0`.
        (self.0 as f32 / 32767.0).max(-1.0)
    }
}
impl<B: Backend> PixelType<B> for SNorm16 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V1(PixelChannel::SNorm16))
    }
}
impl<B: Backend> PixelType<B> for [SNorm16; 2] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V2(PixelChannel::SNorm16))
    }
}
impl<B: Backend> PixelType<B> for [SNorm16; 3] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V3(PixelChannel::SNorm16))
    }
}
impl<B: Backend> PixelType<B> for [SNorm16; 4] {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::V4(PixelChannel::SNorm16))
    }
}

/// Represents a depth value using the device's preferred depth format. Note that this may not be a 32-bit float on the device.
/// For example, on some devices, this may be a 24-bit value.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Depth(pub f32);
impl<B: Backend> PixelType<B> for Depth {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::Depth)
    }
}

/// Represents a depth-stencil value using the device's preferred depth-stencil format. Note that this may not be a 32-bit float
/// on the device. For example, on some devices, this may be a 24-bit value for depth and an 8-bit value for stencil.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct DepthStencil {
    pub depth: f32,
    pub stencil: u8,
}
impl<B: Backend> PixelType<B> for DepthStencil {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::DepthStencil)
    }
}

/// Represents a 8-bit stencil value.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Stencil8(pub u8);
impl<B: Backend> PixelType<B> for Stencil8 {
    fn device_pixel_layout() -> Result<B::PixelLayout, B::Error> {
        B::pixel_layout(PixelFormat::Stencil8)
    }
}

pub fn convert_std_channel_to_linear(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

pub fn convert_linear_channel_to_std(c: f32) -> u8 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.0031308 {
        (c * 12.92 * 255.0 + 0.5) as u8
    } else {
        ((1.055 * c.powf(1.0 / 2.4) - 0.055) * 255.0 + 0.5) as u8
    }
}
