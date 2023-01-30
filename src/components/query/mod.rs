pub struct QueryLexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_function(name: &str) -> bool {
    match name {
        "to_number" => true,
        "to_string" => true,
        "lowercase" => true,
        "uppercase" => true,
        "replace" => true,
        "len" => true,
        "floor" => true,
        "trim" => true,
        "to_integer" => true,
        "to_float" => true,
        _ => false,
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '.'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn count_asterix(input: &Vec<char>) -> usize {
    let mut counter = 0;
    let mut last_char = '\0';
    for char in input {
        if *char == '*' && last_char != '\\' {
            counter += 1;
        }
        last_char = *char;
    }
    counter
}

fn transform_escape_char(ch: char) -> Result<char, ()> {
    match ch {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        '0' => Ok('\0'),
        '*' => Err(()),
        _ => Ok(ch),
    }
}

impl QueryLexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let ch = self.ch;
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                return;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut QueryLexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && (is_letter(l.ch) || is_digit(l.ch)) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };
        let read_literal_string = |l: &mut QueryLexer| -> Vec<char> {
            let mut is_escape = false;
            let mut to_ret = Vec::with_capacity(32);
            while l.position < l.input.len() && !(l.ch == '\'' && !is_escape) {
                if l.ch == '\\' {
                    if is_escape {
                        to_ret.push('\\');
                    }
                    is_escape = !is_escape;
                } else {
                    if is_escape {
                        match transform_escape_char(l.ch) {
                            Ok(ch) => to_ret.push(ch),
                            Err(_) => {
                                to_ret.push(l.ch);
                            }
                        };
                    } else {
                        to_ret.push(l.ch);
                    }
                    is_escape = false;
                }
                l.read_char();
            }
            to_ret
        };

        let read_string = |l: &mut QueryLexer| -> Vec<char> {
            let mut is_escape = false;
            let mut to_ret = Vec::with_capacity(32);
            while l.position < l.input.len() && !(l.ch == '"' && !is_escape) {
                if l.ch == '\\' {
                    if is_escape {
                        to_ret.push('\\');
                    }
                    is_escape = !is_escape;
                } else {
                    if is_escape {
                        match transform_escape_char(l.ch) {
                            Ok(ch) => to_ret.push(ch),
                            Err(_) => {
                                // The \\ character is used
                                to_ret.push('\\');
                                to_ret.push(l.ch);
                            }
                        };
                    } else {
                        to_ret.push(l.ch);
                    }
                    is_escape = false;
                }
                l.read_char();
            }
            to_ret
        };

        let read_number = |l: &mut QueryLexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                tok = Token::ASSIGN;
            }
            '|' => {
                tok = Token::PIPE;
            }
            '+' => {
                tok = Token::PLUS(self.ch);
            }
            '-' => {
                tok = Token::MINUS(self.ch);
            }
            '!' => {
                tok = Token::BANG(self.ch);
            }
            '/' => {
                tok = Token::SLASH(self.ch);
            }
            '*' => {
                tok = Token::ASTERISK(self.ch);
            }
            '<' => {
                tok = Token::LT(self.ch);
            }
            '>' => {
                tok = Token::GT(self.ch);
            }
            ';' => {
                tok = Token::SEMICOLON(self.ch);
            }
            '(' => {
                tok = Token::LPAREN(self.ch);
            }
            ')' => {
                tok = Token::RPAREN(self.ch);
            }
            ',' => {
                tok = Token::COMMA(self.ch);
            }
            '{' => {
                tok = Token::LBRACE(self.ch);
            }
            '}' => {
                tok = Token::RBRACE(self.ch);
            }
            '0' => {
                tok = Token::EOF;
            }
            '\'' => {
                self.read_char();
                let data = read_literal_string(self);
                tok = Token::String(data.iter().collect())
            }
            '"' => {
                self.read_char();
                let data = read_string(self);
                if data.len() > 1 {
                    let n_asterix = count_asterix(&data);
                    //Test if can be a start_with, contains, ends_with or like
                    if n_asterix > 2 {
                        tok = Token::Like(data.iter().collect())
                    } else {
                        let starts_astx = data[0] == '*';
                        let ends_astx = data[data.len() - 1] == '*';
                        if starts_astx && ends_astx {
                            tok = Token::Contains(data.iter().filter(|c| *c != &'*').collect())
                        } else if starts_astx {
                            tok = Token::StartsWith(data.iter().filter(|c| *c != &'*').collect())
                        } else if ends_astx {
                            tok = Token::EndsWith(data.iter().filter(|c| *c != &'*').collect())
                        } else {
                            if n_asterix == 0 {
                                tok = Token::String(data.iter().collect())
                            } else {
                                tok = Token::Like(data.iter().collect())
                            }
                        }
                    }
                } else {
                    tok = Token::String(data.iter().collect())
                }
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        }
                        Err(_err) => {
                            return Token::FIELD(ident.into_iter().collect());
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return Token::INT(ident.into_iter().collect());
                } else {
                    return Token::ILLEGAL;
                }
            }
        }
        self.read_char();
        tok
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    FIELD(String),
    INT(String),
    ASSIGN,
    PIPE,
    PLUS(char),
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION(String),
    TRUE,
    FALSE,
    AND,
    OR,
    NOT,
    RETURN,
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),
    LT(char),
    GT(char),
    FILTER,
    FIELDS,
    AS,
    String(String),
    RegexField(String),
    StartsWith(String),
    EndsWith(String),
    Like(String),
    Contains(String),
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "AND" => Ok(Token::AND),
        "OR" => Ok(Token::OR),
        "NOT" => Ok(Token::NOT),
        "filter" => Ok(Token::FILTER),
        "fields" => Ok(Token::FIELDS),
        "as" => Ok(Token::AS),
        _ => {
            if is_function(&identifier) {
                return Ok(Token::FUNCTION(identifier));
            }
            Err(String::from("Not a keyword"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse_the_query() {
        let input = String::from("filter field_name2=\"*something\" | fields os.actor_process as osap | filter to_string(osap,'something') = \"12345\"");
        let mut l = QueryLexer::new(input.chars().collect());
        l.read_char();
        loop {
            let token = l.next_token();
            if token == Token::ILLEGAL {
                break;
            }
            if token == Token::EOF {
                break;
            } else {
                println!("{:?}", token);
            }
        }
        println!("{} {} {}", char::from(l.ch), l.position, l.read_position);
    }
}
