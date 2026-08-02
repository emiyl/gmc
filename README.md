# GMLC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can generate data.win files using a command like this:

```bash
gmlc --create Project \
    --add-resource room Room1 \
    --add-resource object Object1 \
    --add-object-to-room Room1 Object1 100 120 \
    --add-event Object1 Create 0 "show_debug_message(\"Hello, world!\");" \
    --compile ./data.win
```

This will create a project with a room and an object, add the object to the room, and add a Create event to the object that shows a debug message. You can also use a .gml file for the event code instead of a string.

## Core language
- [x] Variable assignments
- [x] Binary operations
- [x] If/else statements
- [x] Function calls
- [x] Strings
- [x] Unary operators (`!`, `-`, `~`)
- [x] Ternary operator (`?:`)
- [x] Loops (`while`, `repeat`, `for`, `do...until`)
- [x] `switch` / `case`
- [x] `break`
- [x] `continue`
- [x] `return`

## Data types
- [x] Arrays
- [x] Structs
- [x] Array indexing (`a[i]`)
- [x] Nested arrays
- [x] Struct member access (`obj.x`)
- [x] Chained access (`a.b.c`)
- [x] Array/struct literals

## Operators
- [x] Compound assignments (`+=`, `-=`, `*=`, etc.)
- [x] Increment/decrement (`++`, `--`)
- [x] Bitwise operators
- [x] Logical operators
- [x] Comparison operators
- [x] Modulo
- [x] Shift operators (`<<`, `>>`)

## Variables
- [x] Local variables
- [x] Global variables
- [x] Instance variables
- [x] Static variables
- [x] Variable declarations (`var`, `globalvar`, `static`)

## Functions
- [ ] Function declarations
- [ ] Anonymous functions
- [ ] Default arguments
- [ ] Named arguments
- [ ] Closures

## Object access
- [ ] `self`
- [ ] `other`
- [ ] `global`
- [ ] `noone`
- [ ] `with`
- [ ] `enum`