# maya.OpenMayaMPx, Python API 1.0

This is the current Aztek Python 1.0 surface, listed for the implementation agents. It is not a transcription of a Python 1.0 page; the reference names and signatures for these classes are the C++ class pages transcribed in the other files in this folder. Python 1.0 names take precedence over the 2.0 names shown by the underlying classes.

Classes in this module: 16.

Constant members (`k...`) are counted but not listed; see `data/enum_gaps.tsv` for the constant audit.

## Classes

### MFnPlugin

Bases: `MFnPlugin`.

**Members**

- `apiVersion` (method) `(self)`
- `deregisterCommand` (method) `(self, name)`
- `deregisterConstraintCommand` (method) `(self, name)`
- `deregisterData` (method) `(self, typeId)`
- `deregisterFileTranslator` (method) `(self, name)`
- `deregisterIkSolver` (method) `(self, *args)`
- `deregisterNode` (method) `(self, typeId)`
- `hasObj` (method) `(self, obj)`
- `loadPath` (method) `(self)`
- `matrixTypeIdFromXformId` (method) `(self, xformId)`
- `name` (method) `(self)`
- `object` (method) `(self)`
- `registerCommand` (method) `(self, name, creator, syntaxCreator=None)`
- `registerConstraintCommand` (method) `(self, name, creator)`
- `registerData` (method) `(self, name, typeId, creator, dataType=0)`
- `registerFileTranslator` (method) `(self, name, pixmapName, creator, optionsScriptName=None, defaultOptionsString=None, requiresFullMel=False, dataStorage=None)`
- `registerIkSolver` (method) `(self, *args)`
- `registerNode` (method) `(self, name, typeId, creator, initialize=None, nodeType=0, classification=None)`
- `registerShape` (method) `(self, name, typeId, creator, initialize, uiCreator=None, classification=None)`
- `registerTransform` (method) `(self, name, typeId, creator, initialize, matrixCreator=None, matrixId=None, classification=None)`
- `registerUI` (method) `(self, *a)`
- `setName` (method) `(self, name, allowRename=True)`
- `setObject` (method) `(self, obj)`
- `type` (method) `(self)`
- `vendor` (method) `(self)`
- `version` (method) `(self)`


### MIkHandleGroup

Bases: `object`.

**Members**

- `checkEffectorAtGoal` (method) `(self)`
- `dofCount` (method) `(self)`
- `handle` (method) `(self, index)`
- `handleCount` (method) `(self)`
- `isUnderControl` (method) `(self)`
- `priority` (method) `(self)`
- `setPriority` (method) `(self, p)`
- `setSolverID` (method) `(self, i)`
- `solve` (method) `(self)`
- `solverID` (method) `(self)`
- `solverPriority` (method) `(self)`


### MPxCommand

Bases: `object`.

Constants: 4 names.

**Members**

- `appendToResult` (method)
- `clearResult` (method)
- `currentResult` (method)
- `currentResultType` (method)
- `displayError` (method)
- `displayInfo` (method)
- `displayWarning` (method)
- `doIt` (method) `(self, args)`
- `hasSyntax` (method) `(self)`
- `isCurrentResultArray` (method)
- `isUndoable` (method) `(self)`
- `redoIt` (method) `(self)`
- `setResult` (method)
- `syntax` (method) `(self)`
- `undoIt` (method) `(self)`


### MPxConstraint

Bases: `MPxNode`.

Constants: 34 names.

**Members**

- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `constraintRotateOrderAttribute` (method) `(self)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `getOutputAttributes` (method) `(self, attributeArray)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `name` (method) `(self)`
- `passiveOutputAttribute` (method) `(self)`
- `postConstructor` (method) `(self)`
- `targetAttribute` (method) `(self)`
- `thisMObject` (method) `(self)`
- `type` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `weightAttribute` (method) `(self)`


### MPxConstraintCommand

Bases: `MPxCommand`.

Constants: 7 names.

**Members**

- `appendSyntax` (method) `(self)`
- `appendToResult` (method)
- `clearResult` (method)
- `connectObjectAndConstraint` (method) `(self, modifier)`
- `connectObjectAttribute` (method) `(self, child, objectAttr, constraintAttr, objectIsArray=False)`
- `connectTarget` (method) `(self, target, index)`
- `connectTargetAttribute` (method) `(self, target, index, targetAttr, constraintAttr, targetIsArray=False)`
- `connectTargetMatrixAttribute` (method) `(self, target, index, constraintAttr)`
- `constraintInstancedAttribute` (method) `(self)`
- `constraintOutputAttribute` (method) `(self)`
- `constraintTargetAttribute` (method) `(self)`
- `constraintTargetInstancedAttribute` (method) `(self)`
- `constraintTargetWeightAttribute` (method) `(self)`
- `constraintTypeId` (method) `(self)`
- `createdConstraint` (method) `(self, constraint)`
- `currentResult` (method)
- `currentResultType` (method)
- `displayError` (method)
- `displayInfo` (method)
- `displayWarning` (method)
- `doIt` (method) `(self, args)`
- `handleNewTargets` (method) `(self, obj)`
- `hasSyntax` (method) `(self)`
- `hasUpVectorFlags` (method) `(self)`
- `hasVectorFlags` (method) `(self)`
- `isCurrentResultArray` (method)
- `isUndoable` (method) `(self)`
- `objectAttribute` (method) `(self)`
- `parseArgs` (method) `(self, argList)`
- `redoIt` (method) `(self)`
- `setRestPosition` (method) `(self, obj)`
- `setResult` (method)
- `supportsOffset` (method) `(self)`
- `syntax` (method) `(self)`
- `targetType` (method) `(self)`
- `undoIt` (method) `(self)`


