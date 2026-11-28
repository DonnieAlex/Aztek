# OpenMayaUI

Class names, bases and signatures of the module, checked against the reference's 2022 C++ API reference (https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_u_i.html). Descriptions are not kept here; follow the source link on a class for them.

Classes in this module: 65.

## Classes

### M3dView

Module `OpenMayaUI`. Source: [class_m3d_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m3d_view.html).

**Public Types**

- `enum DisplayStyle { kBoundingBox, kFlatShaded, kGouraudShaded, kWireFrame, kPoints }`
- `enum DisplayStatus { kActive, kLive, kDormant, kInvisible, kHilite, kTemplate, kActiveTemplate, kActiveComponent, kLead, kIntermediateObject, kActiveAffected, kNoStatus }`
- `enum ColorTable { kActiveColors = kActive, kDormantColors = kDormant, kTemplateColor = kTemplate, kBackgroundColor }`
- `enum TextPosition { kLeft, kCenter, kRight }`
- `enum DisplayObjects { kDisplayEverything = ~0, kDisplayNurbsCurves = 1 << 0, kDisplayNurbsSurfaces = 1 << 1, kDisplayMeshes = 1 << 2, kDisplayPlanes = 1 << 3, kDisplayLights = 1 << 4, kDisplayCameras = 1 << 5, kDisplayJoints = 1 << 6, kDisplayIkHandles = 1 << 7, kDisplayDeformers = 1 << 8, kDisplayDynamics = 1 << 9, kDisplayParticleInstancers = 1 << 10, kDisplayLocators = 1 << 11, kDisplayDimensions = 1 << 12, kDisplaySelectHandles = 1 << 13, kDisplayPivots = 1 << 14, kDisplayTextures = 1 << 15, kDisplayGrid = 1 << 16, kDisplayCVs = 1 << 17, kDisplayHulls = 1 << 18, kDisplayStrokes = 1 << 19, kDisplaySubdivSurfaces = 1 << 20, kDisplayFluids = 1 << 21, kDisplayFollicles = 1 << 22, kDisplayHairSystems = 1 << 23, kDisplayImagePlane = 1 << 24, kDisplayNCloths = 1 << 25, kDisplayNRigids = 1 << 26, kDisplayDynamicConstraints = 1 << 27, kDisplayManipulators = 1 << 28, kDisplayNParticles = 1 << 29, kExcludeMotionTrails = 1 << 30, kExcludePluginShapes = 1 << 31 }`
- `enum LightingMode { kLightAll, kLightSelected, kLightActive, kLightDefault, kUnused1, kLightNone }`
- `enum RendererName { kDefaultQualityRenderer, kHighQualityRenderer, kViewport2Renderer, kExternalRenderer }`
- `enum DepthBufferFormat { kDepth_8, kDepth_Float }`
- `enum LineStipplePattern { kStippleNone, kStippleDashed }`

**Public Member Functions**

- `MStatus makeSharedContextCurrent ()`
- `MGLContext display (MStatus *ReturnStatus=NULL)`
- `void getScreenPosition (int &x, int &y, MStatus *ReturnStatus=NULL) const`
- `QWidget * widget (MStatus *ReturnStatus=NULL) const`
- `MNativeWindowHdl window (MStatus *ReturnStatus=NULL)`
- `int portWidth (MStatus *ReturnStatus=NULL)`
- `int portHeight (MStatus *ReturnStatus=NULL)`
- `bool isVisible (MStatus *ReturnStatus=NULL)`
- `int playblastPortWidth (MStatus *ReturnStatus=NULL)`
- `int playblastPortHeight (MStatus *ReturnStatus=NULL)`
- `MStatus pushViewport (unsigned int x, unsigned int y, unsigned int width, unsigned int height)`
- `MStatus popViewport ()`
- `MStatus viewport (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`
- `MStatus beginGL ()`
- `MStatus endGL ()`
- `void beginSelect (GLuint *buffer=NULL, GLsizei size=0)`
- `GLint endSelect ()`
- `bool selectMode () const`
- `bool textureMode () const`
- `void loadName (GLuint name)`
- `void pushName (GLuint name)`
- `void popName ()`
- `void initNames ()`
- `MStatus beginXorDrawing (bool drawOrthographic=true, bool disableDepthTesting=true, float lineWidth=1.0f, LineStipplePattern stipplePattern=kStippleNone, const MColor &lineColor=MColor(1, 1, 1))`
- `MStatus endXorDrawing ()`
- `MStatus setDrawColor (unsigned int index, ColorTable table=kActiveColors)`
- `MStatus setDrawColor (const MColor &color)`
- `MStatus setDrawColorAndAlpha (const MColor &color)`
- `unsigned int numDormantColors (MStatus *ReturnStatus=NULL)`
- `unsigned int numActiveColors (MStatus *ReturnStatus=NULL)`
- `unsigned int numUserDefinedColors (MStatus *ReturnStatus=NULL)`
- `MStatus setUserDefinedColor (unsigned int index, const MColor &color)`
- `unsigned int userDefinedColorIndex (unsigned int index, MStatus *ReturnStatus=NULL)`
- `MColor colorAtIndex (unsigned int index, ColorTable table=kActiveColors, MStatus *ReturnStatus=NULL)`
- `MStatus getColorIndexAndTable (unsigned int glindex, unsigned int &index, ColorTable &table) const`
- `MStatus colorMask (bool &r, bool &g, bool &b, bool &a)`
- `MStatus setColorMask (bool r, bool g, bool b, bool a)`
- `MStatus drawText (const MString &text, const MPoint position, TextPosition textPosition=kLeft)`
- `MStatus getCamera (MDagPath &camera)`
- `MStatus setCamera (MDagPath &camera)`
- `MStatus scheduleRefresh ()`
- `MStatus refresh (bool all=false, bool force=false)`
- `MStatus refresh (bool all, bool force, bool offscreen)`
- `MStatus refresh (MPxGlBuffer &buffer)`
- `MStatus refresh (MPxGlBuffer &buffer, bool offscreen)`
- `MStatus refresh (MPxGlBuffer &buffer, bool offscreen, const MMatrix &projectionMatrix)`
- `MStatus getLightCount (unsigned int &count, bool visible=true)`
- `MStatus getLightingMode (LightingMode &mode)`
- `MStatus getLightPath (unsigned int lightNumber, MDagPath &light)`
- `MStatus isLightVisible (unsigned int lightNumber, bool &visible)`
- `MStatus getLightIndex (unsigned int lightNumber, unsigned int &lightIndex)`
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &worldPt, MVector &worldVector) const`
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &nearClipPt, MPoint &farClipPt) const`
- `MStatus viewToObjectSpace (short x_pos, short y_pos, const MMatrix &localMatrixInverse, MPoint &oPt, MVector &oVector) const`
- `bool worldToView (const MPoint &worldPt, short &x_pos, short &y_pos, MStatus *ReturnStatus=NULL) const`
- `MStatus projectionMatrix (MMatrix &projectionMatrix) const`
- `MStatus modelViewMatrix (MMatrix &modelViewMatrix) const`
- `MString viewSelectedPrefix (MStatus *ReturnStatus) const`
- `MStatus setViewSelectedPrefix (const MString &prefix)`
- `bool showViewSelectedChildren (MStatus *ReturnStatus) const`
- `MStatus setShowViewSelectedChildren (bool)`
- `DisplayStyle displayStyle (MStatus *ReturnStatus=NULL) const`
- `bool isShadeActiveOnly (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayStyle (DisplayStyle style, bool activeOnly=false)`
- `MStatus setObjectDisplay (unsigned int displayMask)`
- `unsigned int objectDisplay (MStatus *ReturnStatus=NULL)`
- `MStatus setPluginObjectDisplay (const MString &pluginDisplayFilter, bool on)`
- `bool pluginObjectDisplay (const MString &pluginDisplayFilter, MStatus *ReturnStatus=NULL)`
- `RendererName getRendererName (MStatus *ReturnStatus) const`
- `MString rendererString (MStatus *ReturnStatus=NULL) const`
- `bool wireframeOnlyInShadedMode (MStatus *ReturnStatus) const`
- `bool wireframeOnShaded (MStatus *ReturnStatus=NULL) const`
- `bool xray (MStatus *ReturnStatus=NULL) const`
- `bool xrayJoints (MStatus *ReturnStatus=NULL) const`
- `bool twoSidedLighting (MStatus *ReturnStatus=NULL) const`
- `MStatus readColorBuffer (MImage &image, bool readRGBA=false)`
- `MStatus writeColorBuffer (const MImage &image, signed short x=0, signed short y=0) const`
- `MStatus readDepthMap (unsigned short x, unsigned short y, unsigned int width, unsigned int height, unsigned char *bufferPtr, DepthBufferFormat depthMapPrecision)`
- `MStatus readBufferTo2dTexture (unsigned short x, unsigned short y, unsigned int width, unsigned int height)`
- `bool usingMipmappedTextures () const`
- `bool usingDefaultMaterial () const`
- `MStatus setDisallowPolygonOffset (bool v)`
- `bool disallowPolygonOffset () const`
- `MStatus updateViewingParameters ()`
- `bool multipleDrawEnabled () const`
- `void setMultipleDrawEnable (bool enable)`
- `unsigned multipleDrawPassCount ()`
- `void setMultipleDrawPassCount (unsigned count)`
- `MStatus beginProjMatrixOverride (MMatrix &projectionMatrix)`
- `MStatus endProjMatrixOverride ()`
- `MStatus setRenderOverrideName (const MString &name)`
- `MString renderOverrideName (MStatus *ReturnStatus=NULL) const`
- `MStatus setObjectListFilterName (const MString &name)`
- `MString objectListFilterName (MStatus *ReturnStatus=NULL) const`
- `MStatus setShowObjectFilterNameInHUD (bool show)`
- `bool showObjectFilterNameInHUD (MStatus *ReturnStatus) const`
- `bool viewIsFiltered (MStatus *ReturnStatus=NULL) const`
- `MStatus filteredObjectList (MSelectionList &list) const`
- `MStatus getRendererString (MString &stringName) const`

**Static Public Member Functions**

- `static M3dView active3dView (MStatus *ReturnStatus=NULL)`
- `static unsigned int numberOf3dViews ()`
- `static MStatus get3dView (const unsigned int index, M3dView &view)`
- `static DisplayStatus displayStatus (const MDagPath &path, MStatus *ReturnStatus=NULL)`
- `static MNativeWindowHdl applicationShell (MStatus *ReturnStatus=NULL)`
- `static bool isBackgroundGradient (MStatus *ReturnStatus=NULL)`
- `static MColor templateColor (MStatus *ReturnStatus=NULL)`
- `static MColor backgroundColor (MStatus *ReturnStatus=NULL)`
- `static MColor backgroundColorTop (MStatus *ReturnStatus=NULL)`
- `static MColor backgroundColorBottom (MStatus *ReturnStatus=NULL)`
- `static MColor liveColor (MStatus *ReturnStatus=NULL)`
- `static MColor referenceLayerColor (MStatus *ReturnStatus=NULL)`
- `static MColor activeTemplateColor (MStatus *ReturnStatus=NULL)`
- `static MColor leadColor (MStatus *ReturnStatus=NULL)`
- `static MColor hiliteColor (MStatus *ReturnStatus=NULL)`
- `static MColor activeAffectedColor (MStatus *ReturnStatus=NULL)`
- `static MStatus scheduleRefreshAllViews ()`
- `static MStatus getM3dViewFromModelPanel (const MString &modelPaneName, M3dView &view)`
- `static MStatus getM3dViewFromModelEditor (const MString &modelPaneName, M3dView &view)`
- `static const char * className ()`

