use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HYPHEN_LINE_NUMBER: Regex = Regex::new(r"-(\d+)-").unwrap();
    static ref COLON_LINE_NUMBER: Regex = Regex::new(r":(\d+):").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct FileLine {
    pub file_name: String,
    pub line_num: u64,
}

pub fn parse(text: &String) -> Option<FileLine> {
    if let Some(_) = COLON_LINE_NUMBER.find(text) {
        return _parse(&COLON_LINE_NUMBER, text);
    }
    if let Some(_) = HYPHEN_LINE_NUMBER.find(text) {
        return _parse(&HYPHEN_LINE_NUMBER, text);
    }
    None
}

fn _parse(re: &Regex, text: &String) -> Option<FileLine> {
    let matches = re.find_iter(text);
    for mat in matches {
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
        let got = _parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/sample1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_colon_file() {
        let text = String::from("testdata/sam:9:ple1.txt:1:sushi");
        let got = _parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/sam:9:ple1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_colon_multibyte_file() {
        let text = String::from("testdata/サンプ:9:ル1.txt:1:sushi");
        let got = _parse(&COLON_LINE_NUMBER, &text);
        let want = Some(FileLine {
            file_name: "testdata/サンプ:9:ル1.txt".to_string(),
            line_num: 1,
        });
        assert_eq!(got, want);
    }
}
