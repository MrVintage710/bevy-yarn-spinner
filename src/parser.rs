use std::{collections::VecDeque, result, fmt::Debug};

#[derive(Debug, Clone, Copy)]
pub enum YarnTokenType {
    COLON,
    ARROW,
    WORD,
    TAB,
    START_LINE,
    END_LINE,
    START_NODE,
    END_NODE,
    START_SCRIPT,
    END_SCRIPT,
    SPACE,
    IF
}

const LOOK_UP_MAP : [(YarnTokenType, &'static str); 8] = [
    (YarnTokenType::COLON, ":"),
    (YarnTokenType::ARROW, "->"),
    (YarnTokenType::START_SCRIPT, "<<"),
    (YarnTokenType::END_SCRIPT, ">>"),
    (YarnTokenType::START_NODE, "---"),
    (YarnTokenType::END_NODE, "==="),
    (YarnTokenType::SPACE, " "),
    (YarnTokenType::IF, "if"),
    //(YarnTokenType::END_LINE, "\n")
];

pub struct YarnToken<'queue> {
    token_type : YarnTokenType,
    line : usize,
    col_start : usize,
    col_end : usize,
    contents : &'queue str
}

impl <'a> Debug for YarnToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("('{}' | {:?})", self.contents, self.token_type).as_str())
    }
}

#[derive(Default, Debug)]
pub struct YarnTokenQueue<'a> {
    queue : VecDeque<YarnToken<'a>>,
}

impl <'a> YarnTokenQueue<'a> {
    pub fn push(&mut self, token : YarnToken<'a>) {
        self.queue.push_back(token);
    }
}

pub fn parse_yarn_string<'a>(source : &'a str) -> VecDeque<YarnToken<'a>> {
    let mut tokens : VecDeque<YarnToken<'a>> = VecDeque::new();

    let mut cur_line = 0;

    let lines = source.lines();
    for line in lines {
        tokens.append(&mut parse_line(VecDeque::new(), line, cur_line, 0, 0));
        cur_line += 1;
    }

    tokens
}

pub fn parse_line<'a>(
    mut tokens : VecDeque<YarnToken<'a>>, 
    line : &'a str, 
    cur_line : usize, 
    col_start : usize, 
    col_end : usize
) -> VecDeque<YarnToken<'a>> {
    unsafe {
        let current = line.get_unchecked(col_start .. col_start + col_end);

        if line.len() < col_start + col_end {
            tokens.push_front(YarnToken {
                token_type: YarnTokenType::START_LINE,
                line: cur_line,
                col_start : 0,
                col_end : 0,
                contents: "",
            });

            if col_start != col_end {
                let mut t = match_token(&current, cur_line, col_start, true);
                tokens.append(&mut t)
            }
            
            //println!("{:?}", tokens);
            return tokens;
        }
        
        let mut t = match_token(&current, cur_line, col_start, false);
        
        if !t.is_empty() {
            tokens.append(&mut t);
            return parse_line(tokens, line, cur_line, col_start + col_end, 0);
        } else {
            return parse_line(tokens, line, cur_line, col_start, col_end + 1);
        }
    }
}

unsafe fn match_token<'a>(value : &'a str, line : usize, col : usize, capture_word : bool) -> VecDeque<YarnToken<'a>> {
    let mut tokens = VecDeque::new();
    let mut token_match = false;

    for (t, s) in LOOK_UP_MAP {
        if value.contains(s) {
            let start = value.find(s).unwrap();
            if value == s {
                let token = YarnToken {
                    token_type: t,
                    line,
                    col_start: col + start,
                    col_end: col + start + s.len(),
                    contents: s,
                };
                token_match = true;
                tokens.push_back(token);
            } else {
                let mut offset = 0;
                if start != 0 {
                    let mut result1 = match_token(value.get_unchecked(0 .. start), line, col, true);
                    offset += start;
                    tokens.append(&mut result1)
                }

                let mut result2 = match_token(value.get_unchecked(start .. start + s.len()), line, col + offset, true);
                tokens.append(&mut result2);
                offset += s.len();

                if start + s.len() != value.len() {
                    let mut result3 = match_token(value.get_unchecked(start + s.len() .. value.len()), line, col + offset, true);
                    tokens.append(&mut result3);
                }
            }
        } 
    }

    if capture_word && !token_match {
        let token = YarnToken{
            token_type: YarnTokenType::WORD,
            line,
            col_start: col,
            col_end: col + value.len(),
            contents: value,
        };
        tokens.push_back(token);
    }

    tokens
}