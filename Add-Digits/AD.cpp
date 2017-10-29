#include <iostream>

using namespace std;

int AD(int a){
  int num = a, sum;

  do {
    sum = 0;
    while(num>0) {
      sum += num%10;
      num /= 10;
    }
    num = sum;
  }while(sum > 10);
    
  return sum;
}

int main(){

  int sum = AD(38);
  cout << sum;
  return 0;
}
