long long countSubarrays(int *nums, int numsSize, int minK, int maxK) {
  long long ans = 0;
  int lastIdxMinK = -1, lastIdxMaxK = -1;
  int startIdx = 0;
  for (int i = 0; i < numsSize; i++) {
    if (nums[i] > maxK || nums[i] < minK) {
      lastIdxMaxK = -1;
      lastIdxMinK = -1;
      startIdx = i + 1;
      continue;
    }
    if (nums[i] == minK) {
      lastIdxMinK = i;
    }
    if (nums[i] == maxK) {
      lastIdxMaxK = i;
    }
    int idx = lastIdxMaxK < lastIdxMinK ? lastIdxMaxK : lastIdxMinK;
    if (idx != -1) {
      ans += (idx - startIdx) + 1;
    }
  }
  return ans;
}
