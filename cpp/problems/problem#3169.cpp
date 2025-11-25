#include <algorithm>
#include <vector>

int countDays(int days, std::vector<std::vector<int>> &meetings) {
  sort(meetings.begin(), meetings.end(),
       [](std::vector<int> &a, std::vector<int> &b) {
         if (a[0] == b[0]) {
           return a[1] < b[1];
         }
         return a[0] < b[0];
       });
  std::vector<std::vector<int>> windows;
  std::vector<int> window;
  window.push_back(meetings[0][0]);
  window.push_back(meetings[0][1]);
  for (int i = 1; i < meetings.size(); i++) {
    std::vector<int> w = meetings[i];
    if (w[0] <= window[1]) {
      if (window[1] < w[1]) {
        window[1] = w[1];
      }
    } else {
      std::vector<int> temp;
      temp.push_back(window[0]);
      temp.push_back(window[1]);
      windows.push_back(temp);
      window[0] = w[0];
      window[1] = w[1];
    }
  }
  windows.push_back(window);
  int ans = days - windows[windows.size() - 1][1];
  int cur = 0;

  for (auto x : windows) {
    ans += x[0] - cur - 1;
    cur = x[1];
  }

  return ans;
}
