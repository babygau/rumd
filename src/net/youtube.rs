use regex::Regex;
use super::Downloader;

struct YouTube;


impl Downloader for YouTube {
  fn can_handle(url: &str) -> bool {
    let test = Regex::new(r"^(https://www.youtube.com)").unwrap();
    match test.is_match(url) {
      val => val}
  }
}

#[test]
fn youtube_can_handle() {
  let a_valid_youtube_url = "https://www.youtube.com/watch";
  let not_valid_youtube_urls = vec!["blahblahblah", "https://www.vimeo.com", "http://www.youtube.com"];
  assert_eq!(YouTube::can_handle(a_valid_youtube_url), true);
  for url in not_valid_youtube_urls {
    assert_eq!(YouTube::can_handle(url), false)
  }
}
