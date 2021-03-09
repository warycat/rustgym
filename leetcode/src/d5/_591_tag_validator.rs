struct Solution;

struct Parser {
    code: Vec<char>,
    index: usize,
    n: usize,
}

impl Parser {
    fn new(code: String) -> Self {
        let code: Vec<char> = code.chars().collect();
        let index = 0;
        let n = code.len();
        Parser { code, index, n }
    }

    fn parse_code(&mut self) -> Result<(), String> {
        self.parse_closed_tag()?;
        if self.peek().is_none() {
            Ok(())
        } else {
            Err("parse_code".to_string())
        }
    }

    fn parse_closed_tag(&mut self) -> Result<(), String> {
        let start_tag = self.parse_start_tag()?;
        self.parse_content()?;
        let end_tag = self.parse_end_tag()?;
        if start_tag != end_tag {
            return Err(start_tag);
        }
        Ok(())
    }

    fn parse_content(&mut self) -> Result<(), String> {
        while let Some(c) = self.peek() {
            if c != '<' {
                self.next();
            } else {
                if let Some(s) = self.peek_more(2) {
                    if s == "<!" {
                        self.parse_cdata()?;
                        continue;
                    }
                    if s == "</" {
                        break;
                    }
                    self.parse_closed_tag()?
                } else {
                    return Err("<".to_string());
                }
            }
        }
        Ok(())
    }

    fn parse_cdata(&mut self) -> Result<(), String> {
        if let Some(s) = self.next_more(9) {
            if s != "<![CDATA[" {
                Err(s)
            } else {
                while let Some(s) = self.peek_more(3) {
                    if s == "]]>" {
                        break;
                    } else {
                        self.next();
                    }
                }
                if let Some(s) = self.next_more(3) {
                    if s == "]]>" {
                        Ok(())
                    } else {
                        Err(s)
                    }
                } else {
                    Err("]]>".to_string())
                }
            }
        } else {
            Err("<![CDATA[".to_string())
        }
    }

    fn parse_start_tag(&mut self) -> Result<String, String> {
        if let Some(c) = self.next() {
            if c != '<' {
                Err("<".to_string())
            } else {
                let mut tag = "".to_string();
                while let Some(c) = self.next() {
                    match c {
                        '>' => {
                            break;
                        }
                        'A'..='Z' => {
                            tag.push(c);
                        }
                        _ => return Err("<".to_string()),
                    }
                }
                if !(1..=9).contains(&tag.len()) {
                    return Err(tag);
                }
                Ok(tag)
            }
        } else {
            Err("<".to_string())
        }
    }

    fn parse_end_tag(&mut self) -> Result<String, String> {
        if let Some(s) = self.next_more(2) {
            if s != "</" {
                Err(s)
            } else {
                let mut tag = "".to_string();
                while let Some(c) = self.next() {
                    if c == '>' {
                        break;
                    } else {
                        tag.push(c);
                    }
                }
                Ok(tag)
            }
        } else {
            Err("<".to_string())
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index < self.n {
            Some(self.code[self.index])
        } else {
            None
        }
    }
    fn peek_more(&self, size: usize) -> Option<String> {
        if self.index + size < self.n {
            Some(self.code[self.index..self.index + size].iter().collect())
        } else {
            None
        }
    }
    fn next(&mut self) -> Option<char> {
        if self.index < self.n {
            let c = self.code[self.index];
            self.index += 1;
            Some(c)
        } else {
            None
        }
    }
    fn next_more(&mut self, size: usize) -> Option<String> {
        if self.index + size <= self.n {
            let s = self.code[self.index..self.index + size].iter().collect();
            self.index += size;
            Some(s)
        } else {
            None
        }
    }
}

impl Solution {
    fn is_valid(code: String) -> bool {
        let mut parser = Parser::new(code);
        parser.parse_code().is_ok()
    }
}

#[test]
fn test() {
    let code = "<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string();
    let res = true;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string();
    let res = true;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<A>  <B> </A>   </B>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV>  div tag is not closed  <DIV>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV>  unmatched <  </DIV>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
        .to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<A></A><B></B>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<A><A>/A></A></A>".to_string();
    let res = true;
    assert_eq!(Solution::is_valid(code), res);
    let code = "<A<></A<>".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(code), res);
}
