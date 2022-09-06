use std::{collections::VecDeque, result, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YarnTokenType {
    COLON,
    ARROW,
    WORD,
    TAB,
    START_LINE,
    END_LINE,
    START_NODE,
    END_NODE,
    SPACE,
    IF,
    ELSE,
    ELSEIF,
    ENDIF,
    QUOTATION,
    PERIOD,
    HASHTAG,
    LEFT_SQUARE_BRACKET,
    RIGHT_SQUARE_BRACKET,
    LESS_THAN,
    LESS_THAN_EQ,
    GREATER_THAN,
    GREATER_THAN_EQ,
    FORWARD_SLASH,
    EOF
}

const LOOK_UP_MAP : [(YarnTokenType, &'static str); 19] = [
    (YarnTokenType::COLON, ":"),
    (YarnTokenType::ARROW, "->"),
    (YarnTokenType::START_NODE, "---"),
    (YarnTokenType::END_NODE, "==="),
    (YarnTokenType::SPACE, " "),
    (YarnTokenType::ELSEIF, "elseif"),
    (YarnTokenType::ENDIF, "endif"),
    (YarnTokenType::IF, "if"),
    (YarnTokenType::ELSE, "else"),
    (YarnTokenType::QUOTATION, "\""),
    (YarnTokenType::PERIOD, "."),
    (YarnTokenType::LESS_THAN, "<"),
    (YarnTokenType::GREATER_THAN, ">"),
    (YarnTokenType::LESS_THAN_EQ, "<="),
    (YarnTokenType::GREATER_THAN_EQ, ">="),
    (YarnTokenType::HASHTAG, "#"),
    (YarnTokenType::LEFT_SQUARE_BRACKET, "["),
    (YarnTokenType::RIGHT_SQUARE_BRACKET, "]"),
    (YarnTokenType::FORWARD_SLASH, "/"),
    //(YarnTokenType::END_LINE, "\n")
];

pub struct YarnToken<'queue> {
    token_type : YarnTokenType,
    line : usize,
    col_start : usize,
    col_end : usize,
    contents : &'queue str
}

impl <'a> YarnToken<'a> {
    pub fn token_type(&'a self) -> &YarnTokenType {
        &self.token_type
    }

    pub fn contents(&'a self) -> &str {
        self.contents
    }

    pub fn line(&'a self) -> usize {
        self.line
    }

    pub fn col_start(&'a self) -> usize {
        self.col_start
    }
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

pub fn tokenize_yarn_string<'a>(source : &'a str) -> VecDeque<YarnToken<'a>> {
    let mut tokens : VecDeque<YarnToken<'a>> = VecDeque::new();

    let mut cur_line = 0;

    let lines = source.lines();
    for line in lines {
        tokens.append(&mut parse_line(VecDeque::new(), line, cur_line, 0, 0));
        cur_line += 1;
    }

    tokens.push_back(create_marker_token(YarnTokenType::EOF, cur_line, 0));
    tokens
}

fn create_marker_token<'a>(token_type : YarnTokenType, line : usize, col : usize) -> YarnToken<'a> {
    YarnToken { token_type: token_type, line, col_start: col, col_end: 0, contents: "" }
}

fn parse_line<'a>(
    mut tokens : VecDeque<YarnToken<'a>>, 
    line : &'a str, 
    cur_line : usize, 
    col_start : usize, 
    col_end : usize
) -> VecDeque<YarnToken<'a>> {
    unsafe {
        let current = line.get_unchecked(col_start .. col_start + col_end);

        if line.len() == col_start + col_end {
            if col_end != 0 {
                let mut t = match_token(&current, cur_line, col_start, true);
                tokens.append(&mut t)
            }

            tokens.push_front(YarnToken {
                token_type: YarnTokenType::START_LINE,
                line: cur_line,
                col_start : 0,
                col_end : 0,
                contents: "",
            });

            tokens.push_back(YarnToken {
                token_type: YarnTokenType::END_LINE,
                line: cur_line,
                col_start : col_start + col_end,
                col_end : col_start + col_end,
                contents: "",
            });
            
            //println!("{:?}", tokens);
            return tokens;
        }
        
        let mut t = if col_start + col_end == line.len() {
            match_token(&current, cur_line, col_start, true)
        } else {
            match_token(&current, cur_line, col_start, false)
        };
        
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
        if !token_match {
            if value.contains(s) {
                let start = value.find(s).unwrap();
                token_match = true;
                if value == s {
                    let token = YarnToken {
                        token_type: t,
                        line,
                        col_start: col + start,
                        col_end: col + start + s.len(),
                        contents: s,
                    };
                    tokens.push_back(token);
                } else {
                    //Old cod, keeping around for context
                    // let mut offset = 0;
                    // if start != 0 {
                    //     let mut result1 = match_token(value.get_unchecked(0 .. start), line, col, true);
                    //     offset += start;
                    //     tokens.append(&mut result1)
                    // }
    
                    // let mut result2 = match_token(value.get_unchecked(start .. start + s.len()), line, col + offset, true);
                    // tokens.append(&mut result2);
                    // offset += s.len();
    
                    // if start + s.len() != value.len() {
                    //     let mut result3 = match_token(value.get_unchecked(start + s.len() .. value.len()), line, col + offset, true);
                    //     tokens.append(&mut result3);
                    // }

                    let mut result1 = match_token(value.get_unchecked(0 .. start), line, col, true);
                    let mut result2 = match_token(value.get_unchecked(start .. value.len()), line, col + start, true);

                    tokens.append(&mut result1);
                    tokens.append(&mut result2);
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::YarnTokenType::*;

    macro_rules! has_tokens{
        ($tokens:ident, $($token:ident),*) => {
            {
                $(
                    if let Some(_t) = $tokens.pop_front() {
                        assert_eq!(_t.token_type(), &$token);
                    }
                )*
            }
        }
    }

    #[test]
    fn tokenize_command() {
        let mut tokens = tokenize_yarn_string("<<run_event \"Test\">>");
        has_tokens!(tokens, START_LINE, LESS_THAN, LESS_THAN, WORD, SPACE, QUOTATION, WORD, QUOTATION, GREATER_THAN, GREATER_THAN, END_LINE)
    }

    #[test]
    fn tokenize_split() {
        use crate::token::YarnTokenType::*;

        unsafe {
            let mut tokens = match_token("word->", 0, 0, false);
            has_tokens!(tokens, WORD, ARROW);
        }
    }
}