# Kitty Keyboard Protocol
## Basic CSI u encoding - simple key
```
<ESC>[97u
```

```
key(press:a)
```
---
## Basic CSI u encoding - uppercase A
```
<ESC>[65u
```

```
key(press:A)
```
---
## CSI u with shift modifier (2 = 1 + 0b1)
```
<ESC>[97;2u
```

```
key(press:shift-a)
```
---
## CSI u with alt modifier (3 = 1 + 0b10)
```
<ESC>[97;3u
```

```
key(press:alt-a)
```
---
## CSI u with ctrl modifier (5 = 1 + 0b100)
```
<ESC>[97;5u
```

```
key(press:ctrl-a)
```
---
## CSI u with ctrl+shift (6 = 1 + 0b101)
```
<ESC>[97;6u
```

```
key(press:ctrl-shift-a)
```
---
## CSI u with ctrl+alt (7 = 1 + 0b110)
```
<ESC>[97;7u
```

```
key(press:ctrl-alt-a)
```
---
## CSI u with ctrl+alt+shift (8 = 1 + 0b111)
```
<ESC>[97;8u
```

```
key(press:ctrl-alt-shift-a)
```
---
## CSI u with super modifier (9 = 1 + 0b1000)
```
<ESC>[97;9u
```

```
key(press:super-a)
```
---
## CSI u with hyper modifier (17 = 1 + 0b10000)
```
<ESC>[97;17u
```

```
key(press:hyper-a)
```
---
## CSI u with meta modifier (33 = 1 + 0b100000)
```
<ESC>[97;33u
```

```
key(press:meta-a)
```
---
## CSI u with ctrl+super (13 = 1 + 0b1100)
```
<ESC>[97;13u
```

```
key(press:ctrl-super-a)
```
---
## CSI u with ctrl+alt+shift+super (16 = 1 + 0b1111)
```
<ESC>[97;16u
```

```
key(press:ctrl-alt-shift-super-a)
```
---
## Escape key with CSI u encoding
```
<ESC>[27u
```

```
key(press:esc)
```
---
## Enter key with CSI u encoding
```
<ESC>[13u
```

```
key(press:enter)
```
---
## Tab key with CSI u encoding
```
<ESC>[9u
```

```
key(press:tab)
```
---
## Backspace with CSI u encoding
```
<ESC>[127u
```

```
key(press:backspace)
```
---
## Space with ctrl modifier
```
<ESC>[32;5u
```

```
key(press:ctrl-space)
```
---
## Function key F13
```
<ESC>[57376u
```

```
key(press:f13)
```
---
## Function key F14
```
<ESC>[57377u
```

```
key(press:f14)
```
---
## Function key F24
```
<ESC>[57387u
```

```
key(press:f24)
```
---
## Keypad Left
```
<ESC>[57417u
```

```
key(press:left:keypad)
```
---
## Keypad Right
```
<ESC>[57418u
```

```
key(press:right:keypad)
```
---
## Keypad Up
```
<ESC>[57419u
```

```
key(press:up:keypad)
```
---
## Keypad Down
```
<ESC>[57420u
```

```
key(press:down:keypad)
```
---
## Keypad Insert
```
<ESC>[57425u
```

```
key(press:insert:keypad)
```
---
## Keypad Delete
```
<ESC>[57426u
```

```
key(press:delete:keypad)
```
---
## Keypad Page Up
```
<ESC>[57421u
```

```
key(press:pageup:keypad)
```
---
## Keypad Page Down
```
<ESC>[57422u
```

```
key(press:pagedown:keypad)
```
---
## Keypad Home
```
<ESC>[57423u
```

```
key(press:home:keypad)
```
---
## Keypad End
```
<ESC>[57424u
```

```
key(press:end:keypad)
```
---
## Keypad 0
```
<ESC>[57399u
```

```
key(press:0:keypad)
```
---
## Keypad Enter
```
<ESC>[57414u
```

```
key(press:enter:keypad)
```
---
## Left Shift key press
```
<ESC>[57441u
```

```
key(press:shift-modifier:Left Shift)
```
---
## Left Control key press
```
<ESC>[57442u
```

```
key(press:ctrl-modifier:Left Control)
```
---
## Left Alt key press
```
<ESC>[57443u
```

```
key(press:alt-modifier:Left Option)
```
---
## Left Super key press
```
<ESC>[57444u
```

```
key(press:super-modifier:Left Command)
```
---
## Right Shift key press
```
<ESC>[57447u
```

