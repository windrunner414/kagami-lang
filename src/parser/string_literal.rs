use anyhow::{anyhow, Context, Result};
use std::borrow::Cow;
use std::str::CharIndices;

/// Allowed escape characters: \n, \r, \t, \\, \', \", \u, \u{}
pub fn unescape(string: &str) -> Result<Cow<str>> {
    #[inline]
    fn unescape_unicode(chars: &mut CharIndices) -> Result<char> {
        let mut u_str = String::with_capacity(6);
        match chars.next() {
            None => return Err(anyhow!("unexpected EOF")),
            Some((_, '{')) => loop {
                match chars.next() {
                    None => return Err(anyhow!("unexpected EOF")),
                    Some((_, '}')) => {
                        if u_str.is_empty() {
                            return Err(anyhow!("Unicode escape must have at least 1 hex digit"));
                        }
                        break;
                    }
                    Some((_, u_c)) => {
                        u_str.push(u_c);
                        if u_str.len() > 6 {
                            return Err(anyhow!("Unicode escape must have at most 6 hex digits"));
                        }
                    }
                }
            },
            Some((_, u_c)) => {
                u_str.push(u_c);
                for _ in 0..3 {
                    match chars.next() {
                        None => return Err(anyhow!("unexpected EOF")),
                        Some((_, u_c)) => u_str.push(u_c),
                    }
                }
            }
        }
        Ok(char::try_from(u32::from_str_radix(&u_str, 16)?)?)
    }

    #[inline]
    fn push_unescape(dst: &mut String, string: &str, o_offset: usize) -> Result<()> {
        let mut chars = string.char_indices();

        while let Some((_, c1)) = chars.next() {
            if c1 != '\\' {
                dst.push(c1);
            } else {
                match chars.next() {
                    None => return Err(anyhow!("unexpected EOF")),
                    Some((i, c2)) => {
                        dst.push(match c2 {
                            '"' | '\'' | '\\' => c2,
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => unescape_unicode(&mut chars).with_context(|| {
                                anyhow!("Invalid Unicode escape sequence from {}", o_offset + i)
                            })?,
                            _ => {
                                return Err(anyhow!(
                                    "Invalid escape character '{}' at {}",
                                    c2,
                                    o_offset + i,
                                ))
                            }
                        });
                    }
                }
            }
        }

        Ok(())
    }

    match string.find('\\') {
        Some(i) => {
            let mut u_string = String::with_capacity(string.len() - 1);
            u_string.push_str(&string[0..i]);
            push_unescape(&mut u_string, &string[i..], i)
                .with_context(|| "Invalid unescape sequence")?;
            Ok(u_string.into())
        }
        None => Ok(string.into()),
    }
}
