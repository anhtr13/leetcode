int totalMoney(int n) {
  int ans = 0;
  int base = 0;
  for (int i = 0; i < n; i++) {
    int k = i % 7;
    if (k == 0) {
      base++;
    }
    ans += base + k;
  }
  return ans;
}