```
key(press:shift-modifier:Right Shift)
```
---
## Right Control key press
```
<ESC>[57448u
```

```
key(press:ctrl-modifier:Right Control)
```
---
## CSI u with alternate key - shifted a becomes A
```
<ESC>[97:65;2u
```

```
key(press:A)
```
---
## CSI u with base layout key - Cyrillic layout
```
<ESC>[1057::99;5u
```

```
key(press:ctrl-С:base=c)
```
---
## CSI u with both shifted and base layout keys
```
<ESC>[97:65:99;2u
```

```
key(press:A:base=c)
```
---
## Press event explicitly specified
```
<ESC>[97;1:1u
```

```
key(press:a)
```
---
## Repeat event
```
<ESC>[97;1:2u
```

```
key(repeat:a)
```
---
## Release event
```
<ESC>[97;1:3u
```

```
key(release:a)
```
---
## Shift+a press event
```
<ESC>[97;2:1u
```

```
key(press:shift-a)
```
---
## Shift+a repeat event
```
<ESC>[97;2:2u
```

```
key(repeat:shift-a)
```
---
## Shift+a release event
```
<ESC>[97;2:3u
```

```
key(release:shift-a)
```
---
## Ctrl+c release event
```
<ESC>[99;5:3u
```

```
key(release:ctrl-c)
```
---
## Keypad Left press with modifiers
```
<ESC>[57417;5:1u
```

```
key(press:ctrl-left:keypad)
```
---
## Keypad Left repeat with modifiers
```
<ESC>[57417;5:2u
```

```
key(repeat:ctrl-left:keypad)
```
---
## Keypad Left release with modifiers
```
<ESC>[57417;5:3u
```

```
key(release:ctrl-left:keypad)
```
---
## CSI u with text as codepoints - shift+a produces A (65)
```
<ESC>[97;2;;65u
```

```
key(press:shift-a:text="A")
```
---
## CSI u with text - option+a produces å (229)
```
<ESC>[97;;;229u
```

```
key(press:a:text="å")
```
---
## CSI u with multiple text codepoints
```
<ESC>[97;;;65:66u
```

```
key(press:a:text="AB")
```
---
## Key with no keycode but text (composition)
```
<ESC>[0;;;229u
```

```
key(press::text="å")
```
---
## Caps Lock on (modifier bit 64 = 0b1000000, so 65 = 1 + 64)
```
<ESC>[97;65u
```

```
key(press:a:caps_lock)
```
---
## Num Lock on (modifier bit 128 = 0b10000000, so 129 = 1 + 128)
```
<ESC>[57399;129u
```

```
key(press:0:keypad:num_lock)
```
---
## Caps Lock + Shift (1 + 64 + 1 = 66)
```
<ESC>[97;66u
```

```
key(press:shift-a:caps_lock)
```
---
## Ctrl+Shift+A with caps lock (1 + 4 + 1 + 64 = 70)
```
<ESC>[97;70u
```

```
key(press:ctrl-shift-a:caps_lock)
```
---
## Legacy compatibility - Ctrl+Shift+A before disambiguation
```
<ESC>[65;6u
```

```
key(press:ctrl-shift-A)
```
---
## F1 with shift
```
<ESC>[57344;2u
```

```
key(press:shift-f1)
```
---
## F5 with ctrl+alt
```
<ESC>[57348;7u
```

```
key(press:ctrl-alt-f5)
```
---
## Keypad 5 with num lock
```
<ESC>[57404;129u
```

```
key(press:5:keypad:num_lock)
```
---
## Media Play key
```
<ESC>[57428u
```

```
key(press:media:Play)
```
---
## Media Pause key
```
<ESC>[57429u
```

```
key(press:media:Pause)
```
---
## Volume Up key
```
<ESC>[57439u
```

```
key(press:media:Raise Volume)
```
---
## Volume Down key
```
<ESC>[57438u
```

```
key(press:media:Lower Volume)
```
---
## Mute key
```
<ESC>[57440u
```

```
key(press:media:Mute Volume)
```
---
## Left Shift press with shift modifier set
```
<ESC>[57441;2u
```

```
key(press:shift-modifier:Left Shift)
```
---
## Left Control press with ctrl modifier set
```
<ESC>[57442;5u
```

```
key(press:ctrl-modifier:Left Control)
```
---
## Left Control release with ctrl modifier cleared
```
<ESC>[57442;1:3u
```

