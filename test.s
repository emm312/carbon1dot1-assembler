LIR R3 0 // X
PLD $0
RST R1 // Slope
PLD $1
RST R2 // Y intercept
.loop
RLD R1 // Copy slope
RST R4 // to R4
.inner_loop
RLD R2
PST $6 // Port Y
INC R2
DEC R4
BRC NEQ .inner_loop
INC R3
RLD R3
PST $7 // Port X
BRC JMP .loop
