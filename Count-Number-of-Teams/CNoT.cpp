#include <iostream>
#include <tuple>

typedef std::tuple<int, int> pair;

int sum_rating(int* rating, int start, int len) {
  int count = 0;
  for (int i = start; i < len; i++) {
    // std::cout << *(rating + start) << ", ";
    // std::cout << *(rating + i) << "; ";

    if (*(rating + start) < *(rating + i)) {
      count += 1;
    };
  }
  // std::cout << count << "\n";
  return count;
}

pair* count_larger_smaller_pair(int* rating, int len) {
  static pair cache[100];  // has to be static len
  for (int i = 0; i < len - 1; i++) {
    std::get<0>(cache[i]) = sum_rating(rating, i, len);
    std::get<1>(cache[i]) = len - 1 - i - std::get<0>(cache[i]);
    // std::cout << std::get<0>(cache[i]) << ": ";
    // std::cout << std::get<1>(cache[i]) << "\n";
  }
  std::get<0>(cache[len - 1]) = 0;
  std::get<0>(cache[len - 1]) = 0;

  // std::cout << &cache << ".\n";
  return cache;
}

int num_teams(int* rating, int len) {
  pair* cache = count_larger_smaller_pair(rating, len);
  int result = 0;

  for (int i = 0; i < len - 2; i++) {
    for (int j = i + 1; j < len - 1; j++) {
      if (rating[j] > rating[i]) {
        result += std::get<0>(cache[j]);
      } else {
        result += std::get<1>(cache[j]);
      }
    }
  }

  return result;
}

int main() {
  int a[] = {2, 5, 3, 4, 1};
  // pair* result_a = count_larger_smaller_pair(a, 5);

  // for (int i = 0; i < 5; i++) {
  //   std::cout << std::get<0>(result_a[i]) << ": ";
  //   std::cout << std::get<1>(result_a[i]) << "\n";
  // }

  std::cout << num_teams(a, 5) << "\n";  // 3

  int b[] = {2, 1, 3};
  std::cout << num_teams(b, 3) << "\n";  // 0

  int c[] = {1, 2, 3, 4};
  std::cout << num_teams(c, 4) << "\n";  // 4

  return 0;
}
