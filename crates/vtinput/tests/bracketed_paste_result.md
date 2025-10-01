# Bracketed Paste
## Simple bracketed paste with ASCII text
```
<ESC>[200~hello<ESC>[201~
```

```
Paste(hello)
```
---
## Bracketed paste with UTF-8 (café 😀)
```
<ESC>[200~café 😀<ESC>[201~
```

```
Paste(café 😀)
```
---
## Bracketed paste with newlines
```
<ESC>[200~line1<LF>line2<CR><LF>line3<ESC>[201~
```

```
Paste(line1<LF>line2<CR><LF>line3)
```
---
## Events before and after bracketed paste
```
A<ESC>[200~pasted<ESC>[201~B
```

```
key(press:shift-A)
Paste(pasted)
key(press:shift-B)
```
---
## Empty bracketed paste
```
<ESC>[200~<ESC>[201~
```

```
Paste()
```
---
## Bracketed paste with only whitespace
```
<ESC>[200~   <TAB>  <LF><ESC>[201~
```

```
Paste(   <TAB>  <LF>)
```
---
## Multiple bracketed pastes in sequence
```
<ESC>[200~first<ESC>[201~<ESC>[200~second<ESC>[201~
```

```
Paste(first)
Paste(second)
```
---
## Bracketed paste end without start
```
<ESC>[201~
```

```
LowLevel(Csi('201', '', '~'))
```
---
## Multiple consecutive bracketed pastes
```
<ESC>[200~one<ESC>[201~<ESC>[200~two<ESC>[201~<ESC>[200~three<ESC>[201~
```

```
Paste(one)
Paste(two)
Paste(three)
```
---
## Bracketed paste with special characters
```
<ESC>[200~!@#$%^&*()<ESC>[201~
```

```
Paste(!@#$%^&*())
```
---
## Bracketed paste with tabs
```
<ESC>[200~<TAB>text<TAB>more<ESC>[201~
```

```
Paste(<TAB>text<TAB>more)
```
---
## Bracketed paste with control characters
```
<ESC>[200~<TAB>text<CR><LF>more<ESC>[201~
```

```
Paste(<TAB>text<CR><LF>more)
```
---
## Bracketed paste with backslash and quotes
```
<ESC>[200~path\to\file.txt "quoted"<ESC>[201~
```

```
Paste(path\to\file.txt "quoted")
```
---
## Bracketed paste with null bytes
```
<ESC>[200~text<NUL>with<NUL>nulls<ESC>[201~
```

```
Paste(text<NUL>with<NUL>nulls)
```
---
## Long bracketed paste
```
<ESC>[200~The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs.<ESC>[201~
```

```
Paste(The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs.)
```
---
