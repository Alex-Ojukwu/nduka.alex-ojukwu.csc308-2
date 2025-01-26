#include <stdio.h>

int main() {
    int choice;
    float num1, num2, result;
    printf("this is a simple calculator \n");
    printf("choose an operation to perform \n");
    printf("1 addition (+)\n");
    printf("2 subtraction(-)\n");
    printf("3 multiplication (*)\n");
    printf("4 division (/)\n");

    printf(" Enter your choice (1-4) ");
    scanf("%i", &choice);

    printf(" Enter num1 ");
    scanf("%f", &num1);

    printf(" Enter your num2 ");
    scanf("%f", &num2);

    switch (choice)
    { case 1:
    result = num1 + num2;
      printf(" the addition is ");
    scanf("%f", &result);
        break;
    case 2: 
    result = num1 -num2;
    printf(" subtraction is ");
    scanf("%f", &result);
    break;
    case 3:
    result = num1 * num2;
    printf(" multiplication is ");
    scanf("%f", &result);
    break;
    case 4 :
   if (num2 != 0) { 
    result = num1 / num2 ;
    printf(" division is ");
    scanf("%f", &result);} 
    else {
        printf("error");
    }
    break;
    
    default:
    printf(" invalid choice");
        break;
    }
    return 0;




}