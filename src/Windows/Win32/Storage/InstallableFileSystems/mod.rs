#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_BASIC_INFORMATION_0,
}
impl FILTER_AGGREGATE_BASIC_INFORMATION {}
impl ::std::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_0,
}
impl FILTER_AGGREGATE_BASIC_INFORMATION_0 {}
impl ::std::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {}
impl ::std::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_LegacyFilter_e__Struct")
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {}
impl ::std::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MiniFilter_e__Struct")
            .field("FrameID", &self.FrameID)
            .field("NumberOfInstances", &self.NumberOfInstances)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("FilterAltitudeLength", &self.FilterAltitudeLength)
            .field(
                "FilterAltitudeBufferOffset",
                &self.FilterAltitudeBufferOffset,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.FrameID == other.FrameID
            && self.NumberOfInstances == other.NumberOfInstances
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
            && self.FilterAltitudeLength == other.FilterAltitudeLength
            && self.FilterAltitudeBufferOffset == other.FilterAltitudeBufferOffset
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_STANDARD_INFORMATION_0,
}
impl FILTER_AGGREGATE_STANDARD_INFORMATION {}
impl ::std::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl FILTER_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::std::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::std::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_LegacyFilter_e__Struct")
            .field("Flags", &self.Flags)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("FilterAltitudeLength", &self.FilterAltitudeLength)
            .field(
                "FilterAltitudeBufferOffset",
                &self.FilterAltitudeBufferOffset,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
            && self.FilterAltitudeLength == other.FilterAltitudeLength
            && self.FilterAltitudeBufferOffset == other.FilterAltitudeBufferOffset
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::std::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MiniFilter_e__Struct")
            .field("Flags", &self.Flags)
            .field("FrameID", &self.FrameID)
            .field("NumberOfInstances", &self.NumberOfInstances)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("FilterAltitudeLength", &self.FilterAltitudeLength)
            .field(
                "FilterAltitudeBufferOffset",
                &self.FilterAltitudeBufferOffset,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.FrameID == other.FrameID
            && self.NumberOfInstances == other.NumberOfInstances
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
            && self.FilterAltitudeLength == other.FilterAltitudeLength
            && self.FilterAltitudeBufferOffset == other.FilterAltitudeBufferOffset
    }
}
impl ::std::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {}
unsafe impl ::windows::runtime::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBuffer: [u16; 1],
}
impl FILTER_FULL_INFORMATION {}
impl ::std::default::Default for FILTER_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_FULL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTER_FULL_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FrameID", &self.FrameID)
            .field("NumberOfInstances", &self.NumberOfInstances)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBuffer", &self.FilterNameBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FrameID == other.FrameID
            && self.NumberOfInstances == other.NumberOfInstances
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBuffer == other.FilterNameBuffer
    }
}
impl ::std::cmp::Eq for FILTER_FULL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILTER_FULL_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FILTER_INFORMATION_CLASS(pub i32);
pub const FilterFullInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(0i32);
pub const FilterAggregateBasicInformation: FILTER_INFORMATION_CLASS =
    FILTER_INFORMATION_CLASS(1i32);
