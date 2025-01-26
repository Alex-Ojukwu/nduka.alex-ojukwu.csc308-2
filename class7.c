#include <stdio.h>

/*int main() {

    int a;

    int *p2a;
    p2a = &a;

    printf("%p\n", p2a); //%p is the place holder for pointer
}

int main() {
// pointer arithmetic
  char name[] = "goerge";
   for (int i=0; i<6; i++) {
    printf("%c", name[i]);
   }

   char *p2name = name;

   for (int i=0; i<6; i++) {

    printf("%s\n", p2name);
    p2name++;
   }*/

   /* int a;
    float b;
    double c;
    char d;
    double money[5];
    char name[5];

    printf("%s", a);
      //print out the memory address of the variables

      //puts("memory of the variables declared: ");
      //printf("Address of a: %p\n", &a);
      //printf("Address of b: %p\n", &b);
      //printf("Address of c: %p\n", &c);
      //printf("Address of d: %p\n", &d);

 

}*/



int main() {
    int boys[] = {2, 4, 6, 8};
    int girls[] = {1, 3, 5, 7};

    for (int i = 0; i < 4; i++) {
        printf("Boys[%i] = %d\n", i, *(boys + i));
        printf("Girls[%i] = %d\n", i, *(girls + i));
    }

    return 0;
}
