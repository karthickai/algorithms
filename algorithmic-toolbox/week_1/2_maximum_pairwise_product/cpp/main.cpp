/// Maximum Pairwise Product
/// Input
/// 3
/// 1 2 3
/// Output
/// 6

#include <iostream>
#include <vector>
#include <algorithm>

int main(){
    int size = {};
    std::cin >> size;
    std::vector<int> numbers = {}; 
    for (int i=0; i < size; i++){
        int num;
        std::cin >> num;
        numbers.push_back(num);
    }
    std::sort(numbers.begin(), numbers.end());
    int first_largest_num = numbers.at(numbers.size() - 1);
    numbers.pop_back();
    int second_largest_num = numbers.at(numbers.size() - 1);
    std::cout << first_largest_num * second_largest_num << std::endl;
    return 0;
}