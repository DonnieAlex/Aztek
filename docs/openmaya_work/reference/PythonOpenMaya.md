# maya.OpenMaya, Python API 1.0

This is the current Aztek Python 1.0 surface, listed for the implementation agents. It is not a transcription of a Python 1.0 page; the reference names and signatures for these classes are the C++ class pages transcribed in the other files in this folder. Python 1.0 names take precedence over the 2.0 names shown by the underlying classes.

Classes in this module: 100.

Constant members (`k...`) are counted but not listed; see `data/enum_gaps.tsv` for the constant audit.

## Classes

### MAngle

Bases: `MAngle`.

Constants: 6 names.

**Members**

- `asAngMinutes` (method) `(*a, **k)`
- `asAngSeconds` (method) `(*a, **k)`
- `asDegrees` (method) `(*a, **k)`
- `asRadians` (method) `(*a, **k)`
- `asUnits` (method) `(*a, **k)`
- `internalToUI` (method)
- `internalUnit` (method)
- `setUIUnit` (method)
- `uiToInternal` (method)
- `uiUnit` (method)


### MArgDatabase

Bases: `MArgDatabase`.

**Members**

- `commandArgumentAngle` (method) `(*a, **k)`
- `commandArgumentBool` (method) `(*a, **k)`
- `commandArgumentDistance` (method) `(*a, **k)`
- `commandArgumentDouble` (method) `(*a, **k)`
- `commandArgumentFloat` (method) `(*a, **k)`
- `commandArgumentInt` (method) `(*a, **k)`
- `commandArgumentString` (method) `(*a, **k)`
- `commandArgumentTime` (method) `(*a, **k)`
- `flagArgumentAngle` (method) `(*a, **k)`
- `flagArgumentBool` (method) `(*a, **k)`
- `flagArgumentDistance` (method) `(*a, **k)`
- `flagArgumentDouble` (method) `(*a, **k)`
- `flagArgumentFloat` (method) `(*a, **k)`
- `flagArgumentInt` (method) `(*a, **k)`
- `flagArgumentMSelectionList` (method) `(self, name, index, sel=None)`
- `flagArgumentString` (method) `(*a, **k)`
- `flagArgumentTime` (method) `(*a, **k)`
- `getFlagArgument` (method) `(self, name, index, out=None)`
- `getFlagArgumentList` (method) `(*a, **k)`
- `getFlagArgumentPosition` (method) `(*a, **k)`
- `getObjectList` (method) `(self, sel=None)`
- `getObjectStrings` (method) `(self, arr=None)`
- `getObjects` (method) `(self, sel=None)`
- `isEdit` (method) `(*a, **k)`
- `isFlagSet` (method) `(*a, **k)`
- `isQuery` (method) `(*a, **k)`
- `numberOfFlagUses` (method) `(*a, **k)`
- `numberOfFlagsUsed` (method) `(*a, **k)`
- `selectionItem` (method) `(*a, **k)`


### MArgList

Bases: `MArgList`.

**Members**

- `addArg` (method) `(*a, **k)`
- `asAngle` (method) `(*a, **k)`
- `asBool` (method) `(*a, **k)`
- `asDistance` (method) `(*a, **k)`
- `asDouble` (method) `(*a, **k)`
- `asDoubleArray` (method) `(*a, **k)`
- `asFloat` (method) `(*a, **k)`
- `asInt` (method) `(*a, **k)`
- `asIntArray` (method) `(*a, **k)`
- `asMatrix` (method) `(*a, **k)`
- `asPoint` (method) `(*a, **k)`
- `asString` (method) `(*a, **k)`
- `asStringArray` (method) `(*a, **k)`
- `asTime` (method) `(*a, **k)`
- `asVector` (method) `(*a, **k)`
- `flagIndex` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`


### MArgParser

Bases: `MArgParser`.

**Members**

- `commandArgumentAngle` (method) `(*a, **k)`
- `commandArgumentBool` (method) `(*a, **k)`
- `commandArgumentDistance` (method) `(*a, **k)`
- `commandArgumentDouble` (method) `(*a, **k)`
- `commandArgumentFloat` (method) `(*a, **k)`
- `commandArgumentInt` (method) `(*a, **k)`
- `commandArgumentString` (method) `(*a, **k)`
- `commandArgumentTime` (method) `(*a, **k)`
- `flagArgumentAngle` (method) `(*a, **k)`
- `flagArgumentBool` (method) `(*a, **k)`
- `flagArgumentDistance` (method) `(*a, **k)`
- `flagArgumentDouble` (method) `(*a, **k)`
- `flagArgumentFloat` (method) `(*a, **k)`
- `flagArgumentInt` (method) `(*a, **k)`
- `flagArgumentString` (method) `(*a, **k)`
- `flagArgumentTime` (method) `(*a, **k)`
- `getFlagArgumentList` (method) `(*a, **k)`
- `getFlagArgumentPosition` (method) `(*a, **k)`
- `getObjectStrings` (method) `(*a, **k)`
- `isEdit` (method) `(*a, **k)`
- `isFlagSet` (method) `(*a, **k)`
- `isQuery` (method) `(*a, **k)`
- `numberOfFlagUses` (method) `(*a, **k)`
- `numberOfFlagsUsed` (method) `(*a, **k)`


### MArrayDataHandle

Bases: `MArrayDataHandle`.

**Members**

- `addElement` (method) `(*a, **k)`
- `elementCount` (method) `(*a, **k)`
- `elementLogicalIndex` (method) `(*a, **k)`
- `inputValue` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `jumpToArrayElement` (method) `(*a, **k)`
- `jumpToLogicalElement` (method) `(*a, **k)`
- `jumpToPhysicalElement` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `outputValue` (method) `(*a, **k)`
- `setAllClean` (method) `(*a, **k)`
- `setClean` (method) `(*a, **k)`


### MBoundingBox

Bases: `MBoundingBox`.

**Members**

- `center` (method) `(self)`
- `clear` (method) `(*a, **k)`
- `contains` (method) `(*a, **k)`
- `depth` (method) `(self)`
- `expand` (method) `(*a, **k)`
- `height` (method) `(self)`
- `intersects` (method) `(*a, **k)`
- `max` (method) `(self)`
- `min` (method) `(self)`
- `transformUsing` (method) `(*a, **k)`
- `width` (method) `(self)`


### MCallbackIdArray

Bases: `MCallbackIdArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MColor

Bases: `MColor`.

Constants: 7 names.

**Members**

- `get` (method) `(self, ptr)`
- `getColor` (method) `(*a, **k)`
- `setColor` (method) `(*a, **k)`


### MColorArray

