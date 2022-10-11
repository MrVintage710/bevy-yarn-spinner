use std::{collections::VecDeque, fmt::Debug, result};

//==================================================================================================================
//                       Token Queue
//==================================================================================================================

pub struct YarnTokenQueue<'a> {
    source : &'a str,
    tokens : VecDeque<YarnToken<'a>>
}

impl <'a> YarnTokenQueue<'a> {
    pub fn add(&mut self, line : usize, col : usize, offset : usize, size : usize, token_type : YarnTokenType) {
        if offset + size > self.source.len() {
            panic!("The token offset is too big, there wil be an error.")
        }
        
        let token : YarnToken<'a> = YarnToken {
            source: self.source,
            token_type,
            line,
            col,
            token_offset: offset,
            token_length: size,
        };

        self.tokens.push_back(token);
    }

    pub fn re_add(&mut self, token : YarnToken<'a>) {
        self.tokens.push_front(token)
    }

    pub fn merge_tokens(&mut self, start : usize, length : usize, new_type : YarnTokenType) {
        let mut token_1 = self.tokens.get_mut(start).unwrap().clone();
        let token_2 = self.tokens.get(start + length).unwrap().clone();

        token_1.merge(&token_2, new_type);

        for i in (start..=(start+length)).rev() {
            self.tokens.remove(i);
        }

        self.tokens.insert(start, token_1)
    }

    pub fn check(&self, token_type : YarnTokenType) -> bool {
        if let Some(token) = self.tokens.front() {
            token.token_type == token_type
        } else {
            false
        }
    }

    pub fn check_index(&self, index : usize, token_type : YarnTokenType) -> bool {
        if let Some(token) = self.tokens.get(index) {
            token.token_type == token_type
        } else {
            false
        }
    }

    pub fn check_and_pop(&mut self, token_type : YarnTokenType) -> bool {
        if self.check(token_type) {
            self.tokens.pop_front();
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<YarnToken<'a>> {
        self.tokens.pop_front()
    }

    pub fn pop_if_type(&mut self, token_type : YarnTokenType) -> Option<YarnToken> {
        if (self.check(token_type)) {
            self.pop()
        } else {
            None
        }
    } 

    pub fn peek(&self, index : usize) -> Option<&YarnToken<'a>> {
        self.tokens.get(index)
    }

    pub fn peek_only_if_type(&self, index : usize, t : YarnTokenType) -> Option<&YarnToken<'a>> {
        if let Some(token) = self.tokens.get(index) {
            if token.token_type == t {
                return Some(token)
            }
        }

        None
    }

    pub fn peek_line(&self, offset : usize) -> usize {
        if let Some(token) = self.peek(offset) {
            token.line
        } else {
            0
        }
    }

    pub fn peek_col(&self, offset : usize) -> usize {
        if let Some(token) = self.peek(offset) {
            token.col
        } else {
            0
        }
    }

    pub fn remove_leading_spaces(&mut self) {
        while self.check(YarnTokenType::SPACE) {
            self.pop();
        }
    }

    pub fn front(&self) -> Option<&YarnToken<'a>> {
        self.tokens.front()
    }

    pub fn next_non_space_after(&self, offset : usize) -> usize {
        let mut next_index = 1;
        while self.check_index(offset + next_index, YarnTokenType::SPACE) {
            next_index += 1;
        }
        return offset + next_index;
    }
}

impl <'a> Debug for YarnTokenQueue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tokens.fmt(f)
    }
}

//==================================================================================================================
//                       Tokens Type
//==================================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YarnTokenType {
    COLON,
    ARROW,
    WORD,
    TAB,
    MULT,
    ADD,
    SUB,
    EQUAL,
    START_LINE,
    END_LINE,
    START_NODE,
    END_NODE,
    START_COMMAND,
    END_COMMAND,
    SPACE,
    IF,
    ELSE,
    ELSEIF,
    END,
    ENDIF,
    QUOTATION,
    PERIOD,
    BANG,
    HASHTAG,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_SQUARE_BRACKET,
    RIGHT_SQUARE_BRACKET,
    EQUAL_TOO,
    NOT_EQUAL_TOO,
    LESS_THAN,
    LESS_THAN_EQ,
    GREATER_THAN,
    GREATER_THAN_EQ,
    FORWARD_SLASH,
    BACKWARD_SLASH,
    EOF,
    DOLLAR_SIGN
}

