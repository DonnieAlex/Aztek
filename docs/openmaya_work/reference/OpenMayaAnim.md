# OpenMayaAnim

Class names, bases and signatures of the module, checked against the reference's 2022 C++ API reference (https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_anim.html). Descriptions are not kept here; follow the source link on a class for them.

Classes in this module: 53.

## Classes

### MAnimControl

Module `OpenMayaAnim`. Source: [class_m_anim_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_control.html).

**Public Types**

- `enum PlaybackMode { kPlaybackOnce, kPlaybackLoop, kPlaybackOscillate }`
- `enum PlaybackViewMode { kPlaybackViewAll, kPlaybackViewActive }`

**Public Member Functions**

- `MAnimControl ()`
- `virtual ~MAnimControl ()`

**Static Public Member Functions**

- `static bool isValid ()`
- `static MAnimControl::PlaybackMode playbackMode ()`
- `static MStatus setPlaybackMode (PlaybackMode newMode)`
- `static MAnimControl::PlaybackViewMode viewMode ()`
- `static MStatus setViewMode (PlaybackViewMode newMode)`
- `static double playbackBy ()`
- `static MStatus setPlaybackBy (const double &newTime)`
- `static MTime minTime ()`
- `static MTime maxTime ()`
- `static MStatus setMinTime (MTime newMinTime)`
- `static MStatus setMaxTime (MTime newMaxTime)`
- `static MStatus setMinMaxTime (MTime min, MTime max)`
- `static MTime animationStartTime ()`
- `static MTime animationEndTime ()`
- `static MStatus setAnimationStartTime (MTime newStartTime)`
- `static MStatus setAnimationEndTime (MTime newEndTime)`
- `static MStatus setAnimationStartEndTime (MTime newStartTime, MTime newEndTime)`
- `static MTime currentTime ()`
- `static MStatus setCurrentTime (const MTime &newTime)`
- `static double playbackSpeed ()`
- `static MStatus setPlaybackSpeed (double newSpeed)`
- `static MStatus playForward ()`
- `static MStatus playBackward ()`
- `static bool isPlaying ()`
- `static bool isScrubbing ()`
- `static MStatus stop ()`
- `static bool autoKeyMode ()`
- `static MStatus setAutoKeyMode (bool mode)`
- `static MFnAnimCurve::TangentType globalInTangentType (MStatus *ReturnStatus=NULL)`
- `static MStatus setGlobalInTangentType (const MFnAnimCurve::TangentType &tangentType)`
- `static MFnAnimCurve::TangentType globalOutTangentType (MStatus *ReturnStatus=NULL)`
- `static MStatus setGlobalOutTangentType (const MFnAnimCurve::TangentType &tangentType)`
- `static bool weightedTangents (MStatus *ReturnStatus=NULL)`
- `static MStatus setWeightedTangents (bool weightState)`

### MAnimCurveChange

Module `OpenMayaAnim`. Source: [class_m_anim_curve_change.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_change.html).

**Public Member Functions**

- `MAnimCurveChange (MStatus *ReturnStatus=NULL)`
- `~MAnimCurveChange ()`
- `MStatus undoIt ()`
- `MStatus redoIt ()`
- `bool isInteractive () const`
- `void setInteractive (bool value)`

**Static Public Member Functions**

- `static const char * className ()`

### MAnimCurveClipboard

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard.html).

**Public Member Functions**

- `MAnimCurveClipboard ()`
- `~MAnimCurveClipboard ()`
- `MStatus set (const MAnimCurveClipboard &cb)`
- `MStatus set (const MAnimCurveClipboardItemArray &clipboardItemArray)`
- `MStatus set (const MAnimCurveClipboardItemArray &clipboardItemArray, const MTime &startTime, const MTime &endTime, const float &startUnitlessInput, const float &endUnitlessInput, bool strictValidation=true)`
- `MStatus clear ()`
- `bool isEmpty (MStatus *ReturnStatus=NULL) const`
- `const MAnimCurveClipboardItemArray clipboardItems (MStatus *ReturnStatus=NULL) const`
- `MTime startTime (MStatus *ReturnStatus=NULL) const`
- `MTime endTime (MStatus *ReturnStatus=NULL) const`
- `float startUnitlessInput (MStatus *ReturnStatus=NULL) const`
- `float endUnitlessInput (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static MAnimCurveClipboard & theAPIClipboard ()`

### MAnimCurveClipboardItem

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard_item.html).

**Public Member Functions**

- `MAnimCurveClipboardItem ()`
- `MAnimCurveClipboardItem (const MAnimCurveClipboardItem &r)`
- `~MAnimCurveClipboardItem ()`
- `const MObject animCurve (MStatus *ReturnStatus=NULL) const`
- `MStatus getAddressingInfo (unsigned int &rowCount, unsigned int &childCount, unsigned int &attributeCount) const`
- `const MString & fullAttributeName (MStatus *ReturnStatus=NULL) const`
- `const MString & leafAttributeName (MStatus *ReturnStatus=NULL) const`
- `const MString & nodeName (MStatus *ReturnStatus=NULL) const`
- `MFnAnimCurve::AnimCurveType animCurveType (MStatus *ReturnStatus=NULL) const`
- `MStatus setAnimCurve (const MObject &curve)`
- `MStatus setAddressingInfo (unsigned int rowCount, unsigned int childCount, unsigned int attributeCount)`
- `MStatus setNameInfo (const MString &nodeName, const MString &fullName, const MString &leafName)`
- `MAnimCurveClipboardItem & operator= (const MAnimCurveClipboardItem &from)`
- `bool operator== (const MAnimCurveClipboardItem &rhs) const`

**Static Public Member Functions**

- `static const char * className ()`

### MAnimCurveClipboardItemArray

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard_item_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard_item_array.html).

**Public Member Functions**

