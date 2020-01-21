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


struct PatternInfo{
  pattern:String,
  uri_vars: i32,
  single_wildcards: i32,
  double_wildcards: i32,
  catch_all_pattern: bool,
  prefix_pattern: bool,
  length:usize,
}

impl PatternInfo{
  pub fn new(pattern:&mut String)->Self{
      let pattern :String=pattern.clone();
      let uri_vars =0;
    
      let pattern_info=PatternInfo{
        pattern:pattern,
        uri_vars:0,
        single_wildcards:0,
        double_wildcards:0,
        catch_all_pattern:false,
        prefix_pattern:false,
        length:0,
      };
      return pattern_info
  }

  pub fn get_total_count(&self) -> i32{
    self.uri_vars+self.single_wildcards+self.double_wildcards*2
  }

 // Returns the length of the given pattern, where template variables are considered to be 1 long.
  pub fn get_length(&self) -> usize{
     let after = Regex::new(VARIABLE_PATTERN).unwrap().replace_all(&self.pattern, "#");

     after.chars().count()
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
