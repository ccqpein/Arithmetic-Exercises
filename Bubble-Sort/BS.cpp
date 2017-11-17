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

struct node {
  int data;
  node * next;
};

class list {
private:
  node * head, * tail;

  node * createnode(int i){
    node * temp = new node;
    temp->data = i;
    temp->next = NULL;

    return temp;
  };

  
public:
  list() { // construct function
    head = NULL;
  }

  list(int i,...) {
    //https://www.cprogramming.com/tutorial/lesson17.html
    va_list args;
    va_start (args,i);
    
    int s = sizeof(args)/sizeof(long); // it should be long type

    node * first = new node;
    first->data = i;
    first->next = NULL;
    head = first;

    node * temp = head;
    
    for (int ind = 0; ind < s; ind++) {
      temp->next = createnode(va_arg(args,int));
      temp = temp->next;
    };
  };

  int show() {
    node * temp = this->head;
    do{
      cout << temp->data << endl;
      temp = temp->next;
    }while(temp != NULL);
  };
  
};

int main(){
  list a  = list(1,2,3,4);
  a.show();
}
