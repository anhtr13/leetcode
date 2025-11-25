#include <string>
#include <unordered_map>
#include <vector>

class Trie {
public:
  char val;
  std::unordered_map<char, Trie *> next;

  Trie() {}
  Trie(char c) { this->val = c; }
  void Insert(std::string str) {
    Trie *t = this;
    for (auto c : str) {
      if (t->next[c] == NULL) {
        t->next[c] = new Trie(c);
      }
      t = t->next[c];
    }
  }
};

std::string longestCommonPrefix(std::vector<std::string> &strs) {
  Trie root = Trie();
  int len = strs[0].length();
  for (auto str : strs) {
    if (str.length() < len) {
      len = str.length();
    }
    root.Insert(str);
  }
  Trie *t = &root;
  std::string ans = "";
  while (t != NULL) {
    if (t->next.size() != 1 || ans.length() == len) {
      break;
    }
    for (auto pair : t->next) {
      ans += pair.first;
      t = pair.second;
    }
  }
  return ans;
}
