# How GameMaker Handles Variables

## Introduction

Variables are one of the fundamental parts of GameMaker Language (GML). Although they appear to behave like variables in many other scripting languages, GameMaker has several unique implementation details involving scopes, instance lookup, dynamic typing, and variable references.

This document describes how variables are represented, resolved, and manipulated by the GameMaker runtime.

## Dynamic Typing

GameMaker variables are dynamically typed.
A variable does not permanently have a type. Instead, every stored value carries its own type information.

For example:

```
x = 5;
x = "Hello";
x = [1, 2, 3];
```

The same variable can store:

- integers
- reals
- strings
- arrays
- structs
- objects
- methods
- asset references
- pointers to other runtime objects

The compiler therefore does not assign a fixed type to variables.
## Variable Names
Variable names are stored separately from the code.

During compilation, every unique variable name is added to a global string table.

For example:

```
health = 100;
score = 0;
health += 5;
```

The string table only contains:

- health
- score

Instructions reference these names through IDs rather than embedding text directly.

## Variable References

Compiled bytecode does not directly store strings for variables. Instead, instructions contain a variable reference. Conceptually:

```
Push Variable "health"
```

becomes something similar to:

```
Push.v variable_ref(health)
```

Internally this reference identifies:
- variable name
- variable category
- sometimes the owning scope

Older GameMaker runtimes encode this as a 32-bit value.

## Variable Categories

Internally variables belong to several categories.

### Normal variables
Variables created by user code.
```
score = 10;
```
### Built-in variables
Engine-defined variables

Examples:
- `x`
- `y`
- `image_xscale`
- `speed`
- `alarm`
- `direction`
- `room_speed`

These behave like ordinary variables in GML but are backed directly by engine data.

Reading `x` retrieves the instance position.

Writing `x` changes the instance position.

### Global variables
Stored in a global namespace.
```
global.score = 100;
```
Only one copy exists regardless of how many instances exist.
### Instance variables
Each instance owns its own variable table.

Player
```gml
health = 100;
```

Enemy
```gml
health = 50;
```
Both variables have the same name but exist in different instances.
### Local variables
Declared using `var`.
```gml
var temp = 5;
```
These exist only while the current script or event executes.
The compiler usually allocates local slots for these.
### Static variables
Declared using `static`.
```gml
static counter = 0;
```
A static variable is shared across every invocation of that function.
It persists for the lifetime of the game.
## Variable Resolution
When the compiler encounters:
```
health += 5;
```
it records only that the code accesses the variable named health. The runtime later decides which variable this actually refers to.

Typical lookup order is:
1. local variable
2. current instance variable
3. inherited instance variable
4. built-in variable
5. global namespace (when explicitly referenced)

The exact lookup rules depend on the expression.
## Instance Variable Lookup
Variables may be accessed through another instance.
```
player.health
enemy.x
```
The runtime:
1. evaluates the left side
2. finds the referenced instance
3. searches that instance's variable table
4. returns the value

## The `with` Statement

Inside a `with` block, the current instance changes.
```
with (enemy)
{
    health = 0;
}
```
`health` now refers to the enemy instance rather than the caller.

The original instance remains available through other.

## Variable Creation

Assigning to an unknown variable creates it automatically.
```
coins = 5;
```

No declaration is required.

The variable is inserted into the current scope.

## Dynamic Variables

Variables may be created at runtime.

```
variable_instance_set(id, "health", 100);
```

The runtime performs a lookup using the string `"health"`.

Likewise:
```
variable_instance_get(id, "health");
```

retrieves the variable dynamically.

## Arrays

Arrays are stored as values inside variables.
```
inventory = [1, 2, 3];
```
The variable stores a reference to the array object.

The array itself is allocated elsewhere.

## Structs
Structs are heap-allocated objects.

```
player = {
    hp: 100,
    mana: 50
};
```

The variable stores a reference to the struct.

Member access:
```
player.hp
```

performs a lookup inside the struct.

## Function Variables

Functions are first-class values.
```
f = show_message;
f("Hello");
```

The variable stores a function reference. Methods additionally capture an instance binding.

## Built-in Variables

Many built-in variables map directly to engine state.
For example:
```
x
```
may internally read:
```
instance->x
```
Similarly:
```
image_angle
```
may read:
```
instance->image_angle
```
The runtime therefore does not need to store these inside the ordinary dynamic variable table.

## Access Instructions

Typical bytecode operations include:
- Push Variable
- Pop Variable
- Duplicate
- Push Immediate

For example:
```
score = 10;
```
roughly becomes:
```
PushI 10
Pop score
```
Reading:
```
show_debug_message(score);
```
becomes approximately:
```
Push score
Call show_debug_message
```

## Variable Lifetime

Different variable types have different lifetimes.
| Variable | Lifetime |
|----------|----------|
| Local    | Until function/event exits |
| Instance | Until instance is destroyed |
| Global   | Entire game |
| Static   | Entire game |
| Built-in | Lifetime of owning engine object |

## Memory Representation

Each runtime value typically stores:
- value type
- numeric value or pointer
- ownership/reference information

Variables themselves generally store these runtime values rather than raw integers or floating-point numbers.

## Summary

GameMaker variables are dynamically typed values identified by names rather than fixed memory locations.

During compilation, variable names become references used by bytecode instructions. At runtime, those references are resolved according to the current execution context (such as locals, instances, globals, or built-in variables). Instance variables live inside each object instance, global variables exist in a shared namespace, and built-in variables provide direct access to engine-managed state.
This design allows GML to support dynamic variable creation, runtime reflection, first-class functions, structs, arrays, and flexible instance scoping while maintaining compact compiled bytecode.