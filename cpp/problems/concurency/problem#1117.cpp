#include <condition_variable>
#include <functional>
#include <mutex>

class H2O {
private:
  int h_count = 0;
  std::mutex glock;
  std::condition_variable cv;

public:
  H2O() { h_count = 0; }

  void hydrogen(std::function<void()> releaseHydrogen) {
    std::unique_lock<std::mutex> lock(glock);
    cv.wait(lock, [this] { return h_count < 2; });
    // releaseHydrogen() outputs "H". Do not change or remove this line.
    releaseHydrogen();
    h_count++;
    cv.notify_all();
  }

  void oxygen(std::function<void()> releaseOxygen) {
    std::unique_lock<std::mutex> lock(glock);
    cv.wait(lock, [this] { return h_count >= 2; });
    // releaseOxygen() outputs "O". Do not change or remove this line.
    releaseOxygen();
    h_count = 0;
    cv.notify_all();
  }
};
