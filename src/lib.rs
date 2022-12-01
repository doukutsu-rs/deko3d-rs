use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::pin::Pin;

use bitflags::bitflags;
use deko3d_sys::*;
pub use deko3d_sys::DK_CMDMEM_ALIGNMENT;
pub use deko3d_sys::DK_DEFAULT_MAX_COMPUTE_CONCURRENT_JOBS;
pub use deko3d_sys::DK_IMAGE_DESCRIPTOR_ALIGNMENT;
pub use deko3d_sys::DK_IMAGE_LINEAR_STRIDE_ALIGNMENT;
pub use deko3d_sys::DK_MAX_RENDER_TARGETS;
pub use deko3d_sys::DK_MAX_VERTEX_ATTRIBS;
pub use deko3d_sys::DK_MAX_VERTEX_BUFFERS;
pub use deko3d_sys::DK_MEMBLOCK_ALIGNMENT;
pub use deko3d_sys::DK_NUM_IMAGE_BINDINGS;
pub use deko3d_sys::DK_NUM_SCISSORS;
pub use deko3d_sys::DK_NUM_STORAGE_BUFS;
pub use deko3d_sys::DK_NUM_TEXTURE_BINDINGS;
pub use deko3d_sys::DK_NUM_UNIFORM_BUFS;
pub use deko3d_sys::DK_NUM_VIEWPORTS;
pub use deko3d_sys::DK_PER_WARP_SCRATCH_MEM_ALIGNMENT;
pub use deko3d_sys::DK_QUEUE_MIN_CMDMEM_SIZE;
pub use deko3d_sys::DK_SAMPLER_DESCRIPTOR_ALIGNMENT;
pub use deko3d_sys::DK_SHADER_CODE_ALIGNMENT;
pub use deko3d_sys::DK_SHADER_CODE_UNUSABLE_SIZE;
pub use deko3d_sys::DK_UNIFORM_BUF_ALIGNMENT;
pub use deko3d_sys::DK_UNIFORM_BUF_MAX_SIZE;

pub type Result = std::result::Result<(), DekoError>;

enum DekoError {
    Fail,
    Timeout,
    OutOfMemory,
    NotImplemented,
    MisalignedSize,
    MisalignedData,
    BadInput,
    BadFlags,
    BadState,
    Unknown,
}

trait DkResultExt {
    fn into_result(self) -> Result;
}

impl DkResultExt for DkResult {
    fn into_result(self) -> Result {
        match self {
            DkResult_DkResult_Success => Ok(()),
            DkResult_DkResult_Fail => Err(DekoError::Fail),
            DkResult_DkResult_Timeout => Err(DekoError::Timeout),
            DkResult_DkResult_OutOfMemory => Err(DekoError::OutOfMemory),
            DkResult_DkResult_NotImplemented => Err(DekoError::NotImplemented),
            DkResult_DkResult_MisalignedSize => Err(DekoError::MisalignedSize),
            DkResult_DkResult_MisalignedData => Err(DekoError::MisalignedData),
            DkResult_DkResult_BadInput => Err(DekoError::BadInput),
            DkResult_DkResult_BadFlags => Err(DekoError::BadFlags),
            DkResult_DkResult_BadState => Err(DekoError::BadState),
            _ => Err(DekoError::Unknown),
        }
    }
}

