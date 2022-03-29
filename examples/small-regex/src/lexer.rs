#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Token
{
    Char(char),
    LParen,
    RParen,
    Star,
    Pipe
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TokenizeError
{
    Unexpected(char),
}

pub struct TokenIter<It>
{
    inner_it: It,
    had_err: bool,
}

impl<It> TokenIter<It>
{
    pub fn new(inner_it: It) -> Self
    { TokenIter{ inner_it, had_err: false } }
}

impl<It> Iterator for TokenIter<It>
    where It: Iterator<Item = char>
{
    type Item = Result<Token, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_err {
            return None;
        }

        use Token::*;
        use TokenizeError::*;
        while let Some(ch) = self.inner_it.next() {
            let curr_tok = match ch {
                '(' => Ok(LParen),
                ')' => Ok(RParen),
                '*' => Ok(Star),
                '|' => Ok(Pipe),
                ch if ch.is_alphanumeric() => Ok(Char(ch)),
                other if other.is_whitespace() => continue,
                other => {
                    self.had_err = true;
                    Err(Unexpected(other))
                }
            };
            return Some(curr_tok);
        }
        return None
    }
}


// just to have a fancy way to write '.tokenize()' for any iter
pub trait Tokenize
{
    type CharIt;

    fn tokenize(self) -> TokenIter<Self::CharIt>;
}

impl<It> Tokenize for It
    where It: Iterator<Item = char>
{
    type CharIt = Self;

    fn tokenize(self) -> TokenIter<Self::CharIt> {
        TokenIter::new(self)
    }
}