- `MAnimCurveClipboardItemArray ()`
- `MAnimCurveClipboardItemArray (const MAnimCurveClipboardItemArray &other)`
- `MAnimCurveClipboardItemArray (unsigned int initialSize, const MAnimCurveClipboardItem &initialValue)`
- `~MAnimCurveClipboardItemArray ()`
- `const MAnimCurveClipboardItem & operator[] (unsigned int index) const`
- `MStatus set (const MAnimCurveClipboardItem &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MAnimCurveClipboardItem &element, unsigned int index)`
- `MStatus append (const MAnimCurveClipboardItem &element)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `bool isValid (unsigned int &failedIndex) const`
- `MAnimCurveClipboardItemArray (const MAnimCurveClipboardItem src[], unsigned int count)`
- `MStatus get (MAnimCurveClipboardItem array[]) const`
- `MAnimCurveClipboardItem & operator[] (unsigned int index)`
- `MAnimCurveClipboardItemArray & operator= (const MAnimCurveClipboardItemArray &other)`
- `MStatus copy (const MAnimCurveClipboardItemArray &source)`

**Static Public Member Functions**

- `static const char * className ()`

### MAnimMessage

Module `OpenMayaAnim`. Inherits `MMessage`. Source: [class_m_anim_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_message.html).

**Static Public Member Functions**

- `static MCallbackId addAnimCurveEditedCallback (MMessage::MObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAnimKeyframeEditedCallback (MMessage::MObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeAnimKeyframeEditedCallback (MObject &animNode, MMessage::MNodeObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAnimKeyframeEditCheckCallback (MMessage::MCheckPlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addPreBakeResultsCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addPostBakeResultsCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDisableImplicitControlCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAnimKeyframeEditedCallback (MObject &animNode, MMessage::MNodeObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static void flushAnimKeyframeEditedCallbacks ()`
- `static const char * className ()`

### MAnimUtil

Module `OpenMayaAnim`. Source: [class_m_anim_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_util.html).

**Static Public Member Functions**

- `static bool isAnimated (const MObject &node, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool isAnimated (const MDagPath &path, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool isAnimated (const MPlug &plug, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool isAnimated (const MSelectionList &selectionList, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool findAnimatedPlugs (const MObject &node, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool findAnimatedPlugs (const MDagPath &path, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool findAnimatedPlugs (const MSelectionList &selectionList, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`
- `static bool findAnimation (const MPlug &plug, MObjectArray &animation, MStatus *ReturnStatus=NULL)`
- `static bool findSetDrivenKeyAnimation (const MPlug &plug, MObjectArray &animation, MPlugArray &drivers, MStatus *ReturnStatus=NULL)`
- `static bool findConstraint (const MPlug &plug, MObject &constraint, MObjectArray &targets, MStatus *ReturnStatus=NULL)`
- `static bool findAnimatablePlugs (const MSelectionList &selectionList, MPlugArray &animatablePlugs)`
- `static const char * className ()`

### MBufferIdentifier

Module `OpenMayaAnim`. Source: [class_m_buffer_identifier.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_buffer_identifier.html).

**Public Member Functions**

- `MBufferIdentifier (MPxNode *source, unsigned int layerId=0)`

### MFalloffContext

Module `OpenMayaAnim`. Source: [class_m_falloff_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_falloff_context.html).

**Public Member Functions**

- `MFloatArray getValues (const MBufferIdentifier &bufferId) const`
- `void setValue (const MFloatArray &array, const MBufferIdentifier &bufferId)`
- `bool isTopologyDirty () const`
- `bool isSync (int syncId, const MBufferIdentifier &bufferId) const`
- `void sync (int syncId, const MBufferIdentifier &bufferId)`
- `MObject getGeometry (bool originalGeometry=false)`
- `MIndexMapper getIndexMapper () const`
- `~MFalloffContext ()`

### MFalloffFunction

Module `OpenMayaAnim`. Source: [class_m_falloff_function.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_falloff_function.html).

**Classes**

- `struct ReturnValue`

### MFnAnimCurve

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_anim_curve.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_anim_curve.html).

**Public Types**

- `enum AnimCurveType { kAnimCurveTA, kAnimCurveTL, kAnimCurveTT, kAnimCurveTU, kAnimCurveUA, kAnimCurveUL, kAnimCurveUT, kAnimCurveUU, kAnimCurveUnknown }`
- `enum TangentType { kTangentGlobal, kTangentFixed, kTangentLinear, kTangentFlat, kTangentSmooth, kTangentStep, kTangentSlow, kTangentFast, kTangentClamped, kTangentPlateau, kTangentStepNext, kTangentAuto , kTangentAutoMix, kTangentAutoEase, kTangentAutoCustom }`
- `enum InfinityType { kConstant = 0, kLinear = 1, kCycle = 3, kCycleRelative = 4, kOscillate = 5 }`
- `typedef double TangentValue`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAnimCurve ()`
- `MFnAnimCurve ()`
- `MFnAnimCurve (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAnimCurve (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &node, const MObject &attribute, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &node, const MObject &attribute, AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MObject create (const MPlug &plug, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MObject create (const MPlug &plug, AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MObject create (AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `AnimCurveType animCurveType (MStatus *ReturnStatus=NULL) const`
- `AnimCurveType timedAnimCurveTypeForPlug (MPlug &plug, MStatus *ReturnStatus=NULL) const`
- `AnimCurveType unitlessAnimCurveTypeForPlug (MPlug &plug, MStatus *ReturnStatus=NULL) const`
- `double evaluate (const MTime &atTime, MStatus *ReturnStatus=NULL) const`
- `MStatus evaluate (const MTime &atTime, double &value) const`
- `MStatus evaluate (const MTime &atTime, MTime &timeValue) const`
- `MStatus evaluate (const double &atUnitlessInput, double &value) const`
- `MStatus evaluate (const double &atUnitlessInput, MTime &timeValue) const`
- `bool isStatic (MStatus *ReturnStatus=NULL) const`
- `unsigned int numKeyframes (MStatus *ReturnStatus=NULL) const`
- `unsigned int numKeys (MStatus *ReturnStatus=NULL) const`
- `MStatus remove (unsigned int index, MAnimCurveChange *change=NULL)`
- `MStatus addKeyframe (const MTime &time, double value, MAnimCurveChange *change=NULL)`
- `MStatus addKeyframe (const MTime &time, double value, TangentType tangentInType, TangentType tangentOutType, MAnimCurveChange *change=NULL)`
- `unsigned int addKey (const MTime &time, double value, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`
- `unsigned int addKey (const MTime &timeInput, const MTime &timeValue, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`
- `unsigned int addKey (double unitlessInput, double value, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`
- `unsigned int addKey (double unitlessInput, const MTime &time, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`
- `MStatus addKeys (MTimeArray *timeArray, MDoubleArray *valueArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, bool keepExistingKeys=false, MAnimCurveChange *change=NULL)`
- `MStatus addKeysWithTangents (MTimeArray *timeArray, MDoubleArray *valueArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MIntArray *tangentInTypeArray=NULL, MIntArray *tangentOutTypeArray=NULL, MDoubleArray *tangentInXArray=NULL, MDoubleArray *tangentInYArray=NULL, MDoubleArray *tangentOutXArray=NULL, MDoubleArray *tangentOutYArray=NULL, MIntArray *tangentsLockedArray=NULL, MIntArray *weightsLockedArray=NULL, bool convertUnits=true, bool keepExistingKeys=false, MAnimCurveChange *change=NULL)`
- `unsigned int insertKey (const MTime &time, bool breakdown=false, MAnimCurveChange *change=nullptr, MStatus *ReturnStatus=nullptr)`
- `bool find (const MTime &time, unsigned int &index, MStatus *ReturnStatus=NULL) const`
- `bool find (double unitlessInput, unsigned int &index, MStatus *ReturnStatus=NULL) const`
- `unsigned int findClosest (const MTime &time, MStatus *ReturnStatus=NULL) const`
- `unsigned int findClosest (double unitlessInput, MStatus *ReturnStatus=NULL) const`
- `MTime time (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `double value (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `double unitlessInput (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setValue (unsigned int index, double value, MAnimCurveChange *change=NULL)`
- `MStatus setTime (unsigned int index, const MTime &time, MAnimCurveChange *change=NULL)`
- `MStatus setUnitlessInput (unsigned int index, double unitlessInput, MAnimCurveChange *change=NULL)`
- `bool isTimeInput (MStatus *ReturnStatus=NULL) const`
- `bool isUnitlessInput (MStatus *ReturnStatus=NULL) const`
- `TangentType inTangentType (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `TangentType outTangentType (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setInTangentType (unsigned int index, TangentType tangentType, MAnimCurveChange *change=NULL)`
- `MStatus setOutTangentType (unsigned int index, TangentType tangentType, MAnimCurveChange *change=NULL)`
- `MStatus setTangentTypes (MIntArray indexArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL)`
- `MStatus getTangent (unsigned int index, TangentValue &x, TangentValue &y, bool inTangent) const`
- `MStatus getTangent (unsigned int index, MAngle &angle, double &weight, bool inTangent) const`
- `MStatus setTangent (unsigned int index, TangentValue x, TangentValue y, bool inTangent, MAnimCurveChange *change=NULL, bool convertUnits=true)`
- `MStatus setTangent (unsigned int index, const MAngle &angle, double weight, bool inTangent, MAnimCurveChange *change=NULL, bool convertUnits=true)`
- `MStatus setAngle (unsigned int index, const MAngle &angle, bool inTangent, MAnimCurveChange *change=NULL)`
- `MStatus setWeight (unsigned int index, double weight, bool inTangent, MAnimCurveChange *change=NULL)`
- `bool weightsLocked (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `bool tangentsLocked (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setWeightsLocked (unsigned int index, bool locked, MAnimCurveChange *change=NULL)`
- `MStatus setTangentsLocked (unsigned int index, bool locked, MAnimCurveChange *change=NULL)`
- `InfinityType preInfinityType (MStatus *ReturnStatus=NULL) const`
- `InfinityType postInfinityType (MStatus *ReturnStatus=NULL) const`
- `MStatus setPreInfinityType (InfinityType infinityType, MAnimCurveChange *change=NULL)`
- `MStatus setPostInfinityType (InfinityType infinityType, MAnimCurveChange *change=NULL)`
- `bool isWeighted (MStatus *ReturnStatus=NULL) const`
- `MStatus setIsWeighted (bool isWeighted, MAnimCurveChange *change=NULL)`
- `bool isBreakdown (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setIsBreakdown (unsigned int index, bool isBreakdown, MAnimCurveChange *change=NULL)`
- `MFnAnimCurve (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnBlendShapeDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_blend_shape_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_blend_shape_deformer.html).

**Public Types**

- `enum Origin { kLocalOrigin, kWorldOrigin }`
- `enum HistoryLocation { kFrontOfChain, kNormal, kPost, kOther }`
- `enum TargetType { kObject, kTangent }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnBlendShapeDeformer ()`
- `MFnBlendShapeDeformer ()`
- `MFnBlendShapeDeformer (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MObject baseObject, Origin originSpace=kLocalOrigin, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObjectArray &baseObjects, Origin originSpace=kLocalOrigin, HistoryLocation=kNormal, MStatus *ReturnStatus=NULL)`
- `MStatus addBaseObject (MObject &object)`
- `MStatus getBaseObjects (MObjectArray &objects) const`
- `MStatus addTarget (const MObject &baseObject, int weightIndex, const MObject &newTarget, double fullWeight, TargetType targetType=kObject)`
- `MStatus addTarget (const MObject &baseObject, int weightIndex, double fullWeight, TargetType targetType=kObject)`
- `MStatus removeTarget (const MObject &baseObject, int weightIndex, const MObject &target, double fullWeight)`
- `MStatus getTargets (MObject baseObject, int weightIndex, MObjectArray &targetObjects) const`
- `unsigned int numWeights (MStatus *ReturnStatus=NULL) const`
- `MStatus weightIndexList (MIntArray &indexList) const`
- `MStatus targetItemIndexList (unsigned int weightIndex, MObject baseObject, MIntArray &targetItemIndices) const`
- `float weight (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setWeight (unsigned int index, float weight)`
- `float envelope (MStatus *ReturnStatus=NULL) const`
- `MStatus setEnvelope (float envelope)`
- `Origin origin (MStatus *ReturnStatus=NULL) const`
- `MStatus setOrigin (Origin space)`
- `HistoryLocation historyLocation (MStatus *ReturnStatus=NULL) const`
- `MFnBlendShapeDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnCharacter

Module `OpenMayaAnim`. Inherits `MFnSet`. Source: [class_m_fn_character.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_character.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCharacter ()`
- `MFnCharacter ()`
- `MFnCharacter (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus attachSourceToCharacter (MObject &sourceClip, MDGModifier &dgMod)`
- `MStatus attachInstanceToCharacter (MObject &instanceClip, MDGModifier &dgMod)`
- `MStatus addCurveToClip (MObject &curve, MObject &sourceClip, MPlug &characterPlug, MDGModifier &dgMod)`
- `MObject createBlend (MObject &instancedClip1, MObject &instancedClip2, MObject &blendAnimCurve, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`
- `bool blendExists (MObject &instancedClip1, MObject &instancedClip2, MObject &blendResult)`
- `bool removeBlend (MObject &instancedClip1, MObject &instancedClip2, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`
- `bool getCharacterThatOwnsPlug (MPlug &plug, MObject &result) const`
- `MStatus getMemberPlugs (MPlugArray &result) const`
- `MStatus getSubCharacters (MSelectionList &result) const`
- `MObject getClipScheduler (MStatus *ReturnStatus=NULL)`
- `int getScheduledClipCount (MStatus *ReturnStatus=NULL)`
- `MObject getScheduledClip (int index, MStatus *ReturnStatus=NULL)`
- `int getSourceClipCount (MStatus *ReturnStatus=NULL)`
- `MObject getSourceClip (int index, MStatus *ReturnStatus=NULL)`
- `int getBlendCount (MStatus *ReturnStatus=NULL)`
- `MObject getBlend (int index, MStatus *ReturnStatus=NULL)`
- `MStatus getBlendClips (int index, MObject &clip1, MObject &clip2)`
- `MFnCharacter (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnClip

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_clip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_clip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnClip ()`
- `MFnClip ()`
- `MFnClip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject createSourceClip (const MTime &sourceStart, const MTime &sourceDuration, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`
- `MObject createInstancedClip (MObject &sourceClip, const MTime &start, MDGModifier &dgMod, MStatus *ReturnStatus=NULL, bool absolute=1, double postCycle=0.0, double weight=1.0, double scale=1.0, double preCycle=0.0)`
- `bool isInstancedClip (MStatus *ReturnStatus=NULL)`
- `bool isPose (MStatus *ReturnStatus=NULL)`
- `MObject sourceClip (MStatus *ReturnStatus=NULL)`
- `double getPreCycle (MStatus *ReturnStatus=NULL)`
- `double getPostCycle (MStatus *ReturnStatus=NULL)`
- `double getCycle (MStatus *ReturnStatus=NULL)`
- `double getWeight (MStatus *ReturnStatus=NULL)`
- `double getScale (MStatus *ReturnStatus=NULL)`
- `bool getAbsolute (MStatus *ReturnStatus=NULL)`
- `bool getEnabled (MStatus *ReturnStatus=NULL)`
- `MTime getStartFrame (MStatus *ReturnStatus=NULL)`
- `MTime getSourceStart (MStatus *ReturnStatus=NULL)`
- `MTime getSourceDuration (MStatus *ReturnStatus=NULL)`
- `MStatus getMemberAnimCurves (MObjectArray &curves, MPlugArray &associatedAttrs)`
- `MStatus getAbsoluteChannelSettings (MIntArray &absoluteChannels)`
- `int getTrack (MStatus *ReturnStatus=NULL)`
- `MStatus setPoseClip (bool state, MDGModifier *mod=NULL)`
- `MStatus setPreCycle (double cycle, MDGModifier *mod=NULL)`
- `MStatus setPostCycle (double cycle, MDGModifier *mod=NULL)`
- `MStatus setCycle (double cycle, MDGModifier *mod=NULL)`
- `MStatus setWeight (double wt, MDGModifier *mod=NULL)`
- `MStatus setScale (double scale, MDGModifier *mod=NULL)`
- `MStatus setAbsolute (bool abs, MDGModifier *mod=NULL)`
- `MStatus setEnabled (bool val, MDGModifier *mod=NULL)`
- `MStatus setStartFrame (const MTime &start, MDGModifier *mod=NULL)`
- `MStatus setSourceData (const MTime &start, const MTime &duration, MDGModifier *mod=NULL)`
- `MStatus setTrack (int index, MDGModifier *mod=NULL)`
- `MStatus setAbsoluteChannelSettings (const MIntArray &absoluteChannels, MDGModifier *mod=NULL)`
- `MFnClip (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnGeometryFilter

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_geometry_filter.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnGeometryFilter ()`
- `MFnGeometryFilter ()`
- `MFnGeometryFilter (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getInputGeometry (MObjectArray &objects) const`
- `MStatus getOutputGeometry (MObjectArray &objects) const`
- `MObject inputShapeAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MObject outputShapeAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `unsigned int indexForOutputShape (const MObject &shape, MStatus *ReturnStatus=NULL) const`
- `MStatus getPathAtIndex (unsigned int index, MDagPath &path) const`
- `unsigned int indexForGroupId (unsigned int groupId, MStatus *ReturnStatus=NULL) const`
- `unsigned int groupIdAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `unsigned int numOutputConnections (MStatus *ReturnStatus=NULL) const`
- `unsigned int indexForOutputConnection (unsigned int connectionIndex, MStatus *ReturnStatus=NULL) const`
- `MObject deformerSet (MStatus *ReturnStatus=NULL) const`
- `float envelope (MStatus *ReturnStatus=NULL) const`
- `MStatus setEnvelope (float envelope)`
- `MObject getComponentAtIndex (unsigned int index, MStatus *ReturnStatus=NULL)`
- `MFnGeometryFilter (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnHikEffector

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_hik_effector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_hik_effector.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnHikEffector ()`
- `MFnHikEffector ()`
- `MFnHikEffector (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnHikEffector (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MVector getPivotOffset (MStatus *ReturnStatus=NULL) const`
- `MStatus setPivotOffset (const MVector &vector)`
- `MColor getEffColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setEffColor (const MColor &color)`
- `MStatus getAuxiliaryEffectors (MObjectArray &effs) const`
- `MFnHikEffector (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnIkEffector

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_effector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_effector.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnIkEffector ()`
- `MFnIkEffector ()`
- `MFnIkEffector (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnIkEffector (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MFnIkEffector (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnIkHandle

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_handle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_handle.html).

**Public Types**

- `enum Stickiness { kStickyOff, kStickyOn, kSuperSticky }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnIkHandle ()`
- `MFnIkHandle ()`
- `MFnIkHandle (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnIkHandle (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MDagPath &startJoint, MDagPath &effector, MStatus *ReturnStatus=NULL)`
- `MStatus getStartJoint (MDagPath &jointPath)`
- `MStatus setStartJoint (const MDagPath &jointPath)`
- `MStatus getEffector (MDagPath &effectorPath)`
- `MStatus setEffector (const MDagPath &effectorPath)`
- `MStatus setStartJointAndEffector (const MDagPath &jointPath, const MDagPath &effectorPath)`
- `unsigned int priority (MStatus *ReturnStatus=NULL)`
- `MStatus setPriority (unsigned int priority)`
- `Stickiness stickiness (MStatus *ReturnStatus=NULL)`
- `MStatus setStickiness (Stickiness stickiness)`
- `double weight (MStatus *ReturnStatus=NULL)`
- `MStatus setWeight (double weight)`
- `double poWeight (MStatus *ReturnStatus=NULL)`
- `MStatus setPOWeight (double weight)`
- `MObject solver (MStatus *ReturnStatus=NULL)`
- `MStatus setSolver (const MObject &solver)`
- `MStatus setSolver (const MString &solverName)`
- `MFnIkHandle (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnIkJoint

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_joint.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_joint.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnIkJoint ()`
- `MFnIkJoint ()`
- `MFnIkJoint (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnIkJoint (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus getOrientation (MQuaternion &quaternion) const`
- `MStatus setOrientation (const MQuaternion &quaternion)`
- `MStatus getOrientation (MEulerRotation &rotation) const`
- `MStatus setOrientation (const MEulerRotation &rotation)`
- `MStatus getOrientation (double rotation[3], MTransformationMatrix::RotationOrder &order) const`
- `MStatus setOrientation (const double rotation[3], MTransformationMatrix::RotationOrder order)`
- `MStatus getScaleOrientation (MQuaternion &quaternion) const`
- `MStatus setScaleOrientation (const MQuaternion &quaternion)`
- `MStatus getScaleOrientation (double rotation[3], MTransformationMatrix::RotationOrder &order) const`
- `MStatus setScaleOrientation (const double rotation[3], MTransformationMatrix::RotationOrder order)`
- `MStatus getSegmentScale (double scale[3]) const`
- `MStatus setSegmentScale (const double scale[3])`
- `MStatus getStiffness (double stiffness[3]) const`
- `MStatus setStiffness (const double stiffness[3])`
- `MStatus getPreferredAngle (double rotation[3]) const`
- `MStatus setPreferredAngle (const double rotation[3])`
- `MStatus getDegreesOfFreedom (bool &freeInX, bool &freeInY, bool &freeInZ) const`
- `MStatus setDegreesOfFreedom (bool freeInX, bool freeInY, bool freeInZ)`
- `double minRotateDampXRange (MStatus *ReturnStatus=NULL) const`
- `double minRotateDampYRange (MStatus *ReturnStatus=NULL) const`
- `double minRotateDampZRange (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampXRange (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampYRange (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampZRange (MStatus *ReturnStatus=NULL) const`
- `MStatus setMinRotateDampXRange (double angle)`
- `MStatus setMinRotateDampYRange (double angle)`
- `MStatus setMinRotateDampZRange (double angle)`
- `MStatus setMaxRotateDampXRange (double angle)`
- `MStatus setMaxRotateDampYRange (double angle)`
- `MStatus setMaxRotateDampZRange (double angle)`
- `double minRotateDampXStrength (MStatus *ReturnStatus=NULL) const`
- `double minRotateDampYStrength (MStatus *ReturnStatus=NULL) const`
- `double minRotateDampZStrength (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampXStrength (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampYStrength (MStatus *ReturnStatus=NULL) const`
- `double maxRotateDampZStrength (MStatus *ReturnStatus=NULL) const`
- `MStatus setMinRotateDampXStrength (double angle)`
- `MStatus setMinRotateDampYStrength (double angle)`
- `MStatus setMinRotateDampZStrength (double angle)`
- `MStatus setMaxRotateDampXStrength (double angle)`
- `MStatus setMaxRotateDampYStrength (double angle)`
- `MStatus setMaxRotateDampZStrength (double angle)`
- `MString hikJointName (MStatus *ReturnStatus=NULL) const`
- `MFnIkJoint (const MObject &object, MStatus *ret=NULL)`
- `MStatus getHikJointName (MString &name) const`
- `MStatus getPreferedAngle (double rotation[3]) const`
- `MStatus setPreferedAngle (const double rotation[3])`

**Static Public Member Functions**

- `static const char * className ()`

### MFnIkSolver

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_ik_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_solver.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnIkSolver ()`
- `MFnIkSolver ()`
- `MFnIkSolver (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int maxIterations (MStatus *ReturnStatus=NULL)`
- `MStatus setMaxIterations (unsigned int maxIters)`
- `double tolerance (MStatus *ReturnStatus=NULL)`
- `MStatus setTolerance (double tolerance)`
- `MFnIkSolver (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDelta

Module `OpenMayaAnim`. Inherits `MFnBase`. Source: [class_m_fn_keyframe_delta.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDelta ()`
- `MFnKeyframeDelta ()`
- `MFnKeyframeDelta (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject paramCurve (MStatus *ReturnStatus=NULL) const`
- `virtual unsigned int keyIndex (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDelta (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaAddRemove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_add_remove.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_add_remove.html).

**Public Types**

- `enum DeltaType { kAdded, kRemoved, kReplaced }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaAddRemove ()`
- `MFnKeyframeDeltaAddRemove ()`
- `MFnKeyframeDeltaAddRemove (MObject &object, MStatus *ReturnStatus=NULL)`
- `DeltaType deltaType (MStatus *ReturnStatus=NULL) const`
- `double value (MStatus *ReturnStatus=NULL) const`
- `MTime time (MStatus *ReturnStatus=NULL) const`
- `unsigned int keyIndex (MStatus *ReturnStatus=NULL) const override`
- `double replacedValue (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaAddRemove (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaBlockAddRemove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_block_add_remove.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_block_add_remove.html).

**Public Types**

- `enum DeltaType { kAdded, kRemoved }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaBlockAddRemove ()`
- `MFnKeyframeDeltaBlockAddRemove ()`
- `MFnKeyframeDeltaBlockAddRemove (MObject &object, MStatus *ReturnStatus=NULL)`
- `DeltaType deltaType (MStatus *ReturnStatus=NULL) const`
- `MTime startTime (MStatus *ReturnStatus=NULL) const`
- `MTime endTime (MStatus *ReturnStatus=NULL) const`
- `unsigned int numKeys (MStatus *ReturnStatus=NULL) const`
- `void getValues (MDoubleArray &values, MStatus *ReturnStatus=NULL) const`
- `void getTimes (MTimeArray &times, MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaBlockAddRemove (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaBreakdown

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_breakdown.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_breakdown.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaBreakdown ()`
- `MFnKeyframeDeltaBreakdown ()`
- `MFnKeyframeDeltaBreakdown (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool wasBreakdown (MStatus *ReturnStatus=NULL) const`
- `bool isBreakdown (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaBreakdown (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaInfType

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_inf_type.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_inf_type.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaInfType ()`
- `MFnKeyframeDeltaInfType ()`
- `MFnKeyframeDeltaInfType (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAnimCurve::InfinityType previousInfinityType (MStatus *ReturnStatus=NULL) const`
- `MFnAnimCurve::InfinityType currentInfinityType (MStatus *ReturnStatus=NULL) const`
- `bool isPreInfinity (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaInfType (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaMove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_move.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_move.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaMove ()`
- `MFnKeyframeDeltaMove ()`
- `MFnKeyframeDeltaMove (MObject &object, MStatus *ReturnStatus=NULL)`
- `MTime previousTime (MStatus *ReturnStatus=NULL) const`
- `MTime currentTime (MStatus *ReturnStatus=NULL) const`
- `double previousValue (MStatus *ReturnStatus=NULL) const`
- `double currentValue (MStatus *ReturnStatus=NULL) const`
- `unsigned int previousIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int keyIndex (MStatus *ReturnStatus=NULL) const override`
- `MFnKeyframeDeltaMove (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaScale

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_scale.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_scale.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaScale ()`
- `MFnKeyframeDeltaScale ()`
- `MFnKeyframeDeltaScale (MObject &object, MStatus *ReturnStatus=NULL)`
- `MTime startTime (MStatus *ReturnStatus=NULL) const`
- `MTime endTime (MStatus *ReturnStatus=NULL) const`
- `MTime currentStartTime (MStatus *ReturnStatus=NULL) const`
- `MTime currentEndTime (MStatus *ReturnStatus=NULL) const`
- `MTime pivotTime (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaScale (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaTangent

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_tangent.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_tangent.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaTangent ()`
- `MFnKeyframeDeltaTangent ()`
- `MFnKeyframeDeltaTangent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAnimCurve::TangentType previousTangentType (MStatus *ReturnStatus=NULL) const`
- `MFnAnimCurve::TangentType currentTangentType (MStatus *ReturnStatus=NULL) const`
- `MStatus getPreviousPosition (MFnAnimCurve::TangentValue &x, MFnAnimCurve::TangentValue &y) const`
- `MStatus getCurrentPosition (MFnAnimCurve::TangentValue &x, MFnAnimCurve::TangentValue &y) const`
- `bool isInTangent (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaTangent (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnKeyframeDeltaWeighted

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_weighted.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_weighted.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnKeyframeDeltaWeighted ()`
- `MFnKeyframeDeltaWeighted ()`
- `MFnKeyframeDeltaWeighted (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool wasWeighted (MStatus *ReturnStatus=NULL) const`
- `MFnKeyframeDeltaWeighted (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLattice

Module `OpenMayaAnim`. Inherits `MFnDagNode`. Source: [class_m_fn_lattice.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lattice.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLattice ()`
- `MFnLattice ()`
- `MFnLattice (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnLattice (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (unsigned int xDiv, unsigned int yDiv, unsigned int zDiv, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus reset (double sSize=1.0, double tSize=1.0, double uSize=1.0)`
- `MStatus getDivisions (unsigned int &s, unsigned int &t, unsigned int &u)`
- `MStatus setDivisions (unsigned int s, unsigned int t, unsigned int u)`
- `MPoint & point (unsigned int s, unsigned int t, unsigned int u, MStatus *ReturnStatus=NULL)`
- `MFnLattice (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLatticeDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_lattice_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lattice_deformer.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLatticeDeformer ()`
- `MFnLatticeDeformer ()`
- `MFnLatticeDeformer (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (unsigned int xDiv, unsigned int yDiv, unsigned int zDiv, MStatus *ReturnStatus=NULL)`
- `MStatus addGeometry (const MObject &object)`
- `MStatus removeGeometry (const MObject &object)`
- `MStatus getAffectedGeometry (MObjectArray &objects)`
- `MStatus getDivisions (unsigned int &x, unsigned int &y, unsigned int &z)`
- `MStatus setDivisions (unsigned int x, unsigned int y, unsigned int z)`
- `MStatus resetLattice (bool centerLattice=false)`
- `MObject deformLattice (MStatus *ReturnStatus=NULL)`
- `MObject baseLattice (MStatus *ReturnStatus=NULL)`
- `MFnLatticeDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMotionPath

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_motion_path.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_motion_path.html).

**Public Types**

- `enum Axis { kXaxis, kYaxis, kZaxis }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMotionPath ()`
- `MFnMotionPath ()`
- `MFnMotionPath (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MDagPath &pathObject, const MDagPath &objectToAnimate, MTime &timeStart, MTime &timeEnd, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MStatus setPathObject (const MDagPath &pathObject, MDGModifier *modifier=NULL)`
- `MDagPath pathObject (MStatus *ReturnStatus=NULL)`
- `MStatus addAnimatedObject (const MDagPath &objectToAnimate, MDGModifier *modifier=NULL)`
- `MStatus getAnimatedObjects (MDagPathArray &array)`
- `MStatus setFollow (bool on, MDGModifier *modifier=NULL)`
- `bool follow (MStatus *ReturnStatus=NULL) const`
- `MStatus setFollowAxis (Axis axis)`
- `Axis followAxis (MStatus *ReturnStatus=NULL)`
- `MStatus setUpAxis (Axis axis)`
- `Axis upAxis (MStatus *ReturnStatus=NULL)`
- `MStatus setBank (bool bank)`
- `bool bank (MStatus *ReturnStatus=NULL) const`
- `MStatus setBankScale (double bankScale)`
- `double bankScale (MStatus *ReturnStatus=NULL)`
- `MStatus setBankThreshold (double bankThreshold)`
- `double bankThreshold (MStatus *ReturnStatus=NULL)`
- `MStatus setUseNormal (bool use)`
- `bool useNormal (MStatus *ReturnStatus=NULL)`
- `MStatus setInverseNormal (bool invert)`
- `bool inverseNormal (MStatus *ReturnStatus=NULL)`
- `MStatus setUStart (double start)`
- `MStatus setUEnd (double end)`
- `double uStart (MStatus *ReturnStatus=NULL)`
- `double uEnd (MStatus *ReturnStatus=NULL)`
- `MStatus setUTimeStart (MTime &start)`
- `MStatus setUTimeEnd (MTime &end)`
- `MTime uTimeStart (MStatus *ReturnStatus=NULL)`
- `MTime uTimeEnd (MStatus *ReturnStatus=NULL)`
- `unsigned int numPositionMarkers (MStatus *ReturnStatus=NULL)`
- `MObject getPositionMarker (unsigned int, MStatus *ReturnStatus=NULL)`
- `unsigned int numOrientationMarkers (MStatus *ReturnStatus=NULL)`
- `MObject getOrientationMarker (unsigned int, MStatus *ReturnStatus=NULL)`
- `MFnMotionPath (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSkinCluster

Module `OpenMayaAnim`. Inherits `MFnGeometryFilter`. Source: [class_m_fn_skin_cluster.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_skin_cluster.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSkinCluster ()`
- `MFnSkinCluster ()`
- `MFnSkinCluster (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getWeights (const MDagPath &path, const MObject &components, unsigned int influenceIndex, MDoubleArray &weights) const`
- `MStatus getWeights (const MDagPath &path, const MObject &components, MDoubleArray &weights, unsigned int &influenceCount) const`
- `MStatus getWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MDoubleArray &weights) const`
- `MStatus setWeights (const MDagPath &path, const MObject &components, unsigned int jointIndex, double value, bool normalize=true, MDoubleArray *oldValues=NULL)`
- `MStatus setWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MDoubleArray &values, bool normalize=true, MDoubleArray *oldValues=NULL)`
- `MStatus getPointsAffectedByInfluence (const MDagPath &path, MSelectionList &result, MDoubleArray &weights)`
- `unsigned int indexForInfluenceObject (const MDagPath &path, MStatus *ReturnStatus=NULL) const`
- `unsigned int influenceObjects (MDagPathArray &paths, MStatus *ReturnStatus=NULL) const`
- `MStatus getWeights (const MDagPath &path, const MObject &components, unsigned int influenceIndex, MFloatArray &weights) const`
- `MStatus getWeights (const MDagPath &path, const MObject &components, MFloatArray &weights, unsigned int &influenceCount) const`
- `MStatus setWeights (const MDagPath &path, const MObject &components, unsigned int jointIndex, float value, bool normalize=true, MFloatArray *oldValues=NULL)`
- `MStatus setWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MFloatArray &values, bool normalize=true, MFloatArray *oldValues=NULL)`
- `MStatus getPointsAffectedByInfluence (const MDagPath &path, MSelectionList &result, MFloatArray &weights)`
- `MStatus getBlendWeights (const MDagPath &path, const MObject &components, MDoubleArray &weights) const`
- `MStatus setBlendWeights (const MDagPath &path, const MObject &components, const MDoubleArray &weights)`
- `MFnSkinCluster (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnWeightGeometryFilter

Module `OpenMayaAnim`. Inherits `MFnGeometryFilter`. Source: [class_m_fn_weight_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_weight_geometry_filter.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnWeightGeometryFilter ()`
- `MFnWeightGeometryFilter ()`
- `MFnWeightGeometryFilter (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getWeights (unsigned int index, const MObject &components, MFloatArray &weights) const`
- `MStatus getWeights (const MDagPath &path, const MObject &components, MFloatArray &weights) const`
- `MStatus setWeight (const MDagPath &path, unsigned int index, const MObject &components, float weight, MFloatArray *oldValues=NULL)`
- `MStatus setWeight (const MDagPath &path, const MObject &components, float weight, MFloatArray *oldValues=NULL)`
- `MStatus setWeight (const MDagPath &path, unsigned int index, const MObject &components, MFloatArray &values)`
- `MStatus setWeight (const MDagPath &path, const MObject &components, MFloatArray &values)`
- `MString weightPlugStrings (const MSelectionList &list, MStatus *ReturnStatus=NULL) const`
- `MStatus getWeightPlugStrings (const MSelectionList &list, MStringArray &plugStringArray) const`
- `MFnWeightGeometryFilter (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getWeightPlugStrings (const MSelectionList &list, MString &plugStrings) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnWireDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_wire_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_wire_deformer.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnWireDeformer ()`
- `MFnWireDeformer ()`
- `MFnWireDeformer (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MStatus addGeometry (const MObject &object)`
- `MStatus removeGeometry (const MObject &object)`
- `MStatus getAffectedGeometry (MObjectArray &objects)`
- `unsigned int numWires (MStatus *ReturnStatus=NULL) const`
- `MStatus addWire (const MObject &object)`
- `MObject wire (unsigned int wireIndex, MStatus *ReturnStatus=NULL)`
- `float wireDropOffDistance (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus setWireDropOffDistance (unsigned int wireIndex, float dropOff)`
- `float wireScale (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus setWireScale (unsigned int wireIndex, float scale)`
- `MObject holdingShape (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus setHoldingShape (unsigned int wireIndex, MObject holdingCurve)`
- `float envelope (MStatus *ReturnStatus=NULL) const`
- `MStatus setEnvelope (float envelope)`
- `float rotation (MStatus *ReturnStatus=NULL) const`
- `MStatus setRotation (float rotation)`
- `float localIntensity (MStatus *ReturnStatus=NULL) const`
- `MStatus setLocalIntensity (float localIntensity)`
- `float crossingEffect (MStatus *ReturnStatus=NULL) const`
- `MStatus setCrossingEffect (float crossingEffect)`
- `unsigned int numDropoffLocators (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus setDropoffLocator (unsigned int wireIndex, unsigned int locatorIndex, float param, float percentage)`
- `MStatus getDropoffLocator (unsigned int wireIndex, unsigned int locatorIndex, float &param, float &percentage)`
- `MFnWireDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MGPUDeformerBuffer

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_buffer.html).

**Public Member Functions**

- `MGPUDeformerBuffer (const MPlug &plug, const MUniqueString &name, bool readOnly, unsigned int elementSizeInBytes, unsigned int elementCount, unsigned int capacityInBytes, const MAutoCLMem &buffer, const MAutoCLEvent &event)`
- `MGPUDeformerBuffer ()`
- `~MGPUDeformerBuffer ()`
- `MGPUDeformerBuffer (const MGPUDeformerBuffer &other)`
- `MGPUDeformerBuffer (MGPUDeformerBuffer &&other)`
- `MGPUDeformerBuffer & operator= (const MGPUDeformerBuffer &other)`
- `MGPUDeformerBuffer & operator= (MGPUDeformerBuffer &&other)`
- `MOpenCLBuffer createMOpenCLBuffer ()`
- `bool isValid () const`
- `const MUniqueString name () const`
- `const MPlug plug () const`
- `bool isReadOnly () const`
- `unsigned int elementSizeInBytes () const`
- `unsigned int elementCount () const`
- `const MAutoCLMem buffer () const`
- `const MAutoCLEvent bufferReadyEvent () const`
- `void setBufferReadyEvent (const MAutoCLEvent &)`
- `void destroy ()`

### MGPUDeformerData

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_data.html).

**Public Member Functions**

- `virtual void setBuffer (const MGPUDeformerBuffer &)`
- `virtual MGPUDeformerBuffer getBuffer (const MUniqueString &name) const`
- `virtual MGPUDeformerBuffer getBuffer (const MUniqueString &name, const MPlug &plug) const`
- `MGPUDeformerData (void *impl)`
- `MGPUDeformerData (const MGPUDeformerData &other)`
- `virtual ~MGPUDeformerData ()`

### MGPUDeformerRegistrationInfo

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_registration_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_registration_info.html).

**Public Member Functions**

- `virtual MPxGPUDeformer * createGPUDeformer ()=0`
- `virtual bool validateNodeInGraph (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &plug, MStringArray *messages)=0`
- `virtual bool validateNodeValues (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &plug, MStringArray *messages)=0`
- `virtual const MObject outputMeshAttribute ()`
- `virtual const MObject inputMeshAttribute ()`
- `virtual void inputMeshAttributes (MObjectArray &inputAttributes)`
- `virtual bool matchAncestorLogicalIndices (const MPlug &original, MPlug &target)`
- `virtual const MObject passThroughInputMeshAttribute ()`

### MGPUDeformerRegistry

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_registry.html).

**Static Public Member Functions**

- `static MStatus registerGPUDeformerCreator (const MString &nodeClassName, const MString &registrantId, MGPUDeformerRegistrationInfo *registrationInfo)`
- `static MStatus deregisterGPUDeformerCreator (const MString &nodeClassName, const MString &registrantId)`
- `static MStatus addConditionalAttribute (const MString &nodeClassName, const MString &registrantId, const MObject &conditionalAttribute)`

### MGPUEventList

Module `OpenMayaAnim`. Source: [class_m_g_p_u_event_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_event_list.html).

**Public Member Functions**

- `MGPUEventList ()`
- `MGPUEventList (const MGPUEventList &)`
- `~MGPUEventList ()`
- `void add (const MAutoCLEvent &event)`
- `void add (MOpenCLBuffer &buf)`
- `const cl_event * array () const`
- `cl_uint size () const`

### MIkHandleGroup

Module `OpenMayaAnim`. Source: [class_m_ik_handle_group.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ik_handle_group.html).

**Public Member Functions**

- `MIkHandleGroup ()`
- `~MIkHandleGroup ()`
- `int priority (MStatus *ReturnStatus=NULL) const`
- `int solverID (MStatus *ReturnStatus=NULL) const`
- `int solverPriority (MStatus *ReturnStatus=NULL) const`
- `MStatus setPriority (int)`
- `MStatus setSolverID (int)`
- `bool checkEffectorAtGoal (MStatus *ReturnStatus=NULL)`
- `MStatus solve ()`
- `int dofCount (MStatus *ReturnStatus=NULL) const`
- `int handleCount (MStatus *ReturnStatus=NULL) const`
- `MObject handle (int ith, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MPxIkSolver`
- `class MPxIkSolverNode`

### MIkSystem

Module `OpenMayaAnim`. Source: [class_m_ik_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ik_system.html).

**Static Public Member Functions**

- `static MObject findSolver (MString name, MStatus *ReturnStatus=NULL)`
- `static MStatus getSolvers (MStringArray &names)`
- `static bool isGlobalSnap (MStatus *ReturnStatus=NULL)`
- `static MStatus setGlobalSnap (bool isSnap)`
- `static bool isGlobalSolve (MStatus *ReturnStatus=NULL)`
- `static MStatus setGlobalSolve (bool isSnap)`
- `static const char * className ()`

### MIndexMapper

Module `OpenMayaAnim`. Source: [class_m_index_mapper.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_index_mapper.html).

**Public Member Functions**

- `MIndexMapper ()`
- `~MIndexMapper ()`
- `MIndexMapper (const MIndexMapper &)`
- `Index fullCount () const`
- `Index affectCount () const`
- `Index affectToFull (Index aid) const`

### MItKeyframe

Module `OpenMayaAnim`. Source: [class_m_it_keyframe.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_keyframe.html).

**Public Types**

- `enum TangentType { kTangentGlobal = 0, kTangentFixed, kTangentLinear, kTangentFlat, kTangentSmooth, kTangentStep, kTangentSlow, kTangentFast, kTangentClamped, kTangentPlateau, kTangentStepNext, kTangentAuto, kTangentAutoMix, kTangentAutoEase, kTangentAutoCustom }`
- `typedef double TangentValue`

**Public Member Functions**

- `MItKeyframe (MObject &animCurveNode, MStatus *ReturnStatus=NULL)`
- `~MItKeyframe ()`
- `MStatus reset (MObject &animCurveNode)`
- `MStatus reset ()`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MTime time (MStatus *ReturnStatus=NULL)`
- `MStatus setTime (MTime time)`
- `double value (MStatus *ReturnStatus=NULL)`
- `MStatus setValue (double value)`
- `TangentType inTangentType (MStatus *ReturnStatus=NULL)`
- `TangentType outTangentType (MStatus *ReturnStatus=NULL)`
- `MStatus setInTangentType (TangentType tangentType)`
- `MStatus setOutTangentType (TangentType tangentType)`
- `MStatus getTangentOut (TangentValue &x, TangentValue &y)`
- `MStatus getTangentIn (TangentValue &x, TangentValue &y)`
- `bool tangentsLocked (MStatus *ReturnStatus=NULL) const`
- `MStatus setTangentsLocked (bool locked)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxBlendShape

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_blend_shape.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_blend_shape.html).

**Public Member Functions**

- `MPxBlendShape ()`
- `~MPxBlendShape () override`
- `MPxNode::Type type () const override`
- `virtual MStatus deformData (MDataBlock &block, MDataHandle geomData, unsigned int groupId, const MMatrix &mat, unsigned int multiIndex)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject weight`
- `static MObject inputTarget`
- `static MObject inputTargetGroup`
- `static MObject inputTargetItem`
- `static MObject inputGeomTarget`
- `static MObject inputPointsTarget`
- `static MObject inputComponentsTarget`
- `static MObject targetWeights`

### MPxConstraint

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_constraint.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_constraint.html).

**Public Types**

- `enum UpVectorType { kScene = 0, kObject, kObjectRotation, kVector, kLast }`

**Public Member Functions**

- `MPxConstraint ()`
- `~MPxConstraint () override`
- `virtual const MObject weightAttribute () const`
- `virtual const MObject targetAttribute () const`
- `virtual const MObject constraintRotateOrderAttribute () const`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject enableRestPosition`
- `static MObject lockOutput`

**Protected Member Functions**

- `virtual const MObject passiveOutputAttribute () const`
- `virtual void getOutputAttributes (MObjectArray &attributeArray)`
- `MStatus closestPoint (const MObject &dataGeometryObject, const MPoint &toThisPoint, MPoint &theClosestPoint) const`
- `MStatus closestNormal (const MObject &dataGeometryObject, const MPoint &toThisPoint, MVector &theNormal) const`
- `MStatus closestTangent (const MObject &dataGeometryObject, const MPoint &toThisPoint, MVector &theTangent) const`
- `MEulerRotation computeAim (const MMatrix &parentInverseMatrix, const MVector &targetVector, const MVector &aimVector, const MVector &upVector, const MVector &wupVector, MEulerRotation::RotationOrder order=MEulerRotation::kXYZ, const MQuaternion &jointOrient=MQuaternion::identity, MStatus *ResultStatus=NULL) const`
- `MVector worldUpVector (MPxConstraint::UpVectorType upType, const MVector &upVector, const MMatrix &upMatrix, const MPoint &constrPoint, MStatus *ResultStatus=NULL) const`
- `MPoint worldConstraintPoint (const MMatrix &parentInverseMatrix, const MVector &translate, const MVector &rotatePivot, const MVector &rotatePivotTranslate, MStatus *ResultStatus=NULL) const`

### MPxConstraintCommand

Module `OpenMayaAnim`. Inherits `MPxCommand`. Source: [class_m_px_constraint_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_constraint_command.html).

**Public Types**

- `enum TargetType { kTransform, kGeometryShape, kLast }`

**Public Member Functions**

- `MPxConstraintCommand ()`
- `~MPxConstraintCommand () override`
- `MStatus doIt (const MArgList &argList) override`
- `MStatus undoIt () override`
- `MStatus redoIt () override`
- `virtual MStatus appendSyntax ()`
- `MPxConstraint * constraintNode ()`

**Protected Member Functions**

- `virtual MStatus parseArgs (const MArgList &argList)`
- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`
- `virtual MStatus doEdit ()`
- `virtual MStatus doCreate ()`
- `virtual MStatus doQuery ()`
- `virtual void createdConstraint (MPxConstraint *constraint)`
- `virtual bool supportsOffset () const`
- `virtual bool hasVectorFlags () const`
- `virtual MTypeId constraintTypeId () const`
- `virtual MPxConstraintCommand::TargetType targetType () const`
- `virtual const MObject & aimVectorAttribute () const`
- `virtual const MObject & upVectorAttribute () const`
- `virtual const MObject & worldUpMatrixAttribute () const`
- `virtual const MObject & worldUpTypeAttribute () const`
- `virtual const MObject & worldUpVectorAttribute () const`
- `virtual const MObject & offsetAttribute () const`
- `virtual const MObject & constraintInstancedAttribute () const`
- `virtual const MObject & constraintOutputAttribute () const`
- `virtual const MObject & constraintRestAttribute () const`
- `virtual const MObject & constraintEnableRestAttribute () const`
- `virtual const MObject & constraintTargetInstancedAttribute () const`
- `virtual const MObject & constraintTargetAttribute () const`
- `virtual const MObject & constraintTargetWeightAttribute () const`
- `virtual const MObject & objectAttribute () const`
- `virtual void getObjectAttributesArray (MObjectArray &array)`
- `virtual void handleNewTargets (MObject &dagObject)`
- `virtual MStatus handleNewTargets (MDagPath &dagObject)`
- `virtual MStatus connectTarget (void *opaqueTarget, int index)`
- `virtual MStatus connectTarget (MDagPath &targetPath, int index)`
- `virtual MStatus connectObjectAndConstraint (MDGModifier &modifier)`
- `virtual MStatus setRestPosition (MDGModifier &modifier)`
- `MStatus connectGeometryAttribute (void *opaqueTarget, int index, MObject &constraintAttr)`
- `MStatus connectTargetAttribute (void *opaqueTarget, int index, MObject &constraintAttr)`
- `MStatus connectTargetAttribute (MDagPath &targetPath, int index, MObject &tarAttr, MObject &constraintAttr, bool instanced=false)`
- `MStatus connectObjectAttribute (const MObject &objectAttr, const MObject &constraintAttr, bool toConstraint=true, bool instanced=false)`
- `const MObject transformObject ()`

### MPxDeformerNode

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_deformer_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_deformer_node.html).

**Public Member Functions**

- `MPxDeformerNode ()`
- `~MPxDeformerNode () override`
- `MPxNode::Type type () const override`
- `float weightValue (MDataBlock &mblock, unsigned int multiIndex, unsigned int wtIndex)`
- `void setUseExistingConnectionWhenSetEditing (bool state)`
- `MStatus setDeformationDetails (unsigned int flags)`
- `unsigned int getDeformationDetails (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject weightList`
- `static MObject weights`

### MPxGPUDeformer

Module `OpenMayaAnim`. Source: [class_m_px_g_p_u_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_g_p_u_deformer.html).

**Public Member Functions**

- `MPxGPUDeformer ()`
- `virtual ~MPxGPUDeformer ()`
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, unsigned int numElements, const MAutoCLMem, const MAutoCLEvent, MAutoCLMem, MAutoCLEvent &)`
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, const MGPUDeformerData &inputData, MGPUDeformerData &outputData)`
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, const MPlugArray &inputPlugs, const MGPUDeformerData &inputData, MGPUDeformerData &outputData)`
- `virtual void terminate ()`
- `MGPUDeformerBuffer createOutputBuffer (const MGPUDeformerBuffer &inputBuffer)`
- `bool getIndexMapper (MIndexMapper &imap)`
- `MObject getFixedSetupData (const MString &name)`
- `bool isFalloffWeightsDirty () const`

**Static Public Member Functions**

- `static bool hasAttributeBeenModified (const MEvaluationNode &evaluationNode, const MObject &attribute)`
- `static bool hasPlugBeenModified (const MPlug &plug)`
- `static MUniqueString sPositionsName ()`
- `static MUniqueString sGeometryMatrixName ()`
- `static MUniqueString sInverseGeometryMatrixName ()`
- `static bool isFalloffWeightsDependantOnCurrentGeometry (const MEvaluationNode &evaluationNode, unsigned int multiIndex)`

### MPxGeometryFilter

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_geometry_filter.html).

**Public Types**

- `enum DeformationDetails { kDeformsUVs = (1<<1), kDeformsColors = (1<<2) }`

**Public Member Functions**

- `MPxGeometryFilter ()`
- `~MPxGeometryFilter () override`
- `MPxNode::Type type () const override`
- `virtual MStatus deform (MDataBlock &block, MItGeometry &iter, const MMatrix &mat, unsigned int multiIndex)`
- `virtual MObject & accessoryAttribute () const`
- `virtual MStatus accessoryNodeSetup (MDagModifier &cmd)`
- `void setUseExistingConnectionWhenSetEditing (bool state)`
- `MStatus setDeformationDetails (unsigned int flags)`
- `unsigned int getDeformationDetails (MStatus *ReturnStatus=NULL)`
- `virtual void setModifiedCallback (MSelectionList &list, bool listAdded)`
- `MFnGeometryData::SubsetState getGeometryIterator (MItGeometry &iter, MDataBlock &block, MDataHandle &dataHandle, unsigned int multiIndex, bool readOnly=true, MStatus *ReturnStatus=nullptr) const`
- `virtual MObject getFixedSetupData (const MString &name)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject input`
- `static MObject inputGeom`
- `static MObject groupId`
- `static MObject componentTagExpression`
- `static MObject outputGeom`
- `static MObject envelope`

### MPxIkSolver

Module `OpenMayaAnim`. Source: [class_m_px_ik_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_ik_solver.html).

**Public Member Functions**

- `virtual ~MPxIkSolver ()`
- `virtual MStatus preSolve ()`
- `virtual MStatus doSolve ()`
- `virtual MStatus postSolve (MStatus)`
- `virtual MString solverTypeName () const`
- `virtual bool isSingleChainOnly () const`
- `virtual bool isPositionOnly () const`
- `virtual bool hasJointLimitSupport () const`
- `virtual bool hasUniqueSolution () const`
- `virtual bool groupHandlesByTopology () const`
- `virtual MStatus setFuncValueTolerance (double tolerance)`
- `virtual MStatus setMaxIterations (int value)`
- `MIkHandleGroup * handleGroup () const`
- `virtual void setHandleGroup (MIkHandleGroup *)`
- `const MMatrix * toWorldSpace () const`
- `const MMatrix * toSolverSpace () const`
- `double funcValueTolerance () const`
- `int maxIterations () const`
- `virtual void snapHandle (MObject &handle)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus getJointAngles (MDoubleArray &) const`
- `MStatus setJointAngles (const MDoubleArray &)`
- `void setToRestAngles ()`
- `MPxIkSolver ()`

### MPxIkSolverNode

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_ik_solver_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_ik_solver_node.html).

**Public Member Functions**

- `~MPxIkSolverNode () override`
- `MPxNode::Type type () const override`
- `virtual MStatus preSolve ()`
- `virtual MStatus doSolve ()`
- `virtual MStatus postSolve (MStatus)`
- `virtual MString solverTypeName () const`
- `bool rotatePlane (MStatus *ReturnStatus=NULL) const`
- `MStatus setRotatePlane (bool rotatePlane)`
- `bool singleChainOnly (MStatus *ReturnStatus=NULL) const`
- `MStatus setSingleChainOnly (bool singleChainOnly)`
- `bool positionOnly (MStatus *ReturnStatus=NULL) const`
- `MStatus setPositionOnly (bool positionOnly)`
- `bool supportJointLimits (MStatus *ReturnStatus=NULL) const`
- `MStatus setSupportJointLimits (bool supportJointLimits)`
- `bool uniqueSolution (MStatus *ReturnStatus=NULL) const`
- `MStatus setUniqueSolution (bool uniqueSolution)`
- `virtual bool isSingleChainOnly () const`
- `virtual bool isPositionOnly () const`
- `virtual bool hasJointLimitSupport () const`
- `virtual bool hasUniqueSolution () const`
- `virtual bool groupHandlesByTopology () const`
- `virtual MStatus setFuncValueTolerance (double tolerance)`
- `virtual MStatus setMaxIterations (int value)`
- `MIkHandleGroup * handleGroup () const`
- `virtual void setHandleGroup (MIkHandleGroup *)`
- `const MMatrix * toWorldSpace () const`
- `const MMatrix * toSolverSpace () const`
- `double funcValueTolerance () const`
- `int maxIterations () const`
- `virtual void snapHandle (MObject &handle)`
- `virtual bool isAttributeCreatedBySolver (MObject &attr) const`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus getJointAngles (MDoubleArray &) const`
- `MStatus setJointAngles (const MDoubleArray &)`
- `void setToRestAngles ()`
- `MPxIkSolverNode ()`

### MPxSkinCluster

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_skin_cluster.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_skin_cluster.html).

**Public Member Functions**

- `MPxSkinCluster ()`
- `~MPxSkinCluster () override`
- `MPxNode::Type type () const override`
- `float weightValue (MDataBlock &mblock, unsigned int multiIndex, unsigned int wtIndex)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject weightList`
- `static MObject weights`
- `static MObject matrix`
- `static MObject bindPreMatrix`
