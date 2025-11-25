#define ull unsigned long long

ull mod = 1e9 + 7;

ull pow1922(ull x, ull y) {
  if (y == 0) {
    return 1;
  }
  if (y % 2 == 1) {
    return (x * pow1922(x, y - 1)) % mod;
  }
  ull xx = (x * x) % mod;
  y = y / 2;
  return pow1922(xx, y);
}

int countGoodNumbers(ull n) {
  ull even = (n + 1) / 2;
  ull odd = n / 2;
  ull ans = (pow1922(5, even) * pow1922(4, odd)) % mod;
  return ans;
}