```
key(release:ctrl-modifier:Left Control)
```
---
## Right Alt press with alt modifier set
```
<ESC>[57449;3u
```

```
key(press:alt-modifier:Right Option)
```
---
## Ctrl+[ (Escape) with CSI u
```
<ESC>[27;5u
```

```
key(press:ctrl-esc)
```
---
## Ctrl+Space (null)
```
<ESC>[32;5u
```

```
key(press:ctrl-space)
```
---
## Alt+Escape
```
<ESC>[27;3u
```

```
key(press:alt-esc)
```
---
## Super+a (Windows/Command key)
```
<ESC>[97;9u
```

```
key(press:super-a)
```
---
## Hyper+Meta+a
```
<ESC>[97;49u
```

```
key(press:hyper-meta-a)
```
---
## All modifiers except locks on 'a' (1+1+2+4+8+16+32=64, so 65)
```
<ESC>[97;64u
```

```
key(press:ctrl-alt-shift-super-hyper-meta-a)
```
---
## Shift+Tab (BackTab)
```
<ESC>[9;2u
```

```
key(press:backtab)
```
---
## Ctrl+BackTab
```
<ESC>[9;6u
```

```
key(press:ctrl-backtab)
```
---
## Escape key press event
```
<ESC>[27;1:1u
```

```
key(press:esc)
```
---
## Escape key release event
```
<ESC>[27;1:3u
```

```
key(release:esc)
```
---
## ISO Level3 Shift key
```
<ESC>[57453u
```

```
key(press:modifier:Iso Level 3 Shift)
```
---
## ISO Level5 Shift key
```
<ESC>[57454u
```

```
key(press:modifier:Iso Level 5 Shift)
```
---
## Caps Lock key press
```
<ESC>[57358u
```

```
key(press:capslock)
```
---
## Scroll Lock key
```
<ESC>[57359u
```

```
key(press:scrolllock)
```
---
## Num Lock key
```
<ESC>[57360u
```

```
key(press:numlock)
```
---
## Print Screen key
```
<ESC>[57361u
```

```
key(press:printscreen)
```
---
## Pause key
```
<ESC>[57362u
```

```
key(press:pause)
```
---
## Menu key
```
<ESC>[57363u
```

```
key(press:menu)
```
---
## F15
```
<ESC>[57378u
```

```
key(press:f15)
```
---
## F20
```
<ESC>[57383u
```

```
key(press:f20)
```
---
## F35
```
<ESC>[57398u
```

```
key(press:f35)
```
---
## Keypad Begin
```
<ESC>[57427u
```

```
key(press:keypadbegin:keypad)
```
---
## Keypad Decimal (period)
```
<ESC>[57409u
```

```
key(press:.:keypad)
```
---
## Keypad Divide (slash)
```
<ESC>[57410u
```

```
key(press:/:keypad)
```
---
## Keypad Multiply (asterisk)
```
<ESC>[57411u
```

```
key(press:*:keypad)
```
---
## Keypad Subtract (minus)
```
<ESC>[57412u
```

```
key(press:-:keypad)
```
---
## Keypad Add (plus)
```
<ESC>[57413u
```

```
key(press:+:keypad)
```
---
## Keypad Equal
```
<ESC>[57415u
```

```
key(press:=:keypad)
```
---
## Keypad Separator (comma)
```
<ESC>[57416u
```

```
key(press:,:keypad)
```
---
## Multiple modifier keys - Ctrl+Alt+Shift+Super+a (1+1+2+4+8=16, so 17)
```
<ESC>[97;17u
```

```
key(press:hyper-a)
```
---
## Test uppercase letter Z with shift
```
<ESC>[122:90;2u
```

```
key(press:Z)
```
---
## Test number 1 with shift (becomes !)
```
<ESC>[49:33;2u
```

```
key(press:!)
```
---
## Test equals with shift (becomes +)
```
<ESC>[61:43;2u
```

```
key(press:+)
```
---
## Ctrl+Shift+equals (for ctrl+plus shortcut matching)
```
<ESC>[61;6u
```

```
key(press:ctrl-shift-=)
```
---
## Alt+[ (not CSI, using CSI u encoding)
```
<ESC>[91;3u
```

```
key(press:alt-[)
```
---
## Alt+]
```
<ESC>[93;3u
```

```
key(press:alt-])
```
---
## Ctrl+\ (FS)
```
<ESC>[92;5u
```

```
key(press:ctrl-\)
```
---
## Ctrl+]
```
<ESC>[93;5u
```

