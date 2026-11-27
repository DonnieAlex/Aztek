# OpenMaya

Class names, bases and signatures of the module, checked against the reference's 2022 C++ API reference (https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya.html). Descriptions are not kept here; follow the source link on a class for them.

Classes in this module: 271.

## Classes

### MAddRemoveAttrEdit

Module `OpenMaya`. Inherits `MEdit`. Source: [class_m_add_remove_attr_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_add_remove_attr_edit.html).

**Public Member Functions**

- `MObject node (MStatus *ReturnStatus=NULL) const`
- `MString attributeName (MStatus *ReturnStatus=NULL) const`
- `MString nodeName (MStatus *ReturnStatus=NULL) const`
- `bool isAttributeAdded () const`
- `EditType editType (MStatus *ReturnStatus=NULL) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MAngle

Module `OpenMaya`. Source: [class_m_angle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_angle.html).

**Public Types**

- `enum Unit { kInvalid, kRadians, kDegrees, kAngMinutes, kAngSeconds, kLast }`

**Public Member Functions**

- `MAngle ()`
- `MAngle (const MAngle &src)`
- `MAngle (double value, Unit u=kRadians)`
- `~MAngle ()`
- `MAngle & operator= (const MAngle &other)`
- `Unit unit () const`
- `double value () const`
- `MStatus setUnit (Unit newUnit)`
- `MStatus setValue (double newValue)`
- `double as (Unit otherUnit, MStatus *ReturnStatus=NULL) const`
- `double asUnits (Unit otherUnit, MStatus *ReturnStatus=NULL) const`
- `double asRadians () const`
- `double asDegrees () const`
- `double asAngMinutes () const`
- `double asAngSeconds () const`

**Static Public Member Functions**

- `static Unit uiUnit ()`
- `static MStatus setUIUnit (Unit newUnit)`
- `static Unit internalUnit ()`
- `static double internalToUI (double internalValue)`
- `static double uiToInternal (double uiValue)`
- `static const char * className ()`
- `static MStatus setInternalUnit (Unit internalUnit)`

### MArgDatabase

Module `OpenMaya`. Inherits `MArgParser`. Source: [class_m_arg_database.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_database.html).

**Public Member Functions**

- `MArgDatabase ()`
- `MArgDatabase (const MSyntax &syntax, const MArgList &argList, MStatus *ReturnStatus=NULL)`
- `~MArgDatabase () override`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MSelectionList &result) const`
- `MStatus getCommandArgument (unsigned int index, MSelectionList &result) const`
- `MStatus getObjects (MSelectionList &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, bool &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, int &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, double &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MString &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, unsigned int &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MDistance &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MAngle &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MTime &result) const`
- `MStatus getCommandArgument (unsigned int index, bool &result) const`
- `MStatus getCommandArgument (unsigned int index, int &result) const`
- `MStatus getCommandArgument (unsigned int index, double &result) const`
- `MStatus getCommandArgument (unsigned int index, MString &result) const`
- `MStatus getCommandArgument (unsigned int index, MDistance &result) const`
- `MStatus getCommandArgument (unsigned int index, MAngle &result) const`
- `MStatus getCommandArgument (unsigned int index, MTime &result) const`
- `MStatus getObjects (MStringArray &result) const`

**Static Public Member Functions**

- `static const char * className ()`

### MArgList

Module `OpenMaya`. Source: [class_m_arg_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_list.html).

**Public Member Functions**

- `MArgList ()`
- `MArgList (const MArgList &other)`
- `~MArgList ()`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `bool asBool (unsigned int index, MStatus *ret=NULL) const`
- `int asInt (unsigned int index, MStatus *ret=NULL) const`
- `double asDouble (unsigned int index, MStatus *ret=NULL) const`
- `MString asString (unsigned int index, MStatus *ret=NULL) const`
- `MDistance asDistance (unsigned int index, MStatus *ret=NULL) const`
- `MAngle asAngle (unsigned int index, MStatus *ret=NULL) const`
- `MTime asTime (unsigned int index, MStatus *ret=NULL) const`
- `MVector asVector (unsigned int &index, unsigned int numElements=3, MStatus *ReturnStatus=NULL) const`
- `MPoint asPoint (unsigned int &index, unsigned int numElements=3, MStatus *ReturnStatus=NULL) const`
- `MMatrix asMatrix (unsigned int &index, MStatus *ret=NULL) const`
- `MIntArray asIntArray (unsigned int &index, MStatus *ReturnStatus=NULL) const`
- `MDoubleArray asDoubleArray (unsigned int &index, MStatus *ReturnStatus=NULL) const`
- `const MStringArray asStringArray (unsigned int &index, MStatus *ReturnStatus=NULL) const`
- `MArgList & operator= (const MArgList &other)`
- `unsigned int flagIndex (const char *shortFlag, const char *longFlag=NULL) const`
- `MStatus addArg (bool arg)`
- `MStatus addArg (int arg)`
- `MStatus addArg (double arg)`
- `MStatus addArg (const MString &arg)`
- `MStatus addArg (const MDistance &arg)`
- `MStatus addArg (const MAngle &arg)`
- `MStatus addArg (const MTime &arg)`
- `MStatus addArg (const MVector &arg)`
- `MStatus addArg (const MPoint &arg)`
- `MStatus get (unsigned int index, bool &ret) const`
- `MStatus get (unsigned int index, int &ret) const`
- `MStatus get (unsigned int index, double &ret) const`
- `MStatus get (unsigned int index, MString &ret) const`
- `MStatus get (unsigned int index, MDistance &ret) const`
- `MStatus get (unsigned int index, MAngle &ret) const`
- `MStatus get (unsigned int index, MTime &ret) const`
- `MStatus get (unsigned int &index, MVector &ret, unsigned int numElements=3) const`
- `MStatus get (unsigned int &index, MPoint &ret, unsigned int numElements=3) const`
- `MStatus get (unsigned int &index, MMatrix &ret) const`
- `MStatus get (unsigned int &index, MIntArray &ret) const`
- `MStatus get (unsigned int &index, MDoubleArray &ret) const`
- `MStatus get (unsigned int &index, MStringArray &ret) const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MArgList &args)`

### MArgParser

Module `OpenMaya`. Source: [class_m_arg_parser.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html).

**Public Member Functions**

- `MArgParser ()`
- `MArgParser (const MSyntax &syntax, const MArgList &argList, MStatus *ReturnStatus=NULL)`
- `virtual ~MArgParser ()`
- `bool isFlagSet (const char *flag, MStatus *ReturnStatus=NULL) const`
- `bool isEdit (MStatus *ReturnStatus=NULL) const`
- `bool isQuery (MStatus *ReturnStatus=NULL) const`
- `unsigned int numberOfFlagsUsed () const`
- `unsigned int numberOfFlagUses (const char *flag) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, bool &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, int &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, double &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MString &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, unsigned int &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MDistance &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MAngle &result) const`
- `MStatus getFlagArgument (const char *flag, unsigned int index, MTime &result) const`
- `bool flagArgumentBool (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `int flagArgumentInt (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `double flagArgumentDouble (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MString flagArgumentString (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MDistance flagArgumentMDistance (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MAngle flagArgumentMAngle (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MTime flagArgumentMTime (const char *flag, unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getFlagArgumentPosition (const char *flag, unsigned int i, unsigned int &position) const`
- `MStatus getFlagArgumentList (const char *flag, unsigned int i, MArgList &args) const`
- `MStatus getCommandArgument (unsigned int index, bool &result) const`
- `MStatus getCommandArgument (unsigned int index, int &result) const`
- `MStatus getCommandArgument (unsigned int index, double &result) const`
- `MStatus getCommandArgument (unsigned int index, MString &result) const`
- `MStatus getCommandArgument (unsigned int index, MDistance &result) const`
- `MStatus getCommandArgument (unsigned int index, MAngle &result) const`
- `MStatus getCommandArgument (unsigned int index, MTime &result) const`
- `bool commandArgumentBool (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `int commandArgumentInt (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `double commandArgumentDouble (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MString commandArgumentString (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MDistance commandArgumentMDistance (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MAngle commandArgumentMAngle (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MTime commandArgumentMTime (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getObjects (MStringArray &result) const`

**Static Public Member Functions**

- `static const char * className ()`

### MArrayDataBuilder

Module `OpenMaya`. Source: [class_m_array_data_builder.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_array_data_builder.html).

**Public Member Functions**

- `MArrayDataBuilder (MDataBlock *, const MObject &attribute, unsigned int numElements, MStatus *ReturnStatus=NULL)`
- `MDataHandle addLast (MStatus *ReturnStatus=NULL)`
- `MDataHandle addElement (unsigned int index, MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle addLastArray (MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle addElementArray (unsigned int index, MStatus *ReturnStatus=NULL)`
- `MStatus removeElement (unsigned int index)`
- `unsigned int elementCount (MStatus *ReturnStatus=NULL) const`
- `MStatus growArray (unsigned int amount)`
- `MStatus setGrowSize (unsigned int size)`
- `MArrayDataBuilder (const MArrayDataBuilder &other)`
- `MArrayDataBuilder & operator= (const MArrayDataBuilder &other)`
- `~MArrayDataBuilder ()`
- `MArrayDataBuilder (const MObject &attribute, unsigned int numElements, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MArrayDataHandle`

### MArrayDataHandle

Module `OpenMaya`. Source: [class_m_array_data_handle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_array_data_handle.html).

**Public Member Functions**

- `MArrayDataHandle (const MDataHandle &in, MStatus *ReturnStatus=NULL)`
- `MDataHandle inputValue (MStatus *ReturnStatus=NULL)`
- `MDataHandle outputValue (MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle inputArrayValue (MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle outputArrayValue (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `unsigned int elementCount (MStatus *ReturnStatus=NULL)`
- `unsigned int elementIndex (MStatus *ReturnStatus=NULL)`
- `MStatus jumpToElement (unsigned int index)`
- `MStatus jumpToArrayElement (unsigned int position)`
- `MStatus setClean ()`
- `MStatus setAllClean ()`
- `MArrayDataBuilder builder (MStatus *ReturnStatus=NULL)`
- `MStatus set (MArrayDataBuilder &builder)`
- `MArrayDataHandle (const MArrayDataHandle &other)`
- `MArrayDataHandle & operator= (const MArrayDataHandle &other)`

**Static Public Member Functions**

- `static const char * className ()`

### MAtomic

Module `OpenMaya`. Source: [class_m_atomic.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_atomic.html).

**Static Public Member Functions**

- `static FORCE_INLINE int preIncrement (volatile int *variable)`
- `static FORCE_INLINE int postIncrement (volatile int *variable)`
- `static FORCE_INLINE int increment (volatile int *variable, int incrementValue)`
- `static FORCE_INLINE int preDecrement (volatile int *variable)`
- `static FORCE_INLINE int postDecrement (volatile int *variable)`
- `static FORCE_INLINE int decrement (volatile int *variable, int decrementValue)`
- `static FORCE_INLINE int set (volatile int *variable, int newValue)`
- `static FORCE_INLINE int compareAndSwap (volatile int *variable, int compareValue, int swapValue)`

### MAttributeIndex

Module `OpenMaya`. Source: [class_m_attribute_index.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_attribute_index.html).

**Public Types**

- `enum MIndexType { kInteger, kFloat }`

**Public Member Functions**

- `MAttributeIndex ()`
- `~MAttributeIndex ()`
- `MAttributeIndex (const MAttributeIndex &other)`
- `MAttributeIndex (int value)`
- `MAttributeIndex (double value)`
- `MIndexType type () const`
- `bool hasRange () const`
- `bool hasValidRange () const`
- `bool hasLowerBound () const`
- `bool hasUpperBound () const`
- `MStatus getLower (int &value) const`
- `MStatus getLower (double &value) const`
- `MStatus getUpper (int &value) const`
- `MStatus getUpper (double &value) const`
- `bool isBounded () const`
- `MStatus getValue (int &value) const`
- `MStatus getValue (double &value) const`
- `MStatus setType (MIndexType type)`
- `MStatus setValue (int value)`
- `MStatus setValue (double value)`
- `MStatus setLower (int value)`
- `MStatus setLower (double value)`
- `MStatus setUpper (int value)`
- `MStatus setUpper (double value)`
- `MAttributeIndex & operator= (const MAttributeIndex &other)`
- `bool operator== (const MAttributeIndex &other) const`
- `bool operator!= (const MAttributeIndex &other) const`

**Static Public Member Functions**

- `static const char * className ()`

### MAttributePattern

Module `OpenMaya`. Source: [class_m_attribute_pattern.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_attribute_pattern.html).

**Public Member Functions**

- `MAttributePattern ()`
- `MAttributePattern (const MString &name)`
- `MAttributePattern (const MAttributePattern &rhs)`
- `MAttributePattern & operator= (const MAttributePattern &rhs)`
- `bool operator== (const MAttributePattern &rhs) const`
- `~MAttributePattern ()`
- `MString name (MStatus *ReturnStatus=NULL) const`
- `unsigned int rootAttrCount (MStatus *ReturnStatus=NULL) const`
- `MObject rootAttr (unsigned int idx, MStatus *ReturnStatus=NULL) const`
- `MStatus removeRootAttr (unsigned int idx)`
- `MStatus removeRootAttr (const MObject &attr)`
- `MStatus addRootAttr (const MObject &attr)`

**Static Public Member Functions**

- `static MAttributePattern * findPattern (const MString &name)`
- `static unsigned int attrPatternCount ()`
- `static MAttributePattern * attrPattern (unsigned int n)`
- `static const char * className ()`

### MAttributePatternArray

Module `OpenMaya`. Source: [class_m_attribute_pattern_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_attribute_pattern_array.html).

**Public Member Functions**

- `MAttributePatternArray ()`
- `MAttributePatternArray (const MAttributePatternArray &other)`
- `MAttributePatternArray (unsigned int initializeSize, const MAttributePattern &initialValue)`
- `~MAttributePatternArray ()`
- `const MAttributePattern & operator[] (unsigned int index) const`
- `MAttributePatternArray & operator= (const MAttributePatternArray &rhs)`
- `MStatus set (const MAttributePattern &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MAttributePattern &element, unsigned int index)`
- `MStatus append (const MAttributePattern &element)`
- `MStatus copy (const MAttributePatternArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MAttributePattern & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MAttributePatternArray &array)`

### MAttributeSpec

Module `OpenMaya`. Source: [class_m_attribute_spec.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_attribute_spec.html).

**Public Member Functions**

- `MAttributeSpec ()`
- `MAttributeSpec (const char *name)`
- `MAttributeSpec (const MString &name)`
- `MAttributeSpec (const MAttributeSpec &other)`
- `~MAttributeSpec ()`
- `const MString name () const`
- `int dimensions () const`
- `void setName (const MString &name)`
- `void setDimensions (int value)`
- `MAttributeSpec & operator= (const MAttributeSpec &rhs)`
- `const MAttributeIndex operator[] (int index) const`
- `bool operator== (const MAttributeSpec &other) const`
- `MAttributeIndex operator[] (int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MAttributeSpecArray

Module `OpenMaya`. Source: [class_m_attribute_spec_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_attribute_spec_array.html).

**Public Member Functions**

- `MAttributeSpecArray ()`
- `MAttributeSpecArray (const MAttributeSpecArray &other)`
- `MAttributeSpecArray (unsigned int initializeSize, const MAttributeSpec &initialValue)`
- `~MAttributeSpecArray ()`
- `MAttributeSpec operator[] (unsigned int index) const`
- `MAttributeSpecArray & operator= (const MAttributeSpecArray &other)`
- `MStatus set (const MAttributeSpec &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MAttributeSpec &element, unsigned int index)`
- `MStatus append (const MAttributeSpec &element)`
- `MStatus copy (const MAttributeSpecArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

### MAutoCLEvent

Module `OpenMaya`. Source: [class_m_auto_c_l_event.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_auto_c_l_event.html).

**Public Member Functions**

- `MAutoCLEvent ()`
- `~MAutoCLEvent ()`
- `MAutoCLEvent (const MAutoCLEvent &other)`
- `MAutoCLEvent & operator= (const MAutoCLEvent &other)`
- `MAutoCLEvent (cl_event value, NoRef dummy)`
- `void attach (cl_event value)`
- `cl_event detach ()`
- `void reset ()`
- `cl_event get () const`
- `bool isNull () const`
- `const cl_event * getReadOnlyRef () const`
- `cl_event * getReferenceForAssignment ()`
- `bool operator== (const MAutoCLEvent &other) const`
- `bool operator== (cl_event other) const`
- `bool operator! () const`
- `void swap (MAutoCLEvent &other)`
- `bool operator< (const MAutoCLEvent &other) const`

### MAutoCLKernel

Module `OpenMaya`. Source: [class_m_auto_c_l_kernel.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_auto_c_l_kernel.html).

**Public Member Functions**

- `MAutoCLKernel ()`
- `~MAutoCLKernel ()`
- `MAutoCLKernel (const MAutoCLKernel &other)`
- `MAutoCLKernel & operator= (const MAutoCLKernel &other)`
- `MAutoCLKernel (cl_kernel value, NoRef dummy)`
- `void attach (cl_kernel value)`
- `cl_kernel detach ()`
- `void reset ()`
- `cl_kernel get () const`
- `bool isNull () const`
- `const cl_kernel * getReadOnlyRef () const`
- `cl_kernel * getReferenceForAssignment ()`
- `bool operator== (const MAutoCLKernel &other) const`
- `bool operator== (cl_kernel other) const`
- `bool operator! () const`
- `void swap (MAutoCLKernel &other)`
- `bool operator< (const MAutoCLKernel &other) const`

### MAutoCLMem

Module `OpenMaya`. Source: [class_m_auto_c_l_mem.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_auto_c_l_mem.html).

**Public Member Functions**

- `MAutoCLMem ()`
- `~MAutoCLMem ()`
- `MAutoCLMem (const MAutoCLMem &other)`
- `MAutoCLMem & operator= (const MAutoCLMem &other)`
- `MAutoCLMem (cl_mem value, NoRef dummy)`
- `void attach (cl_mem value)`
- `cl_mem detach ()`
- `void reset ()`
- `cl_mem get () const`
- `bool isNull () const`
- `const cl_mem * getReadOnlyRef () const`
- `cl_mem * getReferenceForAssignment ()`
- `bool operator== (const MAutoCLMem &other) const`
- `bool operator== (cl_mem other) const`
- `bool operator! () const`
- `void swap (MAutoCLMem &other)`
- `bool operator< (const MAutoCLMem &other) const`

### MBoundingBox

Module `OpenMaya`. Source: [class_m_bounding_box.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_bounding_box.html).

**Public Member Functions**

- `MBoundingBox ()`
- `MBoundingBox (const MBoundingBox &src)`
- `MBoundingBox (const MPoint &corner1, const MPoint &corner2)`
- `~MBoundingBox ()`
- `void clear ()`
- `void transformUsing (const MMatrix &matrix)`
- `void expand (const MPoint &point)`
- `void expand (const MBoundingBox &box)`
- `bool contains (const MPoint &point) const`
- `bool intersects (const MBoundingBox &box, double tol=0.0) const`
- `double width () const`
- `double height () const`
- `double depth () const`
- `MPoint center () const`
- `MPoint min () const`
- `MPoint max () const`
- `MBoundingBox & operator= (const MBoundingBox &other)`

### MCacheConfigRuleRegistry

Module `OpenMaya`. Source: [class_m_cache_config_rule_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_cache_config_rule_registry.html).

**Static Public Member Functions**

- `static MStatus registerFilter (const MString &filterName, MCreateCacheConfigRuleFilterFunction creatorFunction)`
- `static MStatus unregisterFilter (const MString &filterName)`
- `static const char * className ()`
- `static void setRegisteringCallableScript ()`
- `static bool registeringCallableScript ()`

### MCacheFormatDescription

Module `OpenMaya`. Source: [class_m_cache_format_description.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_cache_format_description.html).

**Public Types**

- `enum CacheFileDistribution { kNoFile, kOneFile, kOneFilePerFrame }`
- `enum CacheDataType { kUnknownData, kDouble, kDoubleArray, kDoubleVectorArray, kInt32Array, kFloatArray, kFloatVectorArray }`
- `enum CacheSamplingType { kRegular, kIrregular }`

**Public Member Functions**

- `void setDistribution (CacheFileDistribution distribution)`
- `void setTimePerFrame (const MTime &timePerFrame)`
- `void addDescriptionInfo (const MString &info)`
- `unsigned int addChannel (const MString &channelName, const MString &interpretation, CacheDataType dataType, CacheSamplingType samplingType, const MTime &samplingRate, const MTime &startTime, const MTime &endTime, MStatus *status=NULL)`
- `CacheFileDistribution getDistribution () const`
- `MTime getTimePerFrame () const`
- `void getStartAndEndTimes (MTime &startTime, MTime &endTime) const`
- `void getDescriptionInfo (MStringArray &info) const`
- `MStringArray getDescriptionInfo () const`
- `unsigned int getNumChannels () const`
- `MString getChannelName (unsigned int channelIndex) const`
- `MString getChannelInterpretation (unsigned int channelIndex) const`
- `CacheDataType getChannelDataType (unsigned int channelIndex) const`
- `CacheSamplingType getChannelSamplingType (unsigned int channelIndex) const`
- `MTime getChannelSamplingRate (unsigned int channelIndex) const`
- `MTime getChannelStartTime (unsigned int channelIndex) const`
- `MTime getChannelEndTime (unsigned int channelIndex) const`
- `virtual ~MCacheFormatDescription ()`

**Static Public Member Functions**

- `static const char * className ()`

### MCallbackIdArray

Module `OpenMaya`. Source: [class_m_callback_id_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_callback_id_array.html).

**Public Member Functions**

- `MCallbackIdArray ()`
- `MCallbackIdArray (const MCallbackIdArray &other)`
- `MCallbackIdArray (unsigned int initialSize, MCallbackId initialValue=0)`
- `~MCallbackIdArray ()`
- `MCallbackId operator[] (unsigned int index) const`
- `MCallbackIdArray & operator= (const MCallbackIdArray &other)`
- `MStatus set (MCallbackId element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (MCallbackId element, unsigned int index)`
- `MStatus append (MCallbackId element)`
- `MStatus copy (const MCallbackIdArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MCallbackIdArray (const MCallbackId src[], unsigned int count)`
- `MStatus get (MCallbackId[]) const`
- `MCallbackId & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MCallbackIdArray &array)`

### MCameraMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_camera_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_camera_message.html).

**Static Public Member Functions**

- `static MCallbackId addBeginManipulationCallback (const MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addEndManipulationCallback (const MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MCameraSetMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_camera_set_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_camera_set_message.html).

**Static Public Member Functions**

- `static MCallbackId addCameraLayerCallback (MMessage::MCameraLayerFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCameraChangedCallback (MMessage::MCameraLayerCameraFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MColor

Module `OpenMaya`. Source: [class_m_color.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color.html).

**Public Types**

- `enum MColorType { kRGB, kHSV, kCMY, kCMYK }`

**Public Member Functions**

- `MColor ()`
- `MColor (const MColor &src)`
- `MColor (float rr, float gg, float bb=0.0, float aa=1.0)`
- `MColor (const float d[3])`
- `MColor (float alpha)`
- `MColor (MColorType colorModel, unsigned char c1, unsigned char c2, unsigned char c3, unsigned char alpha=255U)`
- `MColor (MColorType colorModel, unsigned short c1, unsigned short c2, unsigned short c3, unsigned short alpha=65535U)`
- `MColor (MColorType colorModel, float c1, float c2, float c3, float alpha=1.0)`
- `MColor (MColorType colorModel, double c1, double c2, double c3, double alpha=1.0)`
- `~MColor ()`
- `MColor & operator= (const MColor &src)`
- `float operator() (unsigned int i) const`
- `float operator[] (unsigned int i) const`
- `MColor & operator*= (float scalar)`
- `MColor operator* (float scalar) const`
- `MColor & operator*= (const MColor &other)`
- `MColor operator* (const MColor &other) const`
- `MColor & operator/= (float scalar)`
- `MColor operator/ (float scalar) const`
- `MColor & operator+= (const MColor &other)`
- `MColor operator+ (const MColor &other) const`
- `MColor operator- () const`
- `MColor operator- (const MColor &other) const`
- `bool operator== (const MColor &other) const`
- `bool operator!= (const MColor &other) const`
- `bool get (float d[3]) const`
- `bool get (MColorType colorModel, float &, float &, float &) const`
- `bool get (MColorType colorModel, float &, float &, float &, float &alpha) const`
- `bool set (MColorType colorModel, float, float, float, float alpha=1.0)`
- `float & operator() (unsigned int i)`
- `float & operator[] (unsigned int i)`

**Public Attributes**

- `float r`
- `float g`
- `float b`
- `float a`

**Static Public Attributes**

- `static const MColor kOpaqueBlack`

**Friends**

- `OPENMAYA_EXPORT MColor operator* (float scalar, const MColor &other)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MColor &c)`

### MColorArray

Module `OpenMaya`. Source: [class_m_color_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_array.html).

**Public Member Functions**

- `MColorArray ()`
- `MColorArray (const MColorArray &other)`
- `MColorArray (const MColor src[], unsigned int count)`
- `MColorArray (const float src[][4], unsigned int count)`
- `MColorArray (const double src[][4], unsigned int count)`
- `MColorArray (unsigned int initialSize, const MColor &initialValue=MColor::kOpaqueBlack)`
- `~MColorArray ()`
- `const MColor & operator[] (unsigned int index) const`
- `MColorArray & operator= (const MColorArray &other)`
- `MStatus set (const MColor &element, unsigned int index)`
- `MStatus set (unsigned int index, float r, float g, float b, float a=1)`
- `MStatus set (float val[4], unsigned int index)`
- `MStatus set (float elements[][4], unsigned int size)`
- `MStatus set (double elements[][4], unsigned int size)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MColor &element, unsigned int index)`
- `MStatus append (const MColor &element)`
- `MStatus append (float r, float g, float b, float a=1)`
- `MStatus copy (const MColorArray &source)`
- `MStatus clear ()`
- `MStatus get (float dest[][4]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MColor & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MColorArray &array)`

### MCommandMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_command_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_command_message.html).

**Public Types**

- `enum MessageType { kHistory, kDisplay, kInfo, kWarning, kError, kResult, kStackTrace }`
- `typedef void(* MMessageFunction) (const MString &message, MCommandMessage::MessageType messageType, void *clientData)`
- `typedef void(* MMessageFilterFunction) (const MString &message, MCommandMessage::MessageType messageType, bool &filterOutput, void *clientData)`

**Static Public Member Functions**

- `static MCallbackId addCommandCallback (MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCommandOutputCallback (MCommandMessage::MMessageFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCommandOutputFilterCallback (MCommandMessage::MMessageFilterFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`
- `static MCallbackId addProcCallback (MMessage::MStringIntBoolIntFunction func, void *clientData, MStatus *ReturnStatus)`

### MCommandResult

Module `OpenMaya`. Source: [class_m_command_result.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_command_result.html).

**Public Types**

- `enum Type { kInvalid = 0, kInt, kInt64, kIntArray, kInt64Array, kDouble, kDoubleArray, kString, kStringArray, kVector, kVectorArray, kMatrix, kMatrixArray }`

**Public Member Functions**

- `MCommandResult (MStatus *ReturnStatus=NULL)`
- `virtual ~MCommandResult ()`
- `Type resultType (MStatus *ReturnStatus=NULL) const`
- `MStatus getResult (int &result) const`
- `MStatus getResult (MInt64 &result) const`
- `MStatus getResult (MIntArray &result) const`
- `MStatus getResult (MInt64Array &result) const`
- `MStatus getResult (double &result) const`
- `MStatus getResult (MDoubleArray &result) const`
- `MString stringResult (MStatus *ReturnResult=NULL) const`
- `MStatus getResult (MString &result) const`
- `MStatus getResult (MStringArray &result) const`
- `MStatus getResult (MVector &result) const`
- `MStatus getResult (MVectorArray &result) const`
- `MStatus getResult (MDoubleArray &result, int &numRows, int &numColumns) const`

**Static Public Member Functions**

- `static const char * className ()`

### MCommonSystemUtils

Module `OpenMaya`. Source: [class_m_common_system_utils.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_common_system_utils.html).

**Static Public Member Functions**

- `static MString getEnv (const MString &variable, MStatus *ResultStatus=NULL)`
- `static MStatus putEnv (const MString &variable, const MString &value)`
- `static MStatus makeDirectory (const MString &path)`
- `static MStatus getEnv (const MString &variable, MString &result)`
- `static const char * className ()`

### MComputation

Module `OpenMaya`. Source: [class_m_computation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_computation.html).

**Public Member Functions**

- `MComputation ()`
- `virtual ~MComputation ()`
- `void beginComputation (bool showProgressBar=false, bool isInterruptable=true, bool useWaitCursor=true)`
- `bool isInterruptRequested ()`
- `void endComputation ()`
- `MStatus setProgressRange (const int minValue, const int maxValue)`
- `int progressMin (MStatus *ReturnStatus=NULL) const`
- `int progressMax (MStatus *ReturnStatus=NULL) const`
- `MStatus setProgress (const int amount)`
- `int progress (MStatus *ReturnStatus=NULL) const`
- `MStatus setProgressStatus (const MString &)`

**Static Public Member Functions**

- `static const char * className ()`

### MConditionMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_condition_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_condition_message.html).

**Static Public Member Functions**

- `static MCallbackId addConditionCallback (const MString &conditionName, MMessage::MStateFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MStatus getConditionNames (MStringArray &names)`
- `static bool getConditionState (const MString &name, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MConnectDisconnectAttrEdit

Module `OpenMaya`. Inherits `MEdit`. Source: [class_m_connect_disconnect_attr_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_connect_disconnect_attr_edit.html).

**Public Member Functions**

- `MPlug srcPlug (MStatus *ReturnStatus=NULL) const`
- `MPlug dstPlug (MStatus *ReturnStatus=NULL) const`
- `MString srcPlugName (MStatus *ReturnStatus=NULL) const`
- `MString dstPlugName (MStatus *ReturnStatus=NULL) const`
- `bool isConnection () const`
- `EditType editType (MStatus *ReturnStatus=NULL) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MContainerMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_container_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_container_message.html).

**Static Public Member Functions**

- `static MCallbackId addPublishAttrCallback (MMessage::MNodeStringBoolFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addBoundAttrCallback (MMessage::MNodeStringBoolFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MCurveAttribute

Module `OpenMaya`. Source: [class_m_curve_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_curve_attribute.html).

**Public Member Functions**

- `MCurveAttribute ()`
- `MCurveAttribute (const MCurveAttribute &other)`
- `MCurveAttribute (const MPlug &obj, MStatus *ReturnStatus=NULL)`
- `MCurveAttribute (const MObject &node, const MObject &attr, MStatus *ReturnStatus=NULL)`
- `MCurveAttribute & operator= (const MCurveAttribute &other)`
- `~MCurveAttribute ()`
- `unsigned int getNumEntries (MStatus *returnStatus=NULL)`
- `void getEntries (MIntArray &indexes, MFloatArray &positions, MFloatArray &values, MStatus *returnStatus=NULL)`
- `void addEntries (MFloatArray &positions, MFloatArray &values, MStatus *returnStatus=NULL)`
- `void deleteEntries (MIntArray &indexes, MStatus *returnStatus=NULL)`
- `void setValueAtIndex (float value, unsigned int index, MStatus *returnStatus=NULL)`
- `void setPositionAtIndex (float position, unsigned int index, MStatus *returnStatus=NULL)`
- `void getValuesAtPositions (const MFloatArray &positions, MFloatArray &values, MStatus *returnStatus=NULL)`
- `void getValueAtPosition (float position, float &value, MStatus *returnStatus=NULL)`
- `void sampleValueCurve (unsigned int numberOfSamples, MFloatArray &values, MStatus *returnStatus=NULL)`
- `bool hasIndex (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus sort (bool ascending=true)`
- `MStatus pack ()`
- `MStatus setCurve (const MFloatArray &values, const MFloatArray &positions)`

**Static Public Member Functions**

- `static MObject createCurveAttr (const MString &attrLongName, const MString &attrShortName, MStatus *ReturnStatus=NULL)`
- `static MObject createCurve (const MString &attrLongName, const MString &attrShortName, const MObject &node, const MFloatArray &values, const MFloatArray &positions, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MCustomEvaluatorClusterNode

Module `OpenMaya`. Source: [class_m_custom_evaluator_cluster_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_custom_evaluator_cluster_node.html).

**Public Types**

- `enum SchedulingType { , kSerial, kGloballySerial, kUntrusted, kLast }`

**Public Member Functions**

- `MPxCustomEvaluator evaluator (MStatus *ReturnStatus) const`
- `MEvaluationGraph ownerGraph (MStatus *ReturnStatus) const`
- `MCustomEvaluatorClusterNode::SchedulingType schedulingType (MStatus *ReturnStatus=NULL) const`
- `void evaluate (MStatus *ReturnStatus=NULL) const`
- `void evaluateNode (const MEvaluationNode &node, MStatus *ReturnStatus) const`
- `MObject asMObject (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MDAGDrawOverrideInfo

Module `OpenMaya`. Source: [class_m_d_a_g_draw_override_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d_a_g_draw_override_info.html).

**Public Types**

- `enum DrawOverrideDisplayType { kDisplayTypeNormal, kDisplayTypeReference, kDisplayTypeTemplate }`
- `enum DrawOverrideLOD { kLODFull, kLODBoundingBox }`

### MDGContext

Module `OpenMaya`. Source: [class_m_d_g_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d_g_context.html).

**Public Member Functions**

- `MDGContext ()`
- `MDGContext (const MTime &when)`
- `MDGContext (const MDGContext &in)`
- `~MDGContext ()`
- `bool isNormal (MStatus *ReturnStatus=NULL) const`
- `MStatus getTime (MTime &) const`
- `MDGContext & operator= (const MDGContext &other)`
- `bool isCurrent (MStatus *ReturnStatus=NULL) const`
- `const MDGContext makeCurrent (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`
- `static const MDGContext current ()`

### MDGContextGuard

Module `OpenMaya`. Source: [class_m_d_g_context_guard.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d_g_context_guard.html).

**Public Member Functions**

- `~MDGContextGuard ()`
- `MDGContextGuard (const MDGContext &tempContext)`

### MDGMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_d_g_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d_g_message.html).

**Static Public Member Functions**

- `static MCallbackId addTimeChangeCallback (MMessage::MTimeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDelayedTimeChangeCallback (MMessage::MTimeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDelayedTimeChangeRunupCallback (MMessage::MTimeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addForceUpdateCallback (MMessage::MTimeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeAddedCallback (MMessage::MNodeFunction func, const MString &nodeType=kDefaultNodeType, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeRemovedCallback (MMessage::MNodeFunction func, const MString &nodeType=kDefaultNodeType, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addConnectionCallback (MMessage::MPlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addPreConnectionCallback (MMessage::MPlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeChangeUuidCheckCallback (MMessage::MCheckNodeUuidFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`

### MDGModifier

Module `OpenMaya`. Source: [class_m_d_g_modifier.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d_g_modifier.html).

**Public Member Functions**

- `MDGModifier ()`
- `virtual ~MDGModifier ()`
- `MObject createNode (const MTypeId &typeId, MStatus *ReturnStatus=NULL)`
- `MObject createNode (const MString &type, MStatus *ReturnStatus=NULL)`
- `MStatus deleteNode (const MObject &node)`
- `MStatus deleteNode (const MObject &node, bool includeParents)`
- `MStatus renameNode (const MObject &node, const MString &newName)`
- `MStatus setNodeLockState (const MObject &node, bool newState)`
- `MStatus connect (const MObject &sourceNode, const MObject &sourceAttr, const MObject &destNode, const MObject &destAttr)`
- `MStatus disconnect (const MObject &sourceNode, const MObject &sourceAttr, const MObject &destNode, const MObject &destAttr)`
- `MStatus connect (const MPlug &source, const MPlug &dest)`
- `MStatus disconnect (const MPlug &source, const MPlug &dest)`
- `MStatus addAttribute (const MObject &node, const MObject &attribute)`
- `MStatus removeAttribute (const MObject &node, const MObject &attribute)`
- `MStatus renameAttribute (const MObject &node, const MObject &attribute, const MString &shortName, const MString &longName)`
- `MStatus addExtensionAttribute (const MNodeClass &nodeClass, const MObject &attribute)`
- `MStatus removeExtensionAttribute (const MNodeClass &nodeClass, const MObject &attribute)`
- `MStatus removeExtensionAttributeIfUnset (const MNodeClass &nodeClass, const MObject &attribute)`
- `MStatus linkExtensionAttributeToPlugin (const MObject &plugin, const MObject &attribute)`
- `MStatus unlinkExtensionAttributeFromPlugin (const MObject &mPlugin, const MObject &mAttribute)`
- `MStatus commandToExecute (const MString &command)`
- `MStatus pythonCommandToExecute (const MString &command)`
- `MStatus newPlugValue (const MPlug &plug, MObject &plugValue)`
- `MStatus newPlugValueBool (const MPlug &plug, bool plugValue)`
- `MStatus newPlugValueChar (const MPlug &plug, char plugValue)`
- `MStatus newPlugValueDouble (const MPlug &plug, double plugValue)`
- `MStatus newPlugValueFloat (const MPlug &plug, float plugValue)`
- `MStatus newPlugValueInt (const MPlug &plug, int plugValue)`
- `MStatus newPlugValueInt64 (const MPlug &plug, MInt64 plugValue)`
- `MStatus newPlugValueMAngle (const MPlug &plug, const MAngle &plugValue)`
- `MStatus newPlugValueMDistance (const MPlug &plug, const MDistance &plugValue)`
- `MStatus newPlugValueMTime (const MPlug &plug, const MTime &plugValue)`
- `MStatus newPlugValueShort (const MPlug &plug, short plugValue)`
- `MStatus newPlugValueString (const MPlug &plug, const MString &plugValue)`
- `MStatus removeMultiInstance (const MPlug &plug, bool breakConnections)`
- `MStatus setMetadata (const MObject &node, const adsk::Data::Associations &metadata)`
- `MStatus deleteMetadata (const MObject &node)`
- `MStatus doIt ()`
- `MStatus undoIt ()`
- `MStatus addAttribute (const MObject &node, const MObject &attribute, MFnDependencyNode::MAttrClass type)`
- `MStatus removeAttribute (const MObject &node, const MObject &attribute, MFnDependencyNode::MAttrClass type)`

**Static Public Member Functions**

- `static const char * className ()`

### MDagMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_dag_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dag_message.html).

**Public Types**

- `enum DagMessage { kInvalidMsg = -1, kParentAdded, kParentRemoved, kChildAdded, kChildRemoved, kChildReordered, kInstanceAdded, kInstanceRemoved, kLast }`
- `enum MatrixModifiedFlags { kScaleX = 1<<0, kScaleY = 1<<1, kScaleZ = 1<<2, kShearXY = 1<<3, kShearXZ = 1<<4, kShearYZ = 1<<5, kRotateX = 1<<6, kRotateY = 1<<7, kRotateZ = 1<<8, kTranslateX = 1<<9, kTranslateY = 1<<10, kTranslateZ = 1<<11, kScalePivotX = 1<<12, kScalePivotY = 1<<13, kScalePivotZ = 1<<14, kRotatePivotX = 1<<15, kRotatePivotY = 1<<16, kRotatePivotZ = 1<<17, kScaleTransX = 1<<18, kScaleTransY = 1<<19, kScaleTransZ = 1<<20, kRotateTransX = 1<<21, kRotateTransY = 1<<22, kRotateTransZ = 1<<23, kRotateOrientX = 1<<24, kRotateOrientY = 1<<25, kRotateOrientZ = 1<<26, kRotateOrder = 1<<27, kAll = (1<<28)-1, kScale = kScaleX | kScaleY | kScaleZ, kShear = kShearXY | kShearXZ | kShearYZ, kRotation = kRotateX | kRotateY | kRotateZ, kTranslation = kTranslateX | kTranslateY | kTranslateZ, kScalePivot = kScalePivotX | kScalePivotY | kScalePivotZ, kRotatePivot = kRotatePivotX | kRotatePivotY | kRotatePivotZ, kScalePivotTrans =kScaleTransX | kScaleTransY | kScaleTransZ, kRotatePivotTrans =kRotateTransX | kRotateTransY | kRotateTransZ, kRotateOrient = kRotateOrientX | kRotateOrientY | kRotateOrientZ }`
- `typedef void(* MMessageParentChildFunction) (MDagMessage::DagMessage msgType, MDagPath &child, MDagPath &parent, void *clientData)`
- `typedef void(* MWorldMatrixModifiedFunction) (MObject &transformNode, MDagMessage::MatrixModifiedFlags &modified, void *clientData)`

**Static Public Member Functions**

- `static MCallbackId addParentAddedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addParentAddedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addParentRemovedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addParentRemovedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildAddedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildAddedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildRemovedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildRemovedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildReorderedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildReorderedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDagCallback (DagMessage msgType, MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDagDagPathCallback (MDagPath &node, DagMessage msgType, MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAllDagChangesCallback (MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAllDagChangesDagPathCallback (MDagPath &node, MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceAddedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceAddedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceRemovedCallback (MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceRemovedDagPathCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addMatrixModifiedCallback (MDagPath &node, MDagMessage::MWorldMatrixModifiedFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addWorldMatrixModifiedCallback (MDagPath &node, MDagMessage::MWorldMatrixModifiedFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`
- `static MCallbackId addParentAddedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addParentRemovedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildAddedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildRemovedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addChildReorderedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceAddedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addInstanceRemovedCallback (MDagPath &node, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDagCallback (DagMessage msgType, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDagCallback (MDagPath &node, DagMessage msgType, MMessage::MParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addDagCallback (MDagPath &node, DagMessage msgType, MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAllDagChangesCallback (MDagPath &node, MDagMessage::MMessageParentChildFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`

### MDagModifier

Module `OpenMaya`. Inherits `MDGModifier`. Source: [class_m_dag_modifier.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dag_modifier.html).

**Public Member Functions**

- `MDagModifier ()`
- `~MDagModifier () override`
- `MObject createNode (const MTypeId &typeId, const MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject createNode (const MString &type, const MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus reparentNode (const MObject &node, const MObject &newParent=MObject::kNullObj)`

**Static Public Member Functions**

- `static const char * className ()`

### MDagPath

Module `OpenMaya`. Source: [class_m_dag_path.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dag_path.html).

**Public Member Functions**

- `MDagPath ()`
- `MDagPath (const MDagPath &src)`
- `virtual ~MDagPath ()`
- `MStatus getAllPathsBelow (MDagPathArray &pathArray)`
- `bool hasFn (MFn::Type type, MStatus *ReturnStatus=nullptr) const`
- `MFn::Type apiType (MStatus *ReturnStatus=nullptr) const`
- `bool isValid (MStatus *ReturnStatus=nullptr) const`
- `MObject node (MStatus *ReturnStatus=nullptr) const`
- `MObject transform (MStatus *ReturnStatus=nullptr) const`
- `unsigned int length (MStatus *ReturnStatus=nullptr) const`
- `MStatus extendToShape ()`
- `MStatus extendToShapeDirectlyBelow (unsigned int)`
- `MStatus numberOfShapesDirectlyBelow (unsigned int &) const`
- `MStatus push (const MObject &child)`
- `MStatus pop (unsigned int num=1)`
- `unsigned int childCount (MStatus *ReturnStatus=nullptr) const`
- `MObject child (unsigned int i, MStatus *ReturnStatus=nullptr) const`
- `MMatrix inclusiveMatrix (MStatus *ReturnStatus=nullptr) const`
- `MMatrix exclusiveMatrix (MStatus *ReturnStatus=nullptr) const`
- `MMatrix inclusiveMatrixInverse (MStatus *ReturnStatus=nullptr) const`
- `MMatrix exclusiveMatrixInverse (MStatus *ReturnStatus=nullptr) const`
- `MDagPath & operator= (const MDagPath &src)`
- `bool operator== (const MDagPath &src) const`
- `MStatus set (const MDagPath &src)`
- `unsigned int pathCount (MStatus *ReturnStatus=nullptr) const`
- `MStatus getPath (MDagPath &path, unsigned int i=0) const`
- `MString fullPathName (MStatus *ReturnStatus=nullptr) const`
- `MString partialPathName (MStatus *ReturnStatus=nullptr) const`
- `bool isInstanced (MStatus *ReturnStatus=nullptr) const`
- `unsigned int instanceNumber (MStatus *ReturnStatus=nullptr) const`
- `bool isVisible (MStatus *ReturnStatus=nullptr) const`
- `bool isTemplated (MStatus *ReturnStatus=nullptr) const`
- `MDAGDrawOverrideInfo getDrawOverrideInfo () const`

**Static Public Member Functions**

- `static MStatus getAllPathsTo (const MObject &node, MDagPathArray &pathArray)`
- `static MStatus getAPathTo (const MObject &node, MDagPath &path)`
- `static MStatus matchTransform (const MDagPath &source, const MDagPath &target, const MDagPath &relative, MTransformationMatrix &transformationMatrix, bool preserveOffsetParentMatrix=true, bool preservePivot=true, bool preservePivotOffset=false)`
- `static const char * className ()`
- `static MDagPath getAPathTo (const MObject &node, MStatus *ReturnStatus=nullptr)`

### MDagPathArray

Module `OpenMaya`. Source: [class_m_dag_path_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dag_path_array.html).

**Public Member Functions**

- `MDagPathArray ()`
- `MDagPathArray (const MDagPathArray &other)`
- `MDagPathArray (unsigned int initializeSize, const MDagPath &initialValue)`
- `~MDagPathArray ()`
- `const MDagPath & operator[] (unsigned int index) const`
- `MDagPathArray & operator= (const MDagPathArray &other)`
- `MStatus set (const MDagPath &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MDagPath &element, unsigned int index)`
- `MStatus append (const MDagPath &element)`
- `MStatus copy (const MDagPathArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MDagPath & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MDagPathArray &array)`

### MDataBlock

Module `OpenMaya`. Source: [class_m_data_block.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_data_block.html).

**Public Member Functions**

- `MDataBlock (const MDataBlock &in)`
- `virtual ~MDataBlock ()`
- `MDataHandle inputValue (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MDataHandle inputValue (const MObject &attribute, MStatus *ReturnStatus=NULL)`
- `MDataHandle outputValue (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MDataHandle outputValue (const MObject &attribute, MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle inputArrayValue (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle inputArrayValue (const MObject &attribute, MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle outputArrayValue (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MArrayDataHandle outputArrayValue (const MObject &attribute, MStatus *ReturnStatus=NULL)`
- `MStatus setClean (const MPlug &plug)`
- `MStatus setClean (const MObject &attribute)`
- `bool isClean (const MPlug &plug)`
- `bool isClean (const MObject &attribute, MStatus *ReturnStatus=NULL)`
- `MDGContext context (MStatus *ReturnStatus=NULL)`
- `MStatus setContext (const MDGContext &ctx)`

**Static Public Member Functions**

- `static const char * className ()`

### MDataHandle

Module `OpenMaya`. Source: [class_m_data_handle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_data_handle.html).

**Public Member Functions**

- `MDataHandle ()`
- `bool isNumeric () const`
- `bool isGeneric (bool &isNumeric, bool &isNull) const`
- `MFnNumericData::Type numericType () const`
- `MFnData::Type type () const`
- `MTypeId typeId () const`
- `MStatus acceptedTypeIds (MUintArray &) const`
- `MObject data ()`
- `MStatus copy (const MDataHandle &src)`
- `MStatus copyWritable (const MDataHandle &src)`
- `MObject attribute ()`
- `void setClean ()`
- `bool & asBool () const`
- `char & asChar () const`
- `unsigned char & asUChar () const`
- `short & asShort () const`
- `int & asLong () const`
- `int & asInt () const`
- `MInt64 & asInt64 () const`
- `void *& asAddr () const`
- `float & asFloat () const`
- `double & asDouble () const`
- `MDistance asDistance () const`
- `MAngle asAngle () const`
- `MTime asTime () const`
- `short2 & asShort2 () const`
- `long2 & asLong2 () const`
- `int2 & asInt2 () const`
- `float2 & asFloat2 () const`
- `double2 & asDouble2 () const`
- `short3 & asShort3 () const`
- `long3 & asLong3 () const`
- `int3 & asInt3 () const`
- `float3 & asFloat3 () const`
- `double3 & asDouble3 () const`
- `double4 & asDouble4 () const`
- `MVector & asVector () const`
- `MFloatVector & asFloatVector () const`
- `MMatrix & asMatrix () const`
- `MFloatMatrix & asFloatMatrix () const`
- `MString & asString () const`
- `MObject asNurbsCurve () const`
- `MObject asNurbsSurface () const`
- `MObject asMesh () const`
- `MObject asSubdSurface () const`
- `MObject asNurbsCurveTransformed () const`
- `MObject asNurbsSurfaceTransformed () const`
- `MObject asMeshTransformed () const`
- `MObject asSubdSurfaceTransformed () const`
- `const MMatrix & geometryTransformMatrix () const`
- `MPxData * asPluginData () const`
- `void set (bool)`
- `void set (char)`
- `void set (short)`
- `void set (int)`
- `void set (MInt64)`
- `void set (float)`
- `void set (double)`
- `void set (const MMatrix &)`
- `void set (const MFloatMatrix &)`
- `void set (const MVector &)`
- `void set (const MFloatVector &)`
- `void set (const MDistance &)`
- `void set (const MAngle &)`
- `void set (const MTime &)`
- `void set (short, short)`
- `void set (int, int)`
- `void set (float, float)`
- `void set (double, double)`
- `void set (short, short, short)`
- `void set (int, int, int)`
- `void set (float, float, float)`
- `void set (double, double, double)`
- `void set (const MString &)`
- `MStatus set (const MObject &data)`
- `MStatus set (MPxData *data)`
- `void setBool (bool)`
- `void setChar (char)`
- `void setShort (short)`
- `void setInt (int)`
- `void setInt64 (MInt64)`
- `void setFloat (float)`
- `void setDouble (double)`
- `void setMMatrix (const MMatrix &)`
- `void setMFloatMatrix (const MFloatMatrix &)`
- `void setMVector (const MVector &)`
- `void setMFloatVector (const MFloatVector &)`
- `void setMDistance (const MDistance &)`
- `void setMAngle (const MAngle &)`
- `void setMTime (const MTime &)`
- `void set2Short (short, short)`
- `void set2Int (int, int)`
- `void set2Float (float, float)`
- `void set2Double (double, double)`
- `void set3Short (short, short, short)`
- `void set3Int (int, int, int)`
- `void set3Float (float, float, float)`
- `void set3Double (double, double, double)`
- `void set4Double (double, double, double, double)`
- `void setString (const MString &)`
- `MStatus setMObject (const MObject &data)`
- `MStatus setMPxData (MPxData *data)`
- `bool asGenericBool () const`
- `unsigned char asGenericChar () const`
- `double asGenericDouble () const`
- `float asGenericFloat () const`
- `short asGenericShort () const`
- `int asGenericInt () const`
- `MInt64 asGenericInt64 () const`
- `void setGenericBool (bool value, bool force)`
- `void setGenericChar (unsigned char value, bool force)`
- `void setGenericDouble (double value, bool force)`
- `void setGenericFloat (float value, bool force)`
- `void setGenericShort (short value, bool force)`
- `void setGenericInt (int value, bool force)`
- `void setGenericInt64 (MInt64 value, bool force)`
- `MDataHandle child (const MPlug &plug)`
- `MDataHandle child (const MObject &attribute)`
- `MDataBlock datablock (MStatus *ReturnStatus=NULL) const`
- `MDataHandle & operator= (const MDataHandle &other)`
- `MDataHandle (const MDataHandle &)`

**Static Public Member Functions**

- `static const char * className ()`

### MDistance

Module `OpenMaya`. Source: [class_m_distance.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_distance.html).

**Public Types**

- `enum Unit { , kInches, kFeet, kYards, kMiles, kMillimeters, kCentimeters, kKilometers, kMeters }`

**Public Member Functions**

- `MDistance ()`
- `MDistance (double value, Unit unitSystem=kCentimeters)`
- `MDistance (const MDistance &src)`
- `~MDistance ()`
- `MDistance & operator= (const MDistance &src)`
- `Unit unit () const`
- `double value () const`
- `MStatus setUnit (Unit newUnit)`
- `MStatus setValue (double newValue)`
- `double as (Unit newUnit, MStatus *ReturnStatus=NULL) const`
- `double asUnits (Unit newUnit, MStatus *ReturnStatus=NULL) const`
- `double asInches () const`
- `double asFeet () const`
- `double asYards () const`
- `double asMiles () const`
- `double asMillimeters () const`
- `double asCentimeters () const`
- `double asKilometers () const`
- `double asMeters () const`

**Static Public Member Functions**

- `static Unit uiUnit ()`
- `static MStatus setUIUnit (Unit newUnit)`
- `static Unit internalUnit ()`
- `static double internalToUI (double internalValue)`
- `static double uiToInternal (double uiValue)`
- `static const char * className ()`
- `static MStatus setInternalUnit (Unit internalUnit)`

### MDoubleArray

Module `OpenMaya`. Source: [class_m_double_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_double_array.html).

**Public Member Functions**

- `MDoubleArray ()`
- `MDoubleArray (const MDoubleArray &other)`
- `MDoubleArray (const double src[], unsigned int count)`
- `MDoubleArray (const float src[], unsigned int count)`
- `MDoubleArray (unsigned int initialSize, double initialValue=0)`
- `~MDoubleArray ()`
- `MDoubleArray & operator= (const MDoubleArray &other)`
- `MStatus set (double element, unsigned int index)`
- `MStatus set (float element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (double element, unsigned int index)`
- `MStatus append (double element)`
- `MStatus copy (const MDoubleArray &source)`
- `MStatus clear ()`
- `MStatus get (double[]) const`
- `MStatus get (float[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `double & operator[] (unsigned int index)`
- `double operator[] (unsigned int index) const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MDoubleArray &array)`

### MEdit

Module `OpenMaya`. Source: [class_m_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_edit.html).

**Public Types**

- `enum EditType { kNullEdit, kSetAttrEdit, kConnectDisconnectEdit, kAddRemoveAttrEdit, kParentEdit, kFcurveEdit }`

**Public Member Functions**

- `virtual EditType editType (MStatus *ReturnStatus=NULL) const`
- `MString getString (MStatus *ReturnStatus=NULL) const`
- `bool isApplied (MStatus *ReturnStatus=NULL) const`
- `bool isFailed (MStatus *ReturnStatus=NULL) const`
- `bool isTopLevel (MStatus *ReturnStatus=NULL) const`
- `MStatus setApplied (bool isApplied)`
- `MStatus setFailed (bool failed)`
- `bool hasEditData () const`
- `bool matches (const MPxEditData *editData) const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MEulerRotation

Module `OpenMaya`. Source: [class_m_euler_rotation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_euler_rotation.html).

**Public Types**

- `enum RotationOrder { kXYZ, kYZX, kZXY, kXZY, kYXZ, kZYX }`

**Public Member Functions**

- `MEulerRotation ()`
- `MEulerRotation (const MEulerRotation &src)`
- `MEulerRotation (const MVector &v, RotationOrder ord=kXYZ)`
- `MEulerRotation (double xx, double yy, double zz, RotationOrder ord=kXYZ)`
- `~MEulerRotation ()`
- `MEulerRotation & operator= (const MEulerRotation &src)`
- `MEulerRotation & operator= (const MQuaternion &q)`
- `MEulerRotation & operator= (const MMatrix &m)`
- `MEulerRotation & operator= (const MVector &v)`
- `MEulerRotation & setValue (const MVector &v, RotationOrder ord=kXYZ)`
- `MEulerRotation & setValue (double xx, double yy, double zz, RotationOrder ord=kXYZ)`
- `MQuaternion asQuaternion () const`
- `MMatrix asMatrix () const`
- `MVector asVector () const`
- `double operator[] (unsigned int i) const`
- `MEulerRotation operator+ (const MEulerRotation &other) const`
- `MEulerRotation & operator+= (const MEulerRotation &other)`
- `MEulerRotation operator- (const MEulerRotation &other) const`
- `MEulerRotation & operator-= (const MEulerRotation &other)`
- `MEulerRotation operator- () const`
- `MEulerRotation operator* (const MEulerRotation &other) const`
- `MEulerRotation operator* (const MQuaternion &other) const`
- `MEulerRotation operator* (double other) const`
- `MEulerRotation & operator*= (const MEulerRotation &other)`
- `MEulerRotation & operator*= (const MQuaternion &other)`
- `MEulerRotation & operator*= (double other)`
- `bool operator== (const MEulerRotation &other) const`
- `bool operator!= (const MEulerRotation &other) const`
- `bool isEquivalent (const MEulerRotation &other, double tolerance=kEulerRotationEpsilon) const`
- `bool isZero (double tolerance=kEulerRotationEpsilon) const`
- `MEulerRotation & incrementalRotateBy (const MVector &axis, double angle)`
- `MEulerRotation inverse () const`
- `MEulerRotation & invertIt ()`
- `MEulerRotation reorder (RotationOrder ord) const`
- `MEulerRotation & reorderIt (RotationOrder ord)`
- `MEulerRotation bound () const`
- `MEulerRotation & boundIt (const MEulerRotation &src)`
- `MEulerRotation & boundIt ()`
- `MEulerRotation alternateSolution () const`
- `MEulerRotation & setToAlternateSolution (const MEulerRotation &src)`
- `MEulerRotation & setToAlternateSolution ()`
- `MEulerRotation closestSolution (const MEulerRotation &dst) const`
- `MEulerRotation & setToClosestSolution (const MEulerRotation &src, const MEulerRotation &dst)`
- `MEulerRotation & setToClosestSolution (const MEulerRotation &dst)`
- `MEulerRotation closestCut (const MEulerRotation &dst) const`
- `MEulerRotation & setToClosestCut (const MEulerRotation &src, const MEulerRotation &dst)`
- `MEulerRotation & setToClosestCut (const MEulerRotation &dst)`
- `double & operator[] (unsigned int i)`

**Static Public Member Functions**

- `static MEulerRotation decompose (const MMatrix &matrix, RotationOrder ord)`
- `static MEulerRotation bound (const MEulerRotation &src)`
- `static MEulerRotation alternateSolution (const MEulerRotation &src)`
- `static MEulerRotation closestSolution (const MEulerRotation &src, const MEulerRotation &dst)`
- `static MEulerRotation closestCut (const MEulerRotation &src, const MEulerRotation &dst)`

**Public Attributes**

- `double x`
- `double y`
- `double z`
- `RotationOrder order`

**Static Public Attributes**

- `static const MEulerRotation identity`

**Friends**

- `OPENMAYA_EXPORT MEulerRotation operator* (double scale, const MEulerRotation &other)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MEulerRotation &rotation)`

### MEvaluationGraph

Module `OpenMaya`. Source: [class_m_evaluation_graph.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_evaluation_graph.html).

**Public Member Functions**

- `virtual ~MEvaluationGraph ()`
- `MDGContext context (MStatus *ReturnStatus) const`
- `bool partialEvaluation () const`
- `MEvaluationNode rootNode () const`

**Static Public Member Functions**

- `static MEvaluationGraph ownerGraph (const MEvaluationNode &evalNode)`
- `static const char * className ()`

### MEvaluationManager

Module `OpenMaya`. Source: [class_m_evaluation_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_evaluation_manager.html).

**Static Public Member Functions**

- `static bool graphConstructionActive ()`
- `static bool evaluationManagerActive (const MDGContext &context)`
- `static bool evaluationInExecution (const MDGContext &context)`
- `static const char * className ()`

### MEvaluationNode

Module `OpenMaya`. Source: [class_m_evaluation_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_evaluation_node.html).

**Public Member Functions**

- `~MEvaluationNode ()`
- `MEvaluationNode (const MEvaluationNode &evalNode)`
- `int parentCount () const`
- `MEvaluationNode parent (int index) const`
- `int childCount () const`
- `MEvaluationNode child (int index) const`
- `MEvaluationNodeIterator iterator (MStatus *ReturnStatus=nullptr) const`
- `bool dirtyPlugExists (const MObject &attribute, MStatus *ReturnStatus=nullptr) const`
- `MPlug dirtyPlug (const MObject &attribute, MStatus *ReturnStatus=nullptr) const`
- `MObject dependencyNode (MStatus *ReturnStatus=nullptr) const`
- `MDataBlock datablock (MStatus *ReturnStatus=nullptr) const`
- `void connect (MEvaluationNode &child, MStatus *ReturnStatus=nullptr)`
- `void skipEvaluation (const MObject &attribute, bool allowSingleDownstreamDependency=false, MStatus *ReturnStatus=nullptr) const`
- `bool skippingEvaluation (const MObject &attribute, MStatus *ReturnStatus=nullptr) const`

**Static Public Member Functions**

- `static const char * className ()`

### MEvaluationNodeIterator

Module `OpenMaya`. Source: [class_m_evaluation_node_iterator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_evaluation_node_iterator.html).

**Public Member Functions**

- `~MEvaluationNodeIterator ()`
- `MEvaluationNodeIterator (const MEvaluationNode &node, MStatus *ReturnStatus=NULL)`
- `MEvaluationNodeIterator (const MEvaluationNodeIterator &it)`
- `MEvaluationNodeIterator & operator= (MEvaluationNodeIterator &it)`
- `MPlug plug ()`
- `bool isDone () const`
- `void next ()`
- `void reset ()`

**Static Public Member Functions**

- `static const char * className ()`

### MEventMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_event_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_event_message.html).

**Static Public Member Functions**

- `static MCallbackId addEventCallback (const MString &event, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MStatus getEventNames (MStringArray &names)`
- `static const char * className ()`

### MExternalContentInfoTable

Module `OpenMaya`. Source: [class_m_external_content_info_table.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_content_info_table.html).

**Public Member Functions**

- `MExternalContentInfoTable ()`
- `~MExternalContentInfoTable ()`
- `MStatus addResolvedEntry (const MString &key, const MString &unresolvedLocation, const MString &resolvedLocation, const MStringArray &roles)`
- `MStatus addResolvedEntry (const MString &key, const MString &unresolvedLocation, const MString &resolvedLocation, const MString &contextNodeFullName, const MStringArray &roles)`
- `MStatus addUnresolvedEntry (const MString &key, const MString &unresolvedLocation, const MStringArray &roles=MStringArray())`
- `MStatus addUnresolvedEntry (const MString &key, const MString &unresolvedLocation, const MString &contextNodeFullName, const MStringArray &roles=MStringArray())`
- `MStatus getInfo (const MString &key, MString &unresolvedLocation, MString &resolvedLocation, MStringArray &roles) const`
- `MStatus getInfo (const MString &key, MString &unresolvedLocation, MString &resolvedLocation, MString &contextNodeFullName, MStringArray &roles) const`
- `MStatus getEntry (unsigned int index, MString &key, MString &unresolvedLocation, MString &resolvedLocation, MStringArray &roles) const`
- `MStatus getEntry (unsigned int index, MString &key, MString &unresolvedLocation, MString &resolvedLocation, MString &contextNodeFullName, MStringArray &roles) const`
- `unsigned int length () const`
- `MExternalContentInfoTable (void *data)`

### MExternalContentLocationTable

Module `OpenMaya`. Source: [class_m_external_content_location_table.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_content_location_table.html).

**Public Member Functions**

- `MExternalContentLocationTable ()`
- `~MExternalContentLocationTable ()`
- `MStatus addEntry (const MString &key, const MString &location)`
- `MStatus getEntry (unsigned int index, MString &key, MString &location) const`
- `MStatus getLocation (const MString &key, MString &location) const`
- `unsigned int length () const`
- `MExternalContentLocationTable (const void *const data)`

### MFcurveEdit

Module `OpenMaya`. Inherits `MEdit`. Source: [class_m_fcurve_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fcurve_edit.html).

**Public Member Functions**

- `MObject fcurve (MStatus *ReturnStatus=NULL) const`
- `MString fcurveName (MStatus *ReturnStatus=NULL) const`
- `EditType editType (MStatus *ReturnStatus=NULL) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MFileIO

Module `OpenMaya`. Source: [class_m_file_i_o.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_file_i_o.html).

**Public Types**

- `enum ReferenceMode { kLoadDefault, kLoadAllReferences, kLoadNoReferences }`
- `enum MayaFileFormatVersion`
- `enum NamespaceMergeMethod { kRemoveNamespaceIfEmpty, kMergeNamespaceWithRootNamespace, kMergeNamespaceWithParentNamespace, kForceDeleteNamespaceContent }`
- `enum ExportType { kExportTypeAll, kExportTypeSelected, kExportTypeAnim, kExportTypeAnimFromReference, kExportTypeAsReference, kExportTypeEdits, kExportTypePrefObjects }`

**Public Member Functions**

- `MFileIO ()`
- `MFileIO (const MString &fileName)`
- `virtual ~MFileIO ()`

**Static Public Member Functions**

- `static MString currentFile ()`
- `static MStatus setCurrentFile (const MString &fileName)`
- `static MString fileType ()`
- `static MStatus getFileTypes (MStringArray &types)`
- `static MStatus newFile (bool force=false)`
- `static MStatus open (const MString &fileName, const char *type=NULL, bool force=false, ReferenceMode refMode=kLoadDefault, bool ignoreVersion=false)`
- `static MStatus save (bool force=false)`
- `static MStatus saveAs (const MString &fileName, const char *type=NULL, bool force=false)`
- `static MStatus importFile (const MString &fileName, const char *type=NULL, bool preserveReferences=false, const char *nameSpace=NULL, bool ignoreVersion=false)`
- `static MStatus exportSelected (const MString &fileName, const char *type=NULL, bool preserveReferences=false)`
- `static MStatus exportAll (const MString &fileName, const char *type=NULL, bool preserveReferences=false)`
- `static MStatus exportAnimFromReference (const MString &fileName, const char *type=NULL)`
- `static MStatus exportAnimFromReference (const MString &fileName, const MString &refNodeName, const char *type=NULL)`
- `static MStatus exportSelectedAnimFromReference (const MString &fileName, const char *type=NULL)`
- `static MStatus exportSelectedAnimFromReference (const MString &fileName, const MString &refNodeName, const char *type=NULL)`
- `static MStatus exportAnim (const MString &fileName, const char *type=NULL)`
- `static MStatus exportSelectedAnim (const MString &fileName, const char *type=NULL)`
- `static MStatus exportAsReference (const MString &fileName, const char *type=NULL)`
- `static MStatus getReferences (MStringArray &references, bool wantUnresolvedNames=false)`
- `static MStatus getFiles (MStringArray &files, bool wantUnresolvedNames=false, bool wantReferencesOnly=false)`
- `static MStatus getReferenceNodes (const MString &fileName, MStringArray &nodes)`
- `static MStatus getReferenceNodes (const MString &fileName, MStringArray &nodes, bool dagPath)`
- `static MStatus getReferenceNodes (const MString &fileName, MSelectionList &list)`
- `static MStatus getReferenceConnectionsMade (const MString &fileName, MStringArray &connections)`
- `static MStatus getReferenceConnectionsBroken (const MString &fileName, MStringArray &connections)`
- `static MStatus getReferenceConnectionsBroken (const MString &fileName, MPlugArray &srcPlugs, MPlugArray &destPlugs)`
- `static MStatus getReferenceConnectionsBroken (const MObject &node, MStringArray &connections, bool append=false, bool removeDuplicates=true)`
- `static MStatus getReferenceConnectionsBroken (const MObject &node, MPlugArray &srcPlugs, MPlugArray &destPlugs, bool append=false, bool removeDuplicates=true)`
- `static MStatus reference (const MString &fileName, bool deferReference=false, bool lockReference=false, const MString &nameSpace="")`
- `static MStatus removeReference (const MString &fileName, NamespaceMergeMethod namespaceMergeMethod=kRemoveNamespaceIfEmpty)`
- `static bool isReadingFile ()`
- `static bool isWritingFile ()`
- `static bool isOpeningFile ()`
- `static bool isNewingFile ()`
- `static bool isSavingReference ()`
- `static bool mustRenameToSave (MStatus *ReturnStatus=NULL)`
- `static MStatus setMustRenameToSave (bool)`
- `static MString mustRenameToSaveMsg (MStatus *ReturnStatus=NULL)`
- `static MStatus setMustRenameToSaveMsg (const MString &)`
- `static MString beforeOpenFilename (MStatus *ReturnStatus=NULL)`
- `static MString beforeImportFilename (MStatus *ReturnStatus=NULL)`
- `static MString beforeSaveFilename (MStatus *ReturnStatus=NULL)`
- `static MString beforeExportFilename (MStatus *ReturnStatus=NULL)`
- `static MString beforeReferenceFilename (MStatus *ReturnStatus=NULL)`
- `static MPxFileTranslator * beforeOpenUserFileTranslator (MStatus *ReturnStatus=NULL)`
- `static MPxFileTranslator * beforeImportUserFileTranslator (MStatus *ReturnStatus=NULL)`
- `static MPxFileTranslator * beforeSaveUserFileTranslator (MStatus *ReturnStatus=NULL)`
- `static MPxFileTranslator * beforeExportUserFileTranslator (MStatus *ReturnStatus=NULL)`
- `static MPxFileTranslator * beforeReferenceUserFileTranslator (MStatus *ReturnStatus=NULL)`
- `static ExportType exportType (MStatus *ReturnStatus=NULL)`
- `static MString getLastTempFile (MStatus *ReturnStatus=NULL)`
- `static bool getErrorStatus (MStatus *ReturnStatus=NULL)`
- `static MStatus resetError ()`
- `static MStatus setError ()`
- `static MString loadReference (const MString &referenceFileName, MStatus *returnStatus=NULL)`
- `static MString loadReferenceByNode (const MString &referenceNodeName, MStatus *returnStatus=NULL)`
- `static MString loadReferenceByNode (MObject &referenceNode, MStatus *returnStatus=NULL)`
- `static MString unloadReference (const MString &referenceFileName, MStatus *returnStatus=NULL)`
- `static MString unloadReferenceByNode (const MString &referenceNodeName, MStatus *returnStatus=NULL)`
- `static MString unloadReferenceByNode (MObject &referenceNode, MStatus *returnStatus=NULL)`
- `static MString getReferenceFileByNode (MObject &referenceNode, MStatus *returnStatus=NULL)`
- `static MStatus cleanReference (const MString &referenceNodeName, const char *editCommand=NULL, MStatus *returnStatus=NULL)`
- `static MStatus saveReference (const MString &referenceNodeName, MStatus *returnStatus=NULL)`
- `static MString fileCurrentlyLoading ()`
- `static bool isImportingFile ()`
- `static bool isReferencingFile ()`
- `static int currentlyReadingFileVersion ()`
- `static int latestMayaFileVersion ()`
- `static const char * className ()`
- `static MStatus import (const MString &fileName, const char *type=NULL, bool preserveReferences=false, const char *nameSpace=NULL)`
- `static const adsk::Data::Associations * metadata (MStatus *ReturnStatus=NULL)`
- `static MStatus setMetadata (const adsk::Data::Associations &)`
- `static MStatus deleteMetadata ()`

### MFileObject

Module `OpenMaya`. Source: [class_m_file_object.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_file_object.html).

**Public Types**

- `enum MFileResolveMethod { kNone = (1<<0), kExact = (1<<1), kDirMap = (1<<2), kReferenceMappings = (1<<3), kRelative = (1<<4), kBaseName = (1<<5), kInputFile = kExact | kDirMap | kRelative | kBaseName, kInputReference = kInputFile | kReferenceMappings, kStrict = kExact | kDirMap }`

**Public Member Functions**

- `MFileObject ()`
- `MFileObject (const MFileObject &other)`
- `virtual ~MFileObject ()`
- `MFileObject & operator= (const MFileObject &other)`
- `MStatus setRawName (const MString &fileName)`
- `MStatus setRawPath (const MString &filePath)`
- `MStatus setRawFullName (const MString &fullFileName)`
- `MStatus setRawURI (const MString &uri)`
- `MStatus setRawURI (const MURI &uri)`
- `MStatus overrideResolvedFullName (const MString &fullFileName, bool reresolveType=false)`
- `MString rawName () const`
- `MString rawPath () const`
- `MString rawFullName () const`
- `MURI rawURI () const`
- `MString expandedPath () const`
- `MString expandedFullName () const`
- `MString resolvedName () const`
- `MString resolvedPath () const`
- `MString resolvedFullName () const`
- `unsigned int pathCount ()`
- `MString ithPath (unsigned int index)`
- `MString ithFullName (unsigned int index)`
- `bool exists ()`
- `bool exists (unsigned int index)`
- `bool isSet () const`
- `void setResolveMethod (MFileResolveMethod method)`
- `MFileResolveMethod resolveMethod () const`
- `MStatus setName (const MString &fileName)`
- `MStatus setFullName (const MString &fileName)`
- `MString name () const`
- `MString path () const`
- `MString fullName () const`

**Static Public Member Functions**

- `static bool isAbsolutePath (const MString &fileName)`
- `static MString getResolvedFullName (const MString &rawFullName)`
- `static MString getResolvedFullName (const MString &rawFullName, bool &exists, MFileObject::MFileResolveMethod method=MFileObject::kNone)`

### MFloatArray

Module `OpenMaya`. Source: [class_m_float_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_array.html).

**Public Member Functions**

- `MFloatArray ()`
- `MFloatArray (const MFloatArray &other)`
- `MFloatArray (const float src[], unsigned int count)`
- `MFloatArray (const double src[], unsigned int count)`
- `MFloatArray (unsigned int initialSize, float initialValue=0)`
- `~MFloatArray ()`
- `MFloatArray & operator= (const MFloatArray &other)`
- `MStatus set (float element, unsigned int index)`
- `MStatus set (double element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (float element, unsigned int index)`
- `MStatus append (float element)`
- `MStatus copy (const MFloatArray &source)`
- `MStatus clear ()`
- `MStatus get (float[]) const`
- `MStatus get (double[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `float operator[] (unsigned int index) const`
- `float & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatArray &array)`

### MFloatMatrix

Module `OpenMaya`. Source: [class_m_float_matrix.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_matrix.html).

**Public Member Functions**

- `MFloatMatrix ()`
- `MFloatMatrix (const MFloatMatrix &src)`
- `MFloatMatrix (const double m[4][4])`
- `MFloatMatrix (const float m[4][4])`
- `~MFloatMatrix ()`
- `MFloatMatrix & operator= (const MFloatMatrix &)`
- `float operator() (unsigned int row, unsigned int col) const`
- `const float * operator[] (unsigned int row) const`
- `MStatus get (double dest[4][4]) const`
- `MStatus get (float dest[4][4]) const`
- `MFloatMatrix transpose () const`
- `MFloatMatrix & setToIdentity ()`
- `MFloatMatrix & setToProduct (const MFloatMatrix &left, const MFloatMatrix &right)`
- `MFloatMatrix & operator+= (const MFloatMatrix &right)`
- `MFloatMatrix operator+ (const MFloatMatrix &right) const`
- `MFloatMatrix & operator-= (const MFloatMatrix &right)`
- `MFloatMatrix operator- (const MFloatMatrix &right) const`
- `MFloatMatrix & operator*= (const MFloatMatrix &right)`
- `MFloatMatrix operator* (const MFloatMatrix &right) const`
- `MFloatMatrix & operator*= (float)`
- `MFloatMatrix operator* (float) const`
- `bool operator== (const MFloatMatrix &other) const`
- `bool operator!= (const MFloatMatrix &other) const`
- `MFloatMatrix inverse () const`
- `MFloatMatrix adjoint () const`
- `MFloatMatrix homogenize () const`
- `float det4x4 () const`
- `float det3x3 () const`
- `bool isEquivalent (const MFloatMatrix &other, float tolerance=MFloatMatrix_kTol) const`
- `float & operator() (unsigned int row, unsigned int col)`
- `float * operator[] (unsigned int row)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `float matrix [4][4]`

**Friends**

- `OPENMAYA_EXPORT MFloatMatrix operator* (float, const MFloatMatrix &right)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatMatrix &m)`

### MFloatPoint

Module `OpenMaya`. Source: [class_m_float_point.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_point.html).

**Public Member Functions**

- `MFloatPoint ()`
- `MFloatPoint (const MFloatPoint &srcpt)`
- `MFloatPoint (const MPoint &srcpt)`
- `MFloatPoint (const MFloatVector &src)`
- `MFloatPoint (const MVector &src)`
- `MFloatPoint (float xx, float yy, float zz=0.0, float ww=1.0)`
- `MFloatPoint (const float f[4])`
- `MFloatPoint (const double d[4])`
- `~MFloatPoint ()`
- `MStatus get (double dest[4]) const`
- `MStatus get (float dest[4]) const`
- `MStatus setCast (const MPoint &srcpt)`
- `MStatus setCast (const MVector &src)`
- `MStatus setCast (const double d[4])`
- `MFloatPoint & operator= (const MFloatPoint &src)`
- `float operator() (unsigned int i) const`
- `float operator[] (unsigned int i) const`
- `MFloatVector operator- (const MFloatPoint &other) const`
- `MFloatPoint operator+ (const MFloatVector &other) const`
- `MFloatPoint operator- (const MFloatVector &other) const`
- `MFloatPoint & operator+= (const MFloatVector &vector)`
- `MFloatPoint & operator-= (const MFloatVector &vector)`
- `MFloatPoint operator* (const int scale) const`
- `MFloatPoint operator* (const short scale) const`
- `MFloatPoint operator* (const unsigned int scale) const`
- `MFloatPoint operator* (const unsigned short scale) const`
- `MFloatPoint operator* (const float scale) const`
- `MFloatPoint operator* (const double scale) const`
- `MFloatPoint operator/ (const float scale) const`
- `MFloatPoint operator* (const MFloatMatrix &) const`
- `MFloatPoint & operator*= (const MFloatMatrix &)`
- `MFloatPoint & operator*= (const int)`
- `MFloatPoint & operator*= (const short)`
- `MFloatPoint & operator*= (const unsigned int)`
- `MFloatPoint & operator*= (const unsigned short)`
- `MFloatPoint & operator*= (const float)`
- `MFloatPoint & operator*= (const double)`
- `bool operator== (const MFloatPoint &other) const`
- `bool operator!= (const MFloatPoint &other) const`
- `MFloatPoint & cartesianize ()`
- `MFloatPoint & rationalize ()`
- `MFloatPoint & homogenize ()`
- `float distanceTo (const MFloatPoint &other) const`
- `bool isEquivalent (const MFloatPoint &other, float tolerance=MFloatPoint_kTol) const`
- `float & operator() (unsigned int i)`
- `float & operator[] (unsigned int i)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `float x`
- `float y`
- `float z`
- `float w`

**Static Public Attributes**

- `static const MFloatPoint origin`

**Friends**

- `OPENMAYA_EXPORT MFloatPoint operator* (int, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (short, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (unsigned int, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (unsigned short, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (float, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (double, const MFloatPoint &)`
- `OPENMAYA_EXPORT MFloatPoint operator* (const MFloatMatrix &, const MFloatPoint &)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatPoint &p)`

### MFloatPointArray

Module `OpenMaya`. Source: [class_m_float_point_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_point_array.html).

**Public Member Functions**

- `MFloatPointArray ()`
- `MFloatPointArray (const MFloatPointArray &other)`
- `MFloatPointArray (const MFloatPoint points[], unsigned int count)`
- `MFloatPointArray (const float points[][4], unsigned int count)`
- `MFloatPointArray (const double points[][4], unsigned int count)`
- `MFloatPointArray (unsigned int initialSize, const MFloatPoint &initialValue=MFloatPoint::origin)`
- `~MFloatPointArray ()`
- `const MFloatPoint & operator[] (unsigned int index) const`
- `MFloatPointArray & operator= (const MFloatPointArray &other)`
- `MStatus set (const MFloatPoint &element, unsigned int index)`
- `MStatus set (unsigned int index, float x, float y, float z=0, float w=1)`
- `MStatus set (float element[4], unsigned int index)`
- `MStatus set (double element[4], unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MFloatPoint &element, unsigned int index)`
- `MStatus append (const MFloatPoint &element)`
- `MStatus append (float x, float y, float z=0, float w=1)`
- `MStatus copy (const MFloatPointArray &source)`
- `MStatus clear ()`
- `MStatus get (float[][4]) const`
- `MStatus get (double[][4]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MFloatPoint & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatPointArray &array)`

### MFloatVector

Module `OpenMaya`. Source: [class_m_float_vector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_vector.html).

**Public Member Functions**

- `MFloatVector ()`
- `MFloatVector (const MFloatVector &)`
- `MFloatVector (const MVector &)`
- `MFloatVector (const MFloatPoint &)`
- `MFloatVector (const MPoint &)`
- `MFloatVector (float xx, float yy, float zz=0.0)`
- `MFloatVector (const float f[3])`
- `MFloatVector (const double d[3])`
- `~MFloatVector ()`
- `MFloatVector & operator= (const MFloatVector &src)`
- `float operator() (unsigned int i) const`
- `float operator[] (unsigned int i) const`
- `MFloatVector operator^ (const MFloatVector &right) const`
- `MFloatVector & operator/= (float scalar)`
- `MFloatVector operator/ (float scalar) const`
- `MFloatVector & operator*= (int scalar)`
- `MFloatVector & operator*= (short scalar)`
- `MFloatVector & operator*= (unsigned int scalar)`
- `MFloatVector & operator*= (unsigned short scalar)`
- `MFloatVector & operator*= (float scalar)`
- `MFloatVector & operator*= (double)`
- `MFloatVector operator* (int scalar) const`
- `MFloatVector operator* (short scalar) const`
- `MFloatVector operator* (unsigned int scalar) const`
- `MFloatVector operator* (unsigned short scalar) const`
- `MFloatVector operator* (float scalar) const`
- `MFloatVector operator* (double scalar) const`
- `MFloatVector operator+ (const MFloatVector &other) const`
- `MFloatVector & operator+= (const MFloatVector &other)`
- `MFloatVector operator- () const`
- `MFloatVector operator- (const MFloatVector &other) const`
- `MFloatVector & operator-= (const MFloatVector &other)`
- `MFloatVector operator* (const MFloatMatrix &) const`
- `MFloatVector & operator*= (const MFloatMatrix &)`
- `float operator* (const MFloatVector &other) const`
- `bool operator!= (const MFloatVector &other) const`
- `bool operator== (const MFloatVector &other) const`
- `MStatus get (float[3]) const`
- `float length () const`
- `MFloatVector normal () const`
- `MStatus normalize ()`
- `float angle (const MFloatVector &other) const`
- `bool isEquivalent (const MFloatVector &other, float tolerance=MFloatVector_kTol) const`
- `bool isParallel (const MFloatVector &other, float tolerance=MFloatVector_kTol) const`
- `float & operator() (unsigned int i)`
- `float & operator[] (unsigned int i)`
- `MFloatVector transformAsNormal (const MFloatMatrix &matrix) const`

**Public Attributes**

- `float x`
- `float y`
- `float z`

**Static Public Attributes**

- `static const MFloatVector zero`
- `static const MFloatVector one`
- `static const MFloatVector xAxis`
- `static const MFloatVector yAxis`
- `static const MFloatVector zAxis`
- `static const MFloatVector xNegAxis`
- `static const MFloatVector yNegAxis`
- `static const MFloatVector zNegAxis`

**Friends**

- `OPENMAYA_EXPORT MFloatVector operator* (int, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (short, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (unsigned int, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (unsigned short, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (float, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (double, const MFloatVector &)`
- `OPENMAYA_EXPORT MFloatVector operator* (const MFloatMatrix &, const MFloatVector &)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatVector &v)`

### MFloatVectorArray

Module `OpenMaya`. Source: [class_m_float_vector_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_float_vector_array.html).

**Public Member Functions**

- `MFloatVectorArray ()`
- `MFloatVectorArray (const MFloatVectorArray &other)`
- `MFloatVectorArray (const MFloatVector vectors[], unsigned int count)`
- `MFloatVectorArray (const double vectors[][3], unsigned int count)`
- `MFloatVectorArray (const float vectors[][3], unsigned int count)`
- `MFloatVectorArray (unsigned int initialSize, const MFloatVector &initialValue=MFloatVector::zero)`
- `~MFloatVectorArray ()`
- `const MFloatVector & operator[] (unsigned int index) const`
- `MFloatVectorArray & operator= (const MFloatVectorArray &other)`
- `MStatus set (const MFloatVector &element, unsigned int index)`
- `MStatus set (double element[3], unsigned int index)`
- `MStatus set (float element[3], unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MFloatVector &element, unsigned int index)`
- `MStatus append (const MFloatVector &element)`
- `MStatus copy (const MFloatVectorArray &source)`
- `MStatus clear ()`
- `MStatus get (double[][3]) const`
- `MStatus get (float[][3]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MFloatVector & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MFloatVectorArray &array)`

### MFn

Module `OpenMaya`. Source: [class_m_fn.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn.html).

**Public Types**

- `enum Type { kInvalid = 0, kBase, kNamedObject, kModel, kDependencyNode, kAddDoubleLinear, kAffect, kAnimCurve, kAnimCurveTimeToAngular, kAnimCurveTimeToDistance, kAnimCurveTimeToTime, kAnimCurveTimeToUnitless, kAnimCurveUnitlessToAngular, kAnimCurveUnitlessToDistance, kAnimCurveUnitlessToTime, kAnimCurveUnitlessToUnitless, kResultCurve, kResultCurveTimeToAngular, kResultCurveTimeToDistance, kResultCurveTimeToTime, kResultCurveTimeToUnitless, kAngleBetween, kAudio, kBackground, kColorBackground, kFileBackground, kRampBackground, kBlend, kBlendTwoAttr, kBlendWeighted, kBlendDevice, kBlendColors, kBump, kBump3d, kCameraView, kChainToSpline, kChoice, kCondition, kContrast, kClampColor, kCreate, kAlignCurve, kAlignSurface, kAttachCurve, kAttachSurface, kAvgCurves, kAvgSurfacePoints, kAvgNurbsSurfacePoints, kBevel, kBirailSrf, kDPbirailSrf, kMPbirailSrf, kSPbirailSrf, kBoundary, kCircle, kCloseCurve, kClosestPointOnSurface, kCloseSurface, kCurveFromSurface, kCurveFromSurfaceBnd, kCurveFromSurfaceCoS, kCurveFromSurfaceIso, kCurveInfo, kDetachCurve, kDetachSurface, kExtendCurve, kExtendSurface, kExtrude, kFFblendSrf, kFFfilletSrf, kFilletCurve, kFitBspline, kFlow, kHardenPointCurve, kIllustratorCurve, kInsertKnotCrv, kInsertKnotSrf, kIntersectSurface, kNurbsTesselate, kNurbsPlane, kNurbsCube, kOffsetCos, kOffsetCurve, kPlanarTrimSrf, kPointOnCurveInfo, kPointOnSurfaceInfo, kPrimitive, kProjectCurve, kProjectTangent, kRBFsurface, kRebuildCurve, kRebuildSurface, kReverseCurve, kReverseSurface, kRevolve, kRevolvedPrimitive, kCone, kRenderCone, kCylinder, kSphere, kSkin, kStitchSrf, kSubCurve, kSurfaceInfo, kTextCurves, kTrim, kUntrim, kDagNode, kProxy, kUnderWorld, kTransform, kAimConstraint, kLookAt, kGeometryConstraint, kGeometryVarGroup, kAnyGeometryVarGroup, kCurveVarGroup, kMeshVarGroup, kSurfaceVarGroup, kIkEffector, kIkHandle, kJoint, kManipulator3D, kArrowManip, kAxesActionManip, kBallProjectionManip, kCircleManip, kScreenAlignedCircleManip, kCircleSweepManip, kConcentricProjectionManip, kCubicProjectionManip, kCylindricalProjectionManip, kDiscManip, kFreePointManip, kCenterManip, kLimitManip, kEnableManip, kFreePointTriadManip, kPropMoveTriadManip, kTowPointManip, kPolyCreateToolManip, kPolySplitToolManip, kGeometryOnLineManip, kCameraPlaneManip, kToggleOnLineManip, kStateManip, kIsoparmManip, kLineManip, kManipContainer, kAverageCurveManip, kBarnDoorManip, kBevelManip, kBlendManip, kButtonManip, kCameraManip, kCoiManip, kCpManip, kCreateCVManip, kCreateEPManip, kCurveEdManip, kCurveSegmentManip, kDirectionManip, kDofManip, kDropoffManip, kExtendCurveDistanceManip, kExtrudeManip, kIkSplineManip, kIkRPManip, kJointClusterManip, kLightManip, kMotionPathManip, kOffsetCosManip, kOffsetCurveManip, kProjectionManip, kPolyProjectionManip, kProjectionUVManip, kProjectionMultiManip, kProjectTangentManip, kPropModManip, kQuadPtOnLineManip, kRbfSrfManip, kReverseCurveManip, kReverseCrvManip, kReverseSurfaceManip, kRevolveManip, kRevolvedPrimitiveManip, kSpotManip, kSpotCylinderManip, kTriplanarProjectionManip, kTrsManip, kDblTrsManip, kPivotManip2D, kManip2DContainer, kPolyMoveUVManip, kPolyMappingManip, kPolyModifierManip, kPolyMoveVertexManip, kPolyVertexNormalManip, kTexSmudgeUVManip, kTexLatticeDeformManip, kTexLattice, kTexSmoothManip, kTrsTransManip, kTrsInsertManip, kTrsXformManip, kManipulator2D, kTranslateManip2D, kPlanarProjectionManip, kPointOnCurveManip, kTowPointOnCurveManip, kMarkerManip, kPointOnLineManip, kPointOnSurfaceManip, kTranslateUVManip, kRotateBoxManip, kRotateManip, kHandleRotateManip, kRotateLimitsManip, kScaleLimitsManip, kScaleManip, kScalingBoxManip, kScriptManip, kSphericalProjectionManip, kTextureManip3D, kToggleManip, kTranslateBoxManip, kTranslateLimitsManip, kTranslateManip, kTrimManip, kJointTranslateManip, kManipulator, kCirclePointManip, kDimensionManip, kFixedLineManip, kLightProjectionGeometry, kLineArrowManip, kPointManip, kTriadManip, kNormalConstraint, kOrientConstraint, kPointConstraint, kSymmetryConstraint, kParentConstraint, kPoleVectorConstraint, kScaleConstraint, kTangentConstraint, kUnknownTransform, kWorld, kShape, kBaseLattice, kCamera, kCluster, kSoftMod, kCollision, kDummy, kEmitter, kField, kAir, kDrag, kGravity, kNewton, kRadial, kTurbulence, kUniform, kVortex, kGeometric, kCurve, kNurbsCurve, kNurbsCurveGeom, kDimension, kAngle, kAnnotation, kDistance, kArcLength, kRadius, kParamDimension, kDirectedDisc, kRenderRect, kEnvFogShape, kLattice, kLatticeGeom, kLocator, kDropoffLocator, kMarker, kOrientationMarker, kPositionMarker, kOrientationLocator, kTrimLocator, kPlane, kSketchPlane, kGroundPlane, kOrthoGrid, kSprite, kSurface, kNurbsSurface, kNurbsSurfaceGeom, kMesh, kMeshGeom, kRenderSphere, kFlexor, kClusterFlexor, kGuideLine, kLight, kAmbientLight, kNonAmbientLight, kAreaLight, kLinearLight, kNonExtendedLight, kDirectionalLight, kPointLight, kSpotLight, kParticle, kPolyToolFeedbackShape, kRigidConstraint, kRigid, kSpring, kUnknownDag, kDefaultLightList, kDeleteComponent, kDispatchCompute, kShadingEngine, kDisplacementShader, kDistanceBetween, kDOF, kDummyConnectable, kDynamicsController, kGeoConnectable, kExpression, kExtract, kFilter, kFilterClosestSample, kFilterEuler, kFilterSimplify, kGammaCorrect, kGeometryFilt, kBendLattice, kBlendShape, kCombinationShape, kBulgeLattice, kFFD, kFfdDualBase, kRigidDeform, kSculpt, kTextureDeformer, kTextureDeformerHandle, kTweak, kWeightGeometryFilt, kClusterFilter, kSoftModFilter, kJointCluster, kDeltaMush, kTension, kMorph, kSolidify, kProximityWrap, kWire, kGroupId, kGroupParts, kGuide, kHsvToRgb, kHyperGraphInfo, kHyperLayout, kHyperView, kIkSolver, kMCsolver, kPASolver, kSCsolver, kRPsolver, kSplineSolver, kIkSystem, kImagePlane, kLambert, kReflect, kBlinn, kPhong, kPhongExplorer, kLayeredShader, kStandardSurface, kLightInfo, kLeastSquares, kLightFogMaterial, kEnvFogMaterial, kLightList, kLightSource, kLuminance, kMakeGroup, kMaterial, kDiffuseMaterial, kLambertMaterial, kBlinnMaterial, kPhongMaterial, kLightSourceMaterial, kMaterialInfo, kMaterialTemplate, kMatrixAdd, kMatrixHold, kMatrixMult, kMatrixPass, kMatrixWtAdd, kMidModifier, kMidModifierWithMatrix, kPolyBevel, kPolyTweak, kPolyAppend, kPolyChipOff, kPolyCloseBorder, kPolyCollapseEdge, kPolyCollapseF, kPolyCylProj, kPolyDelEdge, kPolyDelFacet, kPolyDelVertex, kPolyExtrudeFacet, kPolyMapCut, kPolyMapDel, kPolyMapSew, kPolyMergeEdge, kPolyMergeFacet, kPolyMoveEdge, kPolyMoveFacet, kPolyMoveFacetUV, kPolyMoveUV, kPolyMoveVertex, kPolyMoveVertexUV, kPolyNormal, kPolyPlanProj, kPolyProj, kPolyQuad, kPolySmooth, kPolySoftEdge, kPolySphProj, kPolySplit, kPolySubdEdge, kPolySubdFacet, kPolyTriangulate, kPolyCreator, kPolyPrimitive, kPolyCone, kPolyCube, kPolyCylinder, kPolyMesh, kPolySphere, kPolyTorus, kPolyCreateFacet, kPolyUnite, kMotionPath, kPluginMotionPathNode, kMultilisterLight, kMultiplyDivide, kOldGeometryConstraint, kOpticalFX, kParticleAgeMapper, kParticleCloud, kParticleColorMapper, kParticleIncandecenceMapper, kParticleTransparencyMapper, kPartition, kPlace2dTexture, kPlace3dTexture, kPluginDependNode, kPluginLocatorNode, kPlusMinusAverage, kPointMatrixMult, kPolySeparate, kPostProcessList, kProjection, kRecord, kRenderUtilityList, kReverse, kRgbToHsv, kRigidSolver, kSet, kTextureBakeSet, kVertexBakeSet, kSetRange, kShaderGlow, kShaderList, kShadingMap, kSamplerInfo, kShapeFragment, kSimpleVolumeShader, kSl60, kSnapshot, kStoryBoard, kSummaryObject, kSuper, kControl, kSurfaceLuminance, kSurfaceShader, kTextureList, kTextureEnv, kEnvBall, kEnvCube, kEnvChrome, kEnvSky, kEnvSphere, kTexture2d, kBulge, kChecker, kCloth, kFileTexture, kFractal, kGrid, kMountain, kRamp, kStencil, kWater, kTexture3d, kBrownian, kCloud, kCrater, kGranite, kLeather, kMarble, kRock, kSnow, kSolidFractal, kStucco, kTxSl, kWood, kTime, kTimeToUnitConversion, kRenderSetup, kRenderGlobals, kRenderGlobalsList, kRenderQuality, kResolution, kHardwareRenderGlobals, kArrayMapper, kUnitConversion, kUnitToTimeConversion, kUseBackground, kUnknown, kVectorProduct, kVolumeShader, kComponent, kCurveCVComponent, kCurveEPComponent, kCurveKnotComponent, kCurveParamComponent, kIsoparmComponent, kPivotComponent, kSurfaceCVComponent, kSurfaceEPComponent, kSurfaceKnotComponent, kEdgeComponent, kLatticeComponent, kSurfaceRangeComponent, kDecayRegionCapComponent, kDecayRegionComponent, kMeshComponent, kMeshEdgeComponent, kMeshPolygonComponent, kMeshFrEdgeComponent, kMeshVertComponent, kMeshFaceVertComponent, kOrientationComponent, kSubVertexComponent, kMultiSubVertexComponent, kSetGroupComponent, kDynParticleSetComponent, kSelectionItem, kDagSelectionItem, kNonDagSelectionItem, kItemList, kAttribute, kNumericAttribute, kDoubleAngleAttribute, kFloatAngleAttribute, kDoubleLinearAttribute, kFloatLinearAttribute, kTimeAttribute, kEnumAttribute, kUnitAttribute, kTypedAttribute, kCompoundAttribute, kGenericAttribute, kLightDataAttribute, kMatrixAttribute, kFloatMatrixAttribute, kMessageAttribute, kPlugin, kData, kComponentListData, kDoubleArrayData, kIntArrayData, kUintArrayData, kLatticeData, kMatrixData, kMeshData, kNurbsSurfaceData, kNurbsCurveData, kNumericData, kData2Double, kData2Float, kData2Int, kData2Short, kData3Double, kData3Float, kData3Int, kData3Short, kPluginData, kPointArrayData, kMatrixArrayData, kSphereData, kStringData, kStringArrayData, kVectorArrayData, kSelectionList, kTransformGeometry, kCommEdgePtManip, kCommEdgeOperManip, kCommEdgeSegmentManip, kCommCornerManip, kCommCornerOperManip, kPluginDeformerNode, kTorus, kPolyBoolOp, kSingleShadingSwitch, kDoubleShadingSwitch, kTripleShadingSwitch, kNurbsSquare, kAnisotropy, kNonLinear, kDeformFunc, kDeformBend, kDeformTwist, kDeformSquash, kDeformFlare, kDeformSine, kDeformWave, kDeformBendManip, kDeformTwistManip, kDeformSquashManip, kDeformFlareManip, kDeformSineManip, kDeformWaveManip, kSoftModManip, kDistanceManip, kScript, kCurveFromMeshEdge, kCurveCurveIntersect, kNurbsCircular3PtArc, kNurbsCircular2PtArc, kOffsetSurface, kRoundConstantRadius, kRoundRadiusManip, kRoundRadiusCrvManip, kRoundConstantRadiusManip, kThreePointArcManip, kTwoPointArcManip, kTextButtonManip, kOffsetSurfaceManip, kImageData, kImageLoad, kImageSave, kImageNetSrc, kImageNetDest, kImageRender, kImageAdd, kImageDiff, kImageMultiply, kImageOver, kImageUnder, kImageColorCorrect, kImageBlur, kImageFilter, kImageDepth, kImageDisplay, kImageView, kImageMotionBlur, kViewColorManager, kMatrixFloatData, kSkinShader, kComponentManip, kSelectionListData, kObjectFilter, kObjectMultiFilter, kObjectNameFilter, kObjectTypeFilter, kObjectAttrFilter, kObjectRenderFilter, kObjectScriptFilter, kSelectionListOperator, kSubdiv, kPolyToSubdiv, kSkinClusterFilter , kCharacter, kCharacterOffset, kDagPose, kStitchAsNurbsShell, kExplodeNurbsShell, kNurbsBoolean, kStitchSrfManip, kForceUpdateManip, kPluginManipContainer, kPolySewEdge, kPolyMergeVert, kPolySmoothFacet, kSmoothCurve, kGlobalStitch, kSubdivCVComponent, kSubdivEdgeComponent, kSubdivFaceComponent, kUVManip2D, kTranslateUVManip2D, kRotateUVManip2D, kScaleUVManip2D, kPolyTweakUV, kMoveUVShellManip2D, kPluginShape, kGeometryData, kSingleIndexedComponent, kDoubleIndexedComponent, kTripleIndexedComponent, kExtendSurfaceDistanceManip, kSquareSrf, kSquareSrfManip, kSubdivToPoly, kDynBase, kDynEmitterManip, kDynFieldsManip, kDynBaseFieldManip, kDynAirManip, kDynNewtonManip, kDynTurbulenceManip, kDynSpreadManip, kDynAttenuationManip, kDynArrayAttrsData, kPluginFieldNode, kPluginEmitterNode, kPluginSpringNode, kDisplayLayer, kDisplayLayerManager, kPolyColorPerVertex, kCreateColorSet, kDeleteColorSet, kCopyColorSet, kBlendColorSet, kPolyColorMod, kPolyColorDel, kCharacterMappingData, kDynSweptGeometryData, kWrapFilter, kMeshVtxFaceComponent, kBinaryData, kAttribute2Double, kAttribute2Float, kAttribute2Short, kAttribute2Int, kAttribute3Double, kAttribute3Float, kAttribute3Short, kAttribute3Int, kReference, kBlindData, kBlindDataTemplate, kPolyBlindData, kPolyNormalPerVertex, kNurbsToSubdiv, kPluginIkSolver, kInstancer, kMoveVertexManip, kStroke, kBrush, kStrokeGlobals, kPluginGeometryData, kLightLink, kDynGlobals, kPolyReduce, kLodThresholds, kChooser, kLodGroup, kMultDoubleLinear, kFourByFourMatrix, kTowPointOnSurfaceManip, kSurfaceEdManip, kSurfaceFaceComponent, kClipScheduler, kClipLibrary, kSubSurface, kSmoothTangentSrf, kRenderPass, kRenderPassSet, kRenderLayer, kRenderLayerManager, kPassContributionMap, kPrecompExport, kRenderTarget, kRenderedImageSource, kImageSource, kPolyFlipEdge, kPolyExtrudeEdge, kAnimBlend, kAnimBlendInOut, kPolyAppendVertex, kUvChooser, kSubdivCompId, kVolumeAxis, kDeleteUVSet, kSubdHierBlind, kSubdBlindData, kCharacterMap, kLayeredTexture, kSubdivCollapse, kParticleSamplerInfo, kCopyUVSet, kCreateUVSet, kClip, kPolySplitVert, kSubdivData, kSubdivGeom, kUInt64ArrayData, kInt64ArrayData, kPolySplitEdge, kSubdivReverseFaces, kMeshMapComponent, kSectionManip, kXsectionSubdivEdit, kSubdivToNurbs, kEditCurve, kEditCurveManip, kCrossSectionManager, kCreateSectionManip, kCrossSectionEditManip, kDropOffFunction, kSubdBoolean, kSubdModifyEdge, kModifyEdgeCrvManip, kModifyEdgeManip, kScalePointManip, kTransformBoxManip, kSymmetryLocator, kSymmetryMapVector, kSymmetryMapCurve, kCurveFromSubdivEdge, kCreateBPManip, kModifyEdgeBaseManip, kSubdExtrudeFace, kSubdivSurfaceVarGroup, kSfRevolveManip, kCurveFromSubdivFace, kUnused1, kUnused2, kUnused3, kUnused4, kUnused5, kUnused6, kPolyTransfer, kPolyAverageVertex, kPolyAutoProj, kPolyLayoutUV, kPolyMapSewMove, kSubdModifier, kSubdMoveVertex, kSubdMoveEdge, kSubdMoveFace, kSubdDelFace, kSnapshotShape, kSubdivMapComponent, kJiggleDeformer, kGlobalCacheControls, kDiskCache, kSubdCloseBorder, kSubdMergeVert, kBoxData, kBox, kRenderBox, kSubdSplitFace, kVolumeFog, kSubdTweakUV, kSubdMapCut, kSubdLayoutUV, kSubdMapSewMove, kOcean, kVolumeNoise, kSubdAutoProj, kSubdSubdivideFace, kNoise, kAttribute4Double, kData4Double, kSubdPlanProj, kSubdTweak, kSubdProjectionManip, kSubdMappingManip, kHardwareReflectionMap, kPolyNormalizeUV, kPolyFlipUV, kHwShaderNode, kPluginHardwareShader, kPluginHwShaderNode, kSubdAddTopology, kSubdCleanTopology, kImplicitCone, kImplicitSphere, kRampShader, kVolumeLight, kOceanShader, kBevelPlus, kStyleCurve, kPolyCut, kPolyPoke, kPolyWedgeFace, kPolyCutManipContainer, kPolyCutManip, kPolyMirrorManipContainer, kPolyPokeManip, kFluidTexture3D, kFluidTexture2D, kPolyMergeUV, kPolyStraightenUVBorder, kAlignManip, kPluginTransformNode, kFluid, kFluidGeom, kFluidData, kSmear, kStringShadingSwitch, kStudioClearCoat, kFluidEmitter, kHeightField, kGeoConnector, kSnapshotPath, kPluginObjectSet, kQuadShadingSwitch, kPolyExtrudeVertex, kPairBlend, kTextManip, kViewManip, kXformManip, kMute, kConstraint, kTrimWithBoundaries, kCurveFromMeshCoM, kFollicle, kHairSystem, kRemapValue, kRemapColor, kRemapHsv, kHairConstraint, kTimeFunction , kObjectBinFilter, kPolySmoothProxy, kPfxGeometry, kPfxHair, kHairTubeShader, kPsdFileTexture, kKeyframeDelta, kKeyframeDeltaMove, kKeyframeDeltaScale, kKeyframeDeltaAddRemove, kKeyframeDeltaBlockAddRemove, kKeyframeDeltaInfType, kKeyframeDeltaTangent, kKeyframeDeltaWeighted, kKeyframeDeltaBreakdown, kPolyMirror, kPolyCreaseEdge, kPolyPinUV, kHikEffector, kHikIKEffector, kHikFKJoint, kHikSolver, kHikHandle, kProxyManager, kPolyAutoProjManip, kPolyPrism, kPolyPyramid, kPolySplitRing, kPfxToon, kToonLineAttributes, kPolyDuplicateEdge, kFacade, kMaterialFacade, kEnvFacade, kAISEnvFacade, kLineModifier, kPolyArrow, kPolyPrimitiveMisc, kPolyPlatonicSolid, kPolyPipe, kHikFloorContactMarker, kHikGroundPlane, kPolyComponentData, kPolyHelix, kCacheFile, kHistorySwitch, kClosestPointOnMesh, kUVPin, kProximityPin, kTransferAttributes, kDynamicConstraint, kNComponent, kPolyBridgeEdge, kCacheableNode, kNucleus, kNBase, kCacheBase, kCacheBlend, kCacheTrack, kKeyframeRegionManip, kCurveNormalizerAngle, kCurveNormalizerLinear, kHyperLayoutDG, kPluginImagePlaneNode, kNCloth, kNParticle, kNRigid, kPluginParticleAttributeMapperNode, kCameraSet, kPluginCameraSet, kContainer, kFloatVectorArrayData, kNObjectData, kNObject, kPluginConstraintNode, kAsset, kPolyEdgeToCurve, kAnimLayer, kBlendNodeBase, kBlendNodeBoolean, kBlendNodeDouble, kBlendNodeDoubleAngle, kBlendNodeDoubleLinear, kBlendNodeEnum, kBlendNodeFloat, kBlendNodeFloatAngle, kBlendNodeFloatLinear, kBlendNodeInt16, kBlendNodeInt32, kBlendNodeAdditiveScale, kBlendNodeAdditiveRotation, kPluginManipulatorNode, kNIdData, kNId, kFloatArrayData, kMembrane, kMergeVertsToolManip, kUint64SingleIndexedComponent, kPolyToolFeedbackManip, kPolySelectEditFeedbackManip, kWriteToFrameBuffer, kWriteToColorBuffer, kWriteToVectorBuffer, kWriteToDepthBuffer, kWriteToLabelBuffer, kStereoCameraMaster , kBlendNodeTime, kCreateBezierManip, kBezierCurve, kBezierCurveData, kNurbsCurveToBezier, kBezierCurveToNurbs, kPolySpinEdge, kPolyHoleFace, kPointOnPolyConstraint, kPolyConnectComponents, kSkinBinding, kVolumeBindManip, kVertexWeightSet, kNearestPointOnCurve, kColorProfile, kAdskMaterial, kContainerBase, kDagContainer, kPolyUVRectangle, kHardwareRenderingGlobals, kPolyProjectCurve, kRenderingList, kPolyExtrudeManip, kPolyExtrudeManipContainer, kThreadedDevice, kClientDevice, kPluginClientDevice, kPluginThreadedDevice, kTimeWarp, kAssembly, kClipGhostShape, kClipToGhostData, kMandelbrot, kMandelbrot3D, kGreasePlane, kGreasePlaneRenderShape, kGreasePencilSequence, kEditMetadata, kPolyEditEdgeFlow, kFosterParent, kSnapUVManip2D, kToolContext, kNLE, kShrinkWrapFilter, kEditsManager, kPolyBevel2, kGeomBind, kColorMgtGlobals, kPolyBevel3, kTimeEditorClipBase, kTimeEditorClipEvaluator, kTimeEditorClip, kTimeEditor, kTimeEditorTracks, kTimeEditorInterpolator, kTimeEditorAnimSource, kCaddyManipBase, kPolyCaddyManip, kPolyModifierManipContainer, kPolyRemesh, kPolyContourProj, kContourProjectionManip, kNodeGraphEditorInfo, kNodeGraphEditorBookmarks, kNodeGraphEditorBookmarkInfo, kPluginSkinCluster, kPluginGeometryFilter, kPluginBlendShape , kTrackInfoManager, kPolyClean, kShapeEditorManager, kOceanDeformer, kPoseInterpolatorManager, kControllerTag, kCustomEvaluatorClusterNode, kPolyCircularize , kLast, kAttribute2Long = kAttribute2Int, kAttribute3Long = kAttribute3Int, kData2Long = kData2Int, kData3Long = kData3Int }`

**Public Member Functions**

- `MFn ()`

### MFnAmbientLight

Module `OpenMaya`. Inherits `MFnLight`. Source: [class_m_fn_ambient_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_ambient_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAmbientLight ()`
- `MFnAmbientLight ()`
- `MFnAmbientLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAmbientLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `float ambientShade (MStatus *ReturnStatus=NULL) const`
- `MStatus setAmbientShade (const float &ambient_shade)`
- `bool castSoftShadows (MStatus *ReturnStatus=NULL) const`
- `MStatus setCastSoftShadows (const bool &cast_soft_shadows)`
- `float shadowRadius (MStatus *ReturnStatus=NULL) const`
- `MStatus setShadowRadius (const float &shadow_radius)`
- `MFnAmbientLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnAnisotropyShader

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_anisotropy_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_anisotropy_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAnisotropyShader ()`
- `MFnAnisotropyShader ()`
- `MFnAnisotropyShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MColor tangentUCamera (MStatus *ReturnStatus=NULL) const`
- `MStatus setTangentUCamera (const MColor &tangu_camera)`
- `MColor tangentVCamera (MStatus *ReturnStatus=NULL) const`
- `MStatus setTangentVCamera (const MColor &tangv_camera)`
- `float correlationX (MStatus *ReturnStatus=NULL) const`
- `MStatus setCorrelationX (const float &correl_x)`
- `float correlationY (MStatus *ReturnStatus=NULL) const`
- `MStatus setCorrelationY (const float &correl_y)`
- `float roughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setRoughness (const float &roughness)`
- `float rotateAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setRotateAngle (const float &rotate_angle)`
- `float refractiveIndex (MStatus *ReturnStatus=NULL) const`
- `MStatus setRefractiveIndex (const float &refractive_index)`
- `bool anisotropicReflectivity (MStatus *ReturnStatus=NULL) const`
- `MStatus setAnisotropicReflectivity (const bool &anisotropic_reflectivity)`
- `MFnAnisotropyShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnAreaLight

Module `OpenMaya`. Inherits `MFnNonExtendedLight`. Source: [class_m_fn_area_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_area_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAreaLight ()`
- `MFnAreaLight ()`
- `MFnAreaLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAreaLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `MFnAreaLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnArrayAttrsData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_array_attrs_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_array_attrs_data.html).

**Public Types**

- `enum Type { , kVectorArray, kDoubleArray, kIntArray, kStringArray }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnArrayAttrsData ()`
- `MFnArrayAttrsData ()`
- `MFnArrayAttrsData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus clear ()`
- `unsigned int count () const`
- `MStringArray list (MStatus *ReturnStatus=NULL) const`
- `bool checkArrayExist (const MString attrName, MFnArrayAttrsData::Type &arrayType, MStatus *ReturnStatus=NULL)`
- `MVectorArray vectorArray (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MDoubleArray doubleArray (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MIntArray intArray (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MStringArray stringArray (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MVectorArray getVectorData (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MDoubleArray getDoubleData (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MIntArray getIntData (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MStringArray getStringData (const MString attrName, MStatus *ReturnStatus=NULL)`
- `MFnArrayAttrsData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnAssembly

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_assembly.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_assembly.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAssembly ()`
- `MFnAssembly ()`
- `MFnAssembly (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAssembly (const MDagPath &object, MStatus *ret=NULL)`
- `MString createRepresentation (const MString &input, const MString &type, MDagModifier *undoRedo=NULL, MStatus *status=NULL)`
- `MString createRepresentation (const MString &input, const MString &type, const MString &representation, MDagModifier *undoRedo=NULL, MStatus *status=NULL)`
- `MStatus postLoad ()`
- `MStatus activate (const MString &representation)`
- `MString getActive (MStatus *status=NULL) const`
- `MStatus activateNonRecursive (const MString &representation)`
- `bool canActivate (MStatus *status=NULL) const`
- `bool isActive (const MString &representation, MStatus *status=NULL) const`
- `MString getInitialRep (MStatus *status=NULL) const`
- `MString getInitialRep (bool &hasInitialRep, MStatus *status=NULL) const`
- `MStringArray getRepresentations (MStatus *status=NULL) const`
- `MString getRepType (const MString &representation, MStatus *status=NULL) const`
- `MString getRepLabel (const MString &representation, MStatus *status=NULL) const`
- `MStringArray repTypes (MStatus *status=NULL) const`
- `MStatus deleteRepresentation (const MString &representation)`
- `MStatus deleteAllRepresentations ()`
- `MString getRepNamespace (MStatus *status=NULL) const`
- `MString setRepName (const MString &representation, const MString &newName, MStatus *status=NULL)`
- `MStatus setRepLabel (const MString &representation, const MString &label)`
- `MStatus importFile (const MString &fileName, const char *type=NULL, bool preserveReferences=false, const char *nameSpace=NULL, bool ignoreVersion=false)`
- `MString getAbsoluteRepNamespace (MStatus *status=NULL) const`
- `bool isTopLevel (MStatus *status=NULL) const`
- `bool supportsEdits (MStatus *status=NULL) const`
- `bool supportsMemberChanges (MStatus *status=NULL) const`
- `bool canRepApplyEdits (const MString &representation, MStatus *status=NULL) const`
- `bool handlesAddEdits (MStatus *status=NULL) const`
- `MObject getParentAssembly (MStatus *status=NULL) const`
- `MObjectArray getSubAssemblies (MStatus *status=NULL) const`
- `MFnAssembly (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`
- `static MObjectArray getTopLevelAssemblies ()`

### MFnAttribute

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_attribute.html).

**Public Types**

- `enum DisconnectBehavior { kDelete, kReset, kNothing }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAttribute ()`
- `MFnAttribute ()`
- `MFnAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool isReadable (MStatus *ReturnStatus=NULL) const`
- `bool isWritable (MStatus *ReturnStatus=NULL) const`
- `bool isConnectable (MStatus *ReturnStatus=NULL) const`
- `bool isStorable (MStatus *ReturnStatus=NULL) const`
- `bool isCached (MStatus *ReturnStatus=NULL) const`
- `bool isArray (MStatus *ReturnStatus=NULL) const`
- `bool indexMatters (MStatus *ReturnStatus=NULL) const`
- `bool isKeyable (MStatus *ReturnStatus=NULL) const`
- `bool isChannelBoxFlagSet (MStatus *ReturnStatus=NULL) const`
- `bool isHidden (MStatus *ReturnStatus=NULL) const`
- `bool isUsedAsColor (MStatus *ReturnStatus=NULL) const`
- `bool isIndeterminant (MStatus *ReturnStatus=NULL) const`
- `bool isRenderSource (MStatus *ReturnStatus=NULL) const`
- `bool isDynamic (MStatus *ReturnStatus=NULL) const`
- `bool isExtension (MStatus *ReturnStatus=NULL) const`
- `bool isWorldSpace (MStatus *ReturnStatus=NULL) const`
- `bool isAffectsWorldSpace (MStatus *ReturnStatus=NULL) const`
- `bool isUsedAsFilename (MStatus *ReturnStatus=NULL) const`
- `bool affectsAppearance (MStatus *ReturnStatus=NULL) const`
- `bool isProxyAttribute (MStatus *ReturnStatus=NULL) const`
- `DisconnectBehavior disconnectBehavior (MStatus *ReturnStatus=NULL) const`
- `bool usesArrayDataBuilder (MStatus *ReturnStatus=NULL) const`
- `bool internal (MStatus *ReturnStatus=NULL) const`
- `MStatus setReadable (bool state)`
- `MStatus setWritable (bool state)`
- `MStatus setConnectable (bool state)`
- `MStatus setStorable (bool state)`
- `MStatus setCached (bool state)`
- `MStatus setArray (bool state)`
- `MStatus setIndexMatters (bool state)`
- `MStatus setKeyable (bool state)`
- `MStatus setChannelBox (bool state)`
- `MStatus setHidden (bool state)`
- `MStatus setUsedAsColor (bool state)`
- `MStatus setIndeterminant (bool state)`
- `MStatus setRenderSource (bool state)`
- `MStatus setWorldSpace (bool state)`
- `MStatus setAffectsWorldSpace (bool state)`
- `MStatus setUsedAsFilename (bool state)`
- `MStatus setAffectsAppearance (bool state)`
- `MStatus setProxyAttribute (bool state)`
- `MStatus setDisconnectBehavior (DisconnectBehavior behavior)`
- `MStatus setUsesArrayDataBuilder (bool state)`
- `MStatus setInternal (bool state)`
- `MStatus setNiceNameOverride (const MString &localizedName)`
- `bool acceptsAttribute (const MFnAttribute &attr, MStatus *ReturnStatus=NULL) const`
- `bool accepts (MFnData::Type type, MStatus *ReturnStatus=NULL) const`
- `bool accepts (const MTypeId &id, MStatus *ReturnStatus=NULL) const`
- `MObject parent (MStatus *ReturnStatus=NULL) const`
- `MString name (MStatus *ReturnStatus=NULL) const`
- `MString shortName (MStatus *ReturnStatus=NULL) const`
- `MString getAddAttrCmd (bool useLongName=false, MStatus *status=NULL) const`
- `MStatus addToCategory (const MString &category)`
- `MStatus removeFromCategory (const MString &category)`
- `bool hasCategory (const MString &category) const`
- `MStatus getCategories (MStringArray &ReturnStatus) const`
- `MFnAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnBase

Module `OpenMaya`. Source: [class_m_fn_base.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_base.html).

**Public Member Functions**

- `virtual ~MFnBase ()`
- `virtual MFn::Type type () const`
- `bool hasObj (MFn::Type) const`
- `bool hasObj (const MObject &) const`
- `MObject object (MStatus *ReturnStatus=nullptr) const`
- `virtual MStatus setObject (MObject &object)`
- `virtual MStatus setObject (const MObject &object)`

**Static Public Member Functions**

- `static const char * className ()`
- `static const char * typeString (MFn::Type)`
- `static bool isValid (MFn::Type)`

### MFnBlinnShader

Module `OpenMaya`. Inherits `MFnReflectShader`. Source: [class_m_fn_blinn_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_blinn_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnBlinnShader ()`
- `MFnBlinnShader ()`
- `MFnBlinnShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `float eccentricity (MStatus *ReturnStatus=NULL) const`
- `MStatus setEccentricity (const float &eccentricity)`
- `float specularRollOff (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularRollOff (const float &specular_rolloff)`
- `MFnBlinnShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnCamera

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_camera.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_camera.html).

**Public Types**

- `enum FilmFit { kFillFilmFit, kHorizontalFilmFit, kVerticalFilmFit, kOverscanFilmFit, kInvalid }`
- `enum RollOrder { kRotateTranslate, kTranslateRotate }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCamera ()`
- `MFnCamera ()`
- `MFnCamera (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnCamera (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (MObject &parent, MStatus *ReturnStatus=NULL)`
- `MPoint eyePoint (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MVector viewDirection (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MVector upDirection (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MVector rightDirection (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MPoint centerOfInterestPoint (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus set (const MPoint &wsEyeLocation, const MVector &wsViewDirection, const MVector &wsUpDirection, double horizFieldOfView, double aspectRatio)`
- `MStatus setEyePoint (const MPoint &eyeLocation, MSpace::Space space=MSpace::kObject)`
- `MStatus setCenterOfInterestPoint (const MPoint &centerOfInterest, MSpace::Space space=MSpace::kObject)`
- `bool hasSamePerspective (const MDagPath &otherCamera, MStatus *ReturnStatus=NULL)`
- `MStatus copyViewFrom (const MDagPath &otherCamera)`
- `MStatus getFilmFrustum (double distance, MPointArray clipPlanes, bool applyPanZoom=false) const`
- `MStatus getFilmFrustum (double distance, double &hSize, double &vSize, double &hOffset, double &vOffset, bool applyPanZoom=false) const`
- `MStatus getPortFieldOfView (int width, int height, double &horizontalFOV, double &verticalFOV) const`
- `MStatus getViewParameters (double windowAspect, double &apertureX, double &apertureY, double &offsetX, double &offsetY, bool applyOverscan=false, bool applySqueeze=false, bool applyPanZoom=false) const`
- `MStatus getViewingFrustum (double windowAspect, double &left, double &right, double &bottom, double &top, bool applyOverscan=false, bool applySqueeze=false, bool applyPanZoom=false) const`
- `MStatus getRenderingFrustum (double windowAspect, double &left, double &right, double &bottom, double &top) const`
- `MFloatMatrix projectionMatrix (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalFilmAperture (double hFilmAperture)`
- `double horizontalFilmAperture (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalFilmAperture (double vFilmAperture)`
- `double verticalFilmAperture (MStatus *ReturnStatus=NULL) const`
- `MStatus getFilmApertureLimits (double &min, double &max) const`
- `MStatus setAspectRatio (double aspectRatio)`
- `double aspectRatio (MStatus *ReturnStatus=NULL) const`
- `MStatus getAspectRatioLimits (double &min, double &max) const`
- `MStatus setVerticalLock (bool lockFlag)`
- `bool isVerticalLock (MStatus *ReturnStatus=NULL) const`
- `double horizontalFilmOffset (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalFilmOffset (double hOffset)`
- `double verticalFilmOffset (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalFilmOffset (double vOffset)`
- `bool shakeEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setShakeEnabled (bool enabled)`
- `double horizontalShake (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalShake (double hOffset)`
- `double verticalShake (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalShake (double vOffset)`
- `bool shakeOverscanEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setShakeOverscanEnabled (bool enabled)`
- `double shakeOverscan (MStatus *ReturnStatus=NULL) const`
- `MStatus setShakeOverscan (double vOffset)`
- `bool panZoomEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setPanZoomEnabled (bool enabled)`
- `bool renderPanZoom (MStatus *ReturnStatus=NULL) const`
- `MStatus setRenderPanZoom (bool render)`
- `double horizontalPan (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalPan (double hPan)`
- `double verticalPan (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalPan (double vPan)`
- `double zoom (MStatus *ReturnStatus=NULL) const`
- `MStatus setZoom (double zoom)`
- `bool stereoHITEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setStereoHITEnabled (bool enabled)`
- `double stereoHIT (MStatus *ReturnStatus=NULL) const`
- `MStatus setStereoHIT (double vOffset)`
- `MStatus setFilmFit (FilmFit filmFit)`
- `FilmFit filmFit (MStatus *ReturnStatus=NULL) const`
- `MStatus setFilmFitOffset (double filmFitOffset)`
- `double filmFitOffset (MStatus *ReturnStatus=NULL) const`
- `MStatus setOverscan (double overscan)`
- `double overscan (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalRollPivot (double horizontalRollPivot)`
- `double horizontalRollPivot (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalRollPivot (double verticalRollPivot)`
- `double verticalRollPivot (MStatus *ReturnStatus=NULL) const`
- `MStatus setFilmRollValue (double filmRollValue)`
- `double filmRollValue (MStatus *ReturnStatus=NULL) const`
- `MStatus setFilmRollOrder (RollOrder filmRollOrder)`
- `RollOrder filmRollOrder (MStatus *ReturnStatus=NULL) const`
- `MStatus setPreScale (double sf)`
- `double preScale (MStatus *ReturnStatus=NULL) const`
- `MStatus setPostScale (double sf)`
- `double postScale (MStatus *ReturnStatus=NULL) const`
- `MStatus setFilmTranslateH (double translate)`
- `double filmTranslateH (MStatus *ReturnStatus=NULL) const`
- `MStatus setFilmTranslateV (double translate)`
- `double filmTranslateV (MStatus *ReturnStatus=NULL) const`
- `MFloatMatrix postProjectionMatrix (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayGateMask (bool displayGateMask)`
- `bool isDisplayGateMask (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayFilmGate (bool displayFilmGate)`
- `bool isDisplayFilmGate (MStatus *ReturnStatus=NULL) const`
- `MStatus setHorizontalFieldOfView (double fov)`
- `double horizontalFieldOfView (MStatus *ReturnStatus=NULL) const`
- `MStatus setVerticalFieldOfView (double fov)`
- `double verticalFieldOfView (MStatus *ReturnStatus=NULL) const`
- `MStatus setFocalLength (double focalLength)`
- `double focalLength (MStatus *ReturnStatus=NULL) const`
- `MStatus getFocalLengthLimits (double &min, double &max) const`
- `MStatus setLensSqueezeRatio (double lensSqueezeRatio)`
- `double lensSqueezeRatio (MStatus *ReturnStatus=NULL) const`
- `MStatus setClippingPlanes (bool ClippingPlanes)`
- `bool isClippingPlanes (MStatus *ReturnStatus=NULL) const`
- `MStatus setNearClippingPlane (double dNear)`
- `double nearClippingPlane (MStatus *ReturnStatus=NULL) const`
- `MStatus setFarClippingPlane (double dFar)`
- `double farClippingPlane (MStatus *ReturnStatus=NULL) const`
- `MStatus setNearFarClippingPlanes (double dNear, double dFar)`
- `MStatus setDepthOfField (bool depthOfField)`
- `bool isDepthOfField (MStatus *ReturnStatus=NULL) const`
- `MStatus setFStop (double fStop)`
- `double fStop (MStatus *ReturnStatus=NULL) const`
- `MStatus setFocusDistance (double distance)`
- `double focusDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus setNearFocusDistance (double nearFocusDistance)`
- `double nearFocusDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus setFarFocusDistance (double farFocusDistance)`
- `double farFocusDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus computeDepthOfField ()`
- `MStatus computeDepthOfField (double nearLimit)`
- `MStatus setMotionBlur (bool motionBlur)`
- `bool isMotionBlur (MStatus *ReturnStatus=NULL) const`
- `MStatus setShutterAngle (double shutterAngle)`
- `double shutterAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setCenterOfInterest (double dist)`
- `double centerOfInterest (MStatus *ReturnStatus=NULL) const`
- `MStatus setIsOrtho (bool orthoState)`
- `MStatus setIsOrtho (bool orthoState, double useDist)`
- `bool isOrtho (MStatus *ReturnStatus=NULL) const`
- `MStatus setOrthoWidth (double orthoWidth)`
- `double orthoWidth (MStatus *ReturnStatus=NULL) const`
- `MStatus setCameraScale (double scale)`
- `double cameraScale (MStatus *ReturnStatus=NULL) const`
- `MStatus setTumblePivot (const MPoint &point)`
- `MPoint tumblePivot (MStatus *ReturnStatus=NULL) const`
- `double unnormalizedNearClippingPlane (MStatus *ReturnStatus=NULL) const`
- `double unnormalizedFarClippingPlane (MStatus *ReturnStatus=NULL) const`
- `MFnCamera (const MObject &object, MStatus *ret=NULL)`
- `MStatus setStereo (bool stereo)`
- `bool isStereo (MStatus *ReturnStatus=NULL) const`
- `MStatus setParallelView (bool parallelView)`
- `bool isParallelView (MStatus *ReturnStatus=NULL) const`
- `MStatus setEyeOffset (double eyeOffset)`
- `double eyeOffset (MStatus *ReturnStatus=NULL) const`
- `MStatus getFilmFrustum (double distance, MPoint clipPlane[4], bool applyPanZoom=false) const`
- `MStatus setUsePivotAsLocalSpace (const bool how)`
- `bool usePivotAsLocalSpace (MStatus *ReturnStatus=NULL) const`
- `MFloatMatrix postProjectionMatrix (const MDGContext &context, MStatus *ReturnStatus=NULL) const`
- `MFloatMatrix projectionMatrix (const MDGContext &context, MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnCameraSet

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_camera_set.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_camera_set.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCameraSet ()`
- `MFnCameraSet ()`
- `MFnCameraSet (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MDagPathArray &camList, MStatus *returnStatus=NULL)`
- `MObject create (const MString &name, MStatus *ReturnStatus=NULL)`
- `MObject create (const MDagPathArray &camList, const MObjectArray &setList, MStatus *returnStatus=NULL)`
- `MObject create (const MString &name, const MDagPathArray &camList, const MObjectArray &setList, MStatus *returnStatus=NULL)`
- `MStatus appendLayer (const MDagPath &camera, const MObject &set)`
- `MStatus appendLayer (const MDagPath &camera)`
- `MStatus insertLayer (unsigned int index, const MDagPath &camera, const MObject &set)`
- `MStatus insertLayer (unsigned int index, const MDagPath &camera)`
- `MStatus deleteLayer (unsigned int index)`
- `MStatus clear ()`
- `unsigned int getNumLayers (MStatus *returnStatus=NULL)`
- `MStatus setLayerCamera (unsigned int index, const MDagPath &camera)`
- `MStatus getLayerCamera (unsigned int index, MDagPath &camera)`
- `MStatus setLayerSceneData (unsigned int index, const MObject &set)`
- `MStatus getLayerSceneData (unsigned int index, MObject &set)`
- `MStatus setLayerActive (unsigned int index, bool isActive)`
- `bool isLayerActive (unsigned int index, MStatus *returnStatus=NULL)`
- `MStatus setLayerOrder (unsigned int index, int order)`
- `MStatus getLayerOrder (unsigned int index, int &order)`
- `MStatus getSortedIndices (MUintArray &sortedIndices)`
- `MStatus setLayerClearDepthValue (unsigned int index, bool depth)`
- `MStatus getLayerClearDepthValue (unsigned int index, bool &depth) const`
- `MFnCameraSet (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnComponent

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_component.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_component.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnComponent ()`
- `MFnComponent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnComponent ()`
- `int elementCount (MStatus *ReturnStatus=NULL) const`
- `MFn::Type componentType (MStatus *ReturnStatus=NULL) const`
- `bool isEmpty (MStatus *ReturnStatus=NULL) const`
- `bool isEqual (MObject &, MStatus *ReturnStatus=NULL) const`
- `bool isComplete (MStatus *ReturnStatus=NULL) const`
- `MStatus setComplete (bool)`
- `bool hasWeights () const`
- `MWeight weight (int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setWeight (int index, const MWeight &weight)`
- `MStatus setWeights (const MWeight &weight)`
- `MFnComponent (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MFn::Type type (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnComponentListData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_component_list_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_component_list_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnComponentListData ()`
- `MFnComponentListData ()`
- `MFnComponentListData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `bool has (const MObject &obj, MStatus *ReturnStatus=NULL) const`
- `MObject operator[] (unsigned int index) const`
- `MStatus add (MObject &)`
- `MStatus remove (const MObject &)`
- `MStatus remove (unsigned int index)`
- `MStatus clear ()`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnComponentListData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnCompoundAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_compound_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_compound_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCompoundAttribute ()`
- `MFnCompoundAttribute ()`
- `MFnCompoundAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &full, const MString &brief, MStatus *ReturnStatus=NULL)`
- `MStatus addChild (const MObject &child)`
- `MStatus removeChild (const MObject &child)`
- `unsigned int numChildren (MStatus *ReturnStatus=NULL) const`
- `MObject child (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getAddAttrCmds (MStringArray &cmds, bool useLongNames=false) const`
- `MFnCompoundAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnContainerNode

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_container_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_container_node.html).

**Public Types**

- `enum MPublishNodeType { kParentAnchor, kChildAnchor, kGeneric }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnContainerNode ()`
- `MFnContainerNode ()`
- `MFnContainerNode (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getPublishedPlugs (MPlugArray &publishedPlugs, MStringArray &publishedNames) const`
- `MStatus getPublishedNames (MStringArray &publishedNames, bool unboundOnly) const`
- `MStatus getMembers (MObjectArray &members) const`
- `MStatus getSubcontainers (MObjectArray &members) const`
- `MStatus getParentContainer (MObject &parent) const`
- `MStatus getRootTransform (MObject &root) const`
- `MStatus getPublishedNodes (MPublishNodeType type, MStringArray &publishedNames, MObjectArray &nodes) const`
- `MStatus clear ()`
- `MStatus makeCurrent (bool isCurrent=true)`
- `bool isCurrent (MStatus *status=NULL)`
- `MFnContainerNode (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`
- `static MObject getCurrentAsMObject ()`

### MFnDagNode

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_dag_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_dag_node.html).

**Public Types**

- `enum { kNextPos = 0xff }`
- `enum MObjectColorType { kUseDefaultColor = 0, kUseIndexColor, kUseRGBColor }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDagNode ()`
- `MFnDagNode ()`
- `MFnDagNode (MObject &object, MStatus *ret=NULL)`
- `MFnDagNode (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (const MTypeId &typeId, MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MTypeId &typeId, const MString &name, MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &type, MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &type, const MString &name, MObject &parent=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `unsigned int parentCount (MStatus *ReturnStatus=NULL) const`
- `MObject parent (unsigned int i, MStatus *ReturnStatus=NULL) const`
- `MStatus addChild (MObject &child, unsigned int index=kNextPos, bool keepExistingParents=false)`
- `MStatus removeChild (MObject &child)`
- `MStatus removeChildAt (unsigned int index)`
- `unsigned int childCount (MStatus *ReturnStatus=NULL) const`
- `MObject child (unsigned int i, MStatus *ReturnStatus=NULL) const`
- `MObject dagRoot (MStatus *ReturnStatus=NULL)`
- `bool hasParent (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `bool hasChild (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `bool isChildOf (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `bool isParentOf (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `bool inUnderWorld (MStatus *ReturnStatus=NULL) const`
- `bool inModel (MStatus *ReturnStatus=NULL) const`
- `bool isInstanceable (MStatus *ReturnStatus=NULL) const`
- `MStatus setInstanceable (const bool how)`
- `bool isInstanced (bool indirect=true, MStatus *ReturnStatus=NULL) const`
- `bool isInstancedAttribute (const MObject &attr, MStatus *ReturnStatus=NULL) const`
- `unsigned int instanceCount (bool total, MStatus *ReturnStatus=NULL) const`
- `MObject duplicate (bool instance=false, bool instanceLeaf=false, MStatus *ReturnStatus=NULL) const`
- `MStatus getPath (MDagPath &path) const`
- `MStatus getAllPaths (MDagPathArray &paths) const`
- `MString fullPathName (MStatus *ReturnStatus=NULL) const`
- `MString partialPathName (MStatus *ReturnStatus=NULL) const`
- `MMatrix transformationMatrix (MStatus *ReturnStatus=NULL) const`
- `bool isIntermediateObject (MStatus *ReturnStatus=NULL) const`
- `MStatus setIntermediateObject (bool isIntermediate)`
- `int objectColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setObjectColor (int color)`
- `bool usingObjectColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseObjectColor (bool useObjectColor)`
- `MObjectColorType objectColorType (MStatus *ReturnStatus=NULL) const`
- `MStatus setObjectColorType (MObjectColorType type)`
- `MStatus setObjectColor (const MColor &color)`
- `MColor objectColorRGB (MStatus *ReturnStatus=NULL) const`
- `int objectColorIndex (MStatus *ReturnStatus=NULL) const`
- `MColor hiliteColor (MStatus *ReturnStatus=NULL) const`
- `bool usingHiliteColor (MStatus *ReturnStatus=NULL) const`
- `MColor dormantColor (MStatus *ReturnStatus=NULL) const`
- `MColor activeColor (MStatus *ReturnStatus=NULL) const`
- `bool drawOverrideEnabled (MStatus *ReturnStatus=NULL) const`
- `bool drawOverrideIsReference (MStatus *ReturnStatus=NULL) const`
- `bool drawOverrideIsTemplate (MStatus *ReturnStatus=NULL) const`
- `bool drawOverrideColor (MColor &color, MStatus *ReturnStatus=NULL) const`
- `MStatus getConnectedSetsAndMembers (unsigned int instanceNumber, MObjectArray &sets, MObjectArray &comps, bool renderableSetsOnly) const`
- `MBoundingBox boundingBox (MStatus *ReturnStatus=NULL) const`
- `MDagPath dagPath (MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setObject (const MDagPath &path)`
- `MStatus setObject (MObject &object) override`
- `MObject model (MStatus *ReturnStatus=NULL) const`
- `MFnDagNode (const MObject &object, MStatus *ret=NULL)`
- `MStatus setObject (const MObject &object) override`

**Static Public Member Functions**

- `static const char * className ()`

### MFnData

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_data.html).

**Public Types**

- `enum Type { kInvalid, kNumeric, kPlugin, kPluginGeometry, kString, kMatrix, kStringArray, kDoubleArray, kFloatArray, kIntArray, kPointArray, kVectorArray, kMatrixArray, kComponentList, kMesh, kLattice, kNurbsCurve, kNurbsSurface, kSphere, kDynArrayAttrs, kDynSweptGeometry, kSubdSurface, kNObject, kNId, kAny, kFalloffFunction }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnData ()`
- `MFnData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnData ()`
- `MFnData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDependencyNode

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_dependency_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_dependency_node.html).

**Public Types**

- `enum MAttrClass { kLocalDynamicAttr = 1, kNormalAttr, kExtensionAttr, kInvalidAttr }`
- `enum MdgTimerState { kTimerOff, kTimerOn, kTimerUninitialized, kTimerInvalidState }`
- `enum MdgTimerMetric { kTimerMetric_callback, kTimerMetric_compute, kTimerMetric_dirty, kTimerMetric_draw, kTimerMetric_fetch, kTimerMetric_callbackViaAPI, kTimerMetric_callbackNotViaAPI, kTimerMetric_computeDuringCallback, kTimerMetric_computeNotDuringCallback, kTimerMetrics }`
- `enum MdgTimerType { kTimerType_self, kTimerType_inclusive, kTimerType_count, kTimerTypes }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDependencyNode ()`
- `MFnDependencyNode ()`
- `MFnDependencyNode (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MTypeId &typeId, MStatus *ReturnStatus=NULL)`
- `MObject create (const MTypeId &typeId, const MString &name, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &type, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &type, const MString &name, MStatus *ReturnStatus=NULL)`
- `MTypeId typeId (MStatus *ReturnStatus=NULL) const`
- `MString typeName (MStatus *ReturnStatus=NULL) const`
- `MString name (MStatus *ReturnStatus=NULL) const`
- `MString absoluteName (MStatus *ReturnStatus=NULL) const`
- `MString uniqueName (MStatus *ReturnStatus=NULL) const`
- `MString pluginName (MStatus *ReturnStatus=NULL) const`
- `MString setName (const MString &name, bool createNamespace=false, MStatus *ReturnStatus=NULL)`
- `MUuid uuid (MStatus *ReturnStatus=NULL) const`
- `void setUuid (const MUuid &uuid, MStatus *ReturnStatus=NULL)`
- `MStatus getConnections (MPlugArray &array) const`
- `unsigned int attributeCount (MStatus *ReturnStatus=NULL) const`
- `MObject attribute (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MObject reorderedAttribute (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MObject attribute (const MString &attrName, MStatus *ReturnStatus=NULL) const`
- `MAttrClass attributeClass (const MObject &attr, MStatus *ReturnStatus=NULL) const`
- `MStatus getAffectedAttributes (const MObject &attr, MObjectArray &affectedAttributes) const`
- `MStatus getAffectedByAttributes (const MObject &attr, MObjectArray &affectedByAttributes) const`
- `MPlug findPlug (const MObject &attr, bool wantNetworkedPlug, MStatus *ReturnStatus=NULL) const`
- `MPlug findPlug (const MString &attrName, bool wantNetworkedPlug, MStatus *ReturnStatus=NULL) const`
- `MPlug findPlug (const MObject &attr, MStatus *ReturnStatus=NULL) const`
- `MPlug findPlug (const MString &attrName, MStatus *ReturnStatus=NULL) const`
- `MStatus addAttribute (const MObject &attr)`
- `MStatus removeAttribute (const MObject &attr)`
- `MPxNode * userNode (MStatus *ReturnStatus=NULL) const`
- `bool isFromReferencedFile (MStatus *ReturnStatus=NULL) const`
- `bool isShared (MStatus *ReturnStatus=NULL) const`
- `bool isTrackingEdits (MStatus *ReturnStatus=NULL) const`
- `bool hasUniqueName (MStatus *ReturnStatus=NULL) const`
- `MString parentNamespace (MStatus *ReturnStatus=NULL) const`
- `bool isLocked (MStatus *ReturnStatus=NULL) const`
- `MStatus setLocked (bool locked)`
- `bool isNewAttribute (const MObject &attr, MStatus *ReturnStatus=NULL) const`
- `MStatus setFlag (unsigned int flag, bool state)`
- `bool isFlagSet (unsigned int flag, MStatus *ReturnStatus=NULL) const`
- `bool isDefaultNode (MStatus *ReturnStatus=NULL) const`
- `MStatus setDoNotWrite (bool flag)`
- `bool canBeWritten (MStatus *ReturnStatus=NULL) const`
- `bool hasAttribute (const MString &name, MStatus *ReturnStatus=NULL) const`
- `MObject getAliasAttr (bool force, MStatus *ReturnStatus=NULL)`
- `bool setAlias (const MString &alias, const MString &name, const MPlug &plug, bool add=true, MStatus *ReturnStatus=NULL)`
- `bool findAlias (const MString &alias, MObject &attrObj, MStatus *ReturnStatus=NULL) const`
- `bool getAliasList (MStringArray &strArray, MStatus *ReturnStatus=NULL)`
- `MString plugsAlias (const MPlug &plug, MStatus *ReturnStatus=NULL)`
- `MStatus setIcon (const MString &filename)`
- `MString icon (MStatus *ReturnStatus=NULL) const`
- `MStatus getExternalContent (MExternalContentInfoTable &table) const`
- `MStatus addExternalContentForFileAttr (MExternalContentInfoTable &table, const MObject &attr) const`
- `MStatus setExternalContentForFileAttr (const MObject &attr, const MExternalContentLocationTable &table)`
- `MStatus setExternalContent (const MExternalContentLocationTable &table)`
- `bool affectsAnimation (MStatus *ReturnStatus=NULL) const`
- `MStatus setAffectsAnimation ()`
- `MString setName (const MString &name, MStatus *ReturnStatus)`
- `MFnDependencyNode (const MObject &object, MStatus *ReturnStatus=NULL)`
- `bool getPlugsAlias (const MPlug &plug, MString &aliasName, MStatus *ReturnStatus=NULL)`
- `virtual const adsk::Data::Associations * metadata (MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setMetadata (const adsk::Data::Associations &)`
- `virtual MStatus deleteMetadata ()`
- `virtual MStatus validateMetadata (MString &errors) const`
- `MStatus dgTimerOn ()`
- `MStatus dgTimerOff ()`
- `MdgTimerState dgTimerQueryState (MStatus *ReturnStatus=NULL)`
- `MStatus dgTimerReset ()`
- `double dgTimer (const MdgTimerMetric timerMetric, const MdgTimerType timerType, MStatus *ReturnStatus=NULL) const`
- `MStatus dgCallbacks (const MdgTimerType type, MStringArray &callbackName, MDoubleArray &value)`
- `MStatus dgCallbackIds (const MdgTimerType type, const MString &callbackName, MCallbackIdArray &callbackId, MDoubleArray &value)`
- `MStatus addAttribute (const MObject &attr, MAttrClass type)`
- `MStatus removeAttribute (const MObject &attr, MAttrClass type)`

**Static Public Member Functions**

- `static const char * className ()`
- `static MString classification (const MString &nodeTypeName)`
- `static unsigned int allocateFlag (const MString pluginName, MStatus *ReturnStatus=NULL)`
- `static MStatus deallocateFlag (const MString pluginName, unsigned int flag)`
- `static MStatus deallocateAllFlags (const MString pluginName)`
- `static void enableDGTiming (bool enable)`

### MFnDirectionalLight

Module `OpenMaya`. Inherits `MFnNonExtendedLight`. Source: [class_m_fn_directional_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_directional_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDirectionalLight ()`
- `MFnDirectionalLight ()`
- `MFnDirectionalLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnDirectionalLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `float shadowAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setShadowAngle (const float &shadow_angle)`
- `bool useLightPosition (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseLightPosition (const bool use_light_position)`
- `MFnDirectionalLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDoubleArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_double_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_double_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDoubleArrayData ()`
- `MFnDoubleArrayData ()`
- `MFnDoubleArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `double operator[] (unsigned int index) const`
- `MStatus set (double element, unsigned int index)`
- `MStatus copyTo (MDoubleArray &) const`
- `MStatus set (const MDoubleArray &newArray)`
- `MDoubleArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MDoubleArray &in, MStatus *ReturnStatus=NULL)`
- `MFnDoubleArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `double & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDoubleIndexedComponent

Module `OpenMaya`. Inherits `MFnComponent`. Source: [class_m_fn_double_indexed_component.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_double_indexed_component.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDoubleIndexedComponent ()`
- `MFnDoubleIndexedComponent ()`
- `MFnDoubleIndexedComponent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MFn::Type compType, MStatus *ReturnStatus=NULL)`
- `MStatus addElement (int uIndex, int vIndex)`
- `MStatus addElements (const MIntArray &uIndexArray, const MIntArray &vIndexArray)`
- `MStatus getElement (int index, int &uIndex, int &vIndex) const`
- `MStatus getElements (MIntArray &uIndexArray, MIntArray &vIndexArray) const`
- `MStatus setCompleteData (int maxU, int maxV)`
- `MStatus getCompleteData (int &maxU, int &maxV) const`
- `MFnDoubleIndexedComponent (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnEnumAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_enum_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_enum_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnEnumAttribute ()`
- `MFnEnumAttribute ()`
- `MFnEnumAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, short defaultValue=0, MStatus *ReturnStatus=NULL)`
- `MStatus addField (const MString &fieldString, short index)`
- `MString fieldName (short index, MStatus *ReturnStatus=NULL) const`
- `short fieldIndex (const MString &fieldString, MStatus *ReturnStatus=NULL) const`
- `MStatus getMin (short &minValue) const`
- `MStatus getMax (short &maxValue) const`
- `MStatus setDefault (short index)`
- `MStatus setDefault (const MString &fieldString)`
- `MStatus getDefault (short &index) const`
- `MString defaultValue (MStatus *ReturnStatus=NULL) const`
- `MFnEnumAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getDefault (MString &fieldString) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnExpression

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_expression.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_expression.html).

**Public Types**

- `enum UnitConversion { kAll, kNone, kAngularOnly }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnExpression ()`
- `MFnExpression ()`
- `MFnExpression (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &expression, MObject &object=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MString expression (MStatus *ReturnStatus=NULL)`
- `MStatus setExpression (const MString &expression)`
- `MStatus getDefaultObject (MObject &object)`
- `MStatus setDefaultObject (MObject &object)`
- `bool isAnimated (MStatus *ReturnStatus=NULL)`
- `MStatus setAnimated (bool value=false)`
- `MStatus evaluate (MDoubleArray &result)`
- `UnitConversion unitConversion (MStatus *ReturnStatus=NULL) const`
- `MStatus setUnitConversion (UnitConversion conversion)`
- `MFnExpression (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getExpression (MString &expression)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnFalloffData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_falloff_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_falloff_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnFalloffData ()`
- `MFnFalloffData ()`
- `MFnFalloffData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFalloffFunction::ReturnValue call (MFalloffContext &accessor)`
- `MObject create (MSharedPtr< MFalloffFunction >, MStatus *ReturnStatus=NULL)`
- `MFnFalloffData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnFloatArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_float_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_float_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnFloatArrayData ()`
- `MFnFloatArrayData ()`
- `MFnFloatArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `float operator[] (unsigned int index) const`
- `MStatus set (float element, unsigned int index)`
- `MStatus copyTo (MFloatArray &) const`
- `MStatus set (const MFloatArray &newArray)`
- `MFloatArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MFloatArray &in, MStatus *ReturnStatus=NULL)`
- `MFnFloatArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `float & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnFloatVectorArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_float_vector_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_float_vector_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnFloatVectorArrayData ()`
- `MFnFloatVectorArrayData ()`
- `MFnFloatVectorArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (MFloatVector &element, unsigned int index)`
- `MStatus copyTo (MFloatVectorArray &) const`
- `MStatus set (const MFloatVectorArray &newArray)`
- `MFloatVectorArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MFloatVectorArray &in, MStatus *ReturnStatus=NULL)`
- `MFnFloatVectorArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `const MFloatVector & operator[] (unsigned int index) const`
- `MFloatVector & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnGenericAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_generic_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_generic_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnGenericAttribute ()`
- `MFnGenericAttribute ()`
- `MFnGenericAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &full, const MString &brief, MStatus *ReturnStatus=NULL)`
- `MStatus addDataAccept (MFnData::Type newType)`
- `MStatus addNumericDataAccept (MFnNumericData::Type newType)`
- `MStatus addAccept (const MTypeId &id)`
- `MStatus removeDataAccept (MFnData::Type oldType)`
- `MStatus removeNumericDataAccept (MFnNumericData::Type oldType)`
- `MStatus removeAccept (const MTypeId &id)`
- `MFnGenericAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus addAccept (MFnData::Type newType)`
- `MStatus addAccept (MFnNumericData::Type newType)`
- `MStatus removeAccept (MFnData::Type oldType)`
- `MStatus removeAccept (MFnNumericData::Type oldType)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnGeometryData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_geometry_data.html).

**Public Types**

- `enum ComponentTagCategory`
- `enum SubsetState`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnGeometryData ()`
- `MFnGeometryData ()`
- `MFnGeometryData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus setMatrix (const MMatrix &)`
- `MStatus getMatrix (MMatrix &) const`
- `bool matrixIsIdentity (MStatus *ReturnStatus=NULL) const`
- `bool matrixIsNotIdentity (MStatus *ReturnStatus=NULL) const`
- `bool hasObjectGroup (unsigned int id, MStatus *ReturnStatus=NULL) const`
- `MStatus addObjectGroup (unsigned int)`
- `MStatus removeObjectGroup (unsigned int)`
- `MStatus changeObjectGroupId (unsigned int, unsigned int)`
- `unsigned int objectGroupCount (MStatus *ReturnStatus=NULL) const`
- `unsigned int objectGroup (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MFn::Type objectGroupType (unsigned int, MStatus *ReturnStatus=NULL) const`
- `MObject objectGroupComponent (unsigned int, MStatus *ReturnStatus=NULL) const`
- `MStatus setObjectGroupComponent (unsigned int, MObject &)`
- `MStatus addObjectGroupComponent (unsigned int, MObject &)`
- `MStatus removeObjectGroupComponent (unsigned int, MObject &)`
- `SubsetState objectGroupSubsetState (unsigned int, MStatus *ReturnStatus=NULL) const`
- `MStatus copyObjectGroups (MObject &inGeom)`
- `MStatus componentTags (MStringArray &keys) const`
- `bool hasComponentTag (const MString &key, MStatus *ReturnStatus=NULL) const`
- `MStatus addComponentTag (const MString &key)`
- `MStatus removeComponentTag (const MString &key)`
- `MStatus renameComponentTag (const MString &key, const MString &newKey)`
- `MObject componentTagContents (const MString &key, MStatus *ReturnStatus=NULL) const`
- `MStatus setComponentTagContents (const MString &key, MObject &)`
- `MFn::Type componentTagType (const MString &key, MStatus *ReturnStatus=NULL) const`
- `ComponentTagCategory componentTagCategory (const MString &key, MStatus *ReturnStatus=NULL) const`
- `SubsetState componentTagExpressionSubsetState (const MString &expression, ComponentTagCategory ctg=MFnGeometryData::ComponentTagCategory::kAuto, MStatus *ReturnStatus=NULL) const`
- `MObject resolveComponentTagExpression (const MString &expression, ComponentTagCategory ctg=MFnGeometryData::ComponentTagCategory::kAuto, MStatus *ReturnStatus=NULL) const`
- `MFnGeometryData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnIntArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_int_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_int_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnIntArrayData ()`
- `MFnIntArrayData ()`
- `MFnIntArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `int operator[] (unsigned int index) const`
- `MStatus set (int element, unsigned int index)`
- `MStatus copyTo (MIntArray &) const`
- `MStatus set (const MIntArray &newArray)`
- `MIntArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MIntArray &in, MStatus *ReturnStatus=NULL)`
- `MFnIntArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `int & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLambertShader

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_lambert_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lambert_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLambertShader ()`
- `MFnLambertShader ()`
- `MFnLambertShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `short refractedRayDepthLimit (MStatus *ReturnStatus=NULL) const`
- `MStatus setRefractedRayDepthLimit (const short &new_limit)`
- `float refractiveIndex (MStatus *ReturnStatus=NULL) const`
- `MStatus setRefractiveIndex (const float &refractive_index)`
- `bool rtRefractedColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setRtRefractedColor (const bool &rt_refracted_color)`
- `float diffuseCoeff (MStatus *ReturnStatus=NULL) const`
- `MStatus setDiffuseCoeff (const float &diffuse_coeff)`
- `MColor color (MStatus *ReturnStatus=NULL) const`
- `MStatus setColor (const MColor &col)`
- `MColor transparency (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransparency (const MColor &transp)`
- `MColor ambientColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setAmbientColor (const MColor &ambient_color)`
- `MColor incandescence (MStatus *ReturnStatus=NULL) const`
- `MStatus setIncandescence (const MColor &incand)`
- `float translucenceCoeff (MStatus *ReturnStatus=NULL) const`
- `MStatus setTranslucenceCoeff (const float &translucence_coeff)`
- `float glowIntensity (MStatus *ReturnStatus=NULL) const`
- `MStatus setGlowIntensity (const float &glow_intensity)`
- `bool hideSource (MStatus *ReturnStatus=NULL) const`
- `MStatus setHideSource (const bool &hide_source)`
- `MFnLambertShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLatticeData

Module `OpenMaya`. Inherits `MFnGeometryData`. Source: [class_m_fn_lattice_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_lattice_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLatticeData ()`
- `MFnLatticeData ()`
- `MFnLatticeData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject lattice (MStatus *ReturnStatus=NULL)`
- `MFnLatticeData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLayeredShader

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_layered_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_layered_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLayeredShader ()`
- `MFnLayeredShader ()`
- `MFnLayeredShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `short compositingFlag (MStatus *ReturnStatus=NULL) const`
- `MStatus setCompositingFlag (const short &cflag)`
- `MColor color (unsigned int lIndex=0, MStatus *ReturnStatus=NULL) const`
- `MStatus setColor (const MColor &col, unsigned int lIndex=0)`
- `MColor transparency (unsigned int lIndex=0, MStatus *ReturnStatus=NULL) const`
- `MStatus setTransparency (const MColor &transp, unsigned int lIndex=0)`
- `MColor glowColor (unsigned int lIndex=0, MStatus *ReturnStatus=NULL) const`
- `MStatus setGlowColor (const MColor &glow_color, unsigned int lIndex=0)`
- `MColor hardwareColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setHardwareColor (const MColor &hardware_color)`
- `MColor hardwareShader (MStatus *ReturnStatus=NULL) const`
- `MStatus setHardwareShader (const MColor &hardware_shader)`
- `MFnLayeredShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLight

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLight ()`
- `MFnLight ()`
- `MFnLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnLight (const MDagPath &object, MStatus *ret=NULL)`
- `MColor color (MStatus *ReturnStatus=NULL) const`
- `MStatus setColor (const MColor &col)`
- `float intensity (MStatus *ReturnStatus=NULL) const`
- `MStatus setIntensity (const float &intens)`
- `bool useRayTraceShadows (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseRayTraceShadows (const bool &useRayTraceShadows)`
- `MColor shadowColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setShadowColor (const MColor &shadow_color)`
- `double centerOfIllumination (MStatus *ReturnStatus=NULL) const`
- `MStatus setCenterOfIllumination (const double &dist)`
- `short numShadowSamples (MStatus *ReturnStatus=NULL) const`
- `MStatus setNumShadowSamples (const short &num_shadow_samples)`
- `short rayDepthLimit (MStatus *ReturnStatus=NULL) const`
- `MStatus setRayDepthLimit (const short &rayDepthLimit)`
- `MColor opticalFXvisibility (MStatus *ReturnStatus=NULL) const`
- `MStatus setOpticalFXvisibility (const MColor &visibility)`
- `MColor lightIntensity (MStatus *ReturnStatus=NULL) const`
- `MFloatVector lightDirection (int instance, MSpace::Space space=MSpace::kWorld, MStatus *ReturnStatus=NULL) const`
- `MFloatVector lightDirection (MStatus *ReturnStatus=NULL) const`
- `bool lightAmbient (MStatus *ReturnStatus=NULL) const`
- `bool lightDiffuse (MStatus *ReturnStatus=NULL) const`
- `bool lightSpecular (MStatus *ReturnStatus=NULL) const`
- `MFnLight (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnLightDataAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_light_data_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_light_data_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnLightDataAttribute ()`
- `MFnLightDataAttribute ()`
- `MFnLightDataAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MObject &direction, const MObject &intensity, const MObject &ambient, const MObject &diffuse, const MObject &specular, const MObject &shadowFraciton, const MObject &preShadowIntensity, const MObject &blindData, MStatus *ReturnStatus=NULL)`
- `MStatus getDefault (float &defDirectionX, float &defDirectionY, float &defDirectionZ, float &defIntensityR, float &defIntensityG, float &defIntensityB, bool &defAmbient, bool &defDiffuse, bool &defSpecular, float &defShadowFraction, float &defPreShadowIntensity, void *&defBlindData)`
- `MStatus setDefault (float defDirectionX, float defDirectionY, float defDirectionZ, float defIntensityR, float defIntensityG, float defIntensityB, bool defAmbient, bool defDiffuse, bool defSpecular, float defShadowFraction, float defPreShadowIntensity, void *defBlindData)`
- `MObject child (unsigned int index, MStatus *returnStatus)`
- `MFnLightDataAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMatrixArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_matrix_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_matrix_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMatrixArrayData ()`
- `MFnMatrixArrayData ()`
- `MFnMatrixArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (MMatrix &element, unsigned int index)`
- `MStatus copyTo (MMatrixArray &) const`
- `MStatus set (const MMatrixArray &newArray)`
- `MMatrixArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MMatrixArray &in, MStatus *ReturnStatus=NULL)`
- `MFnMatrixArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MMatrix & operator[] (unsigned int index)`
- `const MMatrix & operator[] (unsigned int index) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMatrixAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_matrix_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_matrix_attribute.html).

**Public Types**

- `enum Type { kFloat, kDouble }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMatrixAttribute ()`
- `MFnMatrixAttribute ()`
- `MFnMatrixAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, Type matrixType=kDouble, MStatus *ReturnStatus=NULL)`
- `MStatus getDefault (MMatrix &def)`
- `MStatus getDefault (MFloatMatrix &def)`
- `MStatus setDefault (const MMatrix &def)`
- `MStatus setDefault (const MFloatMatrix &def)`
- `MFnMatrixAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMatrixData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_matrix_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_matrix_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMatrixData ()`
- `MFnMatrixData ()`
- `MFnMatrixData (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool isTransformation (MStatus *ReturnStatus=NULL) const`
- `MTransformationMatrix transformation (MStatus *ReturnStatus=NULL) const`
- `const MMatrix & matrix (MStatus *ReturnStatus=NULL) const`
- `MStatus set (const MTransformationMatrix &transformation)`
- `MStatus set (const MMatrix &matrix)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MMatrix &, MStatus *ReturnStatus=NULL)`
- `MObject create (const MTransformationMatrix &, MStatus *ReturnStatus=NULL)`
- `MFnMatrixData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMesh

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_mesh.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_mesh.html).

**Public Types**

- `enum MColorRepresentation { kAlpha = 1, kRGB = 3, kRGBA = 4 }`
- `enum SplitPlacement { kOnEdge, kInternalPoint, kInvalid }`
- `enum BoolOperation { kUnion = 1, kDifference, kIntersection }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMesh ()`
- `MFnMesh ()`
- `MFnMesh (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnMesh (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (int numVertices, int numPolygons, const MFloatPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (int numVertices, int numPolygons, const MPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (int numVertices, int numPolygons, const MPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, bool storeDoubles, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (int numVertices, int numPolygons, const MFloatPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, const MFloatArray &uArray, const MFloatArray &vArray, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (int numVertices, int numPolygons, const MPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, const MFloatArray &uArray, const MFloatArray &vArray, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MFloatPointArray &vertexArray, const MIntArray &edges, const MIntArray &edgeConnectsCount, const MIntArray &edgeFaceConnects, const MIntArray &edgeFaceDesc, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MPointArray &vertexArray, const MIntArray &edges, const MIntArray &edgeConnectsCount, const MIntArray &edgeFaceConnects, const MIntArray &edgeFaceDesc, bool storeDoubles, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject generateSmoothMesh (MObject parentOrOwner=MObject::kNullObj, MMeshSmoothOptions *options=NULL, MStatus *ReturnStatus=NULL)`
- `MStatus getSmoothMeshDisplayOptions (MMeshSmoothOptions &options) const`
- `MStatus setSmoothMeshDisplayOptions (const MMeshSmoothOptions &options)`
- `MObject addPolygon (const MPointArray &vertexArray, bool mergeVertices=true, double pointTolerance=kMFnMeshPointTolerance, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject addPolygon (const MPointArray &vertexArray, int &faceIndex, bool mergeVertices=true, double pointTolerance=kMFnMeshPointTolerance, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject addPolygon (const MPointArray &vertexArray, const MIntArray &loopCounts, int &faceIndex, bool mergeVertices=true, double pointTolerance=kMFnMeshPointTolerance, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus addHoles (int faceIndex, const MPointArray &vertexArray, const MIntArray &loopCounts, bool mergeVertices=true, double pointTolerance=kMFnMeshPointTolerance)`
- `MStatus deleteFace (int index, MDGModifier *modifier=NULL)`
- `MStatus deleteEdge (int index, MDGModifier *modifier=NULL)`
- `MStatus deleteVertex (int index, MDGModifier *modifier=NULL)`
- `MStatus split (MIntArray &placements, MIntArray &edgeList, MFloatArray &edgeFactors, MFloatPointArray &internalPoints)`
- `MStatus subdivideFaces (MIntArray &faceList, int divisionCount)`
- `MStatus subdivideEdges (MIntArray &edgeList, int divisionCount)`
- `MStatus extrudeFaces (MIntArray &faceList, int extrusionCount, MFloatVector *translation, bool extrudeTogether, float thickness=0.0f, float offset=0.0f)`
- `MStatus extrudeEdges (MIntArray &edgeList, int extrusionCount, MFloatVector *translation, bool extrudeTogether, float thickness=0.0f, float offset=0.0f)`
- `MStatus duplicateFaces (MIntArray &faceList, MFloatVector *translation)`
- `MStatus extractFaces (MIntArray &faceList, MFloatVector *translation)`
- `MStatus collapseFaces (MIntArray &faceList)`
- `MStatus collapseEdges (MIntArray &edgeList)`
- `int numVertices (MStatus *ReturnStatus=NULL) const`
- `int numEdges (MStatus *ReturnStatus=NULL) const`
- `int numPolygons (MStatus *ReturnStatus=NULL) const`
- `int numFaceVertices (MStatus *ReturnStatus=NULL) const`
- `int polygonVertexCount (int polygonId, MStatus *ReturnStatus=NULL) const`
- `int numUVs (MStatus *ReturnStatus=NULL) const`
- `int numUVs (const MString &uvSet, MStatus *ReturnStatus=NULL) const`
- `int numColors (MStatus *ReturnStatus=NULL) const`
- `int numColors (const MString &colorSet, MStatus *ReturnStatus=NULL) const`
- `int numNormals (MStatus *ReturnStatus=NULL) const`
- `bool hasColorChannels (const MString &colorSet, MStatus *ReturnStatus=NULL) const`
- `bool hasAlphaChannels (const MString &colorSet, MStatus *ReturnStatus=NULL) const`
- `MColorRepresentation getColorRepresentation (const MString &colorSet, MStatus *ReturnStatus=NULL) const`
- `bool isColorClamped (const MString &colorSet, MStatus *ReturnStatus=NULL) const`
- `MStatus setIsColorClamped (const MString &colorSet, bool clamped)`
- `MStatus getTriangles (MIntArray &triangleCounts, MIntArray &triangleVertices) const`
- `MStatus getTriangleOffsets (MIntArray &triangleCounts, MIntArray &triangleIndices) const`
- `MStatus booleanOp (BoolOperation op, MFnMesh &mesh1, MFnMesh &mesh2)`
- `MStatus booleanOps (BoolOperation op, const MObjectArray &meshes, bool useLegacy=false)`
- `bool closestIntersection (const MFloatPoint &raySource, const MFloatVector &rayDirection, const MIntArray *faceIds, const MIntArray *triIds, bool idsSorted, MSpace::Space space, float maxParam, bool testBothDirections, MMeshIsectAccelParams *accelerator, MFloatPoint &hitPoint, float *hitRayParam, int *hitFace, int *hitTriangle, float *hitBary1, float *hitBary2, float tolerance=1e-6, MStatus *ReturnStatus=NULL)`
- `bool anyIntersection (const MFloatPoint &raySource, const MFloatVector &rayDirection, const MIntArray *faceIds, const MIntArray *triIds, bool idsSorted, MSpace::Space space, float maxParam, bool testBothDirections, MMeshIsectAccelParams *accelerator, MFloatPoint &hitPoint, float *hitRayParam, int *hitFace, int *hitTriangle, float *hitBary1, float *hitBary2, float tolerance=1e-6, MStatus *ReturnStatus=NULL)`
- `bool allIntersections (const MFloatPoint &raySource, const MFloatVector &rayDirection, const MIntArray *faceIds, const MIntArray *triIds, bool idsSorted, MSpace::Space space, float maxParam, bool testBothDirections, MMeshIsectAccelParams *accelerator, bool sortHits, MFloatPointArray &hitPoints, MFloatArray *hitRayParams, MIntArray *hitFaces, MIntArray *hitTriangles, MFloatArray *hitBary1, MFloatArray *hitBary2, float tolerance=1e-6, MStatus *ReturnStatus=NULL)`
- `MStatus sortIntersectionFaceTriIds (MIntArray *faceIds, MIntArray *triIds)`
- `MStatus freeCachedIntersectionAccelerator ()`
- `MString cachedIntersectionAcceleratorInfo (MStatus *ReturnStatus=NULL)`
- `bool intersect (const MPoint &raySource, const MVector &rayDirection, MPointArray &points, double tolerance=kMFnMeshPointTolerance, MSpace::Space space=MSpace::kObject, MIntArray *polygonIds=NULL, MStatus *ReturnStatus=NULL) const`
- `MStatus getClosestPointAndNormal (const MPoint &toThisPoint, MPoint &theClosestPoint, MVector &theNormal, MSpace::Space space=MSpace::kObject, int *closestPolygon=NULL, MMeshIsectAccelParams *accelParams=NULL) const`
- `MStatus getClosestPoint (const MPoint &toThisPoint, MPoint &theClosestPoint, MSpace::Space space=MSpace::kObject, int *closestPolygon=NULL, MMeshIsectAccelParams *accelerator=NULL) const`
- `MStatus getClosestNormal (const MPoint &toThisPoint, MVector &theNormal, MSpace::Space space=MSpace::kObject, int *closestPolygon=NULL, MMeshIsectAccelParams *accelerator=NULL) const`
- `MStatus getClosestUVs (const float2 &toThisUVPoint, MIntArray &uvIds, const MString *uvSet=NULL) const`
- `MStatus intersectFaceAtUV (const float2 &toThisUVPoint, int &faceId, const MString *uvSet=NULL) const`
- `MStatus getConnectedShaders (unsigned int instanceNumber, MObjectArray &shaders, MIntArray &indices) const`
- `MObject copy (const MObject &source, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus updateSurface ()`
- `MStatus syncObject ()`
- `const float * getRawPoints (MStatus *) const`
- `const double * getRawDoublePoints (MStatus *) const`
- `const float * getRawNormals (MStatus *) const`
- `MStatus getPoints (MFloatPointArray &vertexArray, MSpace::Space space=MSpace::kObject) const`
- `MStatus getPoints (MPointArray &vertexArray, MSpace::Space space=MSpace::kObject) const`
- `MStatus setPoints (MFloatPointArray &vertexArray, MSpace::Space space=MSpace::kObject)`
- `MStatus setPoints (MPointArray &vertexArray, MSpace::Space space=MSpace::kObject)`
- `MStatus getVertices (MIntArray &vertexCount, MIntArray &vertexList) const`
- `MStatus getPolygonVertices (int polygonId, MIntArray &vertexList) const`
- `MStatus getPolygonTriangleVertices (int polygonId, int triangleId, int triangleVertices[3]) const`
- `MStatus setPoint (int vertexId, const MPoint &pos, MSpace::Space space=MSpace::kObject)`
- `MStatus getPoint (int vertexId, MPoint &pos, MSpace::Space space=MSpace::kObject) const`
- `MStatus getNormals (MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject) const`
- `MStatus setNormals (MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject)`
- `MStatus getFaceVertexNormal (int faceIndex, int vertexIndex, MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `MStatus getFaceVertexNormals (int faceIndex, MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject) const`
- `MStatus getNormalIds (MIntArray &normalIdCounts, MIntArray &normalIds) const`
- `MStatus getFaceNormalIds (int faceIndex, MIntArray &normals) const`
- `MStatus setFaceVertexNormal (MVector &normal, int faceIndex, int vertexIndex, MSpace::Space space=MSpace::kObject, MDGModifier *modifier=NULL)`
- `MStatus setVertexNormal (MVector &normal, int vertexIndex, MSpace::Space space=MSpace::kObject, MDGModifier *modifier=NULL)`
- `MStatus setFaceVertexNormals (MVectorArray &normalArray, MIntArray &faceList, MIntArray &vertexList, MSpace::Space space=MSpace::kObject)`
- `MStatus setVertexNormals (MVectorArray &normalArray, MIntArray &vertexList, MSpace::Space space=MSpace::kObject)`
- `MStatus getVertexNormal (int vertexId, MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `MStatus getVertexNormal (int vertexId, bool angleWeighted, MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `MStatus getVertexNormals (bool angleWeighted, MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject) const`
- `MStatus getPolygonNormal (int polygonId, MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `bool isNormalLocked (int normalId, MStatus *ReturnStatus=NULL) const`
- `MStatus lockVertexNormals (MIntArray &vertexList)`
- `MStatus lockFaceVertexNormals (MIntArray &faceList, MIntArray &vertexList)`
- `MStatus unlockVertexNormals (MIntArray &vertexList)`
- `MStatus unlockFaceVertexNormals (MIntArray &faceList, MIntArray &vertexList)`
- `int getTangentId (int faceIndex, int vertexIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus getTangents (MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `MStatus getFaceVertexTangent (int faceIndex, int vertexIndex, MVector &tangent, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `MStatus getFaceVertexTangents (int faceIndex, MFloatVectorArray &tangents, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `MStatus getBinormals (MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `MStatus getFaceVertexBinormal (int faceIndex, int vertexIndex, MVector &normal, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `MStatus getFaceVertexBinormals (int faceIndex, MFloatVectorArray &normals, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL) const`
- `bool isRightHandedTangent (int tangentId, const MString *uvSetName=NULL, MStatus *ReturnStatus=NULL) const`
- `bool isPolygonConvex (int faceIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus getEdgeVertices (int edgeId, int2 &vertexList) const`
- `bool isEdgeSmooth (int edgeId, MStatus *ReturnStatus=NULL) const`
- `MStatus setEdgeSmoothing (int edgeId, bool smooth=true)`
- `MStatus setEdgeSmoothings (const MIntArray &edgeIds, const MIntArray &smooths)`
- `MStatus cleanupEdgeSmoothing ()`
- `MStatus getCreaseEdges (MUintArray &edgeIds, MDoubleArray &creaseData) const`
- `MStatus setCreaseEdges (const MUintArray &edgeIds, const MDoubleArray &creaseData)`
- `MUintArray getInvisibleFaces (MStatus *ReturnStatus=NULL) const`
- `MStatus setInvisibleFaces (const MUintArray &faceIds, bool makeVisible=false)`
- `MStatus getCreaseVertices (MUintArray &vertexIds, MDoubleArray &creaseData) const`
- `MStatus setCreaseVertices (const MUintArray &vertexIds, const MDoubleArray &creaseData)`
- `int numUVSets (MStatus *ReturnStatus=NULL) const`
- `MString createUVSetWithName (const MString &uvSetName, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL, const MUintArray *instances=NULL)`
- `MString copyUVSetWithName (const MString &fromName, const MString &toName, MDGModifier *modifier=NULL, MStatus *ReturnStatus=NULL)`
- `MStatus renameUVSet (const MString &origName, const MString &newName, MDGModifier *modifier=NULL)`
- `MStatus deleteUVSet (const MString &setName, MDGModifier *modifier=NULL, MSelectionList *currentSelection=NULL)`
- `MStatus setCurrentUVSetName (const MString &setName, MDGModifier *modifier=NULL, MSelectionList *currentSelection=NULL)`
- `MString currentUVSetName (MStatus *ReturnStatus=NULL, int instance=kMFnMeshInstanceUnspecified) const`
- `MStatus getUVSetNames (MStringArray &setNames) const`
- `MStatus getUVSetFamilyNames (MStringArray &familyNames) const`
- `MStatus getUVSetsInFamily (const MString &familyName, MStringArray &setNames) const`
- `bool isUVSetPerInstance (const MString &name, MStatus *ReturnStatus=NULL) const`
- `MStatus getFaceUVSetNames (int polygonId, MStringArray &setNames) const`
- `MStatus getAssociatedUVSetTextures (const MString uvSetName, MObjectArray &textures) const`
- `MStatus getAssociatedUVSetInstances (const MString &uvSetName, MIntArray &instances) const`
- `MStatus setUVs (const MFloatArray &uArray, const MFloatArray &vArray, const MString *uvSet=NULL)`
- `MStatus setSomeUVs (const MIntArray &uvIds, const MFloatArray &uArray, const MFloatArray &vArray, const MString *uvSet=NULL)`
- `MStatus getUVs (MFloatArray &uArray, MFloatArray &vArray, const MString *uvSet=NULL) const`
- `MStatus setUV (int uvId, float u, float v, const MString *uvSet=NULL)`
- `MStatus getUV (int uvId, float &u, float &v, const MString *uvSet=NULL) const`
- `MStatus getPointAtUV (int polygonId, MPoint &theClosestPoint, float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, float tolerance=0.0)`
- `MStatus getPointsAtUV (MIntArray &polygonIds, MPointArray &points, const float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, float tolerance=0.0) const`
- `MStatus getUVAtPoint (MPoint &pt, float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, int *closestPolygon=NULL)`
- `MStatus getAxisAtPoint (MPoint &pt, MVector &normal, MVector &uTangent, MVector &vTangent, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, int *closestPolygon=NULL)`
- `MStatus getPolygonUV (int polygonId, int vertexIndex, float &u, float &v, const MString *uvSet=NULL) const`
- `MStatus getPolygonUVid (int polygonId, int vertexIndex, int &uvId, const MString *uvSet=NULL) const`
- `MStatus assignUV (int polygonId, int vertexIndex, int uvId, const MString *uvSet=NULL)`
- `MStatus assignUVs (const MIntArray &uvCounts, const MIntArray &uvIds, const MString *uvSet=NULL)`
- `MStatus clearUVs (const MString *uvSet=NULL)`
- `MStatus getAssignedUVs (MIntArray &uvCounts, MIntArray &uvIds, const MString *uvSet=NULL) const`
- `MStatus getUvShellsIds (MIntArray &uvShellIds, unsigned int &nbUvShells, const MString *uvSet=NULL) const`
- `MStatus getPinUVs (MUintArray &pinIds, MDoubleArray &pinData, const MString *uvSet=NULL) const`
- `MStatus setPinUVs (const MUintArray &pinIds, const MDoubleArray &pinData, const MString *uvSet=NULL)`
- `bool isPolygonUVReversed (int polygonId, const MString *uvSet=NULL, MStatus *ReturnStatus=NULL) const`
- `int numColorSets (MStatus *ReturnStatus=NULL) const`
- `MString createColorSetWithName (const MString &colorSetName, MDGModifier *modifier=NULL, const MUintArray *instances=NULL, MStatus *ReturnStatus=NULL)`
- `MStatus deleteColorSet (const MString &colorSetName, MDGModifier *modifier=NULL, MSelectionList *currentSelection=NULL)`
- `MStatus setCurrentColorSetName (const MString &setName, MDGModifier *modifier=NULL, MSelectionList *currentSelection=NULL)`
- `MString currentColorSetName (int instance=kMFnMeshInstanceUnspecified, MStatus *ReturnStatus=NULL) const`
- `MStatus getColorSetNames (MStringArray &setNames) const`
- `MStatus getColorSetFamilyNames (MStringArray &familyNames) const`
- `MStatus getColorSetsInFamily (const MString &familyName, MStringArray &setNames) const`
- `bool isColorSetPerInstance (const MString &name, MStatus *ReturnStatus=NULL) const`
- `MStatus getAssociatedColorSetInstances (const MString &setName, MIntArray &instances) const`
- `MStatus setFaceColor (MColor &color, int index)`
- `MStatus setFaceColor (MColor &color, int index, MColorRepresentation rep)`
- `MStatus setVertexColor (MColor &color, int index, MDGModifier *modifier=NULL)`
- `MStatus setVertexColor (MColor &color, int index, MDGModifier *modifier, MColorRepresentation rep)`
- `MStatus setFaceVertexColor (MColor &color, int faceIndex, int vertexIndex, MDGModifier *modifier=NULL)`
- `MStatus setFaceVertexColor (MColor &color, int faceIndex, int vertexIndex, MDGModifier *modifier, MColorRepresentation rep)`
- `MStatus setFaceVertexColor (int faceIndex, int localVertexIndex, MColor &color, MDGModifier *modifier=NULL)`
- `MStatus setFaceVertexColor (int faceIndex, int localVertexIndex, MColor &color, MDGModifier *modifier, MColorRepresentation rep)`
- `MStatus setFaceColors (MColorArray &colors, MIntArray &faceList)`
- `MStatus setFaceColors (MColorArray &colors, MIntArray &faceList, MColorRepresentation rep)`
- `MStatus setVertexColors (MColorArray &colors, MIntArray &vertexList, MDGModifier *modifier=NULL)`
- `MStatus setVertexColors (MColorArray &colors, MIntArray &vertexList, MDGModifier *modifier, MColorRepresentation rep)`
- `MStatus setFaceVertexColors (MColorArray &colors, MIntArray &faceList, MIntArray &vertexList, MDGModifier *modifier=NULL)`
- `MStatus setFaceVertexColors (MColorArray &colors, MIntArray &faceList, MIntArray &vertexList, MDGModifier *modifier, MColorRepresentation rep)`
- `MStatus removeFaceColors (MIntArray &faceList)`
- `MStatus removeVertexColors (MIntArray &vertexList)`
- `MStatus removeFaceVertexColors (MIntArray &faceList, MIntArray &vertexList)`
- `MStatus getVertexColors (MColorArray &colors, const MString *colorSet=NULL, const MColor *defaultUnsetColor=NULL)`
- `MStatus getFaceVertexColors (MColorArray &colors, const MString *colorSet=NULL, const MColor *defaultUnsetColor=NULL)`
- `MStatus getFaceVertexColorIndex (int faceIndex, int localVertexIndex, int &colorIndex, const MString *colorSet=NULL)`
- `MStatus setColors (const MColorArray &colors, const MString *colorSet=NULL)`
- `MStatus setColors (const MColorArray &colors, const MString *colorSet, MColorRepresentation rep)`
- `MStatus setSomeColors (const MIntArray &colorIds, const MColorArray &colors, const MString *colorSet=NULL)`
- `MStatus setSomeColors (const MIntArray &colorIds, const MColorArray &colors, const MString *colorSet, MColorRepresentation rep)`
- `MStatus getColors (MColorArray &colors, const MString *colorSet=NULL, const MColor *defaultUnsetColor=NULL) const`
- `MStatus getColorIndex (int faceIndex, int localVertexIndex, int &colorIndex, const MString *colorSet=NULL)`
- `MStatus setColor (int colorId, const MColor &color, const MString *colorSet=NULL)`
- `MStatus setColor (int colorId, const MColor &color, const MString *colorSet, MColorRepresentation rep)`
- `MStatus getColor (int colorId, MColor &color, const MString *colorSet=NULL, const MColor *defaultUnsetColor=NULL) const`
- `MStatus assignColor (int polygonId, int vertexIndex, int colorId, const MString *colorSet=NULL)`
- `MStatus assignColors (const MIntArray &colorIds, const MString *colorSet=NULL)`
- `MStatus clearColors (const MString *colorSet=NULL)`
- `bool displayColors (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayColors (bool enable)`
- `int getHoles (MIntArray &holeInfoArray, MIntArray &holeVertexArray, MStatus *ReturnStatus=NULL)`
- `bool onBoundary (int polygonId, MStatus *ReturnStatus=NULL) const`
- `bool isBlindDataTypeUsed (int blindDataId, MStatus *ReturnStatus=NULL) const`
- `MStatus createBlindDataType (int blindDataId, MStringArray longNames, MStringArray shortNames, MStringArray formatNames)`
- `bool hasBlindDataComponentId (int compID, MFn::Type compType, int blindDataId, MStatus *ReturnStatus=NULL) const`
- `bool hasBlindData (MFn::Type compType, int blindDataId, MStatus *ReturnStatus=NULL) const`
- `bool hasBlindDataComponentId (int compID, MFn::Type compType, MStatus *ReturnStatus=NULL) const`
- `bool hasBlindData (MFn::Type compType, MStatus *ReturnStatus=NULL) const`
- `MStatus getBlindDataTypes (MFn::Type compType, MIntArray &blindDataIds) const`
- `MStatus getBlindDataAttrNames (int blindDataId, MStringArray &longNames, MStringArray &shortNames, MStringArray &formatNames) const`
- `MStatus getFaceVertexBlindDataIndex (int faceIndex, int vertexIndex, int &blindDataIndex) const`
- `MStatus getBlindDataFaceVertexIndices (int blindDataIndex, int &faceIndex, int &vertexIndex) const`
- `MStatus getIntBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, int &data) const`
- `MStatus getIntBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MIntArray &data) const`
- `MStatus getFloatBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, float &data) const`
- `MStatus getFloatBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MFloatArray &data) const`
- `MStatus getDoubleBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, double &data) const`
- `MStatus getDoubleBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MDoubleArray &data) const`
- `MStatus getBoolBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, bool &data) const`
- `MStatus getBoolBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MIntArray &data) const`
- `MString stringBlindDataComponentId (int compID, MFn::Type compType, int blindDataId, MString attrName, MStatus *ReturnStatus=NULL) const`
- `MStatus getStringBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MStringArray &data) const`
- `MString binaryBlindDataComponentId (int compID, MFn::Type compType, int blindDataId, MString attrName, MStatus *ReturnStatus=NULL) const`
- `MStatus getBinaryBlindData (MFn::Type compType, int blindDataId, MString attrName, MIntArray &compIDs, MStringArray &data) const`
- `MStatus setIntBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, int data)`
- `MStatus setIntBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, int data)`
- `MStatus setIntBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MIntArray &data)`
- `MStatus setFloatBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, float data)`
- `MStatus setFloatBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, float data)`
- `MStatus setFloatBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MFloatArray &data)`
- `MStatus setDoubleBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, double data)`
- `MStatus setDoubleBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, double data)`
- `MStatus setDoubleBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MDoubleArray &data)`
- `MStatus setBoolBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, bool data)`
- `MStatus setBoolBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, bool data)`
- `MStatus setBoolBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MIntArray &data)`
- `MStatus setStringBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, MString data)`
- `MStatus setStringBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MString data)`
- `MStatus setStringBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MStringArray &data)`
- `MStatus setBinaryBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, MString data)`
- `MStatus setBinaryBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MString data)`
- `MStatus setBinaryBlindData (MIntArray &compIDs, MFn::Type compType, int blindDataId, MString attrName, MStringArray &data)`
- `MStatus setBinaryBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, char *data, int length)`
- `MStatus clearBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName)`
- `MStatus clearBlindData (MFn::Type compType, int blindDataId, MString attrName)`
- `MStatus clearBlindData (MFn::Type compType)`
- `MStatus clearBlindData (int compID, MFn::Type compType, int blindDataId)`
- `MStatus clearBlindData (MFn::Type compType, int blindDataId)`
- `void setCheckSamePointTwice (bool check=true)`
- `bool getCheckSamePointTwice (void)`
- `MStatus createInPlace (int numVertices, int numPolygons, const MFloatPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects)`
- `MStatus copyInPlace (const MObject &source)`
- `MString createColorSetWithName (const MString &colorSetName, MDGModifier *modifier, MStatus *ReturnStatus)`
- `MObject generateSmoothMesh (MObject parentOrOwner, MStatus *ReturnStatus)`
- `MFnMesh (const MObject &object, MStatus *ret=NULL)`
- `MString currentColorSetName (MStatus *ReturnStatus) const`
- `MObject addPolygon (MPointArray &vertexArray, double polyTolerance, bool mergeVertices=true, double pointTolerance=kMFnMeshPointTolerance, bool forceGeometry=true, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `bool intersect (MPoint &raySource, MVector &rayDirection, MPointArray &points, MStatus *ReturnStatus=NULL) const`
- `bool intersect (const MPoint &raySource, const MVector &rayDirection, MPointArray &points, MStatus *ReturnStatus=NULL) const`
- `MStatus getBlindDataAttrNames (int blindDataId, MStringArray &longNames, MStringArray &shortNames) const`
- `bool hasBlindData (int compID, MFn::Type compType, int blindDataId, MStatus *ReturnStatus=NULL) const`
- `bool hasBlindData (int compID, MFn::Type compType, MStatus *ReturnStatus=NULL) const`
- `MStatus createUVSet (MString &uvSetName, MDGModifier *modifier=NULL, const MUintArray *instances=NULL)`
- `MStatus createUVSetDataMesh (MString &uvSetName)`
- `MStatus copyUVSet (const MString &fromName, MString &toName, MDGModifier *modifier=NULL)`
- `MStatus getCurrentUVSetName (MString &setName, int instance=kMFnMeshInstanceUnspecified) const`
- `MStatus createColorSet (MString &colorSetName, MDGModifier *modifier=NULL, const MUintArray *instances=NULL)`
- `MStatus createColorSet (MString &colorSetName, MDGModifier *modifier, bool clamped, MColorRepresentation rep, const MUintArray *instances=NULL)`
- `MStatus getCurrentColorSetName (MString &setName, int instance=kMFnMeshInstanceUnspecified) const`
- `MStatus getStringBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, MString &data) const`
- `MStatus getBinaryBlindData (int compID, MFn::Type compType, int blindDataId, MString attrName, MString &data) const`
- `const adsk::Data::Associations * metadata (MStatus *ReturnStatus=NULL) const override`
- `MStatus setMetadata (const adsk::Data::Associations &) override`
- `MStatus validateMetadata (MString &errors) const override`

**Static Public Member Functions**

- `static const char * className ()`
- `static MMeshIsectAccelParams uniformGridParams (int xDiv, int yDiv, int zDiv)`
- `static MMeshIsectAccelParams autoUniformGridParams ()`
- `static MString globalIntersectionAcceleratorsInfo ()`
- `static void clearGlobalIntersectionAcceleratorInfo ()`
- `static MString componentTypeName (MFn::Type compType, MStatus &status)`
- `static MFn::Type componentTypeFromName (const MString &assocName)`
- `static MStatus polyTriangulate (const float *pointsAndHoles, unsigned int pointsCount, unsigned int outerPointsCount, unsigned int holesCount, bool considerNormal, const float *normals, unsigned short *triangles, int &trianglesCount)`
- `static MStatus polyTriangulate (const MFloatPointArray &points, const MIntArray &holes, unsigned int outerPointsCount, const MFloatVectorArray &normals, MIntArray &triangles)`

### MFnMeshData

Module `OpenMaya`. Inherits `MFnGeometryData`. Source: [class_m_fn_mesh_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_mesh_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMeshData ()`
- `MFnMeshData ()`
- `MFnMeshData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnMeshData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnMessageAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_message_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_message_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnMessageAttribute ()`
- `MFnMessageAttribute ()`
- `MFnMessageAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, MStatus *ReturnStatus=NULL)`
- `MFnMessageAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNonAmbientLight

Module `OpenMaya`. Inherits `MFnLight`. Source: [class_m_fn_non_ambient_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_non_ambient_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNonAmbientLight ()`
- `MFnNonAmbientLight ()`
- `MFnNonAmbientLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnNonAmbientLight (const MDagPath &object, MStatus *ret=NULL)`
- `short decayRate (MStatus *ReturnStatus=NULL) const`
- `MStatus setDecayRate (const short &decay_rate)`
- `MFnNonAmbientLight (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNonExtendedLight

Module `OpenMaya`. Inherits `MFnNonAmbientLight`. Source: [class_m_fn_non_extended_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_non_extended_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNonExtendedLight ()`
- `MFnNonExtendedLight ()`
- `MFnNonExtendedLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnNonExtendedLight (const MDagPath &object, MStatus *ret=NULL)`
- `float shadowRadius (MStatus *ReturnStatus=NULL) const`
- `MStatus setShadowRadius (const float &shadow_radius)`
- `bool castSoftShadows (MStatus *ReturnStatus=NULL) const`
- `MStatus setCastSoftShadows (const bool &cast_soft_shadows)`
- `bool useDepthMapShadows (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseDepthMapShadows (const bool &use_depth_map)`
- `short depthMapFilterSize (MStatus *ReturnStatus) const`
- `MStatus setDepthMapFilterSize (const short &depth_map_filter_size)`
- `short depthMapResolution (MStatus *ReturnStatus) const`
- `MStatus setDepthMapResolution (const short &depth_map_resolution)`
- `float depthMapBias (MStatus *ReturnStatus) const`
- `MStatus setDepthMapBias (const float &depth_map_bias)`
- `bool useDepthMapAutoFocus (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseDepthMapAutoFocus (const bool use_depth_map_auto_focus)`
- `float depthMapWidthFocus (MStatus *ReturnStatus) const`
- `MStatus setDepthMapWidthFocus (const float depth_map_width_focus)`
- `float depthMapFocus (MStatus *ReturnStatus) const`
- `MStatus setDepthMapFocus (const float depth_map_focus)`
- `MFnNonExtendedLight (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNumericAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_numeric_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_numeric_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNumericAttribute ()`
- `MFnNumericAttribute ()`
- `MFnNumericAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, MFnNumericData::Type unitType, double defaultValue=0, MStatus *ReturnStatus=NULL)`
- `MObject createColor (const MString &fullName, const MString &briefName, MStatus *ReturnStatus=NULL)`
- `MObject createPoint (const MString &fullName, const MString &briefName, MStatus *ReturnStatus=NULL)`
- `MObject createAddr (const MString &fullName, const MString &briefName, void *defaultValue=NULL, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MObject &child1, const MObject &child2, const MObject &child3=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MObject &child1, const MObject &child2, const MObject &child3, const MObject &child4, MStatus *ReturnStatus=NULL)`
- `MFnNumericData::Type unitType (MStatus *ReturnStatus=NULL) const`
- `MObject child (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `bool hasMin (MStatus *ReturnStatus=NULL) const`
- `bool hasMax (MStatus *ReturnStatus=NULL) const`
- `bool hasSoftMin (MStatus *ReturnStatus=NULL) const`
- `bool hasSoftMax (MStatus *ReturnStatus=NULL) const`
- `MStatus getMin (double &min1) const`
- `MStatus getMin (double &min1, double &min2) const`
- `MStatus getMin (double &min1, double &min2, double &min3) const`
- `MStatus getMin (double &min1, double &min2, double &min3, double &min4) const`
- `MStatus getMax (double &max1) const`
- `MStatus getMax (double &max1, double &max2) const`
- `MStatus getMax (double &max1, double &max2, double &max3) const`
- `MStatus getMax (double &max1, double &max2, double &max3, double &max4) const`
- `MStatus getSoftMin (double &min1) const`
- `MStatus getSoftMax (double &max1) const`
- `MStatus setMin (double min1)`
- `MStatus setMin (double min1, double min2)`
- `MStatus setMin (double min1, double min2, double min3)`
- `MStatus setMin (double min1, double min2, double min3, double min4)`
- `MStatus setMax (double max1)`
- `MStatus setMax (double max1, double max2)`
- `MStatus setMax (double max1, double max2, double max3)`
- `MStatus setMax (double max1, double max2, double max3, double max4)`
- `MStatus setSoftMin (double min1)`
- `MStatus setSoftMax (double max1)`
- `MStatus getDefault (bool &def1)`
- `MStatus getDefault (char &def1)`
- `MStatus getDefault (int &def1)`
- `MStatus getDefault (int &def1, int &def2)`
- `MStatus getDefault (int &def1, int &def2, int &def3)`
- `MStatus getDefault (MInt64 &def1)`
- `MStatus getDefault (float &def1)`
- `MStatus getDefault (float &def1, float &def2)`
- `MStatus getDefault (float &def1, float &def2, float &def3)`
- `MStatus getDefault (double &def1)`
- `MStatus getDefault (double &def1, double &def2)`
- `MStatus getDefault (double &def1, double &def2, double &def3)`
- `MStatus getDefault (double &def1, double &def2, double &def3, double &def4)`
- `MStatus getDefault (void *&def1)`
- `MStatus setDefault (bool def1)`
- `MStatus setDefault (char def1)`
- `MStatus setDefault (int def1)`
- `MStatus setDefault (int def1, int def2)`
- `MStatus setDefault (int def1, int def2, int def3)`
- `MStatus setDefault (MInt64 def1)`
- `MStatus setDefault (float def1)`
- `MStatus setDefault (float def1, float def2)`
- `MStatus setDefault (float def1, float def2, float def3)`
- `MStatus setDefault (double def1)`
- `MStatus setDefault (double def1, double def2)`
- `MStatus setDefault (double def1, double def2, double def3)`
- `MStatus setDefault (double def1, double def2, double def3, double def4)`
- `MStatus setDefault (void *def1)`
- `MFnNumericAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNumericData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_numeric_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_numeric_data.html).

**Public Types**

- `enum Type { kInvalid, kBoolean, kByte, kChar, kShort, k2Short, k3Short, kLong, kInt = kLong, k2Long, k2Int = k2Long, k3Long, k3Int = k3Long, kInt64, kFloat, k2Float, k3Float, kDouble, k2Double, k3Double, k4Double, kAddr, kLast }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNumericData ()`
- `MFnNumericData ()`
- `MFnNumericData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (Type dataType, MStatus *ReturnStatus=NULL)`
- `Type numericType (MStatus *ReturnStatus=NULL)`
- `MStatus getData (short &val1, short &val2)`
- `MStatus getData (int &val1, int &val2)`
- `MStatus getData (float &val1, float &val2)`
- `MStatus getData (double &val1, double &val2)`
- `MStatus getData (short &val1, short &val2, short &val3)`
- `MStatus getData (int &val1, int &val2, int &val3)`
- `MStatus getData (float &val1, float &val2, float &val3)`
- `MStatus getData (double &val1, double &val2, double &val3)`
- `MStatus getData (double &val1, double &val2, double &val3, double &val4)`
- `MStatus setData (short val1, short val2)`
- `MStatus setData (int val1, int val2)`
- `MStatus setData (float val1, float val2)`
- `MStatus setData (double val1, double val2)`
- `MStatus setData (short val1, short val2, short val3)`
- `MStatus setData (int val1, int val2, int val3)`
- `MStatus setData (float val1, float val2, float val3)`
- `MStatus setData (double val1, double val2, double val3)`
- `MStatus setData (double val1, double val2, double val3, double val4)`
- `MFnNumericData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MStatus getData2Short (short &val1, short &val2)`
- `MStatus getData2Int (int &val1, int &val2)`
- `MStatus getData2Float (float &val1, float &val2)`
- `MStatus getData2Double (double &val1, double &val2)`
- `MStatus getData3Short (short &val1, short &val2, short &val3)`
- `MStatus getData3Int (int &val1, int &val2, int &val3)`
- `MStatus getData3Float (float &val1, float &val2, float &val3)`
- `MStatus getData3Double (double &val1, double &val2, double &val3)`
- `MStatus getData4Double (double &val1, double &val2, double &val3, double &val4)`
- `MStatus setData2Short (short val1, short val2)`
- `MStatus setData2Int (int val1, int val2)`
- `MStatus setData2Float (float val1, float val2)`
- `MStatus setData2Double (double val1, double val2)`
- `MStatus setData3Short (short val1, short val2, short val3)`
- `MStatus setData3Int (int val1, int val2, int val3)`
- `MStatus setData3Float (float val1, float val2, float val3)`
- `MStatus setData3Double (double val1, double val2, double val3)`
- `MStatus setData4Double (double val1, double val2, double val3, double val4)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNurbsCurve

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_nurbs_curve.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_curve.html).

**Public Types**

- `enum Form { kInvalid, kOpen, kClosed, kPeriodic, kLast }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNurbsCurve ()`
- `MFnNurbsCurve ()`
- `MFnNurbsCurve (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnNurbsCurve (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (const MPointArray &controlVertices, const MDoubleArray &knotSequences, unsigned int degree, Form agForm, bool create2D, bool createRational, MObject &parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject createWithEditPoints (const MPointArray &editPoints, unsigned int degree, Form agForm, bool create2D, bool createRational, bool uniformParam, MObject &parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObjectArray &sources, MObject &parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject copy (const MObject &source, MObject &parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus reverse (bool constructionHistory=false)`
- `MStatus makeMultipleEndKnots ()`
- `MObject cv (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getCV (unsigned int index, MPoint &pt, MSpace::Space space=MSpace::kObject) const`
- `MStatus setCV (unsigned int index, const MPoint &pt, MSpace::Space space=MSpace::kObject)`
- `Form form (MStatus *ReturnStatus=NULL) const`
- `int degree (MStatus *ReturnStatus=NULL) const`
- `int numCVs (MStatus *ReturnStatus=NULL) const`
- `int numSpans (MStatus *ReturnStatus=NULL) const`
- `int numKnots (MStatus *ReturnStatus=NULL) const`
- `MStatus getKnotDomain (double &start, double &end) const`
- `MStatus getKnots (MDoubleArray &array) const`
- `MStatus setKnots (const MDoubleArray &array, unsigned int startIndex, unsigned int endIndex)`
- `MStatus setKnot (unsigned int index, double param)`
- `MObject cvs (unsigned int startIndex, unsigned int endIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus getCVs (MPointArray &array, MSpace::Space space=MSpace::kObject) const`
- `MStatus setCVs (const MPointArray &array, MSpace::Space space=MSpace::kObject)`
- `double knot (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus removeKnot (double atThisParam, bool removeAll=false)`
- `bool isPointOnCurve (const MPoint &point, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus getPointAtParam (double param, MPoint &point, MSpace::Space space=MSpace::kObject) const`
- `MStatus getParamAtPoint (const MPoint &atThisPoint, double &param, MSpace::Space space=MSpace::kObject) const`
- `MStatus getParamAtPoint (const MPoint &atThisPoint, double &param, double tolerance, MSpace::Space space=MSpace::kObject) const`
- `bool isParamOnCurve (double param, MStatus *ReturnStatus=NULL) const`
- `MVector normal (double param, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MVector tangent (double param, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus getDerivativesAtParm (double param, MPoint &pos, MVector &dU, MSpace::Space space, MVector *dUU=NULL) const`
- `bool isPlanar (MVector *planeNormal=NULL, MStatus *ReturnStatus=NULL) const`
- `MPoint closestPoint (const MPoint &toThisPoint, double *param=NULL, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MPoint closestPoint (const MPoint &toThisPoint, bool paramAsGuess, double *param=NULL, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `double distanceToPoint (const MPoint &pt, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `double area (double tolerance=kMFnNurbsEpsilon, MStatus *ReturnStatus=NULL) const`
- `double length (double tolerance=kMFnNurbsEpsilon, MStatus *ReturnStatus=NULL) const`
- `double findParamFromLength (double partLength, MStatus *ReturnStatus=NULL) const`
- `double findParamFromLength (double partLength, double tolerance, MStatus *ReturnStatus=NULL) const`
- `double findLengthFromParam (double param, MStatus *ReturnStatus=NULL) const`
- `bool hasHistoryOnCreate (MStatus *ReturnStatus=NULL) const`
- `MStatus updateCurve ()`
- `MObject rebuild (unsigned int spans, unsigned int degree=3, unsigned int keepRange=1, unsigned int endKnots=0, bool keepEndPoints=true, bool keepTangents=true, bool keepControlPoints=false, float tolerance=0.01f, MStatus *ReturnStatus=NULL) const`
- `MFnNurbsCurve (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNurbsCurveData

Module `OpenMaya`. Inherits `MFnGeometryData`. Source: [class_m_fn_nurbs_curve_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_curve_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNurbsCurveData ()`
- `MFnNurbsCurveData ()`
- `MFnNurbsCurveData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnNurbsCurveData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNurbsSurface

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_nurbs_surface.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html).

**Public Types**

- `enum Form { kInvalid, kOpen, kClosed, kPeriodic, kLast }`
- `enum BoundaryType { kInvalidBoundary, kOuter, kInner, kSegment, kClosedSegment }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNurbsSurface ()`
- `MFnNurbsSurface ()`
- `MFnNurbsSurface (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnNurbsSurface (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (const MPointArray &controlVertices, const MDoubleArray &uKnotSequences, const MDoubleArray &vKnotSequences, unsigned int degreeInU, unsigned int degreeInV, Form formU, Form formV, bool createRational, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject copy (const MObject &source, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject getDataObject () const`
- `MObject cv (unsigned int indexU, unsigned int indexV, MStatus *ReturnStatus=NULL)`
- `MObject cvsInU (unsigned int startIndex, unsigned int endIndex, unsigned int rowIndex, MStatus *ReturnStatus=NULL)`
- `MObject cvsInV (unsigned int startIndex, unsigned int endIndex, unsigned int rowIndex, MStatus *ReturnStatus=NULL)`
- `MStatus getCVs (MPointArray &array, MSpace::Space space=MSpace::kObject) const`
- `MStatus setCVs (const MPointArray &array, MSpace::Space space=MSpace::kObject)`
- `MStatus getCV (unsigned int indexU, unsigned int indexV, MPoint &pnt, MSpace::Space space=MSpace::kObject) const`
- `MStatus setCV (unsigned int indexU, unsigned int indexV, MPoint &pnt, MSpace::Space space=MSpace::kObject) const`
- `Form formInU (MStatus *ReturnStatus=NULL) const`
- `Form formInV (MStatus *ReturnStatus=NULL) const`
- `bool isBezier (MStatus *ReturnStatus=NULL) const`
- `bool isUniform (MStatus *ReturnStatus=NULL) const`
- `bool isKnotU (double param, MStatus *ReturnStatus=NULL) const`
- `bool isKnotV (double param, MStatus *ReturnStatus=NULL) const`
- `bool isParamOnSurface (double paramU, double paramV, MStatus *ReturnStatus=NULL) const`
- `MStatus getKnotDomain (double &startU, double &endU, double &startV, double &endV) const`
- `int degreeU (MStatus *ReturnStatus=NULL) const`
- `int degreeV (MStatus *ReturnStatus=NULL) const`
- `int numSpansInU (MStatus *ReturnStatus=NULL) const`
- `int numSpansInV (MStatus *ReturnStatus=NULL) const`
- `int numNonZeroSpansInU (MStatus *ReturnStatus=NULL) const`
- `int numNonZeroSpansInV (MStatus *ReturnStatus=NULL) const`
- `int numCVsInU (MStatus *ReturnStatus=NULL) const`
- `int numCVsInV (MStatus *ReturnStatus=NULL) const`
- `int numKnotsInU (MStatus *ReturnStatus=NULL) const`
- `int numKnotsInV (MStatus *ReturnStatus=NULL) const`
- `MStatus getKnotsInU (MDoubleArray &array) const`
- `MStatus getKnotsInV (MDoubleArray &array) const`
- `MStatus setKnotsInU (const MDoubleArray &array, unsigned int startIndex, unsigned int endIndex)`
- `MStatus setKnotsInV (const MDoubleArray &array, unsigned int startIndex, unsigned int endIndex)`
- `double knotInU (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `double knotInV (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MStatus setKnotInU (unsigned int index, double param)`
- `MStatus setKnotInV (unsigned int index, double param)`
- `MStatus removeKnotInU (double atThisParam, bool removeAll=false)`
- `MStatus removeKnotInV (double atThisParam, bool removeAll=false)`
- `MStatus removeOneKnotInU (double atThisParam)`
- `MStatus removeOneKnotInV (double atThisParam)`
- `MVector normal (double paramInU, double paramInV, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus getTangents (double paramInU, double paramInV, MVector &vectorInU, MVector &vectorInV, MSpace::Space space=MSpace::kObject) const`
- `MStatus getDerivativesAtParm (double paramInU, double paramInV, MPoint &pos, MVector &dU, MVector &dV, MSpace::Space space, MVector *dUU=NULL, MVector *dVV=NULL, MVector *dUV=NULL) const`
- `bool isFoldedOnBispan () const`
- `double area (double tolerance=kMFnNurbsEpsilon, MStatus *ReturnStatus=NULL) const`
- `double area (MSpace::Space space, double tolerance=kMFnNurbsEpsilon, MStatus *ReturnStatus=NULL) const`
- `MPoint closestPoint (const MPoint &toThisPoint, double *paramU=NULL, double *paramV=NULL, bool ignoreTrimBoundaries=false, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MPoint closestPoint (const MPoint &toThisPoint, bool paramAsStart, double *paramU, double *paramV, bool ignoreTrimBoundaries=false, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `bool isPointOnSurface (const MPoint &point, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus getParamAtPoint (const MPoint &atThisPoint, double &paramU, double &paramV, bool ignoreTrimBoundaries, MSpace::Space space=MSpace::kObject, double tolerance=kMFnNurbsEpsilon) const`
- `MStatus getPointAtParam (double paramU, double paramV, MPoint &point, MSpace::Space space=MSpace::kObject) const`
- `double distanceToPoint (const MPoint &pt, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MObject tesselate (MTesselationParams &parms=MTesselationParams::fsDefaultTesselationParams, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `bool intersect (const MPoint &rayStartingPoint, const MVector &alongThisDirection, double &u, double &v, MPoint &intersectionData, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, bool calculateDistance=false, double *distance=NULL, bool calculateExactHit=false, bool *wasExactHit=NULL, MStatus *ReturnStatus=NULL) const`
- `bool intersect (const MPoint &rayStartingPoint, const MVector &alongThisDirection, MDoubleArray &u, MDoubleArray &v, MPointArray &intersectionData, double tolerance=kMFnNurbsEpsilon, MSpace::Space space=MSpace::kObject, bool calculateDistance=false, MDoubleArray *distances=NULL, bool calculateExactHit=false, bool *wasExactHit=NULL, MStatus *ReturnStatus=NULL) const`
- `bool hasHistoryOnCreate (MStatus *ReturnStatus=NULL) const`
- `MStatus updateSurface ()`
- `bool isTrimmedSurface (MStatus *ReturnStatus=NULL) const`
- `unsigned int numRegions (MStatus *ReturnStatus=NULL) const`
- `bool isFlipNorm (unsigned int region, MStatus *ReturnStatus=NULL) const`
- `unsigned int numBoundaries (unsigned int region, MStatus *ReturnStatus=NULL)`
- `BoundaryType boundaryType (unsigned int region, unsigned int boundary, MStatus *ReturnStatus=NULL)`
- `unsigned int numEdges (unsigned int region, unsigned int boundary, MStatus *ReturnStatus=NULL)`
- `MObjectArray edge (unsigned int region, unsigned int boundary, unsigned int edge, bool paramEdge=false, MStatus *ReturnStatus=NULL)`
- `bool isPointInTrimmedRegion (double u, double v, MStatus *ReturnStatus=NULL)`
- `MStatus getTrimBoundaries (MTrimBoundaryArray &result, unsigned int region, bool paramEdge=true)`
- `MStatus trimWithBoundaries (const MTrimBoundaryArray &mBoundaries, bool flipNormal=false, double e_tol=1e-3, double pe_tol=1e-5, bool createNewRegion=false)`
- `MStatus projectCurve (const MDagPath &curve, const MVector *direction=NULL, bool constructionHistory=false)`
- `MStatus trim (const MDoubleArray &locatorU, const MDoubleArray &locatorV, bool constructionHistory=false)`
- `unsigned int numPatches (MStatus *ReturnStatus=NULL) const`
- `unsigned int numPatchesInU (MStatus *ReturnStatus=NULL) const`
- `unsigned int numPatchesInV (MStatus *ReturnStatus=NULL) const`
- `int numUVs (MStatus *ReturnStatus=NULL) const`
- `MStatus setUVs (const MFloatArray &uArray, const MFloatArray &vArray)`
- `MStatus getUVs (MFloatArray &uArray, MFloatArray &vArray) const`
- `MStatus setUV (int uvId, float u, float v)`
- `MStatus getUV (int uvId, float &u, float &v) const`
- `MStatus getPatchUV (int patchId, int cornerIndex, float &u, float &v) const`
- `MStatus getPatchUVs (int patchId, MFloatArray &uArray, MFloatArray &vArray) const`
- `MStatus getPatchUVid (int patchId, int cornerIndex, int &uvId) const`
- `MStatus assignUV (int patchId, int cornerIndex, int uvId)`
- `MStatus assignUVs (const MIntArray &uvCounts, const MIntArray &uvIds)`
- `MStatus clearUVs ()`
- `MStatus getAssignedUVs (MIntArray &uvCounts, MIntArray &uvIds) const`
- `MStatus getConnectedShaders (unsigned int instanceNumber, MObjectArray &shaders, MIntArray &indices) const`
- `MStatus getParamAtPoint (const MPoint &atThisPoint, double &paramU, double &paramV, MSpace::Space space=MSpace::kObject) const`
- `MObject tesselate (MTesselationParams &parms, MStatus *ReturnStatus)`
- `MFnNurbsSurface (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNurbsSurfaceData

Module `OpenMaya`. Inherits `MFnGeometryData`. Source: [class_m_fn_nurbs_surface_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNurbsSurfaceData ()`
- `MFnNurbsSurfaceData ()`
- `MFnNurbsSurfaceData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnNurbsSurfaceData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPartition

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_partition.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_partition.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPartition ()`
- `MFnPartition ()`
- `MFnPartition (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool isRenderPartition=false, MStatus *ReturnStatus=NULL)`
- `bool isRenderPartition (MStatus *ReturnStatus=NULL) const`
- `MStatus addMember (const MObject &set)`
- `MStatus removeMember (const MObject &set)`
- `MFnPartition (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPhongEShader

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_phong_e_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_phong_e_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPhongEShader ()`
- `MFnPhongEShader ()`
- `MFnPhongEShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `float roughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setRoughness (const float &roughness)`
- `float highlightSize (MStatus *ReturnStatus=NULL) const`
- `MStatus setHighlightSize (const float &highlight_size)`
- `MColor whiteness (MStatus *ReturnStatus=NULL) const`
- `MStatus setWhiteness (const MColor &whiteness)`
- `MFnPhongEShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPhongShader

Module `OpenMaya`. Inherits `MFnReflectShader`. Source: [class_m_fn_phong_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_phong_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPhongShader ()`
- `MFnPhongShader ()`
- `MFnPhongShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `float cosPower (MStatus *ReturnStatus=NULL) const`
- `MStatus setCosPower (const float &cos_power)`
- `MFnPhongShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPlugin

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_plugin.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_plugin.html).

**Public Member Functions**

- `MFnPlugin ()`
- `MFnPlugin (MObject &object, const char *vendor="Unknown", const char *version="Unknown", const char *requiredApiVersion="Any", MStatus *ReturnStatus=0L)`
- `~MFnPlugin () override`
- `MFn::Type type () const override`
- `MString vendor (MStatus *ReturnStatus=nullptr) const`
- `MString version (MStatus *ReturnStatus=nullptr) const`
- `MString apiVersion (MStatus *ReturnStatus=nullptr) const`
- `MString name (MStatus *ReturnStatus=nullptr) const`
- `MString loadPath (MStatus *ReturnStatus=nullptr) const`
- `MStatus setName (const MString &newName, bool allowRename=true)`
- `MStatus setVersion (const MString &newVersion)`
- `MStatus registerCommand (const MString &commandName, MCreatorFunction creatorFunction, MCreateSyntaxFunction createSyntaxFunction=nullptr)`
- `MStatus deregisterCommand (const MString &commandName)`
- `MStatus registerControlCommand (const MString &commandName, MCreatorFunction creatorFunction)`
- `MStatus deregisterControlCommand (const MString &commandName)`
- `MStatus registerModelEditorCommand (const MString &commandName, MCreatorFunction creatorFunction, MCreatorFunction paneCreatorFunction)`
- `MStatus deregisterModelEditorCommand (const MString &commandName)`
- `MStatus registerConstraintCommand (const MString &commandName, MCreatorFunction creatorFunction)`
- `MStatus deregisterConstraintCommand (const MString &commandName)`
- `MStatus registerContextCommand (const MString &commandName, MCreatorFunction creatorFunction)`
- `MStatus registerContextCommand (const MString &commandName, MCreatorFunction creatorFunction, const MString &toolCmdName, MCreatorFunction toolCmdCreator, MCreateSyntaxFunction toolCmdSyntax=nullptr)`
- `MStatus deregisterContextCommand (const MString &commandName)`
- `MStatus deregisterContextCommand (const MString &commandName, const MString &toolCmdName)`
- `MStatus registerNode (const MString &typeName, const MTypeId &typeId, MCreatorFunction creatorFunction, MInitializeFunction initFunction, MPxNode::Type type=MPxNode::kDependNode, const MString *classification=nullptr)`
- `MStatus deregisterNode (const MTypeId &typeId)`
- `MStatus registerEvaluator (const MString &evaluatorName, unsigned int uniquePriority, MCustomEvaluatorCreatorFunction creatorFunction)`
- `MStatus deregisterEvaluator (const MString &evaluatorName)`
- `MStatus registerTopologyEvaluator (const MString &evaluatorName, unsigned int uniquePriority, MTopologyEvaluatorCreatorFunction creatorFunction)`
- `MStatus deregisterTopologyEvaluator (const MString &evaluatorName)`
- `MStatus registerShape (const MString &typeName, const MTypeId &typeId, MCreatorFunction creatorFunction, MInitializeFunction initFunction, MCreatorFunction uiCreatorFunction, const MString *classification=nullptr)`
- `MStatus registerTransform (const MString &typeName, const MTypeId &typeId, MCreatorFunction creatorFunction, MInitializeFunction initFunction, MCreateXformMatrixFunction xformCreatorFunction, const MTypeId &xformId, const MString *classification=nullptr)`
- `MStatus registerData (const MString &typeName, const MTypeId &typeId, MCreatorFunction creatorFunction, MPxData::Type type=MPxData::kData)`
- `MStatus deregisterData (const MTypeId &typeId)`
- `MStatus registerDevice (const MString &deviceName, MCreatorFunction creatorFunction)`
- `MStatus deregisterDevice (const MString &deviceName)`
- `MStatus registerFileTranslator (const MString &translatorName, const char *pixmapName, MCreatorFunction creatorFunction, const char *optionsScriptName=nullptr, const char *defaultOptionsString=nullptr, bool requiresFullMel=false, MString dataStorageLocation=MFnPlugin::kDefaultDataLocation)`
- `MStatus deregisterFileTranslator (const MString &translatorName)`
- `MStatus registerURIFileResolver (const MString &fileResolverName, const MString &uriScheme, MCreatorFunction creatorFunction)`
- `MStatus deregisterURIFileResolver (const MString &fileResolverName)`
- `MStatus registerIkSolver (const MString &ikSolverName, MCreatorFunction creatorFunction)`
- `MStatus deregisterIkSolver (const MString &ikSolverName)`
- `MStatus registerCacheFormat (const MString &cacheFormatName, MCreatorFunction creatorFunction)`
- `MStatus deregisterCacheFormat (const MString &cacheFormatName)`
- `MStatus registerUIStrings (MInitializeFunction registerMStringResources, const MString &pluginStringsProc)`
- `MStatus registerUI (PyObject *creationProc, PyObject *deletionProc, PyObject *creationBatchProc=nullptr, PyObject *deletionBatchProc=nullptr)`
- `MStatus registerDragAndDropBehavior (const MString &behaviorName, MCreatorFunction creatorFunction)`
- `MStatus deregisterDragAndDropBehavior (const MString &behaviorName)`
- `MStatus registerImageFile (const MString &imageFormatName, MCreatorFunction creatorFunction, const MStringArray &imageFileExtensions, ImageFilePriority priority=kImageFilePriorityDefault)`
- `MStatus deregisterImageFile (const MString &imageFormatName)`
- `MStatus registerRenderPassImpl (const MString &passImplId, MRenderPassDef *passDef, MCreatorFunction creatorFunction, bool overload=false)`
- `MStatus deregisterRenderPassImpl (const MString &passImplId)`
- `MStatus registerAttributePatternFactory (const MString &typeName, MCreatorFunction fnPtr)`
- `MStatus deregisterAttributePatternFactory (const MString &typeName)`
- `MStatus registerAnimCurveInterpolator (const MString &typeName, int typeId, MCreatorFunction creatorFunction, int flags=0)`
- `MStatus deregisterAnimCurveInterpolator (const MString &typeName)`
- `MStatus registerDisplayFilter (const MString &name, const MString &label, const MString &classification)`
- `MStatus deregisterDisplayFilter (const MString &name)`
- `MStatus registerRenderer (const MString &name, MCreatorFunction creatorFunction)`
- `MStatus deregisterRenderer (const MString &name)`
- `MTypeId matrixTypeIdFromXformId (const MTypeId &xformTypeId, MStatus *ReturnStatus=nullptr)`
- `MStringArray addMenuItem (const MString &menuItemName, const MString &parentName, const MString &commandName, const MString &commandParams, bool needOptionBox=false, const MString *optBoxFunction=nullptr, MStatus *retStatus=nullptr, const MString *extraMenuItemParams=nullptr)`
- `MStringArray addMenuItem (const MString &menuItemName, const MString &menuItemLabel, const MString &parentName, const MString &commandName, const MString &commandParams, bool needOptionBox=false, const MString *optBoxFunction=nullptr, MStatus *retStatus=nullptr, const MString *extraMenuItemParams=nullptr)`
- `MStatus removeMenuItem (MStringArray &menuItemNames)`
- `MStatus registerMaterialInfo (const MString &type, MMaterialInfoFactoryFnPtr fnPtr)`
- `MStatus unregisterMaterialInfo (const MString &typeName)`
- `MStatus registerBakeEngine (const MString &typeName, MBakeEngineCreatorFnPtr fnPtr)`
- `MStatus unregisterBakeEngine (const MString &typeName)`
- `void setCallableInfo (MPluginCallableInfo *info)`
- `MPluginCallableInfo * getCallableInfo ()`
- `MStatus registerTransform (const MString &typeName, const MTypeId &typeId, MCreatorFunction creatorFunction, MInitializeFunction initFunction, MCreatorFunction xformCreatorFunction, const MTypeId &xformId, const MString *classification=nullptr)`
- `MStatus registerUI (const MString &creationProc, const MString &deletionProc, const MString &creationBatchProc="", const MString &deletionBatchProc="")`
- `MStatus registerEvaluator (const MString &evaluatorName, unsigned int uniquePriority, MCreatorFunction creatorFunction)`
- `MStatus registerTopologyEvaluator (const MString &evaluatorName, unsigned int uniquePriority, MCreatorFunction creatorFunction)`

**Static Public Member Functions**

- `static MObject findPlugin (const MString &pluginName)`
- `static bool isNodeRegistered (const MString &typeName)`
- `static void setRegisteringCallableScript ()`
- `static bool registeringCallableScript ()`
- `static const char * className ()`

**Static Public Attributes**

- `static const MString kDefaultDataLocation = "data"`

### MFnPluginData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_plugin_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_plugin_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPluginData ()`
- `MFnPluginData ()`
- `MFnPluginData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MTypeId typeId (MStatus *ReturnStatus=NULL) const`
- `MPxData * data (MStatus *ReturnStatus=NULL)`
- `const MPxData * constData (MStatus *ReturnStatus=NULL) const`
- `MObject create (const MTypeId &id, MStatus *ReturnStatus=NULL)`
- `MFnPluginData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPointArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_point_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPointArrayData ()`
- `MFnPointArrayData ()`
- `MFnPointArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (MPoint &element, unsigned int index)`
- `MStatus copyTo (MPointArray &) const`
- `MStatus set (const MPointArray &newArray)`
- `MPointArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MPointArray &in, MStatus *ReturnStatus=NULL)`
- `MFnPointArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MPoint & operator[] (unsigned int index)`
- `const MPoint & operator[] (unsigned int index) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPointLight

Module `OpenMaya`. Inherits `MFnNonExtendedLight`. Source: [class_m_fn_point_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_light.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPointLight ()`
- `MFnPointLight ()`
- `MFnPointLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnPointLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `MFnPointLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnReference

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_reference.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_reference.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnReference ()`
- `MFnReference ()`
- `MFnReference (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool isValidReference (MStatus *ReturnStatus=NULL) const`
- `MString fileName (bool resolvedName, bool includePath, bool includeCopyNumber, MStatus *ReturnStatus=NULL) const`
- `MString associatedNamespace (bool shortName, MStatus *ReturnStatus=NULL) const`
- `MString parentFileName (bool resolvedName, bool includePath, bool includeCopyNumber, MStatus *ReturnStatus=NULL) const`
- `MObject parentReference (MStatus *ReturnStatus=NULL) const`
- `MObject parentAssembly (MStatus *ReturnStatus=NULL) const`
- `bool containsNode (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `bool containsNodeExactly (const MObject &node, MStatus *ReturnStatus=NULL) const`
- `void nodes (MObjectArray &nodes, MStatus *ReturnStatus=NULL) const`
- `bool isLoaded (MStatus *ReturnStatus=NULL) const`
- `bool isLocked (MStatus *ReturnStatus=NULL) const`
- `bool isExportEditsFile (MStatus *ReturnStatus=NULL) const`
- `MFnReference (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`
- `static bool ignoreReferenceEdits ()`
- `static void setIgnoreReferenceEdits (bool ignoreEdits)`

### MFnReflectShader

Module `OpenMaya`. Inherits `MFnLambertShader`. Source: [class_m_fn_reflect_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_reflect_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnReflectShader ()`
- `MFnReflectShader ()`
- `MFnReflectShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `short reflectedRayDepthLimit (MStatus *ReturnStatus=NULL) const`
- `MStatus setReflectedRayDepthLimit (const short &new_limit)`
- `MColor specularColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularColor (const MColor &specular_color)`
- `float reflectivity (MStatus *ReturnStatus=NULL) const`
- `MStatus setReflectivity (const float &reflectivity)`
- `MColor reflectedColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setReflectedColor (const MColor &reflected_color)`
- `MFnReflectShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSet

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_set.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_set.html).

**Public Types**

- `enum Restriction { kNone, kVerticesOnly, kEdgesOnly, kFacetsOnly, kEditPointsOnly, kRenderableOnly }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSet ()`
- `MFnSet ()`
- `MFnSet (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MSelectionList &members, Restriction restriction=kNone, MStatus *ReturnStatus=NULL)`
- `MStatus getUnion (const MObject &withSet, MSelectionList &result)`
- `MStatus getUnion (const MObjectArray &setList, MSelectionList &result)`
- `MStatus getIntersection (const MObject &withSet, MSelectionList &result)`
- `MStatus getIntersection (const MObjectArray &setList, MSelectionList &result)`
- `MStatus clear ()`
- `MStatus getMembers (MSelectionList &members, bool flatten) const`
- `MStatus addMember (const MObject &obj)`
- `MStatus addMember (const MDagPath &obj, const MObject &component=MObject::kNullObj)`
- `MStatus addMember (const MPlug &plug)`
- `MStatus addMembers (const MSelectionList &list)`
- `MStatus removeMember (const MObject &obj)`
- `MStatus removeMember (const MDagPath &obj, const MObject &component)`
- `MStatus removeMember (const MPlug &plug)`
- `MStatus removeMembers (const MSelectionList &list)`
- `bool isMember (const MObject &object, MStatus *ReturnStatus=NULL) const`
- `bool isMember (const MDagPath &object, const MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL) const`
- `bool isMember (const MPlug &plug, MStatus *ReturnStatus=NULL) const`
- `bool intersectsWith (const MObject &otherSet, MStatus *ReturnStatus=NULL) const`
- `bool hasRestrictions (MStatus *ReturnStatus=NULL) const`
- `Restriction restriction (MStatus *ReturnStatus=NULL) const`
- `MString annotation (MStatus *ReturnStatus=NULL) const`
- `MStatus setAnnotation (const MString &annotation)`
- `MFnSet (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSingleIndexedComponent

Module `OpenMaya`. Inherits `MFnComponent`. Source: [class_m_fn_single_indexed_component.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_single_indexed_component.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSingleIndexedComponent ()`
- `MFnSingleIndexedComponent ()`
- `MFnSingleIndexedComponent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MFn::Type compType, MStatus *ReturnStatus=NULL)`
- `MStatus addElement (int element)`
- `MStatus addElements (MIntArray &elements)`
- `int elementMax (MStatus *ReturnStatus=NULL) const`
- `int element (int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getElements (MIntArray &elements) const`
- `MStatus setCompleteData (int numElements)`
- `MStatus getCompleteData (int &numElements) const`
- `MFnSingleIndexedComponent (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSphereData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_sphere_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_sphere_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSphereData ()`
- `MFnSphereData ()`
- `MFnSphereData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (double rad=1, MStatus *ReturnStatus=NULL)`
- `double radius (MStatus *ReturnStatus=NULL) const`
- `MStatus setRadius (double rad)`
- `MFnSphereData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSpotLight

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_spot_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_spot_light.html).

**Public Types**

- `enum MBarnDoor { kLeft, kRight, kTop, kBottom }`
- `enum MDecayRegion { kFirst, kSecond, kThird }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSpotLight ()`
- `MFnSpotLight ()`
- `MFnSpotLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnSpotLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `double coneAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setConeAngle (const double &coneAngle)`
- `double penumbraAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setPenumbraAngle (const double &penumbraAngle)`
- `double dropOff (MStatus *ReturnStatus=NULL) const`
- `MStatus setDropOff (const double &dropOff)`
- `bool barnDoors (MStatus *ReturnStatus=NULL) const`
- `MStatus setBarnDoors (const bool &barnDoors)`
- `double barnDoorAngle (MBarnDoor which_door, MStatus *ReturnStatus=NULL) const`
- `MStatus setBarnDoorAngle (MBarnDoor which_door, const double &barn_door_angle)`
- `bool useDecayRegions (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseDecayRegions (const bool &use_decay_regions)`
- `float startDistance (MDecayRegion which_region, MStatus *ReturnStatus=NULL) const`
- `MStatus setStartDistance (MDecayRegion which_region, const float &start_distance)`
- `float endDistance (MDecayRegion which_region, MStatus *ReturnStatus=NULL) const`
- `MStatus setEndDistance (MDecayRegion which_region, const float &end_distance)`
- `MFnSpotLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnStandardSurfaceShader

Module `OpenMaya`. Inherits `MFnDependencyNode`. Source: [class_m_fn_standard_surface_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_standard_surface_shader.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnStandardSurfaceShader ()`
- `MFnStandardSurfaceShader ()`
- `MFnStandardSurfaceShader (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `float base (MStatus *ReturnStatus=NULL) const`
- `MStatus setBase (const float &base)`
- `MColor baseColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setBaseColor (const MColor &base_color)`
- `float diffuseRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setDiffuseRoughness (const float &diffuse_roughness)`
- `float metalness (MStatus *ReturnStatus=NULL) const`
- `MStatus setMetalness (const float &metalness)`
- `float specular (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecular (const float &specular)`
- `MColor specularColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularColor (const MColor &specular_color)`
- `float specularRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularRoughness (const float &specular_roughness)`
- `float specularIOR (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularIOR (const float &specular_i_o_r)`
- `float specularAnisotropy (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularAnisotropy (const float &specular_anisotropy)`
- `float specularRotation (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpecularRotation (const float &specular_rotation)`
- `float transmission (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmission (const float &transmission)`
- `MColor transmissionColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionColor (const MColor &transmission_color)`
- `float transmissionDepth (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionDepth (const float &transmission_depth)`
- `MColor transmissionScatter (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionScatter (const MColor &transmission_scatter)`
- `float transmissionScatterAnisotropy (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionScatterAnisotropy (const float &transmission_scatter_anisotropy)`
- `float transmissionDispersion (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionDispersion (const float &transmission_dispersion)`
- `float transmissionExtraRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setTransmissionExtraRoughness (const float &transmission_extra_roughness)`
- `float subsurface (MStatus *ReturnStatus=NULL) const`
- `MStatus setSubsurface (const float &subsurface)`
- `MColor subsurfaceColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setSubsurfaceColor (const MColor &subsurface_color)`
- `MColor subsurfaceRadius (MStatus *ReturnStatus=NULL) const`
- `MStatus setSubsurfaceRadius (const MColor &subsurface_radius)`
- `float subsurfaceScale (MStatus *ReturnStatus=NULL) const`
- `MStatus setSubsurfaceScale (const float &subsurface_scale)`
- `float subsurfaceAnisotropy (MStatus *ReturnStatus=NULL) const`
- `MStatus setSubsurfaceAnisotropy (const float &subsurface_anisotropy)`
- `float coat (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoat (const float &coat)`
- `MColor coatColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatColor (const MColor &coat_color)`
- `float coatRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatRoughness (const float &coat_roughness)`
- `float coatIOR (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatIOR (const float &coat_i_o_r)`
- `float coatAnisotropy (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatAnisotropy (const float &coat_anisotropy)`
- `float coatRotation (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatRotation (const float &coat_rotation)`
- `float coatAffectColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatAffectColor (const float &coat_affect_color)`
- `float coatAffectRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setCoatAffectRoughness (const float &coat_affect_roughness)`
- `float sheen (MStatus *ReturnStatus=NULL) const`
- `MStatus setSheen (const float &sheen)`
- `MColor sheenColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setSheenColor (const MColor &sheen_color)`
- `float sheenRoughness (MStatus *ReturnStatus=NULL) const`
- `MStatus setSheenRoughness (const float &sheen_roughness)`
- `float emission (MStatus *ReturnStatus=NULL) const`
- `MStatus setEmission (const float &emission)`
- `MColor emissionColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setEmissionColor (const MColor &emission_color)`
- `float thinFilmThickness (MStatus *ReturnStatus=NULL) const`
- `MStatus setThinFilmThickness (const float &thin_film_thickness)`
- `float thinFilmIOR (MStatus *ReturnStatus=NULL) const`
- `MStatus setThinFilmIOR (const float &thin_film_i_o_r)`
- `bool thinWalled (MStatus *ReturnStatus=NULL) const`
- `MStatus setThinWalled (const bool &thin_walled)`
- `MColor opacity (MStatus *ReturnStatus=NULL) const`
- `MStatus setOpacity (const MColor &opacity)`
- `MFnStandardSurfaceShader (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnStringArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_string_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_string_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnStringArrayData ()`
- `MFnStringArrayData ()`
- `MFnStringArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (const MString &element, unsigned int index)`
- `MStatus copyTo (MStringArray &) const`
- `MStatus set (const MStringArray &newArray)`
- `MStringArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MStringArray &in, MStatus *ReturnStatus=NULL)`
- `const MString & operator[] (unsigned int index) const`
- `MFnStringArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MString & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnStringData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_string_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_string_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnStringData ()`
- `MFnStringData ()`
- `MFnStringData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MString string (MStatus *ReturnStatus=NULL) const`
- `MStatus set (const MString &newString)`
- `MObject create (const MString &str, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnStringData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSubd

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_subd.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_subd.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSubd ()`
- `MFnSubd ()`
- `MFnSubd (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnSubd (const MDagPath &object, MStatus *ret=NULL)`
- `MObject createBaseMesh (bool reverseNormal, int numVertices, int numPolygons, const MPointArray &vertexArray, const MIntArray &polygonCounts, const MIntArray &polygonConnects, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject copy (const MObject &source, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject collapse (const int level, bool makeNewNodeInvisible, MStatus *ReturnStatus=NULL)`
- `int vertexBaseMeshAdd (double x, double y, double z, MStatus *ReturnStatus=NULL)`
- `MUint64 polygonBaseMeshAdd (int vertexCount, const int vertexIds[], MStatus *ReturnStatus=NULL)`
- `MUint64 polygonBaseMeshAddWithUVs (int vertexCount, const int vertexIds[], const double uValues[], const double vValues[], MStatus *ReturnStatus=NULL)`
- `int editsPending (MStatus *ReturnStatus=NULL) const`
- `void editsUpdateAll (MStatus *ReturnStatus=NULL)`
- `unsigned int levelMaxCurrent (MStatus *ReturnStatus=NULL) const`
- `unsigned int levelMaxAllowed (MStatus *ReturnStatus=NULL) const`
- `void levelFullySubdivideTo (int level, MStatus *ReturnStatus=NULL)`
- `unsigned int vertexCount (int level=0, MStatus *ReturnStatus=NULL) const`
- `MUint64 vertexIdFromBaseVertexIndex (int baseVertexIndex, MStatus *ReturnStatus=NULL) const`
- `int vertexBaseIndexFromVertexId (MUint64 vertexId, MStatus *ReturnStatus=NULL) const`
- `MStatus vertexEditsGetAllNonBase (MUint64Array &vertexIds, MVectorArray &edits, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexEditsSetAllNonBase (const MUint64Array &vertexIds, const MVectorArray &edits, MSpace::Space space=MSpace::kObject)`
- `MStatus vertexEditsClearAllNonBase ()`
- `MStatus vertexPositionGet (MUint64 vertId, MPoint &position, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexEditGet (MUint64 vertId, MVector &edit, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexPositionGetNoEdit (MUint64 vertId, MPoint &position, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexPositionSet (MUint64 vertId, const MPoint &position, bool updateEdits, MSpace::Space space=MSpace::kObject)`
- `MStatus vertexEditSet (MUint64 vertId, const MVector &edit, bool updateEdits, MSpace::Space space=MSpace::kObject)`
- `MStatus vertexBaseMeshGet (MPointArray &positions, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexBaseMeshSet (const MPointArray &positions, MSpace::Space space=MSpace::kObject)`
- `MStatus vertexBaseMeshGetWithId (MPointArray &positions, MUint64Array &vertexIds, MSpace::Space space=MSpace::kObject) const`
- `MStatus vertexBaseMeshSetWithId (const MPointArray &positions, const MUint64Array &vertexIds, MSpace::Space space=MSpace::kObject)`
- `int vertexValence (MUint64 vertId, MStatus *ReturnStatus=NULL) const`
- `MStatus vertexNormal (MUint64 vertId, MVector &normal) const`
- `MStatus vertexAdjacentVertices (MUint64 vertId, MUint64Array &vertList) const`
- `MStatus vertexIncidentEdges (MUint64 vertId, MUint64Array &edgeList) const`
- `MStatus vertexIncidentPolygons (MUint64 vertId, MUint64Array &polyList) const`
- `bool vertexIsBoundary (MUint64 vertId, MStatus *ReturnStatus=NULL) const`
- `bool vertexIsValid (MUint64 vertId, MStatus *ReturnStatus=NULL) const`
- `bool vertexIsCreased (MUint64 vertId, MStatus *ReturnStatus=NULL) const`
- `bool vertexCreaseRelevant (MUint64 vertId, MStatus *ReturnStatus=NULL) const`
- `MStatus vertexSetCrease (MUint64 vertId, bool creased) const`
- `MStatus vertexChildren (MUint64 vertId, MUint64Array &children) const`
- `MStatus creasesGetAll (MUint64Array &vertexIds, MUint64Array &edgeIds) const`
- `MStatus creasesSetAll (const MUint64Array &vertexIds, const MUint64Array &edgeIds)`
- `MStatus creasesClearAll () const`
- `MStatus updateAllEditsAndCreases ()`
- `unsigned int edgeCount (int level=0, MStatus *ReturnStatus=NULL) const`
- `MUint64 edgeBetween (MUint64 vertex1, MUint64 vertex2, MStatus *ReturnStatus=NULL) const`
- `MStatus edgeVertices (MUint64 edge, MUint64 &v1, MUint64 &v2) const`
- `MStatus edgeAdjacentPolygon (MUint64 edge, MUint64Array &polys) const`
- `bool edgeIsBoundary (MUint64 edge, MStatus *ReturnStatus=NULL) const`
- `bool edgeIsValid (MUint64 edgeId, MStatus *ReturnStatus=NULL) const`
- `bool edgeIsCreased (MUint64 edgeId, MStatus *ReturnStatus=NULL) const`
- `bool edgeCreaseRelevant (MUint64 edgeId, MStatus *ReturnStatus=NULL) const`
- `MStatus edgeSetCrease (MUint64 edgeId, bool creased) const`
- `MStatus edgeChildren (MUint64 edgeId, MUint64Array &children) const`
- `unsigned int polygonCount (int level=0, MStatus *ReturnStatus=NULL) const`
- `int polygonCountMaxWithGivenBaseMesh (int level=1, MStatus *ReturnStatus=NULL) const`
- `unsigned int polygonVertexCount (MUint64 polyId, MStatus *ReturnStatus=NULL) const`
- `MStatus polygonVertices (MUint64 polyId, MUint64Array &vertIds) const`
- `unsigned int polygonEdgeCount (MUint64 polyId, MStatus *ReturnStatus=NULL) const`
- `MStatus polygonEdges (MUint64 polyId, MUint64Array &edgeIds) const`
- `bool polygonIsValid (MUint64 polyId, MStatus *ReturnStatus=NULL) const`
- `bool polygonHasChildren (MUint64 polyId, MStatus *ReturnStatus=NULL) const`
- `MStatus polygonChildren (MUint64 polyId, MUint64Array &children) const`
- `MStatus polygonSubdivide (MUint64 polyId)`
- `MStatus polygonSetUseUVs (MUint64 polyId, bool useThem)`
- `bool polygonHasVertexUVs (MUint64 polyId, MStatus *ReturnStatus=NULL)`
- `MStatus polygonGetVertexUVs (MUint64 polyId, MDoubleArray &uValues, MDoubleArray &vValues) const`
- `MStatus polygonSetVertexUVs (MUint64 polyId, const MDoubleArray &uValues, const MDoubleArray &vValues)`
- `MStatus polygonGetCenterUV (MUint64 polyId, double &u, double &v) const`
- `MStatus evaluatePosition (MUint64 polyId, double u, double v, bool uvNormalized, MPoint &) const`
- `MStatus evaluateNormal (MUint64 polyId, double u, double v, bool uvNormalized, MVector &nrml) const`
- `MStatus evaluatePositionAndNormal (MUint64 polyId, double u, double v, bool uvNormalized, MPoint &pos, MVector &nrml) const`
- `bool getCubicSpline (MUint64 polyId, MPointArray vertices, MStatus *ReturnStatus=NULL)`
- `MStatus getConnectedShaders (unsigned int instanceNumber, MObjectArray &shaders, MUint64Array &faces, MIntArray &indices) const`
- `MObject tesselate (bool uniform, int depth, int sample, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject tessellateViaNurbs (MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus convertToNurbs (MObjectArray &newNurbsSurfaces)`
- `MStatus updateSubdSurface ()`
- `MStatus vertexBaseMeshAddWithIndex (double x, double y, double z, int index)`
- `MFnSubd (const MObject &object, MStatus *ret=NULL)`
- `bool getCubicSpline (MUint64 polyId, MPoint vertices[16], MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSubdData

Module `OpenMaya`. Inherits `MFnGeometryData`. Source: [class_m_fn_subd_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_subd_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnSubdData ()`
- `MFnSubdData ()`
- `MFnSubdData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnSubdData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnSubdNames

Module `OpenMaya`. Source: [class_m_fn_subd_names.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_subd_names.html).

**Public Member Functions**

- `MFnSubdNames ()`
- `~MFnSubdNames ()`

**Static Public Member Functions**

- `static int base (MUint64 id)`
- `static int first (MUint64 id)`
- `static int level (MUint64 id)`
- `static int path (MUint64 id)`
- `static int corner (MUint64 id)`
- `static MStatus fromMUint64 (MUint64 id, int &base, int &first, int &level, int &path, int &corner)`
- `static MStatus toMUint64 (MUint64 &id, int base, int first, int level, int path, int corner)`
- `static MUint64 baseFaceId (MUint64 id)`
- `static long baseFaceIndex (MUint64 id)`
- `static unsigned int baseFaceIndexFromId (MUint64 id)`
- `static MUint64 levelOneFaceId (MUint64 id)`
- `static long levelOneFaceAsLong (MUint64 id)`
- `static unsigned int levelOneFaceIndexFromId (MUint64 id)`
- `static MUint64 levelOneFaceIdFromLong (long one)`
- `static MUint64 levelOneFaceIdFromIndex (unsigned int index)`
- `static MUint64 baseFaceIdFromLong (long base)`
- `static MUint64 baseFaceIdFromIndex (unsigned int index)`
- `static MUint64 parentFaceId (MUint64 id)`
- `static MStatus nonBaseFaceVertices (MUint64 id, MUint64 &vertex1, MUint64 &vertex2, MUint64 &vertex3, MUint64 &vertex4)`
- `static MStatus nonBaseFaceEdges (MUint64 id, MUint64 &edge1, MUint64 &edge2, MUint64 &edge3, MUint64 &edge4)`
- `static MStatus fromSelectionIndices (MUint64 &id, unsigned int firstIndex, unsigned int secondIndex)`
- `static MStatus toSelectionIndices (MUint64 id, unsigned int &firstIndex, unsigned int &secondIndex)`
- `static const char * className ()`

### MFnToolContext

Module `OpenMaya`. Inherits `MFnBase`. Source: [class_m_fn_tool_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_tool_context.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnToolContext ()`
- `MFnToolContext ()`
- `MFnToolContext (MObject &object, MStatus *ReturnStatus=NULL)`
- `MString name (MStatus *=NULL) const`
- `MString title (MStatus *=NULL) const`
- `MFnToolContext (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnTransform

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_transform.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_transform.html).

**Public Types**

- `enum LimitType { kScaleMinX, kScaleMaxX, kScaleMinY, kScaleMaxY, kScaleMinZ, kScaleMaxZ, kShearMinXY, kShearMaxXY, kShearMinXZ, kShearMaxXZ, kShearMinYZ, kShearMaxYZ, kRotateMinX, kRotateMaxX, kRotateMinY, kRotateMaxY, kRotateMinZ, kRotateMaxZ, kTranslateMinX, kTranslateMaxX, kTranslateMinY, kTranslateMaxY, kTranslateMinZ, kTranslateMaxZ }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnTransform ()`
- `MFnTransform ()`
- `MFnTransform (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnTransform (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MObject parent=MObject::kNullObj, MStatus *ReturnStatus=nullptr)`
- `MTransformationMatrix transformation (MStatus *ReturnStatus=nullptr) const`
- `MStatus set (const MTransformationMatrix &transform)`
- `MVector getTranslation (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setTranslation (const MVector &vec, MSpace::Space space)`
- `MStatus translateBy (const MVector &vec, MSpace::Space space)`
- `MStatus getScale (double scale[3]) const`
- `MStatus setScale (const double scale[3])`
- `MStatus scaleBy (const double scale[3])`
- `MPoint scalePivot (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setScalePivot (const MPoint &point, MSpace::Space space, bool balance)`
- `MVector scalePivotTranslation (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setScalePivotTranslation (const MVector &vec, MSpace::Space space)`
- `MStatus getShear (double scale[3]) const`
- `MStatus setShear (const double shear[3])`
- `MStatus shearBy (const double shear[3])`
- `MStatus getRotation (MQuaternion &quaternion, MSpace::Space=MSpace::kTransform) const`
- `MStatus setRotation (const MQuaternion &quaternion, MSpace::Space=MSpace::kTransform)`
- `MStatus rotateBy (const MQuaternion &quaternion, MSpace::Space=MSpace::kTransform)`
- `MStatus getRotation (MEulerRotation &rotation) const`
- `MStatus setRotation (const MEulerRotation &rotation)`
- `MStatus rotateBy (const MEulerRotation &rotation, MSpace::Space=MSpace::kTransform)`
- `MPoint rotatePivot (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setRotatePivot (const MPoint &point, MSpace::Space space, bool balance)`
- `MVector rotatePivotTranslation (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setRotatePivotTranslation (const MVector &vec, MSpace::Space space)`
- `MStatus getRotationQuaternion (double &x, double &y, double &z, double &w, MSpace::Space=MSpace::kTransform) const`
- `MStatus setRotationQuaternion (double x, double y, double z, double w, MSpace::Space=MSpace::kTransform)`
- `MStatus rotateByQuaternion (double x, double y, double z, double w, MSpace::Space=MSpace::kTransform)`
- `MStatus getRotation (double rotation[3], MTransformationMatrix::RotationOrder &order) const`
- `MStatus setRotation (const double rotation[3], MTransformationMatrix::RotationOrder order)`
- `MStatus rotateBy (const double rotation[3], MTransformationMatrix::RotationOrder order, MSpace::Space=MSpace::kTransform)`
- `MQuaternion rotateOrientation (MSpace::Space space, MStatus *ReturnStatus=nullptr) const`
- `MStatus setRotateOrientation (const MQuaternion &quat, MSpace::Space space, bool balance)`
- `MTransformationMatrix::RotationOrder rotationOrder (MStatus *ReturnStatus=nullptr) const`
- `MStatus setRotationOrder (MTransformationMatrix::RotationOrder order, bool reorder)`
- `MTransformationMatrix restPosition (MStatus *ReturnStatus) const`
- `MStatus setRestPosition (const MTransformationMatrix &matrix)`
- `MStatus resetFromRestPosition ()`
- `MStatus clearRestPosition ()`
- `bool isLimited (MFnTransform::LimitType type, MStatus *ReturnStatus=nullptr) const`
- `double limitValue (MFnTransform::LimitType type, MStatus *ReturnStatus=nullptr) const`
- `MStatus setLimit (MFnTransform::LimitType type, double value)`
- `MStatus enableLimit (MFnTransform::LimitType type, bool flag)`
- `MFnTransform (const MObject &object, MStatus *ret=NULL)`
- `MStatus getRotation (double[3], MTransformationMatrix::RotationOrder &order, MSpace::Space space) const`
- `MStatus setRotation (const double[3], MTransformationMatrix::RotationOrder, MSpace::Space space)`
- `MVector translation (MSpace::Space, MStatus *=nullptr) const`

**Static Public Member Functions**

- `static const char * className ()`

### MFnTripleIndexedComponent

Module `OpenMaya`. Inherits `MFnComponent`. Source: [class_m_fn_triple_indexed_component.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_triple_indexed_component.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnTripleIndexedComponent ()`
- `MFnTripleIndexedComponent ()`
- `MFnTripleIndexedComponent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MFn::Type compType, MStatus *ReturnStatus=NULL)`
- `MStatus addElement (int sIndex, int tIndex, int uIndex)`
- `MStatus addElements (const MIntArray &sIndexArray, const MIntArray &tIndexArray, const MIntArray &uIndexArray)`
- `MStatus getElement (int index, int &sIndex, int &tIndex, int &uIndex) const`
- `MStatus getElements (MIntArray &sIndexArray, MIntArray &tIndexArray, MIntArray &uIndexArray) const`
- `MStatus setCompleteData (int maxS, int maxT, int maxU)`
- `MStatus getCompleteData (int &maxS, int &maxT, int &maxU) const`
- `MFnTripleIndexedComponent (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnTypedAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_typed_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_typed_attribute.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnTypedAttribute ()`
- `MFnTypedAttribute ()`
- `MFnTypedAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MTypeId &id, MObject defaultData=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, MFnData::Type type, MObject defaultData=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MFnData::Type attrType (MStatus *ReturnStatus=NULL) const`
- `MStatus getDefault (MObject &defaultCustomData)`
- `MStatus setDefault (const MObject &defaultCustomData)`
- `MObject create (const MString &fullName, const MString &briefName, MFnData::Type type, MStatus *ReturnStatus)`
- `MFnTypedAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnUInt64ArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_u_int64_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_u_int64_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnUInt64ArrayData ()`
- `MFnUInt64ArrayData ()`
- `MFnUInt64ArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (MUint64 element, unsigned int index)`
- `MStatus copyTo (MUint64Array &) const`
- `MStatus set (const MUint64Array &newArray)`
- `MUint64Array array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MUint64Array &in, MStatus *ReturnStatus=NULL)`
- `MFnUInt64ArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `MUint64 operator[] (unsigned int index) const`
- `MUint64 & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnUint64SingleIndexedComponent

Module `OpenMaya`. Inherits `MFnComponent`. Source: [class_m_fn_uint64_single_indexed_component.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_uint64_single_indexed_component.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnUint64SingleIndexedComponent ()`
- `MFnUint64SingleIndexedComponent ()`
- `MFnUint64SingleIndexedComponent (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (MFn::Type compType, MStatus *ReturnStatus=NULL)`
- `MStatus addElement (MUint64 element)`
- `MStatus addElements (const MUint64Array &elements)`
- `MUint64 element (int index, MStatus *ReturnStatus=NULL) const`
- `MStatus getElements (MUint64Array &elements) const`
- `MStatus setCompleteData (MUint64 numElements)`
- `MStatus getCompleteData (MUint64 &numElements) const`
- `MFnUint64SingleIndexedComponent (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnUintArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_uint_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_uint_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnUintArrayData ()`
- `MFnUintArrayData ()`
- `MFnUintArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `unsigned int operator[] (unsigned int index) const`
- `MStatus set (unsigned int element, unsigned int index)`
- `MStatus copyTo (MUintArray &) const`
- `MStatus set (const MUintArray &newArray)`
- `MUintArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MUintArray &in, MStatus *ReturnStatus=NULL)`
- `MFnUintArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnUnitAttribute

Module `OpenMaya`. Inherits `MFnAttribute`. Source: [class_m_fn_unit_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_unit_attribute.html).

**Public Types**

- `enum Type { kInvalid, kAngle, kDistance, kTime, kLast }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnUnitAttribute ()`
- `MFnUnitAttribute ()`
- `MFnUnitAttribute (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, MFnUnitAttribute::Type unitType, double defaultValue=0.0, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MTime &defaultValue, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MAngle &defaultValue, MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &fullName, const MString &briefName, const MDistance &defaultValue, MStatus *ReturnStatus=NULL)`
- `MFnUnitAttribute::Type unitType (MStatus *ReturnStatus=NULL) const`
- `bool hasMin (MStatus *ReturnStatus=NULL) const`
- `bool hasMax (MStatus *ReturnStatus=NULL) const`
- `bool hasSoftMin (MStatus *ReturnStatus=NULL) const`
- `bool hasSoftMax (MStatus *ReturnStatus=NULL) const`
- `MStatus getMin (double &minValue) const`
- `MStatus getMin (MTime &minValue) const`
- `MStatus getMin (MAngle &minValue) const`
- `MStatus getMin (MDistance &minValue) const`
- `MStatus getMax (double &maxValue) const`
- `MStatus getMax (MTime &maxValue) const`
- `MStatus getMax (MAngle &maxValue) const`
- `MStatus getMax (MDistance &maxValue) const`
- `MStatus getSoftMin (double &minValue) const`
- `MStatus getSoftMin (MTime &minValue) const`
- `MStatus getSoftMin (MAngle &minValue) const`
- `MStatus getSoftMin (MDistance &minValue) const`
- `MStatus getSoftMax (double &maxValue) const`
- `MStatus getSoftMax (MTime &maxValue) const`
- `MStatus getSoftMax (MAngle &maxValue) const`
- `MStatus getSoftMax (MDistance &maxValue) const`
- `MStatus setMin (double minValue)`
- `MStatus setMin (const MTime &minValue)`
- `MStatus setMin (const MAngle &minValue)`
- `MStatus setMin (const MDistance &minValue)`
- `MStatus setMax (double maxValue)`
- `MStatus setMax (const MTime &maxValue)`
- `MStatus setMax (const MAngle &maxValue)`
- `MStatus setMax (const MDistance &maxValue)`
- `MStatus setSoftMin (double minValue)`
- `MStatus setSoftMin (const MTime &minValue)`
- `MStatus setSoftMin (const MAngle &minValue)`
- `MStatus setSoftMin (const MDistance &minValue)`
- `MStatus setSoftMax (double maxValue)`
- `MStatus setSoftMax (const MTime &maxValue)`
- `MStatus setSoftMax (const MAngle &maxValue)`
- `MStatus setSoftMax (const MDistance &maxValue)`
- `MStatus getDefault (double &defaultValue)`
- `MStatus getDefault (MTime &defaultValue)`
- `MStatus getDefault (MAngle &defaultValue)`
- `MStatus getDefault (MDistance &defaultValue)`
- `MStatus setDefault (double defaultValue)`
- `MStatus setDefault (const MTime &defaultValue)`
- `MStatus setDefault (const MAngle &defaultValue)`
- `MStatus setDefault (const MDistance &defaultValue)`
- `MFnUnitAttribute (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnVectorArrayData

Module `OpenMaya`. Inherits `MFnData`. Source: [class_m_fn_vector_array_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_vector_array_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnVectorArrayData ()`
- `MFnVectorArrayData ()`
- `MFnVectorArrayData (MObject &object, MStatus *ReturnStatus=NULL)`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus set (MVector &element, unsigned int index)`
- `MStatus copyTo (MVectorArray &) const`
- `MStatus set (const MVectorArray &newArray)`
- `MVectorArray array (MStatus *ReturnStatus=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MVectorArray &in, MStatus *ReturnStatus=NULL)`
- `MFnVectorArrayData (const MObject &object, MStatus *ReturnStatus=NULL)`
- `const MVector & operator[] (unsigned int index) const`
- `MVector & operator[] (unsigned int index)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnVolumeLight

Module `OpenMaya`. Inherits `MFnDagNode`. Source: [class_m_fn_volume_light.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_volume_light.html).

**Public Types**

- `enum MLightShape { kBoxVolume, kSphereVolume, kCylinderVolume, kConeVolume }`
- `enum MLightDirection { kOutward, kInward, kDownAxis }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnVolumeLight ()`
- `MFnVolumeLight ()`
- `MFnVolumeLight (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnVolumeLight (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (bool UIvisible=true, MStatus *ReturnStatus=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, bool wantTransform=false, MStatus *ReturnStatus=NULL)`
- `MFnVolumeLight::MLightShape lightShape (MStatus *ReturnStatus=NULL) const`
- `MStatus setLightShape (const MFnVolumeLight::MLightShape &light_shape)`
- `MFnVolumeLight::MLightDirection volumeLightDirection (MStatus *ReturnStatus=NULL) const`
- `MStatus setVolumeLightDirection (const MFnVolumeLight::MLightDirection &volume_light_direction)`
- `float arc (MStatus *ReturnStatus=NULL) const`
- `MStatus setArc (const float &arc)`
- `float coneEndRadius (MStatus *ReturnStatus=NULL) const`
- `MStatus setConeEndRadius (const float &cone_end_radius)`
- `bool emitAmbient (MStatus *ReturnStatus=NULL) const`
- `MStatus setEmitAmbient (const bool &emit_ambient)`
- `MRampAttribute colorRamp (MStatus *ReturnStatus=NULL) const`
- `MRampAttribute penumbraRamp (MStatus *ReturnStatus=NULL) const`
- `float shadowAngle (MStatus *ReturnStatus=NULL) const`
- `MStatus setShadowAngle (const float &shadow_angle)`
- `MFnVolumeLight (const MObject &object, MStatus *ret=NULL)`
- `MObject create (const MObject &parent, bool UIvisible=true, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MGlobal

Module `OpenMaya`. Source: [class_m_global.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_global.html).

**Public Types**

- `enum MMayaState { kInteractive, kBatch, kLibraryApp, kBaseUIMode }`
- `enum ListAdjustment { kReplaceList, kXORWithList, kAddToList, kRemoveFromList, kAddToHeadOfList }`
- `enum SelectionMethod { kSurfaceSelectMethod, kWireframeSelectMethod }`
- `enum MSelectionMode { kSelectObjectMode, kSelectComponentMode, kSelectRootMode, kSelectLeafMode, kSelectTemplateMode }`
- `enum MIdleTaskPriority { kHighIdlePriority, kLowIdlePriority, kVeryLowIdlePriority }`
- `typedef void(* MIdleTaskCallback) (void *clientData)`

**Static Public Member Functions**

- `static MString mayaVersion ()`
- `static int apiVersion ()`
- `static bool isCustomVersion ()`
- `static MString customVersionString ()`
- `static int customVersionMajor ()`
- `static int customVersionMinor ()`
- `static MString customVersionClient ()`
- `static int customVersion ()`
- `static MString getAbsolutePathToResources ()`
- `static bool isDefaultLanguage ()`
- `static MString getUILanguage ()`
- `static MMayaState mayaState (MStatus *ReturnStatus=NULL)`
- `static MStatus getFunctionSetList (MObject forObject, MStringArray &array)`
- `static MStatus getAssociatedSets (const MSelectionList &list, MObjectArray &setArray)`
- `static MStatus getSelectionListByName (const MString &name, MSelectionList &list)`
- `static MStatus getActiveSelectionList (MSelectionList &dest, bool orderedSelectionIfAvailable=false)`
- `static MStatus getRichSelection (MRichSelection &dest, bool defaultToActiveSelection=true)`
- `static MStatus getLiveList (MSelectionList &dest)`
- `static MStatus getHiliteList (MSelectionList &dest)`
- `static MStatus setHiliteList (MSelectionList &src)`
- `static MStatus setActiveSelectionList (const MSelectionList &src, ListAdjustment=kReplaceList)`
- `static MStatus setRichSelection (const MRichSelection &src)`
- `static MStatus setDisplayCVs (MSelectionList &, bool)`
- `static MStatus selectCommand (const MSelectionList &src, ListAdjustment=kReplaceList)`
- `static MStatus selectByName (const MString &name, ListAdjustment=kAddToList)`
- `static MStatus unselectByName (const MString &name)`
- `static MStatus select (MObject &object, ListAdjustment=kAddToList)`
- `static MStatus select (const MDagPath &object, const MObject &component, ListAdjustment=kAddToList)`
- `static MStatus unselect (MObject &object)`
- `static MStatus unselect (const MDagPath &object, const MObject &component)`
- `static MStatus selectFromScreen (const short &x_pos, const short &y_pos, ListAdjustment=kAddToList, SelectionMethod=kWireframeSelectMethod)`
- `static MStatus selectFromScreen (const short &start_x, const short &start_y, const short &end_x, const short &end_y, ListAdjustment=kAddToList, SelectionMethod=kWireframeSelectMethod)`
- `static SelectionMethod selectionMethod (MStatus *ReturnStatus=NULL)`
- `static MSelectionMode selectionMode (MStatus *ReturnStatus=NULL)`
- `static MStatus setSelectionMode (MSelectionMode mode)`
- `static MSelectionMask objectSelectionMask (MStatus *ReturnStatus=NULL)`
- `static MStatus setObjectSelectionMask (const MSelectionMask &mask)`
- `static MSelectionMask componentSelectionMask (MStatus *ReturnStatus=NULL)`
- `static MStatus setComponentSelectionMask (const MSelectionMask &mask)`
- `static MSelectionMask animSelectionMask (MStatus *ReturnStatus=NULL)`
- `static MStatus setAnimSelectionMask (const MSelectionMask &mask)`
- `static MSelectionMask miscSelectionMask (MStatus *ReturnStatus=NULL)`
- `static MStatus setMiscSelectionMask (const MSelectionMask &mask)`
- `static MStatus clearSelectionList ()`
- `static bool isSelected (MObject &object, MStatus *ReturnStatus=NULL)`
- `static bool trackSelectionOrderEnabled (MStatus *ReturnStatus=NULL)`
- `static MStatus setTrackSelectionOrderEnabled (bool enable)`
- `static MStatus getPreselectionHiliteList (MSelectionList &list)`
- `static MStatus setPreselectionHiliteList (const MSelectionList &list)`
- `static MObject currentToolContext ()`
- `static MStatus viewFrame (const MTime &time)`
- `static MStatus viewFrame (double time)`
- `static MStatus sourceFile (const MString &fileName)`
- `static MStatus executeCommandOnIdle (const MString &command, bool displayEnabled=false)`
- `static MStatus executeCommand (const MString &command, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, MCommandResult &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, int &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, MIntArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, double &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, MDoubleArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeCommand (const MString &command, MStringArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MString executeCommandStringResult (const MString &command, bool displayEnabled=false, bool undoEnabled=false, MStatus *ResultStatus=NULL)`
- `static MStatus executeCommand (const MString &command, MString &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executeTaskOnIdle (MIdleTaskCallback task, void *clientData=NULL, MIdleTaskPriority priority=kLowIdlePriority)`
- `static MStatus executePythonCommand (const MString &command, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommand (const MString &command, int &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommand (const MString &command, MIntArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommand (const MString &command, double &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommand (const MString &command, MDoubleArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MString executePythonCommandStringResult (const MString &command, bool displayEnabled=false, bool undoEnabled=false, MStatus *ReturnStatus=NULL)`
- `static MStatus executePythonCommand (const MString &command, MString &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommand (const MString &command, MStringArray &result, bool displayEnabled=false, bool undoEnabled=false)`
- `static MStatus executePythonCommandOnIdle (const MString &command, bool displayEnabled=false)`
- `static PyObject * callPythonFunction (PyObject *func, PyObject *args)`
- `static void catchAndReportPythonException ()`
- `static void formatPythonError (PyObject *exception, const char *format, va_list args)`
- `static MStatus addToModel (MObject &object, MObject &parent=MObject::kNullObj)`
- `static MStatus addToModelAt (MObject &, const MVector &point, const double scale[3]=NULL, const double rotation[3]=NULL, MTransformationMatrix::RotationOrder order=MTransformationMatrix::kXYZ)`
- `static MStatus removeFromModel (MObject &)`
- `static MStatus deleteNode (MObject &)`
- `static MStatus setYAxisUp (bool rotateView=false)`
- `static MStatus setZAxisUp (bool rotateView=false)`
- `static bool isYAxisUp (MStatus *=NULL)`
- `static bool isZAxisUp (MStatus *=NULL)`
- `static MVector upAxis (MStatus *=NULL)`
- `static void displayInfo (const MString &theMessage)`
- `static void displayWarning (const MString &theWarning)`
- `static void displayError (const MString &theError)`
- `static void setDisableStow (bool state)`
- `static bool disableStow ()`
- `static int optionVarIntValue (const MString &name, bool *exists=NULL)`
- `static double optionVarDoubleValue (const MString &name, bool *exists=NULL)`
- `static MString optionVarStringValue (const MString &name, bool *exists=NULL)`
- `static bool setOptionVarValue (const MString &name, int value)`
- `static bool setOptionVarValue (const MString &name, double value)`
- `static bool setOptionVarValue (const MString &name, MString value)`
- `static void removeOptionVar (const MString &name)`
- `static bool optionVarExists (const MString &name)`
- `static MString defaultErrorLogPathName (MStatus *ReturnStatus=NULL)`
- `static MStatus resetToDefaultErrorLogPathName ()`
- `static MStatus setErrorLogPathName (const MString &logPathName)`
- `static MString errorLogPathName (MStatus *ReturnStatus=NULL)`
- `static MStatus startErrorLogging ()`
- `static MStatus startErrorLogging (const MString &logPathName)`
- `static bool errorLoggingIsOn (MStatus *ReturnStatus=NULL)`
- `static MStatus stopErrorLogging ()`
- `static MStatus closeErrorLog ()`
- `static MStatus doErrorLogEntry (const MString &errorLogEntry)`
- `static bool isUndoing ()`
- `static bool isRedoing ()`
- `static const char * className ()`
- `static bool getOptionVarValue (const MString &name, int &value)`
- `static bool getOptionVarValue (const MString &name, double &value)`
- `static bool getOptionVarValue (const MString &name, MString &value)`
- `static MStatus select (MDagPath &object, const MObject &component, ListAdjustment=kAddToList)`
- `static MStatus unselect (MDagPath &object, const MObject &component)`

### MGraphNodeIterator

Module `OpenMaya`. Source: [class_m_graph_node_iterator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_graph_node_iterator.html).

**Public Member Functions**

- `MGraphNodeIterator (const MCustomEvaluatorClusterNode *cluster, MStatus *ReturnStatus)`
- `MGraphNodeIterator (const MGraphNodeIterator &iterator, MStatus *ReturnStatus)`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `MEvaluationNode currentEvaluationNode (MStatus *ReturnStatus=NULL) const`
- `void next (MStatus *ReturnStatus=NULL) const`
- `void reset (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MIffFile

Module `OpenMaya`. Source: [class_m_iff_file.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_iff_file.html).

**Public Member Functions**

- `MIffFile ()`
- `MIffFile (const MString &fileName)`
- `virtual ~MIffFile ()`
- `MStatus open (const MString &fileName)`
- `MStatus close ()`
- `bool isActive () const`
- `MStatus beginReadGroup (MIffTag &id, MIffTag &type)`
- `MStatus endReadGroup ()`
- `const void * getChunk (MIffTag &chunkTag, unsigned *byteCount)`
- `MStatus beginGet (MIffTag &chunkTag, unsigned *byteCount)`
- `MStatus endGet ()`
- `int get (void *buf, unsigned byteCount, MStatus *stat=NULL)`
- `MStatus iffGetShort (short int &result)`
- `MStatus iffGetInt (int &result)`
- `MStatus iffGetFloat (float &result)`

**Static Public Member Functions**

- `static const char * className ()`

### MIffTag

Module `OpenMaya`. Source: [class_m_iff_tag.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_iff_tag.html).

**Public Member Functions**

- `MIffTag ()`
- `MIffTag (unsigned char a, unsigned char b, unsigned char c, unsigned char d)`
- `MIffTag (unsigned int tag)`
- `~MIffTag ()`
- `bool operator== (const MIffTag &tag) const`

**Static Public Attributes**

- `static const MIffTag kFORM`
- `static const MIffTag kCAT`
- `static const MIffTag kLIST`
- `static const MIffTag kPROP`
- `static const MIffTag kFOR4`
- `static const MIffTag kFOR8`
- `static const MIffTag kCAT4`
- `static const MIffTag kCAT8`
- `static const MIffTag kLIS4`
- `static const MIffTag kLIS8`
- `static const MIffTag kPRO4`
- `static const MIffTag kPRO8`

### MImage

Module `OpenMaya`. Source: [class_m_image.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_image.html).

**Public Types**

- `enum MImageFilterFormat { kNoFormat, kHeightFieldBumpFormat, kNormalMapBumpFormat, kUnknownFormat }`
- `enum MPixelType { kUnknown, kByte, kFloat }`

**Public Member Functions**

- `MImage ()`
- `~MImage ()`
- `MStatus create (unsigned int width, unsigned int height, unsigned int channels=4, MPixelType type=kByte)`
- `MStatus readFromFile (MString pathname, MPixelType type=kByte)`
- `MStatus readFromTextureNode (const MObject &fileTextureObject, MPixelType type=kByte)`
- `MStatus getSize (unsigned int &width, unsigned int &height) const`
- `MPixelType pixelType () const`
- `unsigned char * pixels () const`
- `float * floatPixels () const`
- `void setPixels (unsigned char *pixels, unsigned int width, unsigned int height)`
- `void setFloatPixels (float *pixels, unsigned int width, unsigned int height, unsigned int channels=4)`
- `unsigned int depth () const`
- `MStatus getDepthMapSize (unsigned int &width, unsigned int &height) const`
- `MStatus getDepthMapRange (float &minValue, float &maxValue) const`
- `MStatus setDepthMap (float *depth, unsigned width, unsigned height)`
- `MStatus setDepthMap (const MFloatArray &depth, unsigned width, unsigned height)`
- `float * depthMap (MStatus *ReturnStatus=NULL) const`
- `MStatus readDepthMap (MString pathname)`
- `MStatus resize (int width, int height, bool preserveAspectRatio=true)`
- `MStatus filter (MImageFilterFormat sourceFormat, MImageFilterFormat targetFormat, double scale=1.0, double offset=0.0)`
- `MStatus writeToFile (MString pathname, MString outputFormat=kIffStringDefault) const`
- `MStatus writeToFileWithDepth (MString pathname, MString outputFormat=kIffStringDefault, bool writeDepth=false) const`
- `MStatus release ()`
- `void verticalFlip ()`
- `void setRGBA (bool rgbaFormat)`
- `bool isRGBA () const`
- `bool haveDepth () const`
- `MStatus convertPixelFormat (MPixelType type, double scale=1.0, double offset=0.0)`

**Static Public Member Functions**

- `static bool filterExists (MImageFilterFormat sourceFormat, MImageFilterFormat targetFormat)`
- `static const char * className ()`

### MImageFileInfo

Module `OpenMaya`. Source: [class_m_image_file_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_image_file_info.html).

**Public Types**

- `enum MHwTextureType { kHwTextureUnknown, kHwTexture1D, kHwTexture2D, kHwTexture3D, kHwTextureRectangle, kHwTextureCubeMap }`
- `enum MImageType { kImageTypeUnknown, kImageTypeColor, kImageTypeNormal, kImageTypeBump }`

**Public Member Functions**

- `MImageFileInfo ()`
- `void width (unsigned int value)`
- `void height (unsigned int value)`
- `void channels (unsigned int value)`
- `void numberOfImages (unsigned int value)`
- `void pixelType (MImage::MPixelType value)`
- `void imageType (MImageType value)`
- `void hardwareType (MHwTextureType value)`
- `void hasAlpha (bool value)`
- `void hasMipMaps (bool value)`
- `unsigned int width () const`
- `unsigned int height () const`
- `unsigned int channels () const`
- `unsigned int numberOfImages () const`
- `MImage::MPixelType pixelType () const`
- `MImageType imageType () const`
- `MHwTextureType hardwareType () const`
- `bool hasAlpha () const`
- `bool hasMipMaps () const`

### MInt64Array

Module `OpenMaya`. Source: [class_m_int64_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_int64_array.html).

**Public Member Functions**

- `MInt64Array ()`
- `MInt64Array (const MInt64Array &other)`
- `MInt64Array (const MInt64 src[], unsigned int count)`
- `MInt64Array (unsigned int initialSize, MInt64 initialValue=0)`
- `~MInt64Array ()`
- `MInt64Array & operator= (const MInt64Array &other)`
- `MStatus set (MInt64 element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (MInt64 element, unsigned int index)`
- `MStatus append (MInt64 element)`
- `MStatus copy (const MInt64Array &source)`
- `MStatus clear ()`
- `MStatus get (MInt64[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MInt64 & operator[] (unsigned int index)`
- `MInt64 operator[] (unsigned int index) const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MInt64Array &array)`

### MIntArray

Module `OpenMaya`. Source: [class_m_int_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_int_array.html).

**Public Member Functions**

- `MIntArray ()`
- `MIntArray (const MIntArray &other)`
- `MIntArray (const int src[], unsigned int count)`
- `MIntArray (unsigned int initialSize, int initialValue=0)`
- `~MIntArray ()`
- `MIntArray & operator= (const MIntArray &other)`
- `MStatus set (int element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (int element, unsigned int index)`
- `MStatus append (int element)`
- `MStatus copy (const MIntArray &source)`
- `MStatus clear ()`
- `MStatus get (int[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `int & operator[] (unsigned int index)`
- `int operator[] (unsigned int index) const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MIntArray &array)`

### MItCurveCV

Module `OpenMaya`. Source: [class_m_it_curve_c_v.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_curve_c_v.html).

**Public Member Functions**

- `MItCurveCV (MObject &curve, MStatus *ReturnStatus=NULL)`
- `MItCurveCV (const MDagPath &curve, MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `virtual ~MItCurveCV ()`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus reset (MObject &curve)`
- `MStatus reset (const MDagPath &curve, MObject &component=MObject::kNullObj)`
- `MPoint position (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus setPosition (const MPoint &pt, MSpace::Space space=MSpace::kObject)`
- `MStatus translateBy (const MVector &vec, MSpace::Space space=MSpace::kObject)`
- `int index (MStatus *ReturnStatus=NULL) const`
- `MObject cv (MStatus *ReturnStatus=NULL) const`
- `MObject currentItem (MStatus *ReturnStatus=NULL) const`
- `bool hasHistoryOnCreate (MStatus *ReturnStatus=NULL) const`
- `MStatus updateCurve ()`

**Static Public Member Functions**

- `static const char * className ()`

### MItDag

Module `OpenMaya`. Source: [class_m_it_dag.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_dag.html).

**Public Types**

- `enum TraversalType { kInvalidType, kDepthFirst, kBreadthFirst }`

**Public Member Functions**

- `MItDag (TraversalType=kDepthFirst, MFn::Type=MFn::kInvalid, MStatus *ReturnStatus=NULL)`
- `MItDag (MIteratorType &dagInfoObject, TraversalType=kDepthFirst, MStatus *ReturnStatus=NULL)`
- `MStatus reset ()`
- `MStatus reset (const MObject &object, TraversalType=kDepthFirst, MFn::Type=MFn::kInvalid)`
- `MStatus reset (const MDagPath &path, TraversalType=kDepthFirst, MFn::Type=MFn::kInvalid)`
- `MStatus reset (MIteratorType &dagInfoObject, const MObject *object, const MDagPath *path, TraversalType=kDepthFirst)`
- `MObject item (MStatus *ReturnStatus=NULL)`
- `MObject currentItem (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus prune ()`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `MObject root (MStatus *ReturnStatus=NULL)`
- `unsigned int depth (MStatus *ReturnStatus=NULL) const`
- `TraversalType getType (MStatus *ReturnStatus=NULL) const`
- `MStatus getPath (MDagPath &path) const`
- `MStatus getAllPaths (MDagPathArray &paths) const`
- `MString fullPathName (MStatus *ReturnStatus=NULL) const`
- `MString partialPathName (MStatus *ReturnStatus=NULL) const`
- `bool isInstanced (bool indirect=true, MStatus *ReturnStatus=NULL) const`
- `unsigned int instanceCount (bool total, MStatus *ReturnStatus=NULL) const`
- `MStatus traverseUnderWorld (bool flag)`
- `bool willTraverseUnderWorld (MStatus *ReturnStatus=NULL) const`
- `virtual ~MItDag ()`

**Static Public Member Functions**

- `static const char * className ()`

### MItDependencyGraph

Module `OpenMaya`. Source: [class_m_it_dependency_graph.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_dependency_graph.html).

**Public Types**

- `enum Direction { kDownstream, kUpstream }`
- `enum Traversal { kDepthFirst, kBreadthFirst }`
- `enum Level { kNodeLevel, kPlugLevel }`
- `enum Relationship { kDependsOn, kConnectedTo, kEvaluationGraph }`

**Public Member Functions**

- `MItDependencyGraph (MObject &rootNode, MFn::Type filter=MFn::kInvalid, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kNodeLevel, MStatus *ReturnStatus=NULL)`
- `MItDependencyGraph (MPlug &rootPlug, MFn::Type filter=MFn::kInvalid, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kPlugLevel, MStatus *ReturnStatus=NULL)`
- `MItDependencyGraph (MObject &rootNode, MPlug &rootPlug, MIteratorType &infoObject, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kNodeLevel, MStatus *ReturnStatus=NULL)`
- `MItDependencyGraph (MObject &rootNode, MFn::Type filter, Direction direction, Traversal traversal, Level level, Relationship relationship, MStatus *ReturnStatus=NULL)`
- `MItDependencyGraph (MPlug &rootPlug, MFn::Type filter, Direction direction, Traversal traversal, Level level, Relationship relationship, MStatus *ReturnStatus=NULL)`
- `MItDependencyGraph (MObject &rootNode, MPlug &rootPlug, MIteratorType &infoObject, Direction direction, Traversal traversal, Level level, Relationship relationship, MStatus *ReturnStatus=NULL)`
- `~MItDependencyGraph ()`
- `MStatus reset ()`
- `MStatus resetTo (MObject &rootNode, MFn::Type filter=MFn::kInvalid, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kNodeLevel, Relationship relationship=kDependsOn)`
- `MStatus resetTo (MPlug &rootPlug, MFn::Type filter=MFn::kInvalid, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kPlugLevel, Relationship relationship=kDependsOn)`
- `MStatus resetTo (MObject *rootNode, MPlug *rootPlug, MIteratorType &infoObject, Direction direction=kDownstream, Traversal traversal=kDepthFirst, Level level=kPlugLevel, Relationship relationship=kDependsOn)`
- `MObject rootNode (MStatus *ReturnStatus=NULL)`
- `MPlug rootPlug (MStatus *ReturnStatus=NULL)`
- `MFn::Type currentFilter (MStatus *ReturnStatus=NULL)`
- `MStatus setCurrentFilter (MFn::Type filter=MFn::kInvalid)`
- `MStatus resetFilter ()`
- `bool isPruningOnFilter (MStatus *ReturnStatus=NULL)`
- `MStatus enablePruningOnFilter ()`
- `MStatus disablePruningOnFilter ()`
- `bool isDirectionDownStream (MStatus *ReturnStatus=NULL)`
- `Direction currentDirection (MStatus *ReturnStatus=NULL)`
- `MStatus toggleDirection ()`
- `bool isTraversalDepthFirst (MStatus *ReturnStatus=NULL)`
- `Traversal currentTraversal (MStatus *ReturnStatus=NULL)`
- `MStatus toggleTraversal ()`
- `bool atNodeLevel (MStatus *ReturnStatus=NULL)`
- `Level currentLevel (MStatus *ReturnStatus=NULL)`
- `Relationship currentRelationship (MStatus *ReturnStatus=NULL)`
- `MStatus setCurrentRelationship (Relationship relationship)`
- `MStatus setTraversalOverWorldSpaceDependents (bool val)`
- `bool isTraversingOverWorldSpaceDependents (MStatus *ReturnStatus=NULL)`
- `MStatus toggleLevel ()`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus prune ()`
- `MObject thisNode (MStatus *ReturnStatus=NULL)`
- `MObject currentItem (MStatus *ReturnStatus=NULL)`
- `bool thisNodeHasUnknownType (MStatus *ReturnStatus=NULL)`
- `MPlug thisPlug (MStatus *ReturnStatus=NULL)`
- `MPlug previousPlug (MStatus *ReturnStatus=NULL)`
- `MStatus getNodesVisited (MObjectArray &nodesVisted) const`
- `MStatus getPlugsVisited (MPlugArray &plugsVisted) const`
- `MStatus getNodePath (MObjectArray &path) const`
- `MStatus getPlugPath (MPlugArray &path) const`

**Static Public Member Functions**

- `static const char * className ()`

### MItDependencyNodes

Module `OpenMaya`. Source: [class_m_it_dependency_nodes.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_dependency_nodes.html).

**Public Member Functions**

- `MItDependencyNodes (MFn::Type filter=MFn::kInvalid, MStatus *ReturnStatus=NULL)`
- `MItDependencyNodes (MIteratorType &infoObject, MStatus *ReturnStatus=NULL)`
- `virtual ~MItDependencyNodes ()`
- `MStatus reset (MFn::Type filter=MFn::kInvalid)`
- `MStatus reset (MIteratorType &infoObject)`
- `MObject thisNode (MStatus *ReturnStatus=NULL) const`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `MObject item (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MItEdits

Module `OpenMaya`. Source: [class_m_it_edits.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_edits.html).

**Public Types**

- `enum EditStatus { SUCCESSFUL_EDITS, ALL_EDITS }`
- `enum Direction { kForward, kReverse }`

**Public Member Functions**

- `MItEdits (MObject &editsOwner, MObject &targetNode=MObject::kNullObj, EditStatus editStatus=ALL_EDITS, Direction direction=kForward, MStatus *ReturnStatus=NULL)`
- `MItEdits (MObject &editsOwner, const MString &targetNodeName, EditStatus editStatus=ALL_EDITS, Direction direction=kForward, MStatus *ReturnStatus=NULL)`
- `virtual ~MItEdits ()`
- `MStatus reset ()`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `bool isReverse (MStatus *ReturnStatus=NULL) const`
- `MString currentEditString (MStatus *ReturnStatus=NULL) const`
- `MEdit::EditType currentEditType (MStatus *ReturnStatus=NULL) const`
- `bool removeCurrentEdit (MStatus *ReturnStatus=NULL)`
- `MEdit edit (MStatus *ReturnStatus=NULL) const`
- `MAddRemoveAttrEdit addRemoveAttrEdit (MStatus *ReturnStatus=NULL) const`
- `MSetAttrEdit setAttrEdit (MStatus *ReturnStatus=NULL) const`
- `MParentingEdit parentingEdit (MStatus *ReturnStatus=NULL) const`
- `MFcurveEdit fcurveEdit (MStatus *ReturnStatus=NULL) const`
- `MConnectDisconnectAttrEdit connectDisconnectEdit (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MItGeometry

Module `OpenMaya`. Source: [class_m_it_geometry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_geometry.html).

**Public Member Functions**

- `MItGeometry ()`
- `MItGeometry (const MDagPath &dagPath, MStatus *ReturnStatus=NULL)`
- `MItGeometry (const MDagPath &dagPath, MObject &component, MStatus *ReturnStatus=NULL)`
- `MItGeometry (MObject &dagObject, MStatus *ReturnStatus=NULL)`
- `MItGeometry (MDataHandle &dataHandle, unsigned int groupId, bool readOnly=true, MStatus *ReturnStatus=NULL)`
- `MItGeometry (MDataHandle &dataHandle, bool readOnly=true, MStatus *ReturnStatus=NULL)`
- `MItGeometry (MDataHandle &dataHandle, MObject &component, bool readOnly=true, MStatus *ReturnStatus=NULL)`
- `MItGeometry (MDataHandle &dataHandle, const MString &componentTagExpression, bool readOnly=true, MStatus *ReturnStatus=NULL)`
- `virtual ~MItGeometry ()`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `MStatus next ()`
- `MPoint position (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MVector normal (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus setPosition (const MPoint &point, MSpace::Space space=MSpace::kObject)`
- `MWeight weight (MStatus *ReturnStatus=NULL) const`
- `int index (MStatus *ReturnStatus=NULL) const`
- `int positionIndex (MStatus *ReturnStatus=NULL) const`
- `MObject component (MStatus *ReturnStatus=NULL) const`
- `MObject currentItem (MStatus *ReturnStatus=NULL) const`
- `int count (MStatus *ReturnStatus=NULL) const`
- `int exactCount (MStatus *ReturnStatus=NULL)`
- `MStatus reset ()`
- `MStatus allPositions (MPointArray &points, MSpace::Space space=MSpace::kObject) const`
- `MStatus setAllPositions (const MPointArray &points, MSpace::Space space=MSpace::kObject)`

**Static Public Member Functions**

- `static const char * className ()`

### MItInstancer

Module `OpenMaya`. Source: [class_m_it_instancer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_instancer.html).

**Public Member Functions**

- `MItInstancer ()`
- `virtual ~MItInstancer ()`
- `MStatus reset ()`
- `MStatus next ()`
- `MStatus nextParticle ()`
- `MStatus nextInstancer ()`
- `bool isDone () const`
- `MObject instancer () const`
- `MDagPath instancerPath () const`
- `MDagPath path ()`
- `MMatrix matrix ()`
- `uintptr_t instancerId () const`
- `unsigned int particleId () const`
- `unsigned int pathId () const`

**Static Public Member Functions**

- `static const char * className ()`

### MItMeshEdge

Module `OpenMaya`. Source: [class_m_it_mesh_edge.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_edge.html).

**Public Member Functions**

- `MItMeshEdge (MObject &polyObject, MStatus *ReturnStatus=NULL)`
- `MItMeshEdge (MObject &polyObject, MObject &component, MStatus *ReturnStatus=NULL)`
- `MItMeshEdge (const MDagPath &polyObject, MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `virtual ~MItMeshEdge ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus reset (MObject &polyObject, MObject &component=MObject::kNullObj)`
- `MStatus reset (const MDagPath &polyObject, MObject &component=MObject::kNullObj)`
- `int count (MStatus *ReturnStatus=NULL)`
- `MPoint center (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MPoint point (int index, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MStatus setPoint (const MPoint &point, unsigned int index, MSpace::Space space=MSpace::kObject)`
- `bool isSmooth (MStatus *ReturnStatus=NULL) const`
- `MStatus setSmoothing (bool smooth=true)`
- `MStatus cleanupSmoothing ()`
- `int index (int index, MStatus *ReturnStatus=NULL)`
- `int index (MStatus *ReturnStatus=NULL) const`
- `MObject edge (MStatus *ReturnStatus=NULL)`
- `MObject currentItem (MStatus *ReturnStatus=NULL)`
- `MStatus updateSurface ()`
- `MStatus geomChanged ()`
- `MStatus setIndex (int index, int &prevIndex)`
- `int getConnectedFaces (MIntArray &faceList, MStatus *ReturnStatus=NULL) const`
- `int getConnectedEdges (MIntArray &edgeList, MStatus *ReturnStatus=NULL) const`
- `MStatus numConnectedFaces (int &faceCount) const`
- `MStatus numConnectedEdges (int &edgeCount) const`
- `bool connectedToFace (int index, MStatus *ReturnStatus=NULL)`
- `bool connectedToEdge (int index, MStatus *ReturnStatus=NULL)`
- `bool onBoundary (MStatus *ReturnStatus=NULL)`
- `MStatus getLength (double &length, MSpace::Space space=MSpace::kObject)`

**Static Public Member Functions**

- `static const char * className ()`

### MItMeshFaceVertex

Module `OpenMaya`. Source: [class_m_it_mesh_face_vertex.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_face_vertex.html).

**Public Member Functions**

- `MItMeshFaceVertex (const MObject &polyObject, MStatus *ReturnStatus=NULL)`
- `MItMeshFaceVertex (const MDagPath &polyObject, MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `virtual ~MItMeshFaceVertex ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus reset (const MObject &polyObject)`
- `MStatus reset (const MDagPath &polyObject, MObject &component=MObject::kNullObj)`
- `int vertId (MStatus *ReturnStatus=NULL) const`
- `int faceId (MStatus *ReturnStatus=NULL) const`
- `int faceVertId (MStatus *ReturnStatus=NULL) const`
- `MObject faceVertex (MStatus *ReturnStatus=NULL) const`
- `MObject currentItem (MStatus *ReturnStatus=NULL) const`
- `MPoint position (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MStatus getNormal (MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `int normalId (MStatus *ReturnStatus=NULL) const`
- `int tangentId (MStatus *ReturnStatus=NULL) const`
- `MVector getTangent (MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, MStatus *ReturnStatus=NULL) const`
- `MVector getBinormal (MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, MStatus *ReturnStatus=NULL) const`
- `bool hasUVs (MStatus *ReturnStatus=NULL) const`
- `bool hasUVs (const MString &uvSet, MStatus *ReturnStatus=NULL) const`
- `MStatus getUV (float2 &uvPoint, const MString *uvSet=NULL) const`
- `MStatus getUVIndex (int &index, const MString *uvSet=NULL)`
- `MStatus getUVIndex (int &index, float &u, float &v, const MString *uvSet=NULL)`
- `bool hasColor (MStatus *ReturnStatus=NULL) const`
- `MStatus getColor (MColor &color, const MString *colorSetName=NULL)`
- `MStatus getColorIndex (int &colorIndex, const MString *colorSetName=NULL)`
- `MStatus updateSurface ()`
- `MStatus geomChanged ()`
- `MStatus setIndex (int faceId, int vertFaceId, int &prevFaceId, int &prevVertId)`

**Static Public Member Functions**

- `static const char * className ()`

### MItMeshPolygon

Module `OpenMaya`. Source: [class_m_it_mesh_polygon.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_polygon.html).

**Public Member Functions**

- `MItMeshPolygon (const MObject &polyObject, MStatus *ReturnStatus=NULL)`
- `MItMeshPolygon (const MDagPath &polyObject, MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `virtual ~MItMeshPolygon ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus reset (const MObject &polyObject)`
- `MStatus reset (const MDagPath &polyObject, MObject &component=MObject::kNullObj)`
- `unsigned int count (MStatus *ReturnStatus=NULL)`
- `unsigned int polygonVertexCount (MStatus *ReturnStatus=NULL)`
- `MPoint center (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MObject polygon (MStatus *ReturnStatus=NULL)`
- `MObject currentItem (MStatus *ReturnStatus=NULL)`
- `unsigned int index (MStatus *ReturnStatus=NULL)`
- `MStatus setIndex (int index, int &prevIndex)`
- `unsigned int vertexIndex (int index, MStatus *ReturnStatus=NULL)`
- `MStatus getVertices (MIntArray &vertices)`
- `MPoint point (int index, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `void getPoints (MPointArray &pointArray, MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MStatus setPoint (const MPoint &point, unsigned int index, MSpace::Space space=MSpace::kObject)`
- `MStatus setPoints (MPointArray &pointArray, MSpace::Space space=MSpace::kObject)`
- `unsigned int normalIndex (int vertex, MStatus *ReturnStatus=NULL) const`
- `MStatus getNormal (MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `MStatus getNormal (unsigned int Vertexindex, MVector &normal, MSpace::Space space=MSpace::kObject) const`
- `MStatus getNormals (MVectorArray &vectorArray, MSpace::Space space=MSpace::kObject) const`
- `unsigned int tangentIndex (int vertex, MStatus *ReturnStatus=NULL) const`
- `bool hasUVs (MStatus *ReturnStatus=NULL) const`
- `bool hasUVs (const MString &uvSet, MStatus *ReturnStatus=NULL) const`
- `MStatus setUV (int vertexId, float2 &uvPoint, const MString *uvSet=NULL)`
- `MStatus getUV (int vertexId, float2 &uvPoint, const MString *uvSet=NULL) const`
- `MStatus setUVs (MFloatArray &uArray, MFloatArray &vArray, const MString *uvSet=NULL)`
- `MStatus getUVs (MFloatArray &uArray, MFloatArray &vArray, const MString *uvSet=NULL) const`
- `MStatus getPointAtUV (MPoint &pt, float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, float tolerance=0.0)`
- `MStatus getAxisAtUV (MVector &normal, MVector &uTangent, MVector &vTangent, float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL, float tolerance=0.0)`
- `MStatus getUVAtPoint (MPoint &pt, float2 &uvPoint, MSpace::Space space=MSpace::kObject, const MString *uvSet=NULL)`
- `MStatus getUVIndex (int vertex, int &index, const MString *uvSet=NULL)`
- `MStatus getUVIndex (int vertex, int &index, float &u, float &v, const MString *uvSet=NULL)`
- `MStatus getUVSetNames (MStringArray &setNames) const`
- `bool hasColor (MStatus *ReturnStatus=NULL) const`
- `bool hasColor (int localVertexIndex, MStatus *ReturnStatus=NULL)`
- `MStatus getColor (MColor &color, const MString *colorSetName=NULL)`
- `MStatus getColor (MColor &color, int vertexIndex)`
- `MStatus getColors (MColorArray &colors, const MString *colorSetName=NULL)`
- `MStatus numColors (int &count, const MString *colorSetName=NULL)`
- `MStatus getColorIndex (int vertexIndex, int &colorIndex, const MString *colorSetName=NULL)`
- `MStatus getColorIndices (MIntArray &colorIndex, const MString *colorSetName=NULL)`
- `bool hasValidTriangulation (MStatus *ReturnStatus=NULL) const`
- `MStatus numTriangles (int &count) const`
- `MStatus getTriangle (int localTriIndex, MPointArray &points, MIntArray &vertexList, MSpace::Space space=MSpace::kObject) const`
- `MStatus getTriangles (MPointArray &points, MIntArray &vertexList, MSpace::Space space=MSpace::kObject) const`
- `MStatus updateSurface ()`
- `MStatus geomChanged ()`
- `MStatus getEdges (MIntArray &edges)`
- `MStatus getConnectedFaces (MIntArray &faces)`
- `MStatus getConnectedEdges (MIntArray &edges)`
- `MStatus getConnectedVertices (MIntArray &vertices)`
- `bool isConnectedToFace (int index, MStatus *ReturnStatus=NULL)`
- `bool isConnectedToEdge (int index, MStatus *ReturnStatus=NULL)`
- `bool isConnectedToVertex (int index, MStatus *ReturnStatus=NULL)`
- `MStatus numConnectedFaces (int &faceCount) const`
- `MStatus numConnectedEdges (int &edgeCount) const`
- `bool onBoundary (MStatus *ReturnStatus=NULL)`
- `MStatus getArea (double &area, MSpace::Space space=MSpace::kObject)`
- `bool zeroArea (MStatus *ReturnStatus=NULL)`
- `MStatus getUVArea (double &area, const MString *uvSet=NULL)`
- `bool zeroUVArea (MStatus *ReturnStatus=NULL)`
- `bool zeroUVArea (const MString &uvSet, MStatus *ReturnStatus=NULL)`
- `bool isConvex (MStatus *ReturnStatus=NULL)`
- `bool isStarlike (MStatus *ReturnStatus=NULL)`
- `bool isLamina (MStatus *ReturnStatus=NULL)`
- `bool isHoled (MStatus *ReturnStatus=NULL)`
- `bool isPlanar (MStatus *ReturnStatus=NULL)`
- `bool isUVReversed (const MString *uvSet=NULL, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MItMeshVertex

Module `OpenMaya`. Source: [class_m_it_mesh_vertex.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_vertex.html).

**Public Member Functions**

- `MItMeshVertex (MObject &polyObject, MStatus *ReturnStatus=NULL)`
- `MItMeshVertex (const MDagPath &polyObject, MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `virtual ~MItMeshVertex ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus reset (MObject &polyObject)`
- `MStatus reset (const MDagPath &polyObject, MObject &component=MObject::kNullObj)`
- `int count (MStatus *ReturnStatus=NULL)`
- `int index (MStatus *ReturnStatus=NULL)`
- `MObject vertex (MStatus *ReturnStatus=NULL)`
- `MObject currentItem (MStatus *ReturnStatus=NULL)`
- `MPoint position (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL)`
- `MStatus setPosition (const MPoint &point, MSpace::Space space=MSpace::kObject)`
- `MStatus translateBy (const MVector &vector, MSpace::Space space=MSpace::kObject)`
- `MStatus getNormal (MVector &vector, MSpace::Space space=MSpace::kObject)`
- `MStatus getNormal (MVector &vector, int faceIndex, MSpace::Space space=MSpace::kObject)`
- `MStatus getNormals (MVectorArray &vectorArray, MSpace::Space space=MSpace::kObject)`
- `MStatus getNormalIndices (MIntArray &normalIndices) const`
- `MStatus numUVs (int &count, const MString *uvSet=NULL)`
- `MStatus setUV (float2 &uvPoint, const MString *uvSet=NULL)`
- `MStatus getUV (float2 &uvPoint, const MString *uvSet=NULL)`
- `MStatus setUV (int faceId, float2 &uvPoint, const MString *uvSet=NULL)`
- `MStatus getUV (int faceId, float2 &uvPoint, const MString *uvSet=NULL) const`
- `MStatus setUVs (MFloatArray &uArray, MFloatArray &vArray, MIntArray &faceIds, const MString *uvSet=NULL)`
- `MStatus getUVs (MFloatArray &uArray, MFloatArray &vArray, MIntArray &faceIds, const MString *uvSet=NULL) const`
- `MStatus getUVIndices (MIntArray &uvIndices, const MString *uvSet=NULL) const`
- `MStatus updateSurface ()`
- `MStatus geomChanged ()`
- `MStatus setIndex (int index, int &prevIndex)`
- `MStatus getConnectedFaces (MIntArray &faceList)`
- `MStatus getConnectedEdges (MIntArray &edgeList)`
- `MStatus getConnectedVertices (MIntArray &vertexList)`
- `MStatus numConnectedFaces (int &faceCount) const`
- `MStatus numConnectedEdges (int &edgeCount) const`
- `bool connectedToFace (int faceIndex, MStatus *ReturnStatus=NULL)`
- `bool connectedToEdge (int edgeIndex, MStatus *ReturnStatus=NULL)`
- `MStatus getOppositeVertex (int &vertexId, int edgeId)`
- `bool onBoundary (MStatus *ReturnStatus=NULL)`
- `bool hasColor (MStatus *ReturnStatus=NULL) const`
- `bool hasColor (int faceIndex, MStatus *ReturnStatus=NULL) const`
- `MStatus getColor (MColor &color, int faceIndex, const MString *colorSetName=NULL) const`
- `MStatus getColor (MColor &color, const MString *colorSetName=NULL) const`
- `MStatus getColors (MColorArray &colors, const MString *colorSetName=NULL) const`
- `MStatus getColorIndices (MIntArray &colorIndex, const MString *colorSetName=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MItSelectionList

Module `OpenMaya`. Source: [class_m_it_selection_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_selection_list.html).

**Public Types**

- `enum selItemType { , kDagSelectionItem, kAnimSelectionItem, kDNselectionItem, kPlugSelectionItem }`

**Public Member Functions**

- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `MStatus getDependNode (MObject &depNode)`
- `MStatus getDagPath (MDagPath &dagPath, MObject &component)`
- `MStatus getDagPath (MDagPath &dagPath)`
- `MStatus getPlug (MPlug &plug)`
- `MStatus getStrings (MStringArray &array)`
- `selItemType itemType (MStatus *ReturnStatus=NULL)`
- `MStatus setFilter (MFn::Type filter)`
- `bool hasComponents (MStatus *ReturnStatus=NULL) const`
- `MItSelectionList (const MSelectionList &list, MFn::Type=MFn::kInvalid, MStatus *ReturnStatus=NULL)`
- `virtual ~MItSelectionList ()`

**Static Public Member Functions**

- `static const char * className ()`

### MItSubdEdge

Module `OpenMaya`. Source: [class_m_it_subd_edge.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_subd_edge.html).

**Public Member Functions**

- `MItSubdEdge (MObject &subdObject, MStatus *ReturnStatus=NULL)`
- `virtual ~MItSubdEdge ()`
- `MStatus reset ()`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `bool isValid (MStatus *ReturnStatus=NULL) const`
- `int level (MStatus *ReturnStatus=NULL) const`
- `void setLevel (int level, MStatus *ReturnStatus=NULL)`
- `MUint64 index (MStatus *ReturnStatus=NULL)`
- `bool isSharp (MStatus *ReturnStatus=NULL)`
- `void setSharpness (bool sharp, MStatus *ReturnStatus=NULL)`
- `bool isBoundary (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MItSubdFace

Module `OpenMaya`. Source: [class_m_it_subd_face.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_subd_face.html).

**Public Member Functions**

- `MItSubdFace (MObject &subdObject, MStatus *ReturnStatus=NULL)`
- `virtual ~MItSubdFace ()`
- `MStatus reset ()`
- `MStatus next ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `bool isValid (MStatus *ReturnStatus=NULL) const`
- `unsigned int level (MStatus *ReturnStatus=NULL) const`
- `void setLevel (unsigned int level, MStatus *ReturnStatus=NULL)`
- `MUint64 index (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MItSubdVertex

Module `OpenMaya`. Source: [class_m_it_subd_vertex.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_subd_vertex.html).

**Public Member Functions**

- `MItSubdVertex (MObject &subdObject, MStatus *ReturnStatus=NULL)`
- `virtual ~MItSubdVertex ()`
- `bool isDone (MStatus *ReturnStatus=NULL)`
- `MStatus next ()`
- `MStatus reset ()`
- `int level (MStatus *ReturnStatus=NULL) const`
- `void setLevel (int level, MStatus *ReturnStatus=NULL)`
- `bool isValid (MStatus *ReturnStatus=NULL) const`
- `MUint64 index (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MItSurfaceCV

Module `OpenMaya`. Source: [class_m_it_surface_c_v.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_surface_c_v.html).

**Public Member Functions**

- `MItSurfaceCV (const MObject &surface, bool useURows=true, MStatus *ReturnStatus=NULL)`
- `MItSurfaceCV (const MDagPath &surface, const MObject &component=MObject::kNullObj, bool useURows=true, MStatus *ReturnStatus=NULL)`
- `virtual ~MItSurfaceCV ()`
- `bool isDone (MStatus *ReturnStatus=NULL) const`
- `bool isRowDone (MStatus *ReturnStatus=NULL) const`
- `MStatus next ()`
- `MStatus nextRow ()`
- `MStatus reset ()`
- `MStatus reset (const MObject &surface, bool useURows=true)`
- `MStatus reset (const MDagPath &surface, const MObject &component=MObject::kNullObj, bool useURows=true)`
- `MPoint position (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus setPosition (const MPoint &point, MSpace::Space space=MSpace::kObject)`
- `MStatus translateBy (const MVector &vector, MSpace::Space space=MSpace::kObject)`
- `int index (MStatus *ReturnStatus=NULL) const`
- `MStatus getIndex (int &indexU, int &indexV) const`
- `MObject cv (MStatus *ReturnStatus=NULL) const`
- `MObject currentItem (MStatus *ReturnStatus=NULL) const`
- `bool hasHistoryOnCreate (MStatus *ReturnStatus=NULL) const`
- `MStatus updateSurface ()`

**Static Public Member Functions**

- `static const char * className ()`

### MIteratorType

Module `OpenMaya`. Source: [class_m_iterator_type.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_iterator_type.html).

**Public Types**

- `enum objFilterType { kMObject, kMDagPathObject, kMPlugObject }`

**Public Member Functions**

- `MIteratorType (MStatus *ReturnStatus=NULL)`
- `MIteratorType (const MIteratorType &, MStatus *ReturnStatus=NULL)`
- `~MIteratorType ()`
- `void setFilterType (MFn::Type type, MStatus *ReturnStatus=NULL)`
- `void setFilterList (MIntArray &listOfFilters, MStatus *ReturnStatus=NULL)`
- `void setObjectType (objFilterType objType, MStatus *ReturnStatus=NULL)`
- `MFn::Type getFilterType (MStatus *ReturnStatus=NULL)`
- `MStatus getFilterList (MIntArray &listOfFilters)`
- `MIteratorType::objFilterType getObjectType (MStatus *ReturnStatus=NULL)`
- `bool filterListEnabled ()`

### MLockMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_lock_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_lock_message.html).

**Public Types**

- `enum LockPlugEvent { kInvalidPlug = 0, kPlugLockAttr, kPlugUnlockAttr, kPlugAttrValChange, kPlugRemoveAttr, kPlugRenameAttr, kPlugConnect, kPlugDisconnect, kLastPlug }`
- `enum LockDAGEvent { kInvalidDAG = 0, kGroup, kUnGroup, kReparent, kChildReorder, kCreateNodeInstance, kCreateChildInstance, kCreateParentInstance, kLastDAG }`
- `enum LockEvent { kInvalid = 0, kRename, kDelete, kLockNode, kUnlockNode, kAddAttr, kRemoveAttr, kRenameAttr, kUnlockAttr, kLockAttr, kLast }`
- `typedef void(* MLockEventFunction) (MObject &node, MObject &aux, void *clientData, MLockMessage::LockEvent eventType, bool &decision)`
- `typedef void(* MLockDagEventFunction) (MDagPath &dagPath, MDagPath &otherPath, void *clientData, MLockMessage::LockDAGEvent eventType, bool &decision)`
- `typedef void(* MLockPlugEventFunction) (MPlug &plug, MPlug &otherPlug, void *clientData, MLockMessage::LockPlugEvent eventType, bool &decision)`

**Static Public Member Functions**

- `static MCallbackId setNodeLockQueryCallback (MObject &node, MLockMessage::MLockEventFunction func, void *clientData=NULL, MStatus *status=NULL)`
- `static MCallbackId setNodeLockDAGQueryCallback (MDagPath &path, MLockMessage::MLockDagEventFunction func, void *clientData=NULL, MStatus *status=NULL)`
- `static MCallbackId setPlugLockQueryCallback (MPlug &plug, MLockMessage::MLockPlugEventFunction func, void *clientData=NULL, MStatus *status=NULL)`
- `static MCallbackId setPlugLockQueryCallback (MObject &node, MLockMessage::MLockPlugEventFunction func, void *clientData=NULL, MStatus *status=NULL)`
- `static const char * className ()`

### MMatrix

Module `OpenMaya`. Source: [class_m_matrix.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_matrix.html).

**Public Member Functions**

- `MMatrix ()`
- `MMatrix (const MMatrix &src)`
- `MMatrix (const double m[4][4])`
- `MMatrix (const float m[4][4])`
- `~MMatrix ()`
- `MMatrix & operator= (const MMatrix &)`
- `double operator() (unsigned int row, unsigned int col) const`
- `const double * operator[] (unsigned int row) const`
- `MStatus get (double dest[4][4]) const`
- `MStatus get (float dest[4][4]) const`
- `MMatrix transpose () const`
- `MMatrix & setToIdentity ()`
- `MMatrix & setToProduct (const MMatrix &left, const MMatrix &right)`
- `MMatrix & operator+= (const MMatrix &right)`
- `MMatrix operator+ (const MMatrix &right) const`
- `MMatrix & operator-= (const MMatrix &right)`
- `MMatrix operator- (const MMatrix &right) const`
- `MMatrix & operator*= (const MMatrix &right)`
- `MMatrix operator* (const MMatrix &right) const`
- `MMatrix & operator*= (double)`
- `MMatrix operator* (double) const`
- `bool operator== (const MMatrix &other) const`
- `bool operator!= (const MMatrix &other) const`
- `MMatrix inverse () const`
- `MMatrix adjoint () const`
- `MMatrix homogenize () const`
- `double det4x4 () const`
- `double det3x3 () const`
- `bool isEquivalent (const MMatrix &other, double tolerance=MMatrix_kTol) const`
- `bool isSingular () const`
- `double & operator() (unsigned int row, unsigned int col)`
- `double * operator[] (unsigned int row)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `double matrix [4][4]`

**Static Public Attributes**

- `static const MMatrix identity`

**Friends**

- `OPENMAYA_EXPORT MMatrix operator* (double, const MMatrix &right)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MMatrix &m)`

### MMatrixArray

Module `OpenMaya`. Source: [class_m_matrix_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_matrix_array.html).

**Public Member Functions**

- `MMatrixArray ()`
- `MMatrixArray (const MMatrixArray &other)`
- `MMatrixArray (unsigned initialSize)`
- `~MMatrixArray ()`
- `const MMatrix & operator[] (unsigned index) const`
- `MMatrixArray & operator= (const MMatrixArray &other)`
- `MStatus set (const MMatrix &element, unsigned index)`
- `MStatus setLength (unsigned length)`
- `unsigned length () const`
- `MStatus remove (unsigned index)`
- `MStatus insert (const MMatrix &element, unsigned index)`
- `MStatus append (const MMatrix &element)`
- `MStatus copy (const MMatrixArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned newIncrement)`
- `unsigned sizeIncrement () const`
- `MMatrix & operator[] (unsigned index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MMatrixArray &array)`

### MMeshIntersector

Module `OpenMaya`. Source: [class_m_mesh_intersector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_mesh_intersector.html).

**Public Member Functions**

- `MMeshIntersector (void)`
- `virtual ~MMeshIntersector (void)`
- `MStatus create (MObject &meshObject, const MMatrix &matrix=MMatrix::identity, const MIntArray *facesOfInterest=nullptr)`
- `bool isCreated (void) const`
- `MStatus getClosestPoint (const MPoint &point, MPointOnMesh &meshPoint, double maxDistance=DBL_MAX) const`

**Static Public Member Functions**

- `static const char * className ()`

### MMeshSmoothOptions

Module `OpenMaya`. Source: [class_m_mesh_smooth_options.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_mesh_smooth_options.html).

**Public Types**

- `enum SubdivisionType { kInvalidSubdivision = -1, kCatmullClark = 0, kOpenSubdivCatmullClarkUniform = 2, kOpenSubdivCatmullClarkAdaptive, kLastSubdivision }`
- `enum BoundaryRule { kInvalid = -1, kLegacy = 0, kCreaseAll, kCreaseEdge, kLast }`
- `enum OpenSubdivVertexBoundary { kInvalidBoundary = -1, kNone = 0, kSharpEdgesAndCorners, kSharpEdges, kAlwaysSharp, kLastBoundary }`
- `enum OpenSubdivCreaseMethod { kInvalidCreaseMethod = -1, kNormal = 0, kChaikin, kLastCreaseMethod }`

**Public Member Functions**

- `MMeshSmoothOptions (void)`
- `virtual ~MMeshSmoothOptions (void)`
- `void setDivisions (int numDivisions, MStatus *ReturnStatus=NULL)`
- `int divisions (MStatus *ReturnStatus=NULL) const`
- `void setSmoothness (float smoothness, MStatus *ReturnStatus=NULL)`
- `float smoothness (MStatus *ReturnStatus=NULL) const`
- `void setSmoothUVs (bool smoothUVs, MStatus *ReturnStatus=NULL)`
- `bool smoothUVs (MStatus *ReturnStatus=NULL) const`
- `void setPropEdgeHardness (bool propEdgeHardness, MStatus *ReturnStatus=NULL)`
- `bool propEdgeHardness (MStatus *ReturnStatus=NULL) const`
- `void setKeepBorderEdge (bool keepBorderEdge, MStatus *ReturnStatus=NULL)`
- `bool keepBorderEdge (MStatus *ReturnStatus=NULL) const`
- `void setKeepHardEdge (bool keepHardEdge, MStatus *ReturnStatus=NULL)`
- `bool keepHardEdge (MStatus *ReturnStatus=NULL) const`
- `void setBoundaryRule (BoundaryRule rule, MStatus *ReturnStatus=NULL)`
- `BoundaryRule boundaryRule (MStatus *ReturnStatus=NULL) const`
- `void setOpenSubdivVertexBoundary (OpenSubdivVertexBoundary rule, MStatus *ReturnStatus=NULL)`
- `OpenSubdivVertexBoundary openSubdivVertexBoundary (MStatus *ReturnStatus=NULL) const`
- `void setOpenSubdivFaceVaryingBoundary (OpenSubdivVertexBoundary smoothing, MStatus *ReturnStatus=NULL)`
- `OpenSubdivVertexBoundary openSubdivFaceVaryingBoundary (MStatus *ReturnStatus=NULL) const`
- `void setOpenSubdivSmoothTriangles (bool smoothTriangles, MStatus *ReturnStatus=NULL)`
- `bool openSubdivSmoothTriangles (MStatus *ReturnStatus=NULL)`
- `void setOpenSubdivCreaseMethod (OpenSubdivCreaseMethod method, MStatus *ReturnStatus=NULL)`
- `OpenSubdivCreaseMethod openSubdivCreaseMethod (MStatus *ReturnStatus=NULL) const`
- `void setSubdivisionType (SubdivisionType subdivision, MStatus *ReturnStatus=NULL)`
- `SubdivisionType subdivisionType (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MFnMesh`

### MMessage

Module `OpenMaya`. Source: [class_m_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_message.html).

**Public Types**

- `enum Action { kDefaultAction, kDoNotDoAction, kDoAction }`
- `typedef void(* MBasicFunction) (void *clientData)`
- `typedef void(* MElapsedTimeFunction) (float elapsedTime, float lastTime, void *clientData)`
- `typedef void(* MCheckFunction) (bool *retCode, void *clientData)`
- `typedef void(* MCheckFileFunction) (bool *retCode, MFileObject &file, void *clientData)`
- `typedef void(* MCheckPlugFunction) (bool *retCode, MPlug &plug, void *clientData)`
- `typedef void(* MComponentFunction) (MUintArray componentIds[], unsigned int count, void *clientData)`
- `typedef void(* MNodeFunction) (MObject &node, void *clientData)`
- `typedef void(* MStringFunction) (const MString &str, void *clientData)`
- `typedef void(* MTwoStringFunction) (const MString &str1, const MString &str2, void *clientData)`
- `typedef void(* MThreeStringFunction) (const MString &str1, const MString &str2, const MString &str3, void *clientData)`
- `typedef void(* MStringIntBoolIntFunction) (const MString &str, unsigned int index, bool flag, unsigned int type, void *clientData)`
- `typedef void(* MStringIndexFunction) (const MString &, unsigned int index, void *clientData)`
- `typedef void(* MNodeStringBoolFunction) (MObject &node, const MString &, bool, void *clientData)`
- `typedef void(* MStateFunction) (bool state, void *clientData)`
- `typedef void(* MTimeFunction) (MTime &time, void *clientData)`
- `typedef void(* MPlugFunction) (MPlug &srcPlug, MPlug &destPlug, bool made, void *clientData)`
- `typedef void(* MNodePlugFunction) (MObject &node, MPlug &plug, void *clientData)`
- `typedef void(* MNodeStringFunction) (MObject &node, const MString &str, void *clientData)`
- `typedef void(* MParentChildFunction) (MDagPath &child, MDagPath &parent, void *clientData)`
- `typedef void(* MModifierFunction) (MDGModifier &modifier, void *clientData)`
- `typedef void(* MStringArrayFunction) (const MStringArray &strs, void *clientData)`
- `typedef void(* MNodeModifierFunction) (MObject &node, MDGModifier &modifier, void *clientData)`
- `typedef void(* MObjArray) (MObjectArray &objects, void *clientData)`
- `typedef void(* MNodeObjArray) (MObject &node, MObjectArray &objects, void *clientData)`
- `typedef void(* MStringNode) (const MString &str, MObject &node, void *clientData)`
- `typedef void(* MCameraLayerFunction) (MObject &cameraSetNode, unsigned int multiIndex, bool added, void *clientData)`
- `typedef void(* MCameraLayerCameraFunction) (MObject &cameraSetNode, unsigned int multiIndex, MObject &oldCamera, MObject &newCamera, void *clientData)`
- `typedef void(* MConnFailFunction) (MPlug &srcPlug, MPlug &destPlug, const MString &srcPlugName, const MString &dstPlugName, void *clientData)`
- `typedef void(* MPlugsDGModFunction) (MPlugArray &plugs, MDGModifier &modifier, void *clientData)`
- `typedef void(* MNodeUuidFunction) (MObject &node, const MUuid &uuid, void *clientData)`
- `typedef Action(* MCheckNodeUuidFunction) (bool doAction, MObject &node, MUuid &uuid, void *clientData)`
- `typedef void(* MObjectFileFunction) (const MObject &object, const MFileObject &file, void *clientData)`
- `typedef void(* MCheckObjectFileFunction) (bool *retCode, const MObject &referenceNode, MFileObject &file, void *clientData)`
- `typedef void(* MRenderTileFunction) (int originX, int originY, int tileMaxX, int tileMaxY, const void *tileData)`

**Static Public Member Functions**

- `static MStatus removeCallback (MCallbackId id)`
- `static MStatus removeCallbacks (MCallbackIdArray &ids)`
- `static MCallbackId currentCallbackId (MStatus *ReturnStatus=NULL)`
- `static MStatus nodeCallbacks (MObject &node, MCallbackIdArray &ids)`
- `static void setRegisteringCallableScript ()`
- `static bool registeringCallableScript ()`
- `static void stopRegisteringCallableScript ()`
- `static const char * className ()`
- `static MStatus removeCallbacks (MIntArray &ids)`
- `static MStatus nodeCallbacks (MObject &node, MIntArray &ids)`

**Friends**

- `class MRenderCallback`

### MModelMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_model_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_model_message.html).

**Public Types**

- `enum Message { kActiveListModified }`

**Static Public Member Functions**

- `static MCallbackId addCallback (Message, MMessage::MBasicFunction, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addBeforeDuplicateCallback (MMessage::MBasicFunction, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAfterDuplicateCallback (MMessage::MBasicFunction, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeAddedToModelCallback (const MObject &dagNode, MMessage::MNodeFunction, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeRemovedFromModelCallback (const MObject &dagNode, MMessage::MNodeFunction, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MMutexLock

Module `OpenMaya`. Source: [class_m_mutex_lock.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_mutex_lock.html).

**Public Member Functions**

- `MMutexLock ()`
- `~MMutexLock ()`
- `void lock ()`
- `void unlock ()`
- `bool tryLock ()`

### MNamespace

Module `OpenMaya`. Source: [class_m_namespace.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_namespace.html).

**Static Public Member Functions**

- `static MStatus addNamespace (const MString &name, const MString *parent=NULL)`
- `static MString validateName (const MString &name, MStatus *ReturnStatus=NULL)`
- `static MString currentNamespace (MStatus *ReturnStatus=NULL)`
- `static MStatus setCurrentNamespace (const MString &name)`
- `static MStringArray getNamespaces (const bool recurse=false, MStatus *ReturnStatus=NULL)`
- `static MStringArray getNamespaces (const MString &parentNamespace, const bool recurse=false, MStatus *ReturnStatus=NULL)`
- `static bool namespaceExists (const MString &name, MStatus *ReturnStatus=NULL)`
- `static MString parentNamespace (MStatus *ReturnStatus=NULL)`
- `static MStatus removeNamespace (const MString &name, bool removeContents=false)`
- `static MStatus renameNamespace (const MString &oldName, const MString &newName, const MString *parent=NULL)`
- `static MObjectArray getNamespaceObjects (const MString &parentNamespace, const bool recurse=false, MStatus *ReturnStatus=NULL)`
- `static MStatus moveNamespace (const MString &src, const MString &dst, const bool force=false)`
- `static MString rootNamespace (MStatus *ReturnStatus=NULL)`
- `static bool relativeNames (MStatus *ReturnStatus=NULL)`
- `static MStatus setRelativeNames (bool newState)`
- `static MString getNamespaceFromName (const MString &fullName, MStatus *ReturnStatus=NULL)`
- `static MString stripNamespaceFromName (const MString &fullName, MStatus *ReturnStatus=NULL)`
- `static MString makeNamepathAbsolute (const MString &, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MNodeClass

Module `OpenMaya`. Source: [class_m_node_class.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_node_class.html).

**Public Member Functions**

- `MNodeClass (const MString &nodeClassName)`
- `MNodeClass (const MTypeId &nodeTypeId)`
- `MTypeId typeId () const`
- `MString typeName () const`
- `MString classification () const`
- `MStatus addToClassification (const MString &str)`
- `MStatus removeFromClassification (const MString &str)`
- `MString pluginName (MStatus *ReturnStatus=NULL) const`
- `MStatus addExtensionAttribute (const MObject &attr) const`
- `MStatus removeExtensionAttribute (const MObject &attr) const`
- `MStatus removeExtensionAttributeIfUnset (const MObject &attr) const`
- `MStatus getAttributes (MObjectArray &attr) const`
- `unsigned int attributeCount (MStatus *ReturnStatus=NULL) const`
- `MObject attribute (unsigned int index, MStatus *ReturnStatus=NULL) const`
- `MObject attribute (const MString &attrName, MStatus *ReturnStatus=NULL) const`
- `bool hasAttribute (const MString &attrName, MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MNodeMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_node_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_node_message.html).

**Public Types**

- `enum AttributeMessage { kConnectionMade = 0x01, kConnectionBroken = 0x02, kAttributeEval = 0x04, kAttributeSet = 0x08, kAttributeLocked = 0x10, kAttributeUnlocked = 0x20, kAttributeAdded = 0x40, kAttributeRemoved = 0x80, kAttributeRenamed = 0x100, kAttributeKeyable = 0x200, kAttributeUnkeyable = 0x400, kIncomingDirection = 0x800, kAttributeArrayAdded = 0x1000, kAttributeArrayRemoved = 0x2000, kOtherPlugSet = 0x4000, kLast = 0x8000 }`
- `enum KeyableChangeMsg { kKeyChangeInvalid = 0, kMakeKeyable, kMakeUnkeyable, kKeyChangeLast }`
- `typedef void(* MAttr2PlugFunction) (MNodeMessage::AttributeMessage msg, MPlug &plug, MPlug &otherPlug, void *clientData)`
- `typedef void(* MAttrPlugFunction) (MNodeMessage::AttributeMessage msg, MPlug &plug, void *clientData)`
- `typedef void(* MKeyableFunction) (MPlug &plug, void *clientData, MNodeMessage::KeyableChangeMsg msg, bool &decision)`

**Static Public Member Functions**

- `static MCallbackId addAttributeChangedCallback (MObject &node, MNodeMessage::MAttr2PlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addAttributeAddedOrRemovedCallback (MObject &node, MNodeMessage::MAttrPlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeDirtyCallback (MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeDirtyPlugCallback (MObject &node, MMessage::MNodePlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNameChangedCallback (MObject &node, MMessage::MNodeStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addUuidChangedCallback (MObject &node, MMessage::MNodeUuidFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeAboutToDeleteCallback (MObject &node, MMessage::MNodeModifierFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodePreRemovalCallback (MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeDestroyedCallback (MObject &node, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addKeyableChangeOverride (MPlug &plug, MNodeMessage::MKeyableFunction func, void *clientData=NULL, MStatus *status=NULL)`
- `static const char * className ()`
- `static MCallbackId addNodeDirtyCallback (MObject &node, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeDirtyCallback (MObject &node, MMessage::MNodePlugFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNameChangedCallback (MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNodeAboutToDeleteCallback (MObject &node, MMessage::MModifierFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`

### MNurbsIntersector

Module `OpenMaya`. Source: [class_m_nurbs_intersector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_nurbs_intersector.html).

**Public Member Functions**

- `MNurbsIntersector (void)`
- `virtual ~MNurbsIntersector (void)`
- `MStatus create (MObject &nurbsObject, const MMatrix &matrix=MMatrix::identity)`
- `bool isCreated (void) const`
- `MStatus getClosestPoint (const MPoint &point, MPointOnNurbs &nurbsPoint, double maxDistance=DBL_MAX) const`
- `MStatus getIntersects (const MPoint &rayStartingPoint, const MVector &alongThisDirection, MDoubleArray &uArray, MDoubleArray &vArray, MPointArray &points, double tolerance) const`
- `MStatus getIntersect (const MPoint &rayStartingPoint, const MVector &alongThisDirection, double &u, double &v, MPoint &pntOfIntersection, double tolerance) const`

**Static Public Member Functions**

- `static const char * className ()`

### MObject

Module `OpenMaya`. Source: [class_m_object.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object.html).

**Public Member Functions**

- `MObject ()`
- `MObject (const MObject &other)`
- `~MObject ()`
- `bool hasFn (MFn::Type fs) const`
- `bool isNull () const`
- `MFn::Type apiType () const`
- `const char * apiTypeStr () const`
- `bool operator== (const MObject &) const`
- `bool operator!= (const MObject &) const`
- `MObject & operator= (const MObject &)`

**Static Public Attributes**

- `static MObject kNullObj`

**Friends**

- `class MObjectHandle`

### MObjectArray

Module `OpenMaya`. Source: [class_m_object_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object_array.html).

**Public Member Functions**

- `MObjectArray ()`
- `MObjectArray (const MObjectArray &other)`
- `MObjectArray (unsigned int initialSize, const MObject &initialValue=MObject::kNullObj)`
- `~MObjectArray ()`
- `MObjectArray & operator= (const MObjectArray &)`
- `const MObject & operator[] (unsigned int index) const`
- `MStatus set (const MObject &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MObject &element, unsigned int index)`
- `MStatus append (const MObject &element)`
- `MStatus copy (const MObjectArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MObjectArray (const MObject src[], unsigned int count)`
- `MStatus get (MObject array[]) const`
- `MObject & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

### MObjectHandle

Module `OpenMaya`. Source: [class_m_object_handle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object_handle.html).

**Public Member Functions**

- `MObjectHandle ()`
- `MObjectHandle (const MObject &object)`
- `MObjectHandle (const MObjectHandle &handle)`
- `~MObjectHandle ()`
- `MObject object () const`
- `const MObject & objectRef () const`
- `unsigned int hashCode () const`
- `bool isValid () const`
- `bool isAlive () const`
- `bool operator== (const MObject &object) const`
- `bool operator== (const MObjectHandle &object) const`
- `bool operator!= (const MObject &object) const`
- `bool operator!= (const MObjectHandle &object) const`
- `MObjectHandle & operator= (const MObject &object)`
- `MObjectHandle & operator= (const MObjectHandle &other)`

**Static Public Member Functions**

- `static unsigned int objectHashCode (const MObject &)`

### MObjectSetMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_object_set_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object_set_message.html).

**Static Public Member Functions**

- `static MCallbackId addSetMembersModifiedCallback (MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MOpenCLBuffer

Module `OpenMaya`. Source: [class_m_open_c_l_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_open_c_l_buffer.html).

**Public Member Functions**

- `MOpenCLBuffer (cl_context gpuContext, cl_mem_flags flags)`
- `MOpenCLBuffer (cl_context gpuContext, cl_mem_flags flags, size_t size, void *host_ptr, cl_int *err)`
- `MOpenCLBuffer ()`
- `MOpenCLBuffer (MOpenCLBuffer &&)`
- `~MOpenCLBuffer ()`
- `MOpenCLBuffer & operator= (MOpenCLBuffer &&)`
- `void reset ()`
- `cl_int uploadDataToGPU (cl_command_queue gpuQueue, cl_bool blocking_write, size_t offset, size_t size, void *host_ptr, cl_uint numEvents, const cl_event *events)`
- `cl_int ensureBufferSize (size_t size)`
- `MAutoCLMem buffer ()`
- `const MAutoCLMem buffer () const`
- `MAutoCLEvent bufferReadyEvent ()`

### MOpenCLInfo

Module `OpenMaya`. Source: [class_m_open_c_l_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_open_c_l_info.html).

**Static Public Member Functions**

- `static cl_context getOpenCLContext ()`
- `static cl_command_queue getMayaDefaultOpenCLCommandQueue ()`
- `static cl_command_queue createOpenCLCommandQueue ()`
- `static cl_device_id getOpenCLDeviceId ()`
- `static MAutoCLKernel getOpenCLKernel (const MString &sourceFilePath, const MString &kernelName)`
- `static MAutoCLKernel getOpenCLKernelFromString (const MString &programSource, const MString &programName, const MString &kernelName)`
- `static bool releaseOpenCLKernel (const MAutoCLKernel &kernel)`
- `static void checkCLErrorStatus (cl_int err)`
- `static void appendMessage (MStringArray *messages, const char *format,...)`
- `static void appendMessage (MStringArray *messages, const MStringResourceId *formatResourceId,...)`
- `static bool supportsDoublePrecision ()`

### MPaintMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_paint_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_paint_message.html).

**Public Types**

- `typedef void(* MPathObjectPlugColorsFunction) (MDagPath &path, MObject &object, MPlug &plug, MColorArray &colors, void *clientData)`

**Static Public Member Functions**

- `static MCallbackId addVertexColorCallback (MPathObjectPlugColorsFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MParentingEdit

Module `OpenMaya`. Inherits `MEdit`. Source: [class_m_parenting_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_parenting_edit.html).

**Public Member Functions**

- `MObject parentedObject (MStatus *ReturnStatus=NULL) const`
- `MObject parent (MStatus *ReturnStatus=NULL) const`
- `MString parentedObjectName (MStatus *ReturnStatus=NULL) const`
- `MString parentName (MStatus *ReturnStatus=NULL) const`
- `EditType editType (MStatus *ReturnStatus=NULL) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MPlane

Module `OpenMaya`. Source: [class_m_plane.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_plane.html).

**Public Member Functions**

- `MPlane ()`
- `MPlane (const MPlane &)`
- `MVector normal () const`
- `double distance () const`
- `void setPlane (double a, double b, double c, double d)`
- `void setPlane (const MVector &n, double d)`
- `double distance (const MVector &) const`
- `double directedDistance (const MVector &) const`

**Static Public Member Functions**

- `static const char * className ()`

### MPlug

Module `OpenMaya`. Source: [class_m_plug.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_plug.html).

**Public Types**

- `enum MValueSelector { kAll, kNonDefault, kChanged, kLastAttrSelector }`
- `enum FreeToChangeState { kFreeToChange, kNotFreeToChange, kChildrenNotFreeToChange }`

**Public Member Functions**

- `MPlug ()`
- `MPlug (const MPlug &in)`
- `MPlug (const MObject &node, const MObject &attribute)`
- `virtual ~MPlug ()`
- `MStatus setAttribute (MObject &attribute)`
- `MObject attribute (MStatus *ReturnStatus=nullptr) const`
- `MObject node (MStatus *ReturnStatus=nullptr) const`
- `MString name (MStatus *ReturnStatus=nullptr) const`
- `MString partialName (bool includeNodeName=false, bool includeNonMandatoryIndices=false, bool includeInstancedIndices=false, bool useAlias=false, bool useFullAttributePath=false, bool useLongNames=false, MStatus *ReturnStatus=nullptr) const`
- `MStatus getSetAttrCmds (MStringArray &cmds, MValueSelector valueSelector=kAll, bool useLongNames=false)`
- `bool isNetworked (MStatus *ReturnStatus=nullptr) const`
- `bool isArray (MStatus *ReturnStatus=nullptr) const`
- `bool isElement (MStatus *ReturnStatus=nullptr) const`
- `bool isCompound (MStatus *ReturnStatus=nullptr) const`
- `bool isChild (MStatus *ReturnStatus=nullptr) const`
- `bool isProcedural (MStatus *ReturnStatus=nullptr) const`
- `unsigned int logicalIndex (MStatus *ReturnStatus=nullptr) const`
- `MStatus selectAncestorLogicalIndex (unsigned int index, const MObject &attribute=MObject::kNullObj)`
- `unsigned int getExistingArrayAttributeIndices (MIntArray &indices, MStatus *ReturnStatus=nullptr)`
- `unsigned int numElements (MStatus *ReturnStatus=nullptr) const`
- `unsigned int evaluateNumElements (MStatus *ReturnStatus=nullptr)`
- `unsigned int numChildren (MStatus *ReturnStatus=nullptr) const`
- `unsigned int numConnectedElements (MStatus *ReturnStatus=nullptr) const`
- `unsigned int numConnectedChildren (MStatus *ReturnStatus=nullptr) const`
- `MPlug child (MObject &attr, MStatus *ReturnStatus=nullptr) const`
- `MPlug child (unsigned int index, MStatus *ReturnStatus=nullptr) const`
- `MPlug parent (MStatus *ReturnStatus=nullptr) const`
- `MPlug array (MStatus *ReturnStatus=nullptr) const`
- `MPlug elementByLogicalIndex (unsigned int logicalIndex, MStatus *ReturnStatus=nullptr) const`
- `MPlug elementByPhysicalIndex (unsigned int physicalIndex, MStatus *ReturnStatus=nullptr) const`
- `MPlug connectionByPhysicalIndex (unsigned int physicalIndex, MStatus *ReturnStatus=nullptr) const`
- `bool connectedTo (MPlugArray &array, bool asDst, bool asSrc, MStatus *ReturnStatus=nullptr) const`
- `bool isConnected (MStatus *ReturnStatus=nullptr) const`
- `MPlug source (MStatus *ReturnStatus=nullptr) const`
- `bool destinations (MPlugArray &theDestinations, MStatus *ReturnStatus=nullptr) const`
- `MPlug sourceWithConversion (MStatus *ReturnStatus=nullptr) const`
- `bool destinationsWithConversions (MPlugArray &theDestinations, MStatus *ReturnStatus=nullptr) const`
- `bool isKeyable (MStatus *ReturnStatus=nullptr) const`
- `MStatus setKeyable (bool keyable)`
- `bool isLocked (MStatus *ReturnStatus=nullptr) const`
- `MStatus setLocked (bool locked)`
- `bool isChannelBoxFlagSet (MStatus *ReturnStatus=nullptr) const`
- `MStatus setChannelBox (bool channelBox)`
- `bool isCachingFlagSet (MStatus *ReturnStatus=nullptr) const`
- `MStatus setCaching (bool caching)`
- `bool isNull (MStatus *ReturnStatus=nullptr) const`
- `MString info (MStatus *ReturnStatus=nullptr) const`
- `bool isFromReferencedFile (MStatus *ReturnStatus=nullptr) const`
- `bool isDynamic (MStatus *ReturnStatus=nullptr) const`
- `bool isIgnoredWhenRendering (MStatus *ReturnStatus=nullptr) const`
- `bool isDefaultValue (bool forceEval=true, MStatus *ReturnStatus=nullptr) const`
- `MPlug::FreeToChangeState isFreeToChange (bool checkParents=true, bool checkChildren=true, MStatus *ReturnStatus=nullptr) const`
- `MDataHandle constructHandle (MDataBlock &) const`
- `void destructHandle (MDataHandle &) const`
- `MObject asMObject (MStatus *ReturnStatus=nullptr) const`
- `MDataHandle asMDataHandle (MStatus *ReturnStatus=nullptr) const`
- `double asDouble (MStatus *ReturnStatus=nullptr) const`
- `float asFloat (MStatus *ReturnStatus=nullptr) const`
- `MInt64 asInt64 (MStatus *ReturnStatus=nullptr) const`
- `int asInt (MStatus *ReturnStatus=nullptr) const`
- `short asShort (MStatus *ReturnStatus=nullptr) const`
- `bool asBool (MStatus *ReturnStatus=nullptr) const`
- `MDistance asMDistance (MStatus *ReturnStatus=nullptr) const`
- `MAngle asMAngle (MStatus *ReturnStatus=nullptr) const`
- `MTime asMTime (MStatus *ReturnStatus=nullptr) const`
- `char asChar (MStatus *ReturnStatus=nullptr) const`
- `MString asString (MStatus *ReturnStatus=nullptr) const`
- `MStatus setMObject (const MObject &)`
- `MStatus setMPxData (const MPxData *)`
- `MStatus setMDataHandle (const MDataHandle &)`
- `MStatus setDouble (double)`
- `MStatus setFloat (float)`
- `MStatus setInt64 (MInt64)`
- `MStatus setInt (int)`
- `MStatus setShort (short)`
- `MStatus setBool (bool)`
- `MStatus setMDistance (const MDistance &)`
- `MStatus setMAngle (const MAngle &)`
- `MStatus setMTime (const MTime &)`
- `MStatus setChar (char)`
- `MStatus setString (const MString &)`
- `MPlug & operator= (const MPlug &other)`
- `MPlug operator[] (unsigned int physicalIndex) const`
- `bool operator== (const MPlug &other) const`
- `bool operator== (const MObject &other) const`
- `bool operator!= (const MPlug &other) const`
- `bool operator!= (const MObject &other) const`
- `MStatus setNumElements (unsigned int)`
- `bool isSource (MStatus *ReturnStatus=nullptr) const`
- `bool isDestination (MStatus *ReturnStatus=nullptr) const`
- `MStatus getValue (MObject &) const`
- `MStatus getValue (MDataHandle &) const`
- `MStatus getValue (double &) const`
- `MStatus getValue (float &) const`
- `MStatus getValue (MInt64 &) const`
- `MStatus getValue (int &) const`
- `MStatus getValue (short &) const`
- `MStatus getValue (bool &) const`
- `MStatus getValue (MDistance &) const`
- `MStatus getValue (MAngle &) const`
- `MStatus getValue (MTime &) const`
- `MStatus getValue (char &) const`
- `MStatus getValue (MString &) const`
- `MStatus setValue (const MObject &)`
- `MStatus setValue (const MPxData *)`
- `MStatus setValue (const MDataHandle &)`
- `MStatus setValue (double)`
- `MStatus setValue (float)`
- `MStatus setValue (MInt64)`
- `MStatus setValue (int)`
- `MStatus setValue (short)`
- `MStatus setValue (bool)`
- `MStatus setValue (const MDistance &)`
- `MStatus setValue (const MAngle &)`
- `MStatus setValue (const MTime &)`
- `MStatus setValue (char)`
- `MStatus setValue (const MString &)`
- `MStatus setValue (const char *)`
- `MPlug operator[] (MObject &attr) const`
- `operator MObject () const`
- `bool operator! () const`
- `MObject asMObject (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MDataHandle asMDataHandle (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `double asDouble (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `float asFloat (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MInt64 asInt64 (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `int asInt (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `short asShort (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `bool asBool (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MDistance asMDistance (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MAngle asMAngle (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MTime asMTime (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `char asChar (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MString asString (const MDGContext &, MStatus *ReturnStatus=nullptr) const`
- `MStatus getValue (MObject &, MDGContext &) const`
- `MStatus getValue (MDataHandle &, MDGContext &) const`
- `MStatus getValue (double &, MDGContext &) const`
- `MStatus getValue (float &, MDGContext &) const`
- `MStatus getValue (MInt64 &, MDGContext &) const`
- `MStatus getValue (int &, MDGContext &) const`
- `MStatus getValue (short &, MDGContext &) const`
- `MStatus getValue (bool &, MDGContext &) const`
- `MStatus getValue (MDistance &, MDGContext &) const`
- `MStatus getValue (MAngle &, MDGContext &) const`
- `MStatus getValue (MTime &, MDGContext &) const`
- `MStatus getValue (char &, MDGContext &) const`
- `MStatus getValue (MString &, MDGContext &) const`

**Static Public Member Functions**

- `static const char * className ()`

### MPlugArray

Module `OpenMaya`. Source: [class_m_plug_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_plug_array.html).

**Public Member Functions**

- `MPlugArray ()`
- `MPlugArray (const MPlugArray &other)`
- `MPlugArray (unsigned int initializeSize, const MPlug &initialValue)`
- `~MPlugArray ()`
- `const MPlug & operator[] (unsigned int index) const`
- `MPlugArray & operator= (const MPlugArray &other)`
- `MStatus set (const MPlug &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MPlug &element, unsigned int index)`
- `MStatus append (const MPlug &element)`
- `MStatus copy (const MPlugArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MPlugArray (const MPlug src[], unsigned int count)`
- `MStatus get (MPlug dest[]) const`
- `MPlug & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

### MPluginCallableInfo

Module `OpenMaya`. Source: [class_m_plugin_callable_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_plugin_callable_info.html).

### MPoint

Module `OpenMaya`. Source: [class_m_point.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_point.html).

**Public Member Functions**

- `MPoint ()`
- `MPoint (const MPoint &srcpt)`
- `MPoint (const MFloatPoint &srcpt)`
- `MPoint (const MVector &src)`
- `MPoint (const MFloatVector &src)`
- `MPoint (double xx, double yy, double zz=0.0, double ww=1.0)`
- `MPoint (const double d[4])`
- `MPoint (const float d[4])`
- `~MPoint ()`
- `MStatus get (double[4]) const`
- `MStatus get (float[4]) const`
- `double operator[] (unsigned int i) const`
- `double operator() (unsigned int i) const`
- `MPoint & operator= (const MPoint &src)`
- `MVector operator- (const MPoint &other) const`
- `MPoint operator+ (const MVector &other) const`
- `MPoint operator- (const MVector &other) const`
- `MPoint & operator+= (const MVector &vector)`
- `MPoint & operator-= (const MVector &vector)`
- `MPoint operator* (const int scale) const`
- `MPoint operator* (const short scale) const`
- `MPoint operator* (const unsigned int scale) const`
- `MPoint operator* (const unsigned short scale) const`
- `MPoint operator* (const float scale) const`
- `MPoint operator* (const double scale) const`
- `MPoint operator/ (const double scale) const`
- `MPoint operator* (const MMatrix &) const`
- `MPoint & operator*= (const MMatrix &)`
- `MPoint & operator*= (const int)`
- `MPoint & operator*= (const short)`
- `MPoint & operator*= (const unsigned int)`
- `MPoint & operator*= (const unsigned short)`
- `MPoint & operator*= (const float)`
- `MPoint & operator*= (const double)`
- `bool operator== (const MPoint &other) const`
- `bool operator!= (const MPoint &other) const`
- `MPoint & cartesianize ()`
- `MPoint & rationalize ()`
- `MPoint & homogenize ()`
- `double distanceTo (const MPoint &other) const`
- `bool isEquivalent (const MPoint &other, double tolerance=MPoint_kTol) const`
- `double & operator[] (unsigned int i)`
- `double & operator() (unsigned int i)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `double x`
- `double y`
- `double z`
- `double w`

**Static Public Attributes**

- `static const MPoint origin`

**Friends**

- `OPENMAYA_EXPORT MPoint operator* (int, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (short, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (unsigned int, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (unsigned short, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (float, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (double, const MPoint &)`
- `OPENMAYA_EXPORT MPoint operator* (const MMatrix &, const MPoint &)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MPoint &p)`

### MPointArray

Module `OpenMaya`. Source: [class_m_point_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_point_array.html).

**Public Member Functions**

- `MPointArray ()`
- `MPointArray (const MPointArray &other)`
- `MPointArray (const MPoint points[], unsigned int count)`
- `MPointArray (const double points[][4], unsigned int count)`
- `MPointArray (const float points[][4], unsigned int count)`
- `MPointArray (unsigned int initialSize, const MPoint &initialValue=MPoint::origin)`
- `~MPointArray ()`
- `const MPoint & operator[] (unsigned int index) const`
- `MPointArray & operator= (const MPointArray &other)`
- `MStatus set (const MPoint &element, unsigned int index)`
- `MStatus set (unsigned int index, double x, double y, double z=0, double w=1)`
- `MStatus set (double element[4], unsigned int index)`
- `MStatus set (float element[4], unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MPoint &element, unsigned int index)`
- `MStatus append (const MPoint &element)`
- `MStatus append (double x, double y, double z=0, double w=1)`
- `MStatus copy (const MPointArray &source)`
- `MStatus clear ()`
- `MStatus get (double[][4]) const`
- `MStatus get (float[][4]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MPoint & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MPointArray &array)`

### MPointOnMesh

Module `OpenMaya`. Source: [class_m_point_on_mesh.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_point_on_mesh.html).

**Public Member Functions**

- `MPointOnMesh ()`
- `MFloatPoint & getPoint ()`
- `MFloatVector & getNormal ()`
- `void getBarycentricCoords (float &u, float &v) const`
- `int faceIndex () const`
- `int triangleIndex () const`

**Friends**

- `class MMeshIntersector`

### MPolyMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_poly_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_poly_message.html).

**Public Types**

- `enum { kVertexIndex = 0, kEdgeIndex = 1, kFaceIndex = 2, kLastErrorIndex = 3 }`
- `enum MessageType { kUVSetAdded, kUVSetDeleted, kCurrentUVSetChanged }`
- `enum ColorSetMessageType { kColorSetAdded, kColorSetDeleted, kCurrentColorSetChanged }`
- `typedef void(* MNodeStringTypeFunction) (MObject &node, const MString &name, MPolyMessage::MessageType type, void *clientData)`
- `typedef void(* MNodeStringTypeColorSetFunction) (MObject &node, const MString &name, MPolyMessage::ColorSetMessageType type, void *clientData)`

**Static Public Member Functions**

- `static MCallbackId addPolyComponentIdChangedCallback (MObject &node, bool wantIdModifications[], unsigned int count, MMessage::MComponentFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addPolyTopologyChangedCallback (MObject &node, MMessage::MNodeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addUVSetChangedCallback (MObject &node, MPolyMessage::MNodeStringTypeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addColorSetChangedCallback (MObject &node, MPolyMessage::MNodeStringTypeColorSetFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static unsigned int deletedId (void)`
- `static const char * className ()`

### MProfiler

Module `OpenMaya`. Source: [class_m_profiler.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_profiler.html).

**Public Types**

- `enum ProfilingColor`

**Static Public Member Functions**

- `static int addCategory (const char *categoryName, const char *categoryInfo)`
- `static int removeCategory (const char *categoryName)`
- `static void getAllCategories (MStringArray &categoryNames, MStringArray &categoryInfo)`
- `static int getCategoryIndex (const char *categoryName)`
- `static const char * getCategoryName (int categoryId)`
- `static const char * getCategoryInfo (int categoryId)`
- `static bool categoryRecording (const char *categoryName)`
- `static bool categoryRecording (int categoryId)`
- `static void setCategoryRecording (const char *categoryName, bool active)`
- `static void setCategoryRecording (int categoryId, bool active)`
- `static void signalEvent (int categoryId, ProfilingColor colorIndex, const char *eventName, const char *description=NULL)`
- `static int eventBegin (int categoryId, ProfilingColor colorIndex, const char *eventName, const char *description=NULL)`
- `static void eventEnd (int eventId)`
- `static int getEventCount ()`
- `static MUint64 getEventTime (int eventIndex)`
- `static int getEventDuration (int eventIndex)`
- `static const char * getEventName (int eventIndex)`
- `static const char * getDescription (int eventIndex)`
- `static int getEventCategory (int eventIndex)`
- `static int getThreadId (int eventIndex)`
- `static int getCPUId (int eventIndex)`
- `static bool isSignalEvent (int eventIndex)`
- `static MProfiler::ProfilingColor getColor (int eventIndex)`
- `static int getThreadDuration (int eventIndex)`
- `static int getBufferSize ()`
- `static MStatus setBufferSize (int sizeInMegaytes)`
- `static bool recordingActive ()`
- `static bool eventDataAvailable ()`
- `static bool resetRecording ()`
- `static void setRecordingActive (bool val)`
- `static bool saveRecording (const MString &fileName)`
- `static bool loadRecording (const MString &fileName)`
- `static bool isDataFromFile ()`
- `static int getNumberOfCPUs ()`
- `static const char * className ()`
- `static int addCategory (const char *categoryName)`
- `static void getAllCategories (MStringArray &categoryNames)`

### MProfilingScope

Module `OpenMaya`. Source: [class_m_profiling_scope.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_profiling_scope.html).

**Public Member Functions**

- `MProfilingScope (int categoryId, MProfiler::ProfilingColor colorIndex, const char *eventName, const char *description=NULL, const MObject &associatedNode=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `~MProfilingScope ()`

**Static Public Member Functions**

- `static const char * className ()`

### MPxAnimCurveInterpolator

Module `OpenMaya`. Source: [class_m_px_anim_curve_interpolator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_anim_curve_interpolator.html).

**Public Types**

- `enum InterpolatorFlags { kEvaluateAtKey = 0x001, kLockType = 0x002 }`

**Public Member Functions**

- `virtual void initialize (const MObject &animCurve, unsigned int keyIndex)`
- `virtual double evaluate (const MTime &val)=0`
- `MFnAnimCurve::TangentType typeId () const`
- `MString typeName () const`

### MPxAssembly

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_assembly.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_assembly.html).

**Public Member Functions**

- `MPxAssembly ()`
- `~MPxAssembly () override`
- `MPxNode::Type type () const override`
- `virtual MString createRepresentation (const MString &input, const MString &type, const MString &representation, MDagModifier *undoRedo=NULL, MStatus *ReturnStatus=NULL)=0`
- `virtual bool activate (const MString &rep)`
- `virtual MString getActive () const =0`
- `virtual bool isActive (const MString &rep) const`
- `virtual MStringArray getRepresentations (MStatus *ReturnStatus=NULL) const =0`
- `virtual MString getRepType (const MString &rep) const =0`
- `virtual MString getRepLabel (const MString &rep) const =0`
- `virtual MStringArray repTypes () const =0`
- `virtual MStatus deleteRepresentation (const MString &rep)=0`
- `virtual MStatus deleteAllRepresentations ()=0`
- `virtual MString getRepNamespace () const`
- `virtual void updateRepNamespace (const MString &repNamespace)`
- `virtual MString setRepName (const MString &rep, const MString &newName, MStatus *ReturnStatus=NULL)=0`
- `virtual MStatus setRepLabel (const MString &rep, const MString &label)=0`
- `virtual bool supportsEdits () const`
- `virtual bool supportsMemberChanges () const`
- `virtual bool canRepApplyEdits (const MString &representation) const`
- `virtual bool handlesAddEdits () const`
- `virtual MStatus addEdits ()`
- `virtual void beforeSave ()`
- `virtual void postLoad ()`
- `virtual void memberAdded (MObject &member)`
- `virtual void memberRemoved (MObject &member)`
- `bool performActivate (const MString &representation)`
- `bool performInactivate ()`
- `virtual bool activateRep (const MString &representation)=0`
- `virtual bool inactivateRep ()`
- `virtual void postActivateRep (const MString &representation)`
- `virtual void preApplyEdits ()`
- `virtual void preUnapplyEdits ()`
- `virtual void postApplyEdits ()`
- `virtual void postUnapplyEdits ()`
- `virtual MString getInitialRep (const MObject &assembly, bool &hasInitialRep, MStatus *ReturnStatus=NULL) const`
- `bool activating () const`
- `MTypeId typeId () const override`
- `MString typeName () const override`
- `MString name () const override`
- `MObject thisMObject () const override`
- `MStatus setExistWithoutInConnections (bool flag) override`
- `bool existWithoutInConnections (MStatus *ReturnStatus=NULL) const override`
- `MStatus setExistWithoutOutConnections (bool flag) override`
- `bool existWithoutOutConnections (MStatus *ReturnStatus=NULL) const override`
- `MStatus addSetAttrEdit (const MString &targetAssembly, const MString &plugName, const MString &parameters, MPxEditData *editData=NULL)`
- `MStatus addConnectAttrEdit (const MString &targetAssembly, const MString &srcPlugName, const MString &dstPlugName, MPxEditData *editData=NULL)`
- `MStatus addDisconnectAttrEdit (const MString &targetAssembly, const MString &srcPlugName, const MString &dstPlugName, MPxEditData *editData=NULL)`
- `MStatus addDeleteAttrEdit (const MString &targetAssembly, const MString &nodeName, const MString &attributeName, MPxEditData *editData=NULL)`
- `MStatus addAddAttrEdit (const MString &targetAssembly, const MString &nodeName, const MString &longAttributeName, const MString &shortAttributeName, const MString &parameters, MPxEditData *editData=NULL)`
- `MStatus addParentEdit (const MString &targetAssembly, const MString &childNodeName, const MString &parentNodeName, const MString &parameters, MPxEditData *editData=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MDataBlock forceCache () override`
- `MStatus setDoNotWrite (bool flag) override`
- `bool doNotWrite (MStatus *ReturnStatus=nullptr) const override`
- `void setMPSafe (bool flag) override`
- `virtual MString getInitialRep (const MObject &assembly, MStatus *ReturnStatus=NULL) const`

### MPxAttributePatternFactory

Module `OpenMaya`. Source: [class_m_px_attribute_pattern_factory.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_attribute_pattern_factory.html).

**Public Member Functions**

- `MPxAttributePatternFactory ()`
- `virtual ~MPxAttributePatternFactory ()`
- `virtual MStatus createPatternsFromString (const MString &patternString, MAttributePatternArray &createdPatterns) const =0`
- `virtual MStatus createPatternsFromFile (const MString &patternFile, MAttributePatternArray &createdPatterns) const =0`
- `virtual MString name () const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxBakeEngine

Module `OpenMaya`. Source: [class_m_px_bake_engine.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_bake_engine.html).

**Public Member Functions**

- `MPxBakeEngine ()`
- `virtual ~MPxBakeEngine ()`
- `void setNeedTransparency (bool t)`
- `void getUVRange (MFloatArray minUV, MFloatArray maxUV)`
- `virtual MStatus bake (const MDagPath &objectPath, const MDagPath &cameraPath, const MPlug &samplePlug, MImage &bakeResult)=0`

### MPxCacheConfigRuleFilter

Module `OpenMaya`. Source: [class_m_px_cache_config_rule_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_cache_config_rule_filter.html).

**Public Member Functions**

- `virtual void preRulesExecution ()`
- `virtual void postRulesExecution ()`
- `virtual bool isMatch (const MEvaluationNode &evalNode)`

### MPxCacheFormat

Module `OpenMaya`. Source: [class_m_px_cache_format.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_cache_format.html).

**Public Types**

- `enum FileAccessMode { kRead, kWrite, kReadWrite }`

**Public Member Functions**

- `MPxCacheFormat ()`
- `virtual ~MPxCacheFormat ()`
- `virtual MStatus open (const MString &fileName, FileAccessMode mode)`
- `virtual void close ()`
- `virtual MStatus isValid ()`
- `virtual MStatus rewind ()`
- `virtual MString extension ()`
- `virtual MStatus readHeader ()`
- `virtual MStatus writeHeader (const MString &version, MTime &startTime, MTime &endTime)`
- `virtual void beginWriteChunk ()`
- `virtual void endWriteChunk ()`
- `virtual MStatus beginReadChunk ()`
- `virtual void endReadChunk ()`
- `virtual MStatus writeTime (MTime &time)`
- `virtual MStatus readTime (MTime &time)`
- `virtual MStatus findTime (MTime &time, MTime &foundTime)`
- `virtual MStatus readNextTime (MTime &foundTime)`
- `virtual unsigned readArraySize ()`
- `virtual MStatus writeDoubleArray (const MDoubleArray &)`
- `virtual MStatus writeFloatArray (const MFloatArray &)`
- `virtual MStatus writeIntArray (const MIntArray &)`
- `virtual MStatus writeDoubleVectorArray (const MVectorArray &array)`
- `virtual MStatus writeFloatVectorArray (const MFloatVectorArray &array)`
- `virtual MStatus writeInt32 (int)`
- `virtual MStatus readDoubleArray (MDoubleArray &, unsigned size)`
- `virtual MStatus readFloatArray (MFloatArray &, unsigned size)`
- `virtual MStatus readIntArray (MIntArray &, unsigned size)`
- `virtual MStatus readDoubleVectorArray (MVectorArray &, unsigned arraySize)`
- `virtual MStatus readFloatVectorArray (MFloatVectorArray &array, unsigned arraySize)`
- `virtual int readInt32 ()`
- `virtual MStatus writeChannelName (const MString &name)`
- `virtual MStatus findChannelName (const MString &name)`
- `virtual MStatus readChannelName (MString &name)`
- `virtual bool handlesDescription ()`
- `virtual MStatus readDescription (MCacheFormatDescription &description, const MString &descriptionFileLocation, const MString &baseFileName)`
- `virtual MStatus writeDescription (const MCacheFormatDescription &description, const MString &descriptionFileLocation, const MString &baseFileName)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxCameraSet

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_camera_set.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_camera_set.html).

**Public Member Functions**

- `MPxCameraSet ()`
- `~MPxCameraSet () override`
- `MPxNode::Type type () const override`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject cameraLayer`
- `static MObject camera`
- `static MObject sceneData`
- `static MObject active`
- `static MObject order`

### MPxCommand

Module `OpenMaya`. Source: [class_m_px_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_command.html).

**Public Types**

- `enum MResultType { kLong, kDouble, kString, kNoArg }`

**Public Member Functions**

- `MPxCommand ()`
- `virtual ~MPxCommand ()`
- `virtual MStatus doIt (const MArgList &args)`
- `virtual MStatus undoIt ()`
- `virtual MStatus redoIt ()`
- `virtual bool isUndoable () const`
- `virtual bool hasSyntax () const`
- `MSyntax syntax () const`
- `bool isHistoryOn () const`
- `MString commandString () const`
- `MStatus setHistoryOn (bool state)`
- `MStatus setCommandString (const MString &)`
- `MStatus setUndoable (bool state)`

**Static Public Member Functions**

- `static void displayInfo (const MString &theInfo)`
- `static void displayWarning (const MString &theWarning, bool showLineNumber=false)`
- `static void displayError (const MString &theError, bool showLineNumber=false)`
- `static void clearResult ()`
- `static void setResult (unsigned int val)`
- `static void setResult (int val)`
- `static void setResult (double val)`
- `static void setResult (bool val)`
- `static void setResult (const char *val)`
- `static void setResult (const MString &val)`
- `static void setResult (const MIntArray &val)`
- `static void setResult (const MDoubleArray &val)`
- `static void setResult (const MStringArray &val)`
- `static void appendToResult (int val)`
- `static void appendToResult (double val)`
- `static void appendToResult (bool val)`
- `static void appendToResult (const char *val)`
- `static void appendToResult (const MString &val)`
- `static void appendToResult (const MStringArray &val)`
- `static bool isCurrentResultArray ()`
- `static MResultType currentResultType ()`
- `static MStatus getCurrentResult (int &val)`
- `static MStatus getCurrentResult (double &val)`
- `static MStatus getCurrentResult (MString &val)`
- `static MStatus getCurrentResult (MIntArray &val)`
- `static MStatus getCurrentResult (MDoubleArray &val)`
- `static MStatus getCurrentResult (MStringArray &val)`
- `static int currentIntResult (MStatus *ReturnStatus=NULL)`
- `static double currentDoubleResult (MStatus *ReturnStatus=NULL)`
- `static MString currentStringResult (MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MPxComponentShape

Module `OpenMaya`. Inherits `MPxSurfaceShape`. Source: [class_m_px_component_shape.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_component_shape.html).

**Public Member Functions**

- `void transformUsing (const MMatrix &matrix, const MObjectArray &componentList) override`
- `void transformUsing (const MMatrix &mat, const MObjectArray &componentList, MPxSurfaceShape::MVertexCachingMode cachingMode, MPointArray *pointCache) override`
- `void componentToPlugs (MObject &component, MSelectionList &list) const override`
- `bool match (const MSelectionMask &mask, const MObjectArray &componentList) const override`
- `MObject createFullVertexGroup () const override`
- `MObject localShapeInAttr () const override`
- `MVectorArray * getControlPoints ()`
- `MStatus setControlPoints (MVectorArray *cps)`

### MPxCustomEvaluator

Module `OpenMaya`. Source: [class_m_px_custom_evaluator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_custom_evaluator.html).

**Public Types**

- `enum ConsolidationType { kConsolidateNone, kConsolidateSubgraph, kConsolidateFlood }`

**Public Member Functions**

- `MPxCustomEvaluator ()`
- `virtual ~MPxCustomEvaluator ()`
- `bool active (MStatus *ReturnStatus=NULL) const`
- `void setActive (bool isNowActive, MStatus *ReturnStatus=NULL)`
- `bool ready (MStatus *ReturnStatus=NULL) const`
- `void setReady (bool isNowReady, MStatus *ReturnStatus=NULL)`
- `ConsolidationType consolidation (MStatus *ReturnStatus=NULL) const`
- `void setConsolidation (ConsolidationType newType, MStatus *ReturnStatus=NULL)`
- `virtual bool markIfSupported (const MEvaluationNode *node)`
- `virtual void preEvaluate (const MEvaluationGraph *graph)`
- `virtual void postEvaluate (const MEvaluationGraph *graph)`
- `virtual bool clusterInitialize (const MCustomEvaluatorClusterNode *cluster)`
- `virtual bool clusterReady (const MCustomEvaluatorClusterNode *cluster)`
- `virtual void clusterTerminate (const MCustomEvaluatorClusterNode *cluster)`
- `virtual void clusterEvaluate (const MCustomEvaluatorClusterNode *cluster)`
- `virtual bool wantPruneExecution () const`
- `virtual bool pruneExecution (const MCustomEvaluatorClusterNode *cluster)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxData

Module `OpenMaya`. Source: [class_m_px_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_data.html).

**Public Types**

- `enum Type { kData, kGeometryData, kLast }`

**Public Member Functions**

- `MPxData ()`
- `virtual ~MPxData ()`
- `virtual MStatus readASCII (const MArgList &argList, unsigned int &endOfTheLastParsedElement)`
- `virtual MStatus readBinary (std::istream &in, unsigned int length)`
- `virtual MStatus writeASCII (std::ostream &out)`
- `virtual MStatus writeBinary (std::ostream &out)`
- `virtual void copy (const MPxData &src)=0`
- `virtual MTypeId typeId () const =0`
- `virtual MString name () const =0`

### MPxEditData

Module `OpenMaya`. Source: [class_m_px_edit_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_edit_data.html).

**Public Member Functions**

- `MPxEditData ()`
- `virtual ~MPxEditData ()`
- `bool isEqual (const MPxEditData *other) const`
- `bool isLessThan (const MPxEditData *other) const`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `virtual bool performIsEqual (const MPxEditData *other) const =0`
- `virtual bool performIsLessThan (const MPxEditData *other) const =0`

### MPxFileResolver

Module `OpenMaya`. Source: [class_m_px_file_resolver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_file_resolver.html).

**Public Types**

- `enum MPxFileResolverMode { kNone = (1<<0), kInput = (1<<1) }`

**Public Member Functions**

- `MPxFileResolver ()`
- `virtual ~MPxFileResolver ()`
- `virtual MString resolveURI (const MURI &uriValue, MPxFileResolver::MPxFileResolverMode mode, MStatus *ReturnStatus=NULL)`
- `virtual MString resolveURIWithContext (const MURI &uriValue, MPxFileResolver::MPxFileResolverMode mode, const MString &contextNodeFullName, MStatus *ReturnStatus=NULL)`
- `virtual MString uriScheme () const`
- `MString resolverName () const`
- `virtual void performAfterSaveURI (const MURI &uriValue, const MString &resolvedFullPath)`

**Static Public Member Functions**

- `static const char * className ()`
- `static MPxFileResolver * findURIResolverByScheme (const MString &uriScheme)`
- `static MPxFileResolver * findURIResolverByName (const MString &resolverName)`
- `static MStringArray getURIResolversByScheme ()`
- `static MStringArray getURIResolversByName ()`
- `static int numURIResolvers ()`

### MPxFileTranslator

Module `OpenMaya`. Source: [class_m_px_file_translator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_file_translator.html).

**Public Types**

- `enum MFileKind { kIsMyFileType, kCouldBeMyFileType, kNotMyFileType }`
- `enum FileAccessMode { kUnknownAccessMode, kOpenAccessMode, kReferenceAccessMode, kImportAccessMode, kSaveAccessMode, kExportAccessMode, kExportActiveAccessMode }`

**Public Member Functions**

- `MPxFileTranslator ()`
- `virtual ~MPxFileTranslator ()`
- `virtual MStatus reader (const MFileObject &file, const MString &optionsString, FileAccessMode mode)`
- `virtual MStatus writer (const MFileObject &file, const MString &optionsString, FileAccessMode mode)`
- `virtual bool haveReadMethod () const`
- `virtual bool haveWriteMethod () const`
- `virtual bool haveNamespaceSupport () const`
- `virtual bool haveReferenceMethod () const`
- `virtual MString defaultExtension () const`
- `virtual MString filter () const`
- `virtual bool canBeOpened () const`
- `virtual MPxFileTranslator::MFileKind identifyFile (const MFileObject &file, const char *buffer, short size) const`

**Static Public Member Functions**

- `static MPxFileTranslator::FileAccessMode fileAccessMode ()`

### MPxGeometryData

Module `OpenMaya`. Inherits `MPxData`. Source: [class_m_px_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_geometry_data.html).

**Public Member Functions**

- `MPxGeometryData ()`
- `~MPxGeometryData () override`
- `virtual MPxGeometryIterator * iterator (MObjectArray &componentList, MObject &component, bool useComponents)`
- `virtual MPxGeometryIterator * iterator (MObjectArray &componentList, MObject &component, bool useComponents, bool world) const`
- `virtual bool updateCompleteVertexGroup (MObject &component) const`
- `virtual bool deleteComponent (const MObjectArray &compList)`
- `virtual bool deleteComponentsFromGroups (const MObjectArray &compList, MIntArray &groupIdArray, MObjectArray &groupComponentArray)`
- `virtual bool smartCopy (const MPxGeometryData *srcGeom)`
- `void copy (const MPxData &src) override=0`
- `MTypeId typeId () const override=0`
- `MString name () const override=0`
- `void setMatrix (const MMatrix &)`
- `const MMatrix & matrix () const`
- `bool matrix (MMatrix &) const`

### MPxGeometryIterator

Module `OpenMaya`. Source: [class_m_px_geometry_iterator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_geometry_iterator.html).

**Public Member Functions**

- `MPxGeometryIterator (void *userGeometry, MObjectArray &components)`
- `MPxGeometryIterator (void *userGeometry, MObject &components)`
- `virtual ~MPxGeometryIterator ()`
- `virtual bool isDone () const`
- `virtual MStatus next ()`
- `virtual void reset ()`
- `virtual void component (MObject &component)`
- `virtual bool hasPoints () const`
- `virtual int iteratorCount () const`
- `virtual MPoint point () const`
- `virtual void setPoint (const MPoint &) const`
- `virtual int setPointGetNext (MPoint &)`
- `virtual int index () const`
- `virtual bool hasNormals () const`
- `virtual int indexUnsimplified () const`
- `int currentPoint () const`
- `void setCurrentPoint (int)`
- `int maxPoints () const`
- `void setMaxPoints (int)`
- `void setObject (MPxSurfaceShape &)`
- `void * geometry () const`

**Static Public Member Functions**

- `static const char * className ()`

### MPxImageFile

Module `OpenMaya`. Source: [class_m_px_image_file.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_image_file.html).

**Public Member Functions**

- `MPxImageFile ()`
- `virtual ~MPxImageFile ()`
- `virtual MStatus open (MString pathname, MImageFileInfo *info)`
- `virtual MStatus load (MImage &image, unsigned int imageNumber)`
- `virtual MStatus glLoad (const MImageFileInfo &info, unsigned int imageNumber)`
- `virtual MStatus close ()`

### MPxImagePlane

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_image_plane.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_image_plane.html).

**Public Member Functions**

- `MPxImagePlane ()`
- `~MPxImagePlane () override`
- `MPxNode::Type type () const override`
- `virtual MStatus loadImageMap (const MString &fileName, int frame, MImage &image)`
- `bool getExactImageFile (const MString &refFileName, MString &actualName)`
- `MString exactImageFile (const MString &refFileName)`
- `void refreshImage ()`
- `void setImageDirty ()`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject imageType`
- `static MObject imageName`
- `static MObject useFrameExtension`
- `static MObject frameExtension`
- `static MObject coverage`
- `static MObject coverageX`
- `static MObject coverageY`
- `static MObject coverageOrigin`
- `static MObject coverageOriginX`
- `static MObject coverageOriginY`
- `static MObject sourceTexture`
- `static MObject fit`
- `static MObject displayMode`
- `static MObject displayOnlyIfCurrent`
- `static MObject colorGain`
- `static MObject colorGainR`
- `static MObject colorGainG`
- `static MObject colorGainB`
- `static MObject colorOffset`
- `static MObject colorOffsetR`
- `static MObject colorOffsetG`
- `static MObject colorOffsetB`
- `static MObject alphaGain`
- `static MObject shadingSamplesOverride`
- `static MObject shadingSamples`
- `static MObject maxShadingSamples`
- `static MObject lockedToCamera`
- `static MObject depth`
- `static MObject squeezeCorrection`
- `static MObject size`
- `static MObject sizeX`
- `static MObject sizeY`
- `static MObject offset`
- `static MObject offsetX`
- `static MObject offsetY`
- `static MObject center`
- `static MObject centerX`
- `static MObject centerY`
- `static MObject centerZ`
- `static MObject width`
- `static MObject height`
- `static MObject visibleInReflections`
- `static MObject visibleInRefractions`
- `static MObject rotate`
- `static MObject alreadyPremult`
- `static MObject useDepthMap`
- `static MObject composite`
- `static MObject depthOversample`
- `static MObject separateDepth`
- `static MObject depthFile`
- `static MObject depthBias`
- `static MObject depthScale`

### MPxMaterialInformation

Module `OpenMaya`. Source: [class_m_px_material_information.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_material_information.html).

**Public Types**

- `enum MaterialType { kSimpleMaterial = 0, kTexture, kOverrideDraw }`

**Public Member Functions**

- `MPxMaterialInformation (MObject &materialNode)`
- `virtual ~MPxMaterialInformation ()`
- `bool useMaterialAsTexture ()`
- `virtual bool materialInfoIsDirty (const MPlug &plug)=0`
- `virtual bool connectAsTexture (const MPlug &plug)=0`
- `virtual bool textureDisconnected (const MPlug &plug)=0`
- `virtual bool computeMaterial (MaterialInputData &data)=0`

**Protected Member Functions**

- `MPxMaterialInformation ()`

**Protected Attributes**

- `MObject fMaterialNode`
- `MaterialType fMaterialType`

### MPxMayaAsciiFilter

Module `OpenMaya`. Inherits `MPxFileTranslator`. Source: [class_m_px_maya_ascii_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_maya_ascii_filter.html).

**Public Member Functions**

- `MPxMayaAsciiFilter ()`
- `~MPxMayaAsciiFilter () override`
- `bool haveWriteMethod () const override`
- `bool haveReadMethod () const override`
- `MStatus reader (const MFileObject &file, const MString &optionsString, FileAccessMode mode) override`
- `MStatus writer (const MFileObject &file, const MString &optionsString, FileAccessMode mode) override`

**Protected Member Functions**

- `const MFileObject file () const`
- `virtual MStatus processReadOptions (const MString &optionsString)`
- `virtual MStatus processWriteOptions (const MString &optionsString)`
- `virtual bool writesRequirements () const`
- `virtual bool writesMetadata () const`
- `virtual bool writesCreateNode (const MObject &node) const`
- `virtual bool writesSetAttr (const MPlug &srcPlug) const`
- `virtual bool writesConnectAttr (const MPlug &srcPlug, const MPlug &destPlug) const`
- `virtual bool writesDisconnectAttr (const MPlug &srcPlug, const MPlug &destPlug) const`
- `virtual bool writesParentNode (const MDagPath &parent, const MDagPath &child) const`
- `virtual bool writesSelectNode (const MObject &node) const`
- `virtual bool writesFileReference (const MFileObject &referenceFile) const`
- `virtual MStatus writePostHeader (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePostRequires (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePreCreateNodesBlock (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePostCreateNodesBlock (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePreConnectAttrsBlock (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePostConnectAttrsBlock (MPxMayaAsciiFilterOutput &fileIO)`
- `virtual MStatus writePreTrailer (MPxMayaAsciiFilterOutput &fileIO)`

### MPxMayaAsciiFilterOutput

Module `OpenMaya`. Source: [class_m_px_maya_ascii_filter_output.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_maya_ascii_filter_output.html).

**Public Member Functions**

- `MPxMayaAsciiFilterOutput & operator<< (const char *data)`

### MPxMotionPathNode

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_motion_path_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_motion_path_node.html).

**Public Types**

- `enum worldUpVectorValue { kUpScene, kUpObject, kUpObjectRotation, kUpVector, kUpNormal }`

**Public Member Functions**

- `MPxMotionPathNode ()`
- `~MPxMotionPathNode () override`
- `MPxNode::Type type () const override`
- `MPoint position (MDataBlock &data, double f, MStatus *status=NULL) const`
- `MStatus getVectors (MDataBlock &data, double f, MVector &front, MVector &side, MVector &up, const MVector *worldUp=NULL) const`
- `MQuaternion banking (MDataBlock &data, double f, const MVector &worldUp, double bankScale, double bankLimit, MStatus *status=NULL) const`
- `MStatus evaluatePath (MDataBlock &data, double u, double uRange, bool wraparound, double sideOffset, double upOffset, bool follow, bool inverseFront, bool inverseUp, int frontAxis, int upAxis, double frontTwist, double upTwist, double sideTwist, bool bank, double bankScale, double bankLimit, MPoint &resultPosition, MMatrix &resultOrientation) const`
- `double parametricToFractional (double u, MStatus *status=NULL) const`
- `double fractionalToParametric (double f, MStatus *status=NULL) const`
- `double wraparoundFractionalValue (double f, MStatus *status=NULL) const`
- `MMatrix matrix (const MVector &front, const MVector &side, const MVector &up, int frontAxisIdx=1, int upAxisIdx=2, MStatus *status=NULL) const`

**Static Public Attributes**

- `static MObject uValue`
- `static MObject frontTwist`
- `static MObject upTwist`
- `static MObject sideTwist`
- `static MObject flowNode`
- `static MObject pathGeometry`
- `static MObject positionMarkerTime`
- `static MObject orientationMarkerTime`
- `static MObject follow`
- `static MObject normal`
- `static MObject inverseUp`
- `static MObject inverseFront`
- `static MObject frontAxis`
- `static MObject upAxis`
- `static MObject worldUpType`
- `static MObject worldUpVector`
- `static MObject worldUpMatrix`
- `static MObject bank`
- `static MObject bankScale`
- `static MObject bankThreshold`
- `static MObject fractionMode`
- `static MObject updateOrientationMarkers`
- `static MObject allCoordinates`
- `static MObject xCoordinate`
- `static MObject yCoordinate`
- `static MObject zCoordinate`
- `static MObject orientMatrix`
- `static MObject rotate`
- `static MObject rotateX`
- `static MObject rotateY`
- `static MObject rotateZ`
- `static MObject rotateOrder`

### MPxMultiPolyTweakUVCommand

Module `OpenMaya`. Inherits `MPxCommand`. Source: [class_m_px_multi_poly_tweak_u_v_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_multi_poly_tweak_u_v_command.html).

**Public Member Functions**

- `MPxMultiPolyTweakUVCommand ()`
- `~MPxMultiPolyTweakUVCommand () override`
- `virtual MStatus parseSyntax (MArgDatabase &argData)`
- `virtual MStatus preProcessUVs (const MObject &mesh, const MIntArray &uvList)`
- `virtual MStatus getTweakedUVs (const MObject &mesh, MIntArray &uvList, MFloatArray &uPos, MFloatArray &vPos)`

**Static Public Member Functions**

- `static MSyntax newSyntax ()`

### MPxNode

Module `OpenMaya`. Source: [class_m_px_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_node.html).

**Public Types**

- `enum Type { kDependNode, kLocatorNode, kDeformerNode, kManipContainer, kSurfaceShape, kFieldNode, kEmitterNode, kSpringNode, kIkSolverNode, kHardwareShader, kHwShaderNode, kTransformNode, kObjectSet, kFluidEmitterNode, kImagePlaneNode, kParticleAttributeMapperNode, kCameraSetNode, kConstraintNode, kManipulatorNode, kMotionPathNode, kClientDeviceNode, kThreadedDeviceNode, kAssembly, kSkinCluster, kGeometryFilter, kBlendShape, kLast }`
- `enum SchedulingType { kParallel, kSerial, kGloballySerial, kUntrusted , kDefaultScheduling = kSerial, kSerialize = kSerial, kGloballySerialize = kGloballySerial }`

**Public Member Functions**

- `MPxNode ()`
- `virtual ~MPxNode ()`
- `virtual void postConstructor ()`
- `virtual MStatus compute (const MPlug &, MDataBlock &)`
- `virtual bool getInternalValue (const MPlug &, MDataHandle &)`
- `virtual bool setInternalValue (const MPlug &, const MDataHandle &)`
- `virtual int internalArrayCount (const MPlug &) const`
- `virtual void copyInternalData (MPxNode *)`
- `virtual MStatus legalConnection (const MPlug &, const MPlug &, bool asSrc, bool &isLegal) const`
- `virtual MStatus legalDisconnection (const MPlug &, const MPlug &, bool asSrc, bool &isLegal) const`
- `virtual MStatus setDependentsDirty (const MPlug &plug, MPlugArray &plugArray)`
- `virtual MStatus preEvaluation (const MDGContext &context, const MEvaluationNode &evaluationNode)`
- `virtual MStatus postEvaluation (const MDGContext &context, const MEvaluationNode &evaluationNode, PostEvaluationType evalType)`
- `virtual SchedulingType schedulingType () const`
- `virtual void getCacheSetup (const MEvaluationNode &, MNodeCacheDisablingInfo &, MNodeCacheSetupInfo &, MObjectArray &) const`
- `virtual void configCache (const MEvaluationNode &, MCacheSchema &) const`
- `virtual MTimeRange transformInvalidationRange (const MPlug &source, const MTimeRange &input) const`
- `bool hasInvalidationRangeTransformation () const`
- `virtual MStatus connectionMade (const MPlug &, const MPlug &, bool asSrc)`
- `virtual MStatus connectionBroken (const MPlug &, const MPlug &, bool asSrc)`
- `virtual MStatus dependsOn (const MPlug &, const MPlug &, bool &depends) const`
- `virtual bool isPassiveOutput (const MPlug &) const`
- `virtual MStatus shouldSave (const MPlug &, bool &isSaving)`
- `virtual MPlug passThroughToOne (const MPlug &) const`
- `virtual bool passThroughToMany (const MPlug &, MPlugArray &) const`
- `virtual Type type () const`
- `virtual bool isAbstractClass () const`
- `virtual bool isTrackingTopology () const`
- `virtual MStringArray getFilesToArchive (bool shortName=false, bool unresolvedName=false, bool markCouldBeImageSequence=false) const`
- `virtual void getExternalContent (MExternalContentInfoTable &table) const`
- `bool addExternalContentForFileAttr (MExternalContentInfoTable &, const MObject &attr) const`
- `bool setExternalContentForFileAttr (const MObject &attr, const MExternalContentLocationTable &)`
- `virtual void setExternalContent (const MExternalContentLocationTable &)`
- `virtual MTypeId typeId () const`
- `virtual MString typeName () const`
- `virtual MString name () const`
- `virtual MObject thisMObject () const`
- `virtual MStatus setExistWithoutInConnections (bool flag)`
- `virtual bool existWithoutInConnections (MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setExistWithoutOutConnections (bool flag)`
- `virtual bool existWithoutOutConnections (MStatus *ReturnStatus=NULL) const`
- `void _setMPSafe (bool isMPSafe)`
- `MDataBlock _forceCache (const MDGContext &)`
- `virtual bool getInternalValueInContext (const MPlug &, MDataHandle &, MDGContext &)`
- `virtual bool setInternalValueInContext (const MPlug &, const MDataHandle &, MDGContext &)`
- `virtual int internalArrayCount (const MPlug &, const MDGContext &) const`

**Static Public Member Functions**

- `static MStatus addAttribute (const MObject &attr)`
- `static MStatus inheritAttributesFrom (const MString &parentClassName)`
- `static MStatus attributeAffects (const MObject &whenChanges, const MObject &isAffected)`
- `static MStatus attributeAffects (const MObject &whenChanges, const MObject &isAffected, bool affectsTopology)`
- `static const char * className ()`

**Static Public Attributes**

- `static MObject message`
- `static MObject isHistoricallyInteresting`
- `static MObject caching`
- `static MObject state`
- `static MObject frozen`

**Protected Member Functions**

- `virtual MDataBlock forceCache ()`
- `virtual MStatus setDoNotWrite (bool flag)`
- `virtual bool doNotWrite (MStatus *ReturnStatus=NULL) const`
- `virtual void setMPSafe (bool isMPSafe)`
- `virtual MDataBlock forceCache (const MDGContext &)`

### MPxObjectSet

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_object_set.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_object_set.html).

**Public Member Functions**

- `MPxObjectSet ()`
- `~MPxObjectSet () override`
- `MPxNode::Type type () const override`
- `virtual bool canBeDeleted (bool isSrcNode)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject dagSetMembers`
- `static MObject DNSetMembers`
- `static MObject memberWireframeColor`
- `static MObject annotation`
- `static MObject isLayer`
- `static MObject verticesOnlySet`
- `static MObject edgesOnlySet`
- `static MObject facetsOnlySet`
- `static MObject editPointsOnlySet`
- `static MObject renderableOnlySet`
- `static MObject partition`
- `static MObject groupNodes`
- `static MObject usedByNodes`

### MPxPolyTrg

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_poly_trg.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_poly_trg.html).

**Public Types**

- `typedef void(* polyTrgFnct) (const float *vert, const float *norm, const int *loopSizes, const int nbLoop, const int nbTrg, unsigned short *trg)`

**Public Member Functions**

- `MPxPolyTrg ()`
- `~MPxPolyTrg () override`
- `void postConstructor () override`
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`
- `bool isAbstractClass () const override`
- `MStatus registerTrgFunction (const char *name, polyTrgFnct f)`
- `MStatus unregisterTrgFunction (const char *name)`

### MPxPolyTweakUVCommand

Module `OpenMaya`. Inherits `MPxCommand`. Source: [class_m_px_poly_tweak_u_v_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_poly_tweak_u_v_command.html).

**Public Member Functions**

- `MPxPolyTweakUVCommand ()`
- `~MPxPolyTweakUVCommand () override`
- `virtual MStatus parseSyntax (MArgDatabase &argData)`
- `virtual MStatus getTweakedUVs (const MObject &mesh, MIntArray &uvList, MFloatArray &uPos, MFloatArray &vPos)`

**Static Public Member Functions**

- `static MSyntax newSyntax ()`

### MPxRepresentation

Module `OpenMaya`. Source: [class_m_px_representation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_representation.html).

**Public Member Functions**

- `virtual ~MPxRepresentation ()`
- `virtual bool activate ()=0`
- `virtual bool inactivate ()`
- `virtual MString getType () const =0`
- `MString getName () const`
- `virtual bool canApplyEdits () const`
- `virtual void getExternalContent (MExternalContentInfoTable &table) const`
- `virtual void setExternalContent (const MExternalContentLocationTable &table)`

**Protected Member Functions**

- `MPxRepresentation (MPxAssembly *assembly, const MString &name)`
- `MPxAssembly * getAssembly () const`

### MPxSurfaceShape

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_surface_shape.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_surface_shape.html).

**Public Types**

- `enum MVertexCachingMode { kNoPointCaching, kSavePoints, kRestorePoints, kUpdatePoints, kTransformOriginalPoints }`
- `enum MVertexOffsetMode { kNormal, kUTangent, kVTangent, kUVNTriad }`
- `enum MatchResult { kMatchOk, kMatchNone, kMatchTooMany, kMatchInvalidName, kMatchInvalidAttribute, kMatchInvalidAttributeIndex, kMatchInvalidAttributeRange, kMatchInvalidAttributeDim }`
- `enum MChildChanged { kObjectChanged, kBoundingBoxChanged }`

**Public Member Functions**

- `MPxSurfaceShape ()`
- `~MPxSurfaceShape () override`
- `MPxNode::Type type () const override`
- `virtual bool isBounded () const`
- `virtual MBoundingBox boundingBox () const`
- `virtual bool evalNodeAffectsDrawDb (const MEvaluationNode &evaluationNode)`
- `virtual void transformUsing (const MMatrix &mat, const MObjectArray &componentList)`
- `virtual void transformUsing (const MMatrix &mat, const MObjectArray &componentList, MVertexCachingMode cachingMode, MPointArray *pointCache)`
- `virtual void tweakUsing (const MMatrix &mat, const MObjectArray &componentList, MVertexCachingMode cachingMode, MPointArray *pointCache, MArrayDataHandle &handle)`
- `bool convertToTweakNodePlug (MPlug &plug) const`
- `virtual void weightedTransformUsing (const MTransformationMatrix &xform, const MMatrix *space, const MObjectArray &componentList, MVertexCachingMode cachingMode, MPointArray *pointCache, const MPlane *freezePlane)`
- `virtual void weightedTweakUsing (const MTransformationMatrix &xform, const MMatrix *space, const MObjectArray &componentList, MVertexCachingMode cachingMode, MPointArray *pointCache, const MPlane *freezePlane, MArrayDataHandle &handle)`
- `virtual bool vertexOffsetDirection (MObject &component, MVectorArray &direction, MVertexOffsetMode mode, bool normalize)`
- `virtual MObject newControlPointComponent () const`
- `virtual void componentToPlugs (MObject &component, MSelectionList &selectionList) const`
- `virtual bool match (const MSelectionMask &mask, const MObjectArray &componentList) const`
- `virtual MatchResult matchComponent (const MSelectionList &item, const MAttributeSpecArray &spec, MSelectionList &list)`
- `virtual MSelectionMask getShapeSelectionMask () const`
- `virtual MSelectionMask getComponentSelectionMask () const`
- `virtual MObject createFullVertexGroup () const`
- `virtual MObject createFullRenderGroup () const`
- `virtual MFn::Type renderGroupComponentType () const`
- `virtual bool deleteComponents (const MObjectArray &componentList, MDoubleArray &undoInfo)`
- `virtual bool undeleteComponents (const MObjectArray &componentList, MDoubleArray &undoInfo)`
- `virtual MObject localShapeInAttr () const`
- `virtual MObject localShapeOutAttr () const`
- `virtual MObject worldShapeOutAttr () const`
- `virtual MObject cachedShapeAttr () const`
- `virtual MObject geometryData () const`
- `virtual bool canMakeLive () const`
- `virtual bool closestPoint (const MPoint &raySource, const MVector &rayDirection, MPoint &theClosestPoint, MVector &theClosestNormal, bool findClosestOnMiss, double tolerance=MPoint_kTol)`
- `virtual void closestPoint (const MPoint &toThisPoint, MPoint &theClosestPoint, double tolerance=MPoint_kTol)`
- `virtual bool pointAtParm (const MPoint &atThisParm, MPoint &evaluatedPoint)`
- `virtual MPxGeometryIterator * geometryIteratorSetup (MObjectArray &, MObject &, bool forReadOnly=false)`
- `virtual bool acceptsGeometryIterator (bool writeable=true)`
- `virtual bool acceptsGeometryIterator (MObject &, bool writeable=true, bool forReadOnly=false)`
- `virtual bool excludeAsPluginShape () const`
- `MObjectArray activeComponents () const`
- `bool hasActiveComponents () const`
- `void childChanged (MChildChanged=kObjectChanged)`
- `bool isRenderable () const`
- `void setRenderable (bool)`
- `MMatrix getWorldMatrix (MDataBlock &, unsigned int) const`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject mControlPoints`
- `static MObject mControlValueX`
- `static MObject mControlValueY`
- `static MObject mControlValueZ`
- `static MObject nodeBoundingBox`
- `static MObject nodeBoundingBoxMin`
- `static MObject nodeBoundingBoxMinX`
- `static MObject nodeBoundingBoxMinY`
- `static MObject nodeBoundingBoxMinZ`
- `static MObject nodeBoundingBoxMax`
- `static MObject nodeBoundingBoxMaxX`
- `static MObject nodeBoundingBoxMaxY`
- `static MObject nodeBoundingBoxMaxZ`
- `static MObject nodeBoundingBoxSize`
- `static MObject nodeBoundingBoxSizeX`
- `static MObject nodeBoundingBoxSizeY`
- `static MObject nodeBoundingBoxSizeZ`
- `static MObject center`
- `static MObject boundingBoxCenterX`
- `static MObject boundingBoxCenterY`
- `static MObject boundingBoxCenterZ`
- `static MObject matrix`
- `static MObject inverseMatrix`
- `static MObject worldMatrix`
- `static MObject worldInverseMatrix`
- `static MObject parentMatrix`
- `static MObject parentInverseMatrix`
- `static MObject visibility`
- `static MObject intermediateObject`
- `static MObject isTemplated`
- `static MObject instObjGroups`
- `static MObject objectGroups`
- `static MObject objectGrpCompList`
- `static MObject objectGroupId`
- `static MObject objectGroupColor`
- `static MObject useObjectColor`
- `static MObject objectColor`

### MPxTopologyEvaluator

Module `OpenMaya`. Source: [class_m_px_topology_evaluator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_topology_evaluator.html).

**Public Member Functions**

- `MPxTopologyEvaluator ()`
- `virtual ~MPxTopologyEvaluator ()`
- `bool active (MStatus *ReturnStatus=nullptr) const`
- `void setActive (bool isNowActive, MStatus *ReturnStatus=nullptr)`
- `virtual bool markIfSupported (const MEvaluationNode *node)`
- `virtual void preMakeTopology (const MEvaluationGraph *graph)`
- `virtual bool makeTopology (MTopologyClusterNode *cluster)`
- `virtual void postMakeTopology (const MEvaluationGraph *graph)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxTransform

Module `OpenMaya`. Inherits `MPxNode`. Source: [class_m_px_transform.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_transform.html).

**Public Member Functions**

- `MPxTransform ()`
- `MPxTransform (const MPxTransform &)`
- `MPxTransform & operator= (const MPxTransform &)`
- `~MPxTransform () override`
- `void postConstructor () override`
- `MPxNode::Type type () const override`
- `virtual MPxTransformationMatrix * createTransformationMatrix ()`
- `virtual bool isBounded () const`
- `virtual MBoundingBox boundingBox () const`
- `virtual bool treatAsTransform () const`
- `const MPxTransformationMatrix & transformationMatrix () const`
- `MPxTransformationMatrix * transformationMatrixPtr () const`
- `virtual void resetTransformation (const MMatrix &)`
- `virtual void resetTransformation (MPxTransformationMatrix *)`
- `MStatus compute (const MPlug &plug, MDataBlock &data) override`
- `virtual MStatus computeLocalTransformation (MPxTransformationMatrix *, MDataBlock &)`
- `virtual MStatus clearLimits ()`
- `bool isLimited (MFnTransform::LimitType, MStatus *ReturnStatus=NULL) const`
- `double limitValue (MFnTransform::LimitType, MStatus *ReturnStatus=NULL) const`
- `MStatus setLimit (MFnTransform::LimitType, double value)`
- `MStatus enableLimit (MFnTransform::LimitType, bool flag)`
- `virtual MStatus validateAndSetValue (const MPlug &, const MDataHandle &)`
- `virtual MMatrix getMatrix (MStatus *ReturnStatus=NULL)`
- `virtual MMatrix getMatrixInverse (MStatus *ReturnStatus=NULL)`
- `virtual MVector getTranslation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus translateTo (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus translateBy (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MQuaternion getRotation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MEulerRotation getEulerRotation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus rotateTo (const MQuaternion &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateBy (const MQuaternion &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateTo (const MEulerRotation &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateBy (const MEulerRotation &, MSpace::Space=MSpace::kTransform)`
- `virtual MVector getScale (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus scaleTo (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus scaleBy (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MVector getShear (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus shearTo (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus shearBy (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MPoint getScalePivot (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MPoint getRotatePivot (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MVector getScalePivotTranslation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MVector getRotatePivotTranslation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus setScalePivot (const MPoint &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MStatus setScalePivotTranslation (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus setRotatePivot (const MPoint &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MStatus setRotatePivotTranslation (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MTransformationMatrix::RotationOrder getRotationOrder ()`
- `virtual MStatus setRotationOrder (MTransformationMatrix::RotationOrder ro, bool reorder=true)`
- `virtual MQuaternion getRotateOrientation (MSpace::Space apiSpace=MSpace::kTransform, MStatus *ReturnStatus=NULL)`
- `virtual MStatus setRotateOrientation (const MQuaternion &q, MSpace::Space apiSpace=MSpace::kTransform, bool balance=true)`
- `MStatus updateMatrixAttrs ()`
- `MStatus updateMatrixAttrs (MObject &attr)`
- `void copyInternalData (MPxNode *) override`
- `MPxTransform (MPxTransformationMatrix *)`
- `virtual MStatus validateAndSetValue (const MPlug &, const MDataHandle &, const MDGContext &)`
- `MStatus updateMatrixAttrs (const MDGContext &)`
- `MStatus updateMatrixAttrs (MObject &attr, const MDGContext &)`
- `virtual MMatrix getMatrix (const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MMatrix getMatrixInverse (const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MVector getTranslation (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MStatus translateTo (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MStatus translateBy (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MQuaternion getRotation (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MEulerRotation getEulerRotation (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MStatus rotateTo (const MQuaternion &, MSpace::Space, const MDGContext &)`
- `virtual MStatus rotateBy (const MQuaternion &, MSpace::Space, const MDGContext &)`
- `virtual MStatus rotateTo (const MEulerRotation &, MSpace::Space, const MDGContext &)`
- `virtual MStatus rotateBy (const MEulerRotation &, MSpace::Space, const MDGContext &)`
- `virtual MVector getScale (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MStatus scaleTo (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MStatus scaleBy (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MVector getShear (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MStatus shearTo (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MStatus shearBy (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MPoint getScalePivot (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MPoint getRotatePivot (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MVector getScalePivotTranslation (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MVector getRotatePivotTranslation (MSpace::Space, const MDGContext &, MStatus *ReturnStatus=NULL)`
- `virtual MStatus setScalePivot (const MPoint &, MSpace::Space, bool balance, const MDGContext &)`
- `virtual MStatus setScalePivotTranslation (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MStatus setRotatePivot (const MPoint &, MSpace::Space, bool balance, const MDGContext &)`
- `virtual MStatus setRotatePivotTranslation (const MVector &, MSpace::Space, const MDGContext &)`
- `virtual MTransformationMatrix::RotationOrder getRotationOrder (const MDGContext &)`
- `virtual MStatus setRotationOrder (MTransformationMatrix::RotationOrder, bool reorder, const MDGContext &)`
- `virtual MQuaternion getRotateOrientation (MSpace::Space apiSpace, const MDGContext &apiContext, MStatus *ReturnStatus=NULL)`
- `virtual MStatus setRotateOrientation (const MQuaternion &q, MSpace::Space, bool balance, const MDGContext &)`

**Static Public Member Functions**

- `static void mustCallValidateAndSet (MObject &)`
- `static MStatus setNonAffineMatricesEnabled (bool)`
- `static bool isNonAffineMatricesEnabled (MStatus *ReturnStatus)`
- `static const char * className ()`

**Public Attributes**

- `MPxTransformationMatrix * baseTransformationMatrix`

**Static Public Attributes**

- `static MObject nodeBoundingBox`
- `static MObject nodeBoundingBoxMin`
- `static MObject nodeBoundingBoxMinX`
- `static MObject nodeBoundingBoxMinY`
- `static MObject nodeBoundingBoxMinZ`
- `static MObject nodeBoundingBoxMax`
- `static MObject nodeBoundingBoxMaxX`
- `static MObject nodeBoundingBoxMaxY`
- `static MObject nodeBoundingBoxMaxZ`
- `static MObject nodeBoundingBoxSize`
- `static MObject nodeBoundingBoxSizeX`
- `static MObject nodeBoundingBoxSizeY`
- `static MObject nodeBoundingBoxSizeZ`
- `static MObject center`
- `static MObject boundingBoxCenterX`
- `static MObject boundingBoxCenterY`
- `static MObject boundingBoxCenterZ`
- `static MObject matrix`
- `static MObject inverseMatrix`
- `static MObject worldMatrix`
- `static MObject worldInverseMatrix`
- `static MObject parentMatrix`
- `static MObject parentInverseMatrix`
- `static MObject visibility`
- `static MObject intermediateObject`
- `static MObject isTemplated`
- `static MObject ghosting`
- `static MObject instObjGroups`
- `static MObject objectGroups`
- `static MObject objectGrpCompList`
- `static MObject objectGroupId`
- `static MObject objectGroupColor`
- `static MObject useObjectColor`
- `static MObject objectColor`
- `static MObject drawOverride`
- `static MObject overrideDisplayType`
- `static MObject overrideLevelOfDetail`
- `static MObject overrideShading`
- `static MObject overrideTexturing`
- `static MObject overridePlayback`
- `static MObject overrideEnabled`
- `static MObject overrideVisibility`
- `static MObject overrideColor`
- `static MObject lodVisibility`
- `static MObject renderInfo`
- `static MObject identification`
- `static MObject layerRenderable`
- `static MObject layerOverrideColor`
- `static MObject renderLayerInfo`
- `static MObject renderLayerId`
- `static MObject renderLayerRenderable`
- `static MObject renderLayerColor`
- `static MObject translate`
- `static MObject translateX`
- `static MObject translateY`
- `static MObject translateZ`
- `static MObject rotate`
- `static MObject rotateX`
- `static MObject rotateY`
- `static MObject rotateZ`
- `static MObject rotateOrder`
- `static MObject scale`
- `static MObject scaleX`
- `static MObject scaleY`
- `static MObject scaleZ`
- `static MObject shear`
- `static MObject shearXY`
- `static MObject shearXZ`
- `static MObject shearYZ`
- `static MObject rotatePivot`
- `static MObject rotatePivotX`
- `static MObject rotatePivotY`
- `static MObject rotatePivotZ`
- `static MObject rotatePivotTranslate`
- `static MObject rotatePivotTranslateX`
- `static MObject rotatePivotTranslateY`
- `static MObject rotatePivotTranslateZ`
- `static MObject scalePivot`
- `static MObject scalePivotX`
- `static MObject scalePivotY`
- `static MObject scalePivotZ`
- `static MObject scalePivotTranslate`
- `static MObject scalePivotTranslateX`
- `static MObject scalePivotTranslateY`
- `static MObject scalePivotTranslateZ`
- `static MObject rotateAxis`
- `static MObject rotateAxisX`
- `static MObject rotateAxisY`
- `static MObject rotateAxisZ`
- `static MObject transMinusRotatePivot`
- `static MObject transMinusRotatePivotX`
- `static MObject transMinusRotatePivotY`
- `static MObject transMinusRotatePivotZ`
- `static MObject minTransLimit`
- `static MObject minTransXLimit`
- `static MObject minTransYLimit`
- `static MObject minTransZLimit`
- `static MObject maxTransLimit`
- `static MObject maxTransXLimit`
- `static MObject maxTransYLimit`
- `static MObject maxTransZLimit`
- `static MObject minTransLimitEnable`
- `static MObject minTransXLimitEnable`
- `static MObject minTransYLimitEnable`
- `static MObject minTransZLimitEnable`
- `static MObject maxTransLimitEnable`
- `static MObject maxTransXLimitEnable`
- `static MObject maxTransYLimitEnable`
- `static MObject maxTransZLimitEnable`
- `static MObject minRotLimit`
- `static MObject minRotXLimit`
- `static MObject minRotYLimit`
- `static MObject minRotZLimit`
- `static MObject maxRotLimit`
- `static MObject maxRotXLimit`
- `static MObject maxRotYLimit`
- `static MObject maxRotZLimit`
- `static MObject minRotLimitEnable`
- `static MObject minRotXLimitEnable`
- `static MObject minRotYLimitEnable`
- `static MObject minRotZLimitEnable`
- `static MObject maxRotLimitEnable`
- `static MObject maxRotXLimitEnable`
- `static MObject maxRotYLimitEnable`
- `static MObject maxRotZLimitEnable`
- `static MObject minScaleLimit`
- `static MObject minScaleXLimit`
- `static MObject minScaleYLimit`
- `static MObject minScaleZLimit`
- `static MObject maxScaleLimit`
- `static MObject maxScaleXLimit`
- `static MObject maxScaleYLimit`
- `static MObject maxScaleZLimit`
- `static MObject minScaleLimitEnable`
- `static MObject minScaleXLimitEnable`
- `static MObject minScaleYLimitEnable`
- `static MObject minScaleZLimitEnable`
- `static MObject maxScaleLimitEnable`
- `static MObject maxScaleXLimitEnable`
- `static MObject maxScaleYLimitEnable`
- `static MObject maxScaleZLimitEnable`
- `static MObject geometry`
- `static MObject xformMatrix`
- `static MObject selectHandle`
- `static MObject selectHandleX`
- `static MObject selectHandleY`
- `static MObject selectHandleZ`
- `static MObject inheritsTransform`
- `static MObject displayHandle`
- `static MObject displayScalePivot`
- `static MObject displayRotatePivot`
- `static MObject displayLocalAxis`
- `static MObject dynamics`
- `static MObject showManipDefault`
- `static MObject specifiedManipLocation`
- `static MObject rotateQuaternion`
- `static MObject rotateQuaternionX`
- `static MObject rotateQuaternionY`
- `static MObject rotateQuaternionZ`
- `static MObject rotateQuaternionW`
- `static MObject rotationInterpolation`
- `static MObject offsetParentMatrix`

**Protected Member Functions**

- `virtual MVector applyTranslationLimits (const MVector &unclampedT, MDataBlock &, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyTranslationLocks (const MVector &toTest, const MVector &savedT, MStatus *ReturnStatus=NULL)`
- `virtual MEulerRotation applyRotationLimits (const MEulerRotation &unclampedR, MDataBlock &, MStatus *ReturnStatus=NULL)`
- `virtual MEulerRotation applyRotationLocks (const MEulerRotation &toTest, const MEulerRotation &savedR, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyScaleLimits (const MVector &unclampedS, MDataBlock &, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyScaleLocks (const MVector &toTest, const MVector &savedS, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyShearLocks (const MVector &toTest, const MVector &savedSh, MStatus *ReturnStatus=NULL)`
- `virtual MEulerRotation applyRotateOrientationLocks (const MEulerRotation &toTest, const MEulerRotation &savedRO, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyScaleLocksPivot (const MPoint &toTest, const MPoint &savedSP, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyRotatePivotLocks (const MPoint &toTest, const MPoint &savedRP, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyScaleLocksPivotTranslate (const MVector &toTest, const MVector &savedSPT, MStatus *ReturnStatus=NULL)`
- `virtual MVector applyRotatePivotLocksTranslate (const MVector &toTest, const MVector &savedRPT, MStatus *ReturnStatus=NULL)`
- `MStatus dirtyMatrix ()`
- `MStatus dirtyTranslation (const MVector &)`
- `MStatus dirtyRotation (const MEulerRotation &)`
- `MStatus dirtyScale (const MVector &)`
- `MStatus dirtyShear (const MVector &)`
- `MStatus dirtyRotateOrientation (const MEulerRotation &)`
- `MStatus dirtyScalePivot (const MPoint &)`
- `MStatus dirtyRotatePivot (const MPoint &)`
- `MStatus dirtyScalePivotTranslation (const MVector &)`
- `MStatus dirtyRotatePivotTranslation (const MVector &)`
- `virtual MStatus checkAndSetTranslation (MDataBlock &, const MPlug &, const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus checkAndSetRotation (MDataBlock &, const MPlug &, const MEulerRotation &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus checkAndSetScale (MDataBlock &, const MPlug &, const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus checkAndSetShear (MDataBlock &, const MPlug &, const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus checkAndSetRotateOrientation (MDataBlock &, const MPlug &, const MEulerRotation &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MStatus checkAndSetRotatePivot (MDataBlock &, const MPlug &, const MPoint &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MStatus checkAndSetRotatePivotTranslation (MDataBlock &, const MPlug &, const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus checkAndSetScalePivot (MDataBlock &, const MPlug &, const MPoint &, MSpace::Space=MSpace::kTransform, bool=true)`
- `virtual MStatus checkAndSetScalePivotTranslation (MDataBlock &, const MPlug &, const MVector &, MSpace::Space=MSpace::kTransform)`

### MPxTransformPreRotation

Module `OpenMaya`. Inherits `MPxTransform`. Source: [class_m_px_transform_pre_rotation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_transform_pre_rotation.html).

**Public Member Functions**

- `MPxTransformPreRotation ()`
- `MPxTransformPreRotation (const MPxTransformPreRotation &)`
- `MPxTransformPreRotation & operator= (const MPxTransformPreRotation &)`
- `~MPxTransformPreRotation () override`
- `virtual MQuaternion getPreRotation () const`

### MPxTransformationMatrix

Module `OpenMaya`. Source: [class_m_px_transformation_matrix.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_transformation_matrix.html).

**Public Member Functions**

- `MPxTransformationMatrix ()`
- `MPxTransformationMatrix (const MTransformationMatrix &)`
- `MPxTransformationMatrix (const MPxTransformationMatrix &)`
- `MPxTransformationMatrix (const MMatrix &)`
- `virtual ~MPxTransformationMatrix ()`
- `virtual void copyValues (MPxTransformationMatrix *)`
- `virtual MTypeId typeId () const`
- `MPxTransformationMatrix & operator= (const MPxTransformationMatrix &src)`
- `MPxTransformationMatrix & operator= (const MMatrix &src)`
- `MPxTransformationMatrix & operator= (const MTransformationMatrix &src)`
- `bool operator== (const MPxTransformationMatrix &)`
- `bool operator!= (const MPxTransformationMatrix &)`
- `bool operator== (const MTransformationMatrix &)`
- `bool operator!= (const MTransformationMatrix &)`
- `bool operator== (const MMatrix &)`
- `bool operator!= (const MMatrix &)`
- `virtual bool isEquivalent (const MPxTransformationMatrix &, double tolerance=MMatrix_kTol) const`
- `virtual bool isEquivalent (const MTransformationMatrix &, double tolerance=MMatrix_kTol) const`
- `virtual bool isEquivalent (const MMatrix &, double tolerance=MMatrix_kTol) const`
- `virtual MPxTransformationMatrix reverse () const`
- `virtual MPxTransformationMatrix & transformBy (const MTransformationMatrix &)`
- `virtual MMatrix asMatrix () const`
- `virtual MMatrix asMatrixInverse () const`
- `virtual MMatrix asScaleMatrix () const`
- `virtual MMatrix asScaleMatrixInverse () const`
- `virtual MMatrix asRotateMatrix () const`
- `virtual MMatrix asRotateMatrixInverse () const`
- `virtual MMatrix asMatrix (double percent) const`
- `virtual MMatrix asInterpolationMatrix (const MTransformationMatrix &toM, double percent, bool rot, int direction=0) const`
- `virtual MTransformationMatrix asTransformationMatrix () const`
- `virtual MVector translation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus translateTo (const MVector &vector, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus translateBy (const MVector &vector, MSpace::Space=MSpace::kTransform)`
- `virtual MQuaternion rotation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MEulerRotation eulerRotation (MSpace::Space space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus rotateTo (const MQuaternion &q, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateBy (const MQuaternion &q, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateTo (const MEulerRotation &e, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus rotateBy (const MEulerRotation &e, MSpace::Space=MSpace::kTransform)`
- `virtual MTransformationMatrix::RotationOrder rotationOrder (MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setRotationOrder (MTransformationMatrix::RotationOrder, bool preserve=true)`
- `virtual MQuaternion rotateOrientation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setRotateOrientation (const MQuaternion &q, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MEulerRotation eulerRotateOrientation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setRotateOrientation (const MEulerRotation &euler, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MVector scale (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus scaleTo (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus scaleBy (const MVector &, MSpace::Space=MSpace::kTransform)`
- `virtual MVector shear (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus shearTo (const MVector &shear, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus shearBy (const MVector &shear, MSpace::Space=MSpace::kTransform)`
- `virtual MPoint scalePivot (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setScalePivot (const MPoint &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MVector scalePivotTranslation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setScalePivotTranslation (const MVector &vector, MSpace::Space=MSpace::kTransform)`
- `virtual MPoint rotatePivot (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setRotatePivot (const MPoint &, MSpace::Space=MSpace::kTransform, bool balance=true)`
- `virtual MVector rotatePivotTranslation (MSpace::Space=MSpace::kTransform, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus setRotatePivotTranslation (const MVector &vector, MSpace::Space=MSpace::kTransform)`
- `virtual MStatus unSquishIt ()`
- `virtual MMatrix unSquishMatrix () const`

**Static Public Member Functions**

- `static MPxTransformationMatrix * creator ()`
- `static MEulerRotation::RotationOrder convertTransformationRotationOrder (MTransformationMatrix::RotationOrder, MStatus *ReturnStatus=NULL)`
- `static MTransformationMatrix::RotationOrder convertEulerRotationOrder (MEulerRotation::RotationOrder, MStatus *ReturnStatus=NULL)`
- `static MEulerRotation::RotationOrder convertRotationOrder (MTransformationMatrix::RotationOrder, MStatus *ReturnStatus=NULL)`
- `static MTransformationMatrix::RotationOrder convertRotationOrder (MEulerRotation::RotationOrder, MStatus *ReturnStatus=NULL)`

**Static Public Attributes**

- `static const MPxTransformationMatrix identity`
- `static MTypeId baseTransformationMatrixId = MTypeId(0x58A00001)`

**Protected Member Functions**

- `virtual MStatus decomposeMatrix (const MMatrix &)`

**Static Protected Member Functions**

- `static const char * className ()`

**Protected Attributes**

- `MVector scaleValue`
- `MEulerRotation rotationValue`
- `MVector translationValue`
- `MVector shearValue`
- `MPoint scalePivotValue`
- `MVector scalePivotTranslationValue`
- `MPoint rotatePivotValue`
- `MVector rotatePivotTranslationValue`
- `MQuaternion rotateOrientationValue`

**Friends**

- `std::ostream & operator<< (std::ostream &os, const MPxTransformationMatrix &m)`

### MPxTransformationMatrixPreRotation

Module `OpenMaya`. Inherits `MPxTransformationMatrix`. Source: [class_m_px_transformation_matrix_pre_rotation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_transformation_matrix_pre_rotation.html).

**Public Member Functions**

- `MPxTransformationMatrixPreRotation ()`
- `MPxTransformationMatrixPreRotation (const MTransformationMatrix &)`
- `MPxTransformationMatrixPreRotation (const MPxTransformationMatrixPreRotation &)`
- `MPxTransformationMatrixPreRotation (const MMatrix &)`
- `~MPxTransformationMatrixPreRotation () override`
- `virtual MQuaternion preRotation () const`

### MQuaternion

Module `OpenMaya`. Source: [class_m_quaternion.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_quaternion.html).

**Public Member Functions**

- `MQuaternion ()`
- `MQuaternion (const MQuaternion &src)`
- `MQuaternion (double xx, double yy, double zz, double ww)`
- `MQuaternion (const double q[4])`
- `MQuaternion (const MVector &a, const MVector &b)`
- `MQuaternion (const MVector &a, const MVector &b, double angleFactor)`
- `MQuaternion (double angle, const MVector &axisb)`
- `~MQuaternion ()`
- `MQuaternion & operator= (const MQuaternion &src)`
- `MQuaternion & operator= (const MMatrix &matrix)`
- `MQuaternion & operator= (const MEulerRotation &matrix)`
- `MMatrix asMatrix () const`
- `MEulerRotation asEulerRotation () const`
- `MQuaternion & setAxisAngle (const MVector &axis, double theta)`
- `bool getAxisAngle (MVector &axis, double &theta) const`
- `MQuaternion & setToXAxis (double theta)`
- `MQuaternion & setToYAxis (double theta)`
- `MQuaternion & setToZAxis (double theta)`
- `MStatus get (double dest[4]) const`
- `double operator[] (unsigned int i) const`
- `MQuaternion operator+ (const MQuaternion &other) const`
- `MQuaternion operator* (const MQuaternion &other) const`
- `MQuaternion & operator*= (const MQuaternion &rhs)`
- `MQuaternion operator- (const MQuaternion &other) const`
- `MQuaternion operator- () const`
- `MQuaternion & negateIt ()`
- `bool operator== (const MQuaternion &other) const`
- `bool operator!= (const MQuaternion &other) const`
- `bool isEquivalent (const MQuaternion &other, double tolerance=kQuaternionEpsilon) const`
- `MQuaternion & scaleIt (double scale)`
- `MQuaternion normal () const`
- `MQuaternion & normalizeIt ()`
- `MQuaternion conjugate () const`
- `MQuaternion & conjugateIt ()`
- `MQuaternion inverse () const`
- `MQuaternion & invertIt ()`
- `MQuaternion log () const`
- `MQuaternion exp () const`
- `double & operator[] (unsigned int i)`
- `operator MMatrix () const`

**Public Attributes**

- `double x`
- `double y`
- `double z`
- `double w`

**Static Public Attributes**

- `static const MQuaternion identity`

**Friends**

- `OPENMAYA_EXPORT MQuaternion operator* (double scale, const MQuaternion &other)`
- `OPENMAYA_EXPORT MQuaternion slerp (const MQuaternion &p, const MQuaternion &q, double t)`
- `OPENMAYA_EXPORT MQuaternion slerp (const MQuaternion &p, const MQuaternion &q, double t, short spin)`
- `OPENMAYA_EXPORT MQuaternion dslerp (const MQuaternion &p, const MQuaternion &q, double t, int dir)`
- `OPENMAYA_EXPORT MQuaternion squad (const MQuaternion &p, const MQuaternion &a, const MQuaternion &b, const MQuaternion &q, double t)`
- `OPENMAYA_EXPORT MQuaternion squad (const MQuaternion &p, const MQuaternion &a, const MQuaternion &b, const MQuaternion &q, double t, short spin)`
- `OPENMAYA_EXPORT MQuaternion squadPt (const MQuaternion &q0, const MQuaternion &q1, const MQuaternion &q2)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MQuaternion &q)`

### MRampAttribute

Module `OpenMaya`. Source: [class_m_ramp_attribute.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ramp_attribute.html).

**Public Types**

- `enum MInterpolation { kNone = 0, kLinear = 1, kSmooth = 2, kSpline = 3 }`

**Public Member Functions**

- `MRampAttribute ()`
- `MRampAttribute (const MRampAttribute &other)`
- `MRampAttribute (const MPlug &obj, MStatus *ReturnStatus=nullptr)`
- `MRampAttribute (const MObject &node, const MObject &attr, MStatus *ReturnStatus=nullptr)`
- `MRampAttribute & operator= (const MRampAttribute &other)`
- `~MRampAttribute ()`
- `unsigned int getNumEntries (MStatus *ReturnStatus=nullptr)`
- `void getEntries (MIntArray &indexes, MFloatArray &positions, MFloatArray &values, MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `void getEntries (MIntArray &indexes, MFloatArray &positions, MColorArray &colors, MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `void addEntries (MFloatArray &positions, MColorArray &values, MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `void addEntries (MFloatArray &positions, MFloatArray &values, MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `void deleteEntries (MIntArray &indexes, MStatus *ReturnStatus=nullptr)`
- `void setColorAtIndex (MColor &color, unsigned int index, MStatus *ReturnStatus=nullptr)`
- `void setValueAtIndex (float value, unsigned int index, MStatus *ReturnStatus=nullptr)`
- `void setPositionAtIndex (float position, unsigned int index, MStatus *ReturnStatus=nullptr)`
- `void setInterpolationAtIndex (MRampAttribute::MInterpolation interp, unsigned int index, MStatus *ReturnStatus=nullptr)`
- `bool isColorRamp (MStatus *ReturnStatus=nullptr) const`
- `bool isCurveRamp (MStatus *ReturnStatus=nullptr) const`
- `void getColorAtPosition (float position, MColor &color, MStatus *ReturnStatus=nullptr)`
- `void getValueAtPosition (float position, float &value, MStatus *ReturnStatus=nullptr)`
- `void sampleColorRamp (unsigned int numberOfSamples, MColorArray &colors, MStatus *ReturnStatus=nullptr)`
- `void sampleValueRamp (unsigned int numberOfSamples, MFloatArray &values, MStatus *ReturnStatus=nullptr)`
- `bool hasIndex (unsigned int index, MStatus *ReturnStatus=nullptr) const`
- `MStatus sort (bool ascending=true)`
- `MStatus pack ()`
- `MStatus setRamp (float value, float position=1.0f, MInterpolation interp=kLinear)`
- `MStatus setRamp (const MColor &color, float position=1.0f, MInterpolation interp=kLinear)`
- `MStatus setRamp (const MFloatArray &values, const MFloatArray &positions, const MIntArray &interps)`
- `MStatus setRamp (const MColorArray &values, const MFloatArray &positions, const MIntArray &interps)`
- `void getColorAtPosition (float position, MColor &color, MStatus *ReturnStatus, MDGContext &)`
- `void getValueAtPosition (float position, float &value, MStatus *ReturnStatus, MDGContext &)`
- `void sampleColorRamp (unsigned int numberOfSamples, MColorArray &colors, MStatus *ReturnStatus, MDGContext &)`
- `void sampleValueRamp (unsigned int numberOfSamples, MFloatArray &values, MStatus *ReturnStatus, MDGContext &)`

**Static Public Member Functions**

- `static MObject createCurveRamp (const MString &attrLongName, const MString &attrShortName, MStatus *ReturnStatus=nullptr)`
- `static MObject createColorRamp (const MString &attrLongName, const MString &attrShortName, MStatus *ReturnStatus=nullptr)`
- `static MObject createRamp (const MString &attrLongName, const MString &attrShortName, const MObject &node, float val, float pos=1.f, MInterpolation interp=kLinear, MStatus *ReturnStatus=nullptr)`
- `static MObject createRamp (const MString &attrLongName, const MString &attrShortName, const MObject &node, const MColor &color, float pos=1.0f, MInterpolation interp=kLinear, MStatus *ReturnStatus=nullptr)`
- `static MObject createRamp (const MString &attrLongName, const MString &attrShortName, const MObject &node, const MFloatArray &values, const MFloatArray &positions, const MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `static MObject createRamp (const MString &attrLongName, const MString &attrShortName, const MObject &node, const MColorArray &colors, const MFloatArray &positions, const MIntArray &interps, MStatus *ReturnStatus=nullptr)`
- `static const char * className ()`

### MRichSelection

Module `OpenMaya`. Source: [class_m_rich_selection.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_rich_selection.html).

**Public Member Functions**

- `MRichSelection ()`
- `MRichSelection (const MRichSelection &src)`
- `virtual ~MRichSelection ()`
- `MStatus getSelection (MSelectionList &selection) const`
- `MStatus getSymmetry (MSelectionList &symmetry) const`
- `MStatus getSymmetryMatrix (MMatrix &symmetryMatrix, MSpace::Space &space) const`
- `MStatus getSymmetryMatrix (const MDagPath &path, MSpace::Space space, MMatrix &symmetryMatrix) const`
- `MStatus getSymmetryPlane (const MDagPath &path, MSpace::Space space, MPlane &plane) const`
- `MStatus clear ()`
- `MStatus setSelection (const MSelectionList &selection)`

**Static Public Member Functions**

- `static const char * className ()`

### MSceneMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_scene_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_scene_message.html).

**Public Types**

- `enum Message { kSceneUpdate, kBeforeNew, kAfterNew, kBeforeImport, kAfterImport, kBeforeOpen, kAfterOpen, kBeforeFileRead, kAfterFileRead, kAfterSceneReadAndRecordEdits, kBeforeExport, kAfterExport, kBeforeSave, kAfterSave, kBeforeReference, kAfterReference, kBeforeRemoveReference, kAfterRemoveReference, kBeforeImportReference, kAfterImportReference, kBeforeExportReference, kAfterExportReference, kBeforeUnloadReference, kAfterUnloadReference, kBeforeSoftwareRender, kAfterSoftwareRender, kBeforeSoftwareFrameRender, kAfterSoftwareFrameRender, kSoftwareRenderInterrupted, kMayaInitialized, kMayaExiting, kBeforeNewCheck, kBeforeOpenCheck, kBeforeSaveCheck, kBeforeImportCheck, kBeforeExportCheck, kBeforeLoadReference, kAfterLoadReference, kBeforeLoadReferenceCheck, kBeforeReferenceCheck, kBeforeCreateReferenceCheck = kBeforeReferenceCheck, kBeforePluginLoad, kAfterPluginLoad, kBeforePluginUnload, kAfterPluginUnload, kBeforeCreateReference, kAfterCreateReference, kExportStarted, kBeforeLoadReferenceAndRecordEdits, kAfterLoadReferenceAndRecordEdits, kBeforeCreateReferenceAndRecordEdits, kAfterCreateReferenceAndRecordEdits, kLast }`

**Static Public Member Functions**

- `static MCallbackId addCallback (Message msg, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCheckCallback (Message msg, MMessage::MCheckFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCheckFileCallback (Message msg, MMessage::MCheckFileFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addStringArrayCallback (Message msg, MMessage::MStringArrayFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addReferenceCallback (Message msg, MMessage::MObjectFileFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCheckReferenceCallback (Message msg, MMessage::MCheckObjectFileFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addConnectionFailedCallback (MMessage::MConnFailFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addNamespaceRenamedCallback (MMessage::MTwoStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`
- `static MCallbackId addCallback (Message, void(*func)(bool *retCode, void *clientData), void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCallback (Message, void(*func)(bool *retCode, MFileObject &file, void *clientData), void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCallback (Message, void(*func)(const MStringArray &, void *), void *clientData, MStatus *ReturnStatus)`

### MScriptUtil

Module `OpenMaya`. Source: [class_m_script_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_script_util.html).

**Public Member Functions**

- `MScriptUtil ()`
- `~MScriptUtil ()`
- `MScriptUtil (int value)`
- `MScriptUtil (short value)`
- `MScriptUtil (float value)`
- `MScriptUtil (double value)`
- `MScriptUtil (unsigned int value)`
- `MScriptUtil (bool value)`
- `MScriptUtil (const int *value)`
- `MScriptUtil (const short *value)`
- `MScriptUtil (const float *value)`
- `MScriptUtil (const double *value)`
- `MScriptUtil (const unsigned int *value)`
- `MScriptUtil (const bool *value)`
- `MScriptUtil (const MIntArray &array)`
- `MScriptUtil (const MDoubleArray &array)`
- `MScriptUtil (const MUintArray &array)`
- `MScriptUtil (const MUint64Array &array)`
- `void createFromInt (int x, int y=0, int z=0, int w=0)`
- `void createFromDouble (double x, double y=0, double z=0, double w=0)`
- `bool createFromList (void *pythonList, unsigned int length)`
- `int asInt ()`
- `short asShort ()`
- `float asFloat ()`
- `double asDouble ()`
- `unsigned int asUint ()`
- `bool asBool ()`
- `int * asIntPtr ()`
- `short * asShortPtr ()`
- `unsigned short * asUshortPtr ()`
- `float * asFloatPtr ()`
- `double * asDoublePtr ()`
- `unsigned int * asUintPtr ()`
- `bool * asBoolPtr ()`
- `char * asCharPtr ()`
- `unsigned char * asUcharPtr ()`
- `Mint2 * asInt2Ptr ()`
- `Mint3 * asInt3Ptr ()`
- `Mint4 * asInt4Ptr ()`
- `Mshort2 * asShort2Ptr ()`
- `Mshort3 * asShort3Ptr ()`
- `Mshort4 * asShort4Ptr ()`
- `Mfloat2 * asFloat2Ptr ()`
- `Mfloat3 * asFloat3Ptr ()`
- `Mfloat4 * asFloat4Ptr ()`
- `Mdouble2 * asDouble2Ptr ()`
- `Mdouble3 * asDouble3Ptr ()`
- `Mdouble4 * asDouble4Ptr ()`
- `Muint2 * asUint2Ptr ()`
- `Muint3 * asUint3Ptr ()`
- `Muint4 * asUint4Ptr ()`

**Static Public Member Functions**

- `static void setInt (int &var, int value)`
- `static void setShort (short &var, int value)`
- `static void setFloat (float &var, double value)`
- `static void setDouble (double &var, double value)`
- `static void setUint (unsigned int &var, int value)`
- `static void setBool (bool &var, int value)`
- `static void setChar (char &var, int value)`
- `static void setUchar (unsigned char &var, int value)`
- `static int getInt (int &var)`
- `static int getShort (short &var)`
- `static double getFloat (float &var)`
- `static double getDouble (double &var)`
- `static int getUint (unsigned int &var)`
- `static int getBool (bool &var)`
- `static int getChar (char &var)`
- `static int getUchar (unsigned char &var)`
- `static bool setIntArray (int *var, unsigned int index, int value)`
- `static bool setShortArray (short *var, unsigned int index, int value)`
- `static bool setFloatArray (float *var, unsigned int index, double value)`
- `static bool setDoubleArray (double *var, unsigned int index, double value)`
- `static bool setUintArray (unsigned int *var, unsigned int index, int value)`
- `static bool setUshortArray (unsigned short *var, unsigned int index, int value)`
- `static bool setBoolArray (bool *var, unsigned int index, int value)`
- `static bool setCharArray (char *var, unsigned int index, int value)`
- `static bool setUcharArray (unsigned char *var, unsigned int index, int value)`
- `static int getIntArrayItem (int *var, unsigned int index)`
- `static int getShortArrayItem (short *var, unsigned int index)`
- `static double getFloatArrayItem (float *var, unsigned int index)`
- `static double getDoubleArrayItem (double *var, unsigned int index)`
- `static int getUintArrayItem (unsigned int *var, unsigned int index)`
- `static int getUshortArrayItem (unsigned short *var, unsigned int index)`
- `static int getBoolArrayItem (bool *var, unsigned int index)`
- `static int getCharArrayItem (char *var, unsigned int index)`
- `static int getUcharArrayItem (unsigned char *var, unsigned int index)`
- `static bool createMatrixFromList (void *pythonList, MMatrix &matrix)`
- `static bool createFloatMatrixFromList (void *pythonList, MFloatMatrix &matrix)`
- `static bool createIntArrayFromList (void *pythonList, MIntArray &array)`
- `static bool createFloatArrayFromList (void *pythonList, MFloatArray &array)`
- `static int getInt2ArrayItem (int(*var)[2], unsigned int r, unsigned int c)`
- `static bool setInt2ArrayItem (int(*var)[2], unsigned int r, unsigned int c, int value)`
- `static int getInt3ArrayItem (int(*var)[3], unsigned int r, unsigned int c)`
- `static bool setInt3ArrayItem (int(*var)[3], unsigned int r, unsigned int c, int value)`
- `static int getInt4ArrayItem (int(*var)[4], unsigned int r, unsigned int c)`
- `static bool setInt4ArrayItem (int(*var)[4], unsigned int r, unsigned int c, int value)`
- `static short getShort2ArrayItem (short(*var)[2], unsigned int r, unsigned int c)`
- `static bool setShort2ArrayItem (short(*var)[2], unsigned int r, unsigned int c, short value)`
- `static short getShort3ArrayItem (short(*var)[3], unsigned int r, unsigned int c)`
- `static bool setShort3ArrayItem (short(*var)[3], unsigned int r, unsigned int c, short value)`
- `static short getShort4ArrayItem (short(*var)[4], unsigned int r, unsigned int c)`
- `static bool setShort4ArrayItem (short(*var)[4], unsigned int r, unsigned int c, short value)`
- `static float getFloat2ArrayItem (float(*var)[2], unsigned int r, unsigned int c)`
- `static bool setFloat2ArrayItem (float(*var)[2], unsigned int r, unsigned int c, float value)`
- `static float getFloat3ArrayItem (float(*var)[3], unsigned int r, unsigned int c)`
- `static bool setFloat3ArrayItem (float(*var)[3], unsigned int r, unsigned int c, float value)`
- `static float getFloat4ArrayItem (float(*var)[4], unsigned int r, unsigned int c)`
- `static bool setFloat4ArrayItem (float(*var)[4], unsigned int r, unsigned int c, float value)`
- `static double getDouble2ArrayItem (double(*var)[2], unsigned int r, unsigned int c)`
- `static bool setDouble2ArrayItem (double(*var)[2], unsigned int r, unsigned int c, double value)`
- `static double getDouble3ArrayItem (double(*var)[3], unsigned int r, unsigned int c)`
- `static bool setDouble3ArrayItem (double(*var)[3], unsigned int r, unsigned int c, double value)`
- `static double getDouble4ArrayItem (double(*var)[4], unsigned int r, unsigned int c)`
- `static bool setDouble4ArrayItem (double(*var)[4], unsigned int r, unsigned int c, double value)`
- `static unsigned int getUint2ArrayItem (unsigned int(*var)[2], unsigned int r, unsigned int c)`
- `static bool setUint2ArrayItem (unsigned int(*var)[2], unsigned int r, unsigned int c, unsigned int value)`
- `static unsigned int getUint3ArrayItem (unsigned int(*var)[3], unsigned int r, unsigned int c)`
- `static bool setUint3ArrayItem (unsigned int(*var)[3], unsigned int r, unsigned int c, unsigned int value)`
- `static unsigned int getUint4ArrayItem (unsigned int(*var)[4], unsigned int r, unsigned int c)`
- `static bool setUint4ArrayItem (unsigned int(*var)[4], unsigned int r, unsigned int c, unsigned int value)`

### MSelectionList

Module `OpenMaya`. Source: [class_m_selection_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_selection_list.html).

**Public Types**

- `enum MergeStrategy { kMergeNormal =0, kXORWithList, kRemoveFromList }`

**Public Member Functions**

- `MSelectionList ()`
- `MSelectionList (const MSelectionList &src)`
- `virtual ~MSelectionList ()`
- `MStatus clear ()`
- `bool isEmpty (MStatus *ReturnStatus=NULL) const`
- `unsigned int length (MStatus *ReturnStatus=NULL) const`
- `MStatus getDependNode (unsigned int index, MObject &depNode) const`
- `MStatus getDagPath (unsigned int index, MDagPath &dagPath, MObject &component=MObject::kNullObj) const`
- `MStatus getPlug (unsigned int index, MPlug &plug) const`
- `MStatus getPlug (unsigned int index, bool convertComponents, MPlug &plug) const`
- `MStatus add (const MObject &object, const bool mergeWithExisting=false)`
- `MStatus add (const MDagPath &object, const MObject &component=MObject::kNullObj, const bool mergeWithExisting=false)`
- `MStatus add (const MString &matchString, const bool searchChildNamespacesToo=false)`
- `MStatus add (const MPlug &plug, const bool mergeWithExisting=false)`
- `MStatus add (const MUuid &uuid, bool mergeWithExisting=false)`
- `MStatus remove (unsigned int index)`
- `MStatus replace (unsigned int index, const MObject &item)`
- `MStatus replace (unsigned int index, const MDagPath &item, const MObject &component=MObject::kNullObj)`
- `MStatus replace (unsigned int index, const MPlug &plug)`
- `bool hasItem (const MObject &item, MStatus *ReturnStatus=NULL) const`
- `bool hasItem (const MDagPath &item, const MObject &component=MObject::kNullObj, MStatus *ReturnStatus=NULL) const`
- `bool hasItem (const MPlug &plug, MStatus *ReturnStatus=NULL) const`
- `bool hasItemPartly (const MDagPath &item, const MObject &component, MStatus *ReturnStatus=NULL) const`
- `MStatus toggle (const MDagPath &item, const MObject &component=MObject::kNullObj)`
- `MSelectionList & operator= (const MSelectionList &other)`
- `MStatus merge (const MSelectionList &other, const MergeStrategy strategy=kMergeNormal)`
- `MStatus merge (const MDagPath &object, const MObject &component=MObject::kNullObj, const MergeStrategy strategy=kMergeNormal)`
- `MStatus intersect (const MSelectionList &other, bool expandToLeaves=false)`
- `MStatus getSelectionStrings (MStringArray &array) const`
- `MStatus getSelectionStrings (unsigned int index, MStringArray &array) const`

**Static Public Member Functions**

- `static const char * className ()`

### MSetAttrEdit

Module `OpenMaya`. Inherits `MEdit`. Source: [class_m_set_attr_edit.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_set_attr_edit.html).

**Public Member Functions**

- `MPlug plug (MStatus *ReturnStatus=NULL) const`
- `MString plugName (MStatus *ReturnStatus=NULL) const`
- `EditType editType (MStatus *ReturnStatus=NULL) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MItEdits`

### MSharedPtr&lt; T &gt;

Module `OpenMaya`. Source: [class_m_shared_ptr.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_shared_ptr.html).

**Public Member Functions**

- `T * get () const`
- `T * operator-> () const`
- `decltype(auto) operator* () const`
- `operator bool () const`
- `constexpr MSharedPtr () noexcept`
- `constexpr MSharedPtr (std::nullptr_t _ptr) noexcept`
- `MSharedPtr (const MSharedPtr &_rhs) noexcept`
- `MSharedPtr (MSharedPtr &&_rhs) noexcept`
- `MSharedPtr & operator= (const MSharedPtr &_rhs) noexcept`
- `MSharedPtr & operator= (MSharedPtr &&_rhs) noexcept`
- `void reset () noexcept`

### MSpinLock

Module `OpenMaya`. Source: [class_m_spin_lock.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_spin_lock.html).

**Public Member Functions**

- `MSpinLock ()`
- `~MSpinLock ()`
- `void lock ()`
- `void unlock ()`
- `bool tryLock ()`

### MStatus

Module `OpenMaya`. Source: [class_m_status.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_status.html).

**Public Types**

- `enum MStatusCode { kSuccess = 0, kFailure, kInsufficientMemory, kInvalidParameter, kLicenseFailure, kUnknownParameter, kNotImplemented, kNotFound, kEndOfFile }`

**Public Member Functions**

- `MStatus ()`
- `MStatus (MStatusCode)`
- `MStatus (const MStatus &)`
- `MStatus & operator= (const MStatus &rhs)`
- `bool operator== (const MStatus &rhs) const`
- `bool operator== (const MStatusCode rhs) const`
- `bool operator!= (const MStatus &rhs) const`
- `bool operator!= (const MStatusCode rhs) const`
- `bool error () const`
- `void clear ()`
- `MStatusCode statusCode () const`
- `MString errorString () const`
- `void perror (const char *) const`
- `void pAPIerror (const char *file, int line) const`
- `void perror (const MString &) const`
- `void set (bool status, unsigned char statusCode, unsigned char internalStatusCode)`
- `void setSuccess ()`
- `operator bool () const`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &, MStatus &)`
- `OPENMAYA_EXPORT bool operator== (const MStatus::MStatusCode code, const MStatus &status)`
- `OPENMAYA_EXPORT bool operator!= (const MStatus::MStatusCode code, const MStatus &status)`

### MStreamUtils

Module `OpenMaya`. Source: [class_m_stream_utils.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_stream_utils.html).

**Static Public Member Functions**

- `static std::ostream & stdErrorStream ()`
- `static std::ostream & stdOutStream ()`
- `static MStatus writeChar (std::ostream &out, const char value, bool binary=false)`
- `static MStatus writeCharBuffer (std::ostream &out, const char *value, bool binary=false)`
- `static MStatus writeInt (std::ostream &out, const int value, bool binary=false)`
- `static MStatus writeFloat (std::ostream &out, const float value, bool binary=false)`
- `static MStatus writeDouble (std::ostream &out, const double value, bool binary=false)`
- `static MStatus readChar (std::istream &in, char &value, bool binary=false)`
- `static MStatus readCharBuffer (std::istream &in, char *&value, unsigned int length, bool binary=false)`
- `static MStatus readInt (std::istream &in, int &value, bool binary=false)`
- `static MStatus readFloat (std::istream &in, float &value, bool binary=false)`
- `static MStatus readDouble (std::istream &in, double &value, bool binary=false)`

### MString

Module `OpenMaya`. Source: [class_m_string.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_string.html).

**Public Member Functions**

- `MString ()`
- `MString (const char *charString)`
- `MString (const wchar_t *charString)`
- `MString (const char *charString, int charLength)`
- `MString (const wchar_t *charString, int wcharLength)`
- `MString (const MString &other)`
- `~MString ()`
- `MString & operator+= (const MString &other)`
- `MString & operator+= (const char *other)`
- `MString & operator+= (double other)`
- `MString & operator+= (int other)`
- `MString & operator+= (unsigned int other)`
- `MString & operator+= (float other)`
- `MString & operator= (const MString &other)`
- `MString & operator= (const char *other)`
- `MString & operator= (double value)`
- `bool operator== (const MString &other) const`
- `bool operator== (const char *other) const`
- `bool operator!= (const MString &other) const`
- `bool operator!= (const char *other) const`
- `MString operator+ (const MString &other) const`
- `MString operator+ (const char *other) const`
- `MString operator+ (double value) const`
- `MStatus set (const char *charString)`
- `MStatus setUTF8 (const char *utf8String)`
- `MStatus setWChar (const wchar_t *wcharString)`
- `MStatus set (const char *charString, int charLength)`
- `MStatus setWChar (const wchar_t *wcharString, int wcharLength)`
- `MStatus set (double value)`
- `MStatus set (double value, int precision)`
- `const char * asChar () const`
- `const char * asChar (int &charLength) const`
- `const char * asUTF8 () const`
- `const char * asUTF8 (int &utf8Length) const`
- `const wchar_t * asWChar () const`
- `const wchar_t * asWChar (int &numChars) const`
- `unsigned int length () const`
- `unsigned int numChars () const`
- `void clear ()`
- `int index (char) const`
- `int indexW (char) const`
- `int indexW (wchar_t) const`
- `int indexW (const MString &) const`
- `int rindex (char) const`
- `int rindexW (char) const`
- `int rindexW (wchar_t) const`
- `int rindexW (const MString &) const`
- `MStatus split (char, MStringArray &) const`
- `MStatus split (wchar_t, MStringArray &) const`
- `MString substring (int start, int end) const`
- `MString substringW (int start, int end) const`
- `MString & toUpperCase ()`
- `MString & toLowerCase ()`
- `bool isShort () const`
- `short asShort () const`
- `bool isInt () const`
- `int asInt () const`
- `bool isUnsigned () const`
- `unsigned int asUnsigned () const`
- `bool isFloat () const`
- `float asFloat () const`
- `bool isDouble () const`
- `double asDouble () const`
- `MString expandEnvironmentVariablesAndTilde (MStatus *ReturnStatus=NULL) const`
- `MString expandFilePath (MStatus *ReturnStatus=NULL) const`
- `MStatus format (const MString &fmt, const MStringArray &args)`
- `MStatus format (const MString &fmt, const MString &arg1=MString::argUnused, const MString &arg2=MString::argUnused, const MString &arg3=MString::argUnused, const MString &arg4=MString::argUnused, const MString &arg5=MString::argUnused, const MString &arg6=MString::argUnused, const MString &arg7=MString::argUnused, const MString &arg8=MString::argUnused, const MString &arg9=MString::argUnused, const MString &arg10=MString::argUnused)`
- `MStatus substitute (const MString &src, const MString &target)`
- `MStatus substituteFirst (const MString &src, const MString &target)`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &, const MString &)`
- `OPENMAYA_EXPORT MString operator+ (const char *, const MString &)`

### MStringArray

Module `OpenMaya`. Source: [class_m_string_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_string_array.html).

**Public Member Functions**

- `MStringArray ()`
- `MStringArray (const MStringArray &other)`
- `MStringArray (const MString strings[], unsigned int count)`
- `MStringArray (const char *strings[], unsigned int count)`
- `MStringArray (unsigned int initialSize, const MString &initialValue)`
- `~MStringArray ()`
- `MString operator[] (unsigned int index) const`
- `MStringArray & operator= (const MStringArray &other)`
- `MStatus set (const MString &element, unsigned int index)`
- `MStatus set (char *element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MString &element, unsigned int index)`
- `MStatus append (const MString &element)`
- `MStatus clear ()`
- `MStatus get (MString array[]) const`
- `MStatus get (char *array[]) const`
- `int indexOf (const MString &element) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MString & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MStringArray &array)`

### MStringResource

Module `OpenMaya`. Source: [class_m_string_resource.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_string_resource.html).

**Public Member Functions**

- `MStringResource ()`
- `~MStringResource ()`

**Static Public Member Functions**

- `static const MString getString (const MStringResourceId &id, MStatus &status)`
- `static MStatus registerString (const MStringResourceId &id)`

### MStringResourceId

Module `OpenMaya`. Source: [class_m_string_resource_id.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_string_resource_id.html).

**Public Member Functions**

- `MStringResourceId ()`
- `MStringResourceId (const char *pluginId, const char *keyString, const MString &defaultValue)`
- `MStringResourceId (const MStringResourceId &)`
- `MStringResourceId & operator= (const MStringResourceId &)`
- `bool operator!= (const MStringResourceId &) const`
- `bool operator== (const MStringResourceId &) const`
- `~MStringResourceId ()`

### MSyntax

Module `OpenMaya`. Source: [class_m_syntax.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_syntax.html).

**Public Types**

- `enum MArgType { kInvalidArgType, kNoArg, kBoolean, kLong, kDouble, kString, kUnsigned, kDistance, kAngle, kTime, kSelectionItem, kLastArgType }`
- `enum MObjectFormat { kInvalidObjectFormat, kNone, kStringObjects, kSelectionList, kLastObjectFormat }`

**Public Member Functions**

- `MSyntax ()`
- `MSyntax (const MSyntax &other)`
- `virtual ~MSyntax ()`
- `MSyntax & operator= (const MSyntax &rhs)`
- `MStatus addFlag (const char *shortName, const char *longName, MArgType argType1=kNoArg, MArgType argType2=kNoArg, MArgType argType3=kNoArg, MArgType argType4=kNoArg, MArgType argType5=kNoArg, MArgType argType6=kNoArg)`
- `MStatus makeFlagMultiUse (const char *flag)`
- `MStatus makeFlagQueryWithFullArgs (const char *flag, bool queryArgsAreOptional)`
- `MStatus addArg (MArgType arg)`
- `void useSelectionAsDefault (bool useSelectionList=false)`
- `MStatus setObjectType (MObjectFormat objectFormat, unsigned int minimumObjects=0)`
- `MStatus setObjectType (MObjectFormat objectFormat, unsigned int minimumObjects, unsigned int maximumObjects)`
- `void setMinObjects (unsigned int minimumObjectCount)`
- `void setMaxObjects (unsigned int maximumObjectCount)`
- `void enableQuery (bool supportsQuery=true)`
- `void enableEdit (bool supportsEdit=true)`
- `unsigned int minObjects () const`
- `unsigned int maxObjects () const`
- `bool canQuery () const`
- `bool canEdit () const`

**Static Public Member Functions**

- `static const char * className ()`

### MTemplateAction&lt; ActionClass, CommandName, CommandSyntax &gt;

Module `OpenMaya`. Inherits `MPxCommand`. Source: [class_m_template_action.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_action.html).

**Public Member Functions**

- `MTemplateAction ()`
- `MStatus doIt (const MArgList &) override`

**Static Public Member Functions**

- `static void * creator ()`
- `static MStatus registerCommand (MObject obj)`
- `static MStatus deregisterCommand (MObject obj)`

### MTemplateCommand&lt; CommandClass, CommandName, CommandSyntax &gt;

Module `OpenMaya`. Inherits `MTemplateAction< CommandClass, CommandName, CommandSyntax >`. Source: [class_m_template_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_command.html).

**Public Member Functions**

- `bool isUndoable () const override`

### MTemplateCreateNodeCommand&lt; CommandClass, CommandName, NodeName &gt;

Module `OpenMaya`. Inherits `MTemplateCommand< CommandClass, CommandName, MTemplateCreateNodeCommand_newSyntax >`. Source: [class_m_template_create_node_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_create_node_command.html).

**Public Member Functions**

- `MStatus doIt (const MArgList &args) override`
- `MStatus redoIt () override`
- `MStatus undoIt () override`

### MTesselationParams

Module `OpenMaya`. Source: [class_m_tesselation_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_tesselation_params.html).

**Public Types**

- `enum TessFormat { kTriangleCountFormat, kStandardFitFormat, kGeneralFormat }`
- `enum PolyType { kTriangles, kQuads }`
- `enum IsoparmType { kSurface3DDistance, kSurface3DEquiSpaced, kSurfaceEquiSpaced, kSpanEquiSpaced }`
- `enum SubdivisionType { kUseFractionalTolerance, kUseChordHeightRatio, kUseMinEdgeLength, kUseMaxEdgeLength, kUseMaxNumberPolys, kUseMaxSubdivisionLevel, kUseMinScreenSize, kUseMaxUVRectangleSize, kUseTriangleEdgeSwapping, kUseRelativeTolerance, kUseEdgeSmooth, kLastFlag }`

**Public Member Functions**

- `MTesselationParams (TessFormat format=kStandardFitFormat, PolyType=kTriangles)`
- `MTesselationParams (const MTesselationParams &)`
- `~MTesselationParams ()`
- `void setFormatType (TessFormat type)`
- `void setOutputType (PolyType type)`
- `void setTriangleCount (int)`
- `void setStdChordHeightRatio (double)`
- `void setStdFractionalTolerance (double)`
- `void setStdMinEdgeLength (double)`
- `void setSubdivisionFlag (SubdivisionType type, bool use)`
- `void setFitTolerance (double)`
- `void setChordHeightRatio (double)`
- `void setMinEdgeLength (double)`
- `void setMaxEdgeLength (double)`
- `void setMaxNumberPolys (int)`
- `void setMaxSubdivisionLevel (double)`
- `void setMinScreenSize (double, double)`
- `void setWorldspaceToScreenTransform (MMatrix &)`
- `void setMaxUVRectangleSize (double, double)`
- `void setRelativeFitTolerance (double)`
- `void setEdgeSmoothFactor (double)`
- `void set3DDelta (double)`
- `void setUIsoparmType (IsoparmType type)`
- `void setVIsoparmType (IsoparmType type)`
- `void setUNumber (int count)`
- `void setVNumber (int count)`
- `void setBoundingBoxDiagonal (double distance)`
- `void setUDistanceFraction (double value)`
- `void setVDistanceFraction (double value)`
- `MTesselationParams & operator= (const MTesselationParams &rhs)`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MFnNurbsSurface`

### MThreadAsync

Module `OpenMaya`. Source: [class_m_thread_async.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_thread_async.html).

**Static Public Member Functions**

- `static MStatus init ()`
- `static MStatus createTask (MThreadFunc func, void *data, MThreadAsyncCallbackFunc afunc, MThreadRootTask *root)`
- `static void release ()`
- `static const char * className ()`

### MThreadPool

Module `OpenMaya`. Source: [class_m_thread_pool.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_thread_pool.html).

**Static Public Member Functions**

- `static MStatus init ()`
- `static MStatus newParallelRegion (MThreadCallbackFunc func, void *data)`
- `static MStatus createTask (MThreadFunc func, void *data, MThreadRootTask *root)`
- `static MStatus executeAndJoin (MThreadRootTask *root)`
- `static void release ()`
- `static const char * className ()`

### MThreadUtils

Module `OpenMaya`. Source: [class_m_thread_utils.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_thread_utils.html).

**Static Public Member Functions**

- `static int getNumThreads ()`
- `static void syncNumOpenMPThreads ()`
- `static int getCacheLineSize ()`

### MTime

Module `OpenMaya`. Source: [class_m_time.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_time.html).

**Public Types**

- `enum Unit { kInvalid, kHours, kMinutes, kSeconds, kMilliseconds, kGames, k15FPS = kGames, kFilm, k24FPS = kFilm, kPALFrame, k25FPS = kPALFrame, kNTSCFrame, k30FPS = kNTSCFrame, kShowScan, k48FPS = kShowScan, kPALField, k50FPS = kPALField, kNTSCField, k60FPS = kNTSCField, k2FPS, k3FPS, k4FPS, k5FPS, k6FPS, k8FPS, k10FPS, k12FPS, k16FPS, k20FPS, k40FPS, k75FPS, k80FPS, k100FPS, k120FPS, k125FPS, k150FPS, k200FPS, k240FPS, k250FPS, k300FPS, k375FPS, k400FPS, k500FPS, k600FPS, k750FPS, k1200FPS, k1500FPS, k2000FPS, k3000FPS, k6000FPS, k23_976FPS, k29_97FPS, k29_97DF, k47_952FPS, k59_94FPS, k44100FPS, k48000FPS, k90FPS, kUserDef, kLast }`

**Public Member Functions**

- `MTime ()`
- `MTime (const MTime &)`
- `MTime (double time_val, Unit=kFilm)`
- `~MTime ()`
- `Unit unit () const`
- `double value () const`
- `MStatus setUnit (Unit new_unit)`
- `MStatus setValue (double new_value)`
- `double as (Unit other_unit) const`
- `double asUnits (Unit other_unit) const`
- `MTime & operator= (const MTime &rhs)`
- `bool operator== (const MTime &rhs) const`
- `bool operator!= (const MTime &rhs) const`
- `bool operator<= (const MTime &rhs) const`
- `bool operator>= (const MTime &rhs) const`
- `bool operator< (const MTime &rhs) const`
- `bool operator> (const MTime &rhs) const`
- `MTime operator+ (const MTime &rhs) const`
- `MTime & operator+= (const MTime &rhs)`
- `MTime operator+ (double rhs) const`
- `MTime & operator+= (double rhs)`
- `MTime operator- (const MTime &rhs) const`
- `MTime & operator-= (const MTime &rhs)`
- `MTime operator- (double rhs) const`
- `MTime & operator-= (double rhs)`
- `MTime operator* (double rhs) const`
- `MTime & operator*= (double rhs)`
- `MTime operator/ (double rhs) const`
- `MTime & operator/= (double rhs)`
- `MTime & operator++ ()`
- `MTime & operator++ (int)`
- `MTime & operator-- ()`
- `MTime & operator-- (int)`

**Static Public Member Functions**

- `static Unit uiUnit ()`
- `static MStatus setUIUnit (Unit new_unit)`
- `static const char * className ()`
- `static MTick ticksPerSecond ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MTime &t)`

### MTimeArray

Module `OpenMaya`. Source: [class_m_time_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_time_array.html).

**Public Member Functions**

- `MTimeArray ()`
- `MTimeArray (const MTimeArray &other)`
- `MTimeArray (const MTime src[], unsigned int count)`
- `MTimeArray (unsigned int initialSize, const MTime &initialValue)`
- `~MTimeArray ()`
- `const MTime & operator[] (unsigned int index) const`
- `MStatus set (const MTime &element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MTime &element, unsigned int index)`
- `MStatus append (const MTime &element)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MTime & operator[] (unsigned int index)`
- `MStatus get (MTime array[]) const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`
- `MTimeArray & operator= (const MTimeArray &)`
- `MStatus copy (const MTimeArray &source)`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MTimeArray &array)`

### MTimer

Module `OpenMaya`. Source: [class_m_timer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_timer.html).

**Public Member Functions**

- `MTimer ()`
- `MTimer (const MTimer &)`
- `~MTimer ()`
- `void beginTimer ()`
- `void endTimer ()`
- `double elapsedTime ()`
- `MTimer & operator= (const MTimer &rhs)`
- `bool operator== (const MTimer &rhs) const`
- `bool operator!= (const MTimer &rhs) const`
- `void clear ()`

### MTimerMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_timer_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_timer_message.html).

**Public Types**

- `typedef void(* sleepCallbackPtr) ()`

**Static Public Member Functions**

- `static MCallbackId addTimerCallback (float period, MMessage::MElapsedTimeFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`
- `static MStatus setSleepCallback (sleepCallbackPtr sleepFunc)`
- `static MTimerMessage::sleepCallbackPtr sleepCallback ()`

### MTopologyClusterNode

Module `OpenMaya`. Source: [class_m_topology_cluster_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_topology_cluster_node.html).

**Public Member Functions**

- `MPxTopologyEvaluator evaluator (MStatus *ReturnStatus) const`
- `MEvaluationGraph ownerGraph (MStatus *ReturnStatus) const`
- `MObject asMObject (MStatus *ReturnStatus=nullptr) const`

**Static Public Member Functions**

- `static const char * className ()`

### MTransformationMatrix

Module `OpenMaya`. Source: [class_m_transformation_matrix.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_transformation_matrix.html).

**Public Types**

- `enum RotationOrder { kInvalid, kXYZ, kYZX, kZXY, kXZY, kYXZ, kZYX, kLast }`

**Public Member Functions**

- `~MTransformationMatrix ()`
- `MTransformationMatrix ()`
- `MTransformationMatrix (const MTransformationMatrix &src)`
- `MTransformationMatrix (const MMatrix &src)`
- `MTransformationMatrix & operator= (const MTransformationMatrix &src)`
- `MTransformationMatrix & operator= (const MMatrix &src)`
- `MMatrix asMatrix () const`
- `MMatrix asMatrix (double percent) const`
- `MMatrix asMatrixInverse () const`
- `MMatrix asScaleMatrix () const`
- `MMatrix asRotateMatrix () const`
- `MStatus getScale (double scale[3], MSpace::Space space) const`
- `MStatus setScale (const double scale[3], MSpace::Space space)`
- `MStatus addScale (const double scale[3], MSpace::Space space)`
- `MQuaternion rotation () const`
- `MTransformationMatrix & rotateTo (const MQuaternion &q)`
- `MTransformationMatrix & rotateBy (const MQuaternion &q, MSpace::Space space, MStatus *ReturnStatus=NULL)`
- `MEulerRotation eulerRotation () const`
- `MTransformationMatrix & rotateTo (const MEulerRotation &e)`
- `MTransformationMatrix & rotateBy (const MEulerRotation &e, MSpace::Space space, MStatus *ReturnStatus=NULL)`
- `MStatus getRotationQuaternion (double &x, double &y, double &z, double &w) const`
- `MStatus setRotationQuaternion (double x, double y, double z, double w)`
- `MStatus addRotationQuaternion (double x, double y, double z, double w, MSpace::Space)`
- `MStatus getRotation (double[3], RotationOrder &order) const`
- `MStatus setRotation (const double rotation[3], RotationOrder order)`
- `MStatus addRotation (const double rotation[3], RotationOrder order, MSpace::Space)`
- `RotationOrder rotationOrder (MStatus *ReturnStatus=NULL) const`
- `MStatus reorderRotation (const RotationOrder &order)`
- `MStatus setToRotationAxis (const MVector &axis, double rotation)`
- `MQuaternion rotationOrientation () const`
- `MTransformationMatrix & setRotationOrientation (const MQuaternion &q)`
- `MVector getTranslation (MSpace::Space, MStatus *ReturnStatus=NULL) const`
- `MStatus setTranslation (const MVector &vector, MSpace::Space)`
- `MStatus addTranslation (const MVector &vector, MSpace::Space)`
- `MStatus setShear (const double shear[3], MSpace::Space space)`
- `MStatus getShear (double shear[3], MSpace::Space space) const`
- `MStatus addShear (const double shear[3], MSpace::Space space)`
- `MPoint scalePivot (MSpace::Space, MStatus *ReturnStatus=NULL) const`
- `MStatus setScalePivot (const MPoint &, MSpace::Space, bool balance)`
- `MVector scalePivotTranslation (MSpace::Space, MStatus *ReturnStatus=NULL) const`
- `MStatus setScalePivotTranslation (const MVector &vector, MSpace::Space)`
- `MPoint rotatePivot (MSpace::Space, MStatus *ReturnStatus=NULL) const`
- `MStatus setRotatePivot (const MPoint &point, MSpace::Space, bool balance)`
- `MVector rotatePivotTranslation (MSpace::Space space, MStatus *ReturnStatus=NULL) const`
- `MStatus setRotatePivotTranslation (const MVector &vector, MSpace::Space space)`
- `bool isEquivalent (const MTransformationMatrix &other, double tolerance=MTransformationMatrix_kTol) const`
- `bool operator== (const MTransformationMatrix &other) const`
- `bool operator!= (const MTransformationMatrix &other) const`
- `MStatus getRotation (double[3], RotationOrder &order, MSpace::Space) const`
- `MStatus setRotation (const double rotation[3], RotationOrder order, MSpace::Space)`
- `MStatus getRotationQuaternion (double &x, double &y, double &z, double &w, MSpace::Space) const`
- `MStatus setRotationQuaternion (double x, double y, double z, double w, MSpace::Space)`
- `MVector translation (MSpace::Space, MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static const MTransformationMatrix identity`

### MTrimBoundaryArray

Module `OpenMaya`. Source: [class_m_trim_boundary_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_trim_boundary_array.html).

**Public Member Functions**

- `MTrimBoundaryArray ()`
- `MTrimBoundaryArray (const MTrimBoundaryArray &other)`
- `~MTrimBoundaryArray ()`
- `MStatus reserve (unsigned int length)`
- `unsigned int length () const`
- `unsigned int size () const`
- `const MObjectArray & operator[] (unsigned int index) const`
- `const MObjectArray & get (unsigned int index) const`
- `MStatus set (const MObjectArray &boundary, unsigned int index)`
- `MStatus insert (const MObjectArray &boundary, unsigned int index)`
- `MStatus append (const MObjectArray &boundary)`
- `MStatus remove (unsigned int index)`
- `MStatus clear ()`
- `MObject getMergedBoundary (unsigned int index, MStatus *status=NULL) const`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

### MTypeId

Module `OpenMaya`. Source: [class_m_type_id.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_type_id.html).

**Public Member Functions**

- `MTypeId ()`
- `~MTypeId ()`
- `MTypeId (unsigned int id)`
- `MTypeId (unsigned int prefix, unsigned int id)`
- `MTypeId (const MTypeId &src)`
- `MTypeId & operator= (const MTypeId &rhs)`
- `bool operator== (const MTypeId &rhs) const`
- `bool operator!= (const MTypeId &rhs) const`
- `unsigned int id (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MURI

Module `OpenMaya`. Source: [class_m_u_r_i.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_u_r_i.html).

**Public Member Functions**

- `MURI ()`
- `MURI (const MString &URI)`
- `MURI (const MURI &other)`
- `~MURI ()`
- `MURI & operator= (const MURI &rhs)`
- `MURI & operator= (const MString &URI)`
- `bool operator== (const MURI &rhs) const`
- `bool operator!= (const MURI &rhs) const`
- `MStatus copy (const MURI &other)`
- `MStatus setURI (const MString &URI)`
- `MStatus setScheme (const MString &scheme)`
- `MStatus setPath (const MString &path)`
- `MStatus setFragment (const MString &fragment)`
- `MStatus setFileName (const MString &fileName)`
- `MStatus setDirectory (const MString &directory)`
- `MStatus setAuthority (const MString &authority)`
- `MStatus setUserInfo (const MString &userInfo)`
- `MStatus setUserName (const MString &userName)`
- `MStatus setPassword (const MString &password)`
- `MStatus setHost (const MString &host)`
- `MStatus setPort (int port)`
- `MStatus addQueryItem (const MString &key, const MString &value)`
- `MStatus setQueryDelimiters (char valueDelimiter, char pairDelimiter)`
- `MString asString () const`
- `MString getScheme () const`
- `MString getPath () const`
- `MString getFragment () const`
- `MString getFileName (bool includeExtension=true) const`
- `MString getDirectory () const`
- `MString getAuthority () const`
- `MString getUserInfo () const`
- `MString getUserName () const`
- `MString getPassword () const`
- `MString getHost () const`
- `int getPort () const`
- `MStringArray getAllQueryItemKeys () const`
- `MString getQueryItemValue (const MString &key) const`
- `MStringArray getAllQueryItemValues (const MString &key) const`
- `char getQueryValueDelimiter () const`
- `char getQueryPairDelimiter () const`
- `MStatus removeQueryItem (const MString &key)`
- `MStatus removeAllQueryItems (const MString &key)`
- `bool isEmpty () const`
- `bool isValid () const`
- `MStatus clear ()`

**Static Public Member Functions**

- `static bool isValidURI (const MString &URI)`
- `static const char * className ()`

### MUint64Array

Module `OpenMaya`. Source: [class_m_uint64_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uint64_array.html).

**Public Member Functions**

- `MUint64Array ()`
- `MUint64Array (const MUint64Array &other)`
- `MUint64Array (const MUint64 src[], unsigned int count)`
- `MUint64Array (unsigned int initialSize, MUint64 initialValue=0)`
- `~MUint64Array ()`
- `MUint64Array & operator= (const MUint64Array &other)`
- `MStatus set (MUint64 element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (MUint64 element, unsigned int index)`
- `MStatus append (MUint64 element)`
- `MStatus copy (const MUint64Array &source)`
- `MStatus clear ()`
- `MStatus get (MUint64[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MUint64 operator[] (unsigned int index) const`
- `MUint64 & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MUint64Array &array)`

### MUintArray

Module `OpenMaya`. Source: [class_m_uint_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uint_array.html).

**Public Member Functions**

- `MUintArray ()`
- `MUintArray (const MUintArray &other)`
- `MUintArray (const unsigned int src[], unsigned int count)`
- `MUintArray (unsigned int initialSize, unsigned int initialValue=0)`
- `~MUintArray ()`
- `MUintArray & operator= (const MUintArray &other)`
- `MStatus set (unsigned int element, unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (unsigned int element, unsigned int index)`
- `MStatus append (unsigned int element)`
- `MStatus copy (const MUintArray &source)`
- `MStatus clear ()`
- `MStatus get (unsigned int[]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `unsigned int operator[] (unsigned int index) const`
- `unsigned int & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MUintArray &array)`

### MUniqueString

Module `OpenMaya`. Source: [class_m_unique_string.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_unique_string.html).

**Public Member Functions**

- `MUniqueString ()`
- `MUniqueString (const MUniqueString &)`
- `~MUniqueString ()`
- `MUniqueString (MUniqueString &&)`
- `MUniqueString & operator= (MUniqueString &&)`
- `const MString & string () const`
- `size_t hash () const`
- `bool operator== (const MUniqueString &other) const`
- `bool operator!= (const MUniqueString &other) const`
- `bool operator> (const MUniqueString &other) const`
- `bool operator>= (const MUniqueString &other) const`
- `bool operator< (const MUniqueString &other) const`
- `bool operator<= (const MUniqueString &other) const`
- `bool operator== (const MString &other) const`
- `bool operator== (const char *other) const`
- `bool operator!= (const MString &other) const`
- `bool operator!= (const char *other) const`

**Static Public Member Functions**

- `static MUniqueString intern (const MString &s)`

### MUserEventMessage

Module `OpenMaya`. Inherits `MMessage`. Source: [class_m_user_event_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_user_event_message.html).

**Static Public Member Functions**

- `static MStatus registerUserEvent (const MString &event)`
- `static bool isUserEvent (const MString &event)`
- `static MStatus deregisterUserEvent (const MString &event)`
- `static MCallbackId addUserEventCallback (const MString &event, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MStatus postUserEvent (const MString &event, void *clientData=NULL)`
- `static const char * className ()`

### MUuid

Module `OpenMaya`. Source: [class_m_uuid.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uuid.html).

**Public Member Functions**

- `MUuid ()`
- `MUuid (const MUuid &other)`
- `MUuid (const unsigned int *uuid)`
- `MUuid (const unsigned char *uuid)`
- `MUuid (const MString &value, MStatus *ReturnStatus=NULL)`
- `~MUuid ()`
- `MUuid & operator= (const MUuid &rhs)`
- `bool operator== (const MUuid &rhs) const`
- `bool operator!= (const MUuid &rhs) const`
- `MStatus get (unsigned char *uuid) const`
- `MString asString () const`
- `operator MString () const`
- `void copy (const MUuid &rhs)`
- `bool valid () const`
- `MStatus generate ()`

**Static Public Member Functions**

- `static const char * className ()`

### MVector

Module `OpenMaya`. Source: [class_m_vector.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_vector.html).

**Public Types**

- `enum Axis { kXaxis, kYaxis, kZaxis, kWaxis }`

**Public Member Functions**

- `MVector ()`
- `MVector (const MVector &)`
- `MVector (const MFloatVector &)`
- `MVector (const MPoint &)`
- `MVector (const MFloatPoint &)`
- `MVector (double xx, double yy, double zz=0.0)`
- `MVector (const double d[3])`
- `MVector (const float f[3])`
- `~MVector ()`
- `MVector & operator= (const MVector &src)`
- `double operator() (unsigned int i) const`
- `double operator[] (unsigned int i) const`
- `MVector operator^ (const MVector &right) const`
- `double operator* (const MVector &right) const`
- `MVector & operator/= (double scalar)`
- `MVector operator/ (double scalar) const`
- `MVector & operator*= (int scalar)`
- `MVector & operator*= (short scalar)`
- `MVector & operator*= (unsigned int scalar)`
- `MVector & operator*= (unsigned short scalar)`
- `MVector & operator*= (float scalar)`
- `MVector & operator*= (double scalar)`
- `MVector operator* (int scalar) const`
- `MVector operator* (short scalar) const`
- `MVector operator* (unsigned int scalar) const`
- `MVector operator* (unsigned short scalar) const`
- `MVector operator* (float scalar) const`
- `MVector operator* (double scalar) const`
- `MVector operator+ (const MVector &other) const`
- `MVector & operator+= (const MVector &other)`
- `MVector operator- () const`
- `MVector & operator-= (const MVector &other)`
- `MVector operator- (const MVector &other) const`
- `MVector operator* (const MMatrix &) const`
- `MVector & operator*= (const MMatrix &)`
- `bool operator!= (const MVector &other) const`
- `bool operator== (const MVector &other) const`
- `MVector rotateBy (double x, double y, double z, double w) const`
- `MVector rotateBy (const double rotXYZ[3], MTransformationMatrix::RotationOrder order) const`
- `MVector rotateBy (MVector::Axis axis, const double angle) const`
- `MVector rotateBy (const MQuaternion &) const`
- `MVector rotateBy (const MEulerRotation &) const`
- `MQuaternion rotateTo (const MVector &) const`
- `MStatus get (double[3]) const`
- `double length () const`
- `MVector normal () const`
- `MStatus normalize ()`
- `double angle (const MVector &other) const`
- `bool isEquivalent (const MVector &other, double tolerance=MVector_kTol) const`
- `bool isParallel (const MVector &other, double tolerance=MVector_kTol) const`
- `MVector transformAsNormal (const MMatrix &matrix) const`
- `double & operator() (unsigned int i)`
- `double & operator[] (unsigned int i)`

**Public Attributes**

- `double x`
- `double y`
- `double z`

**Static Public Attributes**

- `static const MVector zero`
- `static const MVector one`
- `static const MVector xAxis`
- `static const MVector yAxis`
- `static const MVector zAxis`
- `static const MVector xNegAxis`
- `static const MVector yNegAxis`
- `static const MVector zNegAxis`

**Friends**

- `OPENMAYA_EXPORT MVector operator* (const MMatrix &, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (int, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (short, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (unsigned int, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (unsigned short, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (float, const MVector &)`
- `OPENMAYA_EXPORT MVector operator* (double, const MVector &)`
- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MVector &v)`

### MVectorArray

Module `OpenMaya`. Source: [class_m_vector_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_vector_array.html).

**Public Member Functions**

- `MVectorArray ()`
- `MVectorArray (const MVectorArray &other)`
- `MVectorArray (const MVector vectors[], unsigned int count)`
- `MVectorArray (const double vectors[][3], unsigned int count)`
- `MVectorArray (const float vectors[][3], unsigned int count)`
- `MVectorArray (unsigned int initialSize, const MVector &initialValue=MVector::zero)`
- `~MVectorArray ()`
- `const MVector & operator[] (unsigned int index) const`
- `MVectorArray & operator= (const MVectorArray &other)`
- `MStatus set (const MVector &element, unsigned int index)`
- `MStatus set (double element[3], unsigned int index)`
- `MStatus set (float element[3], unsigned int index)`
- `MStatus setLength (unsigned int length)`
- `unsigned int length () const`
- `MStatus remove (unsigned int index)`
- `MStatus insert (const MVector &element, unsigned int index)`
- `MStatus append (const MVector &element)`
- `MStatus copy (const MVectorArray &source)`
- `MStatus clear ()`
- `MStatus get (double[][3]) const`
- `MStatus get (float[][3]) const`
- `void setSizeIncrement (unsigned int newIncrement)`
- `unsigned int sizeIncrement () const`
- `MVector & operator[] (unsigned int index)`
- `Iterator begin ()`
- `Iterator end ()`
- `ConstIterator begin () const`
- `ConstIterator end () const`
- `ConstIterator cbegin () const`
- `ConstIterator cend () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `OPENMAYA_EXPORT std::ostream & operator<< (std::ostream &os, const MVectorArray &array)`

### MWeight

Module `OpenMaya`. Source: [class_m_weight.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_weight.html).

**Public Member Functions**

- `MWeight ()`
- `MWeight (const MWeight &src)`
- `~MWeight ()`
- `MWeight & operator= (const MWeight &other)`
- `float influence () const`
- `float seam () const`
- `void setInfluence (float influence)`
- `void setSeam (float seam)`

**Static Public Member Functions**

- `static const char * className ()`

### MaterialInputData

Module `OpenMaya`. Source: [class_material_input_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_material_input_data.html).

**Public Attributes**

- `float shininess`
- `MColor ambient`
- `MColor diffuse`
- `MColor emission`
- `MColor specular`
- `bool hasTransparency`

## Namespaces

### MRandom

## Macros

### OPENMAYA_DEPRECATED(version, message)
