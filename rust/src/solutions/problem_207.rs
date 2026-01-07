pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = num_courses as usize;
    let mut pre_courses = vec![std::collections::HashSet::<usize>::new(); n];
    let mut next_courses = vec![std::collections::HashSet::<usize>::new(); n];
    for p in prerequisites.iter() {
        pre_courses[p[0] as usize].insert(p[1] as usize);
        next_courses[p[1] as usize].insert(p[0] as usize);
    }

    let mut queue = std::collections::VecDeque::<usize>::new();
    for i in 0..n {
        if pre_courses[i].is_empty() {
            queue.push_back(i);
        }
    }

    let mut done_courses = std::collections::HashSet::<usize>::new();
    while !queue.is_empty() {
        let x = queue[0];
        queue.pop_front();
        if done_courses.contains(&x) {
            continue;
        }
        let mut is_done = true;
        for p in pre_courses[x].iter() {
            if !done_courses.contains(p) {
                is_done = false;
                break;
            }
        }
        if is_done {
            done_courses.insert(x);
            for c in next_courses[x].iter() {
                queue.push_back(*c);
            }
        }
    }

    return done_courses.len() == n;
}
