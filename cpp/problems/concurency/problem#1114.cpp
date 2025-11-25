#include <condition_variable>
#include <functional>
#include <mutex>

class Foo {
private:
  std::mutex fooLock;
  std::condition_variable cv;
  int current_thread = 1;

public:
  Foo() = default;

  void first(std::function<void()> printFirst) {
    std::unique_lock<std::mutex> lock(fooLock);
    cv.wait(lock, [this] { return current_thread == 1; });

    // printFirst() outputs "first". Do not change or remove this line.
    printFirst();

    current_thread++;
    cv.notify_all();
  }

  void second(std::function<void()> printSecond) {
    std::unique_lock<std::mutex> lock(fooLock);
    cv.wait(lock, [this] { return current_thread == 2; });

    // printSecond() outputs "second". Do not change or remove this line.
    printSecond();

    current_thread++;
    cv.notify_all();
  }

  void third(std::function<void()> printThird) {
    std::unique_lock<std::mutex> lock(fooLock);
    cv.wait(lock, [this] { return current_thread == 3; });

    // printThird() outputs "third". Do not change or remove this line.

    printThird();
  }
};
