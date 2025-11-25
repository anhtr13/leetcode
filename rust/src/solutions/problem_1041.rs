use crate::Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut dir = 'N';
        let (mut x, mut y) = (0, 0);
        for i in instructions.chars() {
            if dir == 'N' {
                if i == 'G' {
                    y += 1;
                } else if i == 'L' {
                    dir = 'W';
                } else if i == 'R' {
                    dir = 'E';
                }
            } else if dir == 'W' {
                if i == 'G' {
                    x -= 1;
                } else if i == 'L' {
                    dir = 'S';
                } else if i == 'R' {
                    dir = 'N';
                }
            } else if dir == 'S' {
                if i == 'G' {
                    y -= 1;
                } else if i == 'L' {
                    dir = 'E';
                } else if i == 'R' {
                    dir = 'W';
                }
            } else if dir == 'E' {
                if i == 'G' {
                    x += 1;
                } else if i == 'L' {
                    dir = 'N';
                } else if i == 'R' {
                    dir = 'S';
                }
            }
        }
        return (x == 0 && y == 0) || dir != 'N';
    }
}