### MCharBuffer

Module `OpenMayaUI`. Source: [class_m_char_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_char_buffer.html).

**Public Member Functions**

- `MCharBuffer ()`
- `char * ptr ()`
- `long size ()`

**Friends**

- `class MPxThreadedDeviceNode`

### MCursor

Module `OpenMayaUI`. Source: [class_m_cursor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_cursor.html).

**Public Member Functions**

- `MCursor (short width, short height, short hotSpotX, short hotSpotY, unsigned char *bits, unsigned char *mask)`
- `MCursor (const MCursor &other)`
- `~MCursor ()`
- `MCursor & operator= (const MCursor &)`
- `bool operator== (const MCursor &) const`
- `bool operator!= (const MCursor &) const`

**Static Public Attributes**

- `static const MCursor defaultCursor`
- `static const MCursor crossHairCursor`
- `static const MCursor doubleCrossHairCursor`
- `static const MCursor editCursor`
- `static const MCursor pencilCursor`
- `static const MCursor handCursor`

### MDeviceChannel

Module `OpenMayaUI`. Source: [class_m_device_channel.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_device_channel.html).

**Public Member Functions**

- `MDeviceChannel (const MString &, MDeviceChannel *=NULL, int=-1)`
- `~MDeviceChannel ()`
- `MString name () const`
- `MString longName () const`
- `int axisIndex () const`
- `bool hasChildren () const`
- `MDeviceChannel parent () const`
- `MDeviceChannel childByIndex (int)`
- `int numChildren () const`

### MDeviceState

Module `OpenMayaUI`. Source: [class_m_device_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_device_state.html).

**Public Member Functions**

- `virtual ~MDeviceState ()`
- `int devicePosition (const unsigned short int axis) const`
- `int devicePosition (const MString &axisName) const`
- `void setDevicePosition (const int position, const unsigned short int axis)`
- `void setDevicePosition (const int position, const MString &axisName)`
- `bool buttonState (const unsigned short int button) const`
- `bool buttonState (const MString &buttonName) const`
- `void setButtonState (const bool state, const unsigned short int button)`
- `void setButtonState (const bool state, const MString &buttonName)`
- `int maxAxis () const`
- `bool isNull ()`

### MDrawData

Module `OpenMayaUI`. Source: [class_m_draw_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_data.html).

**Public Member Functions**

- `MDrawData ()`
- `MDrawData (const MDrawData &in)`
- `~MDrawData ()`
- `void * geometry ()`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MDrawData & operator= (const MDrawData &other)`

### MDrawInfo

Module `OpenMayaUI`. Source: [class_m_draw_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_info.html).

**Public Member Functions**

- `MDrawInfo ()`
- `MDrawInfo (const MDrawInfo &in)`
- `virtual ~MDrawInfo ()`
- `MDrawRequest getPrototype (const MPxSurfaceShapeUI &drawHandler) const`
- `M3dView view () const`
- `const MDagPath multiPath () const`
- `void setMultiPath (const MDagPath &)`
- `const MMatrix projectionMatrix () const`
- `const MMatrix inclusiveMatrix () const`
- `M3dView::DisplayStyle displayStyle () const`
- `M3dView::DisplayStatus displayStatus () const`
- `bool objectDisplayStatus (M3dView::DisplayObjects) const`
- `bool pluginObjectDisplayStatus (const MString &pluginDisplayFilter) const`
- `bool inSelect () const`
- `bool inUserInteraction () const`
- `bool userChangingViewContext () const`
- `bool completelyInside () const`
- `bool canDrawComponent (bool isDisplayOn, const MSelectionMask &compMask) const`

**Static Public Member Functions**

- `static const char * className ()`

### MDrawRequest

Module `OpenMayaUI`. Source: [class_m_draw_request.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_request.html).

**Public Member Functions**

- `MDrawRequest ()`
- `MDrawRequest (const MDrawRequest &in)`
- `~MDrawRequest ()`
- `M3dView view () const`
- `void setView (M3dView &)`
- `const MDagPath multiPath () const`
- `void setMultiPath (const MDagPath &)`
- `MObject component () const`
- `void setComponent (MObject &)`
- `MDrawData drawData () const`
- `void setDrawData (MDrawData &)`
- `M3dView::DisplayStatus displayStatus () const`
- `void setDisplayStatus (M3dView::DisplayStatus)`
- `bool displayCulling () const`
- `void setDisplayCulling (bool)`
- `bool displayCullOpposite () const`
- `void setDisplayCullOpposite (bool)`
- `M3dView::DisplayStyle displayStyle () const`
- `void setDisplayStyle (M3dView::DisplayStyle)`
- `int color (M3dView::ColorTable table) const`
- `void setColor (int, M3dView::ColorTable table)`
- `MColor color () const`
- `void setColor (const MColor &color)`
- `MMaterial material () const`
- `void setMaterial (MMaterial &)`
- `bool isTransparent () const`
- `void setIsTransparent (bool)`
- `bool drawLast () const`
- `void setDrawLast (bool)`
- `int token () const`
- `void setToken (int)`
- `const MMatrix & matrix () const`
- `void setMatrix (const MMatrix &)`
- `MDrawRequest & operator= (const MDrawRequest &other)`

**Static Public Member Functions**

- `static const char * className ()`

### MDrawRequestQueue

Module `OpenMayaUI`. Source: [class_m_draw_request_queue.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_request_queue.html).

**Public Member Functions**

- `MDrawRequestQueue ()`
- `~MDrawRequestQueue ()`
- `bool isEmpty () const`
- `void add (MDrawRequest &)`
- `MDrawRequest remove ()`
- `MDrawRequestQueue (const MDrawRequestQueue &)`
- `MDrawRequestQueue & operator= (const MDrawRequestQueue &)`

**Static Public Member Functions**

- `static const char * className ()`

### MDrawTraversal

Module `OpenMayaUI`. Source: [class_m_draw_traversal.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_traversal.html).

**Public Types**

- `enum { kActiveItem, kTemplateItem }`

**Public Member Functions**

- `MDrawTraversal ()`
- `virtual ~MDrawTraversal ()`
- `virtual MStatus traverse ()`
- `virtual bool filterNode (const MDagPath &traversalItem)`
- `MStatus setFrustum (const MDagPath &cameraPath, unsigned int portWidth, unsigned int portHeight)`
- `MStatus setOrthoFrustum (double left, double right, double bottom, double top, double nearpt, double farpt, const MMatrix &worldXform)`
- `MStatus setPerspFrustum (double fovX, double aspectXY, double nearDist, double farDist, const MMatrix &worldXform)`
- `MStatus setFrustum (double left, double right, double bottom, double top, double nearpt, double farpt, const MMatrix &worldXform)`
- `MStatus setFrustum (const MPoint &nearBottomLeft, const MPoint &nearBottomRight, const MPoint &nearTopLeft, const MPoint &nearTopRight, const MPoint &farBottomLeft, const MPoint &farBottomRight, const MPoint &farTopLeft, const MPoint &farTopRight, const MMatrix &worldXform)`
- `bool frustumValid () const`
- `void setLeafLevelCulling (bool cullAtLeafLevel)`
- `bool leafLevelCulling () const`
- `void enableFiltering (bool val)`
- `bool filteringEnabled () const`
- `unsigned int numberOfItems () const`
- `MStatus itemPath (unsigned int itemNumber, MDagPath &path) const`
- `bool itemHasStatus (unsigned int itemNumber, unsigned int test) const`

### MEvent

Module `OpenMayaUI`. Source: [class_m_event.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_event.html).

**Public Types**

- `enum ModifierType { shiftKey = 1<<0, controlKey = 1<<2 }`
- `enum MouseButtonType { kLeftMouse = 1<<6, kMiddleMouse = kLeftMouse<<1 }`

**Public Member Functions**

- `MEvent ()`
- `virtual ~MEvent ()`
- `MStatus getPosition (short &x_pos, short &y_pos) const`
- `MStatus setPosition (short x_pos, short y_pos)`
- `MStatus getWindowPosition (short &x_pos, short &y_pos) const`
- `MouseButtonType mouseButton (MStatus *ReturnStatus=NULL) const`
- `bool isModifierKeyRelease (MStatus *ReturnStatus=NULL) const`
- `ModifierType modifiers (MStatus *ReturnStatus=NULL) const`
- `MStatus setModifiers (const ModifierType &modType)`
- `bool isModifierNone (MStatus *ReturnStatus=NULL) const`
- `bool isModifierShift (MStatus *ReturnStatus=NULL) const`
- `bool isModifierControl (MStatus *ReturnStatus=NULL) const`
- `bool isModifierLeftMouseButton (MStatus *ReturnStatus=NULL) const`
- `bool isModifierMiddleMouseButton (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

### MExternalDropCallback

Module `OpenMayaUI`. Source: [class_m_external_drop_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_drop_callback.html).

**Public Types**

- `enum MExternalDropStatus { kMayaDefault = 0, kNoMayaDefaultAndAccept, kNoMayaDefaultAndNoAccept }`

**Public Member Functions**

- `MExternalDropCallback ()`
- `virtual ~MExternalDropCallback ()`
- `virtual MExternalDropStatus externalDropCallback (bool doDrop, const MString &controlName, const MExternalDropData &data)=0`

**Static Public Member Functions**

- `static MStatus addCallback (MExternalDropCallback *, int priority=0)`
- `static MStatus removeCallback (MExternalDropCallback *)`

### MExternalDropData

Module `OpenMayaUI`. Source: [class_m_external_drop_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_drop_data.html).

**Public Member Functions**

- `bool hasText () const`
- `MString text () const`
- `bool hasUrls () const`
- `MStringArray urls () const`
- `bool hasHtml () const`
- `MString html () const`
- `bool hasColor () const`
- `MColor color () const`
- `bool hasImage () const`
- `MImage image () const`
- `MStringArray formats () const`
- `bool hasFormat (const MString &format) const`
- `unsigned int dataSize (const MString &format) const`
- `MStatus copyData (const MString &format, void *buffer, unsigned int bufferSize) const`
- `int keyboardModifiers () const`

**Protected Attributes**

- `OPENMAYA_PRIVATE int mouseButtons`

### MFeedbackLine

Module `OpenMayaUI`. Source: [class_m_feedback_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_feedback_line.html).

**Static Public Member Functions**

- `static MStatus setFormat (const MString &format)`
- `static MStatus setTitle (const MString &title)`
- `static MStatus setValue (short index, double value)`
- `static void clear ()`
- `static bool showFeedback ()`
- `static void setShowFeedback (bool showFeedback)`
- `static const char * className ()`

### MFnCircleSweepManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_circle_sweep_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_circle_sweep_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCircleSweepManip ()`
- `MFnCircleSweepManip ()`
- `MFnCircleSweepManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnCircleSweepManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &angleName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToAnglePlug (MPlug &anglePlug)`
- `MStatus setCenterPoint (const MPoint &centerPoint)`
- `MStatus setStartPoint (const MPoint &startPoint)`
- `MStatus setEndPoint (const MPoint &endPoint)`
- `MStatus setNormal (const MVector &normal)`
- `MStatus setRadius (double radius)`
- `MStatus setAngle (MAngle angle)`
- `MStatus setDrawAsArc (bool state)`
- `MPoint startPoint (MStatus *ReturnStatus=NULL) const`
- `MPoint endPoint (MStatus *ReturnStatus=NULL) const`
- `unsigned int centerIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int axisIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int angleIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int startCircleIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int endCircleIndex (MStatus *ReturnStatus=NULL) const`
- `MFnCircleSweepManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnCurveSegmentManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_curve_segment_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_curve_segment_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnCurveSegmentManip ()`
- `MFnCurveSegmentManip ()`
- `MFnCurveSegmentManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnCurveSegmentManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &startParamName, const MString &endParamName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToCurvePlug (MPlug &curvePlug)`
- `MStatus connectToStartParamPlug (MPlug &startParamPlug)`
- `MStatus connectToEndParamPlug (MPlug &endParamPlug)`
- `MStatus setStartParameter (double startParameter)`
- `MStatus setEndParameter (double endParameter)`
- `double startParameter (MStatus *ReturnStatus=NULL) const`
- `double endParameter (MStatus *ReturnStatus=NULL) const`
- `unsigned int curveIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int startParamIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int endParamIndex (MStatus *ReturnStatus=NULL) const`
- `MFnCurveSegmentManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDirectionManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_direction_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_direction_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDirectionManip ()`
- `MFnDirectionManip ()`
- `MFnDirectionManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnDirectionManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &directionName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToDirectionPlug (MPlug &directionPlug)`
- `MStatus setNormalizeDirection (bool state)`
- `MStatus setDrawStart (bool state)`
- `MStatus setStartPoint (MPoint &startPoint)`
- `MStatus setDirection (MVector &direction)`
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int endPointIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`
- `MFnDirectionManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDiscManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_disc_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_disc_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDiscManip ()`
- `MFnDiscManip ()`
- `MFnDiscManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnDiscManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &angleName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToAnglePlug (MPlug &anglePlug)`
- `MStatus setCenterPoint (const MPoint &centerPoint)`
- `MStatus setNormal (const MVector &normal)`
- `MStatus setRadius (double radius)`
- `MStatus setAngle (MAngle angle)`
- `unsigned int centerIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int axisIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int angleIndex (MStatus *ReturnStatus=NULL) const`
- `MFnDiscManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDistanceManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_distance_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_distance_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDistanceManip ()`
- `MFnDistanceManip ()`
- `MFnDistanceManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnDistanceManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &distanceName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToDistancePlug (MPlug &distancePlug)`
- `MStatus setStartPoint (const MPoint &point)`
- `MStatus setDirection (const MVector &vector)`
- `MStatus setDrawStart (bool state)`
- `MStatus setDrawLine (bool state)`
- `MStatus setScalingFactor (double scalingFactor)`
- `bool isDrawStartOn (MStatus *ReturnStatus=NULL) const`
- `bool isDrawLineOn (MStatus *ReturnStatus=NULL) const`
- `double scalingFactor (MStatus *ReturnStatus=NULL) const`
- `unsigned int distanceIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int currentPointIndex (MStatus *ReturnStatus=NULL) const`
- `MFnDistanceManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnFreePointTriadManip

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_free_point_triad_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_free_point_triad_manip.html).

