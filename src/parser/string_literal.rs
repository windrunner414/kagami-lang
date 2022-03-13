use anyhow::{anyhow, Context, Result};
use memchr::memchr;
use std::borrow::Cow;
use std::cmp::min;
use std::str;

/// Allowed escape characters: \n, \r, \t, \\, \', \", \uXXXX, \u{X{1, 6}}
pub fn unescape(string: &str) -> Result<Cow<str>> {
    fn unescape_unicode(string: &str) -> Result<(char, usize)> {
        // \uXXXX or \u{X{1, 6}}
        let len = string.len();
        if len <= 3 {
            return Err(anyhow!("Unexpected EOF"));
        }

        let bytes = string.as_bytes();
        match unsafe { *bytes.get_unchecked(2) } {
            b'{' => {
                for i in 3..min(len, 10) {
                    if unsafe { *bytes.get_unchecked(i) } == b'}' {
                        if i == 3 {
                            return Err(anyhow!("Unicode escape must have at least 1 hex digit"));
                        }

                        return Ok((
                            char::try_from(u32::from_str_radix(
                                unsafe { str::from_utf8_unchecked(&bytes[3..i]) },
                                16,
                            )?)?,
                            i + 1,
                        ));
                    }
                }

                Err(anyhow!("Unicode escape must have at most 6 hex digits"))
            }
            _ => {
                if len < 6 {
                    Err(anyhow!("Unexpected EOF"))
                } else {
                    let i = unsafe { (bytes.as_ptr().add(2) as *const u32).read_unaligned() };
                    if i & 0x80_80_80_80u32 == 0 {
                        Ok((
                            char::try_from(u32::from_str_radix(
                                unsafe { str::from_utf8_unchecked(&bytes[2..6]) },
                                16,
                            )?)?,
                            6,
                        ))
                    } else {
                        Err(anyhow!("Invalid hex digit"))
                    }
                }
            }
        }
    }

    fn push_unescape_one(dst: &mut String, string: &str) -> Result<usize> {
        if string.len() <= 1 {
            return Err(anyhow!("unexpected EOF"));
        }

        let c = unsafe { *string.as_bytes().get_unchecked(1) };

        dst.push(match c {
            b'"' | b'\'' | b'\\' => c as char,
            b'n' => '\n',
            b'r' => '\r',
            b't' => '\t',
            b'u' => {
                let (u, len) = unescape_unicode(string)?;
                dst.push(u);
                return Ok(len);
            }
            _ => return Err(anyhow!("Invalid escape character '{}'", c as char)),
        });

        Ok(2)
    }

    match memchr(b'\\', string.as_bytes()) {
        Some(i) => {
            let origin_len = string.len();
            let mut u_string = String::with_capacity(origin_len);
            u_string.push_str(&string[..i]);
            let push_len = push_unescape_one(&mut u_string, &string[i..])
                .with_context(|| anyhow!("Invalid escape sequence at {}", i))?;

            let mut start = i + push_len;
            while start < origin_len {
                let search_str = &string[start..];

                match memchr(b'\\', search_str.as_bytes()) {
                    Some(i) => {
                        start += i;

                        u_string.push_str(&search_str[..i]);
                        let push_len = push_unescape_one(&mut u_string, &search_str[i..])
                            .with_context(|| anyhow!("Invalid escape sequence at {}", start))?;

                        start += push_len;
                    }
                    None => {
                        u_string.push_str(search_str);
                        break;
                    }
                }
            }

            Ok(u_string.into())
        }
        None => Ok(string.into()),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_s_no_escape(b: &mut Bencher) {
        b.iter(|| unescape(r"fnaeuoifhda"));
    }

    #[bench]
    fn bench_s(b: &mut Bencher) {
        b.iter(|| unescape(r"fnaeuo\nifh\tda"));
    }

    #[bench]
    fn bench_l_no_escape(b: &mut Bencher) {
        b.iter(|| unescape(r"fnaeuoiarlgijfigexbfasdaojknm23iojuf8asfoiucnmlaw4ijasefdasfhdaaefdasfawefvzxcvzxcvase"));
    }

    #[bench]
    fn bench_l(b: &mut Bencher) {
        b.iter(|| unescape(r"fnaeuoiarl\ngijfigexbfasdaojknm\u{41}23iojuf8asfoi\tucnm\\law4ijasefdasfhdaaefda\u0041sfawefvzxcvzxcvase"));
    }

    #[bench]
    fn bench_unicode_escape(b: &mut Bencher) {
        b.iter(|| unescape(r"\u6b64\u65b9\u5c0f\u955c\u529e\u4e8b\u4e2d\u6b64\u65b9\u5c0f\u955c\u529e\u4e8b\u4e2d"));
    }
}
