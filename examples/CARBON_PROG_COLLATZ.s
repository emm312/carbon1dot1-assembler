pld $0
rst r1

.loop
// acc == r1 here and contains current collatz number
pst $0
lim r0 1
cmp r1
brc eq .halt
rld r1
brc even .divide

.x3plus1
adr r2 
adr r2
adr r2 
inc r2
rld r2
rst r1
brc jmp .loop

.divide
bsr 1
rst r1
brc jmp .loop

.halt
hlt