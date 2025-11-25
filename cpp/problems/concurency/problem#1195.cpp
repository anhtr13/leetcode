#include <condition_variable>
#include <functional>
#include <mutex>

class FizzBuzz {
private:
  int n;
  int cur = 1;
  std::mutex glock;
  std::condition_variable cv;

public:
  FizzBuzz(int n) { this->n = n; }

  // printFizz() outputs "fizz".
  void fizz(std::function<void()> printFizz) {
    for (int i = 3; i <= n; i += 3) {
      if (i % 5 != 0) {
        std::unique_lock<std::mutex> lock(glock);
        cv.wait(lock, [this] { return cur % 3 == 0 && cur % 5 != 0; });
        printFizz();
        cur = cur + 1;
        cv.notify_all();
      }
    }
  }

  // printBuzz() outputs "buzz".
  void buzz(std::function<void()> printBuzz) {
    for (int i = 5; i <= n; i += 5) {
      if (i % 3 != 0) {
        std::unique_lock<std::mutex> lock(glock);
        cv.wait(lock, [this] { return cur % 5 == 0 && cur % 3 != 0; });
        printBuzz();
        cur = cur + 1;
        cv.notify_all();
      }
    }
  }

  // printFizzBuzz() outputs "fizzbuzz".
  void fizzbuzz(std::function<void()> printFizzBuzz) {
    for (int i = 15; i <= n; i += 15) {
      std::unique_lock<std::mutex> lock(glock);
      cv.wait(lock, [this] { return cur % 3 == 0 && cur % 5 == 0; });
      printFizzBuzz();
      cur = cur + 1;
      cv.notify_all();
    }
  }

  // printNumber(x) outputs "x", where x is an integer.
  void number(std::function<void(int)> printNumber) {
    for (int i = 1; i <= n; i++) {
      if (i % 3 != 0 && i % 5 != 0) {
        std::unique_lock<std::mutex> lock(glock);
        cv.wait(lock, [this] { return cur % 3 != 0 && cur % 5 != 0; });
        printNumber(cur);
        cur = cur + 1;
        cv.notify_all();
      }
    }
  }
};
