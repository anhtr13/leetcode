#include <condition_variable>
#include <functional>
#include <mutex>

class DiningPhilosophers {
private:
  bool folks[5] = {true, true, true, true, true};
  std::mutex glock;
  std::condition_variable cv;

public:
  DiningPhilosophers() = default;

  void wantsToEat(int philosopher, std::function<void()> pickLeftFork,
                  std::function<void()> pickRightFork,
                  std::function<void()> eat, std::function<void()> putLeftFork,
                  std::function<void()> putRightFork) {
    std::unique_lock<std::mutex> lock(glock);
    int left = philosopher;
    int right = (philosopher + 1) % 5;
    cv.wait(lock, [&] { return folks[left] && folks[right]; });

    pickLeftFork();
    folks[left] = false;
    pickRightFork();
    folks[right] = false;

    eat();

    putLeftFork();
    folks[left] = true;
    cv.notify_all();

    putRightFork();
    folks[right] = true;
    cv.notify_all();
  }
};