**Public Types**

- `enum ManipPlane { kYZPlane, kXZPlane, kXYPlane, kViewPlane }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnFreePointTriadManip ()`
- `MFnFreePointTriadManip ()`
- `MFnFreePointTriadManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnFreePointTriadManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &pointName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToPointPlug (MPlug &pointPlug)`
- `MStatus setDrawAxes (bool state)`
- `MStatus setSnapMode (bool state)`
- `MStatus setKeyframeAll (bool state)`
- `MStatus setDrawArrowHead (bool state)`
- `MStatus setGlobalTriadPlane (ManipPlane whichPlane)`
- `MStatus setPoint (const MPoint &pointValue)`
- `MStatus setDirection (const MVector &direction)`
- `bool isDrawAxesOn (MStatus *ReturnStatus=NULL) const`
- `bool isSnapModeOn (MStatus *ReturnStatus=NULL) const`
- `bool isKeyframeAllOn (MStatus *ReturnStatus=NULL) const`
- `unsigned int pointIndex (MStatus *ReturnStatus=NULL) const`
- `MFnFreePointTriadManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnManip3D

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_manip3_d.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_manip3_d.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnManip3D ()`
- `MFnManip3D ()`
- `MFnManip3D (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnManip3D (const MDagPath &object, MStatus *ret=NULL)`
- `bool isVisible (MStatus *ReturnStatus) const`
- `MStatus setVisible (bool isVisible)`
- `float manipScale (MStatus *ReturnStatus) const`
- `MStatus setManipScale (float size)`
- `bool isOptimizePlaybackOn (MStatus *ReturnStatus) const`
- `MStatus setOptimizePlayback (bool optimizePlayback)`
- `MEulerRotation rotateXYZValue (const unsigned int valIndex, MStatus *ReturnStatus=NULL)`
- `MFnManip3D (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`
- `static float globalSize ()`
- `static void setGlobalSize (float size)`
- `static float handleSize ()`
- `static void setHandleSize (float size)`
- `static float lineSize ()`
- `static void setLineSize (float size)`
- `static MStatus deleteManipulator (MObject &manip)`
- `static bool drawPlaneHandles ()`
- `static void setDrawPlaneHandles (bool drawPlaneHandles)`

### MFnPointOnCurveManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_point_on_curve_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_on_curve_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPointOnCurveManip ()`
- `MFnPointOnCurveManip ()`
- `MFnPointOnCurveManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnPointOnCurveManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &paramName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToCurvePlug (MPlug &curvePlug)`
- `MStatus connectToParamPlug (MPlug &paramPlug)`
- `MStatus setDrawCurve (bool state)`
- `MStatus setParameter (double parameter)`
- `bool isDrawCurveOn (MStatus *ReturnStatus=NULL) const`
- `double parameter (MStatus *ReturnStatus=NULL) const`
- `MPoint curvePoint (MStatus *ReturnStatus=NULL) const`
- `unsigned int curveIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int paramIndex (MStatus *ReturnStatus=NULL) const`
- `MFnPointOnCurveManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPointOnSurfaceManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_point_on_surface_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_on_surface_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPointOnSurfaceManip ()`
- `MFnPointOnSurfaceManip ()`
- `MFnPointOnSurfaceManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnPointOnSurfaceManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &paramName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToSurfacePlug (MPlug &surfacePlug)`
- `MStatus connectToParamPlug (MPlug &paramPlug)`
- `MStatus setDrawSurface (bool state)`
- `MStatus setDrawArrows (bool state)`
- `MStatus setParameters (double u, double v)`
- `MStatus getParameters (double &u, double &v)`
- `bool isDrawSurfaceOn (MStatus *ReturnStatus=NULL) const`
- `unsigned int surfaceIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int paramIndex (MStatus *ReturnStatus=NULL) const`
- `MFnPointOnSurfaceManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnRotateManip

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_rotate_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_rotate_manip.html).

**Public Types**

- `enum RotateMode { kObjectSpace, kWorldSpace, kGimbal }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnRotateManip ()`
- `MFnRotateManip ()`
- `MFnRotateManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnRotateManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &rotationName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToRotationPlug (const MPlug &rotationPlug)`
- `MStatus connectToRotationCenterPlug (const MPlug &rotationCenterPlug)`
- `MStatus setInitialRotation (const MEulerRotation &rotation)`
- `MStatus displayWithNode (const MObject &node)`
- `MStatus setRotateMode (RotateMode mode)`
- `RotateMode rotateMode () const`
- `MStatus setSnapMode (bool snapEnabled)`
- `bool isSnapModeOn () const`
- `MStatus setSnapIncrement (double snapInc)`
- `double snapIncrement () const`
- `unsigned int rotationIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int rotationCenterIndex (MStatus *ReturnStatus=NULL) const`
- `MStatus setRotationCenter (const MPoint &rotationCenter)`
- `MFnRotateManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnScaleManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_scale_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_scale_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnScaleManip ()`
- `MFnScaleManip ()`
- `MFnScaleManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnScaleManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &scaleName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToScalePlug (const MPlug &scalePlug)`
- `MStatus connectToScaleCenterPlug (const MPlug &scaleCenterPlug)`
- `MStatus setInitialScale (const MVector &scale)`
- `MStatus displayWithNode (const MObject &node)`
- `MStatus setSnapMode (bool snapEnabled)`
- `bool isSnapModeOn () const`
- `MStatus setSnapIncrement (double snapInc)`
- `double snapIncrement () const`
- `MStatus setOrientation (const MEulerRotation &direction)`
- `MEulerRotation getOrientation (MStatus *ReturnStatus=NULL) const`
- `MStatus setOrientationMode (ScaleOrientationMode mode)`
- `ScaleOrientationMode getOrientationMode (MStatus *ReturnStatus=NULL) const`
- `unsigned int scaleIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int scaleCenterIndex (MStatus *ReturnStatus=NULL) const`
- `MFnScaleManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnStateManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_state_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_state_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnStateManip ()`
- `MFnStateManip ()`
- `MFnStateManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnStateManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &stateName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToStatePlug (MPlug &statePlug)`
- `MStatus setInitialState (unsigned int initialState)`
- `MStatus setMaxStates (unsigned int numStates)`
- `unsigned int maxStates (MStatus *ReturnStatus=NULL) const`
- `unsigned int state (MStatus *ReturnStatus=NULL) const`
- `unsigned int positionIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int stateIndex (MStatus *ReturnStatus=NULL) const`
- `MFnStateManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnToggleManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_toggle_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_toggle_manip.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnToggleManip ()`
- `MFnToggleManip ()`
- `MFnToggleManip (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnToggleManip (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MObject create (const MString &manipName, const MString &toggleName, MStatus *ReturnStatus=NULL)`
- `MStatus connectToTogglePlug (MPlug &togglePlug)`
- `MPoint startPoint (MStatus *ReturnStatus=NULL) const`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `double length (MStatus *ReturnStatus=NULL) const`
- `bool toggle (MStatus *ReturnStatus=NULL) const`
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int lengthIndex (MStatus *ReturnStatus=NULL) const`
- `unsigned int toggleIndex (MStatus *ReturnStatus=NULL) const`
- `MStatus setStartPoint (MPoint &startPoint)`
- `MStatus setDirection (MVector &direction)`
- `MStatus setLength (double length)`
- `MStatus setToggle (bool toggle)`
- `MFnToggleManip (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MGraphEditorInfo

Module `OpenMayaUI`. Inherits `MPanelCanvasInfo`. Source: [class_m_graph_editor_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_graph_editor_info.html).

**Public Types**

- `enum AnimCurveQuery { kAnimCurveOutlinerOnly = 0, kAnimCurveHighlighted, kAnimCurveSelected, kAnimCurveAllKnown }`

**Public Member Functions**

- `MGraphEditorInfo (MStatus *ReturnStatus=NULL)`
- `MGraphEditorInfo (const MString &graphEditorName, MStatus *ReturnStatus=NULL)`
- `~MGraphEditorInfo () override`
- `MStatus getAnimCurveNodes (MObjectArray &animCurveNodeArray, AnimCurveQuery animCurveQuery) const`
- `bool isStackedViewportMode () const`
- `bool isNormalizedViewportMode () const`

**Static Public Member Functions**

- `static const char * className ()`

### MHWShaderSwatchGenerator

Module `OpenMayaUI`. Inherits `MSwatchRenderBase`. Source: [class_m_h_w_shader_swatch_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_shader_swatch_generator.html).

**Public Member Functions**

- `MHWShaderSwatchGenerator (MObject obj, MObject renderObj, int res)`
- `bool doIteration () override`

**Static Public Member Functions**

- `static MSwatchRenderBase * createObj (MObject obj, MObject renderObj, int res)`
- `static const MString & initialize ()`
- `static void getSwatchBackgroundColor (float &r, float &g, float &b, float &a)`

### MManipData

Module `OpenMayaUI`. Source: [class_m_manip_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_manip_data.html).

**Public Member Functions**

- `MManipData ()`
- `~MManipData ()`
- `MManipData (const MManipData &)`
- `MManipData (bool)`
- `MManipData (short)`
- `MManipData (int)`
- `MManipData (unsigned int)`
- `MManipData (float)`
- `MManipData (double)`
- `MManipData (const MObject &)`
- `MManipData & operator= (const MManipData &)`
- `MManipData & operator= (bool)`
- `MManipData & operator= (short)`
- `MManipData & operator= (int)`
- `MManipData & operator= (unsigned int)`
- `MManipData & operator= (float)`
- `MManipData & operator= (double)`
- `MManipData & operator= (const MObject &)`
- `bool isSimple () const`
- `bool asBool () const`
- `short asShort () const`
- `int asLong () const`
- `unsigned int asUnsigned () const`
- `float asFloat () const`
- `double asDouble () const`
- `MObject asMObject () const`

**Static Public Member Functions**

- `static const char * className ()`

### MMaterial

Module `OpenMayaUI`. Source: [class_m_material.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_material.html).

**Public Types**

- `enum MtextureChannel { kColor, kTransparency, kAmbientColor, kIncandescence, kBumpMap, kDiffuse, kTransluscence, kRoughness, kHighlightSize, kWhiteness, kCosinePower, kEccentricity, kSpecularRollOff, kSpecularColor, kReflectivity, kReflectedColor }`

**Public Member Functions**

- `MMaterial ()`
- `MMaterial (const MMaterial &in)`
- `MMaterial (const MObject &, MStatus *ReturnStatus=NULL)`
- `~MMaterial ()`
- `MStatus evaluateMaterial (M3dView &, const MDagPath &)`
- `MStatus evaluateShininess ()`
- `MStatus evaluateDiffuse ()`
- `MStatus evaluateEmission ()`
- `MStatus evaluateSpecular ()`
- `MStatus evaluateTexture (MDrawData &data)`
- `bool materialIsTextured () const`
- `MStatus setMaterial (const MDagPath &, bool hasTransparency)`
- `MStatus getShininess (float &)`
- `MStatus getDiffuse (MColor &)`
- `MStatus getEmission (MColor &)`
- `MStatus getSpecular (MColor &)`
- `MStatus getHasTransparency (bool &)`
- `MStatus getTextureTransformation (const MDrawData &data, MMatrix &texXform)`
- `MStatus getTextureTransformation (const MDrawData &data, float &rotateUV, float &scaleU, float &scaleV, float &translateU, float &translateV, float &rotateFrame)`
- `void applyTexture (M3dView &, const MDrawData &)`
- `MStatus textureImage (MImage &image, MColor &color, MtextureChannel chan, bool &mapped, MDagPath &dagPath, int xRes=-1, int yRes=-1)`
- `MPxHwShaderNode * getHwShaderNode (MStatus *ReturnStatus=NULL)`
- `MObject shadingEngine ()`
- `MStatus evaluateTextureTransformation ()`
- `MStatus getTextureTransformation (float &scaleU, float &scaleV, float &translateU, float &translateV, float &rotate)`

**Static Public Member Functions**

- `static MMaterial defaultMaterial ()`
- `static const char * className ()`

### MMaterialArray

Module `OpenMayaUI`. Source: [class_m_material_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_material_array.html).

**Public Member Functions**

- `MMaterialArray ()`
- `MMaterialArray (const MMaterialArray &other)`
- `~MMaterialArray ()`
- `const MMaterial operator[] (unsigned index) const`
- `MMaterialArray & operator= (const MMaterialArray &other)`
- `MStatus set (const MMaterial &element, unsigned index)`
- `MStatus setLength (unsigned length)`
- `unsigned length () const`
- `MStatus remove (unsigned index)`
- `MStatus insert (const MMaterial &element, unsigned index)`
- `MStatus append (const MMaterial &element)`
- `MStatus copy (const MMaterialArray &source)`
- `MStatus clear ()`
- `void setSizeIncrement (unsigned newIncrement)`
- `unsigned sizeIncrement () const`
- `MMaterial operator[] (unsigned index)`

**Static Public Member Functions**

- `static const char * className ()`

### MObjectListFilter

Module `OpenMayaUI`. Source: [class_m_object_list_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object_list_filter.html).

**Public Types**

- `enum MFilterType { kInclusionList = 0, kExclusionList, kNumberOfFilterTypes }`
- `enum MSceneUpdateType { kNone = 0, kAddRemoveObjects = 1<<(0) }`

**Public Member Functions**

- `MObjectListFilter (const MString &name)`
- `virtual bool requireListUpdate ()=0`
- `virtual MStatus getList (MSelectionList &list)=0`
- `virtual MSceneUpdateType dependentOnSceneUpdates ()`
- `void setFilterType (MFilterType filterType)`
- `MFilterType filterType () const`
- `const MString & name () const`
- `const MString & UIname () const`
- `void setUIName (const MString &name)`

**Static Public Member Functions**

- `static MStatus registerFilter (const MObjectListFilter &filter)`
- `static MStatus deregisterFilter (const MObjectListFilter &filter)`
- `static const char * className ()`

### MPanelCanvas

Module `OpenMayaUI`. Source: [class_m_panel_canvas.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_panel_canvas.html).

**Public Types**

- `enum DrawLayer`
- `enum GraphEditorReservedLayer`
- `typedef void(* MDrawUIFunction) (MHWRender::MUIDrawManager &drawMgr, void *userData)`

**Public Member Functions**

- `MPanelCanvas (const MString &panelControlName, MStatus *stat=NULL)`
- `~MPanelCanvas ()`
- `bool supportsUIDrawing ()`
- `bool isValid ()`
- `int createVertexBuffer (const MTimeArray &xvals, const MFloatArray &yvals, const MColorArray *colors=NULL, MStatus *stat=NULL)`
- `int createVertexBuffer (const MFloatArray &xvals, const MFloatArray &yvals, const MColorArray *colors=NULL, MStatus *stat=NULL)`
- `int createPrimitive (MHWRender::MGeometry::Primitive primType, int vertexBufferId, int startIndex, int numVertices, const MDrawProperties &props, MStatus *stat=NULL)`
- `MStatus addPrimitive (int primId, int layer)`
- `MStatus removePrimitive (int primId, int layer)`
- `MStatus destroyPrimitive (int primId)`
- `MStatus destroyVertexBuffer (int vertId)`
- `MStatus refresh ()`
- `MStatus setAutoRefresh (bool enable)`
- `bool isAutoRefresh ()`
- `MStatus setLayerVisible (int layer, bool isVisible)`
- `bool isLayerVisible (int layer, MStatus *stat=NULL)`
- `int registerDrawUICallback (int layer, MDrawUIFunction func, void *clientData=NULL, MStatus *returnStatus=NULL)`
- `void unregisterDrawUICallback (int callbackId, MStatus *returnStatus=NULL)`

### MPanelCanvasInfo

Module `OpenMayaUI`. Source: [class_m_panel_canvas_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_panel_canvas_info.html).

**Public Member Functions**

- `MPanelCanvasInfo (const MString &editorName, MStatus *stat=NULL)`
- `virtual ~MPanelCanvasInfo ()`
- `MStatus getViewportBounds (double &left, double &right, double &bottom, double &top) const`
- `MStatus getViewportBounds (MDoubleArray &boundsArray) const`
- `MStatus getViewportSize (int &width, int &height) const`
- `MStatus setViewportBounds (double left, double right, double bottom, double top) const`
- `MStatus setViewportBounds (const MDoubleArray &boundsArray) const`
- `bool supportsUIDrawing ()`
- `void reset ()`
- `MString name () const`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MGraphEditorInfo`

### MProgressWindow

Module `OpenMayaUI`. Source: [class_m_progress_window.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_progress_window.html).

**Static Public Member Functions**

- `static bool reserve ()`
- `static MStatus startProgress ()`
- `static MStatus endProgress ()`
- `static MStatus setProgressRange (const int minValue, const int maxValue)`
- `static MStatus setProgressMin (const int minValue)`
- `static MStatus setProgressMax (const int maxValue)`
- `static int progressMin (MStatus *ReturnStatus=NULL)`
- `static int progressMax (MStatus *ReturnStatus=NULL)`
- `static MStatus setProgress (const int progress)`
- `static MStatus advanceProgress (const int amount)`
- `static int progress (MStatus *ReturnStatus=NULL)`
- `static MStatus setTitle (const MString &title)`
- `static MString title (MStatus *ReturnStatus=NULL)`
- `static MStatus setProgressStatus (const MString &progressStatus)`
- `static MString progressStatus (MStatus *ReturnStatus=NULL)`
- `static MStatus setInterruptable (const bool value)`
- `static bool isInterruptable (MStatus *ReturnStatus=NULL)`
- `static bool isCancelled (MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MPx3dModelView

Module `OpenMayaUI`. Source: [class_m_px3d_model_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px3d_model_view.html).

**Public Types**

- `enum LightingMode { kLightAll, kLightSelected, kLightActive, kLightDefault, kLightNone, kLightQuality }`
- `enum FogSource { kFogFragment, kFogCoordinate }`
- `enum FogMode { kFogLinear, kFogExponential, kFogExponentialSquared }`

**Public Member Functions**

- `MPx3dModelView ()`
- `virtual ~MPx3dModelView ()`
- `MString name (MStatus *ReturnStatus=NULL) const`
- `virtual MString viewType () const`
- `virtual void preMultipleDraw ()`
- `virtual void postMultipleDraw ()`
- `virtual void preMultipleDrawPass (unsigned int index)`
- `virtual void postMultipleDrawPass (unsigned int index)`
- `virtual bool okForMultipleDraw (const MDagPath &)`
- `virtual unsigned int multipleDrawPassCount ()`
- `bool multipleDrawEnabled () const`
- `void setMultipleDrawEnable (bool enable)`
- `bool destroyOnPanelDestruction () const`
- `void setDestroyOnPanelDestruction (bool how)`
- `MStatus updateViewingParameters ()`
- `virtual void removingCamera (MDagPath &cameraPath)`
- `MStatus setDoUpdateOnMove (bool value)`
- `bool doUpdateOnMove (MStatus *ReturnStatus=NULL) const`
- `MStatus refresh (bool all=false, bool force=false)`
- `MStatus drawText (const MString &text, const MPoint position, M3dView::TextPosition textPosition=M3dView::kLeft)`
- `MStatus beginGL ()`
- `MStatus endGL ()`
- `MStatus setCameraInDraw (MDagPath &camera)`
- `MStatus setDrawCameraOverride (const MMatrix &worldMatrix, const MMatrix &projectionMatrix, double left, double right, double bottom, double top, double nearpt, double farpt, bool isOrtho=false)`
- `MStatus setDrawCameraOverride (const MMatrix &worldMatrix, const MMatrix &projectionMatrix, const MPointArray &frustum)`
- `MStatus setCamera (MDagPath &camera)`
- `MStatus getCamera (MDagPath &camera)`
- `MStatus setCameraSet (MObject &cameraSet)`
- `MStatus getCameraSet (MObject &cameraSet)`
- `MStatus setCurrentCameraSetCamera (const MString &cameraName)`
- `MStatus getCurrentCameraSetCamera (MString &cameraName)`
- `virtual MString getCameraHUDName ()`
- `MStatus setDisplayHUD (bool display)`
- `bool displayHUD (MStatus *ReturnStatus=NULL) const`
- `MStatus drawHUDNow ()`
- `MStatus setDrawAdornments (bool display)`
- `bool drawAdornments (MStatus *ReturnStatus=NULL)`
- `MStatus drawAdornmentsNow ()`
- `MStatus setDisplayAxis (bool display)`
- `bool displayAxisOn (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayAxisAtOrigin (bool display)`
- `bool displayAxisAtOriginOn (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayCameraAnnotation (bool display)`
- `bool displayCameraAnnotationOn (MStatus *ReturnStatus=NULL) const`
- `bool isVisible (MStatus *ReturnStatus=NULL) const`
- `M3dView::DisplayStyle displayStyle (MStatus *ReturnStatus=NULL) const`
- `bool isShadeActiveOnly (MStatus *ReturnStatus=NULL) const`
- `MStatus setDisplayStyle (M3dView::DisplayStyle style, bool activeOnly=false)`
- `int portWidth (MStatus *ReturnStatus=NULL)`
- `int portHeight (MStatus *ReturnStatus=NULL)`
- `MStatus beginXorDrawing (bool drawOrthographic=true, bool disableDepthTesting=true, float lineWidth=1.0f, M3dView::LineStipplePattern stipplePattern=M3dView::kStippleNone, const MColor &lineColor=MColor(1, 1, 1))`
- `MStatus endXorDrawing ()`
- `MStatus setDrawColor (unsigned int index, M3dView::ColorTable table=M3dView::kActiveColors)`
- `MStatus setDrawColor (const MColor &color)`
- `unsigned int numDormantColors (MStatus *ReturnStatus=NULL)`
- `unsigned int numActiveColors (MStatus *ReturnStatus=NULL)`
- `unsigned int numUserDefinedColors (MStatus *ReturnStatus=NULL)`
- `MStatus setUserDefinedColor (unsigned int index, const MColor &color)`
- `unsigned int userDefinedColorIndex (unsigned int index, MStatus *ReturnStatus=NULL)`
- `bool isBackgroundGradient (MStatus *ReturnStatus=NULL)`
- `MColor templateColor (MStatus *ReturnStatus=NULL)`
- `MColor backgroundColor (MStatus *ReturnStatus=NULL)`
- `MColor backgroundColorTop (MStatus *ReturnStatus=NULL)`
- `MColor backgroundColorBottom (MStatus *ReturnStatus=NULL)`
- `MColor colorAtIndex (unsigned int index, M3dView::ColorTable table=M3dView::kActiveColors, MStatus *ReturnStatus=NULL)`
- `MStatus getColorIndexAndTable (unsigned int glindex, unsigned int &index, M3dView::ColorTable &table) const`
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &worldPt, MVector &worldVector) const`
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &nearClipPt, MPoint &farClipPt) const`
- `MStatus viewToObjectSpace (short x_pos, short y_pos, const MMatrix &localMatrixInverse, MPoint &oPt, MVector &oVector) const`
- `bool worldToView (const MPoint &worldPt, short &x_pos, short &y_pos, MStatus *ReturnStatus=NULL) const`
- `MStatus setObjectDisplay (M3dView::DisplayObjects, bool)`
- `bool objectDisplay (M3dView::DisplayObjects, MStatus *ReturnStatus=NULL)`
- `MStatus setBackfaceCulling (bool cull)`
- `bool isBackfaceCulling (MStatus *ReturnStatus=NULL) const`
- `MStatus setWireframeOnShaded (bool on)`
- `bool isWireframeOnShaded (MStatus *ReturnStatus=NULL) const`
- `MStatus setXrayEnabled (bool xray)`
- `bool isXrayEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setTextureDisplayEnabled (bool texture)`
- `bool isTextureDisplayEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setTwoSidedLighting (bool twoSided)`
- `bool isTwoSidedLighting (MStatus *ReturnStatus=NULL) const`
- `MStatus setLightingMode (MPx3dModelView::LightingMode)`
- `MPx3dModelView::LightingMode lightingMode (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogEnabled (bool state)`
- `bool isFogEnabled (MStatus *ReturnStatus=NULL) const`
- `MPx3dModelView::FogSource fogSource (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogSource (MPx3dModelView::FogSource)`
- `MPx3dModelView::FogMode fogMode (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogMode (MPx3dModelView::FogMode)`
- `double fogDensity (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogDensity (double)`
- `double fogStart (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogStart (double)`
- `double fogEnd (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogEnd (double)`
- `MColor fogColor (MStatus *ReturnStatus=NULL) const`
- `MStatus setFogColor (const MColor &)`
- `bool isBackgroundFogEnabled (MStatus *ReturnStatus=NULL) const`
- `MStatus setBackgroundFogEnabled (bool enable)`
- `MString viewSelectedPrefix (MStatus *ReturnStatus) const`
- `MStatus setViewSelectedPrefix (const MString &prefix)`
- `bool viewSelected (MStatus *ReturnStatus=NULL) const`
- `MStatus setViewSelected (bool viewSelected)`
- `MObject viewSelectedSet (MStatus *ReturnStatus=NULL) const`
- `MStatus setViewSelectedSet (const MObject &set)`
- `MStatus getObjectsToView (MSelectionList &list) const`
- `MStatus setObjectsToView (const MSelectionList &list)`
- `bool viewIsFiltered (MStatus *ReturnStatus=NULL) const`
- `MStatus filteredObjectList (MSelectionList &list) const`
- `bool hasStereoBufferSupport () const`
- `MStatus getAsM3dView (M3dView &view)`
- `virtual bool wantStereoGLBuffer () const`
- `MStatus setInStereoDrawMode (bool flag)`
- `virtual bool requestOkForDraw (const MDrawRequest &)`

**Static Public Member Functions**

- `static MPx3dModelView * getModelView (const MString &name, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MPxClientDeviceNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_client_device_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_client_device_node.html).

**Public Types**

- `enum ClientType { kUser, kInternal }`

**Public Member Functions**

- `MPxClientDeviceNode (ClientType typ=kUser)`
- `~MPxClientDeviceNode () override`
- `MPxNode::Type type () const override`
- `ClientType clientType ()`
- `virtual void threadHandler (const char *serverName, const char *deviceName)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject serverName`
- `static MObject deviceName`

### MPxContext

Module `OpenMayaUI`. Source: [class_m_px_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_context.html).

**Public Types**

- `enum ImageIndex { kImage1, kImage2, kImage3 }`

**Public Member Functions**

- `MPxContext ()`
- `virtual ~MPxContext ()`
- `virtual void toolOnSetup (MEvent &event)`
- `virtual void toolOffCleanup ()`
- `virtual MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus doHold (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus drawFeedback (MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus doPtrMoved (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`
- `virtual MStatus doPress (MEvent &event)`
- `virtual MStatus doRelease (MEvent &event)`
- `virtual MStatus doDrag (MEvent &event)`
- `virtual MStatus doHold (MEvent &event)`
- `virtual MStatus doPtrMoved (MEvent &event)`
- `virtual MStatus doEnterRegion (MEvent &event)`
- `virtual MStatus helpStateHasChanged (MEvent &event)`
- `virtual void deleteAction ()`
- `virtual void completeAction ()`
- `virtual MStatus addManipulator (const MObject &manipulator)`
- `virtual MStatus deleteManipulators ()`
- `MStatus setImage (const MString &image, ImageIndex index)`
- `MString image (ImageIndex index, MStatus *ReturnStatus=NULL) const`
- `virtual void abortAction ()`
- `virtual bool processNumericalInput (const MDoubleArray &values, const MIntArray &flags, bool isAbsolute)`
- `virtual bool feedbackNumericalInput () const`
- `virtual MSyntax::MArgType argTypeNumericalInput (unsigned int index) const`
- `virtual MString stringClassName () const`
- `virtual void getClassName (MString &name) const`
- `MStatus getImage (MString &image, ImageIndex index) const`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus setHelpString (const MString &str)`
- `MStatus setTitleString (const MString &str)`
- `MStatus setCursor (const MCursor &newCursor)`
- `MStatus beginMarquee (MEvent &event)`
- `MStatus dragMarquee (MEvent &event)`
- `MStatus releaseMarquee (MEvent &event, short &top, short &left, short &bottom, short &right)`
- `virtual MPxToolCommand * newToolCommand ()`

**Static Protected Member Functions**

- `static bool ignoreEntry (const MIntArray &flags, unsigned int entry)`

### MPxContextCommand

Module `OpenMayaUI`. Source: [class_m_px_context_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_context_command.html).

**Public Member Functions**

- `MPxContextCommand ()`
- `virtual ~MPxContextCommand ()`
- `virtual MStatus doEditFlags ()`
- `virtual MStatus doQueryFlags ()`
- `virtual MPxContext * makeObj ()`
- `virtual MStatus appendSyntax ()`
- `MStatus setResult (bool result)`
- `MStatus setResult (int result)`
- `MStatus setResult (double result)`
- `MStatus setResult (const MString &result)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`

### MPxControlCommand

Module `OpenMayaUI`. Source: [class_m_px_control_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_control_command.html).

**Public Member Functions**

- `MPxControlCommand ()`
- `virtual ~MPxControlCommand ()`
- `virtual MPxUIControl * makeControl ()=0`
- `virtual MStatus doEditFlags ()`
- `virtual MStatus doQueryFlags ()`
- `virtual MStatus appendSyntax ()`
- `virtual bool skipFlagForCreate (const MString &longFlag)`
- `void clearResult ()`
- `MStatus setResult (bool result)`
- `MStatus setResult (int result)`
- `MStatus setResult (double result)`
- `MStatus setResult (const MString &result)`
- `MStatus setResult (const MStringArray &result)`
- `MStatus setResult (const MIntArray &result)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MPxUIControl * control (MStatus *ReturnStatus=NULL)`
- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`

### MPxDragAndDropBehavior

Module `OpenMayaUI`. Source: [class_m_px_drag_and_drop_behavior.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_drag_and_drop_behavior.html).

**Public Member Functions**

- `MPxDragAndDropBehavior ()`
- `virtual ~MPxDragAndDropBehavior ()`
- `MPxDragAndDropBehavior (void *init)`
- `virtual bool shouldBeUsedFor (MObject &sourceNode, MObject &destinationNode, MPlug &sourcePlug, MPlug &destinationPlug)`
- `virtual MStatus connectAttrToAttr (MPlug &sourcePlug, MPlug &destinationPlug, bool force)`
- `virtual MStatus connectAttrToNode (MPlug &sourcePlug, MObject &destinationNode, bool force)`
- `virtual MStatus connectNodeToAttr (MObject &sourceNode, MPlug &destinationPlug, bool force)`
- `virtual MStatus connectNodeToNode (MObject &sourceNode, MObject &destinationNode, bool force)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxGlBuffer

Module `OpenMayaUI`. Source: [class_m_px_gl_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_gl_buffer.html).

**Public Member Functions**

- `MPxGlBuffer ()`
- `MPxGlBuffer (M3dView &view)`
- `MStatus openFbo (short width, short height, M3dView &)`
- `MStatus closeFbo (M3dView &)`
- `MStatus bindFbo ()`
- `MStatus unbindFbo ()`
- `virtual void beginBufferNotify ()`
- `virtual void endBufferNotify ()`

**Static Public Member Functions**

- `static const char * className ()`

### MPxHardwareShader

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_hardware_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_hardware_shader.html).

**Classes**

- `struct RenderParameters`
- `struct ShaderContext`

**Public Types**

- `enum TransparencyOptions { kIsTransparent = 0x0001, kNoTransparencyFrontBackCull = 0x0002, kNoTransparencyPolygonSort = 0x0004 }`

**Public Member Functions**

- `MPxHardwareShader ()`
- `~MPxHardwareShader () override`
- `MPxNode::Type type () const override`
- `MStatus setVaryingParameters (const MVaryingParameterList &parameters, bool remapCurrentValues=true, MDagModifier *dagModifier=NULL)`
- `MStatus setUniformParameters (const MUniformParameterList &parameters, bool remapCurrentValues=true, MDagModifier *dagModifier=NULL)`
- `virtual MStatus render (MGeometryList &iterator)`
- `virtual unsigned int transparencyOptions ()`
- `virtual const MRenderProfile & profile ()`
- `virtual MStatus populateRequirements (const MPxHardwareShader::ShaderContext &context, MGeometryRequirementsLegacy &requirements)`
- `virtual MStatus getAvailableImages (const MPxHardwareShader::ShaderContext &context, const MString &uvSetName, MStringArray &imageNames)`
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, const MString &imageName, floatRegion region, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, const MString &imageName, floatRegion region, const MPxHardwareShader::RenderParameters &parameters, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, MHWRender::MUIDrawManager &uiDrawManager, const MString &imageName, floatRegion region, const MPxHardwareShader::RenderParameters &parameters, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderSwatchImage (MImage &image)`

**Static Public Member Functions**

- `static MPxHardwareShader * getHardwareShaderPtr (MObject &object)`
- `static MString findResource (const MString &name, const MString &shaderPath, MStatus *status=NULL)`
- `static const char * className ()`

**Static Public Attributes**

- `static MObject outColor`
- `static MObject outColorR`
- `static MObject outColorG`
- `static MObject outColorB`

### MPxHwShaderNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_hw_shader_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_hw_shader_node.html).

**Classes**

- `struct RenderParameters`

**Public Types**

- `enum Writeable { kWriteNone = 0x0000, kWriteVertexArray = 0x0001, kWriteNormalArray = 0x0002, kWriteColorArrays = 0x0004, kWriteTexCoordArrays = 0x0008, kWriteAll = 0x000f }`
- `enum DirtyMask { kDirtyNone = 0x0000, kDirtyVertexArray = 0x0001, kDirtyNormalArray = 0x0002, kDirtyColorArrays = 0x0004, kDirtyTexCoordArrays = 0x0008, kDirtyAll = 0x000f }`
- `enum TransparencyOptions { kIsTransparent = 0x0001, kNoTransparencyFrontBackCull = 0x0002, kNoTransparencyPolygonSort = 0x0004 }`

**Public Member Functions**

- `MPxHwShaderNode ()`
- `~MPxHwShaderNode () override`
- `MPxNode::Type type () const override`
- `virtual MStatus bind (const MDrawRequest &request, M3dView &view)`
- `virtual MStatus unbind (const MDrawRequest &request, M3dView &view)`
- `virtual MStatus geometry (const MDrawRequest &request, M3dView &view, int prim, unsigned int writable, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays)`
- `virtual MStatus geometry (const MDrawRequest &request, M3dView &view, int prim, unsigned int writable, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays, const int *faceIDs, const float *localUVCoord)`
- `virtual MStatus glBind (const MDagPath &shapePath)`
- `virtual MStatus glUnbind (const MDagPath &shapePath)`
- `virtual MStatus glGeometry (const MDagPath &shapePath, int glPrim, unsigned int writeMask, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays)`
- `virtual MStatus glGeometry (const MDagPath &shapePath, int glPrim, unsigned int writeMask, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays, const int *faceIDs, const float *localUVCoord)`
- `virtual bool supportsBatching () const`
- `virtual bool invertTexCoords () const`
- `const MDagPath & currentPath () const`
- `unsigned int dirtyMask () const`
- `virtual int normalsPerVertex ()`
- `virtual int colorsPerVertex ()`
- `virtual int getColorSetNames (MStringArray &names)`
- `virtual int texCoordsPerVertex ()`
- `virtual int getTexCoordSetNames (MStringArray &names)`
- `virtual bool hasTransparency ()`
- `virtual bool provideVertexIDs ()`
- `virtual bool provideFaceIDs ()`
- `virtual bool provideLocalUVCoord ()`
- `virtual unsigned int transparencyOptions ()`
- `virtual MStatus getAvailableImages (const MString &uvSetName, MStringArray &imageNames)`
- `virtual MStatus renderImage (const MString &imageName, floatRegion region, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderImage (const MString &imageName, floatRegion region, const MPxHwShaderNode::RenderParameters &parameters, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderImage (MHWRender::MUIDrawManager &uiDrawManager, const MString &imageName, floatRegion region, const MPxHwShaderNode::RenderParameters &parameters, int &imageWidth, int &imageHeight)`
- `virtual MStatus renderSwatchImage (MImage &image)`
- `MObject currentShadingEngine () const`

**Static Public Member Functions**

- `static MPxHwShaderNode * getHwShaderNodePtr (MObject &object)`
- `static const char * className ()`

**Static Public Attributes**

- `static MObject outColor`
- `static MObject outColorR`
- `static MObject outColorG`
- `static MObject outColorB`
- `static MObject outTransparency`
- `static MObject outTransparencyR`
- `static MObject outTransparencyG`
- `static MObject outTransparencyB`
- `static MObject outMatteOpacity`
- `static MObject outMatteOpacityR`
- `static MObject outMatteOpacityG`
- `static MObject outMatteOpacityB`
- `static MObject outGlowColor`
- `static MObject outGlowColorR`
- `static MObject outGlowColorG`
- `static MObject outGlowColorB`

### MPxLocatorNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_locator_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_locator_node.html).

**Public Member Functions**

- `MPxLocatorNode ()`
- `~MPxLocatorNode () override`
- `MPxNode::Type type () const override`
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`
- `virtual bool isBounded () const`
- `virtual MBoundingBox boundingBox () const`
- `void getCacheSetup (const MEvaluationNode &evalNode, MNodeCacheDisablingInfo &disablingInfo, MNodeCacheSetupInfo &cacheSetupInfo, MObjectArray &monitoredAttributes) const override`
- `unsigned int color (M3dView::DisplayStatus displayStatus)`
- `MColor colorRGB (M3dView::DisplayStatus displayStatus)`
- `virtual bool excludeAsLocator () const`
- `virtual bool isTransparent () const`
- `virtual bool drawLast () const`
- `virtual bool useClosestPointForSelection () const`
- `virtual MPoint closestPoint (MPoint cursorRayPoint, MVector cursorRayDir) const`
- `virtual MSelectionMask getShapeSelectionMask () const`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject underWorldObject`
- `static MObject localPosition`
- `static MObject localPositionX`
- `static MObject localPositionY`
- `static MObject localPositionZ`
- `static MObject worldPosition`
- `static MObject worldPositionX`
- `static MObject worldPositionY`
- `static MObject worldPositionZ`
- `static MObject localScale`
- `static MObject localScaleX`
- `static MObject localScaleY`
- `static MObject localScaleZ`
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

### MPxManipContainer

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_manip_container.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_manip_container.html).

**Public Types**

- `enum baseType { kFreePointTriadManip, kDirectionManip, kDistanceManip, kPointOnCurveManip, kPointOnSurfaceManip, kDiscManip, kCircleSweepManip, kToggleManip, kStateManip, kCurveSegmentManip, kCustomManip }`
- `typedef MManipData(MPxManipContainer::* plugToManipConversionCallback) (unsigned int manipIndex)`
- `typedef MManipData(MPxManipContainer::* manipToPlugConversionCallback) (unsigned int plugIndex)`

**Public Member Functions**

- `MPxManipContainer ()`
- `~MPxManipContainer () override`
- `MPxNode::Type type () const override`
- `virtual void preDrawUI (const M3dView &view)`
- `virtual void drawUI (MHWRender::MUIDrawManager &drawManager, const MHWRender::MFrameContext &frameContext) const`
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus status)`
- `virtual MStatus connectToDependNode (const MObject &dependNode)`
- `virtual MStatus createChildren ()`
- `MDagPath addFreePointTriadManip (const MString &manipName, const MString &pointName)`
- `MDagPath addDirectionManip (const MString &manipName, const MString &directionName)`
- `MDagPath addDistanceManip (const MString &manipName, const MString &distanceName)`
- `MDagPath addPointOnCurveManip (const MString &manipName, const MString &paramName)`
- `MDagPath addPointOnSurfaceManip (const MString &manipName, const MString &paramName)`
- `MDagPath addDiscManip (const MString &manipName, const MString &angleName)`
- `MDagPath addCircleSweepManip (const MString &manipName, const MString &angleName)`
- `MDagPath addToggleManip (const MString &manipName, const MString &toggleName)`
- `MDagPath addStateManip (const MString &manipName, const MString &stateName)`
- `MDagPath addCurveSegmentManip (const MString &manipName, const MString &startParamName, const MString &endParamName)`
- `MDagPath addRotateManip (const MString &manipName, const MString &rotationName)`
- `MDagPath addScaleManip (const MString &manipName, const MString &scaleName)`
- `MStatus addMPxManipulatorNode (const MString &manipTypeName, const MString &manipName, MPxManipulatorNode *&proxyManip)`
- `bool isManipActive (const MFn::Type &manipType, MObject &manipObject)`
- `MStatus finishAddingManips ()`
- `virtual MManipData plugToManipConversion (unsigned int manipIndex)`
- `virtual MManipData manipToPlugConversion (unsigned int manipIndex)`
- `void addPlugToManipConversion (unsigned int manipIndex)`
- `unsigned int addManipToPlugConversion (MPlug &plug)`
- `void addPlugToInViewEditor (const MPlug &plug)`
- `void addPlugToManipConversionCallback (unsigned int manipIndex, plugToManipConversionCallback callback)`
- `unsigned int addManipToPlugConversionCallback (MPlug &plug, manipToPlugConversionCallback callback)`
- `MStatus getConverterManipValue (unsigned int manipIndex, unsigned int &value)`
- `MStatus getConverterManipValue (unsigned int manipIndex, double &value)`
- `MStatus getConverterManipValue (unsigned int manipIndex, double &x, double &y)`
- `MStatus getConverterManipValue (unsigned int manipIndex, MPoint &point)`
- `MStatus getConverterManipValue (unsigned int manipIndex, MVector &vector)`
- `MStatus getConverterManipValue (unsigned int manipIndex, MMatrix &matrix)`
- `MStatus getConverterManipValue (unsigned int manipIndex, MEulerRotation &rotation)`
- `MStatus getConverterManipValue (unsigned int manipIndex, MTransformationMatrix &xform)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, double &value)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, double &x, double &y)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, MPoint &point)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, MVector &vector)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, MMatrix &matrix)`
- `MStatus getConverterPlugValue (unsigned int plugIndex, MEulerRotation &rotation)`
- `virtual MStatus doPress ()`
- `virtual MStatus doDrag ()`
- `virtual MStatus doRelease ()`
- `MObject thisMObject () const override`

**Static Public Member Functions**

- `static MStatus initialize ()`
- `static MPxManipContainer * newManipulator (const MString &manipName, MObject &manipObject, MStatus *ReturnStatus=NULL)`
- `static MStatus addToManipConnectTable (MTypeId &)`
- `static MStatus removeFromManipConnectTable (MTypeId &)`
- `static const char * className ()`

### MPxManipulatorNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_manipulator_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_manipulator_node.html).

**Public Member Functions**

- `MPxManipulatorNode ()`
- `~MPxManipulatorNode () override`
- `virtual MStatus connectToDependNode (const MObject &dependNode)`
- `virtual void preDrawUI (const M3dView &view)`
- `virtual void drawUI (MHWRender::MUIDrawManager &drawManager, const MHWRender::MFrameContext &frameContext) const`
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus status)`
- `virtual MStatus doPress (M3dView &view)`
- `virtual MStatus doDrag (M3dView &view)`
- `virtual MStatus doRelease (M3dView &view)`
- `virtual MStatus doMove (M3dView &view, bool &refresh)`
- `MStatus finishAddingManips ()`
- `MStatus colorAndName (M3dView &view, MGLuint glName, bool glNameIsPickable, short colorIndex) const`
- `MStatus shouldDrawHandleAsSelected (unsigned int name, bool &useSelectedColor) const`
- `MStatus setHandleColor (MHWRender::MUIDrawManager &drawManager, unsigned int handleName, short colorIndex) const`
- `MStatus glFirstHandle (MGLuint &firstHandle)`
- `MStatus glActiveName (MGLuint &glName)`
- `MStatus mouseRay (MPoint &linePoint, MVector &lineDirection) const`
- `MStatus mouseRayWorld (MPoint &linePoint, MVector &lineDirection) const`
- `MStatus mousePosition (short &x_pos, short &y_pos)`
- `MStatus mouseDown (short &x_pos, short &y_pos)`
- `MStatus mouseUp (short &x_pos, short &y_pos)`
- `MStatus registerForMouseMove ()`
- `MStatus deregisterForMouseMove ()`
- `MStatus addDoubleValue (const MString &valueName, double defaultValue, int &valueIndex)`
- `MStatus addPointValue (const MString &valueName, const MPoint &defaultValue, int &valueIndex)`
- `MStatus addVectorValue (const MString &valueName, const MVector &defaultValue, int &valueIndex)`
- `MStatus setDoubleValue (int valueIndex, double value)`
- `MStatus setPointValue (int valueIndex, const MPoint &value)`
- `MStatus setVectorValue (int valueIndex, const MVector &value)`
- `MStatus getDoubleValue (int valueIndex, bool previousValue, double &value)`
- `MStatus getPointValue (int valueIndex, bool previousValue, MPoint &value)`
- `MStatus getVectorValue (int valueIndex, bool previousValue, MVector &value)`
- `MStatus connectPlugToValue (const MPlug &plug, int valueIndex, int &plugIndex)`
- `MStatus addDependentPlug (const MPlug &plug)`
- `MStatus dependentPlugsReset ()`
- `short mainColor () const`
- `short xColor () const`
- `short yColor () const`
- `short zColor () const`
- `short prevColor () const`
- `short lineColor () const`
- `short dimmedColor () const`
- `short selectedColor () const`
- `short labelColor () const`
- `short labelBackgroundColor () const`

**Static Public Member Functions**

- `static MPxManipulatorNode * newManipulator (const MString &manipName, MObject &manipObject, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

**Static Public Attributes**

- `static MObject connectedNodes`

### MPxMidiInputDevice

Module `OpenMayaUI`. Source: [class_m_px_midi_input_device.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_midi_input_device.html).

**Public Member Functions**

- `virtual MStatus openDevice ()`
- `virtual void closeDevice ()`
- `virtual void nameAxes ()`
- `virtual void nameButtons ()`
- `virtual MDeviceState * deviceState ()`
- `virtual MStatus sendMessage (const char *const messageType, const char *const messageParams)`
- `virtual char * getMessage (const char *const messageType, char *messageResponse)`
- `virtual void doButtonEvents (bool=true)`
- `virtual void doMovementEvents (bool=true)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus setNamedButton (const MString &, unsigned short)`
- `MStatus addChannel (MDeviceChannel &)`
- `MStatus setDegreesOfFreedom (int freedom)`
- `MStatus setNumberOfButtons (int buttons)`

### MPxModelEditorCommand

Module `OpenMayaUI`. Source: [class_m_px_model_editor_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_model_editor_command.html).

**Public Member Functions**

- `MPxModelEditorCommand ()`
- `virtual ~MPxModelEditorCommand ()`
- `virtual MStatus doEditFlags ()`
- `virtual MStatus doQueryFlags ()`
- `virtual MStatus appendSyntax ()`
- `virtual bool skipFlagForCreate (const MString &longFlag)`
- `MStatus setResult (bool result)`
- `MStatus setResult (int result)`
- `MStatus setResult (double result)`
- `MStatus setResult (const MString &result)`
- `MStatus setResult (const MStringArray &result)`
- `MStatus setResult (const MDoubleArray &result)`
- `MStatus setResult (const MIntArray &result)`
- `virtual MString editorCommandName () const`
- `virtual MString editorMenuScriptName () const`
- `virtual MPx3dModelView * makeModelView (MStatus *ReturnStatus=NULL)`
- `MPx3dModelView * modelView (MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`

### MPxPolyTweakUVInteractiveCommand

Module `OpenMayaUI`. Inherits `MPxToolCommand`. Source: [class_m_px_poly_tweak_u_v_interactive_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_poly_tweak_u_v_interactive_command.html).

**Public Member Functions**

- `MPxPolyTweakUVInteractiveCommand ()`
- `~MPxPolyTweakUVInteractiveCommand () override`
- `void setUVs (const MObject &mesh, MIntArray &uvList, MFloatArray &uPos, MFloatArray &vPos, const MString *uvSet=NULL)`
- `bool isUndoable () const override`
- `MStatus doIt (const MArgList &args) override`
- `MStatus cancel () override`
- `MStatus finalize () override`

**Static Public Member Functions**

- `static const char * className ()`

### MPxSelectionContext

Module `OpenMayaUI`. Inherits `MPxContext`. Source: [class_m_px_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_selection_context.html).

**Public Member Functions**

- `MPxSelectionContext ()`
- `~MPxSelectionContext () override`
- `MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doHold (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doPress (MEvent &event) override`
- `MStatus doRelease (MEvent &event) override`
- `MStatus doDrag (MEvent &event) override`
- `MStatus doHold (MEvent &event) override`
- `MStatus helpStateHasChanged (MEvent &) override`
- `MStatus addManipulator (const MObject &manipulator) override`
- `MStatus deleteManipulators () override`
- `MStatus setAllowPreSelectHilight ()`
- `MStatus setAllowSoftSelect ()`
- `MStatus setAllowSymmetry ()`
- `MStatus setAllowDoubleClickAction ()`
- `MStatus setImage (const MString &image, ImageIndex index)`
- `MString image (ImageIndex index, MStatus *ReturnStatus=NULL) const`
- `MStatus getImage (MString &image, ImageIndex index) const`
- `void abortAction () override`
- `bool processNumericalInput (const MDoubleArray &values, const MIntArray &flags, bool isAbsolute) override`
- `bool feedbackNumericalInput () const override`
- `MSyntax::MArgType argTypeNumericalInput (unsigned int index) const override`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `bool isSelecting ()`
- `MPoint startPoint ()`
- `MPoint lastDragPoint ()`
- `MPxToolCommand * newToolCommand () override`

### MPxSurfaceShapeUI

Module `OpenMayaUI`. Source: [class_m_px_surface_shape_u_i.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_surface_shape_u_i.html).

**Public Types**

- `enum UVSelectionType { kSelectMeshUVs, kSelectMeshVerts, kSelectMeshFaces, kSelectMeshEdges }`

**Public Member Functions**

- `MPxSurfaceShapeUI ()`
- `virtual ~MPxSurfaceShapeUI ()`
- `void getDrawData (const void *geom, MDrawData &)`
- `virtual void getDrawRequests (const MDrawInfo &, bool objectAndActiveOnly, MDrawRequestQueue &requests)`
- `virtual void draw (const MDrawRequest &, M3dView &view) const`
- `virtual bool select (MSelectInfo &selectInfo, MSelectionList &selectionList, MPointArray &worldSpaceSelectPts) const`
- `virtual bool snap (MSelectInfo &snapInfo) const`
- `virtual bool canDrawUV () const`
- `virtual void drawUV (M3dView &view, const MTextureEditorDrawInfo &info) const`
- `virtual bool selectUV (M3dView &view, UVSelectionType selType, int xmin, int ymin, int xmax, int ymax, bool singleSelect, MSelectionList &selList) const`
- `MPxSurfaceShape * surfaceShape () const`
- `MMaterial material (MDagPath &path) const`
- `void materials (MDagPath &path, MObjectArray &componentFilter, MMaterialArray &materials, MObjectArray *componentSet=NULL) const`

**Static Public Member Functions**

- `static MPxSurfaceShapeUI * surfaceShapeUI (const MDagPath &path, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MPxTexContext

Module `OpenMayaUI`. Inherits `MPxContext`. Source: [class_m_px_tex_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_tex_context.html).

**Public Member Functions**

- `MPxTexContext ()`
- `~MPxTexContext () override`
- `MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `MStatus doPtrMoved (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`
- `void viewToPort (double xView, double yView, short &xPort, short &yPort) const`
- `void portToView (short xPort, short yPort, double &xView, double &yView) const`
- `void viewRect (double &left, double &right, double &bottom, double &top) const`
- `void portSize (double &width, double &height) const`
- `MPxToolCommand * newToolCommand () override`

**Static Public Member Functions**

- `static bool getMarqueeSelection (double xMin, double yMin, double xMax, double yMax, const MSelectionMask &mask, bool bPickSingle, bool bIgnoreSelectionMode, MSelectionList &selectionList)`
- `static const char * className ()`

### MPxThreadedDeviceNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_threaded_device_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_threaded_device_node.html).

**Public Member Functions**

- `MPxThreadedDeviceNode ()`
- `~MPxThreadedDeviceNode () override`
- `MPxNode::Type type () const override`
- `virtual void threadHandler ()`
- `virtual void threadShutdownHandler ()`
- `bool isLive ()`
- `bool isDone ()`
- `MStatus setDone (bool done)`
- `MStatus beginThreadLoop ()`
- `MStatus endThreadLoop ()`
- `MStatus createMemoryPools (int nCount, int nElements, long nSize)`
- `MStatus destroyMemoryPools ()`
- `MStatus acquireDataStorage (MCharBuffer &buffer)`
- `MStatus pushThreadData (MCharBuffer &buffer)`
- `MStatus popThreadData (MCharBuffer &buffer)`
- `long threadDataCount ()`
- `MStatus releaseDataStorage (MCharBuffer &buffer)`
- `MStatus setRefreshOutputAttributes (const MObjectArray &attributeList)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject output`
- `static MObject live`
- `static MObject frameRate`

### MPxToolCommand

Module `OpenMayaUI`. Inherits `MPxCommand`. Source: [class_m_px_tool_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_tool_command.html).

**Public Member Functions**

- `MPxToolCommand ()`
- `~MPxToolCommand () override`
- `MStatus doIt (const MArgList &args) override=0`
- `virtual MStatus cancel ()`
- `virtual MStatus finalize ()`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus doFinalize (MArgList &command)`

### MPxUIControl

Module `OpenMayaUI`. Source: [class_m_px_u_i_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_u_i_control.html).

**Public Member Functions**

- `virtual ~MPxUIControl ()`

**Static Public Member Functions**

- `static const char * className ()`

### MPxUITableControl

Module `OpenMayaUI`. Inherits `MPxUIControl`. Source: [class_m_px_u_i_table_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_u_i_table_control.html).

**Public Types**

- `enum MLabelType { kNoLabel, kRowLabel, kColumnLabel, kAllLabels }`

**Public Member Functions**

- `MPxUITableControl (MPxControlCommand &)`
- `~MPxUITableControl () override`
- `MStatus redrawLabels (MLabelType lt=kAllLabels)`
- `MStatus redrawCells ()`
- `MStatus addToSelection (unsigned int row, unsigned int col)`
- `MStatus removeFromSelection (unsigned int row, unsigned int column)`
- `MStatus clearSelection ()`
- `MStatus setSelection (unsigned int row, unsigned int column)`
- `MStatus setSelection (unsigned int firstRow, unsigned int lastRow, unsigned int firstCol, unsigned int lastCol)`
- `MStatus setNumberOfRows (unsigned int count)`
- `unsigned int numberOfRows (MStatus *ReturnStatus=nullptr)`
- `MStatus setNumberOfColumns (unsigned int count)`
- `unsigned int numberOfColumns (MStatus *ReturnStatus=nullptr) const`
- `bool suspendUpdates (bool update, MStatus *ReturnStatue=nullptr)`
- `bool isSelected (unsigned int row, unsigned int col, MStatus *status=nullptr)`
- `virtual bool collapseOrExpandRow (unsigned int row)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `virtual MString cellString (unsigned int r, unsigned int c, bool &ValidCell)`
- `virtual MString labelString (MLabelType labelType, unsigned int n)`
- `virtual bool getCell (unsigned int r, unsigned int c, MString &value)`
- `virtual void getLabel (MLabelType labelType, unsigned int n, MString &value)`
- `virtual bool allowEdit () const`
- `virtual bool allowSelection (int top, int left, int bottom, int right)`
- `virtual bool getCellColor (unsigned int r, unsigned int c, int &red, int &green, int &blue)`

### MQtUtil

Module `OpenMayaUI`. Source: [class_m_qt_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_qt_util.html).

**Public Types**

- `typedef QWidget *(* UITypeCreatorFn) (QWidget *parent, const QString &name)`

**Static Public Member Functions**

- `static QWidget * findControl (MString controlName, const QWidget *ancestor=NULL)`
- `static QWidget * findLayout (MString layoutName, const QWidget *ancestor=NULL)`
- `static QAction * findMenuItem (MString itemName)`
- `static QWidget * findWindow (MString windowName)`
- `static MString fullName (const QObject *uiElement)`
- `static QWidget * getCurrentParent ()`
- `static QPixmap * createPixmap (const MString &imageName, bool autoScale=true)`
- `static QCursor * createCursor (const MString &cursorName, bool autoScale=true)`
- `static QIcon * createIcon (const MString &iconName, bool autoScale=true)`
- `static QList< QObject * > getLayoutChildren (const QWidget *layout)`
- `static QObject * getParent (const QObject *uiElement)`
- `static MString addWidgetToMayaLayout (QWidget *control, QWidget *layout, const MString &uiType=MString())`
- `static bool registerUIType (const MString &className, UITypeCreatorFn, const MString &command)`
- `static bool deregisterUIType (const MString &className)`
- `static QWidget * mainWindow ()`
- `static QGLContext * resourceGLContext ()`
- `static MNativeWindowHdl nativeWindow (const QWidget *control)`
- `static MString toMString (const QString &qstr)`
- `static QString toQString (const MString &mstr)`
- `static int dpiScale (int size)`
- `static float dpiScale (float size)`
- `static QDoubleValidator * newClocaleValidator (double bottom, double top, int decimals, QObject *parent=nullptr)`
- `static const char * className ()`

### MSelectInfo

Module `OpenMayaUI`. Inherits `MDrawInfo`. Source: [class_m_select_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_select_info.html).

**Public Member Functions**

- `MSelectInfo ()`
- `MSelectInfo (const MSelectInfo &in)`
- `~MSelectInfo () override`
- `M3dView view ()`
- `bool singleSelection () const`
- `bool selectClosest () const`
- `bool selectable (MSelectionMask &mask) const`
- `bool selectableComponent (bool displayed, MSelectionMask &mask) const`
- `void selectRect (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`
- `bool isRay () const`
- `MMatrix getAlignmentMatrix () const`
- `void getLocalRay (MPoint &, MVector &) const`
- `bool selectForHilite (const MSelectionMask &) const`
- `bool selectOnHilitedOnly () const`
- `int highestPriority () const`
- `void setHighestPriority (int)`
- `void addSelection (const MSelectionList &item, const MPoint &point, MSelectionList &list, MPointArray &points, const MSelectionMask &mask, bool isComponent)`
- `bool setSnapPoint (const MPoint &point)`
- `MDagPath selectPath () const`

**Static Public Member Functions**

- `static const char * className ()`

### MTemplateContextCommand&lt; ContextCommandName, ContextCommandClass, ContextClass &gt;

Module `OpenMayaUI`. Inherits `MPxContextCommand`. Source: [class_m_template_context_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_context_command.html).

**Public Member Functions**

- `MTemplateContextCommand ()`
- `MPxContext * makeObj () override`

**Static Public Member Functions**

- `static void * creator ()`
- `static MStatus registerContextCommand (MObject obj)`
- `static MStatus deregisterContextCommand (MObject obj)`

### MTemplateSelectionContext&lt; ContextNameString, ContextClass, NodeType, ManipulatorClass, ManipulatorNodeName &gt;

Module `OpenMayaUI`. Inherits `MPxSelectionContext`. Source: [class_m_template_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_selection_context.html).

**Public Member Functions**

- `MTemplateSelectionContext ()`
- `~MTemplateSelectionContext () override`
- `void toolOnSetup (MEvent &event) override`
- `void toolOffCleanup () override`
- `virtual void namesOfAttributes (MStringArray &namesOfAttributes)`
- `virtual void setInitialState ()`
- `bool validGeometrySelected ()`

**Static Public Member Functions**

- `static void updateManipulators (void *data)`

### MTextureEditorDrawInfo

Module `OpenMayaUI`. Source: [class_m_texture_editor_draw_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_texture_editor_draw_info.html).

**Public Types**

- `enum DrawingFunction { kDrawFunctionFirst = 1, kDrawWireframe = kDrawFunctionFirst, kDrawEverything, kDrawVertexForSelect, kDrawEdgeForSelect, kDrawFacetForSelect, kDrawUVForSelect, kDrawFunctionLast = kDrawUVForSelect }`

**Public Member Functions**

- `MTextureEditorDrawInfo ()`
- `MTextureEditorDrawInfo (const MTextureEditorDrawInfo &in)`
- `virtual ~MTextureEditorDrawInfo ()`
- `DrawingFunction drawingFunction () const`
- `void setDrawingFunction (DrawingFunction func)`

**Static Public Member Functions**

- `static const char * className ()`

### MToolsInfo

Module `OpenMayaUI`. Source: [class_m_tools_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_tools_info.html).

**Static Public Member Functions**

- `static void setDirtyFlag (const MPxContext &context)`
- `static void resetDirtyFlag ()`
- `static bool isDirty ()`
- `static const char * className ()`

### MUiMessage

Module `OpenMayaUI`. Inherits `MMessage`. Source: [class_m_ui_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ui_message.html).

**Static Public Member Functions**

- `static MCallbackId addUiDeletedCallback (const MString &uiName, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId addCameraChangedCallback (const MString &panelName, MMessage::MStringNode func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewDestroyMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewPreRenderMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewPostRenderMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewPreMultipleDrawPassMsgCallback (const MString &panelName, MUiMessage::MStringIndexFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewPostMultipleDrawPassMsgCallback (const MString &panelName, MUiMessage::MStringIndexFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewRendererChangedCallback (const MString &panelName, MMessage::MThreeStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static MCallbackId add3dViewRenderOverrideChangedCallback (const MString &panelName, MMessage::MThreeStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`
