#include <functional>
#include <iterator>
#include <set>
#include <stdlib.h>
#include <unordered_map>
#include <utility>
#include <vector>

std::vector<long long> findXSum(std::vector<int> &nums, int k, int x) {
  std::unordered_map<int, int> freq;
  std::set<std::pair<int, int>, std::greater<std::pair<int, int>>> top;
  std::set<std::pair<int, int>, std::greater<std::pair<int, int>>> rest;
  long long x_sum = 0;

  auto insert_num = [&](int n) {
    int &f = freq[n];
    if (f > 0) {
      std::pair<int, int> fn = {f, n};
      auto it = top.find(fn);
      if (it != top.end()) {
        x_sum -= 1LL * it->first * it->second;
        top.erase(it);
      } else {
        rest.erase(fn);
      }
    }
    top.insert({++f, n});
    x_sum += 1LL * f * n;
    if (top.size() > x) {
      auto it = std::prev(top.end());
      x_sum -= 1LL * it->first * it->second;
      rest.insert(*it);
      top.erase(it);
    }
  };

  auto remove_num = [&](int n) {
    int &f = freq[n];
    if (f == 0) {
      return;
    }
    std::pair<int, int> fn = {f, n};
    auto it = top.find(fn);
    if (it != top.end()) {
      x_sum -= 1LL * it->first * it->second;
      top.erase(fn);
    } else {
      rest.erase(fn);
    }
    if (--f == 0) {
      freq.erase(n);
    } else {
      rest.insert({f, n});
    }
    if (top.size() < x && !rest.empty()) {
      auto it = rest.begin();
      x_sum += 1LL * it->first * it->second;
      top.insert(*it);
      rest.erase(it);
    }
  };

  std::vector<long long> ans(nums.size() - k + 1);
  int i = 0;
  for (; i < k; i++) {
    insert_num(nums[i]);
  }

  ans[0] = x_sum;
  for (; i < nums.size(); i++) {
    insert_num(nums[i]);
    remove_num(nums[i - k]);
    ans[i - k + 1] = x_sum;
  }

  return ans;
}
