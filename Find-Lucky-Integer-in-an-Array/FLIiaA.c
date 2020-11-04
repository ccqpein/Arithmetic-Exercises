#include <assert.h>
#include <stdio.h>

int findLucky(int* arr, int arrSize) {
  int cache[501] = {};
  for (int i = 0; i < arrSize; i++) {
    cache[arr[i]] += 1;
  };

  for (int i = 500; i > 0; i--) {
    if (cache[i] == i) {
      return i;
    }
  }

  return -1;
}

int main() {
  int case0[4] = {2, 2, 3, 4};
  assert(2 == findLucky(case0, 4));
}
