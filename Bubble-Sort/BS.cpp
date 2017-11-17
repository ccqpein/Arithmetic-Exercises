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
  node * last;
};

class list {
private:

  node * createnode(int i){
    node * temp = new node;
    temp->data = i;
    temp->next = NULL;
    temp->last = NULL;

    return temp;
  };

  
public:
  
  node * head, * tail;

  list() { // construct function
    head = NULL;
  }

  list(int i,...) {
    //https://www.cprogramming.com/tutorial/lesson17.html
    va_list args;
    va_start (args,i);
    
    int s = sizeof(args)/sizeof(int);

    node * first = createnode(i);
    head = first;

    node * temp = head;
    
    for (int ind = 0; ind < 5; ind++) { // need to solve this problems
      int thisarg = va_arg(args,int);
      if (thisarg != NULL){
	temp->next = createnode(thisarg);
	node * buffer = temp;
	temp = temp->next;
	temp->last = buffer;
      };
    };

    temp->next = NULL;
  };

  int show() {
    node * temp = this->head;
    do{
      cout << temp->data << endl;
      temp = temp->next;
    }while(temp != NULL);
  };
};


int bubbleSort(list * lis) {
  int flag = 0;
  node * temp = lis->head;
  do {
    flag = 0;
    while(temp->next != NULL){
      if (temp->next->data < temp->data) {
	flag = 1;
	node * buffer = temp->next->next;
	temp->next->next = temp;
	temp->next->last = temp->last;
	temp->last->next = temp->next;
	temp->last = temp->next;
	temp->next = buffer;
      }else{
	temp = temp->next;
      };
    };
    temp = lis->head;
  }while(flag != 0);
}

int main(){
  //list a  = list(1,2,3,4);
  //a.show();

  list b = list(2,3,44,5,33,23);
  b.show();
  bubbleSort(&b);
  b.show();
}
