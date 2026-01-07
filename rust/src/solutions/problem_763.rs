pub fn partition_labels(s: String) -> Vec<i32> {
    let bs = s.as_bytes();
    let mut char_ranges = vec![[-1, -1]; 26];
    for (j, c) in bs.iter().enumerate() {
        let i = (c - b'a') as usize;
        if char_ranges[i][0] == -1 {
            char_ranges[i][0] = j as i16;
        }
        char_ranges[i][1] = j as i16;
    }
    let mut ranges = Vec::<[i16; 2]>::with_capacity(26);
    for r in char_ranges {
        if r[0] != -1 {
            ranges.push(r);
        }
    }
    ranges.sort_unstable_by(|a, b| {
        if a[0] == b[0] {
            return a[1].cmp(&b[1]);
        }
        a[0].cmp(&b[0])
    });
    let mut ans = Vec::<i32>::new();
    let (mut start, mut end) = (-1, -1);
    for r in ranges {
        if start == -1 {
            start = r[0];
            end = r[1];
        } else {
            if r[0] <= end {
                end = end.max(r[1]);
            } else {
                ans.push((end - start) as i32 + 1);
                start = r[0];
                end = r[1];
            }
        }
    }
    if start != -1 {
        ans.push((end - start) as i32 + 1);
    }
    ans
}
