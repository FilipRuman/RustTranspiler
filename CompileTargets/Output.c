#include <stdio.h>
 
int main()
{
    int n = 153;
    int temp = n;
    float you = 0;
    int p = n;
    p++;
    char text[] = "test text";

    while (n > 0) {
        int rem = n % 10;
        p = (p) + (rem * rem * rem);
        n = n / 10;
    }
 
    // Condition to check whether the
    // value of P equals to user input
    // or not.
    if (temp == p) {
        printf("It is Armstrong No.");
    }
    else {
        printf(" It is not an Armstrong No. %d %s" , p , text);
    }
    return 0;
}
