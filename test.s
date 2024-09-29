FUNC .multiplication
  LIR R3 0
  PLD $6
  RST R2
  PLD $7
  RST R1
  BRC EQ .end
  .loop
  BRC EVEN .l2
  RLD R3
  ADD R2
  RST R3
  .l2
  PST $0
  RLD R2
  RST R2
  RLD R1
  RST R1
  BRC NEQ .loop
  .end
  RET
END

CAL .multiplication
"test"