const TOKEN_MAP : [(YarnTokenType, &'static str); 22] = [
    (YarnTokenType::COLON, ":"),
    (YarnTokenType::SPACE, " "),
    (YarnTokenType::IF, "if"),
    (YarnTokenType::ELSE, "else"),
    (YarnTokenType::END, "end"),
    (YarnTokenType::QUOTATION, "\""),
    (YarnTokenType::PERIOD, "."),
    (YarnTokenType::LESS_THAN, "<"),
    (YarnTokenType::GREATER_THAN, ">"),
    (YarnTokenType::EQUAL, "="),
    (YarnTokenType::HASHTAG, "#"),
    (YarnTokenType::LEFT_SQUARE_BRACKET, "["),
    (YarnTokenType::RIGHT_SQUARE_BRACKET, "]"),
    (YarnTokenType::LEFT_PAREN, "("),
    (YarnTokenType::RIGHT_PAREN, ")"),
    (YarnTokenType::FORWARD_SLASH, "/"),
    (YarnTokenType::BACKWARD_SLASH, "\\"),
    (YarnTokenType::MULT, "*"),
    (YarnTokenType::ADD, "+"),
    (YarnTokenType::SUB, "-"),
    (YarnTokenType::DOLLAR_SIGN, "$"),
    (YarnTokenType::BANG, "!"),
];

//==================================================================================================================
//                       Token
//==================================================================================================================

#[derive(Clone, Copy)]
pub struct YarnToken<'a> {
    source : &'a str,
    token_type : YarnTokenType,
    line : usize,
    col : usize,
    token_offset : usize,
    token_length : usize
}

impl <'a> YarnToken<'a> {
    pub fn token_type(&'a self) -> &YarnTokenType {
        &self.token_type
    }

    pub fn line(&'a self) -> usize {
        self.line
    }

    pub fn col(&'a self) -> usize {
        self.col
    }

    pub fn content(&self) -> &'a str {
        unsafe {
            self.source.get_unchecked(self.token_offset .. self.token_offset + self.token_length)
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.content().chars().fold(true, |mut acc, c| acc & c.is_numeric())
    }

    fn merge(&mut self, rhs: &YarnToken<'a>, new_type : YarnTokenType)  {
        if !(self.source == rhs.source) {
            panic!("Cannot add tokens that have different sources.")
        }

        (self.token_length, self.token_offset) = {
            let lhs_start = self.token_offset;
            let rhs_start = rhs.token_offset;
            let lhs_end = self.token_offset + self.token_length;
            let rhs_end = rhs.token_offset + rhs.token_length;

            let token_offset = if lhs_start <= rhs_start {
                lhs_start
            } else {
                self.line = rhs.line;
                self.col = rhs.col;
                rhs_start
            };

            let token_legnth = if lhs_end >= rhs_end {
                lhs_end - token_offset
            } else {
                rhs_end - token_offset
            };

            self.token_type = new_type;

            (token_legnth, token_offset)
        }
    }
}

impl <'a> Debug for YarnToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("('{}' | {:?})", self.content(), self.token_type).as_str())
    }
}

//==================================================================================================================
//                       Tokenization
//==================================================================================================================

