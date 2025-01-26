#include <stdio.h>

int main() {

    int rowMatrix[] = {4,5,7,8,9};
    int newArray[5];
    int matrx[3][2] ={ {3,8}, {2,3},{6,7}};

   //printf("%i\n", rowMatrix[2]);
   //printf("%i\n" , matrx[1][1]);

    /* for (int i = 0; i < 3; i++) {         // Loop over rows
        for (int j = 0; j < 2; j++) {     // Loop over columns
            printf("%i\n", matrx[i][j]);  // Print each element
        }
    }*/

   int age= 10;

   int *ptr = &age;

    return 0;
}