#include "../../utils/disjoint_set.hpp"
#include <set>
#include <stdlib.h>
#include <unordered_map>
#include <vector>

std::vector<int> processQueries(int c,
                                std::vector<std::vector<int>> &connections,
                                std::vector<std::vector<int>> &queries) {
  std::vector<bool> online(c + 1, true);
  std::unordered_map<int, std::set<int> *> nets;
  DisjointSet ds(c + 1);
  for (std::vector<int> con : connections) {
    ds.Union(con[0], con[1]);
  }
  for (std::vector<int> con : connections) {
    int id = ds.Find(con[0]);
    if (nets[id] == NULL) {
      nets[id] = new std::set<int>;
    }
    nets[id]->insert(con[0]);
    nets[id]->insert(con[1]);
  }
  std::vector<int> res;
  for (int i = 0; i < queries.size(); i++) {
    int mod = queries[i][0], station = queries[i][1];
    if (mod == 1) {
      if (online[station]) {
        res.push_back(station);
      } else {
        int net_id = ds.Find(station);
        std::set<int> *net = nets[net_id];
        if (net == NULL || net->empty()) {
          res.push_back(-1);
        } else {
          res.push_back(*net->begin());
        }
      }
    } else {
      int net_id = ds.Find(station);
      std::set<int> *net = nets[net_id];
      if (net != NULL) {
        net->erase(station);
      }
      online[station] = false;
    }
  }
  return res;
}
