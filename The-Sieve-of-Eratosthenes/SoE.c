#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int main ()
{
  int tttnum = 50;
  int nL[tttnum];

  for (int i = 1; i<= tttnum; i++) {
    nL[i-1] = i;
  }

  for (int i = 0; i < sizeof(nL)/sizeof(nL[0]); i++){
    printf("%d ", nL[i]);
  }
  printf("\n");

  int endNum = (int)(ceil(sqrt((double)tttnum)));
  int LSize = tttnum;

  // printf("%d,%d \n",endNum,LSize); //right

  // have problem, I think it is array symbol scope issues
  for (int i = 2;i <= endNum; i++){
    int tempL[LSize];
    double d = (double) i;
    int num  = 0;
    
    for (int ii = 0; ii < LSize; ii++){
      if ((modf(((double) nL[ii]), &d) != 0) || (nL[ii] == i)){
        tempL[num] = nL[ii];
        num++;
        }
    }
    *nL = *tempL;

    for (int i = 0; i < sizeof(nL)/sizeof(nL[0]); i++){
      printf("%d ", nL[i]);
    }
    printf("\n");    

    LSize = num;
  }
  
  for (int i = 0; i < sizeof(nL)/sizeof(nL[0]); i++){
    printf("%d ", nL[i]);
  }
  printf("\n");
}
