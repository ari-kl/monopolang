loan @balance * 5

range _ from 0 to 1000 do
    if @can_work then
        work
    end

    if @balance > 1000 then
        buy "HACK" 1
    end
end

set current -> @balance
sell "HACK" 1
set profit -> @balance - current

print "End cost of HACK: " + @balance
