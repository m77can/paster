use regex::Regex;
use std::collections::HashMap;

static DEFAULT_PATH_SEPARATOR: &str = "/";

static CACHE_TURNOFF_THRESHOLD: i32 = 65536;

const VARIABLE_PATTERN: &'static str = "\\{[^/]+?\\}";

static WILDCARD_CHARS: [char; 3] = ['*', '?', '{'];

pub trait PathMatcher {
  fn is_pattern(path: String) -> bool;

  fn r#match(pattern: String, path: String) -> bool;

  fn match_start(pattern: String, path: String) -> bool;
  fn extract_path_within_pattern(pattern: String, path: String) -> String;

  fn extract_uri_template_variables(pattern: String, path: String) -> HashMap<String, String>;

  fn combine(pattern1: String, pattern2: String) -> String;
}

pub struct AntPathMatcher {
  path_separator: String,
  path_separator_pattern_cache: PathSeparatorPatternCache,
  case_sensitive :bool,
  trim_tokes: bool,
  cache_patterns:bool,
  tokenized_pattern_cache:

}

impl PathMatcher for AntPathMatcher {
  fn is_pattern(_: std::string::String) -> bool {
    unimplemented!()
  }
  fn r#match(_: String, _: String) -> bool {
    unimplemented!()
  }
  fn match_start(_: std::string::String, _: std::string::String) -> bool {
    unimplemented!()
  }
  fn extract_path_within_pattern(
    _: std::string::String,
    _: std::string::String,
  ) -> std::string::String {
    unimplemented!()
  }
  fn extract_uri_template_variables(
    _: std::string::String,
    _: std::string::String,
  ) -> HashMap<String, String> {
    unimplemented!()
  }
  fn combine(_: std::string::String, _: std::string::String) -> std::string::String {
    unimplemented!()
  }
}

struct PathSeparatorPatternCache {
  ends_on_wildcard: String,
  ends_on_double_wildcard: String,
}

impl PathSeparatorPatternCache {
  fn new(path_separator: &mut String) -> Self {
    let mut ends_on_wildcard = path_separator.clone();
    let mut ends_on_double_wildcard = path_separator.clone();
    ends_on_wildcard.push_str("*");
    ends_on_double_wildcard.push_str("**");

    PathSeparatorPatternCache {
      ends_on_wildcard: ends_on_wildcard,
      ends_on_double_wildcard: ends_on_double_wildcard,
    }
  }
}
