#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int judge (int n, int endNum)
{ // if is,return n; if not, return 0;
  for (int i = 2; i <= endNum; i++){
    if ((n%i == 0) && (n != i)){
      return 0;
    }else{
      continue;
    }
  }
  return n;
}

int main ()
{
  int n = 50;
  int endNum = (int)(ceil(sqrt((double)n)));

  for (int i = 2; i <= n; i++){
    if (judge(i,endNum) != 0){
      printf("%d ",i);
    }else{
      continue;
    }
  }
}

