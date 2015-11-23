use regex::Regex;
use super::Downloader;

struct Vimeo;


impl Downloader for Vimeo {
  fn can_handle(url: &str) -> bool {
    let test = Regex::new(r"^(https://vimeo.com)").unwrap();
    let res = test.is_match(url);
    match res {
      bool => bool
    }
  }
}

#[test]
fn vimeo_can_handle() {
  let a_valid_vimeo_url = "https://vimeo.com/watch";
  let not_valid_vimeo_urls = vec!["blahblahblah", "https://youtube.com", "http://vimeo.com"];
  assert_eq!(Vimeo::can_handle(a_valid_vimeo_url), true);
  for url in not_valid_vimeo_urls {
    assert_eq!(Vimeo::can_handle(url), false)
  }
}
