#include <condition_variable>
#include <functional>
#include <mutex>

class ZeroEvenOdd {
private:
  int n;
  int cur = 0;
  int state = 0;
  std::mutex glock;
  std::condition_variable cv;

public:
  ZeroEvenOdd(int n) {
    this->n = n;
    state = 0;
    cur = 1;
  }

  // printNumber(x) outputs "x", where x is an integer.
  void zero(std::function<void(int)> printNumber) {
    for (int i = 0; i < n; i++) {
      std::unique_lock<std::mutex> lock(glock);
      cv.wait(lock, [this] { return state == 0; });
      printNumber(0);
      state = cur % 2 == 1 ? 1 : 2;
      cv.notify_all();
    }
  }

  void odd(std::function<void(int)> printNumber) {
    for (int i = 1; i <= n; i += 2) {
      std::unique_lock<std::mutex> lock(glock);
      cv.wait(lock, [this] { return state == 1; });
      printNumber(cur);
      state = 0;
      cur = cur + 1;
      cv.notify_all();
    }
  }

  void even(std::function<void(int)> printNumber) {
    for (int i = 2; i <= n; i += 2) {
      std::unique_lock<std::mutex> lock(glock);
      cv.wait(lock, [this] { return state == 2; });
      printNumber(cur);
      state = 0;
      cur = cur + 1;
      cv.notify_all();
    }
  }
};
