/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */
use super::Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut map: HashMap<char, Vec<char>> = HashMap::new();

        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);

        let digits: Vec<char> = digits.chars().collect();
        let mut result: Vec<String> = vec![];
        Solution::dfs(&mut result, &digits, "", &map);
        result
    }

    fn dfs(result: &mut Vec<String>, digits: &[char], temp: &str, map: &HashMap<char, Vec<char>>) {
        if digits.is_empty() {
            result.push(temp.to_string());
            return;
        }
        map.get(&digits[0]).unwrap().iter().for_each(|char| {
            Solution::dfs(result, &digits[1..], &format!("{}{}", temp, char), map)
        });
    }
}
// @lc code=end

#[test]
fn test_0017() {
    let res: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    assert_eq!(Solution::letter_combinations("23".to_string()), res);

    assert_eq!(
        Solution::letter_combinations("".to_string()),
        vec![] as Vec<String>
    );

    let res: Vec<String> = vec!["a", "b", "c"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    assert_eq!(Solution::letter_combinations("2".to_string()), res);
}
