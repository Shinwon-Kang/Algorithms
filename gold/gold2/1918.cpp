#include <iostream>
#include <string>
#include <stack>

int main()
{
    std::string expr;
    std::cin >> expr;
    
    std::stack<char> expr_stack;
    
    for(int i=0; i<expr.length(); i++) {
        if(expr[i] == '*' || expr[i] == '/') {
            while(!expr_stack.empty()) {
                if(expr_stack.top() == '*' || expr_stack.top() == '/') {
                    std::cout << expr_stack.top();
                    expr_stack.pop();
                } else {
                    break;
                }
            }
            expr_stack.push(expr[i]);
        } else if(expr[i] == '+' || expr[i] == '-') {
            while(!expr_stack.empty()) {
                char temp = expr_stack.top();
                expr_stack.pop();
                
                if(temp == '+' || temp =='-') {
                    std::cout << temp;
                    break;
                } else if(temp == '*' || temp =='/') {
                    std::cout << temp;
                } else {
                    expr_stack.push(temp);
                    break;
                }
            }
            expr_stack.push(expr[i]);
        } else if(expr[i] == ')') {
            while(!expr_stack.empty()) {
                char temp = expr_stack.top();
                expr_stack.pop();
                
                if(temp == '(') {
                    break;
                } else {
                    std::cout << temp;
                }
            }
        } else if(expr[i] == '(') {
            expr_stack.push(expr[i]);
        }else {
            std::cout << expr[i];
        }
    }
    
    while(!expr_stack.empty()) {
        std::cout << expr_stack.top();
        expr_stack.pop();
    }
    
    return 0;
}
