pub use self::youtube::*;
pub use self::vimeo::*;

pub mod youtube;
pub mod vimeo;

pub trait Downloader {
  fn can_handle(&self, url: &str) -> bool;
  fn id(&self) -> &str;
}
