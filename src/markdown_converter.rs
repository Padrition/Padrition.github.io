use std::collections::HashMap;

pub mod markdown_conv {
    use std::{borrow::BorrowMut, collections::HashMap};

    const MAX_HEADING_SIZE: usize = 6;

    #[derive(PartialEq, Debug)]
    enum TextEmphasis {
        Bold,
        Italic,
        BoldItalic,
    }

    #[derive(PartialEq, Debug)]
    enum ListType {
        Ordered,
        Unordered,
    }

    #[derive(PartialEq, Debug)]
    enum TokenType {
        Text,
        WhiteSpace,
        Header,
        Paragraph,
        LineBreak,
        Emphasis(TextEmphasis),
        BlockQuote,
        List(ListType),
        Code,
        Hr,
        Link,
        Image,
        Escape,
    }

    #[derive(PartialEq, Debug)]
    struct Token {
        token_type: TokenType,
        content: String,
    }

    fn tokenize(markdown: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        for c in markdown.chars() {
            match c {
                '#' => match tokens.last_mut() {
                    Some(token) => match token.token_type {
                        TokenType::Header => {
                            if token.content.len() == 5 {
                                token.token_type = TokenType::Paragraph;
                            }
                            token.content += "#";
                        }
                        TokenType::WhiteSpace => {
                            tokens.push(Token {
                                token_type: TokenType::Header,
                                content: "#".to_string(),
                            });
                        }
                        TokenType::Paragraph | TokenType::Text => {
                            token.content += "#";
                        }
                        TokenType::LineBreak => todo!(),
                        TokenType::Emphasis(_) => todo!(),
                        TokenType::BlockQuote => todo!(),
                        TokenType::List(_) => todo!(),
                        TokenType::Code => todo!(),
                        TokenType::Hr => todo!(),
                        TokenType::Link => todo!(),
                        TokenType::Image => todo!(),
                        TokenType::Escape => todo!(),
                    },
                    None => tokens.push(Token {
                        token_type: TokenType::Header,
                        content: "#".to_string(),
                    }),
                },
                ' ' => {
                    let s = c.clone();
                    match tokens.last_mut() {
                        Some(token) => match token.token_type {
                            TokenType::Text => todo!(),
                            TokenType::WhiteSpace => todo!(),
                            TokenType::Header => {
                                match token
                                    .content
                                    .chars()
                                    .last()
                                    .expect("Expected to have chars in a tokne content.")
                                {
                                    '#' => token.content += &s.to_string(),
                                    ' ' => tokens.push(Token{token_type: TokenType::Text, content: s.to_string()}),
                                    _ => todo!("Sould throw a panic?")
                                }
                            }
                            TokenType::Paragraph => todo!(),
                            TokenType::LineBreak => todo!(),
                            TokenType::Emphasis(_) => todo!(),
                            TokenType::BlockQuote => todo!(),
                            TokenType::List(_) => todo!(),
                            TokenType::Code => todo!(),
                            TokenType::Hr => todo!(),
                            TokenType::Link => todo!(),
                            TokenType::Image => todo!(),
                            TokenType::Escape => todo!(),
                        },
                        None => tokens.push(Token {
                            token_type: TokenType::WhiteSpace,
                            content: " ".to_string(),
                        }),
                    }
                }
                _ => {
                    let s = c.clone();
                    match tokens.last_mut() {
                        Some(token) => match token.token_type {
                            TokenType::Text => token.content += &s.to_string(),
                            TokenType::WhiteSpace => tokens.push(Token {
                                token_type: TokenType::Paragraph,
                                content: s.to_string(),
                            }),
                            TokenType::Header => {
                                match token
                                    .content
                                    .chars()
                                    .last()
                                    .expect("Expected to have chars in a token content.")
                                {
                                    ' ' => tokens.push(Token {
                                        token_type: TokenType::Text,
                                        content: c.to_string(),
                                    }),
                                    '#' => {
                                        token.token_type = TokenType::Paragraph;
                                        token.content += &s.to_string();
                                    }
                                    _ => todo!("Should it panic?"),
                                }
                            }
                            TokenType::Paragraph => todo!(),
                            TokenType::LineBreak => todo!(),
                            TokenType::Emphasis(_) => todo!(),
                            TokenType::BlockQuote => todo!(),
                            TokenType::List(_) => todo!(),
                            TokenType::Code => todo!(),
                            TokenType::Hr => todo!(),
                            TokenType::Link => todo!(),
                            TokenType::Image => todo!(),
                            TokenType::Escape => todo!(),
                        },
                        None => tokens.push(Token {
                            token_type: TokenType::Paragraph,
                            content: c.clone().to_string(),
                        }),
                    }
                }
            }
        }

        tokens
    }

    fn convert_heading(markdown: String) -> Option<String> {
        let whitespace_pos = markdown.find(' ');
        match whitespace_pos {
            Some(pos) => {
                let (size, text) = markdown.split_at(pos);
                let size = if size.len() <= MAX_HEADING_SIZE {
                    size.len()
                } else {
                    return None;
                };
                let text = text.strip_prefix(' ');

                text.map(|t| format!("<h{size}>{t}</h{size}>"))
            }
            None => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenization() {
            assert_eq!(
                tokenize("# Header"),
                vec![
                    Token {
                        token_type: TokenType::Header,
                        content: "# ".to_string()
                    },
                    Token {
                        token_type: TokenType::Text,
                        content: "Header".to_string()
                    }
                ]
            )
        }

        #[test]
        fn test_heading_convertion() {
            let markdown = String::from("# Test");
            assert_eq!(
                convert_heading(markdown),
                Some(String::from("<h1>Test</h1>"))
            );

            let markdown = String::from("###### Test");
            assert_eq!(
                convert_heading(markdown),
                Some(String::from("<h6>Test</h6>"))
            );

            let markdown = String::from("######Test");
            assert_eq!(convert_heading(markdown), None);
        }
    }
}