Bases: `MColorArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MCommandResult

Bases: `object`.

Constants: 11 names.

**Members**

- `getResult` (method) `(self, out)`
- `resultType` (method) `(self)`
- `stringResult` (method) `(self)`


### MDGContext

Bases: `MDGContext`.

Constants: 1 names.

**Members**

- `getTime` (method) `(*a, **k)`
- `isNormal` (method) `(*a, **k)`


### MDGMessage

Bases: `MDGMessage`.

Constants: 3 names.

**Members**

- `addConnectionCallback` (method)
- `addDelayedTimeChangeCallback` (method)
- `addDelayedTimeChangeRunupCallback` (method)
- `addForceUpdateCallback` (method)
- `addNodeAddedCallback` (method)
- `addNodeChangeUuidCheckCallback` (method)
- `addNodeRemovedCallback` (method)
- `addPreConnectionCallback` (method)
- `addTimeChangeCallback` (method)
- `currentCallbackId` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MDGModifier

Bases: `MDGModifier`.

**Members**

- `addAttribute` (method) `(*a, **k)`
- `commandToExecute` (method) `(*a, **k)`
- `connect` (method) `(*a, **k)`
- `createNode` (method) `(*a, **k)`
- `deleteNode` (method) `(*a, **k)`
- `disconnect` (method) `(*a, **k)`
- `doIt` (method) `(*a, **k)`
- `newPlugValue` (method) `(*a, **k)`
- `newPlugValueBool` (method) `(*a, **k)`
- `newPlugValueChar` (method) `(*a, **k)`
- `newPlugValueDouble` (method) `(*a, **k)`
- `newPlugValueFloat` (method) `(*a, **k)`
- `newPlugValueInt` (method) `(*a, **k)`
- `newPlugValueMAngle` (method) `(*a, **k)`
- `newPlugValueMDistance` (method) `(*a, **k)`
- `newPlugValueMTime` (method) `(*a, **k)`
- `newPlugValueShort` (method) `(*a, **k)`
- `newPlugValueString` (method) `(*a, **k)`
- `pythonCommandToExecute` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `renameNode` (method) `(*a, **k)`
- `undoIt` (method) `(*a, **k)`


### MDagMessage

Bases: `MDagMessage`.

Constants: 50 names.

**Members**

- `addAllDagChangesCallback` (method)
- `addAllDagChangesDagPathCallback` (method)
- `addChildAddedCallback` (method)
- `addChildAddedDagPathCallback` (method)
- `addChildRemovedCallback` (method)
- `addChildRemovedDagPathCallback` (method)
- `addChildReorderedCallback` (method)
- `addChildReorderedDagPathCallback` (method)
- `addDagCallback` (method)
- `addDagDagPathCallback` (method)
- `addInstanceAddedCallback` (method)
- `addInstanceAddedDagPathCallback` (method)
- `addInstanceRemovedCallback` (method)
- `addInstanceRemovedDagPathCallback` (method)
- `addMatrixModifiedCallback` (method)
- `addParentAddedCallback` (method)
- `addParentAddedDagPathCallback` (method)
- `addParentRemovedCallback` (method)
- `addParentRemovedDagPathCallback` (method)
- `addWorldMatrixModifiedCallback` (method)
- `currentCallbackId` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MDagModifier

Bases: `MDagModifier`.

**Members**

- `addAttribute` (method) `(*a, **k)`
- `commandToExecute` (method) `(*a, **k)`
- `connect` (method) `(*a, **k)`
- `createNode` (method) `(*a, **k)`
- `deleteNode` (method) `(*a, **k)`
- `disconnect` (method) `(*a, **k)`
- `doIt` (method) `(*a, **k)`
- `newPlugValue` (method) `(*a, **k)`
- `newPlugValueBool` (method) `(*a, **k)`
- `newPlugValueChar` (method) `(*a, **k)`
- `newPlugValueDouble` (method) `(*a, **k)`
- `newPlugValueFloat` (method) `(*a, **k)`
- `newPlugValueInt` (method) `(*a, **k)`
- `newPlugValueMAngle` (method) `(*a, **k)`
- `newPlugValueMDistance` (method) `(*a, **k)`
- `newPlugValueMTime` (method) `(*a, **k)`
- `newPlugValueShort` (method) `(*a, **k)`
- `newPlugValueString` (method) `(*a, **k)`
- `pythonCommandToExecute` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `renameNode` (method) `(*a, **k)`
- `reparentNode` (method) `(*a, **k)`
- `undoIt` (method) `(*a, **k)`


### MDagPath

Bases: `MDagPath`.

Constants: 1 names.

**Members**

- `apiType` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `exclusiveMatrix` (method) `(*a, **k)`
- `exclusiveMatrixInverse` (method) `(*a, **k)`
- `extendToShape` (method) `(*a, **k)`
- `extendToShapeDirectlyBelow` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAPathTo` (method)
- `getAllPathsTo` (method)
- `getPath` (method) `(self, path=None)`
- `hasFn` (method) `(*a, **k)`
- `inclusiveMatrix` (method) `(*a, **k)`
- `inclusiveMatrixInverse` (method) `(*a, **k)`
- `instanceNumber` (method) `(*a, **k)`
- `isInstanced` (method) `(*a, **k)`
- `isValid` (method) `(*a, **k)`
- `isVisible` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `node` (method) `(*a, **k)`
- `numberOfShapesDirectlyBelow` (method) `(self, ptr=None)`
- `partialPathName` (method) `(*a, **k)`
- `pop` (method) `(*a, **k)`
- `push` (method) `(*a, **k)`
- `set` (method) `(*a, **k)`
- `transform` (method) `(*a, **k)`


### MDagPathArray

Bases: `MDagPathArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MDataBlock

Bases: `MDataBlock`.

**Members**

- `context` (method) `(*a, **k)`
- `inputArrayValue` (method) `(*a, **k)`
- `inputValue` (method) `(*a, **k)`
- `isClean` (method) `(*a, **k)`
- `outputArrayValue` (method) `(*a, **k)`
- `outputValue` (method) `(*a, **k)`
- `setClean` (method) `(*a, **k)`


### MDataHandle

Bases: `MDataHandle`.

**Members**

- `asAngle` (method) `(*a, **k)`
- `asBool` (method) `(*a, **k)`
- `asChar` (method) `(*a, **k)`
- `asDistance` (method) `(*a, **k)`
- `asDouble` (method) `(*a, **k)`
- `asDouble3` (method) `(*a, **k)`
- `asFloat` (method) `(*a, **k)`
- `asFloat3` (method) `(*a, **k)`
- `asFloatMatrix` (method) `(*a, **k)`
- `asFloatVector` (method) `(*a, **k)`
- `asInt` (method) `(*a, **k)`
- `asInt3` (method) `(*a, **k)`
- `asInt64` (method) `(*a, **k)`
- `asLong` (method) `(*a, **k)`
- `asMObject` (method) `(*a, **k)`
- `asMatrix` (method) `(*a, **k)`
- `asMesh` (method) `(*a, **k)`
- `asMeshTransformed` (method) `(*a, **k)`
- `asPluginData` (method) `(*a, **k)`
- `asShort` (method) `(*a, **k)`
- `asShort3` (method) `(*a, **k)`
- `asString` (method) `(*a, **k)`
- `asTime` (method) `(*a, **k)`
- `asVector` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `copy` (method) `(*a, **k)`
- `data` (method) `(*a, **k)`
- `isNumeric` (method) `(*a, **k)`
- `set3Double` (method) `(*a, **k)`
- `set3Float` (method) `(*a, **k)`
- `set3Int` (method) `(*a, **k)`
- `set3Short` (method) `(*a, **k)`
- `setBool` (method) `(*a, **k)`
- `setChar` (method) `(*a, **k)`
- `setClean` (method) `(*a, **k)`
- `setDouble` (method) `(*a, **k)`
- `setFloat` (method) `(*a, **k)`
- `setInt` (method) `(*a, **k)`
- `setInt64` (method) `(*a, **k)`
- `setMAngle` (method) `(*a, **k)`
- `setMDistance` (method) `(*a, **k)`
- `setMFloatMatrix` (method) `(*a, **k)`
- `setMFloatVector` (method) `(*a, **k)`
- `setMMatrix` (method) `(*a, **k)`
- `setMObject` (method) `(*a, **k)`
- `setMPxData` (method) `(*a, **k)`
- `setMTime` (method) `(*a, **k)`
- `setMVector` (method) `(*a, **k)`
- `setMatrix` (method) `(*a, **k)`
- `setShort` (method) `(*a, **k)`
- `setString` (method) `(*a, **k)`


### MDistance

Bases: `MDistance`.

Constants: 10 names.

**Members**

- `asCentimeters` (method) `(*a, **k)`
- `asFeet` (method) `(*a, **k)`
- `asInches` (method) `(*a, **k)`
- `asKilometers` (method) `(*a, **k)`
- `asMeters` (method) `(*a, **k)`
- `asMiles` (method) `(*a, **k)`
- `asMillimeters` (method) `(*a, **k)`
- `asUnits` (method) `(*a, **k)`
- `asYards` (method) `(*a, **k)`
- `internalToUI` (method)
- `internalUnit` (method)
- `setUIUnit` (method)
- `uiToInternal` (method)
- `uiUnit` (method)


### MDoubleArray

Bases: `MDoubleArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MEulerRotation

Bases: `MEulerRotation`.

Constants: 7 names.

**Members**

- `alternateSolution` (method) `(*a, **k)`
- `asMatrix` (method) `(*a, **k)`
- `asQuaternion` (method) `(*a, **k)`
- `asVector` (method) `(*a, **k)`
- `bound` (method) `(*a, **k)`
- `boundIt` (method) `(*a, **k)`
- `closestCut` (method) `(*a, **k)`
- `closestSolution` (method) `(*a, **k)`
- `computeAlternateSolution` (method)
- `computeBound` (method)
- `computeClosestCut` (method)
- `computeClosestSolution` (method)
- `decompose` (method)
- `incrementalRotateBy` (method) `(*a, **k)`
- `inverse` (method) `(*a, **k)`
- `invertIt` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `isZero` (method) `(*a, **k)`
- `reorder` (method) `(*a, **k)`
- `reorderIt` (method) `(*a, **k)`
- `setToAlternateSolution` (method) `(*a, **k)`
- `setToClosestCut` (method) `(*a, **k)`
- `setToClosestSolution` (method) `(*a, **k)`
- `setToIdentity` (method) `(*a, **k)`
- `setValue` (method) `(*a, **k)`


### MEventMessage

Bases: `MEventMessage`.

Constants: 3 names.

**Members**

- `addEventCallback` (method)
- `currentCallbackId` (method)
- `getEventNames` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MFileObject

Bases: `MFileObject`.

Constants: 6 names.

**Members**

- `copy` (method) `(*a, **k)`
- `exists` (method) `(*a, **k)`
- `expandedFullName` (method) `(*a, **k)`
- `expandedPath` (method) `(*a, **k)`
- `isSet` (method) `(*a, **k)`
- `overrideResolvedFullName` (method) `(*a, **k)`
- `rawFullName` (method) `(*a, **k)`
- `rawName` (method) `(*a, **k)`
- `rawPath` (method) `(*a, **k)`
- `rawURI` (method) `(*a, **k)`
- `resolvedFullName` (method) `(*a, **k)`
- `resolvedName` (method) `(*a, **k)`
- `resolvedPath` (method) `(*a, **k)`
- `setRawFullName` (method) `(*a, **k)`
- `setRawName` (method) `(*a, **k)`
- `setRawPath` (method) `(*a, **k)`
- `setRawURI` (method) `(*a, **k)`


### MFloatArray

