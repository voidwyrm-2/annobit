# Annobit

Annobit is a simple tool that takes this
```
00 00 00 00
Operand B
Operand A
Flags
Opcode
```

And converts it to this
```
00 00 00 00
|  |  |  |
|  |  |  +--- Operand B
|  |  +------ Operand A
|  +--------- Flags
+------------ Opcode
```
