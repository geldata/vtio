# UTF-8 Handling
## Lowercase characters (no SHIFT modifier)
```
hello
```

```
key(press:h)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
```
---
## Uppercase characters (SHIFT modifier)
```
HELLO
```

```
key(press:shift-H)
key(press:shift-E)
key(press:shift-L)
key(press:shift-L)
key(press:shift-O)
```
---
## Mixed case
```
HeLLo
```

```
key(press:shift-H)
key(press:e)
key(press:shift-L)
key(press:shift-L)
key(press:o)
```
---
## Numbers
```
0123456789
```

```
key(press:0)
key(press:1)
key(press:2)
key(press:3)
key(press:4)
key(press:5)
key(press:6)
key(press:7)
key(press:8)
key(press:9)
```
---
## Special ASCII characters
```
!@#$%^&*()
```

```
key(press:!)
key(press:@)
key(press:#)
key(press:$)
key(press:%)
key(press:^)
key(press:&)
key(press:*)
key(press:()
key(press:))
```
---
## Valid 2-byte UTF-8 (café)
```
café
```

```
key(press:c)
key(press:a)
key(press:f)
key(press:é)
```
---
## Valid 3-byte UTF-8 (€100)
```
€100
```

```
key(press:€)
key(press:1)
key(press:0)
key(press:0)
```
---
## Valid 4-byte UTF-8 (😀)
```
😀
```

```
key(press:😀)
```
---
## Multiple emoji
```
😀😁
```

```
key(press:😀)
key(press:😁)
```
---
## Mixed ASCII and UTF-8
```
Hello 😀 World
```

```
key(press:shift-H)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
key(press:space)
key(press:😀)
key(press:space)
key(press:shift-W)
key(press:o)
key(press:r)
key(press:l)
key(press:d)
```
---
## UTF-8 accented characters
```
àéîôù
```

```
key(press:à)
key(press:é)
key(press:î)
key(press:ô)
key(press:ù)
```
---
## UTF-8 German characters
```
äöüßÄÖÜ
```

```
key(press:ä)
key(press:ö)
key(press:ü)
key(press:ß)
key(press:shift-Ä)
key(press:shift-Ö)
key(press:shift-Ü)
```
---
## UTF-8 Greek characters
```
αβγδε
```

```
key(press:α)
key(press:β)
key(press:γ)
key(press:δ)
key(press:ε)
```
---
## UTF-8 Cyrillic characters
```
абвгд
```

```
key(press:а)
key(press:б)
key(press:в)
key(press:г)
key(press:д)
```
---
## UTF-8 Chinese characters
```
你好世界
```

```
key(press:你)
key(press:好)
key(press:世)
key(press:界)
```
---
## UTF-8 Japanese characters
```
こんにちは
```

```
key(press:こ)
key(press:ん)
key(press:に)
key(press:ち)
key(press:は)
```
---
## UTF-8 with newlines
```
Hello<LF>café
```

```
key(press:shift-H)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
key(press:
)
key(press:c)
key(press:a)
key(press:f)
key(press:é)
```
---
## Mixed regular and UTF-8
```
Hello café 世界 😀
```

```
key(press:shift-H)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
key(press:space)
key(press:c)
key(press:a)
key(press:f)
key(press:é)
key(press:space)
key(press:世)
key(press:界)
key(press:space)
key(press:😀)
```
---
## Invalid UTF-8: start byte 0xFF (invalid in UTF-8)
```
H<ff>i
```

```
key(press:shift-H)
key(press:ctrl-l)
key(press:i)
```
---
## Invalid UTF-8: continuation byte 0x80 without start
```
A<80>B
```

```
key(press:shift-A)
key(press:shift-B)
```
---
## Invalid UTF-8: incomplete 2-byte sequence (0xC3 alone)
```
AB<c3>
```

```
key(press:shift-A)
key(press:shift-B)
```
---
## Invalid UTF-8: incomplete 3-byte sequence (0xE2 0x82 alone)
```
<e2><82>
```

```
```
---
## Invalid UTF-8: incomplete 4-byte sequence (0xF0 0x9F 0x98 alone)
```
<f0><9f><98>
```

```
```
---
## Invalid UTF-8: invalid continuation in 2-byte (0xC3 0x28)
```
<c3><28>
```

```
key(press:()
```
---
## Invalid UTF-8: overlong encoding (0xC0 0xAF for '/')
```
<c0><af>
```

```
```
---
## Invalid UTF-8: surrogate half (0xED 0xA0 0x80)
```
<ed><a0><80>
```

```
```
---
## Mixed valid and invalid UTF-8
```
A<ff>Bcafé<80>D
```

```
key(press:shift-A)
key(press:ctrl-l)
key(press:shift-B)
key(press:c)
key(press:a)
key(press:f)
key(press:é)
key(press:shift-D)
```
---
## Invalid UTF-8 followed by valid
```
<ff>hello
```

```
key(press:ctrl-l)
key(press:h)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
```
---
## Valid UTF-8 followed by invalid
```
world<80>
```

```
key(press:w)
key(press:o)
key(press:r)
key(press:l)
key(press:d)
```
---
## Multiple invalid bytes in sequence
```
<ff><fe><fd>
```

```
key(press:ctrl-l)
```
---
## Invalid in middle of valid text
```
Hello<c3>World
```

```
key(press:shift-H)
key(press:e)
key(press:l)
key(press:l)
key(press:o)
key(press:shift-W)
key(press:o)
key(press:r)
key(press:l)
key(press:d)
```
---
