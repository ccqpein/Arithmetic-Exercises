#include <iostream>
#include <array>
using namespace std;

/* // code below is learning `dangling pointer`
int* bubbleSort(){
  int * arr = new int[3]; // avoid dangling pointer
  for (int i =0 ; i <3; i++) arr[i] = i+1;
  return arr;
}

int main(){
  int* arr = bubbleSort();
  for (int i = 0; i <3; i++){
    cout << *(arr+i) << endl;
  }
  delete [] arr;
  return 0;
}
*/

int main(){
  array<int,3> arr = {1,2,3};
  cout << arr[1] <<endl;
}