bitflags! {
    pub struct DeviceFlags: u32 {
        const DepthZeroToOne = DkDeviceFlags_DepthZeroToOne as _;
        const DepthMinusOneToOne = DkDeviceFlags_DepthMinusOneToOne as _;
        const OriginUpperLeft = DkDeviceFlags_OriginUpperLeft as _;
        const OriginLowerLeft = DkDeviceFlags_OriginLowerLeft as _;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MemAccess {
    None = DkMemAccess_None as _,
    Uncached = DkMemAccess_Uncached as _,
    Cached = DkMemAccess_Cached as _,
    Mask = DkMemAccess_Mask as _,
}

bitflags! {
    pub struct MemBlockFlags: u32 {
        const CpuAccessShift = DkMemBlockFlags_CpuAccessShift as _;
        const GpuAccessShift = DkMemBlockFlags_GpuAccessShift as _;
        const CpuUncached = DkMemBlockFlags_CpuUncached as _;
        const CpuCached = DkMemBlockFlags_CpuCached as _;
        const CpuAccessMask = DkMemBlockFlags_CpuAccessMask as _;
        const GpuUncached = DkMemBlockFlags_GpuUncached as _;
        const GpuCached = DkMemBlockFlags_GpuCached as _;
        const GpuAccessMask = DkMemBlockFlags_GpuAccessMask as _;
        const Code = DkMemBlockFlags_Code as _;
        const Image = DkMemBlockFlags_Image as _;
        const ZeroFillInit = DkMemBlockFlags_ZeroFillInit as _;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VarOp {
    Set = DkVarOp_DkVarOp_Set as _,
    Add = DkVarOp_DkVarOp_Add as _,
    Sub = DkVarOp_DkVarOp_Sub as _,
    And = DkVarOp_DkVarOp_And as _,
    Or = DkVarOp_DkVarOp_Or as _,
    Xor = DkVarOp_DkVarOp_Xor as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VarCompareOp {
    Equal = DkVarCompareOp_DkVarCompareOp_Equal as _,
    Sequential = DkVarCompareOp_DkVarCompareOp_Sequential as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PipelinePos {
    Top = DkPipelinePos_DkPipelinePos_Top as _,
    Rasterizer = DkPipelinePos_DkPipelinePos_Rasterizer as _,
    Bottom = DkPipelinePos_DkPipelinePos_Bottom as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Counter {
    TimestampPipelineTop = DkCounter_DkCounter_TimestampPipelineTop as _,
    Timestamp = DkCounter_DkCounter_Timestamp as _,
    SamplesPassed = DkCounter_DkCounter_SamplesPassed as _,
    ZcullStats = DkCounter_DkCounter_ZcullStats as _,
    InputVertices = DkCounter_DkCounter_InputVertices as _,
    InputPrimitives = DkCounter_DkCounter_InputPrimitives as _,
    VertexShaderInvocations = DkCounter_DkCounter_VertexShaderInvocations as _,
    TessControlShaderInvocations = DkCounter_DkCounter_TessControlShaderInvocations as _,
    TessEvaluationShaderInvocations = DkCounter_DkCounter_TessEvaluationShaderInvocations as _,
    GeometryShaderInvocations = DkCounter_DkCounter_GeometryShaderInvocations as _,
    FragmentShaderInvocations = DkCounter_DkCounter_FragmentShaderInvocations as _,
    TessEvaluationShaderPrimitives = DkCounter_DkCounter_TessEvaluationShaderPrimitives as _,
    GeometryShaderPrimitives = DkCounter_DkCounter_GeometryShaderPrimitives as _,
    ClipperInputPrimitives = DkCounter_DkCounter_ClipperInputPrimitives as _,
    ClipperOutputPrimitives = DkCounter_DkCounter_ClipperOutputPrimitives as _,
    PrimitivesGenerated = DkCounter_DkCounter_PrimitivesGenerated as _,
    TransformFeedbackPrimitivesWritten = DkCounter_DkCounter_TransformFeedbackPrimitivesWritten as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Barrier {
    None = DkBarrier_DkBarrier_None as _,
    Tiles = DkBarrier_DkBarrier_Tiles as _,
    Fragments = DkBarrier_DkBarrier_Fragments as _,
    Primitives = DkBarrier_DkBarrier_Primitives as _,
    Full = DkBarrier_DkBarrier_Full as _,
}

bitflags! {
    pub struct InvalidateFlags: u32 {
        const None = 0;
        const Image = DkInvalidateFlags_Image as _;
        const Shader = DkInvalidateFlags_Shader as _;
        const Descriptors = DkInvalidateFlags_Descriptors as _;
        const Zcull = DkInvalidateFlags_Zcull as _;
        const L2Cache = DkInvalidateFlags_L2Cache as _;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageType {
    None = DkImageType_DkImageType_None as _,
    D1 = DkImageType_DkImageType_1D as _,
    D2 = DkImageType_DkImageType_2D as _,
    D3 = DkImageType_DkImageType_3D as _,
    D1Array = DkImageType_DkImageType_1DArray as _,
    D2Array = DkImageType_DkImageType_2DArray as _,
    D2MS = DkImageType_DkImageType_2DMS as _,
    D2MSArray = DkImageType_DkImageType_2DMSArray as _,
    Rectangle = DkImageType_DkImageType_Rectangle as _,
    Cubemap = DkImageType_DkImageType_Cubemap as _,
    CubemapArray = DkImageType_DkImageType_CubemapArray as _,
    Buffer = DkImageType_DkImageType_Buffer as _,
}

impl From<DkImageType> for ImageType {
    fn from(v: DkImageType) -> Self {
        match v {
            DkImageType_DkImageType_None => ImageType::None,
            DkImageType_DkImageType_1D => ImageType::D1,
            DkImageType_DkImageType_2D => ImageType::D2,
            DkImageType_DkImageType_3D => ImageType::D3,
            DkImageType_DkImageType_1DArray => ImageType::D1Array,
            DkImageType_DkImageType_2DArray => ImageType::D2Array,
            DkImageType_DkImageType_2DMS => ImageType::D2MS,
            DkImageType_DkImageType_2DMSArray => ImageType::D2MSArray,
            DkImageType_DkImageType_Rectangle => ImageType::Rectangle,
            DkImageType_DkImageType_Cubemap => ImageType::Cubemap,
            DkImageType_DkImageType_CubemapArray => ImageType::CubemapArray,
            DkImageType_DkImageType_Buffer => ImageType::Buffer,
            _ => unreachable!("invalid DkImageType"),
        }
    }
}

bitflags! {
    pub struct ImageFlags: u32 {
        const BlockLinear = DkImageFlags_BlockLinear as _;
        const PitchLinear = DkImageFlags_PitchLinear as _;
        const CustomTileSize = DkImageFlags_CustomTileSize as _;
        const HwCompression = DkImageFlags_HwCompression as _;
        const Z16EnableZbc = DkImageFlags_Z16EnableZbc as _;
        const UsageRender = DkImageFlags_UsageRender as _;
        const UsageLoadStore = DkImageFlags_UsageLoadStore as _;
        const UsagePresent = DkImageFlags_UsagePresent as _;
        const Usage2DEngine = DkImageFlags_Usage2DEngine as _;
        const UsageVideo = DkImageFlags_UsageVideo as _;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    None = DkImageFormat_DkImageFormat_None as _,
    R8Unorm = DkImageFormat_DkImageFormat_R8_Unorm as _,
    R8Snorm = DkImageFormat_DkImageFormat_R8_Snorm as _,
    R8Uint = DkImageFormat_DkImageFormat_R8_Uint as _,
    R8Sint = DkImageFormat_DkImageFormat_R8_Sint as _,
    R16Float = DkImageFormat_DkImageFormat_R16_Float as _,
    R16Unorm = DkImageFormat_DkImageFormat_R16_Unorm as _,
    R16Snorm = DkImageFormat_DkImageFormat_R16_Snorm as _,
    R16Uint = DkImageFormat_DkImageFormat_R16_Uint as _,
    R16Sint = DkImageFormat_DkImageFormat_R16_Sint as _,
    R32Float = DkImageFormat_DkImageFormat_R32_Float as _,
    R32Uint = DkImageFormat_DkImageFormat_R32_Uint as _,
    R32Sint = DkImageFormat_DkImageFormat_R32_Sint as _,
    RG8Unorm = DkImageFormat_DkImageFormat_RG8_Unorm as _,
    RG8Snorm = DkImageFormat_DkImageFormat_RG8_Snorm as _,
    RG8Uint = DkImageFormat_DkImageFormat_RG8_Uint as _,
    RG8Sint = DkImageFormat_DkImageFormat_RG8_Sint as _,
    RG16Float = DkImageFormat_DkImageFormat_RG16_Float as _,
    RG16Unorm = DkImageFormat_DkImageFormat_RG16_Unorm as _,
    RG16Snorm = DkImageFormat_DkImageFormat_RG16_Snorm as _,
    RG16Uint = DkImageFormat_DkImageFormat_RG16_Uint as _,
    RG16Sint = DkImageFormat_DkImageFormat_RG16_Sint as _,
    RG32Float = DkImageFormat_DkImageFormat_RG32_Float as _,
    RG32Uint = DkImageFormat_DkImageFormat_RG32_Uint as _,
    RG32Sint = DkImageFormat_DkImageFormat_RG32_Sint as _,
    RGB32Float = DkImageFormat_DkImageFormat_RGB32_Float as _,
    RGB32Uint = DkImageFormat_DkImageFormat_RGB32_Uint as _,
    RGB32Sint = DkImageFormat_DkImageFormat_RGB32_Sint as _,
    RGBA8Unorm = DkImageFormat_DkImageFormat_RGBA8_Unorm as _,
    RGBA8Snorm = DkImageFormat_DkImageFormat_RGBA8_Snorm as _,
    RGBA8Uint = DkImageFormat_DkImageFormat_RGBA8_Uint as _,
    RGBA8Sint = DkImageFormat_DkImageFormat_RGBA8_Sint as _,
    RGBA16Float = DkImageFormat_DkImageFormat_RGBA16_Float as _,
    RGBA16Unorm = DkImageFormat_DkImageFormat_RGBA16_Unorm as _,
    RGBA16Snorm = DkImageFormat_DkImageFormat_RGBA16_Snorm as _,
    RGBA16Uint = DkImageFormat_DkImageFormat_RGBA16_Uint as _,
    RGBA16Sint = DkImageFormat_DkImageFormat_RGBA16_Sint as _,
    RGBA32Float = DkImageFormat_DkImageFormat_RGBA32_Float as _,
    RGBA32Uint = DkImageFormat_DkImageFormat_RGBA32_Uint as _,
    RGBA32Sint = DkImageFormat_DkImageFormat_RGBA32_Sint as _,
    S8 = DkImageFormat_DkImageFormat_S8 as _,
    Z16 = DkImageFormat_DkImageFormat_Z16 as _,
    Z24X8 = DkImageFormat_DkImageFormat_Z24X8 as _,
    ZF32 = DkImageFormat_DkImageFormat_ZF32 as _,
    Z24S8 = DkImageFormat_DkImageFormat_Z24S8 as _,
    ZF32X24S8 = DkImageFormat_DkImageFormat_ZF32_X24S8 as _,
    RGBX8UnormSrgb = DkImageFormat_DkImageFormat_RGBX8_Unorm_sRGB as _,
    RGBA8UnormSrgb = DkImageFormat_DkImageFormat_RGBA8_Unorm_sRGB as _,
    RGBA4Unorm = DkImageFormat_DkImageFormat_RGBA4_Unorm as _,
    RGB5Unorm = DkImageFormat_DkImageFormat_RGB5_Unorm as _,
    RGB5A1Unorm = DkImageFormat_DkImageFormat_RGB5A1_Unorm as _,
    RGB565Unorm = DkImageFormat_DkImageFormat_RGB565_Unorm as _,
    RGB10A2Unorm = DkImageFormat_DkImageFormat_RGB10A2_Unorm as _,
    RGB10A2Uint = DkImageFormat_DkImageFormat_RGB10A2_Uint as _,
    RG11B10Float = DkImageFormat_DkImageFormat_RG11B10_Float as _,
    E5BGR9Float = DkImageFormat_DkImageFormat_E5BGR9_Float as _,
    RGBBC1 = DkImageFormat_DkImageFormat_RGB_BC1 as _,
    RGBABC1 = DkImageFormat_DkImageFormat_RGBA_BC1 as _,
    RGBABC2 = DkImageFormat_DkImageFormat_RGBA_BC2 as _,
    RGBABC3 = DkImageFormat_DkImageFormat_RGBA_BC3 as _,
    RGBBC1Srgb = DkImageFormat_DkImageFormat_RGB_BC1_sRGB as _,
    RGBABC1Srgb = DkImageFormat_DkImageFormat_RGBA_BC1_sRGB as _,
    RGBABC2Srgb = DkImageFormat_DkImageFormat_RGBA_BC2_sRGB as _,
    RGBABC3Srgb = DkImageFormat_DkImageFormat_RGBA_BC3_sRGB as _,
    RBC4Unorm = DkImageFormat_DkImageFormat_R_BC4_Unorm as _,
    RBC4Snorm = DkImageFormat_DkImageFormat_R_BC4_Snorm as _,
    RGBC5Unorm = DkImageFormat_DkImageFormat_RG_BC5_Unorm as _,
    RGBC5Snorm = DkImageFormat_DkImageFormat_RG_BC5_Snorm as _,
    RGBABC7Unorm = DkImageFormat_DkImageFormat_RGBA_BC7_Unorm as _,
    RGBABC7UnormSrgb = DkImageFormat_DkImageFormat_RGBA_BC7_Unorm_sRGB as _,
    RGBABC6HSF16Float = DkImageFormat_DkImageFormat_RGBA_BC6H_SF16_Float as _,
    RGBABC6HUF16Float = DkImageFormat_DkImageFormat_RGBA_BC6H_UF16_Float as _,
    RGBX8Unorm = DkImageFormat_DkImageFormat_RGBX8_Unorm as _,
    RGBX8Snorm = DkImageFormat_DkImageFormat_RGBX8_Snorm as _,
    RGBX8Uint = DkImageFormat_DkImageFormat_RGBX8_Uint as _,
    RGBX8Sint = DkImageFormat_DkImageFormat_RGBX8_Sint as _,
    RGBX16Float = DkImageFormat_DkImageFormat_RGBX16_Float as _,
    RGBX16Unorm = DkImageFormat_DkImageFormat_RGBX16_Unorm as _,
    RGBX16Snorm = DkImageFormat_DkImageFormat_RGBX16_Snorm as _,
    RGBX16Uint = DkImageFormat_DkImageFormat_RGBX16_Uint as _,
    RGBX16Sint = DkImageFormat_DkImageFormat_RGBX16_Sint as _,
    RGBX32Float = DkImageFormat_DkImageFormat_RGBX32_Float as _,
    RGBX32Uint = DkImageFormat_DkImageFormat_RGBX32_Uint as _,
    RGBX32Sint = DkImageFormat_DkImageFormat_RGBX32_Sint as _,
    RGBAAstc4x4 = DkImageFormat_DkImageFormat_RGBA_ASTC_4x4 as _,
    RGBAAstc5x4 = DkImageFormat_DkImageFormat_RGBA_ASTC_5x4 as _,
    RGBAAstc5x5 = DkImageFormat_DkImageFormat_RGBA_ASTC_5x5 as _,
    RGBAAstc6x5 = DkImageFormat_DkImageFormat_RGBA_ASTC_6x5 as _,
    RGBAAstc6x6 = DkImageFormat_DkImageFormat_RGBA_ASTC_6x6 as _,
    RGBAAstc8x5 = DkImageFormat_DkImageFormat_RGBA_ASTC_8x5 as _,
    RGBAAstc8x6 = DkImageFormat_DkImageFormat_RGBA_ASTC_8x6 as _,
    RGBAAstc8x8 = DkImageFormat_DkImageFormat_RGBA_ASTC_8x8 as _,
    RGBAAstc10x5 = DkImageFormat_DkImageFormat_RGBA_ASTC_10x5 as _,
    RGBAAstc10x6 = DkImageFormat_DkImageFormat_RGBA_ASTC_10x6 as _,
    RGBAAstc10x8 = DkImageFormat_DkImageFormat_RGBA_ASTC_10x8 as _,
    RGBAAstc10x10 = DkImageFormat_DkImageFormat_RGBA_ASTC_10x10 as _,
    RGBAAstc12x10 = DkImageFormat_DkImageFormat_RGBA_ASTC_12x10 as _,
    RGBAAstc12x12 = DkImageFormat_DkImageFormat_RGBA_ASTC_12x12 as _,
    RGBAAstc4x4Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_4x4_sRGB as _,
    RGBAAstc5x4Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_5x4_sRGB as _,
    RGBAAstc5x5Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_5x5_sRGB as _,
    RGBAAstc6x5Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_6x5_sRGB as _,
    RGBAAstc6x6Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_6x6_sRGB as _,
    RGBAAstc8x5Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_8x5_sRGB as _,
    RGBAAstc8x6Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_8x6_sRGB as _,
    RGBAAstc8x8Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_8x8_sRGB as _,
    RGBAAstc10x5Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_10x5_sRGB as _,
    RGBAAstc10x6Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_10x6_sRGB as _,
    RGBAAstc10x8Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_10x8_sRGB as _,
    RGBAAstc10x10Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_10x10_sRGB as _,
    RGBAAstc12x10Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_12x10_sRGB as _,
    RGBAAstc12x12Srgb = DkImageFormat_DkImageFormat_RGBA_ASTC_12x12_sRGB as _,
    BGR565Unorm = DkImageFormat_DkImageFormat_BGR565_Unorm as _,
    BGR5Unorm = DkImageFormat_DkImageFormat_BGR5_Unorm as _,
    BGR5A1Unorm = DkImageFormat_DkImageFormat_BGR5A1_Unorm as _,
    A5BGR5Unorm = DkImageFormat_DkImageFormat_A5BGR5_Unorm as _,
    BGRX8Unorm = DkImageFormat_DkImageFormat_BGRX8_Unorm as _,
    BGRA8Unorm = DkImageFormat_DkImageFormat_BGRA8_Unorm as _,
    BGRX8UnormSrgb = DkImageFormat_DkImageFormat_BGRX8_Unorm_sRGB as _,
    BGRA8UnormSrgb = DkImageFormat_DkImageFormat_BGRA8_Unorm_sRGB as _,
    RETC2Unorm = DkImageFormat_DkImageFormat_R_ETC2_Unorm as _,
    RETC2Snorm = DkImageFormat_DkImageFormat_R_ETC2_Snorm as _,
    RGETC2Unorm = DkImageFormat_DkImageFormat_RG_ETC2_Unorm as _,
    RGETC2Snorm = DkImageFormat_DkImageFormat_RG_ETC2_Snorm as _,
    RGBETC2 = DkImageFormat_DkImageFormat_RGB_ETC2 as _,
    RGBPTAETC2 = DkImageFormat_DkImageFormat_RGB_PTA_ETC2 as _,
    RGBAETC2 = DkImageFormat_DkImageFormat_RGBA_ETC2 as _,
    RGBETC2Srgb = DkImageFormat_DkImageFormat_RGB_ETC2_sRGB as _,
    RGBPTAETC2Srgb = DkImageFormat_DkImageFormat_RGB_PTA_ETC2_sRGB as _,
    RGBAETC2Srgb = DkImageFormat_DkImageFormat_RGBA_ETC2_sRGB as _,
}

impl From<DkImageFormat> for ImageFormat {
    fn from(format: DkImageFormat) -> Self {
        match format {
            DkImageFormat_DkImageFormat_None => ImageFormat::None,
            DkImageFormat_DkImageFormat_R8_Unorm => ImageFormat::R8Unorm,
            DkImageFormat_DkImageFormat_R8_Snorm => ImageFormat::R8Snorm,
            DkImageFormat_DkImageFormat_R8_Uint => ImageFormat::R8Uint,
            DkImageFormat_DkImageFormat_R8_Sint => ImageFormat::R8Sint,
            DkImageFormat_DkImageFormat_R16_Float => ImageFormat::R16Float,
            DkImageFormat_DkImageFormat_R16_Unorm => ImageFormat::R16Unorm,
            DkImageFormat_DkImageFormat_R16_Snorm => ImageFormat::R16Snorm,
            DkImageFormat_DkImageFormat_R16_Uint => ImageFormat::R16Uint,
            DkImageFormat_DkImageFormat_R16_Sint => ImageFormat::R16Sint,
            DkImageFormat_DkImageFormat_R32_Float => ImageFormat::R32Float,
            DkImageFormat_DkImageFormat_R32_Uint => ImageFormat::R32Uint,
            DkImageFormat_DkImageFormat_R32_Sint => ImageFormat::R32Sint,
            DkImageFormat_DkImageFormat_RG8_Unorm => ImageFormat::RG8Unorm,
            DkImageFormat_DkImageFormat_RG8_Snorm => ImageFormat::RG8Snorm,
            DkImageFormat_DkImageFormat_RG8_Uint => ImageFormat::RG8Uint,
            DkImageFormat_DkImageFormat_RG8_Sint => ImageFormat::RG8Sint,
            DkImageFormat_DkImageFormat_RG16_Float => ImageFormat::RG16Float,
            DkImageFormat_DkImageFormat_RG16_Unorm => ImageFormat::RG16Unorm,
            DkImageFormat_DkImageFormat_RG16_Snorm => ImageFormat::RG16Snorm,
            DkImageFormat_DkImageFormat_RG16_Uint => ImageFormat::RG16Uint,
            DkImageFormat_DkImageFormat_RG16_Sint => ImageFormat::RG16Sint,
            DkImageFormat_DkImageFormat_RG32_Float => ImageFormat::RG32Float,
            DkImageFormat_DkImageFormat_RG32_Uint => ImageFormat::RG32Uint,
            DkImageFormat_DkImageFormat_RG32_Sint => ImageFormat::RG32Sint,
            DkImageFormat_DkImageFormat_RGB32_Float => ImageFormat::RGB32Float,
            DkImageFormat_DkImageFormat_RGB32_Uint => ImageFormat::RGB32Uint,
            DkImageFormat_DkImageFormat_RGB32_Sint => ImageFormat::RGB32Sint,
            DkImageFormat_DkImageFormat_RGBA8_Unorm => ImageFormat::RGBA8Unorm,
            DkImageFormat_DkImageFormat_RGBA8_Snorm => ImageFormat::RGBA8Snorm,
            DkImageFormat_DkImageFormat_RGBA8_Uint => ImageFormat::RGBA8Uint,
            DkImageFormat_DkImageFormat_RGBA8_Sint => ImageFormat::RGBA8Sint,
            DkImageFormat_DkImageFormat_RGBA16_Float => ImageFormat::RGBA16Float,
            DkImageFormat_DkImageFormat_RGBA16_Unorm => ImageFormat::RGBA16Unorm,
            DkImageFormat_DkImageFormat_RGBA16_Snorm => ImageFormat::RGBA16Snorm,
            DkImageFormat_DkImageFormat_RGBA16_Uint => ImageFormat::RGBA16Uint,
            DkImageFormat_DkImageFormat_RGBA16_Sint => ImageFormat::RGBA16Sint,
            DkImageFormat_DkImageFormat_RGBA32_Float => ImageFormat::RGBA32Float,
            DkImageFormat_DkImageFormat_RGBA32_Uint => ImageFormat::RGBA32Uint,
            DkImageFormat_DkImageFormat_RGBA32_Sint => ImageFormat::RGBA32Sint,
            DkImageFormat_DkImageFormat_S8 => ImageFormat::S8,
            DkImageFormat_DkImageFormat_Z16 => ImageFormat::Z16,
            DkImageFormat_DkImageFormat_Z24X8 => ImageFormat::Z24X8,
            DkImageFormat_DkImageFormat_ZF32 => ImageFormat::ZF32,
            DkImageFormat_DkImageFormat_Z24S8 => ImageFormat::Z24S8,
            DkImageFormat_DkImageFormat_ZF32_X24S8 => ImageFormat::ZF32X24S8,
            DkImageFormat_DkImageFormat_RGBX8_Unorm_sRGB => ImageFormat::RGBX8UnormSrgb,
            DkImageFormat_DkImageFormat_RGBA8_Unorm_sRGB => ImageFormat::RGBA8UnormSrgb,
            DkImageFormat_DkImageFormat_RGBA4_Unorm => ImageFormat::RGBA4Unorm,
            DkImageFormat_DkImageFormat_RGB5_Unorm => ImageFormat::RGB5Unorm,
            DkImageFormat_DkImageFormat_RGB5A1_Unorm => ImageFormat::RGB5A1Unorm,
            DkImageFormat_DkImageFormat_RGB565_Unorm => ImageFormat::RGB565Unorm,
            DkImageFormat_DkImageFormat_RGB10A2_Unorm => ImageFormat::RGB10A2Unorm,
            DkImageFormat_DkImageFormat_RGB10A2_Uint => ImageFormat::RGB10A2Uint,
            DkImageFormat_DkImageFormat_RG11B10_Float => ImageFormat::RG11B10Float,
            DkImageFormat_DkImageFormat_E5BGR9_Float => ImageFormat::E5BGR9Float,
            DkImageFormat_DkImageFormat_RGB_BC1 => ImageFormat::RGBBC1,
            DkImageFormat_DkImageFormat_RGBA_BC1 => ImageFormat::RGBABC1,
            DkImageFormat_DkImageFormat_RGBA_BC2 => ImageFormat::RGBABC2,
            DkImageFormat_DkImageFormat_RGBA_BC3 => ImageFormat::RGBABC3,
            DkImageFormat_DkImageFormat_RGB_BC1_sRGB => ImageFormat::RGBBC1Srgb,
            DkImageFormat_DkImageFormat_RGBA_BC1_sRGB => ImageFormat::RGBABC1Srgb,
            DkImageFormat_DkImageFormat_RGBA_BC2_sRGB => ImageFormat::RGBABC2Srgb,
            DkImageFormat_DkImageFormat_RGBA_BC3_sRGB => ImageFormat::RGBABC3Srgb,
            DkImageFormat_DkImageFormat_R_BC4_Unorm => ImageFormat::RBC4Unorm,
            DkImageFormat_DkImageFormat_R_BC4_Snorm => ImageFormat::RBC4Snorm,
            DkImageFormat_DkImageFormat_RG_BC5_Unorm => ImageFormat::RGBC5Unorm,
            DkImageFormat_DkImageFormat_RG_BC5_Snorm => ImageFormat::RGBC5Snorm,
            DkImageFormat_DkImageFormat_RGBA_BC7_Unorm => ImageFormat::RGBABC7Unorm,
            DkImageFormat_DkImageFormat_RGBA_BC7_Unorm_sRGB => ImageFormat::RGBABC7UnormSrgb,
            DkImageFormat_DkImageFormat_RGBA_BC6H_SF16_Float => ImageFormat::RGBABC6HSF16Float,
            DkImageFormat_DkImageFormat_RGBA_BC6H_UF16_Float => ImageFormat::RGBABC6HUF16Float,
            DkImageFormat_DkImageFormat_RGBX8_Unorm => ImageFormat::RGBX8Unorm,
            DkImageFormat_DkImageFormat_RGBX8_Snorm => ImageFormat::RGBX8Snorm,
            DkImageFormat_DkImageFormat_RGBX8_Uint => ImageFormat::RGBX8Uint,
            DkImageFormat_DkImageFormat_RGBX8_Sint => ImageFormat::RGBX8Sint,
            DkImageFormat_DkImageFormat_RGBX16_Float => ImageFormat::RGBX16Float,
            DkImageFormat_DkImageFormat_RGBX16_Unorm => ImageFormat::RGBX16Unorm,
            DkImageFormat_DkImageFormat_RGBX16_Snorm => ImageFormat::RGBX16Snorm,
            DkImageFormat_DkImageFormat_RGBX16_Uint => ImageFormat::RGBX16Uint,
            DkImageFormat_DkImageFormat_RGBX16_Sint => ImageFormat::RGBX16Sint,
            DkImageFormat_DkImageFormat_RGBX32_Float => ImageFormat::RGBX32Float,
            DkImageFormat_DkImageFormat_RGBX32_Uint => ImageFormat::RGBX32Uint,
            DkImageFormat_DkImageFormat_RGBX32_Sint => ImageFormat::RGBX32Sint,
            DkImageFormat_DkImageFormat_RGBA_ASTC_4x4 => ImageFormat::RGBAAstc4x4,
            DkImageFormat_DkImageFormat_RGBA_ASTC_5x4 => ImageFormat::RGBAAstc5x4,
            DkImageFormat_DkImageFormat_RGBA_ASTC_5x5 => ImageFormat::RGBAAstc5x5,
            DkImageFormat_DkImageFormat_RGBA_ASTC_6x5 => ImageFormat::RGBAAstc6x5,
            DkImageFormat_DkImageFormat_RGBA_ASTC_6x6 => ImageFormat::RGBAAstc6x6,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x5 => ImageFormat::RGBAAstc8x5,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x6 => ImageFormat::RGBAAstc8x6,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x8 => ImageFormat::RGBAAstc8x8,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x5 => ImageFormat::RGBAAstc10x5,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x6 => ImageFormat::RGBAAstc10x6,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x8 => ImageFormat::RGBAAstc10x8,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x10 => ImageFormat::RGBAAstc10x10,
            DkImageFormat_DkImageFormat_RGBA_ASTC_12x10 => ImageFormat::RGBAAstc12x10,
            DkImageFormat_DkImageFormat_RGBA_ASTC_12x12 => ImageFormat::RGBAAstc12x12,
            DkImageFormat_DkImageFormat_RGBA_ASTC_4x4_sRGB => ImageFormat::RGBAAstc4x4Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_5x4_sRGB => ImageFormat::RGBAAstc5x4Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_5x5_sRGB => ImageFormat::RGBAAstc5x5Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_6x5_sRGB => ImageFormat::RGBAAstc6x5Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_6x6_sRGB => ImageFormat::RGBAAstc6x6Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x5_sRGB => ImageFormat::RGBAAstc8x5Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x6_sRGB => ImageFormat::RGBAAstc8x6Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_8x8_sRGB => ImageFormat::RGBAAstc8x8Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x5_sRGB => ImageFormat::RGBAAstc10x5Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x6_sRGB => ImageFormat::RGBAAstc10x6Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x8_sRGB => ImageFormat::RGBAAstc10x8Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_10x10_sRGB => ImageFormat::RGBAAstc10x10Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_12x10_sRGB => ImageFormat::RGBAAstc12x10Srgb,
            DkImageFormat_DkImageFormat_RGBA_ASTC_12x12_sRGB => ImageFormat::RGBAAstc12x12Srgb,
            DkImageFormat_DkImageFormat_BGR565_Unorm => ImageFormat::BGR565Unorm,
            DkImageFormat_DkImageFormat_BGR5_Unorm => ImageFormat::BGR5Unorm,
            DkImageFormat_DkImageFormat_BGR5A1_Unorm => ImageFormat::BGR5A1Unorm,
            DkImageFormat_DkImageFormat_A5BGR5_Unorm => ImageFormat::A5BGR5Unorm,
            DkImageFormat_DkImageFormat_BGRX8_Unorm => ImageFormat::BGRX8Unorm,
            DkImageFormat_DkImageFormat_BGRA8_Unorm => ImageFormat::BGRA8Unorm,
            DkImageFormat_DkImageFormat_BGRX8_Unorm_sRGB => ImageFormat::BGRX8UnormSrgb,
            DkImageFormat_DkImageFormat_BGRA8_Unorm_sRGB => ImageFormat::BGRA8UnormSrgb,
            DkImageFormat_DkImageFormat_R_ETC2_Unorm => ImageFormat::RETC2Unorm,
            DkImageFormat_DkImageFormat_R_ETC2_Snorm => ImageFormat::RETC2Snorm,
            DkImageFormat_DkImageFormat_RG_ETC2_Unorm => ImageFormat::RGETC2Unorm,
            DkImageFormat_DkImageFormat_RG_ETC2_Snorm => ImageFormat::RGETC2Snorm,
            DkImageFormat_DkImageFormat_RGB_ETC2 => ImageFormat::RGBETC2,
            DkImageFormat_DkImageFormat_RGB_PTA_ETC2 => ImageFormat::RGBPTAETC2,
            DkImageFormat_DkImageFormat_RGBA_ETC2 => ImageFormat::RGBAETC2,
            DkImageFormat_DkImageFormat_RGB_ETC2_sRGB => ImageFormat::RGBETC2Srgb,
            DkImageFormat_DkImageFormat_RGB_PTA_ETC2_sRGB => ImageFormat::RGBPTAETC2Srgb,
            DkImageFormat_DkImageFormat_RGBA_ETC2_sRGB => ImageFormat::RGBAETC2Srgb,
            _ => unreachable!("Invalid DkImageFormat"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageSwizzle {
    Zero = DkImageSwizzle_DkImageSwizzle_Zero as _,
    One = DkImageSwizzle_DkImageSwizzle_One as _,
    Red = DkImageSwizzle_DkImageSwizzle_Red as _,
    Green = DkImageSwizzle_DkImageSwizzle_Green as _,
    Blue = DkImageSwizzle_DkImageSwizzle_Blue as _,
    Alpha = DkImageSwizzle_DkImageSwizzle_Alpha as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MsMode {
    Ms1x = DkMsMode_DkMsMode_1x as _,
    Ms2x = DkMsMode_DkMsMode_2x as _,
    Ms4x = DkMsMode_DkMsMode_4x as _,
    Ms8x = DkMsMode_DkMsMode_8x as _,
}

impl From<DkMsMode> for MsMode {
    fn from(val: DkMsMode) -> Self {
        match val {
            DkMsMode_DkMsMode_1x => MsMode::Ms1x,
            DkMsMode_DkMsMode_2x => MsMode::Ms2x,
            DkMsMode_DkMsMode_4x => MsMode::Ms4x,
            DkMsMode_DkMsMode_8x => MsMode::Ms8x,
            _ => unreachable!("Invalid DkMsMode"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DsSource {
    Depth = DkDsSource_DkDsSource_Depth as _,
    Stencil = DkDsSource_DkDsSource_Stencil as _,
}

impl From<DkDsSource> for DsSource {
    fn from(val: DkDsSource) -> Self {
        match val {
            DkDsSource_DkDsSource_Depth => DsSource::Depth,
            DkDsSource_DkDsSource_Stencil => DsSource::Stencil,
            _ => unreachable!("Invalid DkDsSource"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TileSize {
    OneGob = DkTileSize_DkTileSize_OneGob as _,
    TwoGobs = DkTileSize_DkTileSize_TwoGobs as _,
    FourGobs = DkTileSize_DkTileSize_FourGobs as _,
    EightGobs = DkTileSize_DkTileSize_EightGobs as _,
    SixteenGobs = DkTileSize_DkTileSize_SixteenGobs as _,
    ThirtyTwoGobs = DkTileSize_DkTileSize_ThirtyTwoGobs as _,
}

impl From<DkTileSize> for TileSize {
    fn from(val: DkTileSize) -> Self {
        match val {
            DkTileSize_DkTileSize_OneGob => TileSize::OneGob,
            DkTileSize_DkTileSize_TwoGobs => TileSize::TwoGobs,
            DkTileSize_DkTileSize_FourGobs => TileSize::FourGobs,
            DkTileSize_DkTileSize_EightGobs => TileSize::EightGobs,
            DkTileSize_DkTileSize_SixteenGobs => TileSize::SixteenGobs,
            DkTileSize_DkTileSize_ThirtyTwoGobs => TileSize::ThirtyTwoGobs,
            _ => unreachable!("Invalid DkTileSize"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Stage {
    Vertex = DkStage_DkStage_Vertex as _,
    TessCtrl = DkStage_DkStage_TessCtrl as _,
    TessEval = DkStage_DkStage_TessEval as _,
    Geometry = DkStage_DkStage_Geometry as _,
    Fragment = DkStage_DkStage_Fragment as _,
    Compute = DkStage_DkStage_Compute as _,
}

impl From<DkStage> for Stage {
    fn from(stage: DkStage) -> Self {
        match stage {
            DkStage_DkStage_Vertex => Stage::Vertex,
            DkStage_DkStage_TessCtrl => Stage::TessCtrl,
            DkStage_DkStage_TessEval => Stage::TessEval,
            DkStage_DkStage_Geometry => Stage::Geometry,
            DkStage_DkStage_Fragment => Stage::Fragment,
            DkStage_DkStage_Compute => Stage::Compute,
            _ => unreachable!("Invalid DkStage"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    Nearest = DkFilter_DkFilter_Nearest as _,
    Linear = DkFilter_DkFilter_Linear as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MipFilter {
    None = DkMipFilter_DkMipFilter_None as _,
    Nearest = DkMipFilter_DkMipFilter_Nearest as _,
    Linear = DkMipFilter_DkMipFilter_Linear as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WrapMode {
    Repeat = DkWrapMode_DkWrapMode_Repeat as _,
    MirroredRepeat = DkWrapMode_DkWrapMode_MirroredRepeat as _,
    ClampToEdge = DkWrapMode_DkWrapMode_ClampToEdge as _,
    ClampToBorder = DkWrapMode_DkWrapMode_ClampToBorder as _,
    Clamp = DkWrapMode_DkWrapMode_Clamp as _,
    MirrorClampToEdge = DkWrapMode_DkWrapMode_MirrorClampToEdge as _,
    MirrorClampToBorder = DkWrapMode_DkWrapMode_MirrorClampToBorder as _,
    MirrorClamp = DkWrapMode_DkWrapMode_MirrorClamp as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompareOp {
    Never = DkCompareOp_DkCompareOp_Never as _,
    Less = DkCompareOp_DkCompareOp_Less as _,
    Equal = DkCompareOp_DkCompareOp_Equal as _,
    Lequal = DkCompareOp_DkCompareOp_Lequal as _,
    Greater = DkCompareOp_DkCompareOp_Greater as _,
    NotEqual = DkCompareOp_DkCompareOp_NotEqual as _,
    Gequal = DkCompareOp_DkCompareOp_Gequal as _,
    Always = DkCompareOp_DkCompareOp_Always as _,
}

impl From<DkCompareOp> for CompareOp {
    fn from(val: DkCompareOp) -> Self {
        match val {
            DkCompareOp_DkCompareOp_Never => CompareOp::Never,
            DkCompareOp_DkCompareOp_Less => CompareOp::Less,
            DkCompareOp_DkCompareOp_Equal => CompareOp::Equal,
            DkCompareOp_DkCompareOp_Lequal => CompareOp::Lequal,
            DkCompareOp_DkCompareOp_Greater => CompareOp::Greater,
            DkCompareOp_DkCompareOp_NotEqual => CompareOp::NotEqual,
            DkCompareOp_DkCompareOp_Gequal => CompareOp::Gequal,
            DkCompareOp_DkCompareOp_Always => CompareOp::Always,
            _ => unreachable!("Invalid DkCompareOp"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SamplerReduction {
    WeightedAverage = DkSamplerReduction_DkSamplerReduction_WeightedAverage as _,
    Min = DkSamplerReduction_DkSamplerReduction_Min as _,
    Max = DkSamplerReduction_DkSamplerReduction_Max as _,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StageFlag {
    Vertex = DkStageFlag_Vertex as _,
    TessCtrl = DkStageFlag_TessCtrl as _,
    TessEval = DkStageFlag_TessEval as _,
    Geometry = DkStageFlag_Geometry as _,
    Fragment = DkStageFlag_Fragment as _,
    Compute = DkStageFlag_Compute as _,
    GraphicsMask = DkStageFlag_GraphicsMask as _,
}

pub type CmdList = DkCmdList;
pub type GpuAddr = DkGpuAddr;
pub type ResHandle = DkResHandle;
pub type BufExtents = DkBufExtents;
pub type Viewport = DkViewport;

pub const fn make_image_handle(id: u32) -> ResHandle {
    id & ((1 << 20) - 1)
}

pub const fn make_sampler_handle(id: u32) -> ResHandle {
    id << 20
}

pub const fn make_texture_handle(image_id: u32, sampler_id: u32) -> ResHandle {
    make_image_handle(image_id) | make_sampler_handle(sampler_id)
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Swizzle {
    PositiveX = DkSwizzle_DkSwizzle_PositiveX as _,
    NegativeX = DkSwizzle_DkSwizzle_NegativeX as _,
    PositiveY = DkSwizzle_DkSwizzle_PositiveY as _,
    NegativeY = DkSwizzle_DkSwizzle_NegativeY as _,
    PositiveZ = DkSwizzle_DkSwizzle_PositiveZ as _,
    NegativeZ = DkSwizzle_DkSwizzle_NegativeZ as _,
    PositiveW = DkSwizzle_DkSwizzle_PositiveW as _,
    NegativeW = DkSwizzle_DkSwizzle_NegativeW as _,
}

pub struct ViewportSwizzle {
    pub x: Swizzle,
    pub y: Swizzle,
    pub z: Swizzle,
    pub w: Swizzle,
}

pub type Scissor = DkScissor;

#[repr(C)]
pub struct Device(DkDevice);

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { dkDeviceDestroy(self.0) }
    }
}

impl Device {
    pub unsafe fn from_raw(device: DkDevice) -> Self {
        Self(device)
    }

    pub fn into_raw(&self) -> DkDevice {
        self.0
    }

    pub fn get_current_timestamp(&self) -> u64 {
        unsafe { dkDeviceGetCurrentTimestamp(self.0) }
    }

    pub fn get_current_timestamp_in_ns(&self) -> u64 {
        unsafe { dkDeviceGetCurrentTimestampInNs(self.0) }
    }
}

#[repr(C)]
pub struct MemBlock(DkMemBlock);

impl Drop for MemBlock {
    fn drop(&mut self) {
        unsafe { dkMemBlockDestroy(self.0) }
    }
}

impl MemBlock {
    pub unsafe fn from_raw(mem_block: DkMemBlock) -> Self {
        Self(mem_block)
    }

    pub fn into_raw(self) -> DkMemBlock {
        self.0
    }

    pub fn get_cpu_addr(&self) -> *mut std::ffi::c_void {
        unsafe { dkMemBlockGetCpuAddr(self.0) }
    }

    pub fn get_gpu_addr(&self) -> DkGpuAddr {
        unsafe { dkMemBlockGetGpuAddr(self.0) }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { dkMemBlockGetSize(self.0) }
    }

    pub fn flush_cpu_cache(&self, offset: u32, size: u32) -> Result {
        unsafe { dkMemBlockFlushCpuCache(self.0, offset, size).into_result() }
    }
}

#[repr(C)]
pub struct Fence(DkFence);

impl Fence {
    pub unsafe fn from_raw(fence: DkFence) -> Self {
        Self(fence)
    }

    pub fn into_raw(self) -> DkFence {
        self.0
    }

    pub fn wait(&mut self, timeout: i64) -> Result {
        unsafe { dkFenceWait(&mut self.0, timeout).into_result() }
    }

    pub fn wait_no_timeout(&mut self) -> Result {
        self.wait(-1)
    }
}

#[repr(C)]
pub struct Variable(DkVariable);

impl Variable {
    pub unsafe fn from_raw(variable: DkVariable) -> Self {
        Self(variable)
    }

    pub fn into_raw(self) -> DkVariable {
        self.0
    }

    pub fn initialize(&mut self, mem: &MemBlock, offset: u32) {
        unsafe { dkVariableInitialize(&mut self.0, mem.0, offset) }
    }

    pub fn read(&self) -> u32 {
        unsafe { dkVariableRead(&self.0) }
    }

    pub fn signal(&self, op: VarOp, value: u32) {
        unsafe { dkVariableSignal(&self.0, op as _, value) }
    }
}

#[repr(C)]
pub struct CmdBuf(DkCmdBuf);

impl Drop for CmdBuf {
    fn drop(&mut self) {
        unsafe { dkCmdBufDestroy(self.0) }
    }
}

impl CmdBuf {
    pub unsafe fn from_raw(cmd_buf: DkCmdBuf) -> Self {
        Self(cmd_buf)
    }

    pub fn into_raw(self) -> DkCmdBuf {
        self.0
    }

    pub fn add_memory(&self, mem: &MemBlock, offset: u32, size: u32) {
        unsafe { dkCmdBufAddMemory(self.0, mem.0, offset, size) }
    }

    pub fn finish_list(&self) -> DkCmdList {
        unsafe { dkCmdBufFinishList(self.0) }
    }

    pub fn clear(&self) {
        unsafe { dkCmdBufClear(self.0) }
    }

    pub fn begin_capture_cmds(&self, storage: &mut [u32]) {
        assert!(storage.len() <= u32::MAX as usize);
        unsafe { dkCmdBufBeginCaptureCmds(self.0, storage.as_mut_ptr(), storage.len() as u32) }
    }

    pub fn end_capture_cmds(&self) -> u32 {
        unsafe { dkCmdBufEndCaptureCmds(self.0) }
    }

    pub fn replay_cmds(&self, words: &[u32]) {
        unsafe { dkCmdBufReplayCmds(self.0, words.as_ptr(), words.len() as _) }
    }

    pub fn call_list(&self, list: DkCmdList) {
        unsafe { dkCmdBufCallList(self.0, list) }
    }

    pub fn wait_fence(&self, fence: &mut Fence) {
        unsafe { dkCmdBufWaitFence(self.0, &mut fence.0) }
    }

    pub fn signal_fence(&self, fence: &mut Fence, flush: bool) {
        unsafe { dkCmdBufSignalFence(self.0, &mut fence.0, flush) }
    }

    pub fn wait_variable(&self, var: &Variable, op: VarCompareOp, value: u32) {
        unsafe { dkCmdBufWaitVariable(self.0, &var.0, op as _, value) }
    }

    pub fn signal_variable(&self, var: &Variable, op: VarOp, value: u32, pos: PipelinePos) {
        unsafe { dkCmdBufSignalVariable(self.0, &var.0, op as _, value, pos as _) }
    }

    pub fn barrier(&self, mode: Barrier, invalidate_flags: InvalidateFlags) {
        unsafe { dkCmdBufBarrier(self.0, mode as _, invalidate_flags.bits) }
    }

    pub fn bind_shaders(&self, stage_mask: StageFlag, shaders: &[&Shader]) {
        unsafe { dkCmdBufBindShaders(self.0, stage_mask as _, shaders.as_ptr() as *const _, shaders.len() as u32) }
    }

    pub fn bind_uniform_buffer(&self, stage: Stage, id: u32, buf_addr: DkGpuAddr, buf_size: u32) {
        let buf_extents = [DkBufExtents { addr: buf_addr, size: buf_size }];
        self.bind_uniform_buffers(stage, id, &buf_extents);
    }

    pub fn bind_uniform_buffers(&self, stage: Stage, first_id: u32, buffers: &[BufExtents]) {
        unsafe { dkCmdBufBindUniformBuffers(self.0, stage as _, first_id, buffers.as_ptr(), buffers.len() as u32) }
    }

    pub fn bind_storage_buffer(&self, stage: Stage, id: u32, buf_addr: DkGpuAddr, buf_size: u32) {
        let buf_extents = [DkBufExtents { addr: buf_addr, size: buf_size }];
        self.bind_storage_buffers(stage, id, &buf_extents);
    }

    pub fn bind_storage_buffers(&self, stage: Stage, first_id: u32, buffers: &[BufExtents]) {
        unsafe { dkCmdBufBindStorageBuffers(self.0, stage as _, first_id, buffers.as_ptr(), buffers.len() as u32) }
    }

    pub fn bind_textures(&self, stage: Stage, first_id: u32, handles: &[ResHandle]) {
        unsafe { dkCmdBufBindTextures(self.0, stage as _, first_id, handles.as_ptr(), handles.len() as u32) }
    }

    pub fn bind_images(&self, stage: Stage, first_id: u32, handles: &[ResHandle]) {
        unsafe { dkCmdBufBindImages(self.0, stage as _, first_id, handles.as_ptr(), handles.len() as u32) }
    }

    pub fn bind_image_descriptor_set(&self, set_addr: DkGpuAddr, num_descriptors: u32) {
        unsafe { dkCmdBufBindImageDescriptorSet(self.0, set_addr, num_descriptors) }
    }

    pub fn bind_sampler_descriptor_set(&self, set_addr: DkGpuAddr, num_descriptors: u32) {
        unsafe { dkCmdBufBindSamplerDescriptorSet(self.0, set_addr, num_descriptors) }
    }

    pub fn bind_render_targets(&self, color_targets: &[&ImageView], depth_target: Option<&ImageView>) {
        unsafe {
            dkCmdBufBindRenderTargets(
                self.0,
                color_targets.as_ptr() as *const _,
                color_targets.len() as u32,
                depth_target.map_or(std::ptr::null(), |i| &i.0 as *const _),
            )
        }
    }

    pub fn bind_rasterizer_state(&self, state: &RasterizerState) {
        unsafe { dkCmdBufBindRasterizerState(self.0, &state.0) }
    }

    pub fn bind_multisample_state(&self, state: &MultisampleState) {
        unsafe { dkCmdBufBindMultisampleState(self.0, &state.0) }
    }

    pub fn bind_color_state(&self, state: &ColorState) {
        unsafe { dkCmdBufBindColorState(self.0, &state.0) }
    }

    pub fn bind_color_write_state(&self, state: &ColorWriteState) {
        unsafe { dkCmdBufBindColorWriteState(self.0, &state.0) }
    }

    pub fn bind_blend_states(&self, id: u32, state: &[BlendState]) {
        unsafe { dkCmdBufBindBlendStates(self.0, id, state.as_ptr() as *const _, state.len() as u32) }
    }

    pub fn bind_depth_stencil_state(&self, state: &DepthStencilState) {
        unsafe { dkCmdBufBindDepthStencilState(self.0, &state.0) }
    }

    pub fn bind_vtx_attrib_state(&self, attribs: &[VtxAttribState]) {
        unsafe { dkCmdBufBindVtxAttribState(self.0, attribs.as_ptr() as *const _, attribs.len() as u32) }
    }

    pub fn bind_vtx_buffer_state(&self, buffers: &[VtxBufferState]) {
        unsafe { dkCmdBufBindVtxBufferState(self.0, buffers.as_ptr() as *const _, buffers.len() as u32) }
    }

    pub fn bind_vtx_buffer(&self, id: u32, buf_addr: DkGpuAddr, buf_size: u32) {
        let buf_extents = [DkBufExtents { addr: buf_addr, size: buf_size }];
        self.bind_vtx_buffers(id, &buf_extents);
    }

    pub fn bind_vtx_buffers(&self, first_id: u32, buffers: &[BufExtents]) {
        unsafe { dkCmdBufBindVtxBuffers(self.0, first_id, buffers.as_ptr(), buffers.len() as u32) }
    }

    pub fn bind_idx_buffer(&self, format: IdxFormat, address: DkGpuAddr) {
        unsafe { dkCmdBufBindIdxBuffer(self.0, format as _, address) }
    }

    pub fn set_viewports(&self, first_id: u32, viewports: &[Viewport]) {
        unsafe { dkCmdBufSetViewports(self.0, first_id, viewports.as_ptr() as *const _, viewports.len() as u32) }
    }

    pub fn set_viewport_swizzles(&self, first_id: u32, swizzles: &[ViewportSwizzle]) {
        unsafe { dkCmdBufSetViewportSwizzles(self.0, first_id, swizzles.as_ptr() as *const _, swizzles.len() as u32) }
    }

    pub fn set_subpixel_precision_bias(&self, xbits: u32, ybits: u32) {
        unsafe { dkCmdBufSetSubpixelPrecisionBias(self.0, xbits, ybits) }
    }

    pub fn set_scissors(&self, first_id: u32, scissors: &[Scissor]) {
        unsafe { dkCmdBufSetScissors(self.0, first_id, scissors.as_ptr() as *const _, scissors.len() as u32) }
    }

    pub fn set_depth_bias(&self, constant_factor: f32, clamp: f32, slope_factor: f32) {
        unsafe { dkCmdBufSetDepthBias(self.0, constant_factor, clamp, slope_factor) }
    }

    pub fn set_point_size(&self, size: f32) {
        unsafe { dkCmdBufSetPointSize(self.0, size) }
    }

    pub fn set_line_width(&self, width: f32) {
        unsafe { dkCmdBufSetLineWidth(self.0, width) }
    }

    pub fn set_line_stipple(&self, enable: bool, factor: u32, pattern: u16) {
        unsafe { dkCmdBufSetLineStipple(self.0, enable, factor, pattern) }
    }

    pub fn set_polygon_stipple(&self, pattern: &[u32; 32]) {
        unsafe { dkCmdBufSetPolygonStipple(self.0, pattern.as_ptr()) }
    }

    pub fn set_conservative_raster_enable(&self, enable: bool) {
        unsafe { dkCmdBufSetConservativeRasterEnable(self.0, enable) }
    }

    pub fn set_conservative_raster_dilate(&self, dilate: f32) {
        unsafe { dkCmdBufSetConservativeRasterDilate(self.0, dilate) }
    }

    pub fn set_sample_mask(&self, mask: u32) {
        unsafe { dkCmdBufSetSampleMask(self.0, mask) }
    }

    pub fn set_coverage_modulation_table(&self, table: &[f32; 16]) {
        unsafe { dkCmdBufSetCoverageModulationTable(self.0, table.as_ptr()) }
    }

    pub fn set_depth_bounds(&self, enable: bool, near: f32, far: f32) {
        unsafe { dkCmdBufSetDepthBounds(self.0, enable, near, far) }
    }

    pub fn set_alpha_ref(&self, ref_: f32) {
        unsafe { dkCmdBufSetAlphaRef(self.0, ref_) }
    }

    pub fn set_blend_const(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { dkCmdBufSetBlendConst(self.0, red, green, blue, alpha) }
    }

    pub fn set_stencil(&self, face: Face, mask: u8, func_ref: u8, func_mask: u8) {
        unsafe { dkCmdBufSetStencil(self.0, face as _, mask, func_ref, func_mask) }
    }

    pub fn set_primitive_restart(&self, enable: bool, index: u32) {
        unsafe { dkCmdBufSetPrimitiveRestart(self.0, enable, index) }
    }

    pub fn set_patch_size(&self, size: u32) {
        unsafe { dkCmdBufSetPatchSize(self.0, size) }
    }

    pub fn set_tess_outer_levels(&self, level0: f32, level1: f32, level2: f32, level3: f32) {
        unsafe { dkCmdBufSetTessOuterLevels(self.0, level0, level1, level2, level3) }
    }

    pub fn set_tess_inner_levels(&self, level0: f32, level1: f32) {
        unsafe { dkCmdBufSetTessInnerLevels(self.0, level0, level1) }
    }

    pub fn set_tile_size(&self, width: u32, height: u32) {
        unsafe { dkCmdBufSetTileSize(self.0, width, height) }
    }

    pub fn tiled_cache_op(&self, op: TiledCacheOp) {
        unsafe { dkCmdBufTiledCacheOp(self.0, op as _) }
    }

    pub fn clear_color_float(&self, target: u32, clear_mask: ColorMask, r: f32, g: f32, b: f32, a: f32) {
        let data = [r, g, b, a];
        unsafe { dkCmdBufClearColor(self.0, target, clear_mask.bits, data.as_ptr() as *const _) }
    }

    pub fn clear_depth_stencil(&self, clear_depth: bool, depth_value: f32, stencil_mask: u8, stencil_value: u8) {
        unsafe { dkCmdBufClearDepthStencil(self.0, clear_depth, depth_value, stencil_mask, stencil_value) }
    }

    pub fn discard_color(&self, target_id: u32) {
        unsafe { dkCmdBufDiscardColor(self.0, target_id) }
    }

    pub fn discard_depth_stencil(&self) {
        unsafe { dkCmdBufDiscardDepthStencil(self.0) }
    }

    pub fn resolve_depth_values(&self) {
        unsafe { dkCmdBufResolveDepthValues(self.0) }
    }

    pub fn draw(&self, prim: Primitive, num_vertices: u32, num_instances: u32, first_vertex: u32, first_instance: u32) {
        unsafe { dkCmdBufDraw(self.0, prim as _, num_vertices, num_instances, first_vertex, first_instance) }
    }

    pub fn draw_indirect(&self, prim: Primitive, indirect: GpuAddr) {
        unsafe { dkCmdBufDrawIndirect(self.0, prim as _, indirect) }
    }

    pub fn draw_indexed(
        &self,
        prim: Primitive,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            dkCmdBufDrawIndexed(
                self.0,
                prim as _,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }

    pub fn draw_indexed_indirect(&self, prim: Primitive, indirect: GpuAddr) {
        unsafe { dkCmdBufDrawIndexedIndirect(self.0, prim as _, indirect) }
    }

    pub fn dispatch_compute(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
        unsafe { dkCmdBufDispatchCompute(self.0, num_groups_x, num_groups_y, num_groups_z) }
    }

    pub fn dispatch_compute_indirect(&self, indirect: GpuAddr) {
        unsafe { dkCmdBufDispatchComputeIndirect(self.0, indirect) }
    }

    pub fn push_constants(
        &self,
        ubo_addr: GpuAddr,
        ubo_size: u32,
        offset: u32,
        size: u32,
        data: *const std::ffi::c_void,
    ) {
        unsafe { dkCmdBufPushConstants(self.0, ubo_addr, ubo_size, offset, size, data) }
    }

    pub fn push_data(&self, addr: GpuAddr, data: *const std::ffi::c_void, size: u32) {
        unsafe { dkCmdBufPushData(self.0, addr, data, size) }
    }

    pub fn copy_buffer(&self, src_addr: GpuAddr, dst_addr: GpuAddr, size: u32) {
        unsafe { dkCmdBufCopyBuffer(self.0, src_addr, dst_addr, size) }
    }

    pub fn copy_image(
        &self,
        src_view: &ImageView,
        src_rect: &ImageRect,
        dst_view: &ImageView,
        dst_rect: &ImageRect,
        flags: u32,
    ) {
        unsafe { dkCmdBufCopyImage(self.0, &src_view.0, src_rect, &dst_view.0, dst_rect, flags) }
    }

    pub fn blit_image(
        &self,
        src_view: &ImageView,
        src_rect: &ImageRect,
        dst_view: &ImageView,
        dst_rect: &ImageRect,
        flags: u32,
        factor: u32,
    ) {
        unsafe { dkCmdBufBlitImage(self.0, &src_view.0, src_rect, &dst_view.0, dst_rect, flags, factor) }
    }

    pub fn resolve_image(&self, src_view: &ImageView, dst_view: &ImageView) {
        unsafe { dkCmdBufResolveImage(self.0, &src_view.0, &dst_view.0) }
    }

    pub fn copy_buffer_to_image(&self, src: &CopyBuf, dst_view: &ImageView, dst_rect: &ImageRect, flags: u32) {
        unsafe { dkCmdBufCopyBufferToImage(self.0, src, &dst_view.0, dst_rect, flags) }
    }

    pub fn copy_image_to_buffer(&self, src_view: &ImageView, src_rect: &ImageRect, dst: &CopyBuf, flags: u32) {
        unsafe { dkCmdBufCopyImageToBuffer(self.0, &src_view.0, src_rect, dst, flags) }
    }

    pub fn report_counter(&self, counter: Counter, addr: GpuAddr) {
        unsafe { dkCmdBufReportCounter(self.0, counter as _, addr) }
    }

    pub fn report_value(&self, value: u32, addr: GpuAddr) {
        unsafe { dkCmdBufReportValue(self.0, value, addr) }
    }

    pub fn reset_counter(&self, counter: Counter) {
        unsafe { dkCmdBufResetCounter(self.0, counter as _) }
    }
}

#[repr(C)]
pub struct Queue(DkQueue);

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe { dkQueueDestroy(self.0) }
    }
}

impl Queue {
    pub unsafe fn from_raw(queue: DkQueue) -> Self {
        Self(queue)
    }

    pub fn into_raw(self) -> DkQueue {
        self.0
    }

    pub fn is_in_error_state(&self) -> bool {
        unsafe { dkQueueIsInErrorState(self.0) }
    }

    pub fn wait_fence(&self, fence: &mut Fence) {
        unsafe { dkQueueWaitFence(self.0, &mut fence.0) }
    }

    pub fn signal_fence(&self, fence: &mut Fence, flush: bool) {
        unsafe { dkQueueSignalFence(self.0, &mut fence.0, flush) }
    }

    pub fn submit_commands(&self, cmds: DkCmdList) {
        unsafe { dkQueueSubmitCommands(self.0, cmds) }
    }

    pub fn flush(&self) {
        unsafe { dkQueueFlush(self.0) }
    }

    pub fn wait_idle(&self) {
        unsafe { dkQueueWaitIdle(self.0) }
    }

    pub fn acquire_image(&self, swapchain: &Swapchain) -> i32 {
        unsafe { dkQueueAcquireImage(self.0, swapchain.0) }
    }

    pub fn present_image(&self, swapchain: &Swapchain, image_slot: i32) {
        unsafe { dkQueuePresentImage(self.0, swapchain.0, image_slot) }
    }
}

#[repr(C)]
pub struct Shader(DkShader);

impl Shader {
    pub unsafe fn from_raw(shader: DkShader) -> Self {
        Self(shader)
    }

    pub fn into_raw(self) -> DkShader {
        self.0
    }

    pub fn new() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { dkShaderIsValid(&self.0) }
    }

    pub fn get_stage(&self) -> Stage {
        unsafe { Stage::from(dkShaderGetStage(&self.0)) }
    }
}

#[repr(C)]
pub struct ImageLayout(Pin<Box<DkImageLayout>>);

impl ImageLayout {
    pub unsafe fn from_raw(layout: DkImageLayout) -> Self {
        Self(Box::pin(layout))
    }

    pub fn into_raw(self) -> DkImageLayout {
        *self.0
    }

    pub fn new() -> Self {
        unsafe { Self(Box::pin(std::mem::zeroed())) }
    }

    pub fn get_size(&self) -> u64 {
        unsafe { dkImageLayoutGetSize(self.0.as_ref().get_ref()) }
    }

    pub fn get_alignment(&self) -> u32 {
        unsafe { dkImageLayoutGetAlignment(self.0.as_ref().get_ref()) }
    }
}

#[repr(C)]
pub struct Image(Pin<Box<DkImage>>);

impl Image {
    pub unsafe fn from_raw(image: DkImage) -> Self {
        Self(Box::pin(image))
    }

    pub fn into_raw(self) -> DkImage {
        *self.0
    }

    pub fn new() -> Self {
        unsafe { Self(Box::pin(std::mem::zeroed())) }
    }

    pub fn initialize(&mut self, layout: &ImageLayout, mem: &MemBlock, offset: u32) {
        unsafe { dkImageInitialize(self.0.as_mut().get_mut(), layout.0.as_ref().get_ref(), mem.0, offset) }
    }

    pub fn get_gpu_addr(&self) -> DkGpuAddr {
        unsafe { dkImageGetGpuAddr(self.0.as_ref().get_ref()) }
    }

    pub fn get_layout<'a: 'b, 'b>(&'a self) -> &'b ImageLayout {
        // We're assuming that Image and ImageLayout have the same memory layout.
        unsafe { &*(self as *const Image as *const ImageLayout) }
    }
}

#[repr(C)]
pub struct Swapchain(DkSwapchain);

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            dkSwapchainDestroy(self.0);
        }
    }
}

impl Swapchain {
    pub unsafe fn from_raw(swapchain: DkSwapchain) -> Self {
        Self(swapchain)
    }

    pub fn into_raw(self) -> DkSwapchain {
        self.0
    }

    pub fn acquire_image(&self, image_slot: &mut i32, fence: &mut Fence) {
        unsafe { dkSwapchainAcquireImage(self.0, image_slot, &mut fence.0) }
    }

    pub fn set_crop(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { dkSwapchainSetCrop(self.0, left, top, right, bottom) }
    }

    pub fn set_swap_interval(&self, interval: u32) {
        unsafe { dkSwapchainSetSwapInterval(self.0, interval) }
    }
}

#[repr(C)]
pub struct DeviceMaker(DkDeviceMaker);

impl DeviceMaker {
    pub unsafe fn from_raw(maker: DkDeviceMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkDeviceMaker {
        self.0
    }

    pub fn new() -> Self {
        unsafe {
            let mut maker: DkDeviceMaker = std::mem::zeroed();

            maker.userData = std::ptr::null_mut();
            maker.cbDebug = None;
            maker.cbAlloc = None;
            maker.cbFree = None;
            maker.flags = (DeviceFlags::DepthZeroToOne | DeviceFlags::OriginUpperLeft).bits;

            DeviceMaker(maker)
        }
    }

    pub unsafe fn set_user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.userData = user_data;
        self
    }

    pub unsafe fn set_debug_callback(mut self, cb_debug: DkDebugFunc) -> Self {
        self.0.cbDebug = cb_debug;
        self
    }

    pub unsafe fn set_alloc_callback(mut self, cb_alloc: DkAllocFunc) -> Self {
        self.0.cbAlloc = cb_alloc;
        self
    }

    pub unsafe fn set_free_callback(mut self, cb_free: DkFreeFunc) -> Self {
        self.0.cbFree = cb_free;
        self
    }

    pub fn get_debug_callback(&self) -> DkDebugFunc {
        self.0.cbDebug
    }

    pub fn get_alloc_callback(&self) -> DkAllocFunc {
        self.0.cbAlloc
    }

    pub fn get_free_callback(&self) -> DkFreeFunc {
        self.0.cbFree
    }

    pub fn set_flags(mut self, flags: DeviceFlags) -> Self {
        self.0.flags = flags.bits;
        self
    }

    pub fn get_flags(&self) -> DeviceFlags {
        DeviceFlags::from_bits_truncate(self.0.flags)
    }

    pub fn create(&self) -> Device {
        unsafe { Device(dkDeviceCreate(&self.0)) }
    }
}

#[repr(C)]
pub struct MemBlockMaker(DkMemBlockMaker);

impl MemBlockMaker {
    pub unsafe fn from_raw(maker: DkMemBlockMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkMemBlockMaker {
        self.0
    }

    pub fn new(device: &Device, size: u32) -> Self {
        unsafe {
            let mut maker: DkMemBlockMaker = std::mem::zeroed();

            maker.device = device.0;
            maker.size = size;
            maker.flags = DkMemBlockFlags_CpuUncached | DkMemBlockFlags_GpuCached;
            maker.storage = std::ptr::null_mut();

            MemBlockMaker(maker)
        }
    }

    pub fn set_flags(&mut self, flags: MemBlockFlags) -> &mut Self {
        self.0.flags = flags.bits;
        self
    }

    pub fn get_flags(&self) -> MemBlockFlags {
        MemBlockFlags::from_bits_truncate(self.0.flags)
    }

    pub fn set_storage(&mut self, storage: *mut std::ffi::c_void) -> &mut Self {
        self.0.storage = storage;
        self
    }

    pub fn get_storage(&self) -> *mut std::ffi::c_void {
        self.0.storage
    }

    pub fn create(&self) -> MemBlock {
        unsafe { MemBlock(dkMemBlockCreate(&self.0)) }
    }
}

#[repr(C)]
pub struct CmdBufMaker(DkCmdBufMaker);

impl CmdBufMaker {
    pub unsafe fn from_raw(maker: DkCmdBufMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkCmdBufMaker {
        self.0
    }

    pub fn new(device: &Device) -> Self {
        unsafe {
            let mut maker: DkCmdBufMaker = std::mem::zeroed();

            maker.device = device.0;
            maker.userData = std::ptr::null_mut();
            maker.cbAddMem = None;

            CmdBufMaker(maker)
        }
    }

    pub fn set_user_data(&mut self, user_data: *mut std::ffi::c_void) -> &mut Self {
        self.0.userData = user_data;
        self
    }

    pub fn set_add_mem_callback(&mut self, cb_add_mem: DkCmdBufAddMemFunc) -> &mut Self {
        self.0.cbAddMem = cb_add_mem;
        self
    }

    pub fn create(&self) -> CmdBuf {
        unsafe { CmdBuf(dkCmdBufCreate(&self.0)) }
    }
}

bitflags! {
    pub struct QueueFlags: u32 {
        const Graphics = DkQueueFlags_Graphics;
        const Compute = DkQueueFlags_Compute;
        const MediumPrio = DkQueueFlags_MediumPrio;
        const HighPrio = DkQueueFlags_HighPrio;
        const LowPrio = DkQueueFlags_LowPrio;
        const PrioMask = DkQueueFlags_PrioMask;
        const EnableZcull = DkQueueFlags_EnableZcull;
        const DisableZcull = DkQueueFlags_DisableZcull;
    }
}

#[repr(C)]
pub struct QueueMaker(DkQueueMaker);

impl QueueMaker {
    pub unsafe fn from_raw(maker: DkQueueMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkQueueMaker {
        self.0
    }

    pub fn new(device: &Device) -> Self {
        unsafe {
            let mut maker: DkQueueMaker = std::mem::zeroed();

            maker.device = device.0;
            maker.flags =
                DkQueueFlags_Graphics | DkQueueFlags_Compute | DkQueueFlags_MediumPrio | DkQueueFlags_EnableZcull;
            maker.commandMemorySize = DK_QUEUE_MIN_CMDMEM_SIZE;
            maker.flushThreshold = DK_QUEUE_MIN_CMDMEM_SIZE / 8;
            maker.perWarpScratchMemorySize = 4 * DK_PER_WARP_SCRATCH_MEM_ALIGNMENT;
            maker.maxConcurrentComputeJobs = DK_DEFAULT_MAX_COMPUTE_CONCURRENT_JOBS;

            QueueMaker(maker)
        }
    }

    pub fn set_flags(&mut self, flags: QueueFlags) -> &mut Self {
        self.0.flags = flags.bits;
        self
    }

    pub fn get_flags(&self) -> QueueFlags {
        QueueFlags::from_bits_truncate(self.0.flags)
    }

    pub fn set_command_memory_size(&mut self, command_memory_size: u32) -> &mut Self {
        self.0.commandMemorySize = command_memory_size;
        self
    }

    pub fn get_command_memory_size(&self) -> u32 {
        self.0.commandMemorySize
    }

    pub fn set_flush_threshold(&mut self, flush_threshold: u32) -> &mut Self {
        self.0.flushThreshold = flush_threshold;
        self
    }

    pub fn get_flush_threshold(&self) -> u32 {
        self.0.flushThreshold
    }

    pub fn set_per_warp_scratch_memory_size(&mut self, per_warp_scratch_memory_size: u32) -> &mut Self {
        self.0.perWarpScratchMemorySize = per_warp_scratch_memory_size;
        self
    }

    pub fn get_per_warp_scratch_memory_size(&self) -> u32 {
        self.0.perWarpScratchMemorySize
    }

    pub fn set_max_concurrent_compute_jobs(&mut self, max_concurrent_compute_jobs: u32) -> &mut Self {
        self.0.maxConcurrentComputeJobs = max_concurrent_compute_jobs;
        self
    }

    pub fn get_max_concurrent_compute_jobs(&self) -> u32 {
        self.0.maxConcurrentComputeJobs
    }

    pub fn create(&self) -> Queue {
        unsafe { Queue(dkQueueCreate(&self.0)) }
    }
}

#[repr(C)]
pub struct ShaderMaker(DkShaderMaker);

impl ShaderMaker {
    pub unsafe fn from_raw(maker: DkShaderMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkShaderMaker {
        self.0
    }

    pub fn new(code_mem: &MemBlock, code_offset: u32) -> Self {
        unsafe {
            let mut maker: DkShaderMaker = std::mem::zeroed();

            maker.codeMem = code_mem.0;
            maker.control = std::ptr::null();
            maker.codeOffset = code_offset;
            maker.programId = 0;

            ShaderMaker(maker)
        }
    }

    pub fn set_control(&mut self, control: *const std::ffi::c_void) -> &mut Self {
        self.0.control = control;
        self
    }

    pub fn get_control(&self) -> *const std::ffi::c_void {
        self.0.control
    }

    pub fn set_program_id(&mut self, program_id: u32) -> &mut Self {
        self.0.programId = program_id;
        self
    }

    pub fn get_program_id(&self) -> u32 {
        self.0.programId
    }

    pub fn initialize(&self, obj: &mut Shader) {
        unsafe {
            dkShaderInitialize(&mut obj.0, &self.0);
        }
    }
}

#[repr(C)]
pub struct ImageLayoutMaker(DkImageLayoutMaker);

impl ImageLayoutMaker {
    pub unsafe fn from_raw(maker: DkImageLayoutMaker) -> Self {
        Self(maker)
    }

    pub fn into_raw(self) -> DkImageLayoutMaker {
        self.0
    }

    pub fn new(device: &Device) -> Self {
        unsafe {
            let mut maker: DkImageLayoutMaker = std::mem::zeroed();

            maker.device = device.0;
            maker.type_ = DkImageType_DkImageType_2D;
            maker.flags = 0;
            maker.format = DkImageFormat_DkImageFormat_None;
            maker.msMode = DkMsMode_DkMsMode_1x;
            maker.dimensions[0] = 0;
            maker.dimensions[1] = 0;
            maker.dimensions[2] = 0;
            maker.mipLevels = 1;
            maker.__bindgen_anon_1.pitchStride = 0;

            ImageLayoutMaker(maker)
        }
    }

    pub fn set_type(&mut self, type_: ImageType) -> &mut Self {
        self.0.type_ = type_ as _;
        self
    }

    pub fn get_type(&self) -> ImageType {
        unsafe { ImageType::from(self.0.type_) }
    }

    pub fn set_flags(&mut self, flags: ImageFlags) -> &mut Self {
        self.0.flags = flags.bits();
        self
    }

    pub fn get_flags(&self) -> ImageFlags {
        ImageFlags::from_bits_truncate(self.0.flags)
    }

    pub fn set_format(&mut self, format: ImageFormat) -> &mut Self {
        self.0.format = format as _;
        self
    }

    pub fn get_format(&self) -> ImageFormat {
        unsafe { ImageFormat::from(self.0.format) }
    }

    pub fn set_ms_mode(&mut self, ms_mode: MsMode) -> &mut Self {
        self.0.msMode = ms_mode as _;
        self
    }

    pub fn get_ms_mode(&self) -> MsMode {
        unsafe { MsMode::from(self.0.msMode) }
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32, depth: u32) -> &mut Self {
        self.0.dimensions[0] = width;
        self.0.dimensions[1] = height;
        self.0.dimensions[2] = depth;
        self
    }

    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.0.dimensions[0], self.0.dimensions[1], self.0.dimensions[2])
    }

    pub fn set_mip_levels(&mut self, mip_levels: u32) -> &mut Self {
        self.0.mipLevels = mip_levels;
        self
    }

    pub fn get_mip_levels(&self) -> u32 {
        self.0.mipLevels
    }

    pub fn set_pitch_stride(&mut self, pitch_stride: u32) -> &mut Self {
        self.0.__bindgen_anon_1.pitchStride = pitch_stride;
        self
    }

    pub fn set_tile_size(&mut self, tile_size: TileSize) -> &mut Self {
        self.0.__bindgen_anon_1.tileSize = tile_size as _;
        self
    }

    pub fn initialize(&self, obj: &mut ImageLayout) {
        unsafe {
            dkImageLayoutInitialize(obj.0.as_mut().get_mut(), &self.0);
        }
    }
}

pub struct ImageView<'a>(DkImageView, PhantomData<&'a Image>);

impl<'a> ImageView<'a> {
    pub fn into_raw(self) -> DkImageView {
        self.0
    }

    pub fn new(image: &Image) -> ImageView {
        ImageView(
            DkImageView {
                pImage: image.0.as_ref().get_ref(),
                type_: DkImageType_DkImageType_None,
                format: DkImageFormat_DkImageFormat_None,
                swizzle: [
                    DkImageSwizzle_DkImageSwizzle_Red,
                    DkImageSwizzle_DkImageSwizzle_Green,
                    DkImageSwizzle_DkImageSwizzle_Blue,
                    DkImageSwizzle_DkImageSwizzle_Alpha,
                ],
                dsSource: DkDsSource_DkDsSource_Depth,
                layerOffset: 0,
                layerCount: 0,
                mipLevelOffset: 0,
                mipLevelCount: 0,
            },
            PhantomData,
        )
    }
}

#[repr(C)]
pub struct ImageDescriptor(DkImageDescriptor);

impl ImageDescriptor {
    pub fn new() -> Self {
        unsafe { ImageDescriptor(MaybeUninit::<DkImageDescriptor>::zeroed().assume_init()) }
    }

    // void initialize(ImageView const& view, bool usesLoadOrStore = false, bool decayMS = false);
    pub fn initialize(&mut self, view: &ImageView, uses_load_or_store: bool, decay_ms: bool) {
        unsafe { dkImageDescriptorInitialize(&mut self.0, &view.0, uses_load_or_store, decay_ms) }
    }
}

#[repr(C)]
pub struct Sampler(DkSampler);

impl Sampler {
    pub fn new() -> Self {
        unsafe {
            Sampler(DkSampler {
                minFilter: DkFilter_DkFilter_Nearest,
                magFilter: DkFilter_DkFilter_Nearest,
                mipFilter: DkMipFilter_DkMipFilter_None,
                wrapMode: [DkWrapMode_DkWrapMode_Repeat, DkWrapMode_DkWrapMode_Repeat, DkWrapMode_DkWrapMode_Repeat],
                lodClampMin: 0.0,
                lodClampMax: 1000.0,
                lodBias: 0.0,
                lodSnap: 0.0,
                compareEnable: false,
                compareOp: DkCompareOp_DkCompareOp_Less,
                borderColor: [
                    DkSampler__bindgen_ty_1 { value_ui: 0 },
                    DkSampler__bindgen_ty_1 { value_ui: 0 },
                    DkSampler__bindgen_ty_1 { value_ui: 0 },
                    DkSampler__bindgen_ty_1 { value_ui: 0 },
                ],
                maxAnisotropy: 1.0,
                reductionMode: DkSamplerReduction_DkSamplerReduction_WeightedAverage,
            })
        }
    }

    pub fn set_filter(&mut self, min: Filter, mag: Filter, mip: MipFilter) -> &mut Self {
        self.0.minFilter = min as _;
        self.0.magFilter = mag as _;
        self.0.mipFilter = mip as _;
        self
    }

    pub fn set_wrap_mode(&mut self, u: WrapMode, v: WrapMode, p: WrapMode) -> &mut Self {
        self.0.wrapMode[0] = u as _;
        self.0.wrapMode[1] = v as _;
        self.0.wrapMode[2] = p as _;
        self
    }

    pub fn set_lod_clamp(&mut self, min: f32, max: f32) -> &mut Self {
        self.0.lodClampMin = min;
        self.0.lodClampMax = max;
        self
    }

    pub fn set_lod_bias(&mut self, bias: f32) -> &mut Self {
        self.0.lodBias = bias;
        self
    }

    pub fn set_lod_snap(&mut self, snap: f32) -> &mut Self {
        self.0.lodSnap = snap;
        self
    }

    pub fn set_depth_compare(&mut self, enable: bool, op: CompareOp) -> &mut Self {
        self.0.compareEnable = enable;
        self.0.compareOp = op as _;
        self
    }

    pub fn set_border_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.0.borderColor[0].value_f = r;
        self.0.borderColor[1].value_f = g;
        self.0.borderColor[2].value_f = b;
        self.0.borderColor[3].value_f = a;
        self
    }

    pub fn set_border_color_ui(&mut self, r: u32, g: u32, b: u32, a: u32) -> &mut Self {
        self.0.borderColor[0].value_ui = r;
        self.0.borderColor[1].value_ui = g;
        self.0.borderColor[2].value_ui = b;
        self.0.borderColor[3].value_ui = a;
        self
    }

    pub fn set_border_color_i(&mut self, r: i32, g: i32, b: i32, a: i32) -> &mut Self {
        self.0.borderColor[0].value_i = r;
        self.0.borderColor[1].value_i = g;
        self.0.borderColor[2].value_i = b;
        self.0.borderColor[3].value_i = a;
        self
    }

    pub fn set_max_anisotropy(&mut self, max: f32) -> &mut Self {
        self.0.maxAnisotropy = max;
        self
    }

    pub fn set_reduction_mode(&mut self, mode: SamplerReduction) -> &mut Self {
        self.0.reductionMode = mode as _;
        self
    }
}

#[repr(C)]
pub struct SamplerDescriptor(DkSamplerDescriptor);

impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe { SamplerDescriptor(MaybeUninit::<DkSamplerDescriptor>::zeroed().assume_init()) }
    }

    pub fn initialize(&mut self, sampler: &Sampler) {
        unsafe { dkSamplerDescriptorInitialize(&mut self.0, &sampler.0) }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PolygonMode {
    Point = DkPolygonMode_DkPolygonMode_Point,
    Line = DkPolygonMode_DkPolygonMode_Line,
    Fill = DkPolygonMode_DkPolygonMode_Fill,
}

impl From<DkPolygonMode> for PolygonMode {
    fn from(mode: DkPolygonMode) -> Self {
        match mode {
            DkPolygonMode_DkPolygonMode_Point => PolygonMode::Point,
            DkPolygonMode_DkPolygonMode_Line => PolygonMode::Line,
            DkPolygonMode_DkPolygonMode_Fill => PolygonMode::Fill,
            _ => unreachable!("Invalid DkPolygonMode value"),
        }
    }
}

bitflags! {
    pub struct PolygonFlag: u32 {
        const Point = DkPolygonFlag_Point;
        const Line = DkPolygonFlag_Line;
        const Fill = DkPolygonFlag_Fill;
        const All = DkPolygonFlag_All;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Face {
    None = DkFace_DkFace_None,
    Front = DkFace_DkFace_Front,
    Back = DkFace_DkFace_Back,
    FrontAndBack = DkFace_DkFace_FrontAndBack,
}

impl From<DkFace> for Face {
    fn from(face: DkFace) -> Self {
        match face {
            DkFace_DkFace_None => Face::None,
            DkFace_DkFace_Front => Face::Front,
            DkFace_DkFace_Back => Face::Back,
            DkFace_DkFace_FrontAndBack => Face::FrontAndBack,
            _ => unreachable!("Invalid DkFace value"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrontFace {
    CW = DkFrontFace_DkFrontFace_CW,
    CCW = DkFrontFace_DkFrontFace_CCW,
}

impl From<DkFrontFace> for FrontFace {
    fn from(face: DkFrontFace) -> Self {
        match face {
            DkFrontFace_DkFrontFace_CW => FrontFace::CW,
            DkFrontFace_DkFrontFace_CCW => FrontFace::CCW,
            _ => unreachable!("Invalid DkFrontFace value"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProvokingVertex {
    First = DkProvokingVertex_DkProvokingVertex_First,
    Last = DkProvokingVertex_DkProvokingVertex_Last,
}

impl From<DkProvokingVertex> for ProvokingVertex {
    fn from(vertex: DkProvokingVertex) -> Self {
        match vertex {
            DkProvokingVertex_DkProvokingVertex_First => ProvokingVertex::First,
            DkProvokingVertex_DkProvokingVertex_Last => ProvokingVertex::Last,
            _ => unreachable!("Invalid DkProvokingVertex value"),
        }
    }
}

#[repr(C)]
pub struct RasterizerState(DkRasterizerState);

impl RasterizerState {
    pub fn new() -> RasterizerState {
        let state = DkRasterizerState::new_bitfield_1(
            1,
            0,
            0,
            DkPolygonMode_DkPolygonMode_Fill,
            DkPolygonMode_DkPolygonMode_Fill,
            DkFace_DkFace_Back,
            DkFrontFace_DkFrontFace_CCW,
            DkProvokingVertex_DkProvokingVertex_Last,
            0,
            0,
        );

        RasterizerState(DkRasterizerState { _bitfield_align_1: [], _bitfield_1: state, __bindgen_padding_0: 0 })
    }

    pub fn set_rasterizer_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_rasterizerEnable(enable as _);
        self
    }

    pub fn set_depth_clamp_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_depthClampEnable(enable as _);
        self
    }

    pub fn set_fill_rectangle_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_fillRectangleEnable(enable as _);
        self
    }

    pub fn set_polygon_mode(&mut self, mode: PolygonMode) -> &mut Self {
        self.0.set_polygonModeFront(mode as _);
        self.0.set_polygonModeBack(mode as _);
        self
    }

    pub fn set_polygon_mode_front(&mut self, mode: PolygonMode) -> &mut Self {
        self.0.set_polygonModeFront(mode as _);
        self
    }

    pub fn set_polygon_mode_back(&mut self, mode: PolygonMode) -> &mut Self {
        self.0.set_polygonModeBack(mode as _);
        self
    }

    pub fn set_cull_mode(&mut self, cull_mode: Face) -> &mut Self {
        self.0.set_cullMode(cull_mode as _);
        self
    }

    pub fn set_front_face(&mut self, front_face: FrontFace) -> &mut Self {
        self.0.set_frontFace(front_face as _);
        self
    }

    pub fn set_provoking_vertex(&mut self, provoking_vertex: ProvokingVertex) -> &mut Self {
        self.0.set_provokingVertex(provoking_vertex as _);
        self
    }

    pub fn set_polygon_smooth_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_polygonSmoothEnableMask(if enable { PolygonFlag::All.bits() } else { 0 });
        self
    }

    pub fn set_polygon_smooth_enable_mask(&mut self, mask: u32) -> &mut Self {
        self.0.set_polygonSmoothEnableMask(mask);
        self
    }

    pub fn set_depth_bias_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_depthBiasEnableMask(if enable { PolygonFlag::All.bits() } else { 0 });
        self
    }

    pub fn set_depth_bias_enable_mask(&mut self, mask: u32) -> &mut Self {
        self.0.set_depthBiasEnableMask(mask);
        self
    }

    pub fn get_rasterizer_enable(&self) -> bool {
        self.0.rasterizerEnable() != 0
    }

    pub fn get_depth_clamp_enable(&self) -> bool {
        self.0.depthClampEnable() != 0
    }

    pub fn get_fill_rectangle_enable(&self) -> bool {
        self.0.fillRectangleEnable() != 0
    }

    pub fn get_polygon_mode_front(&self) -> PolygonMode {
        PolygonMode::from(self.0.polygonModeFront())
    }

    pub fn get_polygon_mode_back(&self) -> PolygonMode {
        PolygonMode::from(self.0.polygonModeBack())
    }

    pub fn get_cull_mode(&self) -> Face {
        Face::from(self.0.cullMode())
    }

    pub fn get_front_face(&self) -> FrontFace {
        FrontFace::from(self.0.frontFace())
    }

    pub fn get_provoking_vertex(&self) -> ProvokingVertex {
        ProvokingVertex::from(self.0.provokingVertex())
    }

    pub fn get_polygon_smooth_enable(&self) -> bool {
        self.0.polygonSmoothEnableMask() != 0
    }

    pub fn get_polygon_smooth_enable_mask(&self) -> u32 {
        self.0.polygonSmoothEnableMask()
    }

    pub fn get_depth_bias_enable(&self) -> bool {
        self.0.depthBiasEnableMask() != 0
    }

    pub fn get_depth_bias_enable_mask(&self) -> u32 {
        self.0.depthBiasEnableMask()
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CoverageModulation {
    None = DkCoverageModulation_DkCoverageModulation_None,
    Rgb = DkCoverageModulation_DkCoverageModulation_Rgb,
    Alpha = DkCoverageModulation_DkCoverageModulation_Alpha,
    Rgba = DkCoverageModulation_DkCoverageModulation_Rgba,
}

impl From<DkCoverageModulation> for CoverageModulation {
    fn from(value: DkCoverageModulation) -> Self {
        match value {
            DkCoverageModulation_DkCoverageModulation_None => CoverageModulation::None,
            DkCoverageModulation_DkCoverageModulation_Rgb => CoverageModulation::Rgb,
            DkCoverageModulation_DkCoverageModulation_Alpha => CoverageModulation::Alpha,
            DkCoverageModulation_DkCoverageModulation_Rgba => CoverageModulation::Rgba,
            _ => unreachable!("Invalid DkCoverageModulation value"),
        }
    }
}

#[repr(C)]
pub struct MultisampleState(DkMultisampleState);

pub type SampleLocation = DkSampleLocation;

impl MultisampleState {
    pub fn new() -> Self {
        let state = DkMultisampleState::new_bitfield_1(
            DkMsMode_DkMsMode_1x,
            DkMsMode_DkMsMode_1x,
            0,
            1,
            0,
            0,
            DkCoverageModulation_DkCoverageModulation_None,
        );

        MultisampleState(DkMultisampleState {
            _bitfield_align_1: [],
            _bitfield_1: state,
            sampleLocations: [0x88888888, 0x88888888, 0x88888888, 0x88888888],
        })
    }

    pub fn set_mode(&mut self, mode: MsMode) -> &mut Self {
        self.0.set_mode(mode as _);
        self
    }

    pub fn set_rasterizer_mode(&mut self, mode: MsMode) -> &mut Self {
        self.0.set_rasterizerMode(mode as _);
        self
    }

    pub fn set_alpha_to_coverage_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_alphaToCoverageEnable(enable as _);
        self
    }

    pub fn set_alpha_to_coverage_dither(&mut self, dither: bool) -> &mut Self {
        self.0.set_alphaToCoverageDither(dither as _);
        self
    }

    pub fn set_coverage_to_color_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_coverageToColorEnable(enable as _);
        self
    }

    pub fn set_coverage_to_color_output(&mut self, output: u32) -> &mut Self {
        self.0.set_coverageToColorOutput(output);
        self
    }

    pub fn set_coverage_modulation(&mut self, modulation: CoverageModulation) -> &mut Self {
        self.0.set_coverageModulation(modulation as _);
        self
    }

    pub fn set_locations(&mut self, locations: &[SampleLocation]) -> &mut Self {
        unsafe {
            if locations.len() > 0 {
                dkMultisampleStateSetLocations(&mut self.0, locations.as_ptr(), locations.len() as _);
            } else {
                dkMultisampleStateSetLocations(&mut self.0, std::ptr::null(), 0);
            }
        }
        self
    }

    pub fn get_mode(&self) -> MsMode {
        MsMode::from(self.0.mode())
    }

    pub fn get_rasterizer_mode(&self) -> MsMode {
        MsMode::from(self.0.rasterizerMode())
    }

    pub fn get_alpha_to_coverage_enable(&self) -> bool {
        self.0.alphaToCoverageEnable() != 0
    }

    pub fn get_alpha_to_coverage_dither(&self) -> bool {
        self.0.alphaToCoverageDither() != 0
    }

    pub fn get_coverage_to_color_enable(&self) -> bool {
        self.0.coverageToColorEnable() != 0
    }

    pub fn get_coverage_to_color_output(&self) -> u32 {
        self.0.coverageToColorOutput()
    }

    pub fn get_coverage_modulation(&self) -> CoverageModulation {
        CoverageModulation::from(self.0.coverageModulation())
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogicOp {
    Clear = DkLogicOp_DkLogicOp_Clear,
    And = DkLogicOp_DkLogicOp_And,
    AndReverse = DkLogicOp_DkLogicOp_AndReverse,
    Copy = DkLogicOp_DkLogicOp_Copy,
    AndInverted = DkLogicOp_DkLogicOp_AndInverted,
    NoOp = DkLogicOp_DkLogicOp_NoOp,
    Xor = DkLogicOp_DkLogicOp_Xor,
    Or = DkLogicOp_DkLogicOp_Or,
    Nor = DkLogicOp_DkLogicOp_Nor,
    Equivalent = DkLogicOp_DkLogicOp_Equivalent,
    Invert = DkLogicOp_DkLogicOp_Invert,
    OrReverse = DkLogicOp_DkLogicOp_OrReverse,
    CopyInverted = DkLogicOp_DkLogicOp_CopyInverted,
    OrInverted = DkLogicOp_DkLogicOp_OrInverted,
    Nand = DkLogicOp_DkLogicOp_Nand,
    Set = DkLogicOp_DkLogicOp_Set,
}

impl From<DkLogicOp> for LogicOp {
    fn from(value: DkLogicOp) -> Self {
        match value {
            DkLogicOp_DkLogicOp_Clear => LogicOp::Clear,
            DkLogicOp_DkLogicOp_And => LogicOp::And,
            DkLogicOp_DkLogicOp_AndReverse => LogicOp::AndReverse,
            DkLogicOp_DkLogicOp_Copy => LogicOp::Copy,
            DkLogicOp_DkLogicOp_AndInverted => LogicOp::AndInverted,
            DkLogicOp_DkLogicOp_NoOp => LogicOp::NoOp,
            DkLogicOp_DkLogicOp_Xor => LogicOp::Xor,
            DkLogicOp_DkLogicOp_Or => LogicOp::Or,
            DkLogicOp_DkLogicOp_Nor => LogicOp::Nor,
            DkLogicOp_DkLogicOp_Equivalent => LogicOp::Equivalent,
            DkLogicOp_DkLogicOp_Invert => LogicOp::Invert,
            DkLogicOp_DkLogicOp_OrReverse => LogicOp::OrReverse,
            DkLogicOp_DkLogicOp_CopyInverted => LogicOp::CopyInverted,
            DkLogicOp_DkLogicOp_OrInverted => LogicOp::OrInverted,
            DkLogicOp_DkLogicOp_Nand => LogicOp::Nand,
            DkLogicOp_DkLogicOp_Set => LogicOp::Set,
            _ => unreachable!("Invalid DkLogicOp value"),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorState(DkColorState);

impl ColorState {
    pub fn new() -> Self {
        let state = DkColorState::new_bitfield_1(0, DkLogicOp_DkLogicOp_Copy, DkCompareOp_DkCompareOp_Always);

        ColorState(DkColorState { _bitfield_align_1: [], _bitfield_1: state })
    }

    pub fn set_blend_enable(&mut self, id: u32, enable: bool) -> &mut Self {
        if enable {
            self.0.set_blendEnableMask(self.0.blendEnableMask() | (1 << id));
        } else {
            self.0.set_blendEnableMask(self.0.blendEnableMask() & !(1 << id));
        }
        self
    }

    pub fn set_blend_enable_mask(&mut self, mask: u8) -> &mut Self {
        self.0.set_blendEnableMask(mask as u32);
        self
    }

    pub fn set_logic_op(&mut self, op: LogicOp) -> &mut Self {
        self.0.set_logicOp(op as _);
        self
    }

    pub fn set_alpha_compare_op(&mut self, op: CompareOp) -> &mut Self {
        self.0.set_alphaCompareOp(op as _);
        self
    }

    pub fn get_blend_enable(&self, id: u32) -> bool {
        (self.0.blendEnableMask() & (1 << id)) != 0
    }

    pub fn get_blend_enable_mask(&self) -> u8 {
        self.0.blendEnableMask() as u8
    }

    pub fn get_logic_op(&self) -> LogicOp {
        LogicOp::from(self.0.logicOp())
    }

    pub fn get_alpha_compare_op(&self) -> CompareOp {
        CompareOp::from(self.0.alphaCompareOp())
    }
}

bitflags! {
    pub struct ColorMask: u32 {
        const R = DkColorMask_R;
        const G = DkColorMask_G;
        const B = DkColorMask_B;
        const A = DkColorMask_A;

        const RGB = DkColorMask_RGB;
        const RGBA = DkColorMask_RGBA;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorWriteState(DkColorWriteState);

impl ColorWriteState {
    pub fn new() -> Self {
        unsafe {
            let mut state: DkColorWriteState = std::mem::zeroed();

            state.masks = 0xffffffff;

            ColorWriteState(state)
        }
    }

    pub fn set_mask(&mut self, id: u32, mask: u32) -> &mut Self {
        self.0.masks &= !(0xF << (id * 4));
        self.0.masks |= (mask & 0xF) << (id * 4);

        self
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Add = DkBlendOp_DkBlendOp_Add,
    Sub = DkBlendOp_DkBlendOp_Sub,
    RevSub = DkBlendOp_DkBlendOp_RevSub,
    Min = DkBlendOp_DkBlendOp_Min,
    Max = DkBlendOp_DkBlendOp_Max,
}

impl From<DkBlendOp> for BlendOp {
    fn from(value: DkBlendOp) -> Self {
        match value {
            DkBlendOp_DkBlendOp_Add => BlendOp::Add,
            DkBlendOp_DkBlendOp_Sub => BlendOp::Sub,
            DkBlendOp_DkBlendOp_RevSub => BlendOp::RevSub,
            DkBlendOp_DkBlendOp_Min => BlendOp::Min,
            DkBlendOp_DkBlendOp_Max => BlendOp::Max,
            _ => unreachable!("Invalid DkBlendOp value"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero = DkBlendFactor_DkBlendFactor_Zero,
    One = DkBlendFactor_DkBlendFactor_One,
    SrcColor = DkBlendFactor_DkBlendFactor_SrcColor,
    InvSrcColor = DkBlendFactor_DkBlendFactor_InvSrcColor,
    SrcAlpha = DkBlendFactor_DkBlendFactor_SrcAlpha,
    InvSrcAlpha = DkBlendFactor_DkBlendFactor_InvSrcAlpha,
    DstAlpha = DkBlendFactor_DkBlendFactor_DstAlpha,
    InvDstAlpha = DkBlendFactor_DkBlendFactor_InvDstAlpha,
    DstColor = DkBlendFactor_DkBlendFactor_DstColor,
    InvDstColor = DkBlendFactor_DkBlendFactor_InvDstColor,
    SrcAlphaSaturate = DkBlendFactor_DkBlendFactor_SrcAlphaSaturate,
    Src1Color = DkBlendFactor_DkBlendFactor_Src1Color,
    InvSrc1Color = DkBlendFactor_DkBlendFactor_InvSrc1Color,
    Src1Alpha = DkBlendFactor_DkBlendFactor_Src1Alpha,
    InvSrc1Alpha = DkBlendFactor_DkBlendFactor_InvSrc1Alpha,
    ConstColor = DkBlendFactor_DkBlendFactor_ConstColor,
    InvConstColor = DkBlendFactor_DkBlendFactor_InvConstColor,
    ConstAlpha = DkBlendFactor_DkBlendFactor_ConstAlpha,
    InvConstAlpha = DkBlendFactor_DkBlendFactor_InvConstAlpha,
}

impl From<DkBlendFactor> for BlendFactor {
    fn from(value: DkBlendFactor) -> Self {
        match value {
            DkBlendFactor_DkBlendFactor_Zero => BlendFactor::Zero,
            DkBlendFactor_DkBlendFactor_One => BlendFactor::One,
            DkBlendFactor_DkBlendFactor_SrcColor => BlendFactor::SrcColor,
            DkBlendFactor_DkBlendFactor_InvSrcColor => BlendFactor::InvSrcColor,
            DkBlendFactor_DkBlendFactor_SrcAlpha => BlendFactor::SrcAlpha,
            DkBlendFactor_DkBlendFactor_InvSrcAlpha => BlendFactor::InvSrcAlpha,
            DkBlendFactor_DkBlendFactor_DstAlpha => BlendFactor::DstAlpha,
            DkBlendFactor_DkBlendFactor_InvDstAlpha => BlendFactor::InvDstAlpha,
            DkBlendFactor_DkBlendFactor_DstColor => BlendFactor::DstColor,
            DkBlendFactor_DkBlendFactor_InvDstColor => BlendFactor::InvDstColor,
            DkBlendFactor_DkBlendFactor_SrcAlphaSaturate => BlendFactor::SrcAlphaSaturate,
            DkBlendFactor_DkBlendFactor_Src1Color => BlendFactor::Src1Color,
            DkBlendFactor_DkBlendFactor_InvSrc1Color => BlendFactor::InvSrc1Color,
            DkBlendFactor_DkBlendFactor_Src1Alpha => BlendFactor::Src1Alpha,
            DkBlendFactor_DkBlendFactor_InvSrc1Alpha => BlendFactor::InvSrc1Alpha,
            DkBlendFactor_DkBlendFactor_ConstColor => BlendFactor::ConstColor,
            DkBlendFactor_DkBlendFactor_InvConstColor => BlendFactor::InvConstColor,
            DkBlendFactor_DkBlendFactor_ConstAlpha => BlendFactor::ConstAlpha,
            DkBlendFactor_DkBlendFactor_InvConstAlpha => BlendFactor::InvConstAlpha,
            _ => unreachable!("Invalid DkBlendFactor value"),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlendState(DkBlendState);

impl BlendState {
    pub fn new() -> Self {
        let state = DkBlendState::new_bitfield_1(
            DkBlendOp_DkBlendOp_Add,
            DkBlendFactor_DkBlendFactor_SrcAlpha,
            DkBlendFactor_DkBlendFactor_InvSrcAlpha,
            DkBlendOp_DkBlendOp_Add,
            DkBlendFactor_DkBlendFactor_One,
            DkBlendFactor_DkBlendFactor_Zero,
        );

        BlendState(DkBlendState { _bitfield_align_1: [], _bitfield_1: state })
    }

    pub fn set_ops(&mut self, color_blend_op: BlendOp, alpha_blend_op: BlendOp) -> &mut Self {
        self.0.set_colorBlendOp(color_blend_op as _);
        self.0.set_alphaBlendOp(alpha_blend_op as _);

        self
    }

    pub fn set_factors(
        &mut self,
        src_color_blend_factor: BlendFactor,
        dst_color_blend_factor: BlendFactor,
        src_alpha_blend_factor: BlendFactor,
        dst_alpha_blend_factor: BlendFactor,
    ) -> &mut Self {
        self.0.set_srcColorBlendFactor(src_color_blend_factor as _);
        self.0.set_dstColorBlendFactor(dst_color_blend_factor as _);
        self.0.set_srcAlphaBlendFactor(src_alpha_blend_factor as _);
        self.0.set_dstAlphaBlendFactor(dst_alpha_blend_factor as _);

        self
    }

    pub fn set_color_blend_op(&mut self, color_blend_op: BlendOp) -> &mut Self {
        self.0.set_colorBlendOp(color_blend_op as _);

        self
    }

    pub fn set_src_color_blend_factor(&mut self, src_color_blend_factor: BlendFactor) -> &mut Self {
        self.0.set_srcColorBlendFactor(src_color_blend_factor as _);

        self
    }

    pub fn set_dst_color_blend_factor(&mut self, dst_color_blend_factor: BlendFactor) -> &mut Self {
        self.0.set_dstColorBlendFactor(dst_color_blend_factor as _);

        self
    }

    pub fn set_alpha_blend_op(&mut self, alpha_blend_op: BlendOp) -> &mut Self {
        self.0.set_alphaBlendOp(alpha_blend_op as _);

        self
    }

    pub fn set_src_alpha_blend_factor(&mut self, src_alpha_blend_factor: BlendFactor) -> &mut Self {
        self.0.set_srcAlphaBlendFactor(src_alpha_blend_factor as _);

        self
    }

    pub fn set_dst_alpha_blend_factor(&mut self, dst_alpha_blend_factor: BlendFactor) -> &mut Self {
        self.0.set_dstAlphaBlendFactor(dst_alpha_blend_factor as _);

        self
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StencilOp {
    Keep = 1,
    Zero = 2,
    Replace = 3,
    Incr = 4,
    Decr = 5,
    Invert = 6,
    IncrWrap = 7,
    DecrWrap = 8,
}

#[repr(C)]
pub struct DepthStencilState(DkDepthStencilState);

impl DepthStencilState {
    pub fn new() -> Self {
        let state = DkDepthStencilState::new_bitfield_1(
            1,
            1,
            0,
            DkCompareOp_DkCompareOp_Less,
            DkStencilOp_DkStencilOp_Keep,
            DkStencilOp_DkStencilOp_Replace,
            DkStencilOp_DkStencilOp_Keep,
            DkCompareOp_DkCompareOp_Always,
            DkStencilOp_DkStencilOp_Keep,
            DkStencilOp_DkStencilOp_Replace,
            DkStencilOp_DkStencilOp_Keep,
            DkCompareOp_DkCompareOp_Always,
        );

        DepthStencilState(DkDepthStencilState { _bitfield_align_1: [], _bitfield_1: state })
    }

    pub fn set_depth_test_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_depthTestEnable(enable as _);

        self
    }

    pub fn set_depth_write_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_depthWriteEnable(enable as _);

        self
    }

    pub fn set_stencil_test_enable(&mut self, enable: bool) -> &mut Self {
        self.0.set_stencilTestEnable(enable as _);

        self
    }

    pub fn set_depth_compare_op(&mut self, op: CompareOp) -> &mut Self {
        self.0.set_depthCompareOp(op as _);

        self
    }

    pub fn set_stencil_front_fail_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilFrontFailOp(op as _);

        self
    }

    pub fn set_stencil_front_pass_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilFrontPassOp(op as _);

        self
    }

    pub fn set_stencil_front_depth_fail_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilFrontDepthFailOp(op as _);

        self
    }

    pub fn set_stencil_front_compare_op(&mut self, op: CompareOp) -> &mut Self {
        self.0.set_stencilFrontCompareOp(op as _);

        self
    }

    pub fn set_stencil_back_fail_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilBackFailOp(op as _);

        self
    }

    pub fn set_stencil_back_pass_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilBackPassOp(op as _);

        self
    }

    pub fn set_stencil_back_depth_fail_op(&mut self, op: StencilOp) -> &mut Self {
        self.0.set_stencilBackDepthFailOp(op as _);

        self
    }

    pub fn set_stencil_back_compare_op(&mut self, op: CompareOp) -> &mut Self {
        self.0.set_stencilBackCompareOp(op as _);

        self
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TiledCacheOp {
    Disable = 0,
    Enable = 1,
    Flush = 2,
    FlushAlt = 3,
    UnkDisable = 4,
    UnkEnable = 5,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VtxAttribSize {
    _1x32 = DkVtxAttribSize_DkVtxAttribSize_1x32,
    _2x32 = DkVtxAttribSize_DkVtxAttribSize_2x32,
    _3x32 = DkVtxAttribSize_DkVtxAttribSize_3x32,
    _4x32 = DkVtxAttribSize_DkVtxAttribSize_4x32,
    _1x16 = DkVtxAttribSize_DkVtxAttribSize_1x16,
    _2x16 = DkVtxAttribSize_DkVtxAttribSize_2x16,
    _3x16 = DkVtxAttribSize_DkVtxAttribSize_3x16,
    _4x16 = DkVtxAttribSize_DkVtxAttribSize_4x16,
    _1x8 = DkVtxAttribSize_DkVtxAttribSize_1x8,
    _2x8 = DkVtxAttribSize_DkVtxAttribSize_2x8,
    _3x8 = DkVtxAttribSize_DkVtxAttribSize_3x8,
    _4x8 = DkVtxAttribSize_DkVtxAttribSize_4x8,
    _10_10_10_2 = DkVtxAttribSize_DkVtxAttribSize_10_10_10_2,
    _11_11_10 = DkVtxAttribSize_DkVtxAttribSize_11_11_10,
}

impl From<DkVtxAttribSize> for VtxAttribSize {
    fn from(size: DkVtxAttribSize) -> Self {
        match size {
            DkVtxAttribSize_DkVtxAttribSize_1x32 => VtxAttribSize::_1x32,
            DkVtxAttribSize_DkVtxAttribSize_2x32 => VtxAttribSize::_2x32,
            DkVtxAttribSize_DkVtxAttribSize_3x32 => VtxAttribSize::_3x32,
            DkVtxAttribSize_DkVtxAttribSize_4x32 => VtxAttribSize::_4x32,
            DkVtxAttribSize_DkVtxAttribSize_1x16 => VtxAttribSize::_1x16,
            DkVtxAttribSize_DkVtxAttribSize_2x16 => VtxAttribSize::_2x16,
            DkVtxAttribSize_DkVtxAttribSize_3x16 => VtxAttribSize::_3x16,
            DkVtxAttribSize_DkVtxAttribSize_4x16 => VtxAttribSize::_4x16,
            DkVtxAttribSize_DkVtxAttribSize_1x8 => VtxAttribSize::_1x8,
            DkVtxAttribSize_DkVtxAttribSize_2x8 => VtxAttribSize::_2x8,
            DkVtxAttribSize_DkVtxAttribSize_3x8 => VtxAttribSize::_3x8,
            DkVtxAttribSize_DkVtxAttribSize_4x8 => VtxAttribSize::_4x8,
            DkVtxAttribSize_DkVtxAttribSize_10_10_10_2 => VtxAttribSize::_10_10_10_2,
            DkVtxAttribSize_DkVtxAttribSize_11_11_10 => VtxAttribSize::_11_11_10,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VtxAttribType {
    None = DkVtxAttribType_DkVtxAttribType_None,
    Snorm = DkVtxAttribType_DkVtxAttribType_Snorm,
    Unorm = DkVtxAttribType_DkVtxAttribType_Unorm,
    Sint = DkVtxAttribType_DkVtxAttribType_Sint,
    Uint = DkVtxAttribType_DkVtxAttribType_Uint,
    Uscaled = DkVtxAttribType_DkVtxAttribType_Uscaled,
    Sscaled = DkVtxAttribType_DkVtxAttribType_Sscaled,
    Float = DkVtxAttribType_DkVtxAttribType_Float,
}

impl From<DkVtxAttribType> for VtxAttribType {
    fn from(v: DkVtxAttribType) -> Self {
        match v {
            DkVtxAttribType_DkVtxAttribType_None => VtxAttribType::None,
            DkVtxAttribType_DkVtxAttribType_Snorm => VtxAttribType::Snorm,
            DkVtxAttribType_DkVtxAttribType_Unorm => VtxAttribType::Unorm,
            DkVtxAttribType_DkVtxAttribType_Sint => VtxAttribType::Sint,
            DkVtxAttribType_DkVtxAttribType_Uint => VtxAttribType::Uint,
            DkVtxAttribType_DkVtxAttribType_Uscaled => VtxAttribType::Uscaled,
            DkVtxAttribType_DkVtxAttribType_Sscaled => VtxAttribType::Sscaled,
            DkVtxAttribType_DkVtxAttribType_Float => VtxAttribType::Float,
            _ => panic!("Invalid DkVtxAttribType value"),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VtxAttribState(DkVtxAttribState);

impl VtxAttribState {
    pub fn new() -> VtxAttribState {
        unsafe {
            let mut state = std::mem::zeroed();

            VtxAttribState(state)
        }
    }

    pub fn set_buffer_id(&mut self, buffer_id: u8) -> &mut Self {
        self.0.set_bufferId(buffer_id as u32);

        self
    }

    pub fn get_buffer_id(&self) -> u8 {
        self.0.bufferId() as u8
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) -> &mut Self {
        self.0.set_isFixed(is_fixed as u32);

        self
    }

    pub fn get_is_fixed(&self) -> bool {
        self.0.isFixed() != 0
    }

    pub fn set_offset(&mut self, offset: u16) -> &mut Self {
        self.0.set_offset(offset as u32);

        self
    }

    pub fn get_offset(&self) -> u16 {
        self.0.offset() as u16
    }

    pub fn set_size(&mut self, size: VtxAttribSize) -> &mut Self {
        self.0.set_size(size as u32);

        self
    }

    pub fn get_size(&self) -> VtxAttribSize {
        VtxAttribSize::from(self.0.size())
    }

    pub fn set_type(&mut self, type_: VtxAttribType) -> &mut Self {
        self.0.set_type(type_ as u32);

        self
    }

    pub fn get_type(&self) -> VtxAttribType {
        VtxAttribType::from(self.0.type_())
    }

    pub fn set_is_bgra(&mut self, is_bgra: bool) -> &mut Self {
        self.0.set_isBgra(is_bgra as u32);

        self
    }

    pub fn get_is_bgra(&self) -> bool {
        self.0.isBgra() != 0
    }
}

pub type VtxBufferState = DkVtxBufferState;

// impl VtxBufferState {
//     pub fn new() -> Self {
//         VtxBufferState { stride: 0, divisor: 0 }
//     }
// }

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    Points = DkPrimitive_DkPrimitive_Points,
    Lines = DkPrimitive_DkPrimitive_Lines,
    LineLoop = DkPrimitive_DkPrimitive_LineLoop,
    LineStrip = DkPrimitive_DkPrimitive_LineStrip,
    Triangles = DkPrimitive_DkPrimitive_Triangles,
    TriangleStrip = DkPrimitive_DkPrimitive_TriangleStrip,
    TriangleFan = DkPrimitive_DkPrimitive_TriangleFan,
    Quads = DkPrimitive_DkPrimitive_Quads,
    QuadStrip = DkPrimitive_DkPrimitive_QuadStrip,
    Polygon = DkPrimitive_DkPrimitive_Polygon,
    LinesAdjacency = DkPrimitive_DkPrimitive_LinesAdjacency,
    LineStripAdjacency = DkPrimitive_DkPrimitive_LineStripAdjacency,
    TrianglesAdjacency = DkPrimitive_DkPrimitive_TrianglesAdjacency,
    TriangleStripAdjacency = DkPrimitive_DkPrimitive_TriangleStripAdjacency,
    Patches = DkPrimitive_DkPrimitive_Patches,
}

impl From<DkPrimitive> for Primitive {
    fn from(v: DkPrimitive) -> Self {
        match v {
            DkPrimitive_DkPrimitive_Points => Primitive::Points,
            DkPrimitive_DkPrimitive_Lines => Primitive::Lines,
            DkPrimitive_DkPrimitive_LineLoop => Primitive::LineLoop,
            DkPrimitive_DkPrimitive_LineStrip => Primitive::LineStrip,
            DkPrimitive_DkPrimitive_Triangles => Primitive::Triangles,
            DkPrimitive_DkPrimitive_TriangleStrip => Primitive::TriangleStrip,
            DkPrimitive_DkPrimitive_TriangleFan => Primitive::TriangleFan,
            DkPrimitive_DkPrimitive_Quads => Primitive::Quads,
            DkPrimitive_DkPrimitive_QuadStrip => Primitive::QuadStrip,
            DkPrimitive_DkPrimitive_Polygon => Primitive::Polygon,
            DkPrimitive_DkPrimitive_LinesAdjacency => Primitive::LinesAdjacency,
            DkPrimitive_DkPrimitive_LineStripAdjacency => Primitive::LineStripAdjacency,
            DkPrimitive_DkPrimitive_TrianglesAdjacency => Primitive::TrianglesAdjacency,
            DkPrimitive_DkPrimitive_TriangleStripAdjacency => Primitive::TriangleStripAdjacency,
            DkPrimitive_DkPrimitive_Patches => Primitive::Patches,
            _ => panic!("Invalid DkPrimitive value"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IdxFormat {
    Uint8 = DkIdxFormat_DkIdxFormat_Uint8,
    Uint16 = DkIdxFormat_DkIdxFormat_Uint16,
    Uint32 = DkIdxFormat_DkIdxFormat_Uint32,
}

impl From<DkIdxFormat> for IdxFormat {
    fn from(v: DkIdxFormat) -> Self {
        match v {
            DkIdxFormat_DkIdxFormat_Uint8 => IdxFormat::Uint8,
            DkIdxFormat_DkIdxFormat_Uint16 => IdxFormat::Uint16,
            DkIdxFormat_DkIdxFormat_Uint32 => IdxFormat::Uint32,
            _ => panic!("Invalid DkIdxFormat value"),
        }
    }
}

pub type ImageRect = DkImageRect;
pub type CopyBuf = DkCopyBuf;

#[repr(C)]
pub struct SwapchainMaker(DkSwapchainMaker);

impl SwapchainMaker {
    pub fn new(device: &Device, native_window: *mut std::ffi::c_void, images: &[Image]) -> Self {
        unsafe {
            let mut maker: DkSwapchainMaker = std::mem::zeroed();

            maker.device = device.0;
            maker.nativeWindow = native_window;
            maker.pImages = images.as_ptr() as *const _;
            maker.numImages = images.len() as u32;

            SwapchainMaker(maker)
        }
    }

    pub fn create(&self) -> Swapchain {
        unsafe { Swapchain(dkSwapchainCreate(&self.0)) }
    }
}
