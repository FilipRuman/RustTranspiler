a0 +2 b0 *3 c0 +2 d0 $-1

1:

expr(0) =
    nud(a) = a  -> +2 > 0
    led(a, +)= -> 
        expr(2 -> bp of +) = 
            nud(b) = b  -> *3 > 2
            led(b,*) =
                expr(3) = c
                    nud(c) = c -> +2 !> 3
                    return c 
                        
2:

led(b,*) = (b*c) -> +2 !> 2
return (b*c)

3:

led(a,+) = (a + (b*c))
led((a + (b*c)),+) -> 2>0
    expr(2) =
        nud(d) = d  -> 0 !> 2
        return d

4:

led((a + (b*c)),+) = (a + (b*c)) + d

$ 

        

