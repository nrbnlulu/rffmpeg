#[test]
fn test_version_info() {
    rffmpeg::init().unwrap();

    // Get version info using the new struct
    let versions = rffmpeg::version::version();

    // Test Display implementation
    println!("\nUsing Display trait: {}", versions);

    // Test consistency check
    println!(
        "\nVersion consistency check: {}",
        if versions.is_consistent() {
            "OK"
        } else {
            "INCONSISTENT"
        }
    );

    // Access individual library versions
    println!("\nAccessing individual versions:");
    println!("libavutil version: {}", versions.avutil);
    println!("libavcodec version: {}", versions.avcodec);
    println!("libavformat version: {}", versions.avformat);

    // Test Default implementation
    let default_versions = rffmpeg::version::FFmpegVersionsInfo::default();
    assert!(default_versions.avutil.major >= 50); // Reasonable minimum version


    // Assertions to validate the test
    assert!(versions.avutil.major >= 50); // Reasonable minimum version
    assert!(versions.avcodec.major >= 50); // Reasonable minimum version
    assert!(versions.avformat.major >= 50); // Reasonable minimum version
    assert!(versions.is_consistent()); // Should be consistent
}
