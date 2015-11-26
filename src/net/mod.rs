pub use self::youtube::*;
pub use self::vimeo::*;

pub mod youtube;
pub mod vimeo;

pub trait Downloader {
    fn name(&self) -> &str;
}
