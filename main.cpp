#include <iostream>
#include <bitset>

int main() {
  int x;
  std::cout << "Please enter a decimal to convert: ";
  std::cin >> x;

  if (x < 0 || x > 255) {
    std::cout << "Please enter a number between 0 and 255." << std::endl;
    return 1;
  }

  std::bitset<8> bitset(x);
  std::cout << bitset << std::endl;
  std::cout << x;
}
