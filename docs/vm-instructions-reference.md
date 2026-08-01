# GameMaker bytecode instruction reference for compiler authors

This document is a compiler-oriented guide to the bytecode executed by the VM in [Butterscotch/src/vm.c](https://github.com/ButterscotchRunner/Butterscotch/blob/main/src/) and defined in [Butterscotch/src/vm.h](https://github.com/ButterscotchRunner/Butterscotch/blob/main/src/vm.h). It is intentionally implementation-focused: the goal is to explain how this runtime actually decodes and executes instructions so another agent can emit compatible bytecode.

## 1. Scope and assumptions

This guide is based on the runtime behavior in the repository, not on a generic GameMaker specification.

Important points:
- The VM is version-aware. Some semantics change across WAD versions, especially for arrays and BC17+ features.
- The bytecode format is a 32-bit word stream. Most instructions are 4 bytes long; some carry extra data after the instruction word.
- The runtime patches variable and function operands into direct indices during VM creation, so the emitted bytecode should still use the original reference-chain form when writing a compiler, and the loader/runtime will resolve it.
- The VM uses a typed stack with special handling for arrays, methods, structs, and variable references.

## 2. Instruction encoding model

### 2.1 Basic layout

Each instruction is a 32-bit word:

```text
+--------+--------+--------+--------+
| opcode | t1     | t2     | inst/operand |
+--------+--------+--------+--------+
  bits31..24  bits23..20 bits19..16 bits15..0
```

The relevant fields are:
- `opcode`: high byte (`instr >> 24`)
- `type1`: low nibble of bits 16..19 (`instr >> 16 & 0xF`)
- `type2`: low nibble of bits 20..23 (`instr >> 20 & 0xF`)
- `instanceType` / sub-opcode / immediate value: low 16 bits
- `extraData` flag: bit 30 (`0x40000000`) means the instruction has an operand payload after the instruction word

The runtime helpers are:
- `instrOpcode(instr)`
- `instrType1(instr)`
- `instrType2(instr)`
- `instrInstanceType(instr)`
- `instrHasExtraData(instr)`
- `instrJumpOffset(instr)`

### 2.2 Type tags used by the VM

The runtime uses these GML-type tags:

```text
GML_TYPE_DOUBLE   0x0
GML_TYPE_FLOAT    0x1
GML_TYPE_INT32    0x2
GML_TYPE_INT64    0x3
GML_TYPE_BOOL     0x4
GML_TYPE_VARIABLE 0x5
GML_TYPE_STRING   0x6
GML_TYPE_INT16    0x7
```

The VM also uses `GML_TYPE_VARIABLE` as a stack-tag for values that are logically variable-sized or come from a variable slot.

### 2.3 Variable operand encoding

Variable references are stored as a 32-bit operand payload with the variable index in the low 27 bits and a higher-level var-type tag in the upper bits.

The runtime extracts the var type as:

```c
(varRef >> 24) & 0xF8
```

The relevant var-type values are:
- `VARTYPE_ARRAY` = `0x00`
- `VARTYPE_STACKTOP` = `0x80`
- `VARTYPE_NORMAL` = `0xA0`
- `VARTYPE_INSTANCE` = `0xE0`

For compiler authors, the practical implication is:
- `Push.v`/`Pop.v` are not just “load/store a variable”; they are scoped accesses whose semantics depend on the var-type bits and the instruction’s instance type.
- Array access is not encoded as a separate opcode family in the same way as ordinary variable access; it is expressed via the var-type bits and stack data.

### 2.4 Scope and instance encoding

The low 16 bits of an instruction often hold an instance scope id. The runtime recognizes these special values:

```text
INSTANCE_SELF      (-1)
INSTANCE_OTHER     (-2)
INSTANCE_ALL       (-3)
INSTANCE_NOONE     (-4)
INSTANCE_GLOBAL    (-5)
INSTANCE_LOCAL     (-7)
INSTANCE_STACKTOP  (-9)
INSTANCE_ARG       (-15)
INSTANCE_STATIC    (-16)
```

Object references are encoded as non-negative object indices. Instance IDs are encoded as `INSTANCE_ID_BASE + n` (the runtime uses `100000` as the base).

### 2.5 Stack layout conventions

The VM uses a stack of `RValue`s.

For binary operations the runtime expects:
- `a` below `b`
- `b` is popped first
- `a` is popped second

For function calls, arguments are pushed right-to-left, so the first argument is the top-most item at call time.

For array access, the runtime often expects metadata on the stack before the value:
- For `VARTYPE_ARRAY`, the stack usually contains `[arrayIndex, instanceType, value]` or `[arrayIndex, instanceType]` depending on whether the opcode is a store or a load.
- For `VARTYPE_STACKTOP`, the runtime pops an instance-type marker from the stack that can itself resolve to a later stack item in BC17+.

## 3. Core execution model

### 3.1 Push and pop

The VM pushes typed values onto the stack and pops them for arithmetic, comparison, stores, and control flow.

The runtime keeps a `gmlStackType` tag per stack slot so that the stack can be walked in native-byte units for `Dup` and some BC17+ array operations.

### 3.2 Variable resolution

The runtime resolves reads and writes through:
- locals
- self/instance variables
- global variables
- built-in variables
- script arguments
- static variables (BC17+)
- arrays and array-like access

For compiler authors, the important design rule is: emit the variable access in terms of the correct scope and use the instance-type field or the var-type bits so the runtime can resolve it.

### 3.3 Arrays and CoW

Array writes use copy-on-write semantics. The VM has special handling for:
- top-level array materialization
- multi-dimensional arrays via `BREAK_PUSHAC` / `BREAK_PUSHAF` / `BREAK_POPAF`
- owner tracking with `BREAK_SETOWNER`

This is very important for BC17+ bytecode generation. When emitting array accesses in newer bytecode, the compiler must preserve the multi-step stack discipline and not assume a simple scalar assignment.

## 4. Opcode reference

### 4.1 Push/load opcodes

#### `OP_PUSH` (`0xC0`)

Purpose: push an immediate literal or a variable reference.

Operands:
- `type1` selects the payload kind
- `extraData` contains the payload

Supported payloads:
- `Push.d`: double literal
- `Push.f`: float literal
- `Push.i`: int32 literal
- `Push.l`: int64 literal
- `Push.b`: bool literal
- `Push.s`: string literal
- `Push.v`: variable read
- `Push.e`: int16 immediate

Stack effect:
- pushes one value

Compiler guidance:
- Emit `Push` for literals and variable reads.
- For variables, the operand payload is a resolved var reference. The instruction’s instance-type field and the varRef bits determine the scope and access kind.
- For array or stacktop access, add the needed stack metadata before the `Pop` or the later array chain instructions.

#### `OP_PUSHLOC` (`0xC1`)

Purpose: push a local variable.

Stack effect:
- pushes one value

Compiler guidance:
- Use for locals. The runtime reads from the current code frame’s local slot directly.
- For array-valued locals, the same array access conventions apply as with general variables.

#### `OP_PUSHGLB` (`0xC2`)

Purpose: push a global variable.

Stack effect:
- pushes one value

Compiler guidance:
- Use for global variables.

#### `OP_PUSHBLTN` (`0xC3`)

Purpose: push a built-in variable.

Stack effect:
- pushes one value

Compiler guidance:
- Use when the source is a built-in variable such as `x`, `y`, `image_index`, `argument0`, or similar runtime-managed variables.

#### `OP_PUSHI` (`0x84`)

Purpose: push an int16 immediate from the instruction’s low 16 bits.

Stack effect:
- pushes one int16-like value

Compiler guidance:
- Use for small integer constants.

### 4.2 Store opcodes

#### `OP_POP` (`0x45`)

Purpose: store a value to a variable.

Operands:
- `type1` describes the destination semantics
- `type2` describes the source type on the stack
- `extraData` carries the varRef
- the instruction’s instance-type field supplies the scope

Stack effect:
- pops a value (and, for arrays/stacktop access, additional stack metadata)

Compiler guidance:
- Emit `Pop` for plain assignments, array stores, and compound assignments.
- For array writes, the runtime expects the stack to contain the array-index and scope metadata in a specific order.
- For `VARTYPE_ARRAY`, the VM uses the array-index and instance-type from the stack before the value.
- For `VARTYPE_STACKTOP`, the runtime may pop an additional marker from the stack in BC17+.

#### `OP_POPZ` (`0x9E`)

Purpose: discard a value from the stack.

Stack effect:
- pops one value and drops it

Compiler guidance:
- Useful for discarding unused results or for stack cleanup when the compiler has computed a value that is not needed.

### 4.3 Arithmetic and logical opcodes

#### `OP_ADD` (`0x0C`)

Purpose: add two values.

Stack effect:
- pops `b`, `a`
- pushes `a + b`

Compiler guidance:
- Emit for `+`.
- The VM has a fast path for integer/real combinations and a string-concatenation fallback.
- String concatenation is handled specially.

#### `OP_SUB` (`0x0D`)

Purpose: subtract.

Stack effect:
- pops `b`, `a`
- pushes `a - b`

#### `OP_MUL` (`0x08`)

Purpose: multiply.

Stack effect:
- pops `b`, `a`
- pushes `a * b`

Compiler guidance:
- Strings with a numeric repetition count are handled specially.

#### `OP_DIV` (`0x09`)

Purpose: divide.

Stack effect:
- pops `b`, `a`
- pushes `a / b`

Compiler guidance:
- Integer/integer division throws on zero; float/variable paths use IEEE behavior and may produce `NaN`.

#### `OP_REM` (`0x0A`)

Purpose: remainder.

Stack effect:
- pops `b`, `a`
- pushes `a % b` in integer semantics

#### `OP_MOD` (`0x0B`)

Purpose: modulus.

Stack effect:
- pops `b`, `a`
- pushes real modulus

#### `OP_AND` (`0x0E`)

Purpose: bitwise and.

Stack effect:
- pops `b`, `a`
- pushes `a & b`

#### `OP_OR` (`0x0F`)

Purpose: bitwise or.

Stack effect:
- pops `b`, `a`
- pushes `a | b`

#### `OP_XOR` (`0x10`)

Purpose: bitwise xor.

Stack effect:
- pops `b`, `a`
- pushes `a ^ b`

#### `OP_SHL` (`0x13`)

Purpose: shift left.

#### `OP_SHR` (`0x14`)

Purpose: shift right.

### 4.4 Unary opcodes

#### `OP_NEG` (`0x11`)

Purpose: unary negation.

Stack effect:
- pops `a`
- pushes `-a`

#### `OP_NOT` (`0x12`)

Purpose: logical or bitwise not depending on the destination type.

Stack effect:
- pops `a`
- pushes `!a` if the target type is boolean, otherwise `~a`

Compiler guidance:
- Emit this for both `!` and `~` depending on the source language operator and the intended type.

### 4.5 Conversion opcodes

#### `OP_CONV` (`0x07`)

Purpose: convert a stack value from one GML type to another.

Operands:
- `type1` = source type
- `type2` = destination type
- the low nibble of the combined conversion key is used internally to select the exact conversion path

Stack effect:
- pops one value
- pushes one converted value

Compiler guidance:
- Emit for explicit type conversions such as `real`, `string`, `bool`, `int32`, and `int64` casts.
- The runtime has fast paths for a few common conversions; a compiler should still emit the normal `Conv` form for correctness.

### 4.6 Comparison opcodes

#### `OP_CMP` (`0x15`)

Purpose: compare two values.

Operands:
- `instrCmpKind(instr)` selects the comparison kind: `LT`, `LTE`, `EQ`, `NEQ`, `GTE`, `GT`

Stack effect:
- pops `b`, `a`
- pushes a boolean result

Compiler guidance:
- Emit for all comparison operators.
- The VM uses string-to-real coercion for mixed string/number comparison and epsilon-based numeric comparisons, so do not assume plain IEEE equality semantics.

### 4.7 Stack manipulation

#### `OP_DUP` (`0x86`)

Purpose: duplicate the top stack item(s).

Operands:
- low 8 bits of the instruction encode the duplication count in the old form
- BC17+ uses a more complex stack-rotation variant with the high bit set

Stack effect:
- copies one or more stack items

Compiler guidance:
- Use for repeated values and for some compiler-generated temporary duplication patterns.
- BC17+ stack-rotation mode is a low-level bytecode feature; if you are targeting modern bytecode, preserve the exact stack shape if the runtime expects it.

### 4.8 Control-flow opcodes

#### `OP_B` (`0xB6`)

Purpose: unconditional branch.

Operands:
- `instrJumpOffset(instr)` gives the signed offset in instruction units

Stack effect:
- none

#### `OP_BT` (`0xB7`)

Purpose: branch if the top value is non-zero.

Stack effect:
- pops one boolean-ish value

#### `OP_BF` (`0xB8`)

Purpose: branch if the top value is zero.

Stack effect:
- pops one boolean-ish value

Compiler guidance:
- These are the workhorse for `if`, `while`, `for`, and loop back-edges.
- The offset is relative to the current instruction address and is scaled by 4 in the runtime.

### 4.9 Function-call opcodes

#### `OP_CALL` (`0xD9`)

Purpose: call a script/function by index.

Operands:
- low 16 bits = argument count
- the extra-data operand is a patched function index

Stack effect:
- pops `argN ... arg0`
- pushes a result value

Compiler guidance:
- Emit for direct script/function calls.
- The runtime resolves the callee once and caches it.
- Arguments are popped right-to-left, so the compiler must arrange them in the expected order.

#### `OP_CALLV` (`0x99`)

Purpose: dynamic call through a variable/method reference (BC17+).

Stack effect:
- pops `[func, instance, argN ... arg0]`
- pushes a result value

Compiler guidance:
- Use for method calls and dynamic dispatch.
- The function value can be a method object or a numeric script index in newer bytecode.

### 4.10 Return and exit

#### `OP_RET` (`0x9C`)

Purpose: return a value from the current code block.

Stack effect:
- pops one value and returns it

#### `OP_EXIT` (`0x9D`)

Purpose: terminate the current code block without a value.

Stack effect:
- none

Compiler guidance:
- `Exit` returns a default value depending on the runtime version. Older versions fall back to `0.0`; newer ones use `undefined`.

### 4.11 Environment / with-block opcodes

#### `OP_PUSHENV` (`0xBA`)

Purpose: enter a `with` block or similar environment.

Stack effect:
- pops a target value from the stack

Compiler guidance:
- Use for `with (...)` semantics.
- The runtime supports target values such as self, other, noone, all, object index, and instance ID.
- When the target resolves to an empty set, the VM jumps past the with-body.

#### `OP_POPENV` (`0xBB`)

Purpose: leave a `with` block or advance to the next instance in an iteration.

Stack effect:
- none

Compiler guidance:
- The runtime uses this for iteration across instances in a with-block.
- The special operand `0xF00000` means “exit the environment immediately”.

## 5. BC17+ extended opcodes (`OP_BREAK`, `0xFF`)

These are the most important opcode extensions for modern bytecode. They are only meaningful in newer WAD versions; on older versions they are effectively no-ops or debug hooks.

The extended sub-opcodes are encoded in the instruction’s low 16 bits as negative values.

### `BREAK_CHKINDEX` (`-1`)

Purpose: validate array indices.

Stack effect:
- peeks the top of the stack
- expects a signed integer array index

Compiler guidance:
- Emit before indexing into arrays if you want to match the runtime’s bounds behavior.

### `BREAK_PUSHAF` (`-2`)

Purpose: pop an array ref plus an index and push the element at that index.

Compiler guidance:
- Use for final-dimension array reads in multi-dimensional array chains.

### `BREAK_POPAF` (`-3`)

Purpose: store a value into a multi-dimensional array cell.

Compiler guidance:
- Use for final-dimension array writes.
- The VM expects the surrounding array chain to have already been materialized and CoW-forked.

### `BREAK_PUSHAC` (`-4`)

Purpose: pop an array ref plus an index and push a sub-array reference.

Compiler guidance:
- Use to create or traverse intermediate array dimensions.
- This materializes a child array if one is missing.

### `BREAK_SETOWNER` (`-5`)

Purpose: set the current copy-on-write owner token.

Compiler guidance:
- Emit at the start of a function/event scope when generating BC17+ bytecode that uses arrays.
- This token is used to decide whether a later write needs a CoW fork.

### `BREAK_ISSTATICOK` (`-6`)

Purpose: push whether the current function’s static block has already run.

### `BREAK_SETSTATIC` (`-7`)

Purpose: mark the current function’s static block as initialized.

### `BREAK_SAVEAREF` (`-8`)

Purpose: save the current array container reference for a compound assignment chain.

### `BREAK_RESTOREAREF` (`-9`)

Purpose: restore the saved container reference.

### `BREAK_ISNULLISH` (`-10`)

Purpose: test whether a value is nullish (currently handled as undefined by the runtime).

### `BREAK_PUSHREF` (`-11`)

Purpose: push an asset reference or a script/function reference encoded in the 32-bit operand.

Compiler guidance:
- Emitting this correctly matters for script references and other asset references in newer bytecode.

## 6. Compiler guidance by high-level construct

### 6.1 Variable reads and writes

- Use `Push`/`PushLoc`/`PushGlb`/`PushBltn` for reads.
- Use `Pop` for writes.
- For local variables, prefer `PushLoc`/`Pop` against the local slot.
- For globals, use `PushGlb`.
- For builtins, use `PushBltn` and `Pop` with the builtin variable resolution.

### 6.2 Arithmetic expressions

- Lower `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, and `>>` to the corresponding opcode family.
- Preserve the expected operand order on the stack.
- Use `Conv` where the language semantics require type coercion.

### 6.3 Comparisons

- Lower `==`, `!=`, `<`, `<=`, `>`, `>=` to `Cmp` with the correct comparison kind.
- Be aware of the VM’s coercion model for mixed types and strings.

### 6.4 Conditionals and loops

- Lower `if` and loop tests to `BT`/`BF` with branch offsets.
- Keep branch targets in instruction-address space, not source-line space.
- Remember that the runtime’s branch offset is scaled by 4 bytes.

### 6.5 Function calls

- Use `Call` for direct calls.
- Use `CallV` for dynamic dispatch.
- For calls with arguments, push arguments right-to-left.

### 6.6 Arrays

- For 1D arrays, match the stack layout expected by the runtime for reads and writes.
- For multi-dimensional arrays, use the BC17+ chain: `PUSHAC`/`PUSHAF`/`POPAF` plus the appropriate owner-tracking opcodes.
- For BC17+, keep array ownership consistent; otherwise CoW behavior will silently diverge from the native runner.

### 6.7 With-statements

- Lower `with (...)` to `PushEnv`/`PopEnv` with the correct target expression and a body range.
- Ensure the environment stack is balanced and that the exit path is emitted correctly.

## 7. Version-specific caveats

### Older bytecode (WAD 13/14)

The runtime rewrites older bytecode to a BC16-like form during VM creation. For compiler authors, the practical implication is:
- you do not need to mimic the old bytecode verbatim if you target the modern VM’s expected form
- but you should still be careful with globals and variable scoping because the runtime’s rewrite path has specific assumptions

### BC17+

BC17+ adds the most important modern semantics:
- extended array access opcodes
- `CALLV`
- explicit array ownership tracking
- static variables
- `INSTANCE_ARG` and `INSTANCE_STACKTOP` semantics

If your compiler targets GMS 2.3+, it should be designed around these semantics rather than the older BC16-style assumptions.

## 8. Practical checklist for a compiler

When emitting bytecode, verify that each generated instruction satisfies all of the following:
- the opcode is correct for the operation
- the operand payload uses the correct size and layout
- the stack effect matches the runtime’s expectations
- variable accesses use the intended scope and var-type bits
- array writes preserve the runtime’s CoW semantics in BC17+
- branches jump to the correct instruction offset
- function calls push arguments in the correct order
- `with` environments are balanced

If you want a single sentence to guide generation: emit the simplest bytecode that preserves the same stack shape and scope semantics as the VM expects, and let the runtime’s own helpers resolve the rest.
