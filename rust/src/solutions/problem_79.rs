use crate::Solution;

impl Solution {
    fn backtrack_79(
        board: &Vec<Vec<char>>,
        word: &[u8],
        visited: &mut Vec<Vec<bool>>,
        i: u8,
        x: i8,
        y: i8,
    ) -> bool {
        if i >= word.len() as u8 {
            return true;
        }
        let vv = vec![
            vec![x - 1, y],
            vec![x + 1, y],
            vec![x, y - 1],
            vec![x, y + 1],
        ];
        for v in vv {
            if v[0] >= 0
                && v[0] < board.len() as i8
                && v[1] >= 0
                && v[1] < board[0].len() as i8
                && !visited[v[0] as usize][v[1] as usize]
                && board[v[0] as usize][v[1] as usize] as u8 == word[i as usize]
            {
                visited[v[0] as usize][v[1] as usize] = true;
                let flag = Self::backtrack_79(board, word, visited, i + 1, v[0], v[1]);
                visited[v[0] as usize][v[1] as usize] = false;
                if flag {
                    return true;
                }
            }
        }
        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let mut start = Vec::<Vec<i8>>::new();
        let mut visited = Vec::<Vec<bool>>::new();
        let word = word.as_bytes();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] as u8 == word[0] {
                    start.push(vec![i as i8, j as i8]);
                }
            }
            visited.push(vec![false; n]);
        }
        for v in start {
            visited[v[0] as usize][v[1] as usize] = true;
            if Self::backtrack_79(&board, word, &mut visited, 1, v[0], v[1]) {
                return true;
            }
            visited[v[0] as usize][v[1] as usize] = false;
        }
        false
    }
}