Bases: `MFloatArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MFloatMatrix

Bases: `MFloatMatrix`.

Constants: 1 names.

**Members**

- `adjoint` (method) `(*a, **k)`
- `det3x3` (method) `(*a, **k)`
- `det4x4` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `getElement` (method) `(*a, **k)`
- `homogenize` (method) `(*a, **k)`
- `inverse` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `isSingular` (method) `(*a, **k)`
- `setElement` (method) `(*a, **k)`
- `setToIdentity` (method) `(*a, **k)`
- `setToProduct` (method) `(*a, **k)`
- `transpose` (method) `(*a, **k)`


### MFloatPoint

Bases: `MFloatPoint`.

Constants: 1 names.

**Members**

- `cartesianize` (method) `(*a, **k)`
- `distanceTo` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `homogenize` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `rationalize` (method) `(*a, **k)`


### MFloatPointArray

Bases: `MFloatPointArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MFloatVector

Bases: `MFloatVector`.

Constants: 5 names.

**Members**

- `angle` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `isEquivalent` (method) `(*a, **k)`
- `isParallel` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `normalize` (method) `(*a, **k)`
- `rotateBy` (method) `(*a, **k)`
- `rotateTo` (method) `(*a, **k)`
- `transformAsNormal` (method) `(*a, **k)`


### MFloatVectorArray

Bases: `MFloatVectorArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MFn

Bases: `object`.

Constants: 1215 names.


### MFnAttribute

Bases: `MFnAttribute`.

Constants: 4 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnBase

Bases: `MFnBase`.

**Members**

- `hasObj` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `type` (method) `(*a, **k)`


### MFnComponent

Bases: `MFnComponent`.

**Members**

- `componentType` (method) `(*a, **k)`
- `elementCount` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasWeights` (method) `(*a, **k)`
- `isComplete` (method) `(*a, **k)`
- `isComponentType` (method) `(*a, **k)`
- `isElementCount` (method) `(*a, **k)`
- `isEmpty` (method) `(*a, **k)`
- `isEqual` (method) `(*a, **k)`
- `isHasWeights` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnCompoundAttribute

Bases: `MFnCompoundAttribute`.

Constants: 4 names.

**Members**

- `addChild` (method) `(*a, **k)`
- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `getAddAttrCmds` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `numChildren` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnDagNode

Bases: `MFnDagNode`.

Constants: 7 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addChild` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `boundingBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `dagPath` (method) `(*a, **k)`
- `dagRoot` (method) `(*a, **k)`
- `duplicate` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getAllPaths` (method) `(self, arr=None)`
- `getConnections` (method) `(*a, **k)`
- `getPath` (method) `(self, path=None)`
- `hasAttribute` (method) `(*a, **k)`
- `hasChild` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasParent` (method) `(*a, **k)`
- `inModel` (method) `(*a, **k)`
- `inUnderWorld` (method) `(*a, **k)`
- `instanceCount` (method) `(*a, **k)`
- `isBoundingBox` (method) `(*a, **k)`
- `isChildOf` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isInModel` (method) `(*a, **k)`
- `isInUnderWorld` (method) `(*a, **k)`
- `isInstanceable` (method) `(*a, **k)`
- `isIntermediateObject` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isObjectColor` (method) `(*a, **k)`
- `isParentOf` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `objectColor` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `parentCount` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `setIsIntermediateObject` (method) `(self, value)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnData

Bases: `MFnData`.

Constants: 21 names.


### MFnDependencyNode

Bases: `MFnDependencyNode`.

Constants: 6 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `attribute` (method) `(self, a)`
- `attributeCount` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getConnections` (method) `(self, arr=None)`
- `hasAttribute` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnDoubleIndexedComponent

Bases: `MFnDoubleIndexedComponent`.

**Members**

- `addElement` (method) `(*a, **k)`
- `addElements` (method) `(*a, **k)`
- `componentType` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `elementCount` (method) `(*a, **k)`
- `getElement` (method) `(self, i, a=None, b=None)`
- `getElements` (method) `(self, a=None, b=None)`
- `hasObj` (method) `(*a, **k)`
- `hasWeights` (method) `(*a, **k)`
- `isComplete` (method) `(*a, **k)`
- `isComponentType` (method) `(*a, **k)`
- `isElementCount` (method) `(*a, **k)`
- `isEmpty` (method) `(*a, **k)`
- `isEqual` (method) `(*a, **k)`
- `isHasWeights` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnEnumAttribute

Bases: `MFnEnumAttribute`.

Constants: 4 names.

**Members**

- `addField` (method) `(*a, **k)`
- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `fieldName` (method) `(*a, **k)`
- `fieldValue` (method) `(*a, **k)`
- `getMax` (method) `(*a, **k)`
- `getMin` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDefaultByName` (method) `(*a, **k)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnGenericAttribute

Bases: `MFnGenericAttribute`.

Constants: 4 names.

**Members**

- `addDataType` (method) `(*a, **k)`
- `addNumericType` (method) `(*a, **k)`
- `addTypeId` (method) `(*a, **k)`
- `addUnitType` (method) `(*a, **k)`
- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnGeometryData

Bases: `MFnGeometryData`.

**Members**

- `componentTags` (method) `(*a, **k)`
- `hasComponentTag` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `resolveComponentTagExpression` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnMatrixAttribute

Bases: `MFnMatrixAttribute`.

Constants: 6 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `default` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDefault` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDefault` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnMatrixData

Bases: `MFnMatrixData`.

**Members**

- `create` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `matrix` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `set` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `transformation` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnMesh

Bases: `MFnMesh`.

Constants: 12 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addChild` (method) `(*a, **k)`
- `allIntersections` (method) `(self, source, direction, *args, **kw)`
- `anyIntersection` (method) `(self, source, direction, *args, **kw)`
- `area` (method) `(*a, **k)`
- `assignUVs` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `autoUniformGridParams` (method)
- `boundingBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `closestIntersection` (method) `(self, source, direction, *args, **kw)`
- `copyUVSet` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `createUVSet` (method) `(*a, **k)`
- `createUVSetDataMesh` (method) `(*a, **k)`
- `createUVSetWithName` (method) `(*a, **k)`
- `currentUVSetName` (method) `(*a, **k)`
- `dagPath` (method) `(*a, **k)`
- `dagRoot` (method) `(*a, **k)`
- `deleteUVSet` (method) `(*a, **k)`
- `duplicate` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getAllPaths` (method) `(*a, **k)`
- `getAssignedUVs` (method) `(self, *args)`
- `getClosestNormal` (method) `(self, point, out=None, space=2, face_ptr=None, accel=None)`
- `getClosestPoint` (method) `(self, point, out=None, space=2, face_ptr=None, accel=None)`
- `getClosestPointAndNormal` (method) `(self, point, out_p=None, out_n=None, space=2, face_ptr=None, accel=None)`
- `getConnections` (method) `(*a, **k)`
- `getCurrentUVSetName` (method) `(self, *args)`
- `getEdgeVertices` (method) `(self, e, ptr=None)`
- `getFaceNormalIds` (method) `(*a, **k)`
- `getFaceVertexNormal` (method) `(self, face, vert, out=None, space=2)`
- `getFaceVertexNormals` (method) `(self, face, arr=None, space=2)`
- `getFloatPoints` (method) `(self, arr=None, space=2)`
- `getNormalIds` (method) `(*a, **k)`
- `getNormals` (method) `(self, arr=None, space=2)`
- `getPath` (method) `(*a, **k)`
- `getPoint` (method) `(self, i, pt=None, space=2)`
- `getPointAtUV` (method) `(*a, **k)`
- `getPoints` (method) `(self, arr=None, space=2)`
- `getPolygonNormal` (method) `(self, i, out=None, space=2)`
- `getPolygonTriangleVertices` (method) `(*a, **k)`
- `getPolygonUV` (method) `(self, face, vert, u=None, v=None, uv_set='')`
- `getPolygonUVid` (method) `(self, face, vert, ptr=None, uv_set='')`
- `getPolygonVertices` (method) `(self, i, ids=None)`
- `getTriangles` (method) `(self, counts=None, verts=None)`
- `getUVSetNames` (method) `(self, arr=None)`
- `getUVs` (method) `(self, *args)`
- `getVertexNormal` (method) `(self, vid, *args)`
- `getVertexNormals` (method) `(self, weighted, arr=None, space=2)`
- `getVertices` (method) `(self, counts=None, ids=None)`
- `globalIntersectionTolerance` (method)
- `hasAttribute` (method) `(*a, **k)`
- `hasChild` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasParent` (method) `(*a, **k)`
- `inModel` (method) `(*a, **k)`
- `inUnderWorld` (method) `(*a, **k)`
- `instanceCount` (method) `(*a, **k)`
- `isBoundingBox` (method) `(*a, **k)`
- `isChildOf` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isInModel` (method) `(*a, **k)`
- `isInUnderWorld` (method) `(*a, **k)`
- `isInstanceable` (method) `(*a, **k)`
- `isIntermediateObject` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isNumColorSets` (method) `(*a, **k)`
- `isNumEdges` (method) `(*a, **k)`
- `isNumFaceVertices` (method) `(*a, **k)`
- `isNumNormals` (method) `(*a, **k)`
- `isNumPolygons` (method) `(*a, **k)`
- `isNumVertices` (method) `(*a, **k)`
- `isObjectColor` (method) `(*a, **k)`
- `isParentOf` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isPolygonConvex` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `numColorSets` (method) `(*a, **k)`
- `numColors` (method) `(*a, **k)`
- `numEdges` (method) `(*a, **k)`
- `numFaceVertices` (method) `(*a, **k)`
- `numNormals` (method) `(*a, **k)`
- `numPolygons` (method) `(*a, **k)`
- `numUVSets` (method) `(*a, **k)`
- `numUVs` (method) `(*a, **k)`
- `numVertices` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `objectColor` (method) `(*a, **k)`
- `onBoundary` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `parentCount` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `polygonVertexCount` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `renameUVSet` (method) `(*a, **k)`
- `setCurrentUVSetName` (method) `(*a, **k)`
- `setIsIntermediateObject` (method) `(self, value)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `setPoint` (method) `(*a, **k)`
- `setPoints` (method) `(*a, **k)`
- `setUVs` (method) `(*a, **k)`
- `syncObject` (method) `(*a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `uniformGridParams` (method)
- `updateSurface` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnMessageAttribute

Bases: `MFnMessageAttribute`.

Constants: 4 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnNumericAttribute

Bases: `MFnNumericAttribute`.

Constants: 4 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `createColor` (method) `(*a, **k)`
- `createPoint` (method) `(*a, **k)`
- `default` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `getMax` (method) `(*a, **k)`
- `getMin` (method) `(*a, **k)`
- `hasMax` (method) `(*a, **k)`
- `hasMin` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDefault` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `numericType` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDefault` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setMax` (method) `(*a, **k)`
- `setMin` (method) `(*a, **k)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setSoftMax` (method) `(*a, **k)`
- `setSoftMin` (method) `(*a, **k)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnNumericData

Bases: `MFnNumericData`.

Constants: 23 names.

**Members**

- `create` (method) `(*a, **k)`
- `getData` (method) `(self, *ptrs)`
- `hasObj` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setData` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnNurbsCurve

Bases: `MFnNurbsCurve`.

Constants: 14 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addChild` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `boundingBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `closestPoint` (method) `(self, point, *args)`
- `create` (method) `(*a, **k)`
- `cvPosition` (method) `(*a, **k)`
- `cvPositions` (method) `(*a, **k)`
- `dagPath` (method) `(*a, **k)`
- `dagRoot` (method) `(*a, **k)`
- `degree` (method) `(*a, **k)`
- `duplicate` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findLengthFromParam` (method) `(*a, **k)`
- `findParamFromLength` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `form` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getAllPaths` (method) `(*a, **k)`
- `getCV` (method) `(self, i, pt=None, space=2)`
- `getCVs` (method) `(self, arr=None, space=2)`
- `getConnections` (method) `(*a, **k)`
- `getKnotDomain` (method) `(self, a, b)`
- `getKnots` (method) `(self, arr=None)`
- `getParamAtPoint` (method) `(self, point, ptr=None, *args)`
- `getPath` (method) `(*a, **k)`
- `getPointAtParam` (method) `(self, param, pt=None, space=2)`
- `hasAttribute` (method) `(*a, **k)`
- `hasChild` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasParent` (method) `(*a, **k)`
- `inModel` (method) `(*a, **k)`
- `inUnderWorld` (method) `(*a, **k)`
- `instanceCount` (method) `(*a, **k)`
- `isBoundingBox` (method) `(*a, **k)`
- `isChildOf` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isDegree` (method) `(*a, **k)`
- `isForm` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isInModel` (method) `(*a, **k)`
- `isInUnderWorld` (method) `(*a, **k)`
- `isInstanceable` (method) `(*a, **k)`
- `isIntermediateObject` (method) `(*a, **k)`
- `isKnotDomain` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isNumCVs` (method) `(*a, **k)`
- `isNumKnots` (method) `(*a, **k)`
- `isNumSpans` (method) `(*a, **k)`
- `isObjectColor` (method) `(*a, **k)`
- `isParamOnCurve` (method) `(*a, **k)`
- `isParentOf` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isPointOnCurve` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `knot` (method) `(*a, **k)`
- `knotDomain` (method) `(*a, **k)`
- `knots` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `numCVs` (method) `(*a, **k)`
- `numKnots` (method) `(*a, **k)`
- `numSpans` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `objectColor` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `parentCount` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `setCV` (method) `(self, i, pt, space=2)`
- `setCVPosition` (method) `(*a, **k)`
- `setCVPositions` (method) `(*a, **k)`
- `setCVs` (method) `(self, pts, space=2)`
- `setIsIntermediateObject` (method) `(self, value)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `tangent` (method) `(*a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `updateCurve` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnNurbsSurface

Bases: `MFnNurbsSurface`.

Constants: 18 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addChild` (method) `(*a, **k)`
- `area` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `boundingBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `closestPoint` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `cvPosition` (method) `(*a, **k)`
- `cvPositions` (method) `(*a, **k)`
- `dagPath` (method) `(*a, **k)`
- `dagRoot` (method) `(*a, **k)`
- `degreeInU` (method) `(*a, **k)`
- `degreeInV` (method) `(*a, **k)`
- `distanceToPoint` (method) `(*a, **k)`
- `duplicate` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `formInU` (method) `(*a, **k)`
- `formInV` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getAllPaths` (method) `(*a, **k)`
- `getConnections` (method) `(*a, **k)`
- `getDerivativesAtParam` (method) `(*a, **k)`
- `getParamAtPoint` (method) `(*a, **k)`
- `getPath` (method) `(*a, **k)`
- `getPointAtParam` (method) `(*a, **k)`
- `hasAttribute` (method) `(*a, **k)`
- `hasChild` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasParent` (method) `(*a, **k)`
- `inModel` (method) `(*a, **k)`
- `inUnderWorld` (method) `(*a, **k)`
- `instanceCount` (method) `(*a, **k)`
- `isBoundingBox` (method) `(*a, **k)`
- `isChildOf` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isDegreeInU` (method) `(*a, **k)`
- `isDegreeInV` (method) `(*a, **k)`
- `isFormInU` (method) `(*a, **k)`
- `isFormInV` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isInModel` (method) `(*a, **k)`
- `isInUnderWorld` (method) `(*a, **k)`
- `isInstanceable` (method) `(*a, **k)`
- `isIntermediateObject` (method) `(*a, **k)`
- `isKnotDomainInU` (method) `(*a, **k)`
- `isKnotDomainInV` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isNumCVsInU` (method) `(*a, **k)`
- `isNumCVsInV` (method) `(*a, **k)`
- `isNumKnotsInU` (method) `(*a, **k)`
- `isNumKnotsInV` (method) `(*a, **k)`
- `isNumPatches` (method) `(*a, **k)`
- `isNumPatchesInU` (method) `(*a, **k)`
- `isNumPatchesInV` (method) `(*a, **k)`
- `isNumSpansInU` (method) `(*a, **k)`
- `isNumSpansInV` (method) `(*a, **k)`
- `isObjectColor` (method) `(*a, **k)`
- `isParamOnSurface` (method) `(*a, **k)`
- `isParentOf` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isPointOnSurface` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTrimmedSurface` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `knotDomainInU` (method) `(*a, **k)`
- `knotDomainInV` (method) `(*a, **k)`
- `knotInU` (method) `(*a, **k)`
- `knotInV` (method) `(*a, **k)`
- `knotsInU` (method) `(*a, **k)`
- `knotsInV` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `numCVsInU` (method) `(*a, **k)`
- `numCVsInV` (method) `(*a, **k)`
- `numKnotsInU` (method) `(*a, **k)`
- `numKnotsInV` (method) `(*a, **k)`
- `numPatches` (method) `(*a, **k)`
- `numPatchesInU` (method) `(*a, **k)`
- `numPatchesInV` (method) `(*a, **k)`
- `numSpansInU` (method) `(*a, **k)`
- `numSpansInV` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `objectColor` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `parentCount` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `setIsIntermediateObject` (method) `(self, value)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `tangents` (method) `(*a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnPlugin

Bases: `MFnPlugin`.

**Members**

- `apiVersion` (method) `(*a, **k)`
- `deregisterCommand` (method) `(*a, **k)`
- `deregisterData` (method) `(*a, **k)`
- `deregisterFileTranslator` (method) `(*a, **k)`
- `deregisterNode` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `loadPath` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `registerCommand` (method) `(*a, **k)`
- `registerData` (method) `(*a, **k)`
- `registerFileTranslator` (method) `(*a, **k)`
- `registerNode` (method) `(*a, **k)`
- `registerShape` (method) `(*a, **k)`
- `registerUI` (method) `(*a, **k)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `vendor` (method) `(*a, **k)`
- `version` (method) `(*a, **k)`


### MFnPluginData

Bases: `MFnPluginData`.

**Members**

- `create` (method) `(*a, **k)`
- `data` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`


### MFnSet

Bases: `MFnSet`.

Constants: 12 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addMember` (method) `(*a, **k)`
- `addMembers` (method) `(*a, **k)`
- `annotation` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `clear` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getConnections` (method) `(*a, **k)`
- `getMembers` (method) `(self, sel, flatten=False)`
- `hasAttribute` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `isAnnotation` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isMember` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeMember` (method) `(*a, **k)`
- `removeMembers` (method) `(*a, **k)`
- `restriction` (method) `(*a, **k)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnSingleIndexedComponent

Bases: `MFnSingleIndexedComponent`.

**Members**

- `addElement` (method) `(*a, **k)`
- `addElements` (method) `(*a, **k)`
- `componentType` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `element` (method) `(*a, **k)`
- `elementCount` (method) `(*a, **k)`
- `getCompleteData` (method) `(*a, **k)`
- `getElements` (method) `(self, arr=None)`
- `hasObj` (method) `(*a, **k)`
- `hasWeights` (method) `(*a, **k)`
- `isComplete` (method) `(*a, **k)`
- `isComponentType` (method) `(*a, **k)`
- `isElementCount` (method) `(*a, **k)`
- `isEmpty` (method) `(*a, **k)`
- `isEqual` (method) `(*a, **k)`
- `isHasWeights` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `setCompleteData` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnStringData

Bases: `MFnStringData`.

**Members**

- `create` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `set` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `string` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`


### MFnTransform

Bases: `MFnTransform`.

Constants: 8 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addChild` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `boundingBox` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `childCount` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `dagPath` (method) `(*a, **k)`
- `dagRoot` (method) `(*a, **k)`
- `duplicate` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getAllPaths` (method) `(*a, **k)`
- `getConnections` (method) `(*a, **k)`
- `getPath` (method) `(*a, **k)`
- `getRotation` (method) `(self, out, space=1, *rest)`
- `getRotationQuaternion` (method) `(self, x, y, z, w, space=1)`
- `getScale` (method) `(self, ptr)`
- `getShear` (method) `(self, ptr)`
- `hasAttribute` (method) `(*a, **k)`
- `hasChild` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hasParent` (method) `(*a, **k)`
- `inModel` (method) `(*a, **k)`
- `inUnderWorld` (method) `(*a, **k)`
- `instanceCount` (method) `(*a, **k)`
- `isBoundingBox` (method) `(*a, **k)`
- `isChildOf` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isInModel` (method) `(*a, **k)`
- `isInUnderWorld` (method) `(*a, **k)`
- `isInstanceable` (method) `(*a, **k)`
- `isIntermediateObject` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isObjectColor` (method) `(*a, **k)`
- `isParentOf` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `objectColor` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `parentCount` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `removeChild` (method) `(*a, **k)`
- `resetFromRestPosition` (method) `(*a, **k)`
- `rotateBy` (method) `(*a, **k)`
- `rotateOrientation` (method) `(*a, **k)`
- `rotatePivot` (method) `(*a, **k)`
- `rotatePivotTranslation` (method) `(*a, **k)`
- `rotation` (method) `(self, space=1)`
- `rotationOrder` (method) `(*a, **k)`
- `scale` (method) `(*a, **k)`
- `scaleBy` (method) `(self, ptr)`
- `scalePivot` (method) `(*a, **k)`
- `scalePivotTranslation` (method) `(*a, **k)`
- `setIsIntermediateObject` (method) `(self, value)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(self, *a, **k)`
- `setRotateOrientation` (method) `(*a, **k)`
- `setRotatePivot` (method) `(*a, **k)`
- `setRotatePivotTranslation` (method) `(*a, **k)`
- `setRotation` (method) `(self, rot, *rest)`
- `setRotationOrder` (method) `(*a, **k)`
- `setRotationQuaternion` (method) `(self, x, y, z, w, space=1)`
- `setScale` (method) `(self, ptr)`
- `setScalePivot` (method) `(*a, **k)`
- `setScalePivotTranslation` (method) `(*a, **k)`
- `setShear` (method) `(*a, **k)`
- `setTransformation` (method) `(*a, **k)`
- `setTranslation` (method) `(*a, **k)`
- `shear` (method) `(*a, **k)`
- `shearBy` (method) `(*a, **k)`
- `transformation` (method) `(*a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `translateBy` (method) `(*a, **k)`
- `translation` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


### MFnTypedAttribute

Bases: `MFnTypedAttribute`.

Constants: 4 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `attrType` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MFnUnitAttribute

Bases: `MFnUnitAttribute`.

Constants: 9 names.

**Members**

- `affectsAppearance` (method) `(*a, **k)`
- `affectsWorldSpace` (method) `(*a, **k)`
- `array` (method) `(*a, **k)`
- `cached` (method) `(*a, **k)`
- `channelBox` (method) `(*a, **k)`
- `connectable` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `createColor` (method) `(*a, **k)`
- `createPoint` (method) `(*a, **k)`
- `default` (method) `(*a, **k)`
- `disconnectBehavior` (method) `(*a, **k)`
- `dynamic` (method) `(*a, **k)`
- `getMax` (method) `(*a, **k)`
- `getMin` (method) `(*a, **k)`
- `hasMax` (method) `(*a, **k)`
- `hasMin` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `hidden` (method) `(*a, **k)`
- `indeterminant` (method) `(*a, **k)`
- `indexMatters` (method) `(*a, **k)`
- `isAffectsAppearance` (method) `(*a, **k)`
- `isAffectsWorldSpace` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCached` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isConnectable` (method) `(*a, **k)`
- `isDefault` (method) `(*a, **k)`
- `isDisconnectBehavior` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isFreeToChange` (method) `(*a, **k)`
- `isHidden` (method) `(*a, **k)`
- `isIndeterminant` (method) `(*a, **k)`
- `isIndexMatters` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isName` (method) `(*a, **k)`
- `isNiceName` (method) `(*a, **k)`
- `isReadable` (method) `(*a, **k)`
- `isRenderSource` (method) `(*a, **k)`
- `isShortName` (method) `(*a, **k)`
- `isStorable` (method) `(*a, **k)`
- `isUsedAsColor` (method) `(*a, **k)`
- `isUsedAsFilename` (method) `(*a, **k)`
- `isUsesArrayDataBuilder` (method) `(*a, **k)`
- `isWorldSpace` (method) `(*a, **k)`
- `isWritable` (method) `(*a, **k)`
- `keyable` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `niceName` (method) `(*a, **k)`
- `numericType` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `readable` (method) `(*a, **k)`
- `renderSource` (method) `(*a, **k)`
- `setAffectsAppearance` (method) `(self, value)`
- `setAffectsWorldSpace` (method) `(self, value)`
- `setArray` (method) `(self, value)`
- `setCached` (method) `(self, value)`
- `setChannelBox` (method) `(self, value)`
- `setConnectable` (method) `(self, value)`
- `setDefault` (method) `(self, value)`
- `setDisconnectBehavior` (method) `(self, value)`
- `setDynamic` (method) `(self, value)`
- `setHidden` (method) `(self, value)`
- `setIndeterminant` (method) `(self, value)`
- `setIndexMatters` (method) `(self, value)`
- `setKeyable` (method) `(self, value)`
- `setMax` (method) `(*a, **k)`
- `setMin` (method) `(*a, **k)`
- `setNiceName` (method) `(self, value)`
- `setNiceNameOverride` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setReadable` (method) `(self, value)`
- `setRenderSource` (method) `(self, value)`
- `setSoftMax` (method) `(*a, **k)`
- `setSoftMin` (method) `(*a, **k)`
- `setStorable` (method) `(self, value)`
- `setUsedAsColor` (method) `(self, value)`
- `setUsedAsFilename` (method) `(self, value)`
- `setUsesArrayDataBuilder` (method) `(self, value)`
- `setWorldSpace` (method) `(self, value)`
- `setWritable` (method) `(self, value)`
- `shortName` (method) `(*a, **k)`
- `storable` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `unitType` (method) `(*a, **k)`
- `usedAsColor` (method) `(*a, **k)`
- `usedAsFilename` (method) `(*a, **k)`
- `usesArrayDataBuilder` (method) `(*a, **k)`
- `worldSpace` (method) `(*a, **k)`
- `writable` (method) `(*a, **k)`


### MGlobal

Bases: `MGlobal`.

Constants: 11 names.

**Members**

- `apiVersion` (method)
- `clearSelectionList` (method)
- `displayError` (method)
- `displayInfo` (method)
- `displayWarning` (method)
- `executeCommand` (method)
- `executeCommandOnIdle` (method)
- `executeCommandStringResult` (method)
- `executePythonCommand` (method)
- `executePythonCommandOnIdle` (method)
- `executePythonCommandStringResult` (method)
- `getActiveSelectionList` (method)
- `getFunctionSetList` (method)
- `getHiliteList` (method)
- `getLiveList` (method)
- `getSelectionListByName` (method)
- `isRedoing` (method)
- `isUndoing` (method)
- `isYAxisUp` (method)
- `isZAxisUp` (method)
- `mayaState` (method)
- `mayaVersion` (method)
- `select` (method)
- `selectByName` (method)
- `setActiveSelectionList` (method)
- `setYAxisUp` (method)
- `setZAxisUp` (method)
- `upAxis` (method)


### MIntArray

Bases: `MIntArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MItDag

Bases: `MItDag`.

Constants: 3 names.

**Members**

- `currentItem` (method) `(*a, **k)`
- `depth` (method) `(*a, **k)`
- `fullPathName` (method) `(*a, **k)`
- `getAllPaths` (method) `(self, arr)`
- `getPath` (method) `(self, path=None)`
- `instanceCount` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `isInstanced` (method) `(*a, **k)`
- `item` (method) `(self)`
- `next` (method) `(*a, **k)`
- `partialPathName` (method) `(*a, **k)`
- `prune` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `root` (method) `(*a, **k)`
- `traversalType` (method) `(*a, **k)`


### MItDependencyGraph

Bases: `MItDependencyGraph`.

Constants: 6 names.

**Members**

- `currentFilter` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `currentNode` (method) `(*a, **k)`
- `currentPlug` (method) `(*a, **k)`
- `getNodesVisited` (method) `(self, arr=None)`
- `getPlugsVisited` (method) `(self, arr=None)`
- `isDone` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `prune` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `resetTo` (method) `(*a, **k)`
- `rootNode` (method) `(*a, **k)`
- `rootPlug` (method) `(*a, **k)`
- `thisPlug` (method) `(*a, **k)`


### MItDependencyNodes

Bases: `MItDependencyNodes`.

**Members**

- `isDone` (method) `(*a, **k)`
- `item` (method) `(self)`
- `next` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `thisNode` (method) `(*a, **k)`


### MItGeometry

Bases: `MItGeometry`.

**Members**

- `allPositions` (method) `(self, arr=None, space=2)`
- `count` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `exactCount` (method) `(*a, **k)`
- `index` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `position` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setAllPositions` (method) `(*a, **k)`
- `setPosition` (method) `(*a, **k)`
- `weight` (method) `(*a, **k)`


### MItMeshEdge

Bases: `MItMeshEdge`.

**Members**

- `center` (method) `(*a, **k)`
- `connectedEdges` (method) `(*a, **k)`
- `connectedFaces` (method) `(*a, **k)`
- `connectedToEdge` (method) `(*a, **k)`
- `connectedToFace` (method) `(*a, **k)`
- `count` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `getConnectedEdges` (method) `(self, arr=None)`
- `getConnectedFaces` (method) `(self, arr=None)`
- `getLength` (method) `(self, ptr=None, space=2)`
- `index` (method) `(self, which=None)`
- `isDone` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `numConnectedEdges` (method) `(self, ptr=None)`
- `numConnectedFaces` (method) `(self, ptr=None)`
- `onBoundary` (method) `(*a, **k)`
- `point` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setIndex` (method) `(self, i, prev=None)`
- `setPoint` (method) `(*a, **k)`
- `updateSurface` (method) `(*a, **k)`
- `vertexId` (method) `(*a, **k)`


### MItMeshFaceVertex

Bases: `MItMeshFaceVertex`.

**Members**

- `count` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `faceId` (method) `(*a, **k)`
- `faceVertex` (method) `(*a, **k)`
- `faceVertexId` (method) `(*a, **k)`
- `getColor` (method) `(*a, **k)`
- `getNormal` (method) `(self, out=None, space=2)`
- `getUV` (method) `(self, *args)`
- `getUVIndex` (method) `(*a, **k)`
- `hasColor` (method) `(*a, **k)`
- `hasUVs` (method) `(*a, **k)`
- `index` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `normalId` (method) `(*a, **k)`
- `position` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setIndex` (method) `(*a, **k)`
- `vertexId` (method) `(*a, **k)`


### MItMeshPolygon

Bases: `MItMeshPolygon`.

**Members**

- `center` (method) `(*a, **k)`
- `connectedEdges` (method) `(*a, **k)`
- `connectedFaces` (method) `(*a, **k)`
- `connectedVertices` (method) `(*a, **k)`
- `count` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `edgeCount` (method) `(*a, **k)`
- `getArea` (method) `(self, ptr=None, space=2)`
- `getConnectedEdges` (method) `(self, arr=None)`
- `getConnectedFaces` (method) `(self, arr=None)`
- `getConnectedVertices` (method) `(self, arr=None)`
- `getEdges` (method) `(self, arr=None)`
- `getNormal` (method) `(self, out=None, *args)`
- `getNormals` (method) `(self, arr=None, space=2)`
- `getPoints` (method) `(self, arr=None, space=2)`
- `getTriangle` (method) `(*a, **k)`
- `getTriangles` (method) `(self, pts=None, ids=None, space=2)`
- `getUV` (method) `(self, which, *args)`
- `getUVIndex` (method) `(self, which, *args)`
- `getUVs` (method) `(self, *args)`
- `getVertices` (method) `(self, arr=None)`
- `hasUVs` (method) `(*a, **k)`
- `index` (method) `(*a, **k)`
- `isConvex` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `isPlanar` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `normalIndex` (method) `(*a, **k)`
- `numConnectedEdges` (method) `(self, ptr=None)`
- `numConnectedFaces` (method) `(self, ptr=None)`
- `numTriangles` (method) `(self, ptr=None)`
- `numUVs` (method) `(self, ptr=None)`
- `onBoundary` (method) `(*a, **k)`
- `point` (method) `(*a, **k)`
- `polygonVertexCount` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setIndex` (method) `(self, i, prev=None)`
- `setPoint` (method) `(*a, **k)`
- `setPoints` (method) `(*a, **k)`
- `updateSurface` (method) `(*a, **k)`
- `vertexIndex` (method) `(*a, **k)`
- `zeroArea` (method) `(*a, **k)`


### MItMeshVertex

Bases: `MItMeshVertex`.

**Members**

- `connectedEdges` (method) `(*a, **k)`
- `connectedFaces` (method) `(*a, **k)`
- `connectedToEdge` (method) `(*a, **k)`
- `connectedToFace` (method) `(*a, **k)`
- `connectedVertices` (method) `(*a, **k)`
- `count` (method) `(*a, **k)`
- `currentItem` (method) `(*a, **k)`
- `getColor` (method) `(*a, **k)`
- `getConnectedEdges` (method) `(self, arr=None)`
- `getConnectedFaces` (method) `(self, arr=None)`
- `getConnectedVertices` (method) `(self, arr=None)`
- `getNormal` (method) `(self, out=None, *args)`
- `getNormals` (method) `(self, arr=None, space=2)`
- `getUV` (method) `(self, *args)`
- `getUVs` (method) `(self, *args)`
- `hasUVs` (method) `(*a, **k)`
- `index` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `numConnectedEdges` (method) `(self, ptr=None)`
- `numConnectedFaces` (method) `(self, ptr=None)`
- `numUVs` (method) `(self, ptr=None)`
- `onBoundary` (method) `(*a, **k)`
- `position` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setIndex` (method) `(self, i, prev=None)`
- `setPosition` (method) `(*a, **k)`
- `translateBy` (method) `(*a, **k)`
- `updateSurface` (method) `(*a, **k)`


### MItSelectionList

Bases: `MItSelectionList`.

Constants: 5 names.

**Members**

- `getComponent` (method) `(*a, **k)`
- `getDagPath` (method) `(self, path=None, comp=None)`
- `getDependNode` (method) `(self, obj=None)`
- `getPlug` (method) `(self, plug=None)`
- `getStrings` (method) `(self, arr=None)`
- `hasComponents` (method) `(*a, **k)`
- `isDone` (method) `(*a, **k)`
- `itemType` (method) `(*a, **k)`
- `next` (method) `(*a, **k)`
- `reset` (method) `(*a, **k)`
- `setFilter` (method) `(*a, **k)`


### MMatrix

Bases: `MMatrix`.

Constants: 1 names.

**Members**

- `adjoint` (method) `(*a, **k)`
- `det3x3` (method) `(*a, **k)`
- `det4x4` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `getElement` (method) `(*a, **k)`
- `homogenize` (method) `(*a, **k)`
- `inverse` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `isSingular` (method) `(*a, **k)`
- `setElement` (method) `(*a, **k)`
- `setToIdentity` (method) `(*a, **k)`
- `setToProduct` (method) `(*a, **k)`
- `transpose` (method) `(*a, **k)`


### MMatrixArray

Bases: `MMatrixArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MMeshIsectAccelParams

Bases: `MMeshIsectAccelParams`.


### MMessage

Bases: `MMessage`.

Constants: 3 names.

**Members**

- `currentCallbackId` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MNodeMessage

Bases: `MNodeMessage`.

Constants: 23 names.

**Members**

- `addAttributeAddedOrRemovedCallback` (method)
- `addAttributeChangedCallback` (method)
- `addKeyableChangeOverride` (method)
- `addNameChangedCallback` (method)
- `addNodeAboutToDeleteCallback` (method)
- `addNodeDestroyedCallback` (method)
- `addNodeDirtyCallback` (method)
- `addNodeDirtyPlugCallback` (method)
- `addNodePreRemovalCallback` (method)
- `addUuidChangedCallback` (method)
- `currentCallbackId` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MObject

Bases: `MObject`.

**Members**

- `apiType` (method) `(*a, **k)`
- `apiTypeStr` (method) `(*a, **k)`
- `hasFn` (method) `(*a, **k)`
- `isApiTypeStr` (method) `(*a, **k)`
- `isNull` (method) `(*a, **k)`


### MObjectArray

Bases: `MObjectArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MObjectHandle

Bases: `MObjectHandle`.

**Members**

- `assign` (method) `(*a, **k)`
- `hashCode` (method) `(*a, **k)`
- `isAlive` (method) `(*a, **k)`
- `isValid` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`


### MPlane

Bases: `MPlane`.

**Members**

- `directedDistance` (method) `(*a, **k)`
- `distance` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `relativeDistance` (method) `(*a, **k)`
- `setPlane` (method) `(*a, **k)`


### MPlug

Bases: `MPlug`.

Constants: 7 names.

**Members**

- `array` (method) `(*a, **k)`
- `asBool` (method) `(*a, **k)`
- `asChar` (method) `(*a, **k)`
- `asDouble` (method) `(*a, **k)`
- `asFloat` (method) `(*a, **k)`
- `asInt` (method) `(*a, **k)`
- `asMAngle` (method) `(*a, **k)`
- `asMDistance` (method) `(*a, **k)`
- `asMObject` (method) `(*a, **k)`
- `asMTime` (method) `(*a, **k)`
- `asShort` (method) `(*a, **k)`
- `asString` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `child` (method) `(*a, **k)`
- `connectedTo` (method) `(self, *args)`
- `destinations` (method) `(*a, **k)`
- `destinationsWithConversions` (method) `(*a, **k)`
- `elementByLogicalIndex` (method) `(*a, **k)`
- `elementByPhysicalIndex` (method) `(*a, **k)`
- `evaluateNumElements` (method) `(*a, **k)`
- `getExistingArrayAttributeIndices` (method) `(self, arr=None)`
- `getValue` (method) `(self, out)`
- `info` (method) `(*a, **k)`
- `isArray` (method) `(*a, **k)`
- `isCaching` (method) `(*a, **k)`
- `isChannelBox` (method) `(*a, **k)`
- `isChild` (method) `(*a, **k)`
- `isCompound` (method) `(*a, **k)`
- `isConnected` (method) `(*a, **k)`
- `isDefaultValue` (method) `(*a, **k)`
- `isDestination` (method) `(*a, **k)`
- `isDynamic` (method) `(*a, **k)`
- `isElement` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isIgnoredWhenRendering` (method) `(*a, **k)`
- `isInfo` (method) `(*a, **k)`
- `isKeyable` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNetworked` (method) `(*a, **k)`
- `isNull` (method) `(*a, **k)`
- `isProcedural` (method) `(*a, **k)`
- `isSource` (method) `(*a, **k)`
- `logicalIndex` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `node` (method) `(*a, **k)`
- `numChildren` (method) `(*a, **k)`
- `numElements` (method) `(*a, **k)`
- `parent` (method) `(*a, **k)`
- `partialName` (method) `(*a, **k)`
- `setBool` (method) `(*a, **k)`
- `setChannelBox` (method) `(*a, **k)`
- `setChar` (method) `(*a, **k)`
- `setDouble` (method) `(*a, **k)`
- `setFloat` (method) `(*a, **k)`
- `setInt` (method) `(*a, **k)`
- `setKeyable` (method) `(*a, **k)`
- `setLocked` (method) `(*a, **k)`
- `setMAngle` (method) `(*a, **k)`
- `setMDistance` (method) `(*a, **k)`
- `setMObject` (method) `(*a, **k)`
- `setMTime` (method) `(*a, **k)`
- `setShort` (method) `(*a, **k)`
- `setString` (method) `(*a, **k)`
- `setValue` (method) `(self, v)`
- `source` (method) `(*a, **k)`
- `sourceWithConversion` (method) `(*a, **k)`


### MPlugArray

Bases: `MPlugArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MPoint

Bases: `MPoint`.

Constants: 1 names.

**Members**

- `cartesianize` (method) `(*a, **k)`
- `distanceTo` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `homogenize` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `rationalize` (method) `(*a, **k)`


### MPointArray

Bases: `MPointArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MQuaternion

Bases: `MQuaternion`.

Constants: 1 names.

**Members**

- `asAxisAngle` (method) `(*a, **k)`
- `asEulerRotation` (method) `(*a, **k)`
- `asMatrix` (method) `(*a, **k)`
- `conjugate` (method) `(*a, **k)`
- `conjugateIt` (method) `(*a, **k)`
- `exp` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `inverse` (method) `(*a, **k)`
- `invertIt` (method) `(*a, **k)`
- `isEquivalent` (method) `(*a, **k)`
- `log` (method) `(*a, **k)`
- `negateIt` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `normalizeIt` (method) `(*a, **k)`
- `setToXAxis` (method) `(*a, **k)`
- `setToYAxis` (method) `(*a, **k)`
- `setToZAxis` (method) `(*a, **k)`
- `setValue` (method) `(*a, **k)`
- `slerp` (method)
- `squad` (method)


### MRampAttribute

Bases: `MRampAttribute`.

Constants: 4 names.

**Members**

- `addEntries` (method) `(*a, **k)`
- `createColorRamp` (method)
- `createCurveRamp` (method)
- `createRamp` (method)
- `deleteEntries` (method) `(*a, **k)`
- `getColorAtPosition` (method) `(self, position, out=None)`
- `getEntries` (method) `(self, indices=None, positions=None, values=None, interps=None)`
- `getValueAtPosition` (method) `(self, position, out=None)`
- `hasIndex` (method) `(*a, **k)`
- `isColorRamp` (method) `(*a, **k)`
- `isCurveRamp` (method) `(*a, **k)`
- `numEntries` (method) `(*a, **k)`
- `pack` (method) `(*a, **k)`
- `setColorAtIndex` (method) `(*a, **k)`
- `setInterpolationAtIndex` (method) `(*a, **k)`
- `setPositionAtIndex` (method) `(*a, **k)`
- `setRamp` (method) `(*a, **k)`
- `setValueAtIndex` (method) `(*a, **k)`
- `sort` (method) `(*a, **k)`


### MSceneMessage

Bases: `MSceneMessage`.

Constants: 56 names.

**Members**

- `addCallback` (method)
- `addCheckCallback` (method)
- `addCheckFileCallback` (method)
- `addCheckReferenceCallback` (method)
- `addConnectionFailedCallback` (method)
- `addNamespaceRenamedCallback` (method)
- `addReferenceCallback` (method)
- `addStringArrayCallback` (method)
- `currentCallbackId` (method)
- `nodeCallbacks` (method)
- `removeCallback` (method)
- `removeCallbacks` (method)


### MScriptUtil

Bases: `object`.

**Members**

- `asBool` (method) `(self)`
- `asBoolPtr` (method) `(self)`
- `asCharPtr` (method) `(self)`
- `asDouble` (method) `(self)`
- `asDouble2Ptr` (method) `(self)`
- `asDouble3Ptr` (method) `(self)`
- `asDouble4Ptr` (method) `(self)`
- `asDoublePtr` (method) `(self)`
- `asFloat` (method) `(self)`
- `asFloat2Ptr` (method) `(self)`
- `asFloat3Ptr` (method) `(self)`
- `asFloat4Ptr` (method) `(self)`
- `asFloatPtr` (method) `(self)`
- `asInt` (method) `(self)`
- `asInt2Ptr` (method) `(self)`
- `asInt3Ptr` (method) `(self)`
- `asInt4Ptr` (method) `(self)`
- `asIntPtr` (method) `(self)`
- `asShort` (method) `(self)`
- `asShort2Ptr` (method) `(self)`
- `asShort3Ptr` (method) `(self)`
- `asShort4Ptr` (method) `(self)`
- `asShortPtr` (method) `(self)`
- `asUcharPtr` (method) `(self)`
- `asUint` (method) `(self)`
- `asUintPtr` (method) `(self)`
- `asUshortPtr` (method) `(self)`
- `createDoubleArrayFromList` (method)
- `createFloatArrayFromList` (method)
- `createFloatMatrixFromList` (method)
- `createFromDouble` (method) `(self, *values)`
- `createFromInt` (method) `(self, *values)`
- `createFromList` (method) `(self, values, count)`
- `createIntArrayFromList` (method)
- `createMatrixFromList` (method)
- `getBool` (method)
- `getBoolArrayItem` (method)
- `getChar` (method)
- `getCharArrayItem` (method)
- `getDouble` (method)
- `getDouble2ArrayItem` (method)
- `getDouble3ArrayItem` (method)
- `getDouble4ArrayItem` (method)
- `getDoubleArrayItem` (method)
- `getFloat` (method)
- `getFloat2ArrayItem` (method)
- `getFloat3ArrayItem` (method)
- `getFloat4ArrayItem` (method)
- `getFloatArrayItem` (method)
- `getInt` (method)
- `getInt2ArrayItem` (method)
- `getInt3ArrayItem` (method)
- `getInt4ArrayItem` (method)
- `getIntArrayItem` (method)
- `getShort` (method)
- `getShortArrayItem` (method)
- `getUchar` (method)
- `getUcharArrayItem` (method)
- `getUint` (method)
- `getUintArrayItem` (method)
- `getUshort` (method)
- `getUshortArrayItem` (method)
- `setBool` (method)
- `setBoolArray` (method)
- `setChar` (method)
- `setCharArray` (method)
- `setDouble` (method)
- `setDouble2ArrayItem` (method)
- `setDouble3ArrayItem` (method)
- `setDouble4ArrayItem` (method)
- `setDoubleArray` (method)
- `setFloat` (method)
- `setFloat2ArrayItem` (method)
- `setFloat3ArrayItem` (method)
- `setFloat4ArrayItem` (method)
- `setFloatArray` (method)
- `setInt` (method)
- `setInt2ArrayItem` (method)
- `setInt3ArrayItem` (method)
- `setInt4ArrayItem` (method)
- `setIntArray` (method)
- `setShort` (method)
- `setShortArray` (method)
- `setUchar` (method)
- `setUcharArray` (method)
- `setUint` (method)
- `setUintArray` (method)
- `setUshort` (method)
- `setUshortArray` (method)


### MSelectionList

Bases: `MSelectionList`.

Constants: 3 names.

**Members**

- `add` (method) `(self, item, *rest)`
- `clear` (method) `(*a, **k)`
- `copy` (method) `(*a, **k)`
- `getComponent` (method) `(*a, **k)`
- `getDagPath` (method) `(self, i, path=None, comp=None)`
- `getDependNode` (method) `(self, i, obj=None)`
- `getPlug` (method) `(self, i, plug=None)`
- `getSelectionStrings` (method) `(self, *args)`
- `hasItem` (method) `(*a, **k)`
- `hasItemPartly` (method) `(*a, **k)`
- `isEmpty` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `merge` (method) `(*a, **k)`
- `remove` (method) `(*a, **k)`
- `replace` (method) `(*a, **k)`
- `toggle` (method) `(*a, **k)`


### MSpace

Bases: `object`.

Constants: 7 names.


### MStatus

Bases: `object`.

Constants: 9 names.

**Members**

- `clear` (method) `(self)`
- `error` (method) `(self)`
- `errorString` (method) `(self)`
- `perror` (method) `(self, msg='')`
- `statusCode` (method) `(self)`


### MString

Bases: `str`.

**Members**

- `asChar` (method) `(self)`
- `asUTF8` (method) `(self)`
- `capitalize` (method) `(self, /)`
- `casefold` (method) `(self, /)`
- `center` (method) `(self, width, fillchar=' ', /)`
- `count` (method)
- `encode` (method) `(self, /, encoding='utf-8', errors='strict')`
- `endswith` (method)
- `expandtabs` (method) `(self, /, tabsize=8)`
- `find` (method)
- `format` (method) `(self, /, *args, **kwargs)`
- `format_map` (method) `(self, mapping, /)`
- `index` (method)
- `isEmpty` (method) `(self)`
- `isalnum` (method) `(self, /)`
- `isalpha` (method) `(self, /)`
- `isascii` (method) `(self, /)`
- `isdecimal` (method) `(self, /)`
- `isdigit` (method) `(self, /)`
- `isidentifier` (method) `(self, /)`
- `islower` (method) `(self, /)`
- `isnumeric` (method) `(self, /)`
- `isprintable` (method) `(self, /)`
- `isspace` (method) `(self, /)`
- `istitle` (method) `(self, /)`
- `isupper` (method) `(self, /)`
- `join` (method) `(self, iterable, /)`
- `length` (method) `(self)`
- `ljust` (method) `(self, width, fillchar=' ', /)`
- `lower` (method) `(self, /)`
- `lstrip` (method) `(self, chars=None, /)`
- `maketrans` (method)
- `numChars` (method) `(self)`
- `partition` (method) `(self, sep, /)`
- `removeprefix` (method) `(self, prefix, /)`
- `removesuffix` (method) `(self, suffix, /)`
- `replace` (method) `(self, old, new, /, count=-1)`
- `rfind` (method)
- `rindex` (method)
- `rjust` (method) `(self, width, fillchar=' ', /)`
- `rpartition` (method) `(self, sep, /)`
- `rsplit` (method) `(self, /, sep=None, maxsplit=-1)`
- `rstrip` (method) `(self, chars=None, /)`
- `split` (method) `(self, /, sep=None, maxsplit=-1)`
- `splitlines` (method) `(self, /, keepends=False)`
- `startswith` (method)
- `strip` (method) `(self, chars=None, /)`
- `swapcase` (method) `(self, /)`
- `title` (method) `(self, /)`
- `translate` (method) `(self, table, /)`
- `upper` (method) `(self, /)`
- `zfill` (method) `(self, width, /)`


### MStringArray

Bases: `MStringArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MSyntax

Bases: `MSyntax`.

Constants: 17 names.

**Members**

- `addArg` (method) `(*a, **k)`
- `addFlag` (method) `(*a, **k)`
- `host_flags` (method) `(*a, **k)`
- `makeFlagMultiUse` (method) `(*a, **k)`
- `makeFlagQueryWithFullArgs` (method) `(*a, **k)`
- `maxObjects` (method) `(*a, **k)`
- `minObjects` (method) `(*a, **k)`
- `setMaxObjects` (method) `(*a, **k)`
- `setMinObjects` (method) `(*a, **k)`
- `setObjectType` (method) `(*a, **k)`
- `useSelectionAsDefault` (method) `(*a, **k)`


### MTime

Bases: `MTime`.

Constants: 51 names.

**Members**

- `asUnits` (method) `(*a, **k)`
- `setUIUnit` (method)
- `ticksPerSecond` (method)
- `uiUnit` (method)


### MTimeArray

Bases: `MTimeArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MTransformationMatrix

Bases: `MTransformationMatrix`.

Constants: 9 names.

**Members**

- `addScale` (method) `(self, ptr, space=1)`
- `addTranslation` (method) `(self, v, space=1)`
- `asMatrix` (method) `(*a, **k)`
- `asMatrixInverse` (method) `(*a, **k)`
- `asRotateMatrix` (method) `(*a, **k)`
- `asScaleMatrix` (method) `(*a, **k)`
- `eulerRotation` (method) `(self)`
- `getRotation` (method) `(self, out, order=None, space=None)`
- `getRotationQuaternion` (method) `(self, x, y, z, w, space=None)`
- `getScale` (method) `(self, ptr, space=1)`
- `getShear` (method) `(self, ptr, space=1)`
- `getTranslation` (method) `(self, space=1)`
- `isEquivalent` (method) `(*a, **k)`
- `reorderRotation` (method) `(*a, **k)`
- `rotateBy` (method) `(*a, **k)`
- `rotatePivot` (method) `(*a, **k)`
- `rotatePivotTranslation` (method) `(*a, **k)`
- `rotateTo` (method) `(self, rot, space=None)`
- `rotation` (method) `(self)`
- `rotationComponents` (method) `(*a, **k)`
- `rotationOrder` (method) `(*a, **k)`
- `rotationOrientation` (method) `(*a, **k)`
- `scale` (method) `(*a, **k)`
- `scaleBy` (method) `(*a, **k)`
- `scalePivot` (method) `(*a, **k)`
- `scalePivotTranslation` (method) `(*a, **k)`
- `setRotatePivot` (method) `(*a, **k)`
- `setRotatePivotTranslation` (method) `(*a, **k)`
- `setRotation` (method) `(self, rot, order=None, space=None)`
- `setRotationComponents` (method) `(*a, **k)`
- `setRotationOrientation` (method) `(*a, **k)`
- `setRotationQuaternion` (method) `(self, x, y, z, w, space=None)`
- `setScale` (method) `(self, ptr, space=1)`
- `setScalePivot` (method) `(*a, **k)`
- `setScalePivotTranslation` (method) `(*a, **k)`
- `setShear` (method) `(self, ptr, space=1)`
- `setTranslation` (method) `(*a, **k)`
- `shear` (method) `(*a, **k)`
- `shearBy` (method) `(*a, **k)`
- `translateBy` (method) `(*a, **k)`
- `translation` (method) `(*a, **k)`


### MTypeId

Bases: `MTypeId`.

**Members**

- `id` (method) `(*a, **k)`


### MUintArray

Bases: `MIntArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`


### MUuid

Bases: `MUuid`.

**Members**

- `asString` (method) `(*a, **k)`
- `copy` (method) `(*a, **k)`
- `generate` (method) `(*a, **k)`
- `valid` (method) `(*a, **k)`


### MVector

Bases: `MVector`.

Constants: 5 names.

**Members**

- `angle` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `isEquivalent` (method) `(*a, **k)`
- `isParallel` (method) `(*a, **k)`
- `length` (method) `(*a, **k)`
- `normal` (method) `(*a, **k)`
- `normalize` (method) `(*a, **k)`
- `rotateBy` (method) `(*a, **k)`
- `rotateTo` (method) `(*a, **k)`
- `transformAsNormal` (method) `(*a, **k)`


### MVectorArray

Bases: `MVectorArray`.

**Members**

- `append` (method) `(*a, **k)`
- `clear` (method) `(self)`
- `copy` (method) `(*a, **k)`
- `count` (method) `(self, value, /)`
- `extend` (method) `(*a, **k)`
- `get` (method) `(self, ptr)`
- `index` (method) `(self, value, start=0, stop=9223372036854775807, /)`
- `insert` (method) `(*a, **k)`
- `isSizeIncrement` (method) `(*a, **k)`
- `length` (method) `(self)`
- `pop` (method) `(self, index=-1, /)`
- `remove` (method) `(*a, **k)`
- `reverse` (method) `(self, /)`
- `set` (method) `(self, v, i)`
- `setLength` (method) `(*a, **k)`
- `setSizeIncrement` (method) `(self, value)`
- `sizeIncrement` (method) `(*a, **k)`
- `sort` (method) `(self, /, *, key=None, reverse=False)`