```
key(press:ctrl-])
```
---
## Press '/' with no modifiers
```
<ESC>[47u
```

```
key(press:/)
```
---
## Ctrl+/
```
<ESC>[47;5u
```

```
key(press:ctrl-/)
```
---
## Shift+/ (becomes ?)
```
<ESC>[47:63;2u
```

```
key(press:?)
```
---
## Space bar press event
```
<ESC>[32;1:1u
```

```
key(press:space)
```
---
## Space bar release event
```
<ESC>[32;1:3u
```

```
key(release:space)
```
---
## Enter with alt
```
<ESC>[13;3u
```

```
key(press:alt-enter)
```
---
## Backspace with ctrl
```
<ESC>[127;5u
```

```
key(press:ctrl-backspace)
```
---
## Tab with alt
```
<ESC>[9;3u
```

```
key(press:alt-tab)
```
---
## Test digit 0
```
<ESC>[48u
```

```
key(press:0)
```
---
## Test digit 9
```
<ESC>[57u
```

```
key(press:9)
```
---
## Semicolon key
```
<ESC>[59u
```

```
key(press:;)
```
---
## Semicolon with shift (colon)
```
<ESC>[59:58;2u
```

```
key(press::)
```
---
## Comma key
```
<ESC>[44u
```

```
key(press:,)
```
---
## Comma with shift (less-than)
```
<ESC>[44:60;2u
```

```
key(press:<)
```
---
## Period key
```
<ESC>[46u
```

```
key(press:.)
```
---
## Period with shift (greater-than)
```
<ESC>[46:62;2u
```

```
key(press:>)
```
---
## Backtick key
```
<ESC>[96u
```

