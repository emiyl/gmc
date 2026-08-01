# data.win structure reference for generator authors

This document describes the data.win layout that the parser in [Butterscotch](https://github.com/ButterscotchRunner/Butterscotch) expects. It is intended as an implementation-oriented reference for creating or editing data.win files, not as a generic reverse-engineering report.

The authoritative implementation is in:
- [Butterscotch/src/data_win.c](https://github.com/ButterscotchRunner/Butterscotch/blob/main/src/data_win.c)
- [Butterscotch/src/data_win.h](https://github.com/ButterscotchRunner/Butterscotch/blob/main/src/data_win.h)

## 1. File container

A data.win file is a FORM-style container.

```text
FORM
  uint32 length
  chunk0
  chunk1
  chunk2
  ...
```

Each chunk has:

```text
chunkName[4]   // ASCII, 4 bytes, e.g. GEN8, STRG, SPRT
uint32 length
payload bytes
```

Important details:
- All integer values are read as little-endian integers.
- All offsets inside the file are absolute offsets from the start of the file, not from the chunk payload.
- Strings are not stored inline in most chunks. Instead, most string fields are stored as a string pointer that resolves through the STRG chunk.
- The parser reads chunks sequentially and uses the chunk boundaries to know where each chunk ends.

## 2. Common encoding conventions

### Primitive types

- `uint8` = 1 byte
- `uint16` = 2 bytes
- `uint32` / `int32` = 4 bytes
- `uint64` / `int64` = 8 bytes
- `float32` = 4 bytes
- `bool32` = 4 bytes, with `0` or `1` used as a boolean value

### Pointer list

Many chunks store arrays as a pointer list:

```text
uint32 count
uint32 ptr0
uint32 ptr1
...
```

Each `ptrN` is an absolute file offset to an entry record.

### String pointer

A string field is typically stored as:

```text
uint32 offsetToString
```

`offsetToString == 0` means null. Otherwise, the offset resolves into the STRG chunk.

### String storage in STRG

The STRG chunk stores a pointer table to strings, where each string entry is referenced by an offset to the string’s length prefix:

```text
uint32 length
bytes[length]
```

The parser uses the offset to the start of the string data, skipping the length prefix.

## 3. Chunk catalog

The parser recognizes these chunks:

- `GEN8` General info
- `OPTN` Options
- `LANG` Languages
- `EXTN` Extensions
- `SOND` Sounds
- `AGRP` Audio groups
- `SPRT` Sprites
- `BGND` Backgrounds
- `PATH` Paths
- `SCPT` Scripts
- `GLOB` Global init scripts
- `SHDR` Shaders
- `FONT` Fonts
- `TMLN` Timelines
- `OBJT` Objects
- `ROOM` Rooms
- `TPAG` Texture pages
- `CODE` Bytecode
- `VARI` Variables
- `FUNC` Functions
- `STRG` String table
- `TXTR` Textures
- `AUDO` Audio blobs
- `ACRV` Animation curves
- `SEQN` Sequences
- `TAGS` Tags
- `FEDS` Filter effects data
- `FEAT` Feature metadata
- `UILR` UI layer metadata
- `PSEM` Particle emitter metadata
- `PSYS` Particle system metadata
- `DAFL` Empty chunk
- `EMBI` Embedded images
- `TGIN` Texture group info

## 4. Chunk-by-chunk layout

The following section describes the layout that the parser consumes. Field order matters.

### 4.1 GEN8

```text
uint8 isDebuggerDisabled
uint8 wadVersion
uint16 padding
StringPtr fileName
StringPtr config
uint32 lastObj
uint32 lastTile
uint32 gameID
uint8 directPlayGuid[16]
StringPtr name
uint32 major
uint32 minor
uint32 release
uint32 build
uint32 defaultWindowWidth
uint32 defaultWindowHeight
uint32 info
uint32 licenseCRC32
uint8 licenseMD5[16]
uint64 timestamp
StringPtr displayName
uint64 activeTargets
uint64 functionClassifications
int32 steamAppID
uint32 debuggerPort
uint32 roomOrderCount
int32 roomOrder[roomOrderCount]
float32 gms2FPS   // only in newer layouts
```

Version notes:
- Older WAD8/9 layouts are more compact and omit some fields.
- The parser detects several compact and extended variants and adjusts accordingly.

### 4.2 OPTN

```text
int32 shaderExtensionFlag
if shaderExtensionFlag == 0x80000000:
    int32 shaderExtVersion
    uint64 info
    int32 scale
    uint32 windowColor
    uint32 colorDepth
    uint32 resolution
    uint32 frequency
    uint32 vertexSync
    uint32 priority
    uint32 backImage
    uint32 frontImage
    uint32 loadImage
    uint32 loadAlpha
else:
    bool32 fullscreen
    bool32 interpolatePixels
    bool32 useNewAudio
    bool32 noBorder
    bool32 showCursor
    int32 scale
    bool32 sizeable
    bool32 stayOnTop
    uint32 windowColor
    bool32 changeResolution
    uint32 colorDepth
    uint32 resolution
    uint32 frequency
    bool32 noButtons
    uint32 vertexSync
    bool32 screenKey
    bool32 helpKey
    bool32 quitKey
    bool32 saveKey
    bool32 screenShotKey
    bool32 closeSec
    uint32 priority
    bool32 freeze
    bool32 showProgress
    uint32 backImage
    uint32 frontImage
    uint32 loadImage
    bool32 loadTransparent
    uint32 loadAlpha
    bool32 scaleProgress
    bool32 displayErrors
    bool32 writeErrors
    bool32 abortErrors
    bool32 variableErrors
    bool32 creationEventOrder

uint32 constantCount
OptnConstant[constantCount]
```

Each `OptnConstant` is:

```text
StringPtr name
StringPtr value
```

### 4.3 LANG

```text
uint32 unknown1
uint32 languageCount
uint32 entryCount
StringPtr entryIds[entryCount]
Language[languageCount]
```

Each `Language` is:

```text
StringPtr name
StringPtr region
uint32 entryCount
StringPtr entries[entryCount]
```

### 4.4 EXTN

```text
uint32 extensionCount
uint32 extensionPtrs[extensionCount]
```

Each extension entry begins at the pointer target:

```text
StringPtr folderName
StringPtr name
StringPtr className
uint32 fileCount
uint32 filePtrs[fileCount]
```

Each file entry is:

```text
StringPtr filename
StringPtr cleanupScript
StringPtr initScript
uint32 kind
uint32 functionCount
uint32 functionPtrs[functionCount]
```

Each function entry is:

```text
StringPtr name
uint32 id
uint32 kind
uint32 retType
StringPtr extName
uint32 argumentCount
uint32 arguments[argumentCount]
```

### 4.5 SOND

```text
uint32 soundCount
uint32 soundPtrs[soundCount]
```

Each sound entry is:

```text
StringPtr name
uint32 flags
StringPtr type
StringPtr file
uint32 effects
float32 volume
float32 panOrPitch
bool32 embeddedFlagOrPitchField
int32 audioGroupOrPreload
int32 audioFile
```

The exact field mapping depends on the WAD version.

### 4.6 AGRP

```text
uint32 audioGroupCount
uint32 audioGroupPtrs[audioGroupCount]
```

Each audio group entry is:

```text
StringPtr name
StringPtr path   // present in newer formats
```

### 4.7 SPRT

```text
uint32 spriteCount
uint32 spritePtrs[spriteCount]
```

Each sprite entry is:

```text
StringPtr name
uint32 width
uint32 height
int32 marginLeft
int32 marginRight
int32 marginBottom
int32 marginTop
bool32 transparent
bool32 smooth
bool32 preload
uint32 bboxMode
uint32 sepMasks
int32 originX
int32 originY
int32 checkOrSpecialTypeMarker
if check == -1:
    bool32 specialType
    uint32 sVersion
    uint32 sSpriteType
    ... optional extra fields ...
uint32 textureCount
uint32 tpagOffsets[textureCount]
uint32 maskDataCount
maskBytes[maskDataCount]   // packed bit arrays, row-major, MSB first
```

Notes:
- The parser treats `tpagOffsets` as temporary absolute offsets and resolves them later against the TPAG chunk.
- Mask storage is padded to 4-byte alignment.
- Newer versions may contain a nine-slice block.

### 4.8 BGND

```text
uint32 backgroundCount
uint32 backgroundPtrs[backgroundCount]
```

Each background entry is:

```text
StringPtr name
bool32 transparent
bool32 smooth
bool32 preload
uint32 tpagOffset
... version-specific fields ...
```

Newer backgrounds may include tile-size, tile-separation, border, tile-count, sprite index, frame length, and tile-id arrays.

### 4.9 PATH

```text
uint32 pathCount
uint32 pathPtrs[pathCount]
```

Each path entry is:

```text
StringPtr name
bool32 isSmooth
bool32 isClosed
uint32 precision
uint32 pointCount
PathPoint[pointCount]
```

Each `PathPoint` is:

```text
float32 x
float32 y
float32 speed
```

The parser precomputes internal path points and an arc-length table after reading the points.

### 4.10 SCPT

```text
uint32 scriptCount
uint32 scriptPtrs[scriptCount]
```

Each script entry is:

```text
StringPtr name
int32 codeId
```

### 4.11 GLOB

```text
uint32 codeIdCount
int32 codeIds[codeIdCount]
```

### 4.12 SHDR

```text
uint32 shaderCount
uint32 shaderPtrs[shaderCount]
```

Each shader entry is:

```text
StringPtr name
uint32 type
StringPtr glslES_Vertex
StringPtr glslES_Fragment
StringPtr glsl_Vertex
StringPtr glsl_Fragment
StringPtr hlsl9_Vertex
StringPtr hlsl9_Fragment
uint32 hlsl11_VertexOffset
uint32 hlsl11_PixelOffset
uint32 vertexAttributeCount
StringPtr vertexAttributes[vertexAttributeCount]
... version-specific shader payload fields ...
```

### 4.13 FONT

```text
uint32 fontCount
uint32 fontPtrs[fontCount]
```

Each font entry is:

```text
StringPtr name
StringPtr displayName
uint32 rawEmSize
bool32 bold
bool32 italic
uint16 rangeStart
uint8 charset
uint8 antiAliasing
uint32 rangeEnd
uint32 tpagOffset
float32 scaleX
float32 scaleY
... optional fields ...
uint32 glyphCount
uint32 glyphPtrs[glyphCount]
```

Each glyph is:

```text
uint16 character
uint16 sourceX
uint16 sourceY
uint16 sourceWidth
uint16 sourceHeight
int16 shift
int16 offset
uint16 kerningCount
KerningPair[kerningCount]
```

Each kerning pair is:

```text
int16 character
int16 shiftModifier
```

### 4.14 TMLN

```text
uint32 timelineCount
uint32 timelinePtrs[timelineCount]
```

Each timeline entry is:

```text
StringPtr name
uint32 momentCount
Moment[momentCount]
```

Each moment is:

```text
uint32 step
uint32 eventActionListPtr
```

Each event-action list is:

```text
uint32 actionCount
uint32 actionPtrs[actionCount]
```

Each action entry is:

```text
uint32 libID
uint32 id
uint32 kind
bool32 useRelative
bool32 isQuestion
bool32 useApplyTo
uint32 exeType
StringPtr actionName
int32 codeId
uint32 argumentCount
int32 who
bool32 relative
bool32 isNot
uint32 unknownAlwaysZero
```

### 4.15 OBJT

```text
uint32 objectCount
uint32 objectPtrs[objectCount]
```

Each object entry is:

```text
StringPtr name
int32 spriteId
bool32 visible
bool32 managed   // modern format only
bool32 solid
int32 depth
bool32 persistent
int32 parentId
int32 textureMaskId
bool32 usesPhysics
bool32 isSensor
uint32 collisionShape
float32 density
float32 restitution
uint32 group
float32 linearDamping
float32 angularDamping
int32 physicsVertexCount
float32 friction
bool32 awake
bool32 kinematic
PhysicsVertex[physicsVertexCount]
uint32 eventTypeCount
uint32 eventTypePtrs[eventTypeCount]
```

Each event type slot contains another pointer list of object events:

```text
uint32 eventCount
uint32 eventPtrs[eventCount]
```

Each event entry is:

```text
uint32 eventSubtype
uint32 actionCount
uint32 actionPtrs[actionCount]
```

### 4.16 ROOM

The ROOM chunk stores room headers first, and the payload fields are read lazily or eagerly depending on the parser options.

Room header layout:

```text
StringPtr name
StringPtr caption
uint32 width
uint32 height
uint32 speed
bool32 persistent
uint32 backgroundColor
bool32 drawBackgroundColor
int32 creationCodeId
uint32 flags
uint32 backgroundsFileOffset
uint32 viewsFileOffset
uint32 gameObjectsFileOffset
uint32 tilesFileOffset
bool32 world
uint32 top
uint32 left
uint32 right
uint32 bottom
float32 gravityX
float32 gravityY
float32 metersPerPixel
... optional newer fields ...
uint32 layersFileOffset
... optional sequences pointer ...
```

Payload sections are read from the indicated offsets:

```text
backgrounds section
views section
game objects section
tiles section
layers section (optional)
```

Room background entry:

```text
bool32 enabled
bool32 foreground
int32 backgroundDefinition
int32 x
int32 y
int32 tileX
int32 tileY
int32 speedX
int32 speedY
bool32 stretch
```

Room view entry:

```text
bool32 enabled
int32 viewX
int32 viewY
int32 viewWidth
int32 viewHeight
int32 portX
int32 portY
int32 portWidth
int32 portHeight
uint32 borderX
uint32 borderY
int32 speedX
int32 speedY
int32 objectId
```

Room game object entry:

```text
int32 x
int32 y
int32 objectDefinition
uint32 instanceID
int32 creationCode
float32 scaleX
float32 scaleY
float32 imageSpeed   // modern format
int32 imageIndex     // modern format
uint32 color
float32 rotation
int32 preCreateCode  // newer format
```

Room tile entry:

```text
int32 x
int32 y
bool32 useSpriteDefinition
int32 backgroundDefinition
int32 sourceX
int32 sourceY
uint32 width
uint32 height
int32 tileDepth
uint32 instanceID
float32 scaleX
float32 scaleY
uint32 color
float32 alpha
```

Layer entry:

```text
StringPtr name
uint32 id
uint32 type
int32 depth
float32 xOffset
float32 yOffset
float32 hSpeed
float32 vSpeed
bool32 visible
... optional effect data ...
```

Layer payloads vary by `type` and may contain background, instance, asset, tile, or effect data.

### 4.17 TPAG

```text
uint32 texturePageCount
uint32 texturePagePtrs[texturePageCount]
```

Each texture page item is:

```text
uint16 sourceX
uint16 sourceY
uint16 sourceWidth
uint16 sourceHeight
uint16 targetX
uint16 targetY
uint16 targetWidth
uint16 targetHeight
uint16 boundingWidth
uint16 boundingHeight
int16 texturePageId
```

The parser resolves sprite/background/font references to TPAG indices after the TPAG chunk is parsed.

### 4.18 CODE

```text
uint32 codeCount
uint32 codePtrs[codeCount]
```

Each code entry is:

```text
StringPtr name
uint32 length
if old format:
    bytecode bytes[length]
else:
    uint16 localsCount
    uint16 argumentsCount
    int32 bytecodeRelAddr
    uint32 offset
```

The parser then collects the bytecode blobs into a single owned bytecode buffer.

### 4.19 VARI

```text
if old format:
    // no header
    uint32 variableCount
    Variable[variableCount]
else:
    uint32 varCount1
    uint32 varCount2
    uint32 maxLocalVarCount
    uint32 variableCount
    Variable[variableCount]
```

Each variable entry is:

```text
StringPtr name
int32 instanceType   // new format
int32 varID          // new format
uint32 occurrences
uint32 firstAddress
```

### 4.20 FUNC

```text
if old format:
    uint32 functionCount
    Function[functionCount]
else:
    uint32 functionCount
    Function[functionCount]
    uint32 codeLocalsCount
    CodeLocals[codeLocalsCount]
```

Each function entry is:

```text
StringPtr name
uint32 occurrences
uint32 firstAddress
```

Each code locals entry is:

```text
uint32 localVarCount
StringPtr name
LocalVar[localVarCount]
```

Each local variable entry is:

```text
uint32 varID
StringPtr name
```

### 4.21 STRG

```text
uint32 stringCount
uint32 stringPtrs[stringCount]
```

Each string entry is not read as a full string record by the parser; it resolves to an offset to the UTF-8 payload after a length prefix.

### 4.22 TXTR

```text
uint32 textureCount
uint32 texturePtrs[textureCount]
```

Each texture entry is:

```text
uint32 scaled
uint32 generatedMips
uint32 textureBlockSize   // newer versions
int32 textureWidth        // newer versions
int32 textureHeight       // newer versions
int32 indexInGroup       // newer versions
uint32 blobOffset
```

The blob size is inferred from the next texture’s offset or the chunk end.

### 4.23 AUDO

```text
uint32 audioEntryCount
uint32 audioEntryPtrs[audioEntryCount]
```

Each audio entry is:

```text
uint32 dataSize
uint8 data[dataSize]
```

The parser stores the offset to the data payload and loads it on demand if lazy-loading is enabled.

## 5. Versioning and compatibility notes

The parser is version-aware. It detects and supports multiple layout variants, especially for:
- WAD8 / WAD9 / WAD10 / WAD11 / WAD12 / WAD13 / WAD14+ layouts
- GMS 2.3+ animation curves and sequences
- GMS 2022.5+ object format changes
- GMS 2022.1+ room layer effect data
- GMS 2024.2+ room tile RLE compression
- GMS 2024.4+ tile-data alignment
- GMS 2024.6+ sprite mask bounding-box layout
- GMS 2024.14+ audio group path fields

For generator work, the safest strategy is:
1. Emit the common base fields first.
2. Add version-specific fields only when the target runtime version requires them.
3. Keep the STRG chunk populated early and make all string references point into it.
4. Use pointer-list and absolute-offset conventions exactly as shown above.

## 6. Canonical minimal skeleton

A minimal file should at least contain a recognizable container header plus the chunks needed by the target runtime. A practical skeleton is:

```text
FORM
  GEN8
  STRG
  OPTN   (optional for many games)
  SOND   (optional)
  AGRP   (optional)
  SPRT   (optional)
  BGND   (optional)
  PATH   (optional)
  SCPT   (optional)
  GLOB   (optional)
  SHDR   (optional)
  FONT   (optional)
  TMLN   (optional)
  OBJT   (optional)
  ROOM   (optional)
  TPAG   (optional)
  CODE   (optional)
  VARI   (optional)
  FUNC   (optional)
  TXTR   (optional)
  AUDO   (optional)
```

In practice, a generator should reserve space for the STRG chunk first, then place all string references into it, and then emit the other chunks using absolute offsets into the final file.

## 7. Practical writing rules

- Keep all pointers absolute and valid for the final file layout.
- Use the STRG chunk as the single source of truth for string storage.
- For chunks with pointer lists, write the pointer table first, then the referenced records at the target offsets.
- When emitting variable-length payloads, ensure the chunk length field matches the actual bytes written.
- Respect 4-byte alignment for many payload blocks and any alignment rules that the target version requires.

This document is intentionally implementation-focused. If a chunk structure is unclear in a specific runtime version, the parser code should be treated as the source of truth for that version.
