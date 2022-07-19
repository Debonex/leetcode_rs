/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达&式匹配
 */

use super::Solution;
// @lc code=start
#[derive(Debug)]
struct Token {
    char: char,
    is_star: bool,
}

fn convert_to_tokens(p: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    p.chars().for_each(|c| match c {
        '*' => {
            tokens.last_mut().unwrap().is_star = true;
        }
        _ => {
            tokens.push(Token {
                char: c,
                is_star: false,
            });
        }
    });
    tokens
}

fn eq(char: &char, token: &Token) -> bool {
    char == &token.char || token.char == '.'
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let tokens = convert_to_tokens(&p);

        let (chars_len, tokens_len) = (chars.len(), tokens.len());

        struct Dfs<'a> {
            f: &'a dyn Fn(&Dfs, usize, usize) -> bool,
        }

        let dfs = Dfs {
            f: &|dfs, chars_idx, tokens_idx| -> bool {
                if chars_idx == chars_len {
                    if tokens_idx == tokens_len {
                        true
                    } else {
                        tokens[tokens_idx].is_star && (dfs.f)(dfs, chars_idx, tokens_idx + 1)
                    }
                } else if tokens_idx == tokens_len {
                    false
                } else {
                    let token = &tokens[tokens_idx];
                    let char = &chars[chars_idx];
                    match token.is_star {
                        true => match eq(char, token) {
                            true => {
                                (dfs.f)(dfs, chars_idx + 1, tokens_idx)
                                    || (dfs.f)(dfs, chars_idx + 1, tokens_idx + 1)
                                    || (dfs.f)(dfs, chars_idx, tokens_idx + 1)
                            }
                            false => (dfs.f)(dfs, chars_idx, tokens_idx + 1),
                        },
                        false => match eq(char, token) {
                            true => (dfs.f)(dfs, chars_idx + 1, tokens_idx + 1),
                            false => false,
                        },
                    }
                }
            },
        };
        (dfs.f)(&dfs, 0, 0)
    }
}
// @lc code=end

#[test]
fn test_0010() {
    assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    assert!(!Solution::is_match("ab".to_string(), "a*a*".to_string()));
    assert!(!Solution::is_match("ab".to_string(), ".*c".to_string()));
    assert!(!Solution::is_match("a".to_string(), "aa".to_string()));
    assert!(Solution::is_match("aaa".to_string(), "ab*ac*a".to_string()));
    assert!(!Solution::is_match("bb".to_string(), "..*c".to_string()));
    assert!(!Solution::is_match(
        "bbabacccbcbbcaaab".to_string(),
        "a*b*a*a*c*aa*c*bc*".to_string()
    ));
    assert!(Solution::is_match(
        "mississippi".to_string(),
        "mis*is*ip*.".to_string()
    ));
    assert!(!Solution::is_match(
        "mississippi".to_string(),
        "mis*is*p*.".to_string()
    ));
    assert!(Solution::is_match("bbba".to_string(), ".*a*a".to_string()));
}
