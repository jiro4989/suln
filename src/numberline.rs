use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Error, Read};

#[derive(Debug, PartialEq)]
pub struct NumberLine {
    pub line_num: u64,
    pub line: String,
}

pub fn readline_surround_of_line_number(
    inner: &mut dyn Read,
    line_nums: &Vec<u64>,
    before_line_count: u64,
    after_line_count: u64,
) -> Result<Vec<(u64, NumberLine)>, Error> {
    // use HashMap to remove duplicated lines.
    let mut captured_lines: HashMap<u64, NumberLine> = HashMap::new();

    let reader = BufReader::new(inner);
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        // `enumerate` index starts with 0.
        // but `grep` line number starts with 1.
        let i = (i + 1) as u64;
        for line_num in line_nums {
            // TODO: range may overlap.
            // e.g. 1..5 and 3..7
            let num_range = (line_num - before_line_count)..(line_num + after_line_count + 1);

            if !num_range.contains(&i) {
                continue;
            }

            // skip when HashMap has already line number key.
            if captured_lines.contains_key(&i) {
                continue;
            }

            let fl = NumberLine {
                line_num: i,
                line: line.clone(),
            };
            captured_lines.insert(i, fl);
        }
    }

    // sort FileLines with line number,
    // because of order of HashMap is not guaranteed.
    let mut vec = captured_lines.into_iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_stream(s: &str) -> &[u8] {
        s.as_bytes()
    }

    #[test]
    fn test_readline_surround_of_line_number_head() {
        let mut inner = str_stream("hello\nworld\nfoobar");
        let got = readline_surround_of_line_number(&mut inner, &vec![2u64], 1u64, 1u64);
        let want = vec![
            (
                1,
                NumberLine {
                    line_num: 1,
                    line: "hello".to_string(),
                },
            ),
            (
                2,
                NumberLine {
                    line_num: 2,
                    line: "world".to_string(),
                },
            ),
            (
                3,
                NumberLine {
                    line_num: 3,
                    line: "foobar".to_string(),
                },
            ),
        ];
        assert_eq!(got.ok(), Some(want));
    }

    #[test]
    fn test_readline_surround_of_line_number_head2() {
        let mut inner = str_stream("hello\nworld\nfoobar");
        let got = readline_surround_of_line_number(&mut inner, &vec![3u64], 1u64, 1u64);
        let want = vec![
            (
                2,
                NumberLine {
                    line_num: 2,
                    line: "world".to_string(),
                },
            ),
            (
                3,
                NumberLine {
                    line_num: 3,
                    line: "foobar".to_string(),
                },
            ),
        ];
        assert_eq!(got.ok(), Some(want));
    }

    #[test]
    fn test_readline_surround_of_line_number_before_and_after_are_0() {
        let mut inner = str_stream("hello\nworld\nfoobar");
        let got = readline_surround_of_line_number(&mut inner, &vec![3u64], 0u64, 0u64);
        let want = vec![(
            3,
            NumberLine {
                line_num: 3,
                line: "foobar".to_string(),
            },
        )];
        assert_eq!(got.ok(), Some(want));
    }

    #[test]
    fn test_readline_surround_of_line_number_duplicated() {
        let mut inner = str_stream("hello\nworld\nfoobar");
        let got =
            readline_surround_of_line_number(&mut inner, &vec![1u64, 2u64, 3u64, 4u64], 1u64, 1u64);
        let want = vec![
            (
                1,
                NumberLine {
                    line_num: 1,
                    line: "hello".to_string(),
                },
            ),
            (
                2,
                NumberLine {
                    line_num: 2,
                    line: "world".to_string(),
                },
            ),
            (
                3,
                NumberLine {
                    line_num: 3,
                    line: "foobar".to_string(),
                },
            ),
        ];
        assert_eq!(got.ok(), Some(want));
    }
}
