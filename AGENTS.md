# rffmpeg Project Context

## Project Overview

rffmpeg is a safe Rust wrapper for FFmpeg libraries (version 6.1 - 8.0), serving as an FFmpeg 4 compatible fork of the original ffmpeg crate. It provides high-level Rust APIs for multimedia operations including video/audio encoding/decoding, transcoding, format handling, filtering, and more.

### Key Features
- Safe Rust wrapper around FFmpeg C libraries
- Supports FFmpeg versions 6.1 through 8.0
- Modular design with optional features (codec, device, filter, format, etc.)
- Comprehensive multimedia processing capabilities
- Cross-platform support (Windows, Linux, macOS)

### Architecture
The library is organized into several main modules:
- `util`: Core utilities including error handling, frame management, logging, and rational numbers
- `codec`: Video and audio encoding/decoding functionality
- `format`: Container format handling and streaming
- `device`: Device input/output operations
- `filter`: Audio and video filtering capabilities
- `software`: Software-based resampling and scaling

## Building and Running

### Prerequisites
- Rust toolchain (latest stable recommended)
- FFmpeg libraries (version 6.1-8.0)

### Environment Setup

#### Linux
```bash
# Download FFmpeg builds from https://github.com/BtbN/FFmpeg-Builds/releases
export FFMPEG_DIR=/path/to/ffmpeg
export FFMPEG_PATH=/path/to/ffmpeg/bin
export FFMPEG_INCLUDE_DIR=/path/to/ffmpeg/include
export FFMPEG_LIB_DIR=/path/to/ffmpeg/lib
export LD_LIBRARY_PATH=${FFMPEG_LIB_DIR}:${LD_LIBRARY_PATH}
```

#### macOS
```bash
brew install ffmpeg pkg-config
```

#### Windows
1. Download shared builds from [FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds/releases)
2. Set environment variables:
   - `FFMPEG_DIR`
   - `FFMPEG_INCLUDE_DIR`
   - `FFMPEG_LIB_DIR`

### Build Commands
```bash
# Build the project
cargo build

# Build with specific features
cargo build --features="codec,format,filter"

# Run examples
cargo run --example transcode-x264 input.mp4 output.mp4

# Run tests
cargo test
```

### Features Configuration
The library supports many optional features through Cargo features:
- `default`: codec, device, filter, format, software-resampling, software-scaling
- `static`: Link statically with FFmpeg
- `codec`: AVCodec component
- `format`: AVFormat component
- `filter`: AVFilter component
- `device`: AVDevice component
- Various codec-specific features (x264, x265, vp9, etc.)

## Development Conventions

### Code Structure
- The library uses a layered approach: high-level Rust APIs over `ffmpeg-sys-next` C bindings
- Uses idiomatic Rust patterns with Result types for error handling
- Leverages Rust's type system for safety (e.g., preventing use-after-free)
- Follows FFmpeg conventions while providing Rust-friendly interfaces

### API Design
- Initialization with `ffmpeg::init()` before using other APIs
- Resource management through Rust's ownership system
- Modern send/receive APIs preferred over deprecated decode/encode APIs
- Extensive use of builder patterns and context objects

### Error Handling
- All operations that can fail return `Result<T, Error>`
- Errors are represented by the `ffmpeg::Error` enum
- Common POSIX error codes are re-exported under `util::error`

### Example Usage Patterns
```rust
// Initialize FFmpeg
ffmpeg::init()?;

// Open input file
let input_ctx = format::input(&input_file)?;

// Process streams
for stream in input_ctx.streams() {
    // Access stream parameters
    let context = codec::context::Context::from_parameters(stream.parameters())?;
    let decoder = context.decoder().video()?;
    // Process frames...
}

// Transcoding follows send/receive pattern:
// - Send packets to decoder
// - Receive frames from decoder
// - Send frames to encoder  
// - Receive packets from encoder
```

### Testing and Examples
- Multiple example programs in the `examples/` directory demonstrate various use cases
- Examples show transcoding, metadata extraction, frame dumping, and other common operations
- Examples serve as both documentation and integration tests

## Key Files and Directories
- `Cargo.toml`: Feature flags and dependencies (particularly `ffmpeg-sys-next`)
- `src/lib.rs`: Main library entry point and module organization
- `src/version.rs`: FFmpeg version information utilities with `FFmpegVersion` and `FFmpegVersionsInfo` structs
- `build.rs`: Build script for linking with FFmpeg libraries
- `examples/`: Sample programs demonstrating library usage
- `tests/`: Integration tests for the library functionality
- `src/codec/`, `src/format/`, `src/filter/`: Core multimedia modules
- `src/util/`: Common utilities and low-level operations
- `Taskfile.yml`: Development task automation (lint, format, test, etc.)

## Versioning and Compatibility
- Follows FFmpeg version compatibility (currently 6.1-8.0)
- Uses conditional compilation to maintain compatibility across FFmpeg versions
- API deprecations are handled gracefully with new recommended patterns
- Breaking changes are introduced thoughtfully with migration paths

## Version Information API
- `FFmpegVersionsInfo` struct provides access to runtime versions of all major FFmpeg libraries
- Includes `libavutil`, `libavcodec`, `libavformat`, `libavdevice`, `libavfilter`, `libswscale`, and `libswresample`
- Implements `Debug`, `Display`, and `Default` traits for easy usage
- Provides `is_consistent()` method to check compatibility between library versions
- Provides `full_string()` method for string representation (equivalent to Display implementation)

## Development Workflow
- Use Taskfile commands for development tasks:
  - `task lint`: Run clippy linter with warnings as errors
  - `task lint-fix`: Run clippy and attempt to fix issues automatically
  - `task fmt`: Format code with rustfmt
  - `task fmt-check`: Check code formatting without modifying files
  - `task check`: Run all checks (formatting, linting, and tests)
  - `task fix-all`: Format code, fix lint issues, and run tests
- CI workflow in `.github/workflows/build.yml` runs on Linux, macOS, and Windows
- Tests include both unit tests and integration tests in the `tests/` directory
- Integration tests are automatically run in CI alongside example tests