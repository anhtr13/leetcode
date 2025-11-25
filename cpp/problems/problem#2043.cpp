#include <vector>

#define ll long long

class Bank {
private:
  std::vector<ll> balance;
  int n;

public:
  Bank(std::vector<ll> &bl) {
    for (int i = 0; i < bl.size(); i++) {
      this->balance.push_back(bl[i]);
    }
    this->n = bl.size() + 1;
  }

  bool transfer(int account1, int account2, long long money) {
    if (account1 > n || account2 > n) {
      return false;
    }
    if (this->balance[account1 - 1] < money) {
      return false;
    }
    this->balance[account1 - 1] -= money;
    this->balance[account2 - 1] += money;
    return true;
  }

  bool deposit(int account, long long money) {
    if (account > this->n) {
      return false;
    }
    this->balance[account - 1] += money;
    return true;
  }

  bool withdraw(int account, long long money) {
    if (account > this->n || money > this->balance[account - 1]) {
      return false;
    }
    this->balance[account - 1] -= money;
    return true;
  }
};
