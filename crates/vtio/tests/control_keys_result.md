# Control Keys
## Tab key
```
<TAB>
```

```
key(press:tab)
```
---
## Enter key
```
<CR>
```

```
key(press:enter)
```
---
## Ctrl+Space (null byte)
```
<NUL>
```

```
key(press:ctrl-space)
```
---
## Ctrl+A
```
<SOH>
```

```
key(press:ctrl-a)
```
---
## Ctrl+B
```
<STX>
```

```
key(press:ctrl-b)
```
---
## Ctrl+C
```
<ETX>
```

```
key(press:ctrl-c)
```
---
## Ctrl+D
```
<EOT>
```

```
key(press:ctrl-d)
```
---
## Ctrl+E
```
<ENQ>
```

```
key(press:ctrl-e)
```
---
## Ctrl+F
```
<ACK>
```

```
key(press:ctrl-f)
```
---
## Ctrl+G (bell)
```
<BEL>
```

```
key(press:ctrl-g)
```
---
## Ctrl+H (backspace)
```
<BS>
```

```
key(press:backspace)
```
---
## Ctrl+I (tab - same as TAB)
```
<HT>
```

```
key(press:tab)
```
---
## Ctrl+J (line feed)
```
<LF>
```

```
key(press:
)
```
---
## Ctrl+K
```
<VT>
```

```
key(press:ctrl-k)
```
---
## Ctrl+L (form feed)
```
<FF>
```

```
key(press:ctrl-l)
```
---
## Ctrl+M (carriage return - becomes Enter)
```
<CR>
```

```
key(press:enter)
```
---
## Ctrl+N
```
<SO>
```

```
key(press:ctrl-n)
```
---
## Ctrl+O
```
<SI>
```

```
key(press:ctrl-o)
```
---
## Ctrl+P
```
<DLE>
```

```
key(press:ctrl-p)
```
---
## Ctrl+Q
```
<DC1>
```

```
key(press:ctrl-q)
```
---
## Ctrl+R
```
<DC2>
```

```
key(press:ctrl-r)
```
---
## Ctrl+S
```
<DC3>
```

```
key(press:ctrl-s)
```
---
## Ctrl+T
```
<DC4>
```

```
key(press:ctrl-t)
```
---
## Ctrl+U
```
<NAK>
```

```
key(press:ctrl-u)
```
---
## Ctrl+V
```
<SYN>
```

```
key(press:ctrl-v)
```
---
## Ctrl+W
```
<ETB>
```

```
key(press:ctrl-w)
```
---
## Ctrl+X
```
<CAN>
```

```
key(press:ctrl-x)
```
---
## Ctrl+Y
```
<EM>
```

```
key(press:ctrl-y)
```
---
## Ctrl+Z
```
<SUB>
```

```
key(press:ctrl-z)
```
---
## Ctrl+[ (same as ESC)
```
<ESC>
```

```
key(press:esc)
```
---
## Ctrl+\ (FS)
```
<FS>
```

```
key(press:ctrl-\)
```
---
## Ctrl+] (GS)
```
<GS>
```

```
key(press:ctrl-])
```
---
## Ctrl+^ (RS)
```
<RS>
```

```
key(press:ctrl-^)
```
---
## Ctrl+_ (US)
```
<US>
```

```
key(press:ctrl-_)
```
---
## ESC ESC
```
<ESC><ESC>
```

```
key(press:esc)
key(press:esc)
```
---
## Alt+[ (not CSI, just ESC + [)
```
<ESC>[
```

```
```
---
## Multiple control characters in sequence
```
<SOH><STX><ETX>
```

```
key(press:ctrl-a)
key(press:ctrl-b)
key(press:ctrl-c)
```
---
## Mixed control and regular characters
```
A<SOH>B<STX>C
```

```
key(press:shift-A)
key(press:ctrl-a)
key(press:shift-B)
key(press:ctrl-b)
key(press:shift-C)
```
---
## Control characters with uppercase
```
<SOH>A<ETX>B
```

```
key(press:ctrl-a)
key(press:shift-A)
key(press:ctrl-c)
key(press:shift-B)
```
---
## Newline character
```
<LF>
```

```
key(press:
)
```
---
## Carriage return and line feed
```
<CR><LF>
```

```
key(press:enter)
key(press:
)
```
---
## Mixed newlines
```
text<LF>more<CR><LF>final
```

```
key(press:t)
key(press:e)
key(press:x)
key(press:t)
key(press:
)
key(press:m)
key(press:o)
key(press:r)
key(press:e)
key(press:enter)
key(press:
)
key(press:f)
key(press:i)
key(press:n)
key(press:a)
key(press:l)
```
---
## Vertical tab
```
<VT>
```

```
key(press:ctrl-k)
```
---
## Multiple tabs
```
<TAB><TAB><TAB>
```

```
key(press:tab)
key(press:tab)
key(press:tab)
```
---
## Tab separated values
```
one<TAB>two<TAB>three
```

```
key(press:o)
key(press:n)
key(press:e)
key(press:tab)
key(press:t)
key(press:w)
key(press:o)
key(press:tab)
key(press:t)
key(press:h)
key(press:r)
key(press:e)
key(press:e)
```
---
## Null bytes in text
```
text<NUL>with<NUL>nulls
```

```
key(press:t)
key(press:e)
key(press:x)
key(press:t)
key(press:ctrl-space)
key(press:w)
key(press:i)
key(press:t)
key(press:h)
key(press:ctrl-space)
key(press:n)
key(press:u)
key(press:l)
key(press:l)
key(press:s)
```
---
