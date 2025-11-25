#include <algorithm>
#include <stdlib.h>
#include <unordered_map>
#include <vector>

typedef struct {
  int num;
  int freq;
} item;

int x_sum(std::vector<item *> &items, int x) {
  std::sort(items.begin(), items.end(), [](item *a, item *b) {
    if (a->freq == b->freq) {
      return a->num > b->num;
    }
    return a->freq > b->freq;
  });
  int res = 0;
  for (int i = 0; i < x && i < items.size(); i++) {
    res += items[i]->num * items[i]->freq;
  }
  return res;
}

std::vector<int> findXSum(std::vector<int> &nums, int k, int x) {
  std::unordered_map<int, item *> m_num_item;
  std::vector<item *> items;
  std::vector<int> res;
  int i = 0;
  for (; i < k; i++) {
    if (m_num_item[nums[i]] == NULL) {
      m_num_item[nums[i]] = new item();
      m_num_item[nums[i]]->num = nums[i];
      m_num_item[nums[i]]->freq = 1;
      items.push_back(m_num_item[nums[i]]);
    } else {
      m_num_item[nums[i]]->freq++;
    }
  }
  res.push_back(x_sum(items, x));
  for (; i < nums.size(); i++) {
    int l = nums[i - k];
    int r = nums[i];
    m_num_item[l]->freq--;
    if (m_num_item[r] == NULL) {
      m_num_item[r] = new item();
      m_num_item[r]->num = r;
      m_num_item[r]->freq = 1;
      items.push_back(m_num_item[r]);
    } else {
      m_num_item[r]->freq++;
    }
    res.push_back(x_sum(items, x));
  }
  return res;
}
