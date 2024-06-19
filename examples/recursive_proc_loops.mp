proc factorial do
    if n = 0 then
        set return -> 1
    else
        set n -> n - 1
        call factorial
        set n -> n + 1
        set return -> n * return
    end
end

range i from 0 to 10 do
    set n -> i
    call factorial
    print n
    print " factorial is "
    print return
end