pub const FilterAggregateStandardInformation: FILTER_INFORMATION_CLASS =
    FILTER_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for FILTER_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILTER_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_MESSAGE_HEADER {
    pub ReplyLength: u32,
    pub MessageId: u64,
}
impl FILTER_MESSAGE_HEADER {}
impl ::std::default::Default for FILTER_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_MESSAGE_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTER_MESSAGE_HEADER")
            .field("ReplyLength", &self.ReplyLength)
            .field("MessageId", &self.MessageId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_MESSAGE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ReplyLength == other.ReplyLength && self.MessageId == other.MessageId
    }
}
impl ::std::cmp::Eq for FILTER_MESSAGE_HEADER {}
unsafe impl ::windows::runtime::Abi for FILTER_MESSAGE_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FILTER_NAME_MAX_CHARS: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILTER_REPLY_HEADER {
    pub Status: super::super::Foundation::NTSTATUS,
    pub MessageId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl FILTER_REPLY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILTER_REPLY_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FILTER_REPLY_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTER_REPLY_HEADER")
            .field("Status", &self.Status)
            .field("MessageId", &self.MessageId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILTER_REPLY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.MessageId == other.MessageId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILTER_REPLY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILTER_REPLY_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_VOLUME_BASIC_INFORMATION {
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl FILTER_VOLUME_BASIC_INFORMATION {}
impl ::std::default::Default for FILTER_VOLUME_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_VOLUME_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTER_VOLUME_BASIC_INFORMATION")
            .field("FilterVolumeNameLength", &self.FilterVolumeNameLength)
            .field("FilterVolumeName", &self.FilterVolumeName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_VOLUME_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FilterVolumeNameLength == other.FilterVolumeNameLength
            && self.FilterVolumeName == other.FilterVolumeName
    }
}
impl ::std::cmp::Eq for FILTER_VOLUME_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILTER_VOLUME_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FILTER_VOLUME_INFORMATION_CLASS(pub i32);
pub const FilterVolumeBasicInformation: FILTER_VOLUME_INFORMATION_CLASS =
    FILTER_VOLUME_INFORMATION_CLASS(0i32);
pub const FilterVolumeStandardInformation: FILTER_VOLUME_INFORMATION_CLASS =
    FILTER_VOLUME_INFORMATION_CLASS(1i32);
impl ::std::convert::From<i32> for FILTER_VOLUME_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILTER_VOLUME_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILTER_VOLUME_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub FrameID: u32,
    pub FileSystemType: FLT_FILESYSTEM_TYPE,
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl FILTER_VOLUME_STANDARD_INFORMATION {}
impl ::std::default::Default for FILTER_VOLUME_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTER_VOLUME_STANDARD_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTER_VOLUME_STANDARD_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Flags", &self.Flags)
            .field("FrameID", &self.FrameID)
            .field("FileSystemType", &self.FileSystemType)
            .field("FilterVolumeNameLength", &self.FilterVolumeNameLength)
            .field("FilterVolumeName", &self.FilterVolumeName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILTER_VOLUME_STANDARD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.Flags == other.Flags
            && self.FrameID == other.FrameID
            && self.FileSystemType == other.FileSystemType
            && self.FilterVolumeNameLength == other.FilterVolumeNameLength
            && self.FilterVolumeName == other.FilterVolumeName
    }
}
impl ::std::cmp::Eq for FILTER_VOLUME_STANDARD_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILTER_VOLUME_STANDARD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_ASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_ASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_IASIL_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASIM_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_IASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_VSI_DETACHED_VOLUME: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FLT_FILESYSTEM_TYPE(pub i32);
pub const FLT_FSTYPE_UNKNOWN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(0i32);
pub const FLT_FSTYPE_RAW: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(1i32);
pub const FLT_FSTYPE_NTFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(2i32);
pub const FLT_FSTYPE_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(3i32);
pub const FLT_FSTYPE_CDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(4i32);
pub const FLT_FSTYPE_UDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(5i32);
pub const FLT_FSTYPE_LANMAN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(6i32);
pub const FLT_FSTYPE_WEBDAV: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(7i32);
pub const FLT_FSTYPE_RDPDR: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(8i32);
pub const FLT_FSTYPE_NFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(9i32);
pub const FLT_FSTYPE_MS_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(10i32);
pub const FLT_FSTYPE_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(11i32);
pub const FLT_FSTYPE_BSUDF: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(12i32);
pub const FLT_FSTYPE_MUP: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(13i32);
pub const FLT_FSTYPE_RSFX: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(14i32);
pub const FLT_FSTYPE_ROXIO_UDF1: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(15i32);
pub const FLT_FSTYPE_ROXIO_UDF2: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(16i32);
pub const FLT_FSTYPE_ROXIO_UDF3: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(17i32);
pub const FLT_FSTYPE_TACIT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(18i32);
pub const FLT_FSTYPE_FS_REC: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(19i32);
pub const FLT_FSTYPE_INCD: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(20i32);
pub const FLT_FSTYPE_INCD_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(21i32);
pub const FLT_FSTYPE_EXFAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(22i32);
pub const FLT_FSTYPE_PSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(23i32);
pub const FLT_FSTYPE_GPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(24i32);
pub const FLT_FSTYPE_NPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(25i32);
pub const FLT_FSTYPE_MSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(26i32);
pub const FLT_FSTYPE_CSVFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(27i32);
pub const FLT_FSTYPE_REFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(28i32);
pub const FLT_FSTYPE_OPENAFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(29i32);
pub const FLT_FSTYPE_CIMFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(30i32);
impl ::std::convert::From<i32> for FLT_FILESYSTEM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FLT_FILESYSTEM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterAttach<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
    lpvolumename: Param1,
    lpinstancename: Param2,
    dwcreatedinstancenamelength: u32,
    lpcreatedinstancename: super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterAttach(
                lpfiltername: super::super::Foundation::PWSTR,
                lpvolumename: super::super::Foundation::PWSTR,
                lpinstancename: super::super::Foundation::PWSTR,
                dwcreatedinstancenamelength: u32,
                lpcreatedinstancename: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterAttach(
            lpfiltername.into_param().abi(),
            lpvolumename.into_param().abi(),
            lpinstancename.into_param().abi(),
            ::std::mem::transmute(dwcreatedinstancenamelength),
            ::std::mem::transmute(lpcreatedinstancename),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterAttachAtAltitude<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
    lpvolumename: Param1,
    lpaltitude: Param2,
    lpinstancename: Param3,
    dwcreatedinstancenamelength: u32,
    lpcreatedinstancename: super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterAttachAtAltitude(
                lpfiltername: super::super::Foundation::PWSTR,
                lpvolumename: super::super::Foundation::PWSTR,
                lpaltitude: super::super::Foundation::PWSTR,
                lpinstancename: super::super::Foundation::PWSTR,
                dwcreatedinstancenamelength: u32,
                lpcreatedinstancename: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterAttachAtAltitude(
            lpfiltername.into_param().abi(),
            lpvolumename.into_param().abi(),
            lpaltitude.into_param().abi(),
            lpinstancename.into_param().abi(),
            ::std::mem::transmute(dwcreatedinstancenamelength),
            ::std::mem::transmute(lpcreatedinstancename),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterClose<'a, Param0: ::windows::runtime::IntoParam<'a, HFILTER>>(
    hfilter: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterClose(hfilter: HFILTER) -> ::windows::runtime::HRESULT;
        }
        FilterClose(hfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn FilterConnectCommunicationPort<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpportname: Param0,
    dwoptions: u32,
    lpcontext: *const ::std::ffi::c_void,
    wsizeofcontext: u16,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterConnectCommunicationPort(
                lpportname: super::super::Foundation::PWSTR,
                dwoptions: u32,
                lpcontext: *const ::std::ffi::c_void,
                wsizeofcontext: u16,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                hport: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        FilterConnectCommunicationPort(
            lpportname.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(lpcontext),
            ::std::mem::transmute(wsizeofcontext),
            ::std::mem::transmute(lpsecurityattributes),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterCreate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
) -> ::windows::runtime::Result<HFILTER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterCreate(
                lpfiltername: super::super::Foundation::PWSTR,
                hfilter: *mut HFILTER,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HFILTER as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        FilterCreate(lpfiltername.into_param().abi(), &mut result__).from_abi::<HFILTER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterDetach<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
    lpvolumename: Param1,
    lpinstancename: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterDetach(
                lpfiltername: super::super::Foundation::PWSTR,
                lpvolumename: super::super::Foundation::PWSTR,
                lpinstancename: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterDetach(
            lpfiltername.into_param().abi(),
            lpvolumename.into_param().abi(),
            lpinstancename.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterFindClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilterfind: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindClose(
                hfilterfind: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterFindClose(hfilterfind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterFindFirst(
    dwinformationclass: FILTER_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpfilterfind: *mut FilterFindHandle,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindFirst(
                dwinformationclass: FILTER_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
                lpfilterfind: *mut FilterFindHandle,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterFindFirst(
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpfilterfind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FilterFindHandle(pub isize);
impl ::std::default::Default for FilterFindHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FilterFindHandle {}
unsafe impl ::windows::runtime::Abi for FilterFindHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterFindNext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilterfind: Param0,
    dwinformationclass: FILTER_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindNext(
                hfilterfind: super::super::Foundation::HANDLE,
                dwinformationclass: FILTER_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterFindNext(
            hfilterfind.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterGetDosName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpvolumename: Param0,
    lpdosname: super::super::Foundation::PWSTR,
    dwdosnamebuffersize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetDosName(
                lpvolumename: super::super::Foundation::PWSTR,
                lpdosname: super::super::Foundation::PWSTR,
                dwdosnamebuffersize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterGetDosName(
            lpvolumename.into_param().abi(),
            ::std::mem::transmute(lpdosname),
            ::std::mem::transmute(dwdosnamebuffersize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterGetInformation<'a, Param0: ::windows::runtime::IntoParam<'a, HFILTER>>(
    hfilter: Param0,
    dwinformationclass: FILTER_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetInformation(
                hfilter: HFILTER,
                dwinformationclass: FILTER_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterGetInformation(
            hfilter.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn FilterGetMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hport: Param0,
    lpmessagebuffer: *mut FILTER_MESSAGE_HEADER,
    dwmessagebuffersize: u32,
    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetMessage(
                hport: super::super::Foundation::HANDLE,
                lpmessagebuffer: *mut FILTER_MESSAGE_HEADER,
                dwmessagebuffersize: u32,
                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterGetMessage(
            hport.into_param().abi(),
            ::std::mem::transmute(lpmessagebuffer),
            ::std::mem::transmute(dwmessagebuffersize),
            ::std::mem::transmute(lpoverlapped),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterInstanceClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HFILTER_INSTANCE>,
>(
    hinstance: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceClose(hinstance: HFILTER_INSTANCE) -> ::windows::runtime::HRESULT;
        }
        FilterInstanceClose(hinstance.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterInstanceCreate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
    lpvolumename: Param1,
    lpinstancename: Param2,
) -> ::windows::runtime::Result<HFILTER_INSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceCreate(
                lpfiltername: super::super::Foundation::PWSTR,
                lpvolumename: super::super::Foundation::PWSTR,
                lpinstancename: super::super::Foundation::PWSTR,
                hinstance: *mut HFILTER_INSTANCE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HFILTER_INSTANCE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        FilterInstanceCreate(
            lpfiltername.into_param().abi(),
            lpvolumename.into_param().abi(),
            lpinstancename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<HFILTER_INSTANCE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterInstanceFindClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilterinstancefind: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindClose(
                hfilterinstancefind: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterInstanceFindClose(hfilterinstancefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterInstanceFindFirst<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
    dwinformationclass: INSTANCE_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpfilterinstancefind: *mut FilterInstanceFindHandle,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindFirst(
                lpfiltername: super::super::Foundation::PWSTR,
                dwinformationclass: INSTANCE_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
                lpfilterinstancefind: *mut FilterInstanceFindHandle,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterInstanceFindFirst(
            lpfiltername.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpfilterinstancefind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FilterInstanceFindHandle(pub isize);
impl ::std::default::Default for FilterInstanceFindHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FilterInstanceFindHandle {}
unsafe impl ::windows::runtime::Abi for FilterInstanceFindHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterInstanceFindNext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilterinstancefind: Param0,
    dwinformationclass: INSTANCE_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindNext(
                hfilterinstancefind: super::super::Foundation::HANDLE,
                dwinformationclass: INSTANCE_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterInstanceFindNext(
            hfilterinstancefind.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterInstanceGetInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HFILTER_INSTANCE>,
>(
    hinstance: Param0,
    dwinformationclass: INSTANCE_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceGetInformation(
                hinstance: HFILTER_INSTANCE,
                dwinformationclass: INSTANCE_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterInstanceGetInformation(
            hinstance.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterLoad<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterLoad(
                lpfiltername: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterLoad(lpfiltername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterReplyMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hport: Param0,
    lpreplybuffer: *const FILTER_REPLY_HEADER,
    dwreplybuffersize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterReplyMessage(
                hport: super::super::Foundation::HANDLE,
                lpreplybuffer: *const FILTER_REPLY_HEADER,
                dwreplybuffersize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterReplyMessage(
            hport.into_param().abi(),
            ::std::mem::transmute(lpreplybuffer),
            ::std::mem::transmute(dwreplybuffersize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterSendMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hport: Param0,
    lpinbuffer: *const ::std::ffi::c_void,
    dwinbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    dwoutbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterSendMessage(
                hport: super::super::Foundation::HANDLE,
                lpinbuffer: *const ::std::ffi::c_void,
                dwinbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                dwoutbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterSendMessage(
            hport.into_param().abi(),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(dwinbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(dwoutbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterUnload<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfiltername: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterUnload(
                lpfiltername: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterUnload(lpfiltername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterVolumeFindClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hvolumefind: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindClose(
                hvolumefind: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeFindClose(hvolumefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FilterVolumeFindFirst(
    dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpvolumefind: *mut FilterVolumeFindHandle,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindFirst(
                dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
                lpvolumefind: *mut FilterVolumeFindHandle,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeFindFirst(
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpvolumefind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FilterVolumeFindHandle(pub isize);
impl ::std::default::Default for FilterVolumeFindHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FilterVolumeFindHandle {}
unsafe impl ::windows::runtime::Abi for FilterVolumeFindHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterVolumeFindNext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hvolumefind: Param0,
    dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindNext(
                hvolumefind: super::super::Foundation::HANDLE,
                dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeFindNext(
            hvolumefind.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterVolumeInstanceFindClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hvolumeinstancefind: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindClose(
                hvolumeinstancefind: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeInstanceFindClose(hvolumeinstancefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterVolumeInstanceFindFirst<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpvolumename: Param0,
    dwinformationclass: INSTANCE_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindFirst(
                lpvolumename: super::super::Foundation::PWSTR,
                dwinformationclass: INSTANCE_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
                lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeInstanceFindFirst(
            lpvolumename.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpvolumeinstancefind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FilterVolumeInstanceFindHandle(pub isize);
impl ::std::default::Default for FilterVolumeInstanceFindHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FilterVolumeInstanceFindHandle {}
unsafe impl ::windows::runtime::Abi for FilterVolumeInstanceFindHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FilterVolumeInstanceFindNext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hvolumeinstancefind: Param0,
    dwinformationclass: INSTANCE_INFORMATION_CLASS,
    lpbuffer: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindNext(
                hvolumeinstancefind: super::super::Foundation::HANDLE,
                dwinformationclass: INSTANCE_INFORMATION_CLASS,
                lpbuffer: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        FilterVolumeInstanceFindNext(
            hvolumeinstancefind.into_param().abi(),
            ::std::mem::transmute(dwinformationclass),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HFILTER(pub isize);
impl ::std::default::Default for HFILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HFILTER {}
unsafe impl ::windows::runtime::Abi for HFILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HFILTER_INSTANCE(pub isize);
impl ::std::default::Default for HFILTER_INSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HFILTER_INSTANCE {}
unsafe impl ::windows::runtime::Abi for HFILTER_INSTANCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0,
}
impl INSTANCE_AGGREGATE_STANDARD_INFORMATION {}
impl ::std::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::std::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::std::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_LegacyFilter_e__Struct")
            .field("Flags", &self.Flags)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("SupportedFeatures", &self.SupportedFeatures)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.AltitudeLength == other.AltitudeLength
            && self.AltitudeBufferOffset == other.AltitudeBufferOffset
            && self.VolumeNameLength == other.VolumeNameLength
            && self.VolumeNameBufferOffset == other.VolumeNameBufferOffset
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
            && self.SupportedFeatures == other.SupportedFeatures
    }
}
impl ::std::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::std::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MiniFilter_e__Struct")
            .field("Flags", &self.Flags)
            .field("FrameID", &self.FrameID)
            .field("VolumeFileSystemType", &self.VolumeFileSystemType)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("SupportedFeatures", &self.SupportedFeatures)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.FrameID == other.FrameID
            && self.VolumeFileSystemType == other.VolumeFileSystemType
            && self.InstanceNameLength == other.InstanceNameLength
            && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset
            && self.AltitudeLength == other.AltitudeLength
            && self.AltitudeBufferOffset == other.AltitudeBufferOffset
            && self.VolumeNameLength == other.VolumeNameLength
            && self.VolumeNameBufferOffset == other.VolumeNameBufferOffset
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
            && self.SupportedFeatures == other.SupportedFeatures
    }
}
impl ::std::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {}
unsafe impl ::windows::runtime::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
}
impl INSTANCE_BASIC_INFORMATION {}
impl ::std::default::Default for INSTANCE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTANCE_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INSTANCE_BASIC_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INSTANCE_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.InstanceNameLength == other.InstanceNameLength
            && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset
    }
}
impl ::std::cmp::Eq for INSTANCE_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INSTANCE_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl INSTANCE_FULL_INFORMATION {}
impl ::std::default::Default for INSTANCE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTANCE_FULL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INSTANCE_FULL_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INSTANCE_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.InstanceNameLength == other.InstanceNameLength
            && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset
            && self.AltitudeLength == other.AltitudeLength
            && self.AltitudeBufferOffset == other.AltitudeBufferOffset
            && self.VolumeNameLength == other.VolumeNameLength
            && self.VolumeNameBufferOffset == other.VolumeNameBufferOffset
            && self.FilterNameLength == other.FilterNameLength
            && self.FilterNameBufferOffset == other.FilterNameBufferOffset
    }
}
impl ::std::cmp::Eq for INSTANCE_FULL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INSTANCE_FULL_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct INSTANCE_INFORMATION_CLASS(pub i32);
pub const InstanceBasicInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(0i32);
pub const InstancePartialInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(1i32);
pub const InstanceFullInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(2i32);
pub const InstanceAggregateStandardInformation: INSTANCE_INFORMATION_CLASS =
    INSTANCE_INFORMATION_CLASS(3i32);
impl ::std::convert::From<i32> for INSTANCE_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTANCE_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const INSTANCE_NAME_MAX_CHARS: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INSTANCE_PARTIAL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
}
impl INSTANCE_PARTIAL_INFORMATION {}
impl ::std::default::Default for INSTANCE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTANCE_PARTIAL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INSTANCE_PARTIAL_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INSTANCE_PARTIAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.InstanceNameLength == other.InstanceNameLength
            && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset
            && self.AltitudeLength == other.AltitudeLength
            && self.AltitudeBufferOffset == other.AltitudeBufferOffset
    }
}
impl ::std::cmp::Eq for INSTANCE_PARTIAL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INSTANCE_PARTIAL_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VOLUME_NAME_MAX_CHARS: u32 = 1024u32;
pub const WNNC_CRED_MANAGER: u32 = 4294901760u32;
pub const WNNC_NET_10NET: u32 = 327680u32;
pub const WNNC_NET_3IN1: u32 = 2555904u32;
pub const WNNC_NET_9P: u32 = 4718592u32;
pub const WNNC_NET_9TILES: u32 = 589824u32;
pub const WNNC_NET_APPLETALK: u32 = 1245184u32;
pub const WNNC_NET_AS400: u32 = 720896u32;
pub const WNNC_NET_AURISTOR_FS: u32 = 4587520u32;
pub const WNNC_NET_AVID: u32 = 1703936u32;
pub const WNNC_NET_AVID1: u32 = 3801088u32;
pub const WNNC_NET_BMC: u32 = 1572864u32;
pub const WNNC_NET_BWNFS: u32 = 1048576u32;
pub const WNNC_NET_CLEARCASE: u32 = 1441792u32;
pub const WNNC_NET_COGENT: u32 = 1114112u32;
pub const WNNC_NET_CSC: u32 = 2490368u32;
pub const WNNC_NET_DAV: u32 = 3014656u32;
pub const WNNC_NET_DCE: u32 = 1638400u32;
pub const WNNC_NET_DECORB: u32 = 2097152u32;
pub const WNNC_NET_DFS: u32 = 3866624u32;
pub const WNNC_NET_DISTINCT: u32 = 2293760u32;
pub const WNNC_NET_DOCUSHARE: u32 = 4521984u32;
pub const WNNC_NET_DOCUSPACE: u32 = 1769472u32;
pub const WNNC_NET_DRIVEONWEB: u32 = 4063232u32;
pub const WNNC_NET_EXIFS: u32 = 2949120u32;
pub const WNNC_NET_EXTENDNET: u32 = 2686976u32;
pub const WNNC_NET_FARALLON: u32 = 1179648u32;
pub const WNNC_NET_FJ_REDIR: u32 = 2228224u32;
pub const WNNC_NET_FOXBAT: u32 = 2818048u32;
pub const WNNC_NET_FRONTIER: u32 = 1507328u32;
pub const WNNC_NET_FTP_NFS: u32 = 786432u32;
pub const WNNC_NET_GOOGLE: u32 = 4390912u32;
pub const WNNC_NET_HOB_NFS: u32 = 3276800u32;
pub const WNNC_NET_IBMAL: u32 = 3407872u32;
pub const WNNC_NET_INTERGRAPH: u32 = 1310720u32;
pub const WNNC_NET_KNOWARE: u32 = 3080192u32;
pub const WNNC_NET_KWNP: u32 = 3932160u32;
pub const WNNC_NET_LANMAN: u32 = 131072u32;
pub const WNNC_NET_LANSTEP: u32 = 524288u32;
pub const WNNC_NET_LANTASTIC: u32 = 655360u32;
pub const WNNC_NET_LIFENET: u32 = 917504u32;
pub const WNNC_NET_LOCK: u32 = 3473408u32;
pub const WNNC_NET_LOCUS: u32 = 393216u32;
pub const WNNC_NET_MANGOSOFT: u32 = 1835008u32;
pub const WNNC_NET_MASFAX: u32 = 3211264u32;
pub const WNNC_NET_MFILES: u32 = 4259840u32;
pub const WNNC_NET_MSNET: u32 = 65536u32;
pub const WNNC_NET_MS_NFS: u32 = 4325376u32;
pub const WNNC_NET_NDFS: u32 = 4456448u32;
pub const WNNC_NET_NETWARE: u32 = 196608u32;
pub const WNNC_NET_OBJECT_DIRE: u32 = 3145728u32;
pub const WNNC_NET_OPENAFS: u32 = 3735552u32;
pub const WNNC_NET_PATHWORKS: u32 = 851968u32;
pub const WNNC_NET_POWERLAN: u32 = 983040u32;
pub const WNNC_NET_PROTSTOR: u32 = 2162688u32;
pub const WNNC_NET_QUINCY: u32 = 3670016u32;
pub const WNNC_NET_RDR2SAMPLE: u32 = 2424832u32;
pub const WNNC_NET_RIVERFRONT1: u32 = 1966080u32;
pub const WNNC_NET_RIVERFRONT2: u32 = 2031616u32;
pub const WNNC_NET_RSFX: u32 = 4194304u32;
pub const WNNC_NET_SECUREAGENT: u32 = 4653056u32;
pub const WNNC_NET_SERNET: u32 = 1900544u32;
pub const WNNC_NET_SHIVA: u32 = 3342336u32;
pub const WNNC_NET_SMB: u32 = 131072u32;
pub const WNNC_NET_SRT: u32 = 3604480u32;
pub const WNNC_NET_STAC: u32 = 2752512u32;
pub const WNNC_NET_SUN_PC_NFS: u32 = 458752u32;
pub const WNNC_NET_SYMFONET: u32 = 1376256u32;
pub const WNNC_NET_TERMSRV: u32 = 3538944u32;
pub const WNNC_NET_TWINS: u32 = 2359296u32;
pub const WNNC_NET_VINES: u32 = 262144u32;
pub const WNNC_NET_VMWARE: u32 = 4128768u32;
pub const WNNC_NET_YAHOO: u32 = 2883584u32;
pub const WNNC_NET_ZENWORKS: u32 = 3997696u32;