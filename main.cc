#include <iostream>
#include <string>
#include <vector>

#define MAX 32768
#define P 5
#define C 8

std::string print(int p) {
    std::string str;
    for (int i = 0; i < P; ++i) {
        switch (p % C) {
        case 0:
            str += " Bla";
            break;
        case 1:
            str += " Whi";
            break;
        case 2:
            str += " Red";
            break;
        case 3:
            str += " Gre";
            break;
        case 4:
            str += " Blu";
            break;
        case 5:
            str += " Yel";
            break;
        case 6:
            str += " Ora";
            break;
        case 7:
            str += " Bro";
            break;
        }
        p /= C;
    }
    return str;
}

int f(int a, int b) {
    int c = 0;
    for (int i = 0; i < P; ++i) {
        if (a % C == b % C) {
            ++c;
        }
        a /= C;
        b /= C;
    }
    return c;
}

int g(int a, int b) {
    std::vector<bool> x(C, false);
    std::vector<bool> y(C, false);
    for (int i = 0; i < P; ++i) {
        x[a % C] = true;
        y[b % C] = true;
        a /= C;
        b /= C;
    }
    int c = 0;
    for (int i = 0; i < C; ++i) {
        if (x[i] && y[i]) {
            ++c;
        }
    }
    return c;
}

int h(int p) {
    std::vector<bool> x(C, false);
    for (int i = 0; i < P; ++i) {
        x[p % C] = true;
        p /= C;
    }
    int c = 0;
    for (int i = 0; i < C; ++i) {
        if (x[i]) {
            ++c;
        }
    }
    return c;
}

int main(void) {
    int p = 0;
    std::vector<bool> c(MAX, true);
    for (int i = 0; i < MAX; ++i) {
        if (h(i) != P) {
            c[i] = false;
        }
    }
    for (int turn = 1; true; ++turn) {
        int n = 0;
        for (int i = 0; i < MAX; ++i) {
            if (c[i]) {
                ++n;
                p = i;
            }
        }
        if (!n) {
            std::cout << "\x1b[31m.. error:\x1b[0m sequence not found" << std::endl;
            break;
        }
        std::cout << std::endl << "\x1b[33m.. sequences:\x1b[0m " << n << std::endl;
        std::cout << "\x1b[33m.. turn " << turn << ":\x1b[0m" << print(p) << std::endl;
        c[p] = false;
        int b = 0, w = 0;
        std::cout << "\x1b[34m>> black:\x1b[0m ";
        std::cin >> b;
        std::cout << "\x1b[34m>> white:\x1b[0m ";
        std::cin >> w;
        if (b < 0 || 5 < b || w < 0 || 5 < w) {
            std::cout << "\x1b[31m.. error:\x1b[0m wrong input" << std::endl;
            break;
        }
        if (b == P) {
            std::cout << "\x1b[32m.. success:\x1b[0m I win !" << std::endl;
            break;
        }

        for (int i = 0; i < MAX; ++i) {
            if (c[i] && (f(p, i) != b || g(p, i) != b + w)) {
                c[i] = false;
            }
        }
    }
    return 0;
}
