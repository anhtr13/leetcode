#include <unordered_set>
#include <vector>

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

ListNode *modifiedList(std::vector<int> &nums, ListNode *head) {
  std::unordered_set<int> num_set;
  for (int num : nums) {
    num_set.insert(num);
  }
  ListNode *p = new ListNode;
  p->next = head;
  while (p->next != NULL) {
    ListNode *node = p->next;
    if (num_set.count(node->val)) {
      if (node == head) {
        head = head->next;
      }
      p->next = node->next;
    } else {
      p = p->next;
    }
  }
  return head;
}