### MPxData

Bases: `object`.

Constants: 3 names.

**Members**

- `copy` (method) `(self, src)`
- `name` (method) `(self)`
- `readASCII` (method) `(self, argList, endOfTheLastParsedElement)`
- `readBinary` (method) `(self, data, length)`
- `typeId` (method) `(self)`
- `writeASCII` (method) `(self)`
- `writeBinary` (method) `(self)`


### MPxDeformerNode

Bases: `_MPxGeometryFilter`.

Constants: 31 names.

**Members**

- `accessoryAttribute` (method) `(self)`
- `accessoryNodeSetup` (method) `(self, cmd)`
- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `deform` (method) `(self, dataBlock, geomIter, localToWorldMatrix, multiIndex)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `getDeformationDetails` (method) `(self)`
- `getInputGeom` (method) `(self, dataBlock, multiIndex)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `setDeformationDetails` (method) `(self, details)`
- `setUseExistingConnectionWhenSetEditing` (method) `(self, state)`
- `thisMObject` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `weightValue` (method) `(self, dataBlock, multiIndex, index)`


### MPxFileTranslator

Bases: `object`.

Constants: 15 names.

**Members**

- `allowMultipleFileOptimization` (method) `(self)`
- `canBeOpened` (method) `(self)`
- `defaultExtension` (method) `(self)`
- `fileAccessMode` (method)
- `filter` (method) `(self)`
- `haveNamespaceSupport` (method) `(self)`
- `haveReadMethod` (method) `(self)`
- `haveReferenceMethod` (method) `(self)`
- `haveWriteMethod` (method) `(self)`
- `identifyFile` (method) `(self, file, buffer, size)`
- `reader` (method) `(self, file, optionsString, mode)`
- `writer` (method) `(self, file, optionsString, mode)`


### MPxGeometryFilter

Bases: `MPxNode`.

Constants: 31 names.

**Members**

- `accessoryAttribute` (method) `(self)`
- `accessoryNodeSetup` (method) `(self, cmd)`
- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `deform` (method) `(self, dataBlock, geomIter, localToWorldMatrix, multiIndex)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `getDeformationDetails` (method) `(self)`
- `getInputGeom` (method) `(self, dataBlock, multiIndex)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `setDeformationDetails` (method) `(self, details)`
- `setUseExistingConnectionWhenSetEditing` (method) `(self, state)`
- `thisMObject` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `weightValue` (method) `(self, dataBlock, multiIndex, index)`


### MPxIkSolverNode

Bases: `MPxNode`.

Constants: 31 names.

**Members**

- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `doSolve` (method) `(self)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `funcValueTolerance` (method) `(self)`
- `groupHandlesByTopology` (method) `(self)`
- `handleGroup` (method) `(self)`
- `hasJointLimitSupport` (method) `(self)`
- `hasUniqueSolution` (method) `(self)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `isPositionOnly` (method) `(self)`
- `isSingleChainOnly` (method) `(self)`
- `maxIterations` (method) `(self)`
- `name` (method) `(self)`
- `positionOnly` (method) `(self)`
- `postConstructor` (method) `(self)`
- `postSolve` (method) `(self, status=None)`
- `preSolve` (method) `(self)`
- `rotatePlane` (method) `(self)`
- `setFuncValueTolerance` (method) `(self, t)`
- `setHandleGroup` (method) `(self, group)`
- `setMaxIterations` (method) `(self, n)`
- `setPositionOnly` (method) `(self, on)`
- `setRotatePlane` (method) `(self, on)`
- `setSingleChainOnly` (method) `(self, on)`
- `setSupportJointLimits` (method) `(self, on)`
- `setUniqueSolution` (method) `(self, on)`
- `singleChainOnly` (method) `(self)`
- `solverTypeName` (method) `(self)`
- `supportJointLimits` (method) `(self)`
- `thisMObject` (method) `(self)`
- `toSolverSpace` (method) `(self)`
- `toWorldSpace` (method) `(self)`
- `type` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `uniqueSolution` (method) `(self)`


### MPxLocatorNode

Bases: `MPxNode`.

Constants: 31 names.

**Members**

- `addAttribute` (method)
- `attributeAffects` (method)
- `boundingBox` (method) `(self)`
- `closestPoint` (method) `(self, cursorRay, rayDirection)`
- `color` (method) `(self, status)`
- `colorRGB` (method) `(self, status)`
- `compute` (method) `(self, plug, dataBlock)`
- `draw` (method) `(self, view, path, style, status)`
- `drawLast` (method) `(self)`
- `excludeAsLocator` (method) `(self)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `isBounded` (method) `(self)`
- `isTransparent` (method) `(self)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `thisMObject` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `useClosestPointForSelection` (method) `(self)`


