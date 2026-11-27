# maya.OpenMayaAnim, Python API 1.0

This is the current Aztek Python 1.0 surface, listed for the implementation agents. It is not a transcription of a Python 1.0 page; the reference names and signatures for these classes are the C++ class pages transcribed in the other files in this folder. Python 1.0 names take precedence over the 2.0 names shown by the underlying classes.

Classes in this module: 7.

Constant members (`k...`) are counted but not listed; see `data/enum_gaps.tsv` for the constant audit.

## Classes

### MAnimControl

Bases: `MAnimControl`.

Constants: 3 names.

**Members**

- `animationEndTime` (method)
- `animationStartTime` (method)
- `currentTime` (method)
- `isPlaying` (method)
- `isScrubbing` (method)
- `maxTime` (method)
- `minTime` (method)
- `playbackSpeed` (method)
- `setAnimationEndTime` (method)
- `setAnimationStartEndTime` (method)
- `setAnimationStartTime` (method)
- `setCurrentTime` (method)
- `setMaxTime` (method)
- `setMinMaxTime` (method)
- `setMinTime` (method)
- `setPlaybackSpeed` (method)


### MFnAnimCurve

Bases: `MFnAnimCurve`.

Constants: 35 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `addKey` (method) `(*a, **k)`
- `addKeys` (method) `(*a, **k)`
- `animCurveType` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `evaluate` (method) `(*a, **k)`
- `find` (method) `(self, t, ptr=None)`
- `findAlias` (method) `(*a, **k)`
- `findClosest` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getConnections` (method) `(*a, **k)`
- `hasAttribute` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `inTangentType` (method) `(*a, **k)`
- `input` (method) `(*a, **k)`
- `isAnimCurveType` (method) `(*a, **k)`
- `isDefaultNode` (method) `(*a, **k)`
- `isFromReferencedFile` (method) `(*a, **k)`
- `isLocked` (method) `(*a, **k)`
- `isNamespace` (method) `(*a, **k)`
- `isNumKeys` (method) `(*a, **k)`
- `isPluginName` (method) `(*a, **k)`
- `isShared` (method) `(*a, **k)`
- `isTimeInput` (method) `(self)`
- `isTypeId` (method) `(*a, **k)`
- `isTypeName` (method) `(*a, **k)`
- `isUnitlessInput` (method) `(self)`
- `isWeighted` (method) `(*a, **k)`
- `name` (method) `(*a, **k)`
- `namespace` (method) `(*a, **k)`
- `numKeyframes` (method) `(*a, **k)`
- `numKeys` (method) `(*a, **k)`
- `object` (method) `(*a, **k)`
- `outTangentType` (method) `(*a, **k)`
- `pluginName` (method) `(*a, **k)`
- `remove` (method) `(*a, **k)`
- `removeAttribute` (method) `(*a, **k)`
- `setInput` (method) `(*a, **k)`
- `setName` (method) `(*a, **k)`
- `setObject` (method) `(*a, **k)`
- `setTime` (method) `(*a, **k)`
- `setValue` (method) `(*a, **k)`
- `time` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`
- `value` (method) `(*a, **k)`


### MFnIkEffector

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


### MFnIkHandle

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
- `getEffector` (method) `(self, out=None)`
- `getPath` (method) `(*a, **k)`
- `getRotation` (method) `(self, out, space=1, *rest)`
- `getRotationQuaternion` (method) `(self, x, y, z, w, space=1)`
- `getScale` (method) `(self, ptr)`
- `getShear` (method) `(self, ptr)`
- `getStartJoint` (method) `(self, out=None)`
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
- `poWeight` (method) `(self)`
- `priority` (method) `(self)`
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
- `solver` (method) `(self)`
- `transformation` (method) `(*a, **k)`
- `transformationMatrix` (method) `(*a, **k)`
- `translateBy` (method) `(*a, **k)`
- `translation` (method) `(*a, **k)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`
- `weight` (method) `(self)`


### MFnIkJoint

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
- `getOrientation` (method) `(self, out=None)`
- `getPath` (method) `(*a, **k)`
- `getPreferedAngle` (method) `(self, out=None)`
- `getRotation` (method) `(self, out, space=1, *rest)`
- `getRotationQuaternion` (method) `(self, x, y, z, w, space=1)`
- `getScale` (method) `(self, ptr)`
- `getScaleOrientation` (method) `(self, out=None)`
- `getSegmentScale` (method) `(self, ptr=None)`
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
- `setOrientation` (method) `(self, rot)`
- `setPreferedAngle` (method) `(self, rot)`
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


### MFnSkinCluster

Bases: `MFnSkinCluster`.

Constants: 6 names.

**Members**

- `absoluteName` (method) `(*a, **k)`
- `addAttribute` (method) `(*a, **k)`
- `attribute` (method) `(*a, **k)`
- `attributeCount` (method) `(*a, **k)`
- `create` (method) `(*a, **k)`
- `deformerSet` (method) `(*a, **k)`
- `findAlias` (method) `(*a, **k)`
- `findPlug` (method) `(*a, **k)`
- `getAffectedAttributes` (method) `(*a, **k)`
- `getBlendWeights` (method) `(self, path, comp, arr=None)`
- `getConnections` (method) `(*a, **k)`
- `getInputGeometry` (method) `(self, arr=None)`
- `getOutputGeometry` (method) `(self, arr=None)`
- `getPointsAffectedByInfluence` (method) `(self, path, sel=None, weights=None)`
- `getWeights` (method) `(self, path, comp, *args)`
- `hasAttribute` (method) `(*a, **k)`
- `hasObj` (method) `(*a, **k)`
- `indexForInfluenceObject` (method) `(*a, **k)`
- `influenceObjects` (method) `(self, arr=None)`
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
- `setObject` (method) `(*a, **k)`
- `setWeights` (method) `(self, path, comp, influences, values, normalize=True, old=None)`
- `type` (method) `(*a, **k)`
- `typeId` (method) `(*a, **k)`
- `typeName` (method) `(*a, **k)`
- `userNode` (method) `(*a, **k)`


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

