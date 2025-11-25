#include <condition_variable>
#include <functional>
#include <mutex>

class FooBar {
private:
  int n;
  bool state = false;
  std::mutex glock;
  std::condition_variable cv;

public:
  FooBar(int n) { this->n = n; }

  void foo(std::function<void()> printFoo) {
    std::unique_lock<std::mutex> lock(glock);
    for (int i = 0; i < n; i++) {
      cv.wait(lock, [this] { return state == false; });
      printFoo();
      this->state = true;
      cv.notify_all();
    }
  }

  void bar(std::function<void()> printBar) {
    std::unique_lock<std::mutex> lock(glock);
    for (int i = 0; i < n; i++) {
      cv.wait(lock, [this] { return state == true; });
      printBar();
      this->state = false;
      cv.notify_all();
    }
  }
};