```
key(press:`)
```
---
## Backtick with shift (tilde)
```
<ESC>[96:126;2u
```

```
key(press:~)
```
---
## Minus key
```
<ESC>[45u
```

```
key(press:-)
```
---
## Minus with shift (underscore)
```
<ESC>[45:95;2u
```

```
key(press:_)
```
---
## Left bracket
```
<ESC>[91u
```

```
key(press:[)
```
---
## Left bracket with shift (left brace)
```
<ESC>[91:123;2u
```

```
key(press:{)
```
---
## Right bracket
```
<ESC>[93u
```

```
key(press:])
```
---
## Right bracket with shift (right brace)
```
<ESC>[93:125;2u
```

```
key(press:})
```
---
## Backslash
```
<ESC>[92u
```

```
key(press:\)
```
---
## Backslash with shift (pipe)
```
<ESC>[92:124;2u
```

```
key(press:|)
```
---
## Single quote
```
<ESC>[39u
```

```
key(press:')
```
---
## Single quote with shift (double quote)
```
<ESC>[39:34;2u
```

```
key(press:")
```
---
## Legacy F1 with modifiers (SS3 P)
```
<ESC>OP
```

```
key(press:f1)
```
---
## Legacy F2 (SS3 Q)
```
<ESC>OQ
```

```
key(press:f2)
```
---
## Legacy F3 (SS3 R)
```
<ESC>OR
```

```
key(press:f3)
```
---
## Legacy F4 (SS3 S)
```
<ESC>OS
```

```
key(press:f4)
```
---
## Legacy F1 with shift (CSI 1;2P)
```
<ESC>[1;2P
```

```
key(press:shift-f1)
```
---
## Legacy F2 with ctrl (CSI 1;5Q)
```
<ESC>[1;5Q
```

```
key(press:ctrl-f2)
```
---
## Legacy Home (CSI H)
```
<ESC>[H
```

```
key(press:home)
```
---
## Legacy End (CSI F)
```
<ESC>[F
```

```
key(press:end)
```
---
## Legacy Home with shift (CSI 1;2H)
```
<ESC>[1;2H
```

```
key(press:shift-home)
```
---
## Legacy Page Up with ctrl (CSI 5;5~)
```
<ESC>[5;5~
```

```
key(press:ctrl-pageup)
```
---
## Legacy Insert (CSI 2~)
```
<ESC>[2~
```

```
key(press:insert)
```
---
## Legacy Delete with alt (CSI 3;3~)
```
<ESC>[3;3~
```

```
key(press:alt-delete)
```
---
## Escape sequence for Ctrl+I (same as Tab)
```
<HT>
```

```
key(press:tab)
```
---
## Escape sequence for Ctrl+M (same as Enter)
```
<CR>
```

```
key(press:enter)
```
---
## Multiple keys in sequence
```
<ESC>[97u<ESC>[98u<ESC>[99u
```

```
key(press:a)
key(press:b)
key(press:c)
```
---
## Modifier key followed by regular key
```
<ESC>[57441;2u<ESC>[97;2u
```

```
key(press:shift-modifier:Left Shift)
key(press:shift-a)
```
---
## Repeat events for same key
```
<ESC>[97;1:1u<ESC>[97;1:2u<ESC>[97;1:2u<ESC>[97;1:3u
```

```
key(press:a)
key(repeat:a)
key(repeat:a)
key(release:a)
```
---
## Base layout key matching - Cyrillic С maps to Latin c
```
<ESC>[1057::99;5u
```

```
key(press:ctrl-С:base=c)
```
---
## Base layout key matching - Greek α maps to Latin a
```
<ESC>[945::97;5u
```

```
key(press:ctrl-α:base=a)
```
---
## Base layout key matching - Hebrew ש maps to Latin a
```
<ESC>[1513::97;5u
```

```
key(press:ctrl-ש:base=a)
```
---
## Base layout key with shift - Cyrillic С (uppercase)
```
<ESC>[1057:1057:99;6u
```

```
key(press:ctrl-С:base=c)
```
---
## Base layout for number keys - French keyboard 1 with shift (!)
```
<ESC>[49:33:49;2u
```

```
key(press:!:base=1)
```
---
## Base layout for special keys - German keyboard Z key (maps to Y in QWERTY)
```
<ESC>[122::121;5u
```

```
key(press:ctrl-z:base=y)
```
---
## Base layout with function key - keypad key mapping
```
<ESC>[57399::48u
```

```
key(press:0:keypad:base=0)
```
---
## Complex case - alt+ctrl+shift with base layout
```
<ESC>[1057:1057:99;8u
```

```
key(press:ctrl-alt-С:base=c)
```
---
## Base layout only (empty shifted field) - Cyrillic в maps to Latin d
```
<ESC>[1074::100;5u
```

```
key(press:ctrl-в:base=d)
```
---
## Base layout with no modifiers
```
<ESC>[1057::99u
```

```
key(press:С:base=c)
```
---
## Multiple alternates with all fields populated
```
<ESC>[97:65:99;2u
```

```
key(press:A:base=c)
```
---
## Text as Codepoints - Basic single character
```
<ESC>[97;2;;65u
```

```
key(press:shift-a:text="A")
```
---
## Text as Codepoints - Shift+a with text "A"
```
<ESC>[97;2;;65u
```

```
key(press:shift-a:text="A")
```
---
## Text as Codepoints - Multiple codepoints (e.g., composed character)
```
<ESC>[97;5;;195:161u
```

```
key(press:ctrl-a:text="Ã¡")
```
---
## Text as Codepoints - Emoji
```
<ESC>[97;2;;128512u
```

```
key(press:shift-a:text="😀")
```
---
## Text as Codepoints - Empty text field
```
<ESC>[97;2;;u
```

```
key(press:shift-a)
```
---
## Text as Codepoints - Ctrl+a with text (C0 control)
```
<ESC>[97;5;;1u
```

```
key(press:ctrl-a:text="\u{1}")
```
---
## Text as Codepoints - Complex: base layout + text
```
<ESC>[1057::99;2;;1057u
```

```
key(press:shift-С:base=c:text="С")
```
---
## Text as Codepoints - Multiple characters in text
```
<ESC>[97;2;;65:66:67u
```

```
key(press:shift-a:text="ABC")
```
---
## Protocol Negotiation - Response with flags (disambiguate escape codes)
```
<ESC>[?1u
```

```
KeyboardEnhancementFlags(KeyboardEnhancementFlags(DISAMBIGUATE_ESCAPE_CODES))
```
---
## Protocol Negotiation - Response with flags (disambiguate + report event types)
```
<ESC>[?3u
```

```
KeyboardEnhancementFlags(KeyboardEnhancementFlags(DISAMBIGUATE_ESCAPE_CODES | REPORT_EVENT_TYPES))
```
---
## Protocol Negotiation - Response with flags (all enhancements)
```
<ESC>[?31u
```

```
KeyboardEnhancementFlags(KeyboardEnhancementFlags(DISAMBIGUATE_ESCAPE_CODES | REPORT_EVENT_TYPES | REPORT_ALTERNATE_KEYS | REPORT_ALL_KEYS_AS_ESCAPE_CODES | REPORT_ASSOCIATED_TEXT))
```
---
