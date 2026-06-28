impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<u8> = s.into_bytes();
        let mut t_chars: Vec<u8> = t.into_bytes();

        s_chars.sort();
        t_chars.sort();

        s_chars == t_chars
    }
}