pub fn tokenize<'a>(source : &'a str) -> YarnTokenQueue<'a> {
    let mut queue = YarnTokenQueue { source, tokens: VecDeque::new() };

    unsafe {
        let mut line_offset = 0;
        for (line_number, line) in source.lines().enumerate() {
            queue.add(line_number, 0, line_offset, 0, YarnTokenType::START_LINE);
            let mut anchor = 0;
            let mut offset = 0;

            while anchor + offset <= line.len() {
                let segment = line.get_unchecked(anchor .. anchor + offset);
                let mut token_matched = false;
                for (token_type, string_match) in TOKEN_MAP {
                    if !token_matched {
                        if segment.contains(string_match) {
                            if segment == string_match {
                                queue.add(line_number, anchor, line_offset + anchor, offset, token_type)
                            } else {
                                let pattern_offset = segment.find(string_match).unwrap();
                                queue.add(line_number, anchor, line_offset + anchor, pattern_offset, YarnTokenType::WORD);
                                queue.add(line_number, anchor + pattern_offset, line_offset + anchor + pattern_offset, string_match.len(), token_type);
                            }
                            token_matched = true;
                        }
                    }
                }

                if anchor + offset == line.len() && !segment.is_empty() && !token_matched {
                    queue.add(line_number, anchor, line_offset + anchor, offset, YarnTokenType::WORD);
                }

                if token_matched {
                    anchor += offset;
                    offset = 0;
                } else {
                    offset += 1;
                }
            }

            queue.add(line_number, line.len(), line_offset + line.len() - 1, 0, YarnTokenType::END_LINE);
            line_offset += line.len()
        }

        queue.add(0, 0, line_offset, 0, YarnTokenType::EOF)
    }

    match_tokens(&mut queue);

    queue
}

macro_rules! proccess_match {
    ($queue:ident, $index:expr,  $result:ident => $($token_type:ident),*) => {
        {
            let mut matching = true;
            let mut offset : usize = 0;
            $(
                matching &= $queue.check_index($index + offset, $token_type);
                offset += 1;
            )*

            if matching {
                $queue.merge_tokens($index, offset - 1, $result)
            }
        }
    };
}

fn match_tokens(queue : &mut YarnTokenQueue) {
    use YarnTokenType::*;

    for index in 0..queue.tokens.len() {
        if let Some(current_token) = queue.tokens.get(index) {

            //Command start and close
            proccess_match!(queue, index, START_COMMAND => LESS_THAN, LESS_THAN);
            proccess_match!(queue, index, END_COMMAND => GREATER_THAN, GREATER_THAN);

            //Node open and close
            proccess_match!(queue, index, START_NODE => SUB, SUB, SUB);
            proccess_match!(queue, index, END_NODE => EQUAL, EQUAL, EQUAL);

            // == and !=
            proccess_match!(queue, index, EQUAL_TOO => EQUAL, EQUAL);
            proccess_match!(queue, index, NOT_EQUAL_TOO => BANG, EQUAL);

            //Arrow
            proccess_match!(queue, index, ARROW => SUB, GREATER_THAN);

            //LTE, GTE
            proccess_match!(queue, index, GREATER_THAN_EQ => GREATER_THAN, EQUAL);
            proccess_match!(queue, index, LESS_THAN_EQ => LESS_THAN, EQUAL);

            //elseif, endif
            proccess_match!(queue, index, ELSEIF => ELSE, IF);
            proccess_match!(queue, index, ENDIF => END, IF);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::YarnTokenType::*;

    macro_rules! has_tokens{
        ($tokens:ident, $start_index:expr, $($token:ident),*) => {
            {
                let mut offset : usize = $start_index;
                $(
                    assert!($tokens.check_index(offset, $token));
                    offset += 1;
                )*
            }
        }
    }

    #[test]
    fn test_matched_tokens() {
        let mut q = tokenize("<<>>");
        has_tokens!(q, 1, START_COMMAND, END_COMMAND);
    }

    #[test]
    fn test_basic_tokens() {
        let mut q = tokenize("<>");
        has_tokens!(q, 1, LESS_THAN, GREATER_THAN);
    }

    #[test]
    fn test_concat() {
        let source = "This is test source code. -> --- === ";
        let mut token_1 = YarnToken {
            source,
            token_type: WORD,
            line: 0,
            col: 0,
            token_offset: 0,
            token_length: 4,
        };

        let mut token_2 = YarnToken {
            source,
            token_type: WORD,
            line: 0,
            col: 8,
            token_offset: 8,
            token_length: 4,
        };

        token_2.merge(&token_1, WORD);

        assert_eq!(token_2.content(), "This is test")
    }
}