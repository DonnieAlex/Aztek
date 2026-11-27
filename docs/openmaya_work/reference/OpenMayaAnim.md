# OpenMayaAnim, API module for animation

Transcribed from the reference's 2022 C++ API reference, module page https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_anim.html.

This is a class and member transcription: names, inheritance, signatures and the one-line descriptions on the class pages. Parameter and return prose that only appears in the per-member detail sections is not reproduced; follow the source link on a class to read it.

Classes in this module: 53.

## Classes

### MAnimControl

Module `OpenMayaAnim`. Source: [class_m_anim_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_control.html).

Control over animation playback and values. This class provide access to the values that control how an animation is played. This includes the minimum and maximum frames included in the playback, whether the playback loops, and whether the animation runs in all views, or only the active view, etc. Methods also exist that mirror the functionality of the controls found on the time slider in the UI to start and stop the playback.

**Public Types**

- `enum PlaybackMode { kPlaybackOnce, kPlaybackLoop, kPlaybackOscillate }`  
  Animation playback modes.
- `enum PlaybackViewMode { kPlaybackViewAll, kPlaybackViewActive }`  
  Animation playback viewing modes.

**Public Member Functions**

- `MAnimControl ()`  
  Class constructor.
- `virtual ~MAnimControl ()`  
  Class destructor.

**Static Public Member Functions**

- `static bool isValid ()`  
  Return if the AnimControl is valid.
- `static MAnimControl::PlaybackMode playbackMode ()`  
  Return the playback mode currently in effect.
- `static MStatus setPlaybackMode (PlaybackMode newMode)`  
  Set the current playback mode.
- `static MAnimControl::PlaybackViewMode viewMode ()`  
  Return the viewing mode currently in effect.
- `static MStatus setViewMode (PlaybackViewMode newMode)`  
  Set the current viewing mode.
- `static double playbackBy ()`  
  Return a double specifying the increment between times viewed during the playing of the animation.
- `static MStatus setPlaybackBy (const double &newTime)`  
  Specify the increment between times viewed during the playing of the animation.
- `static MTime minTime ()`  
  Return an MTime specifying the first frame of the current playback time range.
- `static MTime maxTime ()`  
  Return an MTime specifying the last frame of the current playback time range.
- `static MStatus setMinTime (MTime newMinTime)`  
  Set the first frame of the current playback time range.
- `static MStatus setMaxTime (MTime newMaxTime)`  
  Set the value of the last frame of the current playback time range.
- `static MStatus setMinMaxTime (MTime min, MTime max)`  
  Set the values of the first and last frames of the playback time range.
- `static MTime animationStartTime ()`  
  Return an MTime specifying the first frame of the animation, as specified by the Maya user in the Range Slider UI.
- `static MTime animationEndTime ()`  
  Return an MTime specifying the last frame of the animation, as specified by the Maya user in the Range Slider UI.
- `static MStatus setAnimationStartTime (MTime newStartTime)`  
  Set the value of the first frame in the animation.
- `static MStatus setAnimationEndTime (MTime newEndTime)`  
  Set the value of the last frame in the animation.
- `static MStatus setAnimationStartEndTime (MTime newStartTime, MTime newEndTime)`  
  Set the values of the first and last frames in the animation.
- `static MTime currentTime ()`  
  Return an MTime instance containing the current animation frame.
- `static MStatus setCurrentTime (const MTime &newTime)`  
  Set the current animation frame.
- `static double playbackSpeed ()`  
  Return the speed with with to play the animation.
- `static MStatus setPlaybackSpeed (double newSpeed)`  
  Set the desired speed factor at which the animation will play back.
- `static MStatus playForward ()`  
  Start playing the current animation forwards.
- `static MStatus playBackward ()`  
  Start playing the current animation backwards.
- `static bool isPlaying ()`  
  Return a value indicating whether Maya is currently playing the animation.
- `static bool isScrubbing ()`  
  Return a value indicating whether interactive scrubbing is occuring while Maya is not currently playing an animation.
- `static MStatus stop ()`  
  Stop playing the current animation.
- `static bool autoKeyMode ()`  
  Return the autoKeyMode.
- `static MStatus setAutoKeyMode (bool mode)`  
  Set the autoKeyMode.
- `static MFnAnimCurve::TangentType globalInTangentType (MStatus *ReturnStatus=NULL)`  
  Return the current global in tangent type.
- `static MStatus setGlobalInTangentType (const MFnAnimCurve::TangentType &tangentType)`  
  Set the current global in tangent type.
- `static MFnAnimCurve::TangentType globalOutTangentType (MStatus *ReturnStatus=NULL)`  
  Return the current global out tangent type.
- `static MStatus setGlobalOutTangentType (const MFnAnimCurve::TangentType &tangentType)`  
  Set the current global out tangent type.
- `static bool weightedTangents (MStatus *ReturnStatus=NULL)`  
  Determine whether or not the tangents on the Anim Curve are weighted.
- `static MStatus setWeightedTangents (bool weightState)`  
  Sets whether or not the tangents on the Anim Curve are weighted.


### MAnimCurveChange

Module `OpenMayaAnim`. Source: [class_m_anim_curve_change.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_change.html).

Anim Curve Change Cache. Adding, removing and changing keyframes on an anim curve cannot be done simply by setting attribute values on the corresponding node. This makes it impossible to capture such changes in an MDGModifier for undo/redo purposes. The MAnimCurveChange class provides persistent storage for information concerning changes to anim curve nodes, along with methods to undo and redo those changes. MFnAnimCurve methods which add, remove or change keyframes take an optional MAnimCurveChange instance in which to store information about the changes being made. If the same MAnimCurveChange instance is used for multiple anim curve edit operations, then the cache maintains an undo/redo queue which allows all changes in the series to be undone or redone. If selective undo/redo is required, then a different MAnimCurveCache instance must be used for each edit.

**Public Member Functions**

- `MAnimCurveChange (MStatus *ReturnStatus=NULL)`  
  Class Constructor.
- `~MAnimCurveChange ()`  
  Class Destructor.
- `MStatus undoIt ()`  
  Undoes all of the Anim Curve edits that have been given to this cache.
- `MStatus redoIt ()`  
  Redoes all of the Anim Curve edits that this cache previously undid.
- `bool isInteractive () const`  
  Return the performance hint flag value.
- `void setInteractive (bool value)`  
  The interactive flag is a performance hint.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MAnimCurveClipboard

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard.html).

Control over the animation clipboard. The clipboard is a list of MAnimCurveClipboardItems (i.e. an MAnimCurveClipboardItemArray). All of the data stored on a clipboard remains static; that is, it will persist as long as the application remains running. The items on the clipboard must be ordered. In the case where the clipboard info represents animation of a hierarchy, the order in which items appear in the clipboard is reliant on a depth-first-iteration from the root or the hierarchy. This ordering, is essential to properly match up hierarchies of objects As an example, consider that animation from the following hierarchy is placed into the clipboard. Slanted Dag Objects are animated.

**Public Member Functions**

- `MAnimCurveClipboard ()`  
  Default Constructor.
- `~MAnimCurveClipboard ()`  
  Class Destructor.
- `MStatus set (const MAnimCurveClipboard &cb)`  
  Replaces the contents of the clipboard.
- `MStatus set (const MAnimCurveClipboardItemArray &clipboardItemArray)`  
  Sets the contents of the clipboard.
- `MStatus set (const MAnimCurveClipboardItemArray &clipboardItemArray, const MTime &startTime, const MTime &endTime, const float &startUnitlessInput, const float &endUnitlessInput, bool strictValidation=true)`  
  Sets the contents of the clipboard.
- `MStatus clear ()`  
  This method empties the clipboard.
- `bool isEmpty (MStatus *ReturnStatus=NULL) const`  
  Determines if the clipboard is empty.
- `const MAnimCurveClipboardItemArray clipboardItems (MStatus *ReturnStatus=NULL) const`  
  Returns the contents of the clipboard.
- `MTime startTime (MStatus *ReturnStatus=NULL) const`  
  Returns the start time of the clipboard.
- `MTime endTime (MStatus *ReturnStatus=NULL) const`  
  Returns the end time of the clipboard.
- `float startUnitlessInput (MStatus *ReturnStatus=NULL) const`  
  Returns the start unitless input of the clipboard.
- `float endUnitlessInput (MStatus *ReturnStatus=NULL) const`  
  Returns the end unitless input of the clipboard.

**Static Public Member Functions**

- `static MAnimCurveClipboard & theAPIClipboard ()`  
  Returns the static API clipboard.


### MAnimCurveClipboardItem

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard_item.html).

Wrapper for a clipboard item. This class provides a wrapper for a clipboard item. Common convenience functions are available, and the implementation is compatible with the internal Maya implementation so that it can be passed efficiently between plugins and internal maya data structures.

**Public Member Functions**

- `MAnimCurveClipboardItem ()`  
  Default Constructor.
- `MAnimCurveClipboardItem (const MAnimCurveClipboardItem &r)`  
  Copy Constructor.
- `~MAnimCurveClipboardItem ()`  
  Class Destructor.
- `const MObject animCurve (MStatus *ReturnStatus=NULL) const`  
  Returns the animCurve held by this clipboard item as an MObject.
- `MStatus getAddressingInfo (unsigned int &rowCount, unsigned int &childCount, unsigned int &attributeCount) const`  
  Returns the addressing information for this clipboard item,.
- `const MString & fullAttributeName (MStatus *ReturnStatus=NULL) const`  
  Returns the attribute's full name.
- `const MString & leafAttributeName (MStatus *ReturnStatus=NULL) const`  
  Returns the attribute's leaf name.
- `const MString & nodeName (MStatus *ReturnStatus=NULL) const`  
  Returns the node name.
- `MFnAnimCurve::AnimCurveType animCurveType (MStatus *ReturnStatus=NULL) const`  
  Returns the animCurve type.
- `MStatus setAnimCurve (const MObject &curve)`  
  Sets the clipboard item's animCurve.
- `MStatus setAddressingInfo (unsigned int rowCount, unsigned int childCount, unsigned int attributeCount)`  
  Sets the clipboard item's addressing info.
- `MStatus setNameInfo (const MString &nodeName, const MString &fullName, const MString &leafName)`  
  Sets the clipboard item's name info.
- `MAnimCurveClipboardItem & operator= (const MAnimCurveClipboardItem &from)`  
  Operator= overloading.
- `bool operator== (const MAnimCurveClipboardItem &rhs) const`  
  Compare the individual members for equality.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MAnimCurveClipboardItemArray

Module `OpenMayaAnim`. Source: [class_m_anim_curve_clipboard_item_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_curve_clipboard_item_array.html).

Array of MAnimCurveClipboardItem data type. This class implements an array of MAnimCurveClipboardItems. Common convenience functions are available, and the implementation is compatible with the internal Maya implementation so that it can be passed efficiently between plugins and internal maya data structures.

**Public Member Functions**

- `MAnimCurveClipboardItemArray ()`  
  Default Constructor.
- `MAnimCurveClipboardItemArray (const MAnimCurveClipboardItemArray &other)`  
  Copy Constructor.
- `MAnimCurveClipboardItemArray (unsigned int initialSize, const MAnimCurveClipboardItem &initialValue)`  
  Create a new array of items of a specified size and initialize all the elements with the given initial value.
- `~MAnimCurveClipboardItemArray ()`  
  Class Destructor.
- `const MAnimCurveClipboardItem & operator[] (unsigned int index) const`  
  Index operator.
- `MStatus set (const MAnimCurveClipboardItem &element, unsigned int index)`  
  Sets the value of the indicated element to the indicated MAnimCurveClipboardItem value.
- `MStatus setLength (unsigned int length)`  
  Set the length of the array.
- `unsigned int length () const`  
  Returns the number of elements in the instance.
- `MStatus remove (unsigned int index)`  
  Remove the array element at the given index.
- `MStatus insert (const MAnimCurveClipboardItem &element, unsigned int index)`  
  Inserts a new value into the array at the given index.
- `MStatus append (const MAnimCurveClipboardItem &element)`  
  Adds a new element to the end of the array.
- `MStatus clear ()`  
  Clear the contents of the array.
- `void setSizeIncrement (unsigned int newIncrement)`  
  Set the size by which the array will be expanded whenever expansion is necessary.
- `unsigned int sizeIncrement () const`  
  Return the size by which the array will be expanded whenever expansion is necessary.
- `bool isValid (unsigned int &failedIndex) const`  
  Ensures that the MAnimCurveClipboard items in the array make sense.
- `MAnimCurveClipboardItemArray (const MAnimCurveClipboardItem src[], unsigned int count)`  
  NO SCRIPT SUPPORT.
- `MStatus get (MAnimCurveClipboardItem array[]) const`  
  NO SCRIPT SUPPORT.
- `MAnimCurveClipboardItem & operator[] (unsigned int index)`  
  NO SCRIPT SUPPORT.
- `MAnimCurveClipboardItemArray & operator= (const MAnimCurveClipboardItemArray &other)`  
  Assignment operator copies all of the elements of the other array into this one.
- `MStatus copy (const MAnimCurveClipboardItemArray &source)`  
  Copy the contents of the source array to this array.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MAnimMessage

Module `OpenMayaAnim`. Inherits `MMessage`. Source: [class_m_anim_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_message.html).

Animation messages. This class is used to register callbacks for animation messages. To remove a callback use MMessage::removeCallback. All callbacks that are registered by a plug-in must be removed by that plug-in when it is unloaded. Failure to do so will result in a fatal error.

**Static Public Member Functions**

- `static MCallbackId addAnimCurveEditedCallback (MMessage::MObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  AnimCurve edited callback.
- `static MCallbackId addAnimKeyframeEditedCallback (MMessage::MObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  AnimCurve keyframe edited callback.
- `static MCallbackId addNodeAnimKeyframeEditedCallback (MObject &animNode, MMessage::MNodeObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  AnimCurve keyframe edited callback.
- `static MCallbackId addAnimKeyframeEditCheckCallback (MMessage::MCheckPlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  AnimCurve keyframe edit check callback.
- `static MCallbackId addPreBakeResultsCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  Pre Bake Simulation callback.
- `static MCallbackId addPostBakeResultsCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  Post Bake Simulation callback.
- `static MCallbackId addDisableImplicitControlCallback (MMessage::MPlugsDGModFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  Disable Implicit Control callback.
- `static MCallbackId addAnimKeyframeEditedCallback (MObject &animNode, MMessage::MNodeObjArray func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method is obsolete.
- `static void flushAnimKeyframeEditedCallbacks ()`  
  AnimCurve keyframe edited callback flush.
- `static const char * className ()`  
  Returns the name of this class.


### MAnimUtil

Module `OpenMayaAnim`. Source: [class_m_anim_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_anim_util.html).

Static class providing common animation helper methods. MAnimUtil is a static class which provides methods which determine if an object is being animated, which attributes are animated for a given object and which animation curves are used to animate a given attribute. Note: for purposes of this helper class, "animation" refers to attributes animated by animCurves (i.e. by setting keys). It does not include any animation through expressions or dynamics or timer node connections. Some helper methods may examine different types of objects. Depending upon how the object is represented, the same object may be examined differently. If the object is specified as an MObject, then all of its attributes are examined. If the MObject is a hierarchical object (such as a dag node) and you specify that the objects parents should be examined, then all the parents of an instanced object will be examined. If the object is specified as an MDagPath, then all of its attributes are examined. If you specify that the objects parents should be examined, then only the specified path of the instanced object will be examined. If the object is specifed as an MPlug and no attribute has been set, then the method will behave as if it was called with an MObject. Otherwise only the specified attribute will be examined. For example, a compound attribute will not have its child attributes examined (i.e. if you specify node.translate, node.translateX will not be examined). If the object is specified through an MSelectionList, it can also have multiple representations. If the object was added to the MSelectionList as an MObject, then it will be examined as an MObject. If the object was added to the MSelectionList as an MDagPath, then it will be examined as an MDagPath. If the object was added to the MSelectionList as either a component or an MPlug, it will be examined as an MPlug with additional expansion of compound attributes into their child attributes (for example, if specify node.translate is specified, node.translateX will be examined. More importantly, if you specify poly.f[4] then xValue, yValue and zValue will be examined for each vertex). Below is a summary of how the same object will be examined by MAnimUtil::isAnimated depending upon how it has been represented. If we have the following hierarchical layout: group1 (translateX is animated) pTorus1 (instanced and not animated) pTorusShape1 (face[4] is animated) group2 (not animated) pTorus1 (another instance of pTorus1 as above) The MAnimUtil::isAnimated will respond as follows: isAnimated ( MObject(pTorus1), false ) => false isAnimated ( MObject(pTorus1), true ) => true isAnimated ( MDagPath(group2|pTorus1), false ) => false isAnimated ( MDagPath(group2|pTorus1), true ) => false isAnimated ( MPlug(pTorus1), false ) => false isAnimated ( MPlug(pTorus1), true ) => true isAnimated ( MPlug(group1.translate) ) => false isAnimated ( MPlug(group1.translateX) ) => true isAnimated ( MPlug(pTorusShape1.pnts[4]) ) => false isAnimated ( MPlug(pTorusShape1.pnts[4].xValue) ) => true The following list assumes that MAnimUtil is called with an MSelectionList named list, and shows the response depending upon how the object has been added to list: isAnimated ( list.add(MObject(pTorus1)), false ) => false isAnimated ( list.add(MObject(pTorus1)), true ) => true isAnimated ( list.add(MDagPath(group2|pTorus1)), false ) => false isAnimated ( list.add(MDagPath(group2|pTorus1)), true ) => false isAnimated ( list.add(MPlug(pTorus1)), false ) => false isAnimated ( list.add(MPlug(pTorus1)), true ) => true isAnimated ( list.add(MPlug(group1.translate)) ) => true isAnimated ( list.add(MPlug(group1.translateX) ) => true isAnimated ( list.add(MPlug(pTorusShape1.pnts[4])) ) => true isAnimated ( list.add(MPlug(pTorusShape1.pnts[4].xValue)) ) => true

**Static Public Member Functions**

- `static bool isAnimated (const MObject &node, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Determine whether or not an MObject is animated.
- `static bool isAnimated (const MDagPath &path, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Determine whether or not an MDagPath is animated.
- `static bool isAnimated (const MPlug &plug, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Determine whether or not an MPlug is animated.
- `static bool isAnimated (const MSelectionList &selectionList, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Determine whether or not any member of an MSelectionList is animated.
- `static bool findAnimatedPlugs (const MObject &node, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Find the list of attributes (MPlugs) on an MObject that is animated.
- `static bool findAnimatedPlugs (const MDagPath &path, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Find the list of attributes (MPlugs) on an MDagPath object that is animated.
- `static bool findAnimatedPlugs (const MSelectionList &selectionList, MPlugArray &animatedPlugs, bool checkParent=false, MStatus *ReturnStatus=NULL)`  
  Find the list of attributes (MPlugs) on any member of an MSelectionList that is animated.
- `static bool findAnimation (const MPlug &plug, MObjectArray &animation, MStatus *ReturnStatus=NULL)`  
  Find the animCurve(s) that are animating a given attribute (MPlug).
- `static bool findSetDrivenKeyAnimation (const MPlug &plug, MObjectArray &animation, MPlugArray &drivers, MStatus *ReturnStatus=NULL)`  
  Find any driven keyframe animCurves, the blendWeighted node and the driver attribute(s) that are animating a given attribute (MPlug).
- `static bool findConstraint (const MPlug &plug, MObject &constraint, MObjectArray &targets, MStatus *ReturnStatus=NULL)`  
  Find any constraint that is directly driving the specified attribute.
- `static bool findAnimatablePlugs (const MSelectionList &selectionList, MPlugArray &animatablePlugs)`  
  Find the list of attributes (MPlugs) on any member of an MSelectionList that is animatable.
- `static const char * className ()`  
  Returns the name of this class.


### MBufferIdentifier

Module `OpenMayaAnim`. Source: [class_m_buffer_identifier.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_buffer_identifier.html).

identifier for weight buffer An MBufferIdentifier form a unique identifier for a weight buffer. It can be form with an MpxNode* and a layerId to be able to provide more than one layer per node.

**Public Member Functions**

- `MBufferIdentifier (MPxNode *source, unsigned int layerId=0)`  
  MBufferIdentifier constructor.


### MFalloffContext

Module `OpenMayaAnim`. Source: [class_m_falloff_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_falloff_context.html).

falloff context use in a falloff function to get/set information for weight computation. An MFalloffContext is provided to a MFalloffFunction::operator() that user override to create its own weight falloff.

**Public Member Functions**

- `MFloatArray getValues (const MBufferIdentifier &bufferId) const`  
  Get the weight for the given bufferId.
- `void setValue (const MFloatArray &array, const MBufferIdentifier &bufferId)`  
  copy the value into the buffer addressed by bufferId
- `bool isTopologyDirty () const`  
  Tells if the geometry topology targeted by this falloff has changed sinced last time it evaluated.
- `bool isSync (int syncId, const MBufferIdentifier &bufferId) const`  
  isSync is used to quickly bail out of a fallout function, if it is in sync and additional requirement (no topology change) are met then we can return bufferId as Cached.
- `void sync (int syncId, const MBufferIdentifier &bufferId)`  
  Synchronize the target buffer with a user supplied id.
- `MObject getGeometry (bool originalGeometry=false)`  
  returns the working indexMapper.
- `MIndexMapper getIndexMapper () const`  
  returns the working indexMapper.
- `~MFalloffContext ()`  
  MFalloffContext destructor.


### MFalloffFunction

Module `OpenMayaAnim`. Source: [class_m_falloff_function.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_falloff_function.html).

base class falloff function An MFalloffFunction is meant to be sub-classed to define your weight falloff function.

**Classes**

- `struct ReturnValue`  
  Return Value data type for a falloff function.


### MFnAnimCurve

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_anim_curve.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_anim_curve.html).

Anim Curve Function Set. Create, query and edit Anim Curve Nodes and the keys internal to those Nodes. Create an Anim Curve Node and connect its output to any animatable attribute on another Node. Evaluate an Anim Curve at a particular time. Determine the number of keys held by an Anim Curve. Determine the time and value of individual keys, as well as the (angle,weight) or (x,y) values and type of key tangents. The in-tangent refers to the tangent entering the key. The out-tangent refers to the tangent leaving the key. Find the index of the key at, or closest to a particular time. Set the time and value of individual keys, as well as the type of the tangent to the curve entering (in-tangent) and leaving (out-tangent) the key, specify the tangent as either (angle,weight) or (x,y). Add and remove keys from an Anim Curve. Anim Curves are implemented as Dependency Graph (DG) Nodes. Each Node can have indexed keys. There are eight different types of Anim Curve nodes: timeToAngular (animCurveTA) timeToLinear (animCurveTL) timeToTime (animCurveTT) timeToUnitless (animCurveTU) unitlessToAngular (animCurveUA) unitlessToLinear (animCurveUL) unitlessToTime (animCurveUT) unitlessToUnitless (animCurveUU) Specifying the correct AnimCurveType during creation will avoid implicit unit conversion nodes. Use the Anim Curve Function Set to create Anim Curve Nodes and connect them to animatable attributes on DG Nodes, as well as to query and edit an Anim Curve Node and its keys. On creation, an Anim Curve Function Set may be attached to a given Anim Curve or it may be unattached. The Function Set may be attached to a different Anim Curve Node using the setObject() methods inherited from the Base Function Set (MFnBase) or with the create() methods. Use the create() methods of an Anim Curve Function Set to create a new Anim Curve Node, attach the Function Set to the new Node and connect the output of the Node to a specific animatable Attribute or Plug of another Node. Additionally, the output of an Anim Curve Node may be attached to an Attribute or Plug of another DG Node using the DG Modifier method MDGModifier::connect(). Use the evaluate() methods to determine the value of an Anim Curve at a particular time or a given unitless input. Use the addKey() and remove() methods to add and remove keys to and from an Anim Curve. The query and edit methods of the Anim Curve Function Set act on keys at a specific 0-based index. [Note that the numKeys() method returns a 1-based value]. Keys can be accessed either randomly (via a 0-based index) or serially (using the find() and findClosest() methods). Use the find() or findClosest() methods to determine the index of a key at or closest to a specific time respectively. Use the specific query methods to determine the time, value and tangent information for a key at a given index. Use the specific edit methods to set the time, value and tangent information for a key at a given index. There are methods for setting the x,y values for the in- and out-tangents, as well as setting their angles and weights. Setting the time of a key will fail if the new time would require a re-ordering of the keys. Use the remove() and addKey() methods to re-order the keys. Or use the keyframe command from mel. The Maya developer's kit ships with example code (animDemo) which demonstrates how to evaluate a Maya animation curve independent from Maya. The animation parameter curves in Maya are defined by a restricted set of cubic two-dimensional Bezier curves. It is defined by four points. P1 = (x1, y1) is the (key,value) pair for the first key. P2 = (x2, y2) is a point which defines the outgoing tangent direction at P1. P4 = (x4, y4) defines the second key and P3 = (x3, y3) is a point which defines the incoming tangent direction at P4. There are some basic restrictions for the x coordinates of these points: x1 <= x2 <= x3 <= x4. The 2-dimensional Bezier curve is defined as F(u) = [ u^3 u^2 u 1 ] * B * | P1 | , 0 <= u <= 1

**Public Types**

- `enum AnimCurveType { kAnimCurveTA, kAnimCurveTL, kAnimCurveTT, kAnimCurveTU, kAnimCurveUA, kAnimCurveUL, kAnimCurveUT, kAnimCurveUU, kAnimCurveUnknown }`  
  Defines the input and output type of the animation curve.
- `enum TangentType { kTangentGlobal, kTangentFixed, kTangentLinear, kTangentFlat, kTangentSmooth, kTangentStep, kTangentSlow, kTangentFast, kTangentClamped, kTangentPlateau, kTangentStepNext, kTangentAuto , kTangentAutoMix, kTangentAutoEase, kTangentAutoCustom }`  
  Defines the type of the tangent.
- `enum InfinityType { kConstant = 0, kLinear = 1, kCycle = 3, kCycleRelative = 4, kOscillate = 5 }`  
  Defines the type of the infinity.
- `typedef double TangentValue`  
  Defines the value type of the tangent.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnAnimCurve ()`  
  Destructor.
- `MFnAnimCurve ()`  
  Default constructor.
- `MFnAnimCurve (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnAnimCurve (const MPlug &plug, MStatus *ReturnStatus=NULL)`  
  Class constructor that initializes the function set to the single Anim Curve Node connected to the given MPlug.
- `MObject create (const MObject &node, const MObject &attribute, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Creates a new Anim Curve Node, attaches the Function Set to the new Node (detaching from the current Node) and connects the output of the new Anim Curve Node to an animatable Attribute on another DG Node.
- `MObject create (const MObject &node, const MObject &attribute, AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Creates a new Anim Curve Node, attaches the Function Set to the new Node (detaching from the current Node) and connects the output of the new Anim Curve Node to an animatable Attribute on another DG Node.
- `MObject create (const MPlug &plug, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Creates a new Anim Curve Node, attaches the Function Set to the new Node (detaching from the current Node) and connects the output of the new Anim Curve Node to the Plug on an animatable Attribute on another DG Node.
- `MObject create (const MPlug &plug, AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Creates a new Anim Curve Node, attaches the Function Set to the new Node (detaching from the current Node) and connects the output of the new Anim Curve Node to the Plug on an animatable Attribute on another DG Node.
- `MObject create (AnimCurveType animCurveType, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Creates a new Anim Curve Node, attaches the Function Set to the new Node (detaching from the current Node) but does not attempt to connect the new Anim Curve Node.
- `AnimCurveType animCurveType (MStatus *ReturnStatus=NULL) const`  
  Returns the animCurve type.
- `AnimCurveType timedAnimCurveTypeForPlug (MPlug &plug, MStatus *ReturnStatus=NULL) const`  
  Returns the timed animCurve type appropriate for the specified plug.
- `AnimCurveType unitlessAnimCurveTypeForPlug (MPlug &plug, MStatus *ReturnStatus=NULL) const`  
  Returns the unitless animCurve type appropriate for the specified plug.
- `double evaluate (const MTime &atTime, MStatus *ReturnStatus=NULL) const`  
  Determines the interpolated output value of Anim Curves of type kAnimCurveTA, kAnimCurveTL and kAnimCurveTU at the specified time.
- `MStatus evaluate (const MTime &atTime, double &value) const`  
  Determines the interpolated output value of Anim Curves of type kAnimCurveTA, kAnimCurveTL and kAnimCurveTU at the specified time.
- `MStatus evaluate (const MTime &atTime, MTime &timeValue) const`  
  Determines the interpolated output value of Anim Curves of type kAnimCurveTT at the specified time.
- `MStatus evaluate (const double &atUnitlessInput, double &value) const`  
  Determines the interpolated output value of Anim Curves of type kAnimCurveUA, kAnimCurveUL and kAnimCurveUU at the specified unitless input.
- `MStatus evaluate (const double &atUnitlessInput, MTime &timeValue) const`  
  Determines the interpolated output value of Anim Curves of type kAnimCurveUT at the specified unitless input.
- `bool isStatic (MStatus *ReturnStatus=NULL) const`  
  Determines whether or not the animCurve is static.
- `unsigned int numKeyframes (MStatus *ReturnStatus=NULL) const`  
  This method is obsolete.
- `unsigned int numKeys (MStatus *ReturnStatus=NULL) const`  
  Determines the number of keys on the Anim Curve Node.
- `MStatus remove (unsigned int index, MAnimCurveChange *change=NULL)`  
  Removes the key at the specified index.
- `MStatus addKeyframe (const MTime &time, double value, MAnimCurveChange *change=NULL)`  
  Adds a new key with the given value at the specified time.
- `MStatus addKeyframe (const MTime &time, double value, TangentType tangentInType, TangentType tangentOutType, MAnimCurveChange *change=NULL)`  
  Adds a new key with the given value and tangent types at the specified time.
- `unsigned int addKey (const MTime &time, double value, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`  
  Adds a new key with the given value and tangent types at the specified time for curves of type kAnimCurveTA, kAnimCurveTL and kAnimCurveTU.
- `unsigned int addKey (const MTime &timeInput, const MTime &timeValue, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`  
  Adds a new key with the given output time value and tangent types at the specified input time for curves of type kAnimCurveTT.
- `unsigned int addKey (double unitlessInput, double value, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`  
  Adds a new key with the given value and tangent types at the specified unitless input for curves of type kAnimCurveUA, kAnimCurveUL and kAnimCurveUU.
- `unsigned int addKey (double unitlessInput, const MTime &time, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL, MStatus *ReturnStatus=NULL)`  
  Adds a new key with the given output time value and tangent types at the specified unitless input for curves of type kAnimCurveUT.
- `MStatus addKeys (MTimeArray *timeArray, MDoubleArray *valueArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, bool keepExistingKeys=false, MAnimCurveChange *change=NULL)`  
  Add a set of new keys with the given corresponding values and tangent types at the specified times.
- `MStatus addKeysWithTangents (MTimeArray *timeArray, MDoubleArray *valueArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MIntArray *tangentInTypeArray=NULL, MIntArray *tangentOutTypeArray=NULL, MDoubleArray *tangentInXArray=NULL, MDoubleArray *tangentInYArray=NULL, MDoubleArray *tangentOutXArray=NULL, MDoubleArray *tangentOutYArray=NULL, MIntArray *tangentsLockedArray=NULL, MIntArray *weightsLockedArray=NULL, bool convertUnits=true, bool keepExistingKeys=false, MAnimCurveChange *change=NULL)`  
  Add a set of new keys with the given corresponding values and tangent types at the specified times.
- `unsigned int insertKey (const MTime &time, bool breakdown=false, MAnimCurveChange *change=nullptr, MStatus *ReturnStatus=nullptr)`  
  Inserts a key at the specified time adjusting neighboring tangents to maintain curve shape.
- `bool find (const MTime &time, unsigned int &index, MStatus *ReturnStatus=NULL) const`  
  Determines the index of the key which is set at the specified time.
- `bool find (double unitlessInput, unsigned int &index, MStatus *ReturnStatus=NULL) const`  
  Determines the index of the key which is set at the specified unitless input value.
- `unsigned int findClosest (const MTime &time, MStatus *ReturnStatus=NULL) const`  
  Determines the index of the key which is set at the time closest to the specified time.
- `unsigned int findClosest (double unitlessInput, MStatus *ReturnStatus=NULL) const`  
  Determines the index of the key which is set at the time closest to the specified unitless input value.
- `MTime time (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines the time of the key at the specified index.
- `double value (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines the value of the key at the specified index.
- `double unitlessInput (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines the unitless input value of the key at the specified index.
- `MStatus setValue (unsigned int index, double value, MAnimCurveChange *change=NULL)`  
  Sets the value of the key at the specified index.
- `MStatus setTime (unsigned int index, const MTime &time, MAnimCurveChange *change=NULL)`  
  Sets the time of the key at the specified index.
- `MStatus setUnitlessInput (unsigned int index, double unitlessInput, MAnimCurveChange *change=NULL)`  
  Sets the value of the unitless input of the key at the specified index.
- `bool isTimeInput (MStatus *ReturnStatus=NULL) const`  
  Determines the input type of the animCurve.
- `bool isUnitlessInput (MStatus *ReturnStatus=NULL) const`  
  Determines the input type of the animCurve.
- `TangentType inTangentType (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines the type of the tangent to the curve entering the current key.
- `TangentType outTangentType (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines the type of the tangent to the curve leaving the current key.
- `MStatus setInTangentType (unsigned int index, TangentType tangentType, MAnimCurveChange *change=NULL)`  
  Sets the type of the tangent to the curve entering the key at the specified index.
- `MStatus setOutTangentType (unsigned int index, TangentType tangentType, MAnimCurveChange *change=NULL)`  
  Sets the type of the tangent to the curve leaving the key at the specified index.
- `MStatus setTangentTypes (MIntArray indexArray, TangentType tangentInType=kTangentGlobal, TangentType tangentOutType=kTangentGlobal, MAnimCurveChange *change=NULL)`  
  Sets the tangent types for multiple keys.
- `MStatus getTangent (unsigned int index, TangentValue &x, TangentValue &y, bool inTangent) const`  
  Determines the x,y value representing the vector of the in- or out-tangent (depending on the value of the inTangent parameter) to the curve for the key at the specified index.
- `MStatus getTangent (unsigned int index, MAngle &angle, double &weight, bool inTangent) const`  
  Determines the angle and weight of the in- or out-tangent to the curve for the key at the specified index.
- `MStatus setTangent (unsigned int index, TangentValue x, TangentValue y, bool inTangent, MAnimCurveChange *change=NULL, bool convertUnits=true)`  
  Sets the tangent for the key at the specified index.
- `MStatus setTangent (unsigned int index, const MAngle &angle, double weight, bool inTangent, MAnimCurveChange *change=NULL, bool convertUnits=true)`  
  Sets the tangent for the key at the specified index.
- `MStatus setAngle (unsigned int index, const MAngle &angle, bool inTangent, MAnimCurveChange *change=NULL)`  
  Set the in- or out-angle of the tangent for the key at the given index.
- `MStatus setWeight (unsigned int index, double weight, bool inTangent, MAnimCurveChange *change=NULL)`  
  Set the in- or out-weight of the tangent for the key at the given index.
- `bool weightsLocked (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines whether the weights are locked at the given key.
- `bool tangentsLocked (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines whether the tangents are locked at the given key.
- `MStatus setWeightsLocked (unsigned int index, bool locked, MAnimCurveChange *change=NULL)`  
  Lock or unlock the weights at the given key.
- `MStatus setTangentsLocked (unsigned int index, bool locked, MAnimCurveChange *change=NULL)`  
  Lock or unlock the tangents at the given key.
- `InfinityType preInfinityType (MStatus *ReturnStatus=NULL) const`  
  Determines the behaviour of the curve for the range occurring before the first key.
- `InfinityType postInfinityType (MStatus *ReturnStatus=NULL) const`  
  Determines the behaviour of the curve for the range occurring after the last key.
- `MStatus setPreInfinityType (InfinityType infinityType, MAnimCurveChange *change=NULL)`  
  Set the behaviour of the curve for the range occurring before the first key.
- `MStatus setPostInfinityType (InfinityType infinityType, MAnimCurveChange *change=NULL)`  
  Set the behaviour of the curve for the range occurring after the last key.
- `bool isWeighted (MStatus *ReturnStatus=NULL) const`  
  Determines whether or not the curve has weighted tangents.
- `MStatus setIsWeighted (bool isWeighted, MAnimCurveChange *change=NULL)`  
  Sets whether or not the curve has weighted tangents.
- `bool isBreakdown (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Determines whether or not a key is a breakdown.
- `MStatus setIsBreakdown (unsigned int index, bool isBreakdown, MAnimCurveChange *change=NULL)`  
  Sets the breakdown state of a key at a given index.
- `MFnAnimCurve (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnBlendShapeDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_blend_shape_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_blend_shape_deformer.html).

blend shape deformer function set MFnBlendShapeDeformer is the function set for blend shape deformers. A blend shape deformer takes a base shape (polygonal surface, curve, surface, or lattice) and blends it with other target shapes based on weight values. The blend shape deformer is actually a small network of dependency nodes in the dependency graph. This function set is provided to make manipulation of the network easier. The main deformer node should be given to this function set as its object. There are three parts to a blend shape deformer. There are the base objects, the target objects, and the weight values. The base objects are the shapes that are to be deformed. There must be at least one base object. The base objects will change form as the targets and deformation parameters are modified. Each base object has a list of target objects that affect its shape. Each target is associated with one of the the deformer's weight values. When the weight value increases, the target has more influence on the base shape. There is just one array of weight values between all of the base objects and targets. So, it is possible for targets of different base objects to share the same weight index. When the weight value changes, it will affect all of the base objects that have targets using that weight value. It is also possible to chain together target shapes so that a base object will deform through each shape one at a time as the weight value increases. This is done by adding multiple targets to a base shape using the same weight index for all of them. When each target is added, a weight value is specified at which that target will be in full effect. Give each target a different full weight value. For example, one could take a sphere and make it blend into a cone and then into a cylinder. One way to do this is to make sphere the base shape. Then, add the cone as a target for the sphere at weight index 0 with a full effect weight of 0.5. Next, add the cylinder as a second target for the sphere also at weight index 0, but with a full effect weight of 1.0. Now, as the weight goes from 0 to 1, the base shape will start as a sphere, morph into a cone, and then into a cylinder. It is not necessary for the base shape and its targets to have identical vertex/CV counts, but the blend will be more effective if they do.

**Public Types**

- `enum Origin { kLocalOrigin, kWorldOrigin }`  
  Specifies the space to use for the deformation origin.
- `enum HistoryLocation { kFrontOfChain, kNormal, kPost, kOther }`  
  Specifies where in the shape's history to place the deformer (deformation order).
- `enum TargetType { kObject, kTangent }`  
  TODO.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnBlendShapeDeformer ()`  
  Destructor.
- `MFnBlendShapeDeformer ()`  
  Default constructor.
- `MFnBlendShapeDeformer (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create (MObject baseObject, Origin originSpace=kLocalOrigin, MStatus *ReturnStatus=NULL)`  
  Creates a new blend shape deformer in the dependency graph with the specified shape as the baseObject.
- `MObject create (const MObjectArray &baseObjects, Origin originSpace=kLocalOrigin, HistoryLocation=kNormal, MStatus *ReturnStatus=NULL)`  
  Creates a new blend shape deformer in the dependency graph.
- `MStatus addBaseObject (MObject &object)`  
  Adds a new base object to the deformer.
- `MStatus getBaseObjects (MObjectArray &objects) const`  
  Get a list of all of the base objects for this deformer.
- `MStatus addTarget (const MObject &baseObject, int weightIndex, const MObject &newTarget, double fullWeight, TargetType targetType=kObject)`  
  Adds a new target object for the given base object.
- `MStatus addTarget (const MObject &baseObject, int weightIndex, double fullWeight, TargetType targetType=kObject)`  
  Adds a new delta target object for the given base object.
- `MStatus removeTarget (const MObject &baseObject, int weightIndex, const MObject &target, double fullWeight)`  
  Remove a target object for the given base object.
- `MStatus getTargets (MObject baseObject, int weightIndex, MObjectArray &targetObjects) const`  
  Get a list of all of the target objects for the given base object that affect it based on the given weight index.
- `unsigned int numWeights (MStatus *ReturnStatus=NULL) const`  
  Return the number of weight values that this blend shape deformer has.
- `MStatus weightIndexList (MIntArray &indexList) const`  
  Return the array index numbers corresponding to the targets.
- `MStatus targetItemIndexList (unsigned int weightIndex, MObject baseObject, MIntArray &targetItemIndices) const`  
  A base object may have more than one target using the same element of the blendShape's 'weights' array.
- `float weight (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Get the weight value at the given index.
- `MStatus setWeight (unsigned int index, float weight)`  
  Set the weight value at the given index.
- `float envelope (MStatus *ReturnStatus=NULL) const`  
  Gets the envelope value of the deformer.
- `MStatus setEnvelope (float envelope)`  
  Sets the envelope value of the deformer.
- `Origin origin (MStatus *ReturnStatus=NULL) const`  
  Gets the origin space.
- `MStatus setOrigin (Origin space)`  
  Sets the origin space.
- `HistoryLocation historyLocation (MStatus *ReturnStatus=NULL) const`  
  Gets the history location (deformation order).
- `MFnBlendShapeDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnCharacter

Module `OpenMayaAnim`. Inherits `MFnSet`. Source: [class_m_fn_character.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_character.html).

Function Set for Characters. Maya offers "character" nodes to simplify the setting of keyframes on collections of attributes. The implementation of the "character" is a sub-class of MFnSet taking advantage of the fact that attributes can be represented as MObjects and can be made members of a set. The fact that sets also derive from MObject means that characters may have other character sets as members thus establishing a hierarchy. Only attributes and characters can be members of a character set. The character node will disallow the addition of other objects to its set. Character sets are also part of a partition, meaning that membership of character sets cannot overlap with other character sets. Thus, when an attribute already in a character is added to another character it must be removed from the original character. Characters are integral to Maya's nonlinear animation system, "Trax". Trax allows the user to create "animation clips", which bundle a set of animation curves so that they can be reused multiple times, with different timing then the original clip. When a clip is created, Maya finds the the animation curves that are attached to the attributes in the character set and moves those animation curves into the newly created clip. The MFnClip function set is the Maya function set for clips. Clips in maya can be of two types: source clips and scheduled clips. In the Maya UI, source clips are visible in the Visor while scheduled clips are visible in Trax. A source clip contains the animation curves for the clip. An scheduled clip contains data about the placement of an instance of a source clip in the Maya timeline. In this context, an "instance" means that the animation curves from the source clip are shared by the scheduled clip. Scheduled clips never contain their own animation curves, they always refer to a source clip's curves. For example, if you create a clip called "run" in maya that lasts from frames 1-20, a source clip node will be created with a start of 1, a duration of 19, and dependency graph connections to all of the animation curves that make up the "run". If you then place an instance of the run clip at frame 5 and another instance of the run clip at frame 20, you have 2 scheduled clips: one with a start frame of 5 and one with a start frame of 20. As mentioned in the previous paragraph, only a single set of animation curves exist for the run regardless of the number of times the run is scheduled. Trax also allows you to create "blends" between clips, which enable you to control the transition between the clips. A blend is represented in the dependency graph by an "animBlendInOut" node, which uses an animation curve to determine the transition type. In the dependency graph, when a character has animation clips, the character node will always be connected to a "clipLibrary" node and a "clipScheduler" node. The clipLibrary node is connected to all of the source clips and their animation curves. The clipScheduler node is connected to the scheduled clips and blends. It is the clipScheduler that computes the final animation by looking at the placement and overlap of the clips and feeding the attribute data back into the character set.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnCharacter ()`  
  Destructor.
- `MFnCharacter ()`  
  Default constructor.
- `MFnCharacter (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MStatus attachSourceToCharacter (MObject &sourceClip, MDGModifier &dgMod)`  
  Attaches a given source clip node (created using MFnClip::createSourceClip) to the character's clipLibrary.
- `MStatus attachInstanceToCharacter (MObject &instanceClip, MDGModifier &dgMod)`  
  Attaches an instance of a clip to the character.
- `MStatus addCurveToClip (MObject &curve, MObject &sourceClip, MPlug &characterPlug, MDGModifier &dgMod)`  
  Adds an animation curve to a clip.
- `MObject createBlend (MObject &instancedClip1, MObject &instancedClip2, MObject &blendAnimCurve, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`  
  Creates a blend between two instanced clips on the character.
- `bool blendExists (MObject &instancedClip1, MObject &instancedClip2, MObject &blendResult)`  
  Return true if a blend exists between the two instanced clips on the character.
- `bool removeBlend (MObject &instancedClip1, MObject &instancedClip2, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`  
  Remove the blend between the two instanced clips on the character.
- `bool getCharacterThatOwnsPlug (MPlug &plug, MObject &result) const`  
  Given a plug, test the plug to see if it is owned by a character.
- `MStatus getMemberPlugs (MPlugArray &result) const`  
  Get the members of the character set that are attributes.
- `MStatus getSubCharacters (MSelectionList &result) const`  
  Get a list of the subcharacters that are members of the character set.
- `MObject getClipScheduler (MStatus *ReturnStatus=NULL)`  
  Get the clipScheduler node that manages the playback of clips on this character.
- `int getScheduledClipCount (MStatus *ReturnStatus=NULL)`  
  Return the number of clips that have been scheduled on this character.
- `MObject getScheduledClip (int index, MStatus *ReturnStatus=NULL)`  
  Return the scheduled animClip node corresponding to the specified index.
- `int getSourceClipCount (MStatus *ReturnStatus=NULL)`  
  Return the number of source clips managed by the clipLibrary node of this character.
- `MObject getSourceClip (int index, MStatus *ReturnStatus=NULL)`  
  Return the animClip node corresponding to the specified index.
- `int getBlendCount (MStatus *ReturnStatus=NULL)`  
  Return the number of blends that have been added to clips on this character.
- `MObject getBlend (int index, MStatus *ReturnStatus=NULL)`  
  Return the animBlendInOut node corresponding to the specified index.
- `MStatus getBlendClips (int index, MObject &clip1, MObject &clip2)`  
  Returns the clip nodes that are blended by the blend node corresponding to the specified index.
- `MFnCharacter (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnClip

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_clip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_clip.html).

clip function set Maya uses animation clips as the building block of its nonlinear animation system, "Trax". An animation clip allows the user to bundle a set of animation curves independent of time so that they can be reused multiple times, with different timing then the original clip. Character sets (MFnCharacter) are integral to Maya's clip architecture, since a clip contains whatever attributes of the character were animated when the clip was created. For more information on character sets, refer to the MFnCharacter documentation. Clips in maya can be of two types: source clips and scheduled clips. In the Maya UI, source clips are visible in the Visor while scheduled clips are visible in Trax. A source clip contains the animation curves for the clip. A scheduled clip contains data about the placement of an instance of a source clip in the Maya timeline. In this context, an "instance" means that the animation curves from the source clip are shared by the scheduled clip. Scheduled clips never contain their own animation curves, they always refer to a source clip's curves. For example, if you create a clip called "run" in maya that lasts from frames 1-20, a source clip node will be created with a start of 1, a duration of 19, and dependency graph connections to all of the animation curves that make up the "run". If you then place an instance of the run clip at frame 5 and another instance of the run clip at frame 20, you have 2 scheduled clips: one with a start frame of 5 and one with a start frame of 20. As mentioned in the previous paragraph, only a single set of animation curves exist for the run regardless of the number of times the run is scheduled. Trax also allows you to create "blends" between clips, which enable you to control the transition between the clips. A blend is represented in the dependency graph by an "animBlendInOut" node, which uses an animation curve to determine the transition type. In the dependency graph, when a character has animation clips, the character node will always be connected to a "clipLibrary" node and a "clipScheduler" node. The clipLibrary node is connected to all of the source clips and their animation curves. The clipScheduler node is connected to the scheduled clips and blends. It is the clipScheduler that computes the final animation by looking at the placement and overlap of the clips and feeding the attribute data back into the character set. To create a source clip, the typical steps are: If the attributes are not already in a character, create a character containing the attributes. Create a source clip using MFnClip::createSourceClip Attach the clip to a character using MFnCharacter::attachSourceToCharacter Attach animation curves to the clip using MFnCharacter::addCurveToClip To create a scheduled clip so that it appears in the Trax Editor, the typical steps are: Create an instanced animClip node using MFnClip::createInstancedClip, and associate it with the source clip that you want to instance. Attach the clip instance to the character using MFnCharacter::attachInstanceToCharacter

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnClip ()`  
  Destructor.
- `MFnClip ()`  
  Default constructor.
- `MFnClip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject createSourceClip (const MTime &sourceStart, const MTime &sourceDuration, MDGModifier &dgMod, MStatus *ReturnStatus=NULL)`  
  Creates a source clip node and associates it with this function set.
- `MObject createInstancedClip (MObject &sourceClip, const MTime &start, MDGModifier &dgMod, MStatus *ReturnStatus=NULL, bool absolute=1, double postCycle=0.0, double weight=1.0, double scale=1.0, double preCycle=0.0)`  
  Creates an instance of a clip that will be viewable in the Trax editor.
- `bool isInstancedClip (MStatus *ReturnStatus=NULL)`  
  Return true or false as to whether the clip node represents the source clip or an instanced clip.
- `bool isPose (MStatus *ReturnStatus=NULL)`  
  Return true or false as to whether the clip node represents a pose.
- `MObject sourceClip (MStatus *ReturnStatus=NULL)`  
  Return the source clip associated with the MFnClip's clip.
- `double getPreCycle (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's pre cycle attribute.
- `double getPostCycle (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's post cycle attribute.
- `double getCycle (MStatus *ReturnStatus=NULL)`  
  This method is obsolete.
- `double getWeight (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's weight attribute.
- `double getScale (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's scale attribute.
- `bool getAbsolute (MStatus *ReturnStatus=NULL)`  
  This method is obsolete.
- `bool getEnabled (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's enable attribute.
- `MTime getStartFrame (MStatus *ReturnStatus=NULL)`  
  Return the value of this clip's start frame.
- `MTime getSourceStart (MStatus *ReturnStatus=NULL)`  
  Return the value of the start frame of this clip's source clip.
- `MTime getSourceDuration (MStatus *ReturnStatus=NULL)`  
  Return the value of the start frame of this clip's source duration.
- `MStatus getMemberAnimCurves (MObjectArray &curves, MPlugArray &associatedAttrs)`  
  Return two arrays: the first contains the animCurves associated with this clip.
- `MStatus getAbsoluteChannelSettings (MIntArray &absoluteChannels)`  
  Return an array indicating which channels of the clip are absolute and which are relative.
- `int getTrack (MStatus *ReturnStatus=NULL)`  
  Return the track number for the clip.
- `MStatus setPoseClip (bool state, MDGModifier *mod=NULL)`  
  Specify whether or not this clip node should be tagged as a pose rather than a clip.
- `MStatus setPreCycle (double cycle, MDGModifier *mod=NULL)`  
  Specify the pre cycle value for the clip.
- `MStatus setPostCycle (double cycle, MDGModifier *mod=NULL)`  
  Specify the post cycle value for the clip.
- `MStatus setCycle (double cycle, MDGModifier *mod=NULL)`  
  This method is obsolete.
- `MStatus setWeight (double wt, MDGModifier *mod=NULL)`  
  Specify a weight value for the clip.
- `MStatus setScale (double scale, MDGModifier *mod=NULL)`  
  Specify a scale value for the clip.
- `MStatus setAbsolute (bool abs, MDGModifier *mod=NULL)`  
  This method is obsolete.
- `MStatus setEnabled (bool val, MDGModifier *mod=NULL)`  
  Specify whether or not the clip is enabled.
- `MStatus setStartFrame (const MTime &start, MDGModifier *mod=NULL)`  
  Specify the start frame for the instanced clip.
- `MStatus setSourceData (const MTime &start, const MTime &duration, MDGModifier *mod=NULL)`  
  Specify the start frame and duration for the source clip associated with this clip.
- `MStatus setTrack (int index, MDGModifier *mod=NULL)`  
  Specify the one-based track number for the clip.
- `MStatus setAbsoluteChannelSettings (const MIntArray &absoluteChannels, MDGModifier *mod=NULL)`  
  Set which channels of the clip are absolute and which are relative.
- `MFnClip (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnGeometryFilter

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_geometry_filter.html).

geometry filter function set MFnGeometryFilter is the function set for geometry filters, the node that is the base class for deformers. Geometry filter nodes include clusters, ffds, nonlinears, user-defined deformers, sculpts, wires and blendShapes. The purpose of the geometry filter is to connect to the geometry that is deformed. The geometry filter is independent of any algorithm that calculates the deformation. This function set provides methods for finding out which geometries are connected to geometry filter nodes.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnGeometryFilter ()`  
  Destructor.
- `MFnGeometryFilter ()`  
  Default constructor.
- `MFnGeometryFilter (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MStatus getInputGeometry (MObjectArray &objects) const`  
  This method returns the input geometry for the deformer by traversing the graph to find upstream shape nodes.
- `MStatus getOutputGeometry (MObjectArray &objects) const`  
  The output geometry is packed into the provided list of MObjects.
- `MObject inputShapeAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Returns the input shape corresponding to the plug index.
- `MObject outputShapeAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Returns the output shape corresponding to the plug index.
- `unsigned int indexForOutputShape (const MObject &shape, MStatus *ReturnStatus=NULL) const`  
  Returns the plug index for the specified output shape.
- `MStatus getPathAtIndex (unsigned int index, MDagPath &path) const`  
  The DAG path of the output geometry at the specified plug index is put in the dagPath argument.
- `unsigned int indexForGroupId (unsigned int groupId, MStatus *ReturnStatus=NULL) const`  
  Returns the plug index corresponding to the groupId.
- `unsigned int groupIdAtIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`  
  Returns the groupId at the specified plug index.
- `unsigned int numOutputConnections (MStatus *ReturnStatus=NULL) const`  
  Returns the number of output geometries connected to this node.
- `unsigned int indexForOutputConnection (unsigned int connectionIndex, MStatus *ReturnStatus=NULL) const`  
  Returns the plug index corresponding to the connection index.
- `MObject deformerSet (MStatus *ReturnStatus=NULL) const`  
  Returns the set containing the objects that are deformed.
- `float envelope (MStatus *ReturnStatus=NULL) const`  
  Returns the envelope value.
- `MStatus setEnvelope (float envelope)`  
  Sets the envelope value.
- `MObject getComponentAtIndex (unsigned int index, MStatus *ReturnStatus=NULL)`  
  Get the component used for the ith deformed geometry.
- `MFnGeometryFilter (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnHikEffector

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_hik_effector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_hik_effector.html).

Full Body IK end effector function set. MFnHikEffector is the function set for full body ik effectors. An Full Body IK (FBIK/HIK) effector is a special transform that allows users to manipulate a Full Body IK system. The methods of the parent class MFnTransform used to position the end effector.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnHikEffector ()`  
  Destructor.
- `MFnHikEffector ()`  
  Default constructor.
- `MFnHikEffector (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnHikEffector (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Creates a new fbik effector.
- `MVector getPivotOffset (MStatus *ReturnStatus=NULL) const`  
  Retrieve the pivot offset of this effector.
- `MStatus setPivotOffset (const MVector &vector)`  
  Set the pivot offset for this effector.
- `MColor getEffColor (MStatus *ReturnStatus=NULL) const`  
  Retrieve the cached humanIK color of this effector.
- `MStatus setEffColor (const MColor &color)`  
  Set the humanIK color for this effector.
- `MStatus getAuxiliaryEffectors (MObjectArray &effs) const`  
  Returns an array of the auxiliary effectors associated with this effector.
- `MFnHikEffector (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnIkEffector

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_effector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_effector.html).

Inverse kinematics end effector function set. MFnIkEffector is the function set for inverse kinematic end effectors. An end effector is a point on the skeleton, usually the last bone. When an IK system solves, it is trying to calculate the rotations on the joints necessary to get the end effector to the target point. The methods of the parent class MFnTransform used to position the end effector.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnIkEffector ()`  
  Destructor.
- `MFnIkEffector ()`  
  Default constructor.
- `MFnIkEffector (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnIkEffector (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Creates a new end effector.
- `MFnIkEffector (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnIkHandle

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_handle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_handle.html).

Function set for inverse kinematics (IK) handles. This is the function set for inverse kinematics (IK) handles. An ik handle specifies the joints in a skeleton that are effected by an attached ik solver.

**Public Types**

- `enum Stickiness { kStickyOff, kStickyOn, kSuperSticky }`  
  Specifies a handle's stickiness when the skeleton is being manipulated interactively.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnIkHandle ()`  
  Destructor.
- `MFnIkHandle ()`  
  Default constructor.
- `MFnIkHandle (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnIkHandle (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MDagPath &startJoint, MDagPath &effector, MStatus *ReturnStatus=NULL)`  
  Creates a new ik handle.
- `MStatus getStartJoint (MDagPath &jointPath)`  
  This method will get a dag path to the starting joint of the handle's joint chain.
- `MStatus setStartJoint (const MDagPath &jointPath)`  
  This method will set the dag path for the starting joint of the handle's joint chain.
- `MStatus getEffector (MDagPath &effectorPath)`  
  Get a dag path to the end-effector of the handle's joint chain.
- `MStatus setEffector (const MDagPath &effectorPath)`  
  Set the dag path to the end-effector of the handle's joint chain.
- `MStatus setStartJointAndEffector (const MDagPath &jointPath, const MDagPath &effectorPath)`  
  This method will set the dag path for the starting joint and the end-effector of the handle's joint chain.
- `unsigned int priority (MStatus *ReturnStatus=NULL)`  
  Get the priority of this handle in case a solution is affected by more than one handle.
- `MStatus setPriority (unsigned int priority)`  
  Set the priority of this handle in case a solution is affected by more than one handle.
- `Stickiness stickiness (MStatus *ReturnStatus=NULL)`  
  Get the stickiness of this handle.
- `MStatus setStickiness (Stickiness stickiness)`  
  Set the stickiness of this handle.
- `double weight (MStatus *ReturnStatus=NULL)`  
  Get the handles weight in error calculations.
- `MStatus setWeight (double weight)`  
  Specifies the handles weight in error calculations.
- `double poWeight (MStatus *ReturnStatus=NULL)`  
  Gets the position/orientation weight of a handle.
- `MStatus setPOWeight (double weight)`  
  Sets the position/orientation weight of a handle.
- `MObject solver (MStatus *ReturnStatus=NULL)`  
  Returns the solver attached to this handle.
- `MStatus setSolver (const MObject &solver)`  
  Set the solver for this handle.
- `MStatus setSolver (const MString &solverName)`  
  Set the solver associated with this handle by name.
- `MFnIkHandle (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnIkJoint

Module `OpenMayaAnim`. Inherits `MFnTransform`. Source: [class_m_fn_ik_joint.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_joint.html).

Function set for joints. This is the function set for joints. The transformation matrix for a joint node is below. matrix = [S] * [RO] * [R] * [JO] * [IS] * [T] (where '*' denotes matrix multiplication). These matrices are defined as follows: [S] : scale [RO] : rotateOrient (attribute name is rotateAxis) [R] : rotate [JO] : jointOrient [IS] : parentScaleInverse [T] : translate The methods to get the value of these matrices are: [S] : getScale [RO] : getScaleOrientation [R] : getRotation [JO] : getOrientation [IS] : (the inverse of the getScale on the parent transformation matrix) [T] : translation

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnIkJoint ()`  
  Destructor.
- `MFnIkJoint ()`  
  Default constructor.
- `MFnIkJoint (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnIkJoint (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Create a new joint in a skeleton.
- `MStatus getOrientation (MQuaternion &quaternion) const`  
  Get the joint orientation.
- `MStatus setOrientation (const MQuaternion &quaternion)`  
  Set the jointOrient value.
- `MStatus getOrientation (MEulerRotation &rotation) const`  
  Get the orientation of the coordinate axes.
- `MStatus setOrientation (const MEulerRotation &rotation)`  
  Set the orientation of the coordinate axes.
- `MStatus getOrientation (double rotation[3], MTransformationMatrix::RotationOrder &order) const`  
  Get the orientation of the coordinate axes.
- `MStatus setOrientation (const double rotation[3], MTransformationMatrix::RotationOrder order)`  
  Set the orientation of the coordinate axes.
- `MStatus getScaleOrientation (MQuaternion &quaternion) const`  
  Get the orientation of the coordinate axes for rotation.
- `MStatus setScaleOrientation (const MQuaternion &quaternion)`  
  Set the orientation of the coordinate axes for rotation.
- `MStatus getScaleOrientation (double rotation[3], MTransformationMatrix::RotationOrder &order) const`  
  Get the orientation of the coordinate axes for rotation.
- `MStatus setScaleOrientation (const double rotation[3], MTransformationMatrix::RotationOrder order)`  
  Set the orientation of the coordinate axes for rotation.
- `MStatus getSegmentScale (double scale[3]) const`  
  Get the local space scale values for the joint segment (bone).
- `MStatus setSegmentScale (const double scale[3])`  
  Set the local space scale values for the joint segment (bone).
- `MStatus getStiffness (double stiffness[3]) const`  
  Get the stiffness (from 0 to 100.0) for the joint.
- `MStatus setStiffness (const double stiffness[3])`  
  Set the stiffness (from 0 to 100.0) for the joint.
- `MStatus getPreferredAngle (double rotation[3]) const`  
  Get the preferred orientation for this joint (in XYZ order)
- `MStatus setPreferredAngle (const double rotation[3])`  
  Set the preferred orientation for this joint (in XYZ order)
- `MStatus getDegreesOfFreedom (bool &freeInX, bool &freeInY, bool &freeInZ) const`  
  Get degrees of freedom of this joint.
- `MStatus setDegreesOfFreedom (bool freeInX, bool freeInY, bool freeInZ)`  
  Set the degrees of freedom of this joint by specifying which axes are allowed to rotate.
- `double minRotateDampXRange (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping range in X.
- `double minRotateDampYRange (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping range in Y.
- `double minRotateDampZRange (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping range in Z.
- `double maxRotateDampXRange (MStatus *ReturnStatus=NULL) const`  
  Get the maximum of the damping range in X.
- `double maxRotateDampYRange (MStatus *ReturnStatus=NULL) const`  
  Get the maximum of the damping range in Y.
- `double maxRotateDampZRange (MStatus *ReturnStatus=NULL) const`  
  Get the maximum of the damping range in Z.
- `MStatus setMinRotateDampXRange (double angle)`  
  Set the minimum of the damping range in X.
- `MStatus setMinRotateDampYRange (double angle)`  
  Set the minimum of the damping range in Y.
- `MStatus setMinRotateDampZRange (double angle)`  
  Set the minimum of the damping range in Z.
- `MStatus setMaxRotateDampXRange (double angle)`  
  Set the maximum of the damping range in X.
- `MStatus setMaxRotateDampYRange (double angle)`  
  Set the maximum of the damping range in Y.
- `MStatus setMaxRotateDampZRange (double angle)`  
  Set the maximum of the damping range in Z.
- `double minRotateDampXStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `double minRotateDampYStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `double minRotateDampZStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `double maxRotateDampXStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `double maxRotateDampYStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `double maxRotateDampZStrength (MStatus *ReturnStatus=NULL) const`  
  Get the minimum of the damping strength in X.
- `MStatus setMinRotateDampXStrength (double angle)`  
  Set the maximum of the damping strength in Z.
- `MStatus setMinRotateDampYStrength (double angle)`  
  Set the maximum of the damping strength in Y.
- `MStatus setMinRotateDampZStrength (double angle)`  
  Set the minimum of the damping strength in Z.
- `MStatus setMaxRotateDampXStrength (double angle)`  
  Set the maximum of the damping strength in X.
- `MStatus setMaxRotateDampYStrength (double angle)`  
  Set the maximum of the damping strength in Y.
- `MStatus setMaxRotateDampZStrength (double angle)`  
  Set the maximum of the damping strength in Z.
- `MString hikJointName (MStatus *ReturnStatus=NULL) const`  
  Get the name that the HumanIK solver uses to identify this joint.
- `MFnIkJoint (const MObject &object, MStatus *ret=NULL)`  
  Constructor.
- `MStatus getHikJointName (MString &name) const`  
  NO SCRIPT SUPPORT.
- `MStatus getPreferedAngle (double rotation[3]) const`  
  Obsolete - use correctly spelled getPreferredAngle.
- `MStatus setPreferedAngle (const double rotation[3])`  
  Obsolete - use correctly spelled setPreferredAngle.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnIkSolver

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_ik_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ik_solver.html).

Function set for inverse kinematics (IK) solvers. This is the function set for inverse kinematics (IK) solvers. This function set is used for setting and querying attached ik solvers.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnIkSolver ()`  
  Destructor.
- `MFnIkSolver ()`  
  Default constructor.
- `MFnIkSolver (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `unsigned int maxIterations (MStatus *ReturnStatus=NULL)`  
  Returns the maximum number of iterations used when solving.
- `MStatus setMaxIterations (unsigned int maxIters)`  
  Sets the maximum number of iterations used when solving.
- `double tolerance (MStatus *ReturnStatus=NULL)`  
  Returns the tolerance used when solving.
- `MStatus setTolerance (double tolerance)`  
  Sets the tolerance used when solving.
- `MFnIkSolver (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDelta

Module `OpenMayaAnim`. Inherits `MFnBase`. Source: [class_m_fn_keyframe_delta.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta.html).

Base function set for keyframe deltas. Keyframe delta objects are returned via the MAnimMessage::animKeyframeEditedCallback( ... ). They describe atomic changes to keyframes as a result of a curve edit operation. Refer to the documentation in MAnimMessage class for further information. Base function set for all keyframe delta objects. A keyframe delta object is generated from the MAnimMessage::addAnimKeyframeEditedCallback.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDelta ()`  
  Destructor.
- `MFnKeyframeDelta ()`  
  Default constructor.
- `MFnKeyframeDelta (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject paramCurve (MStatus *ReturnStatus=NULL) const`  
  Return the Animation Curve MObject that this key belongs to.
- `virtual unsigned int keyIndex (MStatus *ReturnStatus=NULL) const`  
  The index of this key on the animation curve.
- `MFnKeyframeDelta (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaAddRemove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_add_remove.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_add_remove.html).

Function set for the addition or removal keys on a curve. Captures the addition or removal of key on a curve. keyIndex(...) will return the index of the key that was added or removed.

**Public Types**

- `enum DeltaType { kAdded, kRemoved, kReplaced }`  
  Type of change.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaAddRemove ()`  
  Destructor.
- `MFnKeyframeDeltaAddRemove ()`  
  Default constructor.
- `MFnKeyframeDeltaAddRemove (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `DeltaType deltaType (MStatus *ReturnStatus=NULL) const`  
  Indicates the type of change that this class instance represents.
- `double value (MStatus *ReturnStatus=NULL) const`  
  The value of the key that was added or removed.
- `MTime time (MStatus *ReturnStatus=NULL) const`  
  The time value of the key that was added or removed.
- `unsigned int keyIndex (MStatus *ReturnStatus=NULL) const override`  
  The index of this key on the animation curve.
- `double replacedValue (MStatus *ReturnStatus=NULL) const`  
  The value of the key that was replaced.
- `MFnKeyframeDeltaAddRemove (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaBlockAddRemove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_block_add_remove.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_block_add_remove.html).

Function set for block add or removal of keys. Certain keyframe editing operations work on group of keys rather than an single key. These operations require this function set to retrieve changes. Function set for reading a block of keyframe changes. They are used internally to set a large group of keys quickly. Because changes occur in groups using base class keyframe methods on this derived class do not make sense. Therefore, methods describing where the block of keys were added or removed are available though methods on this class.

**Public Types**

- `enum DeltaType { kAdded, kRemoved }`  
  Type of change.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaBlockAddRemove ()`  
  Destructor.
- `MFnKeyframeDeltaBlockAddRemove ()`  
  Default constructor.
- `MFnKeyframeDeltaBlockAddRemove (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `DeltaType deltaType (MStatus *ReturnStatus=NULL) const`  
  Indicates the type of change, i.e.
- `MTime startTime (MStatus *ReturnStatus=NULL) const`  
  An MTime value indicating the start time of the add/remove.
- `MTime endTime (MStatus *ReturnStatus=NULL) const`  
  Returns an MTime value indicating the endTime of the add/remove.
- `unsigned int numKeys (MStatus *ReturnStatus=NULL) const`  
  Total number of keys involved in this add or remove operation.
- `void getValues (MDoubleArray &values, MStatus *ReturnStatus=NULL) const`  
  Returns the values of all keys involved in the group add or remove.
- `void getTimes (MTimeArray &times, MStatus *ReturnStatus=NULL) const`  
  Returns the times of all keys involved in the group add or remove.
- `MFnKeyframeDeltaBlockAddRemove (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaBreakdown

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_breakdown.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_breakdown.html).

Function set for changes in a key's breakdown state. Delta keyframe object for changes in breakdown state of a key.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaBreakdown ()`  
  Destructor.
- `MFnKeyframeDeltaBreakdown ()`  
  Default constructor.
- `MFnKeyframeDeltaBreakdown (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `bool wasBreakdown (MStatus *ReturnStatus=NULL) const`  
  Returns the previous breakdown state of the key.
- `bool isBreakdown (MStatus *ReturnStatus=NULL) const`  
  Returns the current breakdown state of the key.
- `MFnKeyframeDeltaBreakdown (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaInfType

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_inf_type.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_inf_type.html).

Function set for changes in pre or post infinity type. This delta object represents changes in the pre-infinity or post-infinity type of the animation curve. Pre-infinity and post-infinity types are specific to animation curves and do not have a corresponding key associated with them. It is, therefore, invalid to have a keyframe associated with a MFnKeyframeDeltaInfType. To determine if this class represents a change to pre or post infinity use the method isPreInfinity on this class.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaInfType ()`  
  Destructor.
- `MFnKeyframeDeltaInfType ()`  
  Default constructor.
- `MFnKeyframeDeltaInfType (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnAnimCurve::InfinityType previousInfinityType (MStatus *ReturnStatus=NULL) const`  
  The previous infinity type.
- `MFnAnimCurve::InfinityType currentInfinityType (MStatus *ReturnStatus=NULL) const`  
  The current infinity type.
- `bool isPreInfinity (MStatus *ReturnStatus=NULL) const`  
  This class can describe changes to both the pre-infinity and post-infinity this method allows the API user to figure out which type this class represents.
- `MFnKeyframeDeltaInfType (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaMove

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_move.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_move.html).

Function set for change in keyframe value or time. This function set is used for keyframe deltas of type MFn::kKeyframeDeltaMove. These keyframe deltas are generated from 'move' events. For example, the user drags or scales a key from one position to another.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaMove ()`  
  Destructor.
- `MFnKeyframeDeltaMove ()`  
  Default constructor.
- `MFnKeyframeDeltaMove (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MTime previousTime (MStatus *ReturnStatus=NULL) const`  
  The previous time value of this key.
- `MTime currentTime (MStatus *ReturnStatus=NULL) const`  
  The current/current time value.
- `double previousValue (MStatus *ReturnStatus=NULL) const`  
  The previous value of the key prior to the change.
- `double currentValue (MStatus *ReturnStatus=NULL) const`  
  The current value of the key.
- `unsigned int previousIndex (MStatus *ReturnStatus=NULL) const`  
  The previous index value of this key.
- `unsigned int keyIndex (MStatus *ReturnStatus=NULL) const override`  
  The current index value of this key.
- `MFnKeyframeDeltaMove (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaScale

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_scale.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_scale.html).

This method is obsolete. [as of Maya 2018] Deprecated:Not used. Maya instead generates MFnKeyframeDeltaMove and MFnKeyframeDeltaTangent events when a group of keys is scaled.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaScale ()`  
  Destructor.
- `MFnKeyframeDeltaScale ()`  
  Default constructor.
- `MFnKeyframeDeltaScale (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MTime startTime (MStatus *ReturnStatus=NULL) const`  
  The start time of the scaling block.
- `MTime endTime (MStatus *ReturnStatus=NULL) const`  
  The end time of the scaling block.
- `MTime currentStartTime (MStatus *ReturnStatus=NULL) const`  
  The current scale time (after scaling has been performed).
- `MTime currentEndTime (MStatus *ReturnStatus=NULL) const`  
  The current end time.
- `MTime pivotTime (MStatus *ReturnStatus=NULL) const`  
  The pivot point of the scale (in time).
- `MFnKeyframeDeltaScale (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaTangent

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_tangent.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_tangent.html).

Function set for changes to a key's tangent. A MFnKeyframeDeltaTangent function allows API programmers to read changes in keyframe tangent values. It captures changes in tangent type, as well as, changes in tangent time/value pair. MFnKeyframeDeltaTangents are generated by a MAnimMessage::addAnimKeyframeEditedCallback. Because a key's tangent may be unbroken. It is possible to receive to MFnKeyframeDeltaTangent values per key, one for the incoming tangent and one for the outgoing tangent. The API programmer should use the isInTangent method to determine if the tangent change affects the in-bound tangent or out-bound tangent.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaTangent ()`  
  Destructor.
- `MFnKeyframeDeltaTangent ()`  
  Default constructor.
- `MFnKeyframeDeltaTangent (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnAnimCurve::TangentType previousTangentType (MStatus *ReturnStatus=NULL) const`  
  Returns the previous tangent type.
- `MFnAnimCurve::TangentType currentTangentType (MStatus *ReturnStatus=NULL) const`  
  Returns the current tangent type that the key represents.
- `MStatus getPreviousPosition (MFnAnimCurve::TangentValue &x, MFnAnimCurve::TangentValue &y) const`  
  Get the values of the previous time/value position of the tangent for this key.
- `MStatus getCurrentPosition (MFnAnimCurve::TangentValue &x, MFnAnimCurve::TangentValue &y) const`  
  Get the values of the current time/value position of the tangent for this key.
- `bool isInTangent (MStatus *ReturnStatus=NULL) const`  
  Key's have two tangents, in-bound and out-bound.
- `MFnKeyframeDeltaTangent (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnKeyframeDeltaWeighted

Module `OpenMayaAnim`. Inherits `MFnKeyframeDelta`. Source: [class_m_fn_keyframe_delta_weighted.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_keyframe_delta_weighted.html).

Function set for changes in a key's weighted state. Delta keyframe object for changes in weighted state of a key.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnKeyframeDeltaWeighted ()`  
  Destructor.
- `MFnKeyframeDeltaWeighted ()`  
  Default constructor.
- `MFnKeyframeDeltaWeighted (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `bool wasWeighted (MStatus *ReturnStatus=NULL) const`  
  Returns true if the key had weighted tangent, but it is not currently.
- `MFnKeyframeDeltaWeighted (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnLattice

Module `OpenMayaAnim`. Inherits `MFnDagNode`. Source: [class_m_fn_lattice.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lattice.html).

Lattice function set. MFnLattice is the function set for lattice shapes and lattice geometry. It can be used on lattices in the DAG or on lattice geometry from a dependency node attribute. Lattices are most commonly used for specifying FFDs (free-form deformations). See MFnLatticeDeformer for more information on those. MFnLatticeData can be used to create new lattice data objects for use with dependency node attributes.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnLattice ()`  
  Destructor.
- `MFnLattice ()`  
  Default constructor.
- `MFnLattice (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnLattice (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (unsigned int xDiv, unsigned int yDiv, unsigned int zDiv, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Create a new lattice.
- `MStatus reset (double sSize=1.0, double tSize=1.0, double uSize=1.0)`  
  Reset the lattice points to a uniform parallelipiped shape with the specified dimensions: sSize x tSize x uSize.
- `MStatus getDivisions (unsigned int &s, unsigned int &t, unsigned int &u)`  
  Get the number of divisions in the lattice.
- `MStatus setDivisions (unsigned int s, unsigned int t, unsigned int u)`  
  Set the number of divisions in the lattice.
- `MPoint & point (unsigned int s, unsigned int t, unsigned int u, MStatus *ReturnStatus=NULL)`  
  Returns the point in the lattice that is at the given indices.
- `MFnLattice (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnLatticeDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_lattice_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lattice_deformer.html).

FFD lattice deformer function set. MFnLatticeDeformer is the function set for lattice deformers. Lattice deformers use FFDs to deform geometry. The lattice deformer is actually a small network of dependency nodes in the dependency graph. This function set is provided to make manipulation of the network easier. The main deformer node should be given to this function set as its object. There are two lattices associated with a lattice deformer. There is a base lattice that defines the start position for the lattice. The second lattice is the version that is modified to deform the geometry. The difference between the two lattices defines the deformation that is applied to the geometry. The base lattice is a very simple shape that only defines a box in space. The base lattice should be modified by using the standard DAG transformation support. The second deformable lattice has geometry that can be modified using the MFnLattice function set. For a piece of geometry to be modified by this deformer, the geometry must be attached to the deformer (use addGeometry method) and the geometry must be contained within the base lattice. The resetLattice method can be used to make the lattice resize to the bounding box of the attached geometry.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnLatticeDeformer ()`  
  Destructor.
- `MFnLatticeDeformer ()`  
  Default constructor.
- `MFnLatticeDeformer (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create (unsigned int xDiv, unsigned int yDiv, unsigned int zDiv, MStatus *ReturnStatus=NULL)`  
  Creates a new lattice deformer with the given number of divisions.
- `MStatus addGeometry (const MObject &object)`  
  Adds a piece of geometry to the deformation.
- `MStatus removeGeometry (const MObject &object)`  
  Removes a piece of geometry from the deformation.
- `MStatus getAffectedGeometry (MObjectArray &objects)`  
  The geometry affected by this deformer is packed into the provided list of MObjects.
- `MStatus getDivisions (unsigned int &x, unsigned int &y, unsigned int &z)`  
  Retrieve the number of divisions in each of the X, Y, and Z directions.
- `MStatus setDivisions (unsigned int x, unsigned int y, unsigned int z)`  
  Set the number of divisions in each of the X, Y, and Z directions.
- `MStatus resetLattice (bool centerLattice=false)`  
  This method resets the deformed lattice to match the base lattice.
- `MObject deformLattice (MStatus *ReturnStatus=NULL)`  
  This returns the deformed version of the lattice.
- `MObject baseLattice (MStatus *ReturnStatus=NULL)`  
  This returns the base version of the lattice that describes the region of space deformed by the lattice.
- `MFnLatticeDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnMotionPath

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_motion_path.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_motion_path.html).

Motion path animation function set. This class is used for constructing and manipulating motion path animation. Motion path animation requires a curve (or surface) and one or more other objects. During the animation, the objects will be moved along the curve. Setting "follow" for the motion path aligns the object(s) local axis to the tangent of the motion path. Banking can also be applied to objects. Motion path markers are points along the path where the orientation and position for the object(s) travelling along the path can be specified.

**Public Types**

- `enum Axis { kXaxis, kYaxis, kZaxis }`  
  Available axes.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnMotionPath ()`  
  Destructor.
- `MFnMotionPath ()`  
  Default constructor.
- `MFnMotionPath (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create (const MDagPath &pathObject, const MDagPath &objectToAnimate, MTime &timeStart, MTime &timeEnd, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`  
  Create a new motion path dependency node.
- `MStatus setPathObject (const MDagPath &pathObject, MDGModifier *modifier=NULL)`  
  Set the curve or surface for this motion path.
- `MDagPath pathObject (MStatus *ReturnStatus=NULL)`  
  Return a dag path to the motion path object.
- `MStatus addAnimatedObject (const MDagPath &objectToAnimate, MDGModifier *modifier=NULL)`  
  Add an object to be animated along this motion path.
- `MStatus getAnimatedObjects (MDagPathArray &array)`  
  Returns an array of dag paths to the animated objects for this motion path.
- `MStatus setFollow (bool on, MDGModifier *modifier=NULL)`  
  Setting follow on will cause the animated object(s) local axis to be aligned with the tangent of the motion path.
- `bool follow (MStatus *ReturnStatus=NULL) const`  
  Determines whether follow is set for this motion path node.
- `MStatus setFollowAxis (Axis axis)`  
  Sets the axis of the animated object that will follow the motion path.
- `Axis followAxis (MStatus *ReturnStatus=NULL)`  
  Return the follow axis for this motion path.
- `MStatus setUpAxis (Axis axis)`  
  Set the up-axis for this motion path.
- `Axis upAxis (MStatus *ReturnStatus=NULL)`  
  Return the up-axis for this motion path.
- `MStatus setBank (bool bank)`
- `bool bank (MStatus *ReturnStatus=NULL) const`  
  Determines whether bank has been enabled for this motion path.
- `MStatus setBankScale (double bankScale)`  
  Set the bank scale for this motion path.
- `double bankScale (MStatus *ReturnStatus=NULL)`  
  Return the bank scale for this motion path.
- `MStatus setBankThreshold (double bankThreshold)`  
  Set the bank threshold for this motion path.
- `double bankThreshold (MStatus *ReturnStatus=NULL)`  
  Return the bank threshold for this motion path.
- `MStatus setUseNormal (bool use)`  
  If true, enables alignment of the up axis of the animated object to the normal vector of the path geometry.
- `bool useNormal (MStatus *ReturnStatus=NULL)`  
  Determines whether the up-axis of the animated object for this motion path is aligned with the normal vector of the path geometry.
- `MStatus setInverseNormal (bool invert)`  
  If true, enable alignment of the up axis of the moving object(s) to the opposite direction of the normal vector of the path geometry.
- `bool inverseNormal (MStatus *ReturnStatus=NULL)`  
  Determines whether the up-axis of the animated object for this motion path is aligned to the opposite direction of the normal vector of the path geometry.
- `MStatus setUStart (double start)`  
  Sets the starting value of the u parameterization for the animation.
- `MStatus setUEnd (double end)`  
  Sets the end value of the u parameterization for the animation.
- `double uStart (MStatus *ReturnStatus=NULL)`  
  Returns the starting value of the u parameterization for the animation.
- `double uEnd (MStatus *ReturnStatus=NULL)`  
  Returns the end value of the u parameterization for the animation.
- `MStatus setUTimeStart (MTime &start)`  
  Sets the starting time of the animation for the u parameter.
- `MStatus setUTimeEnd (MTime &end)`  
  Sets the end time of the animation for the u parameter.
- `MTime uTimeStart (MStatus *ReturnStatus=NULL)`  
  Returns the start time of the animation for the u parameter.
- `MTime uTimeEnd (MStatus *ReturnStatus=NULL)`  
  Returns the end time of the animation for the u parameter.
- `unsigned int numPositionMarkers (MStatus *ReturnStatus=NULL)`  
  Returns the number of position markers on this motion path.
- `MObject getPositionMarker (unsigned int, MStatus *ReturnStatus=NULL)`  
  Gets the position marker where markerNum is the order in which the marker was created.
- `unsigned int numOrientationMarkers (MStatus *ReturnStatus=NULL)`  
  Returns the number of orientation markers on this motion path.
- `MObject getOrientationMarker (unsigned int, MStatus *ReturnStatus=NULL)`  
  Gets the orientation marker where markerNum is the order in which the marker was created.
- `MFnMotionPath (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnSkinCluster

Module `OpenMayaAnim`. Inherits `MFnGeometryFilter`. Source: [class_m_fn_skin_cluster.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_skin_cluster.html).

skinCluster function set MFnSkinCluster is the function set for skinClusters. SkinCluster nodes are created during a smooth bindSkin. The purpose of the skinCluster is to store a weight per influence object for each component of each geometry that is deformed. Influence objects can be joints or any transform. Note that unlike most deformers, a skinCluster node can deform only a single geometry. Therefore, if additional geometries are added to the skinCluster set, they will be ignored. This function set provides methods for getting and setting weights on skin cluster nodes. It parent class, MFnGeometryFilter provides methods for accessing the skin cluster's input and output geometry.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnSkinCluster ()`  
  Destructor.
- `MFnSkinCluster ()`  
  Default constructor.
- `MFnSkinCluster (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MStatus getWeights (const MDagPath &path, const MObject &components, unsigned int influenceIndex, MDoubleArray &weights) const`  
  Gets the skinCluster weights for the influence object for the specified components of the object whose dagPath is specified.
- `MStatus getWeights (const MDagPath &path, const MObject &components, MDoubleArray &weights, unsigned int &influenceCount) const`  
  Gets the skinCluster weights for all influenceObjects for the specified components of the object whose dagPath is specified.
- `MStatus getWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MDoubleArray &weights) const`  
  Gets the skinCluster weights for the influence objects for the specified components of the object whose dagPath is specified.
- `MStatus setWeights (const MDagPath &path, const MObject &components, unsigned int jointIndex, double value, bool normalize=true, MDoubleArray *oldValues=NULL)`  
  Sets the skinCluster weight for the influence object on the specified components of the object whose dagPath is specified.
- `MStatus setWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MDoubleArray &values, bool normalize=true, MDoubleArray *oldValues=NULL)`  
  Sets the skinCluster weight on the specified components of the object whose dagPath is specified for the influence objects corresponding to the specified influence indices.
- `MStatus getPointsAffectedByInfluence (const MDagPath &path, MSelectionList &result, MDoubleArray &weights)`  
  During deformation, the skinCluster algorithm is applied for a given influence object on all points in the deformer's set whose weights are non-zero.
- `unsigned int indexForInfluenceObject (const MDagPath &path, MStatus *ReturnStatus=NULL) const`  
  Returns the logical index of the matrix array attribute where the specified influence object is attached.
- `unsigned int influenceObjects (MDagPathArray &paths, MStatus *ReturnStatus=NULL) const`  
  Returns an array of paths to the influence objects for the skinCluster.
- `MStatus getWeights (const MDagPath &path, const MObject &components, unsigned int influenceIndex, MFloatArray &weights) const`  
  This method is obsolete.
- `MStatus getWeights (const MDagPath &path, const MObject &components, MFloatArray &weights, unsigned int &influenceCount) const`  
  This method is obsolete.
- `MStatus setWeights (const MDagPath &path, const MObject &components, unsigned int jointIndex, float value, bool normalize=true, MFloatArray *oldValues=NULL)`  
  This method is obsolete.
- `MStatus setWeights (const MDagPath &path, const MObject &components, MIntArray &influenceIndices, MFloatArray &values, bool normalize=true, MFloatArray *oldValues=NULL)`  
  This method is obsolete.
- `MStatus getPointsAffectedByInfluence (const MDagPath &path, MSelectionList &result, MFloatArray &weights)`  
  This method is obsolete.
- `MStatus getBlendWeights (const MDagPath &path, const MObject &components, MDoubleArray &weights) const`  
  This method returns weights from skinCluster's blend weight array.
- `MStatus setBlendWeights (const MDagPath &path, const MObject &components, const MDoubleArray &weights)`  
  This method sets weights in skinCluster's blend weight array.
- `MFnSkinCluster (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnWeightGeometryFilter

Module `OpenMayaAnim`. Inherits `MFnGeometryFilter`. Source: [class_m_fn_weight_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_weight_geometry_filter.html).

Weight geometry filter function set. MFnWeightGeometryFilter is the function set for weight geometry filters. Weight geometry filter nodes include clusters, cluster flexors, and user-defined deformers derived from MPxDeformerNode. The purpose of the weight geometry filter is to store the weights for each component of each geometry that is deformed. The weight geometry filter is independent of any algorithm that calculates a deformation based on the component weight values. Clusters, cluster flexors, and user-defined deformers each have their own algorithm to determine the deformation based on the component weights. This function set provides methods for getting and setting weights on weight geometry filter nodes. In addition to getting and settings weights on components, this class provides methods for accessing the weight geometry filter's input and output geometry through its parent class, MFnGeometryFilter. When getting and setting weights on components, it is more efficient to use the methods that take a plug index. The plug index is a sparse array index, and is the same index used in MEL scripts to index into an array of plugs.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnWeightGeometryFilter ()`  
  Destructor.
- `MFnWeightGeometryFilter ()`  
  Default constructor.
- `MFnWeightGeometryFilter (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MStatus getWeights (unsigned int index, const MObject &components, MFloatArray &weights) const`  
  Gets the weights of the components that correspond to the geometry at the specified plug index.
- `MStatus getWeights (const MDagPath &path, const MObject &components, MFloatArray &weights) const`  
  Gets the weights of the components that correspond to the geometry whose DAG path is specified.
- `MStatus setWeight (const MDagPath &path, unsigned int index, const MObject &components, float weight, MFloatArray *oldValues=NULL)`  
  Sets the weights of the specified components of the object whose DAG path is specified.
- `MStatus setWeight (const MDagPath &path, const MObject &components, float weight, MFloatArray *oldValues=NULL)`  
  Sets the weights of the specified components of the object whose DAG path is specified.
- `MStatus setWeight (const MDagPath &path, unsigned int index, const MObject &components, MFloatArray &values)`  
  Sets the weights of the specified components of the object whose DAG path is specified.
- `MStatus setWeight (const MDagPath &path, const MObject &components, MFloatArray &values)`  
  Sets the weights of the specified components of the object whose DAG path is specified.
- `MString weightPlugStrings (const MSelectionList &list, MStatus *ReturnStatus=NULL) const`  
  Sets the plugStrings argument to be a string (separated by spaces) containing the names of the plugs on this node that correspond to the components in the selection list.
- `MStatus getWeightPlugStrings (const MSelectionList &list, MStringArray &plugStringArray) const`  
  Set the plugStringArray argument to contain the names of the plugs on this node that correspond to the components in the selection list.
- `MFnWeightGeometryFilter (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MStatus getWeightPlugStrings (const MSelectionList &list, MString &plugStrings) const`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnWireDeformer

Module `OpenMayaAnim`. Inherits `MFnDependencyNode`. Source: [class_m_fn_wire_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_wire_deformer.html).

wire deformer function set MFnWireDeformer is the function set for wire deformers. Wire deformers modify geometry based on its proximity to controlling wire curves. As the wire curves are modified, the parts of the geometry close to the curve will follow. The wire deformer is actually a small network of dependency nodes in the dependency graph. This function set is provided to make manipulation of the network easier. The main deformer node should be given to this function set as its object.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnWireDeformer ()`  
  Destructor.
- `MFnWireDeformer ()`  
  Default constructor.
- `MFnWireDeformer (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new wire deformer.
- `MStatus addGeometry (const MObject &object)`  
  Adds a piece of geometry to the deformation.
- `MStatus removeGeometry (const MObject &object)`  
  Removes a piece of geometry from the deformation.
- `MStatus getAffectedGeometry (MObjectArray &objects)`  
  The geometry affected by this deformer is packed into the provided list of MObjects.
- `unsigned int numWires (MStatus *ReturnStatus=NULL) const`  
  returns the number of wire curves connected to this deformer.
- `MStatus addWire (const MObject &object)`  
  Adds a new wire curve to the deformation.
- `MObject wire (unsigned int wireIndex, MStatus *ReturnStatus=NULL)`  
  Return the wire at the given index.
- `float wireDropOffDistance (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`  
  Return the drop off distance of the wire at the given index.
- `MStatus setWireDropOffDistance (unsigned int wireIndex, float dropOff)`  
  Sets the drop off distance of the wire at the given index.
- `float wireScale (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`  
  Return the radial scale of the wire at the given index.
- `MStatus setWireScale (unsigned int wireIndex, float scale)`  
  Sets the radial scale value of the wire at the given index.
- `MObject holdingShape (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`  
  Returns the holding shape for the given wire.
- `MStatus setHoldingShape (unsigned int wireIndex, MObject holdingCurve)`  
  Sets the holding shape for the given wire.
- `float envelope (MStatus *ReturnStatus=NULL) const`  
  Returns the envelope for this deformer.
- `MStatus setEnvelope (float envelope)`  
  Sets the envelope for this deformer.
- `float rotation (MStatus *ReturnStatus=NULL) const`  
  Returns the rotation value for this deformer.
- `MStatus setRotation (float rotation)`  
  Sets the rotation value for this deformer.
- `float localIntensity (MStatus *ReturnStatus=NULL) const`  
  Returns the local intensity for this wire deformer.
- `MStatus setLocalIntensity (float localIntensity)`  
  Sets the local intensity for this wire deformer.
- `float crossingEffect (MStatus *ReturnStatus=NULL) const`  
  Returns the crossing effect for this wire deformer.
- `MStatus setCrossingEffect (float crossingEffect)`  
  Sets the crossing effect for this wire deformer.
- `unsigned int numDropoffLocators (unsigned int wireIndex, MStatus *ReturnStatus=NULL) const`  
  Returns the number of drop off locators.
- `MStatus setDropoffLocator (unsigned int wireIndex, unsigned int locatorIndex, float param, float percentage)`  
  Sets the parameters of a drop off locator.
- `MStatus getDropoffLocator (unsigned int wireIndex, unsigned int locatorIndex, float &param, float &percentage)`  
  Gets the parameters of a drop off locator.
- `MFnWireDeformer (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MGPUDeformerBuffer

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_buffer.html).

Store GPU Representation of a data buffer. MGPUDeformerBuffer is a simple buffer structure to group data related to a single GPU buffer which is associated with a plug in the DG.

**Public Member Functions**

- `MGPUDeformerBuffer (const MPlug &plug, const MUniqueString &name, bool readOnly, unsigned int elementSizeInBytes, unsigned int elementCount, unsigned int capacityInBytes, const MAutoCLMem &buffer, const MAutoCLEvent &event)`  
  Constructs an MGPUDeformerBuffer for an existing MAutoCLMem.
- `MGPUDeformerBuffer ()`  
  Constructs an empty MGPUDeformerBuffer which cannot be used.
- `~MGPUDeformerBuffer ()`  
  Destructor.
- `MGPUDeformerBuffer (const MGPUDeformerBuffer &other)`  
  Copy constructor.
- `MGPUDeformerBuffer (MGPUDeformerBuffer &&other)`  
  Move Constructor.
- `MGPUDeformerBuffer & operator= (const MGPUDeformerBuffer &other)`  
  Copy Assignment Operator.
- `MGPUDeformerBuffer & operator= (MGPUDeformerBuffer &&other)`  
  Move assignment operator.
- `MOpenCLBuffer createMOpenCLBuffer ()`  
  Create and return an MOpenCLBuffer backed by the same GPU buffer that this MGPUDeformerBuffer is backed by.
- `bool isValid () const`
- `const MUniqueString name () const`  
  It is only safe to call name() when MGPUDeformerBuffer is valid.
- `const MPlug plug () const`  
  It is only safe to call name() when MGPUDeformerBuffer is valid.
- `bool isReadOnly () const`  
  It is only safe to call isReadOnly() when MGPUDeformerBuffer is valid.
- `unsigned int elementSizeInBytes () const`  
  It is only safe to call elementSizeInBytes() when MGPUDeformerBuffer is valid.
- `unsigned int elementCount () const`  
  It is only safe to call elementCount() when MGPUDeformerBuffer is valid.
- `const MAutoCLMem buffer () const`  
  It is only safe to call buffer() when MGPUDeformerBuffer is valid.
- `const MAutoCLEvent bufferReadyEvent () const`  
  It is only safe to call bufferReadyEvent() when MGPUDeformerBuffer is valid.
- `void setBufferReadyEvent (const MAutoCLEvent &)`  
  It is only safe to call setBufferReadyEvent() when MGPUDeformerBuffer is valid.
- `void destroy ()`  
  Release all resources and references held by this MGPUDeformerBuffer.


### MGPUDeformerData

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_data.html).

Store GPU Representation of geometry data. MGPUDeformerData is the GPU representation of geometry data used in MPxGPUDeformer. MGPUDeformerData::setBuffer() stores an MGPUDeformerBuffer object. getBuffer() can be used to retrieve this buffer by name & plug. Data stored in MGPUDeformerData is not owned by MGPUDeformerData, ownership remains with the allocator of the data and deleting an MGPUDeformerData does not automatically delete any associated MGPUDeformerBuffer objects.

**Public Member Functions**

- `virtual void setBuffer (const MGPUDeformerBuffer &)`  
  Store an MGPUDeformerBuffer.
- `virtual MGPUDeformerBuffer getBuffer (const MUniqueString &name) const`  
  Retrieve the MGPUDeformerBuffer with the given name.
- `virtual MGPUDeformerBuffer getBuffer (const MUniqueString &name, const MPlug &plug) const`  
  Retrieve the MGPUDeformerBuffer with the given name and plug.
- `MGPUDeformerData (void *impl)`  
  Constructs an MGPUDeformerData from an existing implementation object.
- `MGPUDeformerData (const MGPUDeformerData &other)`  
  Copy constructs an MGPUDeformerData.
- `virtual ~MGPUDeformerData ()`  
  Destructor.


### MGPUDeformerRegistrationInfo

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_registration_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_registration_info.html).

Registration information for an MPxGPUDeformer. MGPUDeformerRegistrationInfo provides a creator method to allocate MPxGPUDeformer objects, validation methods to determine if a given node is supported by the MPxGPUDeformer, as well as utility methods to tell the deformer evaluator which attributes are the input and output mesh attributes of the deformer node. You must implement createGPUDeformer(), validateNodeInGraph() and validateNodeValues(). outputMeshAttribute(), inputMeshAttributes() and passThroughInputMeshAttribute have default implementations which will work correctly if the node being overridden by MPxGPUDeformer is an MPxDeformerNode. If the attribute returned by outputMeshAttribute() is a multi-attribute or a multi parent, then the attribute returned by inputMeshAttribute() must also be a multi-attribute or have a multi parent. The total number of multi attributes in the hierarchy of the output attribute (including the output attribute itself) must be exactly one. The total number of multi attributes in the hierarchy of the input attribute (including the input attribute itself) must be greater than or equal to the number of multi attributes in the hierarchy of the output attribute. For example, geometryFilter's input geometry attribute is inputGeometry. inputGeometry is not a multi attribute, but it has a single multi parent (input). geometryFilter's output geometry attribute is outputGeometry. outputGeometry is a multi attribute. In this case the input and output attribute hierarchies each have a single multi, so the GPU override works. Maya will match the multi index of each multi attribute in the input and output multi hierarchies for each connected input and output plug, starting from the root plug. In the geometryFilter example we use the multi index of the outputGeometry attribute as the multi index of aInput when attempting to determine if inputGeometry is connected.

**Public Member Functions**

- `virtual MPxGPUDeformer * createGPUDeformer ()=0`  
  This method allocates an MPxGPUDeformer.
- `virtual bool validateNodeInGraph (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &plug, MStringArray *messages)=0`  
  This method is called once for each instance of the overridden deformation node type in a Maya scene.
- `virtual bool validateNodeValues (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &plug, MStringArray *messages)=0`  
  This method is called once for each instance of the overridden deformation node type in a Maya scene.
- `virtual const MObject outputMeshAttribute ()`  
  This method returns the output mesh attribute for the node type being overridden.
- `virtual const MObject inputMeshAttribute ()`  
  This method is depreciated, please use inputMeshAttributes instead.
- `virtual void inputMeshAttributes (MObjectArray &inputAttributes)`  
  This method fills inputAttributes with attributes which should be searched for upstream deformation supported by GPU Override.
- `virtual bool matchAncestorLogicalIndices (const MPlug &original, MPlug &target)`  
  This method sets ancestor logical indices of a deformer input plug inputPlug given a specific deformer output plug outputPlug.
- `virtual const MObject passThroughInputMeshAttribute ()`  
  This method returns the input mesh attribute for the node type being overridden which has a special "pass through" relationship with the output mesh attribute.


### MGPUDeformerRegistry

Module `OpenMayaAnim`. Source: [class_m_g_p_u_deformer_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_deformer_registry.html).

Registration class for MPxGPUDeformer. MGPUDeformerRegistry allows each class derived from MPxGPUDeformer to register the type of deformer it supports. Only one override registered at a time for each type of deformer node. Overrides provided by default are registered with registrantId "Maya".

**Static Public Member Functions**

- `static MStatus registerGPUDeformerCreator (const MString &nodeClassName, const MString &registrantId, MGPUDeformerRegistrationInfo *registrationInfo)`  
  This method is called to register an MPxGPUDeformer that overrides a specific type of deformer node.
- `static MStatus deregisterGPUDeformerCreator (const MString &nodeClassName, const MString &registrantId)`  
  This method is called to deregister an MPxGPUDeformer that overrides a specific type of deformer node.
- `static MStatus addConditionalAttribute (const MString &nodeClassName, const MString &registrantId, const MObject &conditionalAttribute)`  
  This method is called to add a conditional attribute to an already registered GPU Deformer.


### MGPUEventList

Module `OpenMayaAnim`. Source: [class_m_g_p_u_event_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_p_u_event_list.html).

Helper class to keep track of CL events. MGPUEventList is a simple class to make it easier to manage an event wait list for kernels that are queued up.

**Public Member Functions**

- `MGPUEventList ()`  
  Constructs a MGPUEventList.
- `MGPUEventList (const MGPUEventList &)`  
  Constructs a MGPUEventList.
- `~MGPUEventList ()`  
  Destructor.
- `void add (const MAutoCLEvent &event)`  
  This method adds a new event to the list.
- `void add (MOpenCLBuffer &buf)`  
  This method adds the bufferReadyEvent of a buffer to the list.
- `const cl_event * array () const`  
  This method returns the array of events contained in the MGPUEventList.
- `cl_uint size () const`  
  This method returns the number of events contained in the MGPUEventList.


### MIkHandleGroup

Module `OpenMayaAnim`. Source: [class_m_ik_handle_group.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ik_handle_group.html).

IK handle groups. Group class for ik handles. Each group has an associated solver and priority. A single chain solver handle group has only one handle in its group.

**Public Member Functions**

- `MIkHandleGroup ()`  
  Default constructor.
- `~MIkHandleGroup ()`  
  The class destructor.
- `int priority (MStatus *ReturnStatus=NULL) const`  
  Return the priority value of this handle group.
- `int solverID (MStatus *ReturnStatus=NULL) const`  
  Return the solver id used by this handle group.
- `int solverPriority (MStatus *ReturnStatus=NULL) const`  
  return the priority of the solver used by this handle group.
- `MStatus setPriority (int)`  
  Set the priority of this handle group.
- `MStatus setSolverID (int)`  
  Set the solver id for this handle group.
- `bool checkEffectorAtGoal (MStatus *ReturnStatus=NULL)`  
  Determines whether the end-effector at the handle(goal) location.
- `MStatus solve ()`  
  Do all ik solving steps for this group.
- `int dofCount (MStatus *ReturnStatus=NULL) const`  
  Return the total number of degrees of freedom of this handle group.
- `int handleCount (MStatus *ReturnStatus=NULL) const`  
  Return the number of handles in the handle list for this group.
- `MObject handle (int ith, MStatus *ReturnStatus=NULL)`  
  Return the ith handle in the handle list for this group.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Friends**

- `class MPxIkSolver`
- `class MPxIkSolverNode`


### MIkSystem

Module `OpenMayaAnim`. Source: [class_m_ik_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ik_system.html).

Inverse kinematics (IK) system class. This class provides an interface to the inverse kinematics (IK) system. The ik system is used to set/query the global snapping flag for handles, set/query the global solve flag for solvers, and to find the ik solvers available in maya.

**Static Public Member Functions**

- `static MObject findSolver (MString name, MStatus *ReturnStatus=NULL)`  
  Returns the ik solver with the given name.
- `static MStatus getSolvers (MStringArray &names)`  
  Get a list of the names for the solvers that are available in the system.
- `static bool isGlobalSnap (MStatus *ReturnStatus=NULL)`  
  Determines whether global snapping is on.
- `static MStatus setGlobalSnap (bool isSnap)`  
  Turns global snapping on or off.
- `static bool isGlobalSolve (MStatus *ReturnStatus=NULL)`  
  Determines whether global solving is on.
- `static MStatus setGlobalSolve (bool isSnap)`  
  Turns global solving on or off.
- `static const char * className ()`  
  Returns the name of this class.


### MIndexMapper

Module `OpenMayaAnim`. Source: [class_m_index_mapper.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_index_mapper.html).

Index mapper containing indices of a subset of a geometry. An MIndexMapper maps indices between the full set of a geometry and the subset of indices that is affected by a deformer. If a deformer affects f.e only 1000 verts of a total of 15000 verts then the affectMap will be of size 1000 and contain the indices into the full table of points.

**Public Member Functions**

- `MIndexMapper ()`  
  Constructs a MIndexMapper.
- `~MIndexMapper ()`  
  Destructor.
- `MIndexMapper (const MIndexMapper &)`  
  Constructs a MIndexMapper.
- `Index fullCount () const`  
  This method returns the total number of points of the geometry.
- `Index affectCount () const`  
  This method returns the number of points in a subset of the geometry.
- `Index affectToFull (Index aid) const`  
  This method remaps the affect index of a subset to the full index of the geometry.


### MItKeyframe

Module `OpenMayaAnim`. Source: [class_m_it_keyframe.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_keyframe.html).

Keyframe Iterator. Iterate over the keyframes of a particular Anim Curve Node, and query and edit the keyframe to which the iterator points. Determine the time and value of the keyframe, as well as the x,y values and type of the tangent to the curve entering (in tangent) and leaving (out tangent) the keyframe. Set the time and value of the keyframe, and the type of the tangents. Anim Curves are implemented as Dependency Graph (DG) Nodes. Each Node has multiple ordered and indexed keyframes. Use the Keyframe Iterator to systematically visit, and query and/or edit the keyframes of a Anim Curve Node. Use the Keyframe Iterator in conjunction with the Anim Curve Function Set (MFnAnimCurve) to edit Anim Curve Nodes. On creation the iterator is attached to a Anim Curve Node. The iterator may be detached from its current Node and attached to another Anim Curve Node using the reset() method. Though keyframes within a Node are ordered and indexed, the iterator maintains the index of the current Node internally and does not export it. Use the Anim Curve Function Set to access the index for a keyframe. The index is zero-based. When an iterator is first attached to a Node, either on creation or reset, the index is set to zero. There is an overloaded version of the reset() method which does not cause the iterator to change Nodes, but merely resets the index. Use the next(), reset() and isDone() methods to perform iterations. Use the specific query methods to determine the time, value and tangent information for the keyframe under the iterator. Use specific edit methods to set the time and value , as well as the incoming and outgoing tangent type (MItKeyframe::TangentType) of the keyframe under the iterator. There is no method for setting the x,y value of either of the tangents. Setting the time of a keyframe will fail if the new time would require a re-ordering of the keyframes. Use the Anim Curve Functiion Set methods MFnAnimCurve::remove() and MFnAnimCurve::addKeyFrame() to re-order the keyframes.

**Public Types**

- `enum TangentType { kTangentGlobal = 0, kTangentFixed, kTangentLinear, kTangentFlat, kTangentSmooth, kTangentStep, kTangentSlow, kTangentFast, kTangentClamped, kTangentPlateau, kTangentStepNext, kTangentAuto, kTangentAutoMix, kTangentAutoEase, kTangentAutoCustom }`  
  Tangent types.
- `typedef double TangentValue`  
  Defines the value type of the tangent.

**Public Member Functions**

- `MItKeyframe (MObject &animCurveNode, MStatus *ReturnStatus=NULL)`  
  Class Constructor.
- `~MItKeyframe ()`  
  Class Destructor.
- `MStatus reset (MObject &animCurveNode)`  
  Detaches the iterator from the current Anim Curve Node and attaches it to the given Node.
- `MStatus reset ()`  
  Resets the keyframe index to 0 (first keyframe) on the Anim Curve Node to which the iterator is attached.
- `MStatus next ()`  
  Moves to the next keyframe on the Anim Curve Node to which the iterator is attached.
- `bool isDone (MStatus *ReturnStatus=NULL)`  
  Indicates that the iterator has moved beyond the last keyframe on the Anim Curve Node to which the iterator is attached.
- `MTime time (MStatus *ReturnStatus=NULL)`  
  Determines the time of the current keyframe.
- `MStatus setTime (MTime time)`  
  Sets the time of the current keyframe.
- `double value (MStatus *ReturnStatus=NULL)`  
  Determines the value of the current keyframe.
- `MStatus setValue (double value)`  
  Sets the value of the current keyframe.
- `TangentType inTangentType (MStatus *ReturnStatus=NULL)`  
  Determines the type of the tangent to the curve entering the current keyframe.
- `TangentType outTangentType (MStatus *ReturnStatus=NULL)`  
  Determines the type of the tangent to the curve leaving the current keyframe.
- `MStatus setInTangentType (TangentType tangentType)`  
  Sets the type of the tangent to the curve entering the current keyframe.
- `MStatus setOutTangentType (TangentType tangentType)`  
  Sets the type of the tangent to the curve entering the current keyframe.
- `MStatus getTangentOut (TangentValue &x, TangentValue &y)`  
  Determines the x,y value of the tangent to the curve leaving the current keyframe.
- `MStatus getTangentIn (TangentValue &x, TangentValue &y)`  
  Determines the x,y value of the tangent to the curve entering the current keyframe.
- `bool tangentsLocked (MStatus *ReturnStatus=NULL) const`  
  Determines whether the tangents are locked at this keyframe.
- `MStatus setTangentsLocked (bool locked)`  
  Lock or unlock the tangents at this keyframe.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxBlendShape

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_blend_shape.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_blend_shape.html).

Base class for user-defined blendshape deformers. MPxBlendShape allows the creation of user-defined blendshape deformers. It derives from MPxGeometryFilter and so offers all the functionality of that class. Additionally, it has the input target and all other attributes of the Maya built-in blendShape node. Custom nodes derived from MPxBlendShape are treated by Maya just like the built-in blendShape, so all the weight painting/editing etc. tools that artists are used to also work on the custom nodes.

**Public Member Functions**

- `MPxBlendShape ()`  
  Class constructor.
- `~MPxBlendShape () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual MStatus deformData (MDataBlock &block, MDataHandle geomData, unsigned int groupId, const MMatrix &mat, unsigned int multiIndex)`  
  This method performs the deformation algorithm.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject weight`  
  weight attribute, multi
- `static MObject inputTarget`  
  inputTarget attribute, multi
- `static MObject inputTargetGroup`  
  inputTargetGroup attribute, multi
- `static MObject inputTargetItem`  
  inputTargetItem attribute, multi
- `static MObject inputGeomTarget`  
  inputGeomTarget attribute
- `static MObject inputPointsTarget`  
  inputPointsTarget attribute
- `static MObject inputComponentsTarget`  
  inputComponentsTarget attribute
- `static MObject targetWeights`  
  targetWeights attribute, multi


### MPxConstraint

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_constraint.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_constraint.html).

Proxy constraint node. MPxConstraint is the parent class for user defined constraint nodes. Position, orientation or scale of an object can be constrained by other objects. This class works in conjunction with the MPxConstraintCommand class.

**Public Types**

- `enum UpVectorType { kScene = 0, kObject, kObjectRotation, kVector, kLast }`  
  The type of up vector.

**Public Member Functions**

- `MPxConstraint ()`  
  Constructor.
- `~MPxConstraint () override`  
  Destructor.
- `virtual const MObject weightAttribute () const`  
  Returns the weight attribute for the constraint.
- `virtual const MObject targetAttribute () const`  
  Returns the target attribute for the constraint.
- `virtual const MObject constraintRotateOrderAttribute () const`  
  Returns the rotate order attribute for the constraint.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject enableRestPosition`  
  Node attribute: enableRestPosition/erp - boolean.
- `static MObject lockOutput`  
  Node attribute: lockOutput/lo - boolean.

**Protected Member Functions**

- `virtual const MObject passiveOutputAttribute () const`  
  Returns the passive output attribute for the constraint.
- `virtual void getOutputAttributes (MObjectArray &attributeArray)`  
  Returns output attributes for the constraint.
- `MStatus closestPoint (const MObject &dataGeometryObject, const MPoint &toThisPoint, MPoint &theClosestPoint) const`  
  Returns the closest point on this surface to the given point.
- `MStatus closestNormal (const MObject &dataGeometryObject, const MPoint &toThisPoint, MVector &theNormal) const`  
  Returns the closest normal on this surface to the given point.
- `MStatus closestTangent (const MObject &dataGeometryObject, const MPoint &toThisPoint, MVector &theTangent) const`  
  Returns the closest tangent on this surface to the given point.
- `MEulerRotation computeAim (const MMatrix &parentInverseMatrix, const MVector &targetVector, const MVector &aimVector, const MVector &upVector, const MVector &wupVector, MEulerRotation::RotationOrder order=MEulerRotation::kXYZ, const MQuaternion &jointOrient=MQuaternion::identity, MStatus *ResultStatus=NULL) const`  
  Returns the rotation which aligns the aimVector in local space with the targetVector in local space while keeping the upVector in local space and the wUpVector world space aligned as closely as possible.
- `MVector worldUpVector (MPxConstraint::UpVectorType upType, const MVector &upVector, const MMatrix &upMatrix, const MPoint &constrPoint, MStatus *ResultStatus=NULL) const`  
  Returns the world up vector based on the requested up type.
- `MPoint worldConstraintPoint (const MMatrix &parentInverseMatrix, const MVector &translate, const MVector &rotatePivot, const MVector &rotatePivotTranslate, MStatus *ResultStatus=NULL) const`  
  Returns the world constraint point.


### MPxConstraintCommand

Module `OpenMayaAnim`. Inherits `MPxCommand`. Source: [class_m_px_constraint_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_constraint_command.html).

Proxy constraint command. MPxConstraintCommand is the base class for user defined commands which create constraints. This command gives all of the flags and options of the base constraint command and in addition allows user defined flags or behaviours. When registering this command, use the MFnPlugin::registerConstraintCommand() method. A MPxConstraint is also required to be used with MPxConstraintCommand. The constraintTypeId() virtual must be implemented to return the correct constraint node type id.

**Public Types**

- `enum TargetType { kTransform, kGeometryShape, kLast }`  
  Defines the type of target object.

**Public Member Functions**

- `MPxConstraintCommand ()`  
  Constructor.
- `~MPxConstraintCommand () override`  
  Destructor.
- `MStatus doIt (const MArgList &argList) override`  
  This virtual method is called when the command is intially executed (i.e.
- `MStatus undoIt () override`  
  This virtual method is called when the command is being undone.
- `MStatus redoIt () override`  
  This virtual method is called when the command is being redone.
- `virtual MStatus appendSyntax ()`  
  This method should be overridden to append syntax to the constraint command.
- `MPxConstraint * constraintNode ()`  
  Returns the custom MPxConstraint-derived node created by this instance of the command.

**Protected Member Functions**

- `virtual MStatus parseArgs (const MArgList &argList)`  
  This virtual method is called by the default behaviour of doIt() to parse the command's arguments.
- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`  
  USE _syntax() IN PYTHON.
- `virtual MStatus doEdit ()`  
  This virtual method is called by the default behaviour of doIt() if the command was executed in create or edit mode (i.e.
- `virtual MStatus doCreate ()`  
  This virtual method is called by the default behaviour of doIt() when the command is being executed in create mode.
- `virtual MStatus doQuery ()`  
  This virtual method is called by the default behaviour of doIt() when the command is being executed in query mode.
- `virtual void createdConstraint (MPxConstraint *constraint)`  
  This method is called when an MPxConstraintCommand creates a new MPxConstraint node.
- `virtual bool supportsOffset () const`  
  This method is used to control if the constraint supports offset.
- `virtual bool hasVectorFlags () const`  
  This method is used to control if the constraint supports the base class vector flags.
- `virtual MTypeId constraintTypeId () const`  
  This method is used to return the MTypeId of the MPxConstraint node that is used with this command.
- `virtual MPxConstraintCommand::TargetType targetType () const`  
  Maya supports constraints targets which are either transforms or nodes derived from "geometryShape".
- `virtual const MObject & aimVectorAttribute () const`  
  This method returns an attribute which defines the aim vector of a constraint.
- `virtual const MObject & upVectorAttribute () const`  
  This method returns an upVector attribute that is used in conjunction with the aimVector.
- `virtual const MObject & worldUpMatrixAttribute () const`  
  This method returns an worldUpMatrix attribute that is used in conjunction with the aimVector.
- `virtual const MObject & worldUpTypeAttribute () const`  
  This method returns an worldUpType attribute that is used in conjunction with the aimVector.
- `virtual const MObject & worldUpVectorAttribute () const`  
  This method returns an worldUpVector attribute that is used in conjunction with the aimVector.
- `virtual const MObject & offsetAttribute () const`  
  This method returns the offset attribute and must be implemented if supportsOffset() returns true.
- `virtual const MObject & constraintInstancedAttribute () const`  
  This method returns the attribute on the constraint node that connects to an instanced constraint attribute of the constrained object.
- `virtual const MObject & constraintOutputAttribute () const`  
  This method returns the attribute this constraint will connect to the constrained attribute of the constrained object.
- `virtual const MObject & constraintRestAttribute () const`  
  This method returns the attribute used to store the constraint's rest state.
- `virtual const MObject & constraintEnableRestAttribute () const`  
  This method returns the attribute used to enable and disable the rest state at runtime.
- `virtual const MObject & constraintTargetInstancedAttribute () const`  
  This method returns the constraintTargetInstanced attribute for the constraint.
- `virtual const MObject & constraintTargetAttribute () const`  
  This method returns the constraintTarget attribute for the constraint.
- `virtual const MObject & constraintTargetWeightAttribute () const`  
  This method returns the constraintTargetWeight attribute for the constraint.
- `virtual const MObject & objectAttribute () const`  
  This method returns the attribute this constraint will drive on the constrained object.
- `virtual void getObjectAttributesArray (MObjectArray &array)`  
  This method returns the list of attributes this particular constraint considers when inserting a pair blend.
- `virtual void handleNewTargets (MObject &dagObject)`  
  This method is obsolete.
- `virtual MStatus handleNewTargets (MDagPath &dagObject)`  
  This method is used to perform any special processing when targets are added to the constraint.
- `virtual MStatus connectTarget (void *opaqueTarget, int index)`  
  This method is obsolete.
- `virtual MStatus connectTarget (MDagPath &targetPath, int index)`  
  This method is called to make connections between the constraint and the target.
- `virtual MStatus connectObjectAndConstraint (MDGModifier &modifier)`  
  This method is used for connecting the constraint and constrained object.
- `virtual MStatus setRestPosition (MDGModifier &modifier)`  
  Override this method if you want to control the value to which the constraint node's rest position attribute (i.e.
- `MStatus connectGeometryAttribute (void *opaqueTarget, int index, MObject &constraintAttr)`  
  Utility method to make any required connections for the constraint.
- `MStatus connectTargetAttribute (void *opaqueTarget, int index, MObject &constraintAttr)`  
  This method is obsolete.
- `MStatus connectTargetAttribute (MDagPath &targetPath, int index, MObject &tarAttr, MObject &constraintAttr, bool instanced=false)`  
  Utility method to make any required connections for the constraint.
- `MStatus connectObjectAttribute (const MObject &objectAttr, const MObject &constraintAttr, bool toConstraint=true, bool instanced=false)`  
  This method can be used by connectObjectAndConstraint() to make any required connections between the constraint and the constrained object.
- `const MObject transformObject ()`  
  This method returns the target object being constrained.


### MPxDeformerNode

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_deformer_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_deformer_node.html).

Base class for user defined deformers with per-vertex weights. MPxDeformerNode builds on MPxGeometryFilter to allow the creation of deformers with per-vertex weights. Built-in Maya nodes which work in this way include the jiggle and cluster deformers. The weight values can be modified by the user using the set editing window or the percent command.

**Public Member Functions**

- `MPxDeformerNode ()`  
  Class constructor.
- `~MPxDeformerNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `float weightValue (MDataBlock &mblock, unsigned int multiIndex, unsigned int wtIndex)`  
  This method returns the weightValue stored in the datablock for the given geometry's lattice point/CV/vertex.
- `void setUseExistingConnectionWhenSetEditing (bool state)`  
  This method allows the plugin node to request special treatment during set editing.
- `MStatus setDeformationDetails (unsigned int flags)`  
  This method allows the plug-in node to inform the system that it intends to deform components other than just positions.
- `unsigned int getDeformationDetails (MStatus *ReturnStatus=NULL)`  
  Retrieves the value set by setDeformationDetails().

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject weightList`  
  weight list attribute, multi
- `static MObject weights`  
  weight attribute, multi


### MPxGPUDeformer

Module `OpenMayaAnim`. Source: [class_m_px_g_p_u_deformer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_g_p_u_deformer.html).

Base class for user defined GPU deformer override evaluators. MPxGPUDeformer lets you create user-defined GPU deformer overrides. A GPU deformer override replaces the CPU implementation of a deformer node when the evaluation manager is enabled and the deformerEvaluator plug-in is enabled. Use MPxGPUDeformer to override the deformation for a Maya deformer or for a plug-in deformer implemented through MPxDeformerNode. MPxGPUDeformer must register which node type it overrides using MGPUDeformerRegistry. MPxGPUDeformer defines a deformer node as a node which has input geometry and output geometry. MPxGPUDeformer assumes that the number of vertices in the input and output geometry of a deformer node are the same. This definition includes nodes which are traditionally thought of as deformer nodes, such as skinCluster or blendShape, but also includes nodes like groupParts, which may be part of deformation chains. To ensure optimal performance when you implement MPxGPUDeformer, keep the following in mind: Calls to the constructor must be fast. Do not do heavy work in the constructor because the deformer evaluator may allocate MPxGPUDeformer instances which are never used. Save heavy work for the evaluate() method. Cache needed values on the graphics card during evaluate(). Use the MEvaluationNode interface to determine if input values are constant or change over time. If you use this interface, you must implement the virtual method evaluate() for MPxGPUDeformer to function. There are several older deprecated signatures of evaluate(). See the evaluate() documentation for additional requirements on the evaluate() method. The terminate() method is optional. About the deformer evaluator: The deformer evaluator identifies chains of supported nodes terminated by a mesh. The deformer evaluator then replaces CPU evaluation of these nodes with GPGPU kernels. The final deformed geometry is directly shared with Viewport 2.0, which avoids any GPU read back. When you implement an MPxGPUDeformer for a given node type, you expand the list of deformer evaluator supported nodes, which then allows more deformer chains to execute on the GPU. A deformer chain is created by identifying an animated display mesh and then following geometry connections upstream until source plugs, that meet any one of the following criteria, is reached: On a node not supported by deformer evaluator. On a node which does not depend on time. Contains fan out connections. When one of the preceding conditions is true for a source plug, that source plug is considered an input to the deformation chain, and the corresponding source plug node is not evaluated in the deformer evaluator. Example Chains: HistoryNode: any node (supported or unsupported) which does not depend on time. origMesh: the original mesh shape: the ultimate source of the deformation chain. SupportedNode: a node that the deformer evaluator explicitly supports that also depends on time. UnsupportedNode: any node that the deformer evaluator does not support that also depends on time. displayMesh: the direct connection to Viewport 2. Example 1: HistoryNode1 -> origMesh -> HistoryNode2 -> SupportedNode1 -> SupportedNode2 -> displayMesh When the deformer evaluator initializes it identifies HistoryNode2.outputData as the source plug. The first time that the deformer evaluator runs on example 1, the HistoryNode2 geometry output is copied to the GPU. SupportedNode1 and SupportedNode2 then run kernels to perform the deformations, and the final deformed result is then shared with VP2. Subsequent evaluations re-use the copy of HistoryNode2 output geometry already on the GPU, which avoids expensive data transfer. Example 2: HistoryNode1 -> origMesh -> HistoryNode2 -> UnsupportedNode1 -> SupportedNode1 -> displayMesh In this scenario, UnsupportedNode1 runs on the CPU and generates an intermediate result. This intermediate result is copied to the GPU. Once copied, SupportedNode1 runs its kernel and displayMesh shares data with VP2. The intermediate result is copied to the GPU every frame, resulting in slower performance. Example 3: HistoryNode1-> origMesh -> HistoryNode2 -> SupportedNode1 -> UnsupportedNode1 -> displayMesh In this scenario, the deformer evaluator may create a chain with just displayMesh in it, or it may do nothing. If we performed SupportedNode1's deformation on the GPU, we would need to read back that data and use it as an input for UnsupportedNode1 on the CPU. Read back is not supported by the deformer evaluator. If the mesh geometry is large then a chain of only displayMesh may be used in order to compute mesh normals on the GPU. Example 4: origMesh1 -> SupportedNode1.outMesh[0] -> displayMesh1 origMesh2 -> SupportedNode1.outMesh[1] -> displayMesh2 In this scenario, the deformer evaluator creates two chains, one for displayMesh1 and a second for displayMesh2. These chains both run on the GPU. Note that if SupportedNode1 is derived from MPxDeformerNode, then there can be zero relationship between the mesh data used in displayMesh1 and displayMesh2. displayMesh1 could have 100 vertices, and displayMesh2 could have one million vertices. Example 5: origMesh1 -> SupportedNode1.outMesh[0] -> displayMesh1 origMesh2 -> SupportedNode1.outMesh[1] -> UnsupportedNode1 ->displayMesh2 In this scenario, the deformer evaluator does nothing. We do support the first chain for displayMesh1, but we do not support the chain for displayMesh2. The deformer evaluator does not support the partial override of a node. In this case, SupportedNode1 has only partial support because the deformer evaluator can override evaluation for outMesh[0] but not for outMesh[1]. This prevents deformer evaluator from doing any GPU work in this scenario. The deformer evaluator allocates a unique MPxGPUDeformer object for each supported node in each supported chain. In Example 4, two MPxGPUDeformer objects are allocated for SupportedNode1, one for multi-element 0 and a second for multi-element 1. The deformer evaluator's emphasis on avoiding geometry read back from the GPU means that unsupported nodes that follow a deformation chain exclude that chain from GPU evaluation. Example 6: origMesh1 -> SupportedNode1.outMesh -> BlendShape.outMesh[0] -> displayMesh1 origMesh2 -> SupportedNode2.outMesh -> BlendShape.inputTarget[0].inputTargetGroup[0].inputTargetItem[6000].inputGeomTarget In this scenario the blend shape node has an animated inputGeom as well as an animated blend shape target. The deformer evaluator forms a single chain which includes SupportedNode1, SupportedNode2, BlendShape and displayMesh1. MGPUDeformerRegistrationInfo::inputMeshAttributes() for the blend shape node returns both inputGeomTarget and inputGeom as attributes to search upstream of for additional GPU deformers. In this case SupportedNode1 and SupportedNode2 may run in any order, or in parallel with each other. BlendShape won't run until all predecessor deformers have finished evaluation. Example 7: origMesh1 -> SupportedNode1.outMesh -> BlendShape.outMesh[0] -> displayMesh1 origMesh2 -> UnsupportedNode1.outMesh -> BlendShape.inputTarget[0].inputTargetGroup[0].inputTargetItem[6000].inputGeomTarget In this scenario we have a combination of Example 2 and Example 6. GPU Override will form a chain including SupportedNode1, BlendShape and displayMesh1. When MPxGPUDeformer::evaluate() is called for BlendShape, inputData will include GPU buffers for BlendShape.input[0].inputGeometry and BlendShape.inputTarget[0].inputTargetGroup[0].inputTargetItem[6000].inputGeomTarget. The buffers coming from UnsupportedNode1.outMesh will automatically be copied to the GPU every frame.

**Public Member Functions**

- `MPxGPUDeformer ()`  
  Constructs an MPxGPUDeformer.
- `virtual ~MPxGPUDeformer ()`  
  Destructor.
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, unsigned int numElements, const MAutoCLMem, const MAutoCLEvent, MAutoCLMem, MAutoCLEvent &)`  
  This method is deprecated and should no longer be used.
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, const MGPUDeformerData &inputData, MGPUDeformerData &outputData)`  
  This method is deprecated and should no longer be used.
- `virtual DeformerStatus evaluate (MDataBlock &block, const MEvaluationNode &evaluationNode, const MPlug &outputPlug, const MPlugArray &inputPlugs, const MGPUDeformerData &inputData, MGPUDeformerData &outputData)`  
  evaluate() is called each time the deformation chain is evaluated.
- `virtual void terminate ()`  
  This method is called when the GPU deformer is being destroyed.
- `MGPUDeformerBuffer createOutputBuffer (const MGPUDeformerBuffer &inputBuffer)`  
  This method creates an output buffer based on inputBuffer.
- `bool getIndexMapper (MIndexMapper &imap)`  
  This method will update the IndexMapper information if required.
- `MObject getFixedSetupData (const MString &name)`  
  This method can retrieve data from the related node that is implemented for the CPU.
- `bool isFalloffWeightsDirty () const`  
  This method tells the GPU deformer if it need to ask weight from the deformer again the plug weightFunction might not be dirty (function is constant) but the falloff function might depends on the current geometry.

**Static Public Member Functions**

- `static bool hasAttributeBeenModified (const MEvaluationNode &evaluationNode, const MObject &attribute)`  
  This method returns true if the data for attribute on the overridden deformation node has been modified since the last call to MPxGPUDeformer::evaluate().
- `static bool hasPlugBeenModified (const MPlug &plug)`  
  This method returns true if the data for plug on the overridden deformation node has been modified since the last call to MPxGPUDeformer::evaluate().
- `static MUniqueString sPositionsName ()`  
  This method returns the name of position buffers on the GPU.
- `static MUniqueString sGeometryMatrixName ()`  
  This method returns the name of geometry matrix buffers on the GPU.
- `static MUniqueString sInverseGeometryMatrixName ()`  
  This method returns the name of inverse geometry matrix buffers on the GPU.
- `static bool isFalloffWeightsDependantOnCurrentGeometry (const MEvaluationNode &evaluationNode, unsigned int multiIndex)`  
  This method tells the GPU deformer if the weight function requires the current geometry to compute.


### MPxGeometryFilter

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_geometry_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_geometry_filter.html).

Base class for user-defined deformers. MPxGeometryFilter allows the creation of user-defined deformers. A deformer is a node which takes any number of input geometries, deforms them, and places the output into the output geometry attribute. If you write a deformer by deriving from MPxGeometryFilter, your deformer will derive the benefit of Maya's internal deformer functionality, namely: The Set Editing Tool and the Set Editor Window can be used to modify the membership of the deformer. "Has No Effect" mode is implemented in the base class, and disables the node. When the deformer node is deleted, its inputs and outputs will be correctly reconnected in the dependency graph. Reordering of the deformer node with other deformers via the Complete List window. Placement and connection of the deformer node in the dependency graph via the "deformer -type" command. The deformer command also has the standard deformer options such as exclusive membership and parallel mode. Deformers are full dependency nodes and can have attributes and a deform() method. In general, to derive the full benefit of the Maya deformer base class, it is suggested that you do not write your own compute() method. Instead, write the deform() method, which is called by the MPxGeometryFilter's compute() method. However, there are some exceptions when you would instead write your own compute(), namely: Your node's deformation algorithm depends on the geometry type, which is not available in the deform() method. Your node's deformation algorithm requires computing all of the output geometries simultaneously. In the case where you cannot simply override the deform() method, the following example code shows one possible compute() method implementation. This compute() example creates an iterator for the deformer set corresponding to the output geometry being computed. Note that this sample compute() implementation does not do any deformation, and does not implement handling of the nodeState attribute. If you do choose to override compute() in your node, there is no reason to implement the deform() method, since it will not be called by the base class. MStatus exampleDeformer::compute(const MPlug& plug, MDataBlock& dataBlock)

**Public Types**

- `enum DeformationDetails { kDeformsUVs = (1<<1), kDeformsColors = (1<<2) }`  
  Deformation details.

**Public Member Functions**

- `MPxGeometryFilter ()`  
  Class constructor.
- `~MPxGeometryFilter () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual MStatus deform (MDataBlock &block, MItGeometry &iter, const MMatrix &mat, unsigned int multiIndex)`  
  This method performs the deformation algorithm.
- `virtual MObject & accessoryAttribute () const`  
  This method returns an MObject for the attribute to which an accessory shape is connected.
- `virtual MStatus accessoryNodeSetup (MDagModifier &cmd)`  
  This method is called by the "deformer -type" command when your node is specified.
- `void setUseExistingConnectionWhenSetEditing (bool state)`  
  This method allows the plugin node to request special treatment during set editing.
- `MStatus setDeformationDetails (unsigned int flags)`  
  This method allows the plug-in node to inform the system that it intends to deform components other than just positions.
- `unsigned int getDeformationDetails (MStatus *ReturnStatus=NULL)`  
  Retrieves the value set by setDeformationDetails().
- `virtual void setModifiedCallback (MSelectionList &list, bool listAdded)`  
  This callback method can be overriden and is called whenever the set this deformer is operating on is modified.
- `MFnGeometryData::SubsetState getGeometryIterator (MItGeometry &iter, MDataBlock &block, MDataHandle &dataHandle, unsigned int multiIndex, bool readOnly=true, MStatus *ReturnStatus=nullptr) const`  
  This method initializes a geometry iterator for the subset of the geometry that must be deformed.
- `virtual MObject getFixedSetupData (const MString &name)`  
  This method retrieves data requested by the related node that is implemented for the GPU.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject input`  
  input attribute, multi
- `static MObject inputGeom`  
  input geometry attribute
- `static MObject groupId`  
  input group id attribute
- `static MObject componentTagExpression`  
  input componentTagExpression attribute
- `static MObject outputGeom`  
  geometry output attribute
- `static MObject envelope`  
  envelope attribute


### MPxIkSolver

Module `OpenMayaAnim`. Source: [class_m_px_ik_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_ik_solver.html).

OBSOLETE CLASS: Base class for user defined IK solvers. This is the obsolete base class for writing user-defined IK solvers. Users must at least override the following methods in order to write a solver: doSolve solverTypeName isSingleChainOnly isPositionOnly hasJointLimitSupport hasUniqueSolution groupHandlesByTopology Users can optionally override the following methods if they want to perform work before or after doSolve is called: preSolve postSolve If users override these methods, they are responsible for calling the base class implementation of the overridden method(s). The base class handles a few things such as solver locking/unlocking and FK/IK solution blending. If users fail to call the base class implementation, instabilities can occur leading to wrong evaluation or even crashes. To register a solver, write a creator method to return an instance of the user solver: userSolver::creator() {

**Public Member Functions**

- `virtual ~MPxIkSolver ()`  
  Destructor.
- `virtual MStatus preSolve ()`  
  This method is called before doSolve.
- `virtual MStatus doSolve ()`  
  This is where the main solving takes place.
- `virtual MStatus postSolve (MStatus)`  
  This method is called after doSolve has finished.
- `virtual MString solverTypeName () const`  
  This method returns the type name of the solver.
- `virtual bool isSingleChainOnly () const`  
  This method indicates whether this solver is a single chain solver.
- `virtual bool isPositionOnly () const`  
  Indicates whether the solver supports handle orientation.
- `virtual bool hasJointLimitSupport () const`  
  This method indicates whether the solver supports limits on joint angles.
- `virtual bool hasUniqueSolution () const`  
  This method indicates whether the solver provides a unique solution.
- `virtual bool groupHandlesByTopology () const`  
  Indicates whether the IK system should group handles according to skeletal topology.
- `virtual MStatus setFuncValueTolerance (double tolerance)`  
  Set the error value for this solver.
- `virtual MStatus setMaxIterations (int value)`  
  Set the maximum iterations for a solution by this solver.
- `MIkHandleGroup * handleGroup () const`  
  Returns the handle group for this solver.
- `virtual void setHandleGroup (MIkHandleGroup *)`  
  Set the handle group of this solver.
- `const MMatrix * toWorldSpace () const`  
  Returns the world space matrix for this solver.
- `const MMatrix * toSolverSpace () const`  
  Returns the local space matrix for this solver.
- `double funcValueTolerance () const`  
  Return the error value for this solver.
- `int maxIterations () const`  
  Return the the maximum nuber of itertations for a solution by this solver.
- `virtual void snapHandle (MObject &handle)`  
  This function positions the handle at the end effector position.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus getJointAngles (MDoubleArray &) const`  
  Returns the joint angles.
- `MStatus setJointAngles (const MDoubleArray &)`  
  Set joint angles to the given values.
- `void setToRestAngles ()`  
  Sets all joints to thier prefered orientations.
- `MPxIkSolver ()`  
  Constructor.


### MPxIkSolverNode

Module `OpenMayaAnim`. Inherits `MPxNode`. Source: [class_m_px_ik_solver_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_ik_solver_node.html).

Base class for user defined IK solvers. This is the base class for writing user-defined IK solvers. Users must at least override the following methods in order to write a solver: doSolve solverTypeName Users can optionally override the following methods if they want to perform work before or after doSolve is called: preSolve postSolve Since Maya 2018, users no longer need to call the base class version of these functions if overriding them. Note that the following virtual methods (declared in MPxNode) are irrelevant for the MPxIkSolverNode. If these methods are overridden in a class derived from MPxIkSolverNode, they will be ignored. compute getInternalValue setInternalValue legalConnection legalDisconnection connectionMade connectionBroken A creator method is necessary to return an instance of the user solver: userSolver::creator() {

**Public Member Functions**

- `~MPxIkSolverNode () override`  
  Destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual MStatus preSolve ()`  
  This method is called before doSolve.
- `virtual MStatus doSolve ()`  
  This is where the main solving takes place.
- `virtual MStatus postSolve (MStatus)`  
  This method is called after doSolve has finished.
- `virtual MString solverTypeName () const`  
  This method returns the type name of the solver.
- `bool rotatePlane (MStatus *ReturnStatus=NULL) const`  
  This method indicates whether this solver supports the rotate plane.
- `MStatus setRotatePlane (bool rotatePlane)`  
  This method sets whether or not this solver supports the rotate plane.
- `bool singleChainOnly (MStatus *ReturnStatus=NULL) const`  
  This method indicates whether this solver is a single chain solver.
- `MStatus setSingleChainOnly (bool singleChainOnly)`  
  This method sets whether or not this solver is a single chain solver.
- `bool positionOnly (MStatus *ReturnStatus=NULL) const`  
  Indicates whether the ik solution is dependent on the ikHandle position only or also uses the orientation.
- `MStatus setPositionOnly (bool positionOnly)`  
  Sets whether or not the solver supports handle orientation.
- `bool supportJointLimits (MStatus *ReturnStatus=NULL) const`  
  This method indicates whether the solver supports limits on joint angles.
- `MStatus setSupportJointLimits (bool supportJointLimits)`  
  This method sets whether or not the solver supports limits on joint angles.
- `bool uniqueSolution (MStatus *ReturnStatus=NULL) const`  
  This method indicates whether the solver provides a unique solution.
- `MStatus setUniqueSolution (bool uniqueSolution)`  
  This method sets whether or not the solver provides a unique solution.
- `virtual bool isSingleChainOnly () const`  
  This method indicates whether this solver is a single chain solver.
- `virtual bool isPositionOnly () const`  
  This method is obsolete.
- `virtual bool hasJointLimitSupport () const`  
  This method is obsolete.
- `virtual bool hasUniqueSolution () const`  
  This method is obsolete.
- `virtual bool groupHandlesByTopology () const`  
  This method is obsolete.
- `virtual MStatus setFuncValueTolerance (double tolerance)`  
  Set the error value for this solver.
- `virtual MStatus setMaxIterations (int value)`  
  Set the maximum iterations for a solution by this solver.
- `MIkHandleGroup * handleGroup () const`  
  Returns the handle group for this solver.
- `virtual void setHandleGroup (MIkHandleGroup *)`  
  Set the handle group of this solver.
- `const MMatrix * toWorldSpace () const`  
  Returns the world space matrix for this solver.
- `const MMatrix * toSolverSpace () const`  
  Returns the local space matrix for this solver.
- `double funcValueTolerance () const`  
  Return the error value for this solver.
- `int maxIterations () const`  
  Return the the maximum nuber of itertations for a solution by this solver.
- `virtual void snapHandle (MObject &handle)`  
  This function positions the handle at the end effector position.
- `virtual bool isAttributeCreatedBySolver (MObject &attr) const`  
  This function returns whether a certain attribute on the ikHandle was created by the solver (and affecting the result of the solve).

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus getJointAngles (MDoubleArray &) const`  
  This method is not available in Python.
- `MStatus setJointAngles (const MDoubleArray &)`  
  This method is not available in Python.
- `void setToRestAngles ()`  
  This method is not available in Python.
- `MPxIkSolverNode ()`  
  Constructor.


### MPxSkinCluster

Module `OpenMayaAnim`. Inherits `MPxGeometryFilter`. Source: [class_m_px_skin_cluster.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_skin_cluster.html).

Base class for user-defined skin deformers. MPxSkinCluster allows the creation of user-defined skin deformers. It derives from MPxGeometryFilter and so offers all the functionality of that class. Additionally, it has the per-vertex skin weights and other skin-related attributes of the Maya built-in skinCluster node. Custom nodes derived from MPxSkinCluster are treated by Maya just like the built-in skinCluster, so all the weight painting/editing etc. tools that artists are used to also work on the custom nodes.

**Public Member Functions**

- `MPxSkinCluster ()`  
  Class constructor.
- `~MPxSkinCluster () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `float weightValue (MDataBlock &mblock, unsigned int multiIndex, unsigned int wtIndex)`  
  This method returns the weightValue stored in the datablock for the given geometry's lattice point/CV/vertex.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject weightList`  
  weight list attribute, multi
- `static MObject weights`  
  weight attribute, multi
- `static MObject matrix`  
  matrix attribute, multi
- `static MObject bindPreMatrix`  
  bindPreMatrix attribute, multi

