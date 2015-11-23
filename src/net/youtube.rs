use regex::Regex;
use super::Downloader;

struct YouTube;

// Reference from: https://github.com/regexps/youtube-regex
impl Downloader for YouTube {
  fn can_handle(url: &str) -> bool {
    let test = Regex::new(r"(?:youtube\.com/\S*(?:(?:/e(?:mbed))?/|watch/?\?(?:\S*?&?v=))|youtu\.be/)([a-zA-Z0-9_-]{6,11})").unwrap();
    match test.is_match(url) {
      val => val
    }
  }
}

#[test]
fn youtube_can_handle() {
  let valid_youtube_urls = vec![
    "youtube.com/user/sandervandoorntv/watch?v=WijF8aivOo8",
    "youtube.com/user/sandervandoorntv/watch?v=WijF8aivOo8&feature=related",
    "youtube.com/user/sandervandoorntv/watch?feature=related&v=WijF8aivOo8",
    "youtube.com/watch?v=0EWbonj7f18",
    "youtube.com/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/watch?v=0EWbonj7f18&feature=related",
    "youtube.com/embed/watch?v=0EWbonj7f18",
    "youtube.com/embed/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/embed/v/0EWbonj7f18",
    "youtube.com/e/v/0EWbonj7f18",
    "youtube.com/e/watch?v=0EWbonj7f18",
    "youtube.com/e/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch?v=WijF8aivOo8",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch?v=WijF8aivOo8&feature=related",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch?feature=related&v=WijF8aivOo8",
    "youtube.com/attribution_link?u=/watch?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/watch?v=0EWbonj7f18&feature=related",
    "youtube.com/attribution_link?u=/embed/watch?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/embed/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/embed/v/0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/v/0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/watch?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/watch?feature=related&v=0EWbonj7f18",
    "youtube.com/user/sandervandoorntv/watch/?v=WijF8aivOo8",
    "youtube.com/user/sandervandoorntv/watch/?v=WijF8aivOo8&feature=related",
    "youtube.com/user/sandervandoorntv/watch/?feature=related&v=WijF8aivOo8",
    "youtube.com/watch/?v=0EWbonj7f18",
    "youtube.com/watch/?feature=related&v=0EWbonj7f18",
    "youtube.com/watch/?v=0EWbonj7f18&feature=related",
    "youtube.com/embed/watch/?v=0EWbonj7f18",
    "youtube.com/embed/watch/?feature=related&v=0EWbonj7f18",
    "youtube.com/e/watch/?v=0EWbonj7f18",
    "youtube.com/e/watch/?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch/?v=WijF8aivOo8",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch/?v=WijF8aivOo8&feature=related",
    "youtube.com/attribution_link?u=/user/sandervandoorntv/watch/?feature=related&v=WijF8aivOo8",
    "youtube.com/attribution_link?u=/watch/?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/watch/?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/watch/?v=0EWbonj7f18&feature=related",
    "youtube.com/attribution_link?u=/embed/watch/?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/embed/watch/?feature=related&v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/embed/v/0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/v/0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/watch/?v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/watch/?feature=related&v=0EWbonj7f18",
    "youtu.be/WijF8aivOo8",
    "youtu.be/0EWbonj7f18"
  ];

  let not_valid_youtube_urls = vec![
    "youtube.com/embed/v=0EWbonj7f18",
    "youtube.com/e/v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/embed/v=0EWbonj7f18",
    "youtube.com/attribution_link?u=/e/v=0EWbonj7f18",
    "hey guys go to youtube and v=thisweird video i found",
    "i found a youtube video on 4chan /v/foobarbaz",
    "but we work at youtu.be and /thisweird video"
  ];

  for valid_url in valid_youtube_urls {
    assert_eq!(YouTube::can_handle(valid_url), true);
  }

  for invalid_url in not_valid_youtube_urls {
    assert_eq!(YouTube::can_handle(invalid_url), false)
  }
}
