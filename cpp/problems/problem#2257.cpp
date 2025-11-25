#include <vector>

int countUnguarded(int m, int n, std::vector<std::vector<int>> &guards,
                   std::vector<std::vector<int>> &walls) {
  // grid[x][y][0] : position[x][y], direction[row]
  // grid[x][y][1] : position[x][y], direction[col]
  // u : unguarded, g: guarded, G: guard, W: wall
  std::vector<std::vector<std::vector<char>>> grid(
      m, std::vector<std::vector<char>>(n, std::vector<char>(2, 'u')));
  for (std::vector<int> guard : guards) {
    int x = guard[0];
    int y = guard[1];
    grid[x][y][0] = 'G';
    grid[x][y][1] = 'G';
  }
  for (std::vector<int> wall : walls) {
    int x = wall[0];
    int y = wall[1];
    grid[x][y][0] = 'W';
    grid[x][y][1] = 'W';
  }
  for (std::vector<int> guard : guards) {
    int x = guard[0];
    int y = guard[1];
    for (int i = x - 1; i >= 0; i--) {
      if (grid[i][y][0] != 'u') {
        break;
      }
      grid[i][y][0] = 'g';
    }
    for (int i = x + 1; i < m; i++) {
      if (grid[i][y][0] != 'u') {
        break;
      }
      grid[i][y][0] = 'g';
    }
    for (int j = y - 1; j >= 0; j--) {
      if (grid[x][j][1] != 'u') {
        break;
      }
      grid[x][j][1] = 'g';
    }
    for (int j = y + 1; j < n; j++) {
      if (grid[x][j][1] != 'u') {
        break;
      }
      grid[x][j][1] = 'g';
    }
  }
  int ans = 0;
  for (auto row : grid) {
    for (auto cell : row) {
      if (cell[0] == 'u' && cell[1] == 'u') {
        ans++;
      }
    }
  }
  return ans;
}
