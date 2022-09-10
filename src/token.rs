use std::{collections::VecDeque, fmt::Debug, result};

//==================================================================================================================
//                       Tokens
//==================================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YarnTokenType {
    COLON,
    ARROW,
    WORD,
    TAB,
    MULT,
    DIV,
    ADD,
    SUB,
    EQUAL,
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
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_SQUARE_BRACKET,
    RIGHT_SQUARE_BRACKET,
    EQUAL_TOO,
    LESS_THAN,
    LESS_THAN_EQ,
    GREATER_THAN,
    GREATER_THAN_EQ,
    FORWARD_SLASH,
    EOF
}

const LOOK_UP_MAP : [(YarnTokenType, &'static str); 26] = [
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
    (YarnTokenType::EQUAL, "="),
    (YarnTokenType::HASHTAG, "#"),
    (YarnTokenType::LEFT_SQUARE_BRACKET, "["),
    (YarnTokenType::RIGHT_SQUARE_BRACKET, "]"),
    (YarnTokenType::LEFT_PAREN, "("),
    (YarnTokenType::RIGHT_PAREN, ")"),
    (YarnTokenType::FORWARD_SLASH, "/"),
    (YarnTokenType::MULT, "*"),
    (YarnTokenType::DIV, "/"),
    (YarnTokenType::ADD, "+"),
    (YarnTokenType::SUB, "-"),
];

//==================================================================================================================
//                       Token Queue Matchers
//==================================================================================================================

struct YarnTokenMacher {
    tokens_to_match : Vec<YarnTokenType>,
    result : YarnTokenType
}

impl YarnTokenMacher {
    
    pub fn new(tokens_to_match : Vec<YarnTokenType>, result : YarnTokenType) -> YarnTokenMacher {
        YarnTokenMacher {
            tokens_to_match,
            result
        }
    }
    
    pub fn try_to_match<'a>(&'a self, tokens : &'a mut VecDeque<YarnToken<'a>>, index : usize) {
        let mut matching = true;
        for (offset, token_type) in self.tokens_to_match.iter().enumerate() {
            if let Some(cur_token) = tokens.get(index + offset) {
                matching &= (token_type == cur_token.token_type());
            }
        }

        if matching {
            let mut contents : Vec<&'a str> = Vec::new();
            let mut line = 0;
            let mut col_start = 0;
            let mut col_end = 0;
            for offset in 0..self.tokens_to_match.len() {
                let removed_token : Option<YarnToken<'a>> = tokens.remove(index + offset);
                if removed_token.is_some() {
                    let removed_token : YarnToken<'a> = removed_token.unwrap();

                    if offset == 0 {
                        col_start = removed_token.col_start;
                    }

                    if offset == self.tokens_to_match.len() - 1 {
                        col_end = removed_token.col_end;
                    }

                    
                }
            }

            let token  = YarnToken {
                token_type: self.result,
                line,
                col_start,
                col_end,
                contents: YarnContent::Joined(contents),
            };
            
            tokens.insert(index, token);
        }
    }
}

//===================================================================================================================================
//                       Token Content
//===================================================================================================================================

pub enum YarnContent<'a> {
    Single(&'a str),
    Joined(Vec<&'a str>)
}

impl <'a> YarnContent<'a> {

    pub fn copy_data(&'a self) -> String {
        match self {
            YarnContent::Single(value) => value.to_string(),
            YarnContent::Joined(list) => list.join(""),
        }
    }

    pub fn single(&'a self) -> Option<&'a str> {
        match self {
            YarnContent::Single(value) => Some(value),
            YarnContent::Joined(_) => None,
        }
    }

    pub fn joined(&'a self) -> Option<&Vec<&'a str>> {
        match self {
            YarnContent::Single(_) => None,
            YarnContent::Joined(v) => Some(v),
        }
    }
}

//===================================================================================================================================
//                       Token
//===================================================================================================================================

pub struct YarnToken<'queue> {
    token_type : YarnTokenType,
    line : usize,
    col_start : usize,
    col_end : usize,
    contents : YarnContent<'queue>
}

impl <'a> YarnToken<'a> {
    pub fn token_type(&'a self) -> &YarnTokenType {
        &self.token_type
    }

    pub fn contents(&'a self) -> YarnContent<'a> {
        self.contents()
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
        f.write_str(format!("('{}' | {:?})", self.contents.copy_data(), self.token_type).as_str())
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
    YarnToken { token_type: token_type, line, col_start: col, col_end: 0, contents: YarnContent::Single("") }
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
                contents: YarnContent::Single(""),
            });

            tokens.push_back(YarnToken {
                token_type: YarnTokenType::END_LINE,
                line: cur_line,
                col_start : col_start + col_end,
                col_end : col_start + col_end,
                contents: YarnContent::Single(""),
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
                        contents: YarnContent::Single(s),
                    };
                    tokens.push_back(token);
                } else {

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
            contents: YarnContent::Single(value),
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

    // #[test]
    // fn token_matcher_test() {
    //     let mut tokens = tokenize_yarn_string("===");
    //     let matcher = YarnTokenMacher::new(vec![
    //         YarnTokenType::EQUAL,
    //         YarnTokenType::EQUAL,
    //         YarnTokenType::EQUAL
    //     ], YarnTokenType::END_NODE);
    //     matcher.try_to_match(&mut tokens, 1);

    //     println!("{:?}", tokens);
    // }

    fn test_concat() {
        let s = "test";
        let f = &s[..2];
        let l = &s[2..];
        let word = [f, l].concat();
        println!("{}{}", f, l)
    }
}