#include <string>
#include <vector>

int minCost(std::string colors, std::vector<int> &neededTime) {
  int sum = neededTime[0];
  int time = 0;
  int x = neededTime[0];
  for (int i = 1; i < colors.size(); i++) {
    if (colors[i] == colors[i - 1]) {
      x = neededTime[i] > x ? neededTime[i] : x;
    } else {
      time += x;
      x = neededTime[i];
    }
    sum += neededTime[i];
  }
  time += x;
  return sum - time;
}
