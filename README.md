# monopolang
langjam - economy language

(get it? monopoly + language = monopolang)

The Idea:
* Each time a program is run, the process has an initial amount of money
* Different language features / operations cost some amount of money to use
* At runtime, the process has access to different financial institutions (banks, loans, etc.) to get more money
* The goal is to write a program that performs a certain task while being economically sustainable
* If you run out of money, the program crashes

The Language:
* The language is a simple imperative language with a few basic operations
* Lua-like syntax
* Dynamic scope (I'm lazy)

The Economy Model:
* Initial money: $100
* Costs:
  * print: $1
  * for loop: $5
  * while loop: $5
  * if statement: $5
  * variable declaration: $10
  * variable assignment: $1
  * function declaration: $20
  * function call: $5
* Financial institutions:
  * Loan: Borrow a certain amount of money, pay it back with interest
  * Stock Market: Buy and sell stocks, make or lose money
  * Gambling: Bet a certain amount of money, win or lose it
  * Taxes: Every X operations, you have to pay Y% of the difference in your balance
  * ???

Progress:
* [ ] Lexer (String -> Tokens)
* [ ] Parser (Tokens -> AST)
* [ ] Compiler (AST -> Bytecode)
* [ ] Interpreter (Execute Bytecode)
* [ ] Economy (Costs, Financial Institutions; added at runtime)
