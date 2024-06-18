let my_num = 20
let initial_balance = @balance

// Gamble $my_num, balance += $my_num if win, balance -= $my_num if lose
gamble my_num

if @balance > initial_balance then
  print "You won!"
end

// Buy 100 shares of AAPL
buy "AAPL" 100

// Sell 50 shares of AAPL
sell "AAPL" 50

// Take a loan of $1000
loan 1000

// Repay $500 of the loan
pay 500

// Print current debt, stored in readonly @debt variable
print "Current debt: " + @debt
