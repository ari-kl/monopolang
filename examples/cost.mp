while true do
    set amount -> @balance / 2
    gamble amount

    if @won = true then
        print "I won $" + amount + "!"
    else
        print "I lost $" + amount + "!"
    end
end
