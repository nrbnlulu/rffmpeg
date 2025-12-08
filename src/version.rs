use std::fmt;

use crate::ffi;

#[derive(Debug)]
pub struct FFmpegVersion {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
}

impl fmt::Display for FFmpegVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[derive(Debug)]
pub struct FFmpegVersionsInfo {
    pub avutil: FFmpegVersion,
    pub avcodec: FFmpegVersion,
    pub avformat: FFmpegVersion,
    pub avdevice: FFmpegVersion,
    pub avfilter: FFmpegVersion,
    pub swscale: FFmpegVersion,
    pub swresample: FFmpegVersion,
    pub ffmpeg_version: String,
}

impl FFmpegVersionsInfo {
    pub fn new() -> Self {
        unsafe {
            Self {
                avutil: Self::decode_version(ffi::avutil_version()),
                avcodec: Self::decode_version(ffi::avcodec_version()),
                avformat: Self::decode_version(ffi::avformat_version()),
                avdevice: Self::decode_version(ffi::avdevice_version()),
                avfilter: Self::decode_version(ffi::avfilter_version()),
                swscale: Self::decode_version(ffi::swscale_version()),
                swresample: Self::decode_version(ffi::swresample_version()),
                ffmpeg_version: Self::get_ffmpeg_version_info(),
            }
        }
    }

    unsafe fn get_ffmpeg_version_info() -> String {
        use std::ffi::CStr;

        let version_ptr = ffi::av_version_info();
        if !version_ptr.is_null() {
            let c_str = CStr::from_ptr(version_ptr);
            c_str.to_string_lossy().into_owned()
        } else {
            String::from("Unknown")
        }
    }

    fn decode_version(version_int: u32) -> FFmpegVersion {
        let major = (version_int >> 16) & 0xFF;
        let minor = (version_int >> 8) & 0xFF;
        let micro = version_int & 0xFF;

        FFmpegVersion {
            major,
            minor,
            micro,
        }
    }

    pub fn print_all(&self) {
        println!("FFmpeg {} Library Versions:", self.ffmpeg_version);
        println!("  libavutil:    {}", self.avutil);
        println!("  libavcodec:   {}", self.avcodec);
        println!("  libavformat:  {}", self.avformat);
        println!("  libavdevice:  {}", self.avdevice);
        println!("  libavfilter:  {}", self.avfilter);
        println!("  libswscale:   {}", self.swscale);
        println!("  libswresample: {}", self.swresample);
    }

    /// Returns true if all library versions are compatible (same major version for core libraries)
    pub fn is_consistent(&self) -> bool {
        // Check if the core libraries have compatible major versions
        self.avutil.major >= 58 && self.avcodec.major >= 58 && self.avformat.major >= 58
    }

    /// Returns a string representation of all version information (same as Display implementation)
    pub fn full_string(&self) -> String {
        format!(
            "ffmpeg={}, libavutil={}, libavcodec={}, libavformat={}, libavdevice={}, libavfilter={}, libswscale={}, libswresample={}",
            self.ffmpeg_version, self.avutil, self.avcodec, self.avformat, self.avdevice, self.avfilter, self.swscale, self.swresample
        )
    }
}

impl fmt::Display for FFmpegVersionsInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "ffmpeg={}, libavutil={}, libavcodec={}, libavformat={}, libavdevice={}, libavfilter={}, libswscale={}, libswresample={}",
            self.ffmpeg_version, self.avutil, self.avcodec, self.avformat, self.avdevice, self.avfilter, self.swscale, self.swresample
        )
    }
}

impl Default for FFmpegVersionsInfo {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export the version functions from different modules for convenience
pub fn version() -> FFmpegVersionsInfo {
    FFmpegVersionsInfo::new()
}

// Access the raw version integers if needed
pub mod raw {
    pub use crate::codec::version as avcodec_version;
    pub use crate::device::version as avdevice_version;
    pub use crate::filter::version as avfilter_version;
    pub use crate::format::version as avformat_version;
    pub use crate::software::resampling::version as swresample_version;
    pub use crate::software::scaling::version as swscale_version;
    pub use crate::util::version as avutil_version;
}