### MPxNode

Bases: `object`.

Constants: 31 names.

**Members**

- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `thisMObject` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`


### MPxSurfaceShape

Bases: `MPxNode`.

Constants: 50 names.

**Members**

- `acceptsGeometryIterator` (method) `(self, *a)`
- `activeComponents` (method) `(self)`
- `addAttribute` (method)
- `attributeAffects` (method)
- `boundingBox` (method) `(self)`
- `cachedShapeAttr` (method) `(self)`
- `canMakeLive` (method) `(self)`
- `childChanged` (method) `(self, state=0)`
- `componentToPlugs` (method) `(self, component, selectionList)`
- `compute` (method) `(self, plug, dataBlock)`
- `createFullRenderGroup` (method) `(self)`
- `createFullVertexGroup` (method) `(self)`
- `deleteComponents` (method) `(self, componentList, undoInfo)`
- `excludeAsPluginShape` (method) `(self)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `geometryData` (method) `(self)`
- `getWorldMatrix` (method) `(self, block, instance)`
- `hasActiveComponents` (method) `(self)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `isBounded` (method) `(self)`
- `localShapeInAttr` (method) `(self)`
- `localShapeOutAttr` (method) `(self)`
- `match` (method) `(self, mask, componentList)`
- `matchComponent` (method) `(self, item, spec, list)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `thisMObject` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `undeleteComponents` (method) `(self, componentList, undoInfo)`
- `worldShapeOutAttr` (method) `(self)`


### MPxSurfaceShapeUI

Bases: `_Thin`.

**Members**

- `surfaceShape` (method) `(self)`


### MPxTransform

Bases: `MPxNode`.

Constants: 31 names.

**Members**

- `addAttribute` (method)
- `attributeAffects` (method)
- `compute` (method) `(self, plug, dataBlock)`
- `computeLocalTransformation` (method) `(self, xform, block)`
- `createTransformationMatrix` (method) `(self)`
- `existWithoutInConnections` (method) `(self, *a)`
- `existWithoutOutConnections` (method) `(self, *a)`
- `inheritAttributesFrom` (method)
- `isAbstractClass` (method) `(self)`
- `isBounded` (method) `(self)`
- `name` (method) `(self)`
- `postConstructor` (method) `(self)`
- `resetTransformation` (method) `(self, matrix=None)`
- `thisMObject` (method) `(self)`
- `transformationMatrix` (method) `(self)`
- `transformationMatrixPtr` (method) `(self)`
- `treatAsTransform` (method) `(self)`
- `type` (method) `(self)`
- `typeId` (method) `(self)`
- `typeName` (method) `(self)`
- `updateMatrixAttrs` (method) `(self, *args)`


### MPxTransformationMatrix

Bases: `MTransformationMatrix`.

Constants: 9 names.

**Members**

- `addScale` (method) `(self, ptr, space=1)`
- `addTranslation` (method) `(self, v, space=1)`
- `asMatrix` (method) `(*a, **k)`
- `asMatrixInverse` (method) `(*a, **k)`
- `asRotateMatrix` (method) `(*a, **k)`
- `asScaleMatrix` (method) `(*a, **k)`
- `copyValues` (method) `(self, other)`
- `creator` (method)
- `decomposeMatrix` (method) `(self, m)`
- `eulerRotation` (method) `(self)`
- `getRotation` (method) `(self, out, order=None, space=None)`
- `getRotationQuaternion` (method) `(self, x, y, z, w, space=None)`
- `getScale` (method) `(self, ptr, space=1)`
- `getShear` (method) `(self, ptr, space=1)`
- `getTranslation` (method) `(self, space=1)`
- `isEquivalent` (method) `(*a, **k)`
- `reorderRotation` (method) `(*a, **k)`
- `rotateBy` (method) `(*a, **k)`
- `rotateOrientation` (method) `(self, space=1)`
- `rotatePivot` (method) `(*a, **k)`
- `rotatePivotTranslation` (method) `(*a, **k)`
- `rotateTo` (method) `(self, rot, space=1)`
- `rotation` (method) `(self)`
- `rotationComponents` (method) `(*a, **k)`
- `rotationOrder` (method) `(*a, **k)`
- `rotationOrientation` (method) `(*a, **k)`
- `scale` (method) `(*a, **k)`
- `scaleBy` (method) `(*a, **k)`
- `scalePivot` (method) `(*a, **k)`
- `scalePivotTranslation` (method) `(*a, **k)`
- `scaleTo` (method) `(self, v, space=1)`
- `setRotateOrientation` (method) `(self, q, space=1, balance=True)`
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
- `shearTo` (method) `(self, v, space=1)`
- `translateBy` (method) `(self, vector, space=1)`
- `translateTo` (method) `(self, vector, space=1)`
- `translation` (method) `(*a, **k)`
- `typeId` (method) `(self)`

