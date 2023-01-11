use semver::{BuildMetadata, Prerelease, Version};

fn main() {
    let mut parsed = Version::parse("0.26.2").unwrap();

    println!("{}", parsed);

    assert_eq!(
        parsed,
        Version {
            major: 0,
            minor: 26,
            patch: 2,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
    );
    /***************************************
     * Check if given version is pre-release
     ***************************************/
    let version_1 = Version::parse("1.0.0-alpha").unwrap();
    let version_2 = Version::parse("1.0.0").unwrap();
    // !note has no is_prerelease method
    // assert!(version_1.is_prerelease());
    // assert!(!version_2.is_prerelease());
}
