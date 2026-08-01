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

This will create a project with a room and an object, add the object to the room, and add a Create event to the object that shows a debug message.

## Core language
- [x] Variable assignments
- [x] Binary operations
- [x] If/else statements
- [x] Function calls
- [x] Strings
- [x] Unary operators (`!`, `-`, `~`)
- [ ] Ternary operator (`?:`)
- [ ] Loops (`while`, `repeat`, `for`, `do...until`)
- [ ] `switch` / `case`
- [ ] `break`
- [ ] `continue`
- [ ] `return`

## Data types
- [ ] Arrays
- [ ] Structs
- [ ] Array indexing (`a[i]`)
- [ ] Nested arrays
- [ ] Struct member access (`obj.x`)
- [ ] Chained access (`a.b.c`)
- [ ] Array/struct literals

## Variables
- [x] Local variables
- [ ] Global variables
- [ ] Instance variables
- [ ] Static variables
- [ ] Variable declarations (`var`, `globalvar`, `static`)

## Operators
- [x] Compound assignments (`+=`, `-=`, `*=`, etc.)
- [x] Increment/decrement (`++`, `--`)
- [x] Bitwise operators
- [x] Logical operators
- [x] Comparison operators
- [x] Modulo
- [x] Shift operators (`<<`, `>>`)

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