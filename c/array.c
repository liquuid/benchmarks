#include<stdio.h>

int main(){
    int n = 100000000L;
    double *a = (double *)malloc(sizeof(double) * 100000000L);
    
    /*for(int i=0;i < n; i++){
        //printf("d ");
        a[i] = 0.0;
    }*/
     
    for(int i=0;i < n; i++){
        a[i] = i % 3;
    }
    
    for (int i = 0; i < 5; i++){
        printf("%f ",a[i]);
    }

}
