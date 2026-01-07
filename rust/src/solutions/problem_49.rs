pub struct Solution {}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::<String, Vec<String>>::new();
    for s in strs {
        let ss = s.clone();
        let mut str = s.into_bytes();
        str.sort_unstable();
        let s = String::from_utf8(str).unwrap();
        map.entry(s).or_default().push(ss);
    }
    map.into_values().collect()
}
