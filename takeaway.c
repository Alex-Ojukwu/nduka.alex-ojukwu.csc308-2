#include <stdio.h>

int main() {
    char type;
    printf("Enter type (c for char, i for int, f for float): ");
    scanf(" %c", &type);

    switch (type) {
        case 'c': {
            char ch;
            printf("Enter a character: ");
            scanf(" %c", &ch);

            printf("Original character: %c, ASCII: %d\n", ch, ch);
            printf("Next 4 characters:\n");

            for (int i = 1; i <= 4; i++) {
                char nextCh = ch + (3 * i);
                printf("%c (ASCII: %d)\n", nextCh, nextCh);
            }

            printf("Size of char: %lu byte\n", sizeof(ch));
            break;
        }

        case 'i': {
            int num;
            printf("Enter an integer: ");
            scanf("%d", &num);

            printf("Original integer: %d\n", num);
            printf("Next 4 integers:\n");

            for (int i = 1; i <= 4; i++) {
                int nextNum = num + (3 * i);
                printf("%d\n", nextNum);
            }

            printf("Size of int: %lu bytes\n", sizeof(num));
            break;
        }

        case 'f': {
            float num;
            printf("Enter a float: ");
            scanf("%f", &num);

            printf("Original float: %.2f\n", num);
            printf("Next 4 floats:\n");

            for (int i = 1; i <= 4; i++) {
                float nextNum = num + (3.0f * i);
                printf("%.2f\n", nextNum);
            }

            printf("Size of float: %lu bytes\n", sizeof(num));
            break;
        }

        default:
            printf("Invalid type.\n");
    }

    return 0;
}
