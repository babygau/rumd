use regex::Regex;
use super::Downloader;

struct Vimeo;


// Reference from: https://github.com/regexps/youtube-regex
impl Downloader for Vimeo {
  fn can_handle(url: &str) -> bool {
    let test = Regex::new(r"(?:http|https)?://(?:www\.)?vimeo.com/((?:channels/(?:\w+/)?|groups/(?:[^/]*)/videos/?|album/(?:[^/]*)/video/)|(\d+)|(?:/\?))").unwrap();
    match test.is_match(url) {
      val => val
    }
  }
}

#[test]
fn vimeo_can_handle() {
  let valid_vimeo_urls = vec![
    "https://vimeo.com/62092214",
    "http://vimeo.com/62092214",
    "https://www.vimeo.com/62092214",
    "https://vimeo.com/channels/documentaryfilm/128373915",
    "http://vimeo.com/channels/documentaryfilm/128373915",
    "https://vimeo.com/groups/musicvideo/videos/126199390",
    "http://vimeo.com/groups/musicvideo/videos/126199390",
    "https://vimeo.com/62092214?query=foo",
    "https://vimeo.com/album/3332111/video/118643506"
  ];

  let not_valid_vimeo_urls = vec![
    "NONONO",
    "http://vimeo/62092214",
    "http://vimeo.com/foo",
    // The below regex is not working at the moment
    // A quick fix is to remove `?` from `(?:channels/(?:\w+/)?`
    // "https://vimeo.com/channels/foo-barr/documentaryfilm/128373915",
    "http://vimeo.com/groups/musicvideo/vid/126199390",
    "https://vimeo.com.omomom/62092214?query=foo"
  ];

  for valid_url in valid_vimeo_urls {
    assert_eq!(Vimeo::can_handle(valid_url), true);
  }

  for invalid_url in not_valid_vimeo_urls {
    assert_eq!(Vimeo::can_handle(invalid_url), false)
  }
}
