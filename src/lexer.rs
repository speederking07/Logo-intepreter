pub mod lexer {
    use std::{iter::Peekable};

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token{
        // Arithmetic
        Plus,
        Minus,
        Multiply,
        Divide,
        Modulo,

        // Consts
        Var(String),
        Word(String),
        Num(f64),
        Function(String),

        LBracket,
        RBracket,
        LParenth,
        RParenth,

        Less,
        LessEq,
        Greater,
        GreaterEq,
        Equal,

        To,
        End
    }
    #[derive(Debug)]
    pub struct Lexer<A: Iterator<Item = char>> {
        iter: Peekable<A>
    }

    fn special (c: char) -> Option<Token> {
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '%' => Some(Token::Modulo),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '<' => Some(Token::Less),
            '>' => Some(Token::Greater),
            '=' => Some(Token::Equal),
            '(' => Some(Token::LParenth),
            ')' => Some(Token::RParenth),
            _ => None
        }
    }

    impl<A: Iterator<Item = char>> Lexer<A> {
        pub fn new(x: A) -> Lexer<A> {
            Lexer { iter: x.peekable() }
        }
    }

    impl<A: Iterator<Item = char>> Iterator for Lexer<A> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.iter.next();
            if current.is_none() {
                return None;
            } else {
                let c = current.unwrap();
                let special_token = special(c);
                if special_token.is_some(){
                    let s = special_token.unwrap();
                    if s == Token::Less || s == Token::Greater {
                        if self.iter.peek().filter(|x| **x == '=').is_some() {
                            self.iter.next();
                            return match s {
                                Token::Less => Some(Token::LessEq),
                                Token::Greater => Some(Token::GreaterEq),
                                _ => Some(s)
                            }
                        } else {
                            return Some(s);
                        }
                    } else {
                        return Some(s);
                    }
                }
                else if c == '"' {
                    let mut word = vec![];
                    while let Some(x) = self.iter.next() {
                        if x == '"' || x.is_whitespace() {
                            break;
                        } else {
                            word.push(x)
                        }
                    } 
                    return Some(Token::Word(word.iter().collect()));
                }
                else if c == ':' {
                    let mut word = vec![];
                    while let Some(x) = self.iter.peek() {
                        if x.is_whitespace() || special(*x).is_some() {
                            break;
                        } else {
                            word.push(self.iter.next().unwrap())
                        }
                    } 
                    return Some(Token::Var(word.iter().collect()));
                }
                else if c.is_numeric() {
                    let mut word = vec![c];
                    while let Some(x) = self.iter.peek() {
                        if x.is_numeric() || *x == '.' {
                            word.push(self.iter.next().unwrap())
                        } else {
                            break;
                        }
                    }
                    return match word.iter().collect::<String>().parse() {
                        Ok(n) => Some(Token::Num(n)),
                        Err(_) => None 
                    }
                } else if c.is_alphabetic() {
                    let mut word = vec![c];
                    while let Some(x) = self.iter.peek() {
                        if x.is_alphabetic() {
                            word.push(self.iter.next().unwrap())
                        } else {
                            break;
                        }
                    }
                    return match word.iter().collect::<String>().as_str() {
                        "TO" => Some(Token::To),
                        "END" => Some(Token::End),
                        s => Some(Token::Function(String::from(s)))
                    }
                } else if c.is_whitespace() {
                    return self.next();
                }
            }
            None
        }
    }

    #[test]
    fn test1() {
        let str = "TO star\n
            repeat 5 [ fd 100 rt 144 ]\n
            END\n
            clearscreen\n
            star";
        assert_eq!(Lexer::new(str.chars()).collect::<Vec<Token>>(), vec![Token::To, Token::Function(String::from("star")),
            Token::Function(String::from("repeat")),Token::Num(5.0), Token::LBracket, Token::Function(String::from("fd")),
            Token::Num(100.0), Token::Function(String::from("rt")), Token::Num(144.0), Token::RBracket, Token::End,
            Token::Function(String::from("clearscreen")), Token::Function(String::from("star"))])
    }
}