- match: \b(put|push|pop)\b
      scope: move.vasm

- match: \b(add|sub|mlt|div)\b
  scope: operator.vasm

- match: \b(r0|r1|r2|r3|r4|r5|r6|r7)\b
  scope: registers.vasm

- match: \b0x\w+
  scope: num.vasm

- match: \ba0x\w+
  scope: address.vasm

- match: \b(jmp|jmpif|return)\b
  scope: jmp.vasm

- match: \b(label)\b
  scope: label.vasm

- match: \b(hlt)\b
  scope: hlt.vasm