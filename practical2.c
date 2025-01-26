#include <stdio.h>

int main () {
   
   int CSC201;
   int CSC205;
   int STA205;

   printf(" Enter your CSC201 score ");
    scanf("%d", &CSC201);
   printf(" Enter your CSC205 score ");
    scanf("%d", &CSC205);
    printf(" Enter your STA205 ");
    scanf("%d", &STA205);

    int scoreaverage = (CSC201 + CSC205 + STA205)/3;

    printf("%i", scoreaverage );

}