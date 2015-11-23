mod youtube;
mod vimeo;

pub trait Downloader {
  fn can_handle(url: &str) -> bool;
}
