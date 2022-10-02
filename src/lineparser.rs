use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HYPHEN_LINE_NUMBER: Regex = Regex::new(r"-(\d+)-").unwrap();
    static ref COLON_LINE_NUMBER: Regex = Regex::new(r":(\d+):").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct LineParser {
    // filename-1-
    hyphenLine: Option<FileLine>,
    // filename:1:
    coronLine: Option<FileLine>,
}

#[derive(Debug, PartialEq)]
struct FileLine {
    file_name: String,
    line_num: u64,
}

impl LineParser {
    pub fn parse(&mut self, text: &String) {
        // if let Some(v) = self.hyphenLine {
        //     // self.hyphenLine = Some(FileLine {});
        //     return;
        // }

        // Parsing is the first time if hyphenLine does not exists.
        self.hyphenLine = parse(&COLON_LINE_NUMBER, text);
    }
}

fn parse(re: &Regex, text: &String) -> Option<FileLine> {
    let found = re.find_iter(text);
    let matches = re.find_iter(text);
    for (pos, mat) in matches.enumerate() {
        // enumerate index is starts with 0.
        // but captures need 1.
        let pos = pos + 1;

        let start_pos = mat.start();
        let file_name = text.get(0..start_pos).unwrap();
        if !Path::new(file_name).exists() {
            // check next matches if file_name does not exist.
            continue;
        }

        let tail = text.get(start_pos..).unwrap();
        let line_num = re.captures(tail).unwrap().get(1).unwrap().as_str().parse();
        if let Err(_) = line_num {
            continue;
        }

        let line_num = line_num.ok().unwrap();
        let fl = FileLine {
            file_name: file_name.to_string(),
            line_num,
        };
        return Some(fl);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_normal_file() {
        let text = String::from("testdata/sample1.txt:1:hello world");
        let got = parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/sample1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_colon_file() {
        let text = String::from("testdata/sam:9:ple1.txt:1:sushi");
        let got = parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/sam:9:ple1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_colon_multibyte_file() {
        let text = String::from("testdata/サンプ:9:ル1.txt:1:sushi");
        let got = parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/サンプ:9:ル1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }
}
