use clap::Subcommand;

pub mod bump_version;
pub mod publish;
pub mod semver_check;
pub mod tag_releases;

pub use bump_version::*;
pub use publish::*;
pub use semver_check::*;
pub use tag_releases::*;

// ----------------------------------------------------------------------------
// Subcommands

#[derive(Debug, Subcommand)]
pub enum Release {
    /// Bump the version of the specified package(s).
    ///
    /// This command will, for each specified package:
    /// - Verify that the crate can be released (e.g. it doesn't refer to git
    ///   dependencies)
    /// - Update the version in `Cargo.toml` files
    /// - Update the version in dependencies' `Cargo.toml` files
    /// - Check if the changelog can be finalized
    /// - Update the version in the changelog
    /// - Replaces `{{currentVersion}}` markers in source files and the
    ///   migration guide.
    BumpVersion(BumpVersionArgs),
    /// Attempt to publish the specified package.
    Publish(PublishArgs),
    /// Generate git tags for all new package releases.
    TagReleases(TagReleasesArgs),
}
