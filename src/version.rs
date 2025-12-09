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
    #[cfg(feature = "codec")]
    pub avcodec: FFmpegVersion,
    #[cfg(feature = "format")]
    pub avformat: FFmpegVersion,
    #[cfg(feature = "device")]
    pub avdevice: FFmpegVersion,
    #[cfg(feature = "filter")]
    pub avfilter: FFmpegVersion,
    #[cfg(feature = "software-scaling")]
    pub swscale: FFmpegVersion,
    #[cfg(feature = "software-resampling")]
    pub swresample: FFmpegVersion,
    #[cfg(feature = "resampling")]
    pub avresample: FFmpegVersion,
    #[cfg(feature = "postprocessing")]
    pub postproc: FFmpegVersion,
    pub ffmpeg_version: String,
}

impl FFmpegVersionsInfo {
    pub fn new() -> Self {
        unsafe {
            Self {
                avutil: Self::decode_version(ffi::avutil_version()),
                #[cfg(feature = "codec")]
                avcodec: Self::decode_version(ffi::avcodec_version()),
                #[cfg(feature = "format")]
                avformat: Self::decode_version(ffi::avformat_version()),
                #[cfg(feature = "device")]
                avdevice: Self::decode_version(ffi::avdevice_version()),
                #[cfg(feature = "filter")]
                avfilter: Self::decode_version(ffi::avfilter_version()),
                #[cfg(feature = "software-scaling")]
                swscale: Self::decode_version(ffi::swscale_version()),
                #[cfg(feature = "software-resampling")]
                swresample: Self::decode_version(ffi::swresample_version()),
                #[cfg(feature = "resampling")]
                avresample: Self::decode_version(ffi::avresample_version()),
                #[cfg(feature = "postprocessing")]
                postproc: Self::decode_version(ffi::postproc_version()),
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

    /// Returns true if all available library versions are compatible (same major version for core libraries)
    pub fn is_consistent(&self) -> bool {
        // Check if the core libraries that are available have compatible major versions
        let mut consistent = self.avutil.major >= 58;

        #[cfg(feature = "codec")]
        {
            consistent = consistent && self.avcodec.major >= 58;
        }

        #[cfg(feature = "format")]
        {
            consistent = consistent && self.avformat.major >= 58;
        }

        consistent
    }
}

impl fmt::Display for FFmpegVersionsInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = format!("ffmpeg={}, libavutil={}", self.ffmpeg_version, self.avutil);

        #[cfg(feature = "codec")]
        {
            output.push_str(&format!(", libavcodec={}", self.avcodec));
        }

        #[cfg(feature = "format")]
        {
            output.push_str(&format!(", libavformat={}", self.avformat));
        }

        #[cfg(feature = "device")]
        {
            output.push_str(&format!(", libavdevice={}", self.avdevice));
        }

        #[cfg(feature = "filter")]
        {
            output.push_str(&format!(", libavfilter={}", self.avfilter));
        }

        #[cfg(feature = "software-scaling")]
        {
            output.push_str(&format!(", libswscale={}", self.swscale));
        }

        #[cfg(feature = "software-resampling")]
        {
            output.push_str(&format!(", libswresample={}", self.swresample));
        }

        #[cfg(feature = "resampling")]
        {
            output.push_str(&format!(", libavresample={}", self.avresample));
        }

        #[cfg(feature = "postprocessing")]
        {
            output.push_str(&format!(", libpostproc={}", self.postproc));
        }

        write!(f, "{}", output)
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
