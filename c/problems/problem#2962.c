long long countSubarrays(int *nums, int numsSize, int k) {
  long long ans = 0;
  int maxNum = nums[0];
  for (int i = 1; i < numsSize; i++) {
    if (nums[i] > maxNum) {
      maxNum = nums[i];
    }
  }
  int left = 0, right = 0;
  int count = 0;
  while (right < numsSize) {
    int n = nums[right];
    if (n == maxNum) {
      count++;
    }
    while (count == k) {
      if (nums[left] == maxNum) {
        count--;
      }
      left++;
    }
    ans += left;
    right++;
  }
  return ans;
}
