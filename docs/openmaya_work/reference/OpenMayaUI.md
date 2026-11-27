# OpenMayaUI, API module for user interface

Transcribed from the reference's 2022 C++ API reference, module page https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_u_i.html.

This is a class and member transcription: names, inheritance, signatures and the one-line descriptions on the class pages. Parameter and return prose that only appears in the per-member detail sections is not reproduced; follow the source link on a class to read it.

Classes in this module: 65.

## Classes

### M3dView

Module `OpenMayaUI`. Source: [class_m3d_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m3d_view.html).

A 3-D view. M3dView provides methods for working with 3D model views. 3D views are based on OpenGL drawing areas. Maya can operate in two different color modes, RGBA and color index. Color index mode is used to increase performance when shading is not required. Drawing in color index mode is more complicated, but this class provides methods to simplify color selection. Maya has four color tables that can be used in RGBA, and that must be used in color index mode. These four color tables represent four sets of bit planes that are independent of each other. So, for example, it is possible to clear all active objects from the display and redraw them without redrawing the dormant and templated objects. The active and dormant color tables contain the same colors, but use different bitplanes. The extra performance of color index mode comes at the cost of a limited number of colors. If this restriction causes difficulty, then it is possible for the user to force all displays into RGBA mode where any color may be used. When an object is affected by another in the scene, it is drawn in a magenta colour by default. This is denoted in the DisplayStatus enum by kActiveAffected. These objects are drawn in the active planes even though they are dormant for performance reasons.

**Public Types**

- `enum DisplayStyle { kBoundingBox, kFlatShaded, kGouraudShaded, kWireFrame, kPoints }`  
  Display styles for a 3D view.
- `enum DisplayStatus { kActive, kLive, kDormant, kInvisible, kHilite, kTemplate, kActiveTemplate, kActiveComponent, kLead, kIntermediateObject, kActiveAffected, kNoStatus }`  
  Drawing modes for individual objects.
- `enum ColorTable { kActiveColors = kActive, kDormantColors = kDormant, kTemplateColor = kTemplate, kBackgroundColor }`  
  Reference to color palettes.
- `enum TextPosition { kLeft, kCenter, kRight }`  
  Alignment values when drawing text.
- `enum DisplayObjects { kDisplayEverything = ~0, kDisplayNurbsCurves = 1 << 0, kDisplayNurbsSurfaces = 1 << 1, kDisplayMeshes = 1 << 2, kDisplayPlanes = 1 << 3, kDisplayLights = 1 << 4, kDisplayCameras = 1 << 5, kDisplayJoints = 1 << 6, kDisplayIkHandles = 1 << 7, kDisplayDeformers = 1 << 8, kDisplayDynamics = 1 << 9, kDisplayParticleInstancers = 1 << 10, kDisplayLocators = 1 << 11, kDisplayDimensions = 1 << 12, kDisplaySelectHandles = 1 << 13, kDisplayPivots = 1 << 14, kDisplayTextures = 1 << 15, kDisplayGrid = 1 << 16, kDisplayCVs = 1 << 17, kDisplayHulls = 1 << 18, kDisplayStrokes = 1 << 19, kDisplaySubdivSurfaces = 1 << 20, kDisplayFluids = 1 << 21, kDisplayFollicles = 1 << 22, kDisplayHairSystems = 1 << 23, kDisplayImagePlane = 1 << 24, kDisplayNCloths = 1 << 25, kDisplayNRigids = 1 << 26, kDisplayDynamicConstraints = 1 << 27, kDisplayManipulators = 1 << 28, kDisplayNParticles = 1 << 29, kExcludeMotionTrails = 1 << 30, kExcludePluginShapes = 1 << 31 }`  
  Display modes Bit masks used in combination with the return value of the dirtyMask() method to determine which portions of the geometry are dirty.
- `enum LightingMode { kLightAll, kLightSelected, kLightActive, kLightDefault, kUnused1, kLightNone }`  
  Lighting mode used in this 3D view.
- `enum RendererName { kDefaultQualityRenderer, kHighQualityRenderer, kViewport2Renderer, kExternalRenderer }`  
  Current hardware rendering engine used in this view.
- `enum DepthBufferFormat { kDepth_8, kDepth_Float }`  
  Possible depth buffer formats to read into.
- `enum LineStipplePattern { kStippleNone, kStippleDashed }`  
  Line stipple pattern.

**Public Member Functions**

- `MStatus makeSharedContextCurrent ()`  
  makes the shared context current.
- `MGLContext display (MStatus *ReturnStatus=NULL)`  
  Mac OS X and Windows.
- `void getScreenPosition (int &x, int &y, MStatus *ReturnStatus=NULL) const`  
  Returns the current position of this view window in screen coordinates.
- `QWidget * widget (MStatus *ReturnStatus=NULL) const`  
  Returns the view's Qt widget.
- `MNativeWindowHdl window (MStatus *ReturnStatus=NULL)`  
  Returns the native window for this view.
- `int portWidth (MStatus *ReturnStatus=NULL)`  
  Returns the width of the current viewport.
- `int portHeight (MStatus *ReturnStatus=NULL)`  
  Returns the height of the current viewport.
- `bool isVisible (MStatus *ReturnStatus=NULL)`  
  Returns true if this viewport is visible.
- `int playblastPortWidth (MStatus *ReturnStatus=NULL)`  
  Returns the port width of current playblast.
- `int playblastPortHeight (MStatus *ReturnStatus=NULL)`  
  Returns the port height of current playblast.
- `MStatus pushViewport (unsigned int x, unsigned int y, unsigned int width, unsigned int height)`  
  Set the current viewport dimensions.
- `MStatus popViewport ()`  
  Pop the current viewport off of the viewport stack.
- `MStatus viewport (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`  
  Get the current viewport dimensions.
- `MStatus beginGL ()`  
  This method is obsolete.
- `MStatus endGL ()`  
  This method is obsolete.
- `void beginSelect (GLuint *buffer=NULL, GLsizei size=0)`  
  Start selecting.
- `GLint endSelect ()`  
  Finish a selection sequence.
- `bool selectMode () const`  
  Tells if this M3dView is in selection mode.
- `bool textureMode () const`  
  Tells if this M3dView is in texture mode.
- `void loadName (GLuint name)`  
  Replace the top of the name stack with the given name.
- `void pushName (GLuint name)`  
  Push a new name on the name stack.
- `void popName ()`  
  Removes the top of the name stack.
- `void initNames ()`  
  Reset the name stack.
- `MStatus beginXorDrawing (bool drawOrthographic=true, bool disableDepthTesting=true, float lineWidth=1.0f, LineStipplePattern stipplePattern=kStippleNone, const MColor &lineColor=MColor(1, 1, 1))`  
  Setup the context for exclusive-or (XOR) drawing.
- `MStatus endXorDrawing ()`  
  Reset the context to non-exclusive-or (non-XOR) screen drawing.
- `MStatus setDrawColor (unsigned int index, ColorTable table=kActiveColors)`  
  This method is obsolete.
- `MStatus setDrawColor (const MColor &color)`  
  This method is obsolete.
- `MStatus setDrawColorAndAlpha (const MColor &color)`  
  This method is obsolete.
- `unsigned int numDormantColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of dormant object colors in the internal application color table.
- `unsigned int numActiveColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of active object colors in the internal application color table.
- `unsigned int numUserDefinedColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of user defined colors in the internal application color table.
- `MStatus setUserDefinedColor (unsigned int index, const MColor &color)`  
  Sets the user defined color at the given index.
- `unsigned int userDefinedColorIndex (unsigned int index, MStatus *ReturnStatus=NULL)`  
  Returns the index for the given user-defined color.
- `MColor colorAtIndex (unsigned int index, ColorTable table=kActiveColors, MStatus *ReturnStatus=NULL)`  
  Returns the value of the color at the given index in the application's color table.
- `MStatus getColorIndexAndTable (unsigned int glindex, unsigned int &index, ColorTable &table) const`  
  Returns the color table and index representing the given OpenGL color-index value.
- `MStatus colorMask (bool &r, bool &g, bool &b, bool &a)`  
  This method is obsolete.
- `MStatus setColorMask (bool r, bool g, bool b, bool a)`  
  This method is obsolete.
- `MStatus drawText (const MString &text, const MPoint position, TextPosition textPosition=kLeft)`  
  This method is obsolete.
- `MStatus getCamera (MDagPath &camera)`  
  Get the camera for this view.
- `MStatus setCamera (MDagPath &camera)`  
  Set the camera for this view.
- `MStatus scheduleRefresh ()`  
  Schedule a forced refresh for this 3d-view.
- `MStatus refresh (bool all=false, bool force=false)`  
  Refresh the this view.
- `MStatus refresh (bool all, bool force, bool offscreen)`  
  Refresh the this view.
- `MStatus refresh (MPxGlBuffer &buffer)`  
  Refresh the this view into the GL buffer buffer.
- `MStatus refresh (MPxGlBuffer &buffer, bool offscreen)`  
  Refresh the this view into the GL buffer buffer.
- `MStatus refresh (MPxGlBuffer &buffer, bool offscreen, const MMatrix &projectionMatrix)`  
  Refresh the this view into the GL buffer buffer.
- `MStatus getLightCount (unsigned int &count, bool visible=true)`  
  Get the number of lights for the view.
- `MStatus getLightingMode (LightingMode &mode)`  
  Get the current lighting mode for the view.
- `MStatus getLightPath (unsigned int lightNumber, MDagPath &light)`  
  Get the path to a certain light.
- `MStatus isLightVisible (unsigned int lightNumber, bool &visible)`  
  Find out if a light is visible in the view.
- `MStatus getLightIndex (unsigned int lightNumber, unsigned int &lightIndex)`  
  Get the internal light index for a given light number.
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &worldPt, MVector &worldVector) const`  
  Takes a point in port coordinates and returns a corresponding ray in world coordinates.
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &nearClipPt, MPoint &farClipPt) const`  
  Takes a point in port coordinates and returns a point on the near and far clipping planes.
- `MStatus viewToObjectSpace (short x_pos, short y_pos, const MMatrix &localMatrixInverse, MPoint &oPt, MVector &oVector) const`  
  Takes a point in port coordinates and returns a corresponding ray in object coordinates.
- `bool worldToView (const MPoint &worldPt, short &x_pos, short &y_pos, MStatus *ReturnStatus=NULL) const`  
  converts a point in world space to port space.
- `MStatus projectionMatrix (MMatrix &projectionMatrix) const`  
  Returns the projection matrix currently being used by OpenGL in the current view.
- `MStatus modelViewMatrix (MMatrix &modelViewMatrix) const`  
  Returns the modelview matrix currently being used by OpenGL in the current view.
- `MString viewSelectedPrefix (MStatus *ReturnStatus) const`  
  Returns the Returns the prefix used when displaying the camera name in the heads up display when view selected in on.
- `MStatus setViewSelectedPrefix (const MString &prefix)`  
  Sets the prefix for the camera name as displayed in the heads up display when view selected is enabled.
- `bool showViewSelectedChildren (MStatus *ReturnStatus) const`  
  Returns turn if view selected shows all of the children of the obejcts that are flagged for view selected.
- `MStatus setShowViewSelectedChildren (bool)`  
  This method changes the way that view selected works.
- `DisplayStyle displayStyle (MStatus *ReturnStatus=NULL) const`  
  Return the display style for this 3d view.
- `bool isShadeActiveOnly (MStatus *ReturnStatus=NULL) const`  
  Returns true if this view's display style is shaded for objects that are active and wireframe otherwise.
- `MStatus setDisplayStyle (DisplayStyle style, bool activeOnly=false)`  
  Sets the display style for this view.
- `MStatus setObjectDisplay (unsigned int displayMask)`  
  It is assumed in the below accessor methods that M3dView::DisplayObjects enum values can be cast to and from TexcludeObjectDisplay values impicitly, but that is not enforced elsewhere.
- `unsigned int objectDisplay (MStatus *ReturnStatus=NULL)`  
  Returns a display object mask that indicates which object types are drawn in the current view.
- `MStatus setPluginObjectDisplay (const MString &pluginDisplayFilter, bool on)`  
  Enables or disables a user-defined display filter (i.e.
- `bool pluginObjectDisplay (const MString &pluginDisplayFilter, MStatus *ReturnStatus=NULL)`  
  Returns true if the plugin display filter specified by the pluginDisplayFilter is enabled in the current view.
- `RendererName getRendererName (MStatus *ReturnStatus) const`  
  Get the name of the current renderer being used for drawing to this view.
- `MString rendererString (MStatus *ReturnStatus=NULL) const`  
  Get the string name of the current renderer being used for drawing to this view.
- `bool wireframeOnlyInShadedMode (MStatus *ReturnStatus) const`  
  Return whether we are in shaded mode, but that only non shaded drawing should occur (wireframe).
- `bool wireframeOnShaded (MStatus *ReturnStatus=NULL) const`  
  Return whether we draw wireframe in shaded mode.
- `bool xray (MStatus *ReturnStatus=NULL) const`  
  Return true if the X-Ray mode is enabled.
- `bool xrayJoints (MStatus *ReturnStatus=NULL) const`  
  Return true if the X-Ray Joints mode is enabled.
- `bool twoSidedLighting (MStatus *ReturnStatus=NULL) const`  
  Return true if the Two-sided lighting mode is enabled.
- `MStatus readColorBuffer (MImage &image, bool readRGBA=false)`  
  This method is obsolete.
- `MStatus writeColorBuffer (const MImage &image, signed short x=0, signed short y=0) const`  
  This method is obsolete.
- `MStatus readDepthMap (unsigned short x, unsigned short y, unsigned int width, unsigned int height, unsigned char *bufferPtr, DepthBufferFormat depthMapPrecision)`  
  This method is obsolete.
- `MStatus readBufferTo2dTexture (unsigned short x, unsigned short y, unsigned int width, unsigned int height)`  
  This method is obsolete.
- `bool usingMipmappedTextures () const`  
  Returns if the view is using mipmapped texture display.
- `bool usingDefaultMaterial () const`  
  Returns true if the view is currently displaying objects using the default material.
- `MStatus setDisallowPolygonOffset (bool v)`  
  Certain Maya actions will use glPolygonOffset to offset polygons drawing into the depth buffer.
- `bool disallowPolygonOffset () const`  
  Returns the current state of the disallow polygon offset bit.
- `MStatus updateViewingParameters ()`  
  This method tells the camera to set the view's transformation matrix.
- `bool multipleDrawEnabled () const`  
  This method returns the multiple draw enable state for this view.
- `void setMultipleDrawEnable (bool enable)`  
  This method is obsolete.
- `unsigned multipleDrawPassCount ()`  
  This method is obsolete.
- `void setMultipleDrawPassCount (unsigned count)`  
  This method is obsolete.
- `MStatus beginProjMatrixOverride (MMatrix &projectionMatrix)`  
  This method is obsolete.
- `MStatus endProjMatrixOverride ()`  
  This method is obsolete.
- `MStatus setRenderOverrideName (const MString &name)`  
  Set the name of a render override (MHWRender::MRenderOverride) to use.
- `MString renderOverrideName (MStatus *ReturnStatus=NULL) const`  
  Get the current render override name.
- `MStatus setObjectListFilterName (const MString &name)`  
  Set the name of the object list filter (MObjectListFilter) to use.
- `MString objectListFilterName (MStatus *ReturnStatus=NULL) const`  
  Get the current object list filter name.
- `MStatus setShowObjectFilterNameInHUD (bool show)`  
  Sets whether or not to display the object filter UI name in the heads up display when an object filter is active.
- `bool showObjectFilterNameInHUD (MStatus *ReturnStatus) const`  
  Returns whether the object filter UI name is shown in the heads up display when an object filter is active.
- `bool viewIsFiltered (MStatus *ReturnStatus=NULL) const`  
  Returns the state of view filtering for this view.
- `MStatus filteredObjectList (MSelectionList &list) const`  
  Returns a selection list containing all of the objects that remain after filtering is applied to the view.
- `MStatus getRendererString (MString &stringName) const`  
  This method is not available in Python.

**Static Public Member Functions**

- `static M3dView active3dView (MStatus *ReturnStatus=NULL)`  
  Returns the active view in the form of a class (M3dView) that can operate on it.
- `static unsigned int numberOf3dViews ()`  
  Returns the number of 3D views currently in existance.
- `static MStatus get3dView (const unsigned int index, M3dView &view)`  
  Returns the 3D view at the given index.
- `static DisplayStatus displayStatus (const MDagPath &path, MStatus *ReturnStatus=NULL)`  
  Returns the display status of the given DAG path.
- `static MNativeWindowHdl applicationShell (MStatus *ReturnStatus=NULL)`  
  Returns the native handle for Maya's main window.
- `static bool isBackgroundGradient (MStatus *ReturnStatus=NULL)`  
  Returns whether a gradient is being used as the background color.
- `static MColor templateColor (MStatus *ReturnStatus=NULL)`  
  Returns the value of the template color.
- `static MColor backgroundColor (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background color.
- `static MColor backgroundColorTop (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background gradient top color.
- `static MColor backgroundColorBottom (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background gradient bottom color.
- `static MColor liveColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for live objects.
- `static MColor referenceLayerColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for objects which belong to a display layer whose display type is Reference.
- `static MColor activeTemplateColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for active template objects.
- `static MColor leadColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for lead objects.
- `static MColor hiliteColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for hilited objects.
- `static MColor activeAffectedColor (MStatus *ReturnStatus=NULL)`  
  Returns the color for active affected objects.
- `static MStatus scheduleRefreshAllViews ()`  
  Schedule a forced refresh for all 3d-views.
- `static MStatus getM3dViewFromModelPanel (const MString &modelPaneName, M3dView &view)`  
  Given the name of a model panel, get the M3dView used by that panel.
- `static MStatus getM3dViewFromModelEditor (const MString &modelPaneName, M3dView &view)`  
  Given the name of a model editor, get the M3dView used by that editor.
- `static const char * className ()`  
  Returns the name of this class.


### MCharBuffer

Module `OpenMayaUI`. Source: [class_m_char_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_char_buffer.html).

Memory buffer utility class. Simple utility class so that we do not pass a reference to a char*

**Public Member Functions**

- `MCharBuffer ()`  
  Constructor.
- `char * ptr ()`  
  Returns the pointer of the buffer.
- `long size ()`  
  Returns the size of the buffer.

**Friends**

- `class MPxThreadedDeviceNode`


### MCursor

Module `OpenMayaUI`. Source: [class_m_cursor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_cursor.html).

Manipulate Cursors. The MCursor class implements a cursor class, and is used to pass all cursor arguments to Maya API methods. The cursor image and mask are stored in xbm format to the constructor along with the cursor dimensions and the cursor hotspot.

**Public Member Functions**

- `MCursor (short width, short height, short hotSpotX, short hotSpotY, unsigned char *bits, unsigned char *mask)`  
  Class constructor.
- `MCursor (const MCursor &other)`  
  Copy constructor.
- `~MCursor ()`  
  The class destructor.
- `MCursor & operator= (const MCursor &)`  
  Assignment operator.
- `bool operator== (const MCursor &) const`  
  Equality operator.
- `bool operator!= (const MCursor &) const`  
  Inequality operator.

**Static Public Attributes**

- `static const MCursor defaultCursor`  
  Maya default cursor, the left arrow.
- `static const MCursor crossHairCursor`  
  '+' cursor.
- `static const MCursor doubleCrossHairCursor`  
  '+' cursor with double lines.
- `static const MCursor editCursor`  
  Wedge-shaped arrow pointing left.
- `static const MCursor pencilCursor`  
  Pencil shape.
- `static const MCursor handCursor`  
  Open hand shaped cursor.


### MDeviceChannel

Module `OpenMayaUI`. Source: [class_m_device_channel.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_device_channel.html).

Input device channel. Input device channel class.

**Public Member Functions**

- `MDeviceChannel (const MString &, MDeviceChannel *=NULL, int=-1)`  
  Constructor.
- `~MDeviceChannel ()`  
  Class destructor.
- `MString name () const`  
  Return the short name of the channel.
- `MString longName () const`  
  Return the long name of the channel.
- `int axisIndex () const`  
  Returns the device state index corresponding to this device channel.
- `bool hasChildren () const`  
  Determine whether this channel has children.
- `MDeviceChannel parent () const`  
  Return the parent of this channel.
- `MDeviceChannel childByIndex (int)`  
  Return the specified child of this channel.
- `int numChildren () const`  
  Return the number of children of this channel.


### MDeviceState

Module `OpenMayaUI`. Source: [class_m_device_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_device_state.html).

Input device state. MDeviceState is a generic event class for input devices. Input device classes (such as MPxMidiInputDevice) are responsible for converting specific event types to an MDeviceState which Maya understands.

**Public Member Functions**

- `virtual ~MDeviceState ()`  
  Class destructor.
- `int devicePosition (const unsigned short int axis) const`  
  Returns the position of the device for the specified axis.
- `int devicePosition (const MString &axisName) const`  
  Returns the position of the device for the specified axis.
- `void setDevicePosition (const int position, const unsigned short int axis)`  
  Sets the position of the device for the specified axis.
- `void setDevicePosition (const int position, const MString &axisName)`  
  Sets the position of the device for the specified axis.
- `bool buttonState (const unsigned short int button) const`  
  Returns the state of the given button.
- `bool buttonState (const MString &buttonName) const`  
  Returns the state of the named button.
- `void setButtonState (const bool state, const unsigned short int button)`  
  Set the state of the specified button.
- `void setButtonState (const bool state, const MString &buttonName)`  
  Set the state of the specified button.
- `int maxAxis () const`  
  Return the value of the axis with the largest value.
- `bool isNull ()`  
  Returns true if this device state is NULL;.


### MDrawData

Module `OpenMayaUI`. Source: [class_m_draw_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_data.html).

The MDrawData class holds geometry specific information for user defined shapes which maya does not intrinsicly know about. This class is used in the draw methods of MPxSurfaceShapeUI For each draw request you must create and add a draw data object which will contain geometry specific information that you will need in the subsequent call to MPxSurfaceShapeUI::draw. MDrawData contains one void* member which is a pointer to an object that you define. This object is the geometry needed to draw your shape. To create draw data use the function MPxSurfaceShapeUI::getDrawData. This function takes two arguments, the first is a pointer to your geometry object, the second is the draw data being created. To add the data to a request use MDrawRequest::setDrawData. Draw data is also used to carry texture information to your draw method. For materials with texture you must call MMaterial::evaluateTexture from your MPxSurfaceShapeUI::getDrawRequests method. Then in your draw method use MMaterial::applyTexture to set up the viewport to draw using the textured material. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MRenderItem and MHWRender::MPxGeometryOverride instead for Viewport 2.0 support.

**Public Member Functions**

- `MDrawData ()`  
  Constructor.
- `MDrawData (const MDrawData &in)`  
  Copy constructor.
- `~MDrawData ()`  
  Destructor.
- `void * geometry ()`  
  Returns the geometry associated with this draw data object.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MDrawData & operator= (const MDrawData &other)`  
  Operator =.


### MDrawInfo

Module `OpenMayaUI`. Source: [class_m_draw_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_info.html).

This class is used by the getDrawRequests method of MPxSurfaceShapeUI to specify the current object drawing state for a user defined shape. This getPrototype method is used to construct a draw request object based on the current draw state for the object. See MDrawRequest for more information. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MRenderItem and MHWRender::MPxGeometryOverride instead for Viewport 2.0 support.

**Public Member Functions**

- `MDrawInfo ()`  
  Constructor.
- `MDrawInfo (const MDrawInfo &in)`  
  Copy constructor.
- `virtual ~MDrawInfo ()`  
  Destructor.
- `MDrawRequest getPrototype (const MPxSurfaceShapeUI &drawHandler) const`  
  This method creates a draw request based on the current draw state.
- `M3dView view () const`  
  Returns the view that the drawing will take place.
- `const MDagPath multiPath () const`  
  Returns the path to the object to be drawn.
- `void setMultiPath (const MDagPath &)`  
  Sets the path of the object to be drawn.
- `const MMatrix projectionMatrix () const`  
  Returns the camera*projection matrix.
- `const MMatrix inclusiveMatrix () const`  
  Returns the world space inclusive matrix.
- `M3dView::DisplayStyle displayStyle () const`  
  The display appearance.
- `M3dView::DisplayStatus displayStatus () const`  
  Returns the status of the object to draw.
- `bool objectDisplayStatus (M3dView::DisplayObjects) const`  
  Determines whether the specified objects are allowed to be displayed.
- `bool pluginObjectDisplayStatus (const MString &pluginDisplayFilter) const`  
  Determines whether the specified plugin object is allowed to be displayed.
- `bool inSelect () const`  
  Returns true if this is called from within the select method of MPxSurfaceShapeUI.
- `bool inUserInteraction () const`  
  Returns true during any interactive refresh, as when user is interacting with the scene in any way including camera changes, object or component TRS changes, etc.
- `bool userChangingViewContext () const`  
  Returns true during any interactive refresh, as when user is changing the view using view context tools such as tumble, dolly or track.
- `bool completelyInside () const`  
  Returns true if the object being drawn is inside the viewing frustum.
- `bool canDrawComponent (bool isDisplayOn, const MSelectionMask &compMask) const`  
  Convenience method to test if components specified by the given mask can be drawn.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDrawRequest

Module `OpenMayaUI`. Source: [class_m_draw_request.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_request.html).

This class encapsulates all the information needed to fulfill a request to draw an object or part of an object. This class is used by the draw methods of MPxSurfaceShapeUI derived objects. The draw request should be created in the overridden MPxSurfaceShapeUI::getDrawRequests method. Once created the appropriate "set" methods of this class should be used to define what is being requested. Then the request should be placed on the draw reqeust queue using MDrawRequestQueue::add. When your request gets processed by maya, your overriden MPxSurfaceShape::draw method will get called with your request. Use the query methods of this class to determine what to draw. You create a draw request using the method MDrawInfo::getPrototype. A draw request automatically picks up certain information (listed below) upon its creation. So you don't have to set any of this information unless you want to change it. Information automatically set by MDrawInfo::getPrototype : path path to object to be drawn view view to draw in matrix world matrix for object display appearance how object should be drawn display status active, dormant etc. The draw token is an integer value which you can use to specify what is to be drawn. This is object specific and so you should define an enum with the information you require to decide what is being drawn in your MPxSurfaceShapeUI::draw method. Here is an example of draw token values for a polygonal mesh object as defined in an MPxSurfaceShapeUI derived class. // Draw Tokens

**Public Member Functions**

- `MDrawRequest ()`  
  Constructor.
- `MDrawRequest (const MDrawRequest &in)`  
  Copy constructor.
- `~MDrawRequest ()`  
  Destructor.
- `M3dView view () const`  
  Returns the view where drawing will be done.
- `void setView (M3dView &)`  
  Sets the view where drawing will be done.
- `const MDagPath multiPath () const`  
  Returns the path to the object to be drawn.
- `void setMultiPath (const MDagPath &)`  
  Sets the path to the object to be drawn.
- `MObject component () const`  
  An optional component.
- `void setComponent (MObject &)`  
  Set a component to be drawn.
- `MDrawData drawData () const`  
  Returns the object specific draw data.
- `void setDrawData (MDrawData &)`  
  Set the object specific draw data.
- `M3dView::DisplayStatus displayStatus () const`  
  Return the state of object (active, dormant, etc.).
- `void setDisplayStatus (M3dView::DisplayStatus)`  
  Set the state of object (active, dormant, etc.).
- `bool displayCulling () const`  
  Returns the state of the culling flag for the object.
- `void setDisplayCulling (bool)`  
  Sets the state of the culling flag for the object.
- `bool displayCullOpposite () const`  
  Returns the state of the culling flag for the object.
- `void setDisplayCullOpposite (bool)`  
  Sets the state of the culling flag for the object.
- `M3dView::DisplayStyle displayStyle () const`  
  Returns how the object should be drawn (wireframe, shaded, etc.).
- `void setDisplayStyle (M3dView::DisplayStyle)`  
  Sets how the object should be drawn (wireframe, shaded, etc.).
- `int color (M3dView::ColorTable table) const`  
  Returns the wireframe display color.
- `void setColor (int, M3dView::ColorTable table)`  
  Sets the wireframe display color.
- `MColor color () const`  
  Returns the RGBA wireframe display color.
- `void setColor (const MColor &color)`  
  Sets the RGBA wireframe display color.
- `MMaterial material () const`  
  Returns the shaded material.
- `void setMaterial (MMaterial &)`  
  Returns the shaded material.
- `bool isTransparent () const`  
  Returns the transparency state of the object.
- `void setIsTransparent (bool)`  
  Sets the transparency state of the object.
- `bool drawLast () const`  
  Returns the order in which this object will be drawn.
- `void setDrawLast (bool)`  
  Specifies the order in which this object will be drawn.
- `int token () const`  
  Returns the user-defined draw token for this request.
- `void setToken (int)`  
  Set the user-defined draw token for this request.
- `const MMatrix & matrix () const`  
  Returns the draw matrix.
- `void setMatrix (const MMatrix &)`  
  Set the draw matrix.
- `MDrawRequest & operator= (const MDrawRequest &other)`  
  Assignment operator.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDrawRequestQueue

Module `OpenMayaUI`. Source: [class_m_draw_request_queue.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_request_queue.html).

This class defines a simple interface for a collection of MDrawRequest objects. An MDrawRequestQueue object is passed to the getDrawRequests method of a user defined shape's UI class (MPxSurfaceShapeUI). This queue keeps track of all the things that need to get draw when a refresh of the view occurs. Maya will call the getDrawRequest methods of all the visible DAG objects before a refresh happens. Then as the refresh happens, each draw request will be processed and the corresponding draw method for each DAG object will get called. For user defined shapes MPxSurfaceShapeUI::draw will get called. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MRenderItem and MHWRender::MPxGeometryOverride instead for Viewport 2.0 support.

**Public Member Functions**

- `MDrawRequestQueue ()`  
  Constructor.
- `~MDrawRequestQueue ()`  
  Destructor.
- `bool isEmpty () const`  
  Returns true if the queu is empty.
- `void add (MDrawRequest &)`  
  Adds a draw request to the draw queue.
- `MDrawRequest remove ()`  
  Removes a draw request from the draw queue.
- `MDrawRequestQueue (const MDrawRequestQueue &)`  
  Copy Constructor.
- `MDrawRequestQueue & operator= (const MDrawRequestQueue &)`  
  Assignment operator.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDrawTraversal

Module `OpenMayaUI`. Source: [class_m_draw_traversal.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_traversal.html).

MDrawTraversal is a utility class for interactive drawing. The purpose of the class is to traverse through the current scene dag and provide a means of accessing a list of visible objects with respect to a given frustum specification, and application "visibility" criteria. The additional "visibility" criteria include: Visibility overrides set per object . Visibility overrides set per application (global display settings) . Visibility overrides set per display layer (global display settings). Note that as the traversal is not dependent on a 3d modeling viewport. There are thus no checks for per viewport visibility properties such as per viewport dag object type overrides, and "isolate select". Additionally, render layer visibility checks are not performed, as these pertain to software rendering. After traversal the list will contain leaf level nodes (shapes), which have passed the "visibility" criteria. For the actual frustum culling, there is a choice of two culling algorithms: Perform culling tests only when leaf level dag objects (shapes) are encountered. Perform hierarchical culling (cull sub-trees) as they are encountered. Note that hierarchical culling may force evaluation of geometry on dag objects which are not visible. The default is thus to perform leaf level culling only. Frustum culling is performed against the bounding boxes of dag objects encountered. API writers can derive from this class and change the filterNode() virtual method to perform custom filtering of objects during traversal. That is they will be excluded from the output list. By default this class does no additional filtering. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MSceneRender and MHWRender::MRenderOverride instead.

**Public Types**

- `enum { kActiveItem, kTemplateItem }`  
  Item status enum.

**Public Member Functions**

- `MDrawTraversal ()`  
  Default constructor.
- `virtual ~MDrawTraversal ()`  
  Default destructor.
- `virtual MStatus traverse ()`  
  Perform traversal of the current scene from the root of the dag hierarchy.
- `virtual bool filterNode (const MDagPath &traversalItem)`  
  Method to allow filtering during traversal.
- `MStatus setFrustum (const MDagPath &cameraPath, unsigned int portWidth, unsigned int portHeight)`  
  Set the frustum to cull with.
- `MStatus setOrthoFrustum (double left, double right, double bottom, double top, double nearpt, double farpt, const MMatrix &worldXform)`  
  Set up an orthographic view frustum to cull with.
- `MStatus setPerspFrustum (double fovX, double aspectXY, double nearDist, double farDist, const MMatrix &worldXform)`  
  Set up an perspective view frustum to cull with.
- `MStatus setFrustum (double left, double right, double bottom, double top, double nearpt, double farpt, const MMatrix &worldXform)`  
  Set up a frustum to cull with.
- `MStatus setFrustum (const MPoint &nearBottomLeft, const MPoint &nearBottomRight, const MPoint &nearTopLeft, const MPoint &nearTopRight, const MPoint &farBottomLeft, const MPoint &farBottomRight, const MPoint &farTopLeft, const MPoint &farTopRight, const MMatrix &worldXform)`  
  Set up a frustum to cull with by specifying the 8 corner points.
- `bool frustumValid () const`  
  Returns whether the current frustum set is valid or not.
- `void setLeafLevelCulling (bool cullAtLeafLevel)`  
  Set whether to cull at the leaf levels, or perform hierarchical culling.
- `bool leafLevelCulling () const`  
  Returns whether the current cull algorithm will cull at the leaf levels, or perform hierarchical culling.
- `void enableFiltering (bool val)`  
  Sets whether to use enable usage of the filterNode() method to perform custom filtering.
- `bool filteringEnabled () const`  
  Tells whether custom filtering has been enabled.
- `unsigned int numberOfItems () const`  
  Return the number of items found after traversal.
- `MStatus itemPath (unsigned int itemNumber, MDagPath &path) const`  
  Get the path for a given item in the list of found items after traversal.
- `bool itemHasStatus (unsigned int itemNumber, unsigned int test) const`  
  Test the display status for a given item in the list of found items after traversal.


### MEvent

Module `OpenMayaUI`. Source: [class_m_event.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_event.html).

System event information. The MEvent class is used for querying system events such as mouse presses. Events are handled by an MPxContext derived class in which MEvents are passed and can be accessed. Since Maya has default actions for several events, only a subset are avalaible through the API. The events that can be accessed are: buttonPress - mouse button press event (left & middle only) buttonRelease - mouse buttons release event (left & middle only) mouseDrag - mouse button drag event (left & middle only) hold - mouse button hold event (left & middle only) deleteEvent - delete/backspace key event commandCompletion - complete key event Several modifiers for events are also accessible through the API. Modifiers are actions that occur during an event. For example, holding down the shift key while pressing a mouse button causes a button press event to occur with a shift modifier. A modifier can be used to determine if two mouse events occur simulaneously. The second mouse event is registered as a modifier in the hold event of the first mouse button. So if you wanted to determine if both the left and middle buttons are pressed then you would query the modifier in the hold event of the first mouse button using the isModifierMiddleMouseButton() and isModifierLeftMouseButton() methods.

**Public Types**

- `enum ModifierType { shiftKey = 1<<0, controlKey = 1<<2 }`  
  Modifier key types.
- `enum MouseButtonType { kLeftMouse = 1<<6, kMiddleMouse = kLeftMouse<<1 }`  
  Mouse button types.

**Public Member Functions**

- `MEvent ()`  
  Constructor.
- `virtual ~MEvent ()`  
  Destructor.
- `MStatus getPosition (short &x_pos, short &y_pos) const`  
  Get the location of the event in view co-ordinates.
- `MStatus setPosition (short x_pos, short y_pos)`  
  set the location of the event to the specified location.
- `MStatus getWindowPosition (short &x_pos, short &y_pos) const`  
  This routine is used by responders to query the position of the pointer when the event occurred.
- `MouseButtonType mouseButton (MStatus *ReturnStatus=NULL) const`  
  Get the mouse button of the last event.
- `bool isModifierKeyRelease (MStatus *ReturnStatus=NULL) const`  
  Was a modifier key released.
- `ModifierType modifiers (MStatus *ReturnStatus=NULL) const`  
  This routine is used by responders to find the state of the modifiers during the event.
- `MStatus setModifiers (const ModifierType &modType)`  
  set the event modifiers.
- `bool isModifierNone (MStatus *ReturnStatus=NULL) const`  
  Determines if there are any modifiers for this event.
- `bool isModifierShift (MStatus *ReturnStatus=NULL) const`  
  return state of shift key.
- `bool isModifierControl (MStatus *ReturnStatus=NULL) const`  
  return state of control key.
- `bool isModifierLeftMouseButton (MStatus *ReturnStatus=NULL) const`  
  Return the state of the left mouse button.
- `bool isModifierMiddleMouseButton (MStatus *ReturnStatus=NULL) const`  
  Return the state of the middle mouse button.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MExternalDropCallback

Module `OpenMayaUI`. Source: [class_m_external_drop_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_drop_callback.html).

Provides a callback hook into Maya's drag-and-drop mechanism. This class is used to register callbacks to gain access to Maya's drag-and-drop information during dropping an external object to Maya. You can replace or augment Maya's drop behavior for external drag-and-drop operations. To register callbacks, inherit from this class and override externalDropCallback(). The method can be overridden by the callback. Then register the callbacks by calling the addCallback() method. Each of the callback methods gets passed the name of the drop site control and an MExternalDropData instance which contains additional information provided by the system (for example, text or URL information associated with the drop). If multiple callbacks need to be registered, the order of invocation can be set by adding each callback with a priority number, 0 being the highest priority. Callbacks are then invoked in priority order. Note that the first callback that does not return kMayaDefault will cause the invocations to stop; lower priority callbacks will then not be invoked.

**Public Types**

- `enum MExternalDropStatus { kMayaDefault = 0, kNoMayaDefaultAndAccept, kNoMayaDefaultAndNoAccept }`  
  Possible return values from externalDropCallback(), used to inform Maya of what further action to take, if any.

**Public Member Functions**

- `MExternalDropCallback ()`  
  Constructor.
- `virtual ~MExternalDropCallback ()`  
  Destructor.
- `virtual MExternalDropStatus externalDropCallback (bool doDrop, const MString &controlName, const MExternalDropData &data)=0`  
  This pure virtual method must be implemented by derived callback classes.

**Static Public Member Functions**

- `static MStatus addCallback (MExternalDropCallback *, int priority=0)`  
  Add a callback to the list.
- `static MStatus removeCallback (MExternalDropCallback *)`  
  Remove a callback from the list.


### MExternalDropData

Module `OpenMayaUI`. Source: [class_m_external_drop_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_external_drop_data.html).

Drag-and-drop data, used with MExternalDropCallback. MExternalDropData is the data that a drag-and-drop operation carries if dragging from an external application and dropping onto Maya. It typically arrives from a MExternalDropCallback callback method.

**Public Member Functions**

- `bool hasText () const`  
  Query whether the drop contains text data.
- `MString text () const`  
  Obtain the text data contained in the drop, if any.
- `bool hasUrls () const`  
  Query whether the drop contains URL data.
- `MStringArray urls () const`  
  Obtain the URL data contained in the drop, if any.
- `bool hasHtml () const`  
  Query whether the drop contains html data.
- `MString html () const`  
  Obtain the html data contained in the drop, if any.
- `bool hasColor () const`  
  Query whether the drop contains color data.
- `MColor color () const`  
  Obtain the color data contained in the drop, if any.
- `bool hasImage () const`  
  Query whether the drop contains image data.
- `MImage image () const`  
  Obtain the image data contained in the drop, if any.
- `MStringArray formats () const`  
  Obtain the list of data formats contained in the drop.
- `bool hasFormat (const MString &format) const`  
  Query whether the drop contains data in a given format.
- `unsigned int dataSize (const MString &format) const`  
  Return the size (in bytes) of the data with the given format contained in the drop.
- `MStatus copyData (const MString &format, void *buffer, unsigned int bufferSize) const`  
  This method is not available in Python.
- `int keyboardModifiers () const`  
  Return the modifier state for this drop.

**Protected Attributes**

- `OPENMAYA_PRIVATE int mouseButtons`  
  Return the mouse button state for this drop.


### MFeedbackLine

Module `OpenMayaUI`. Source: [class_m_feedback_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_feedback_line.html).

Feedback line. The MFeedbackLine class is used to display information back to the user. The format for the feedback line which indicates the number and type of the arguments should be set with the setFormat method. The values of the arguments should be set using the setValue method.

**Static Public Member Functions**

- `static MStatus setFormat (const MString &format)`  
  Set the format string for the feedback line.
- `static MStatus setTitle (const MString &title)`  
  Set the title string.
- `static MStatus setValue (short index, double value)`  
  Set the value of a given index in the feedback line.
- `static void clear ()`  
  Clear whatever is showing in the feedback line.
- `static bool showFeedback ()`  
  Return whether or not the feedback line is is supposed to be displaying data.
- `static void setShowFeedback (bool showFeedback)`  
  Set whether the feedback line is supposed to be displaying data.
- `static const char * className ()`  
  Returns the name of this class.


### MFnCircleSweepManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_circle_sweep_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_circle_sweep_manip.html).

CircleSweepManip function set. The CircleSweepManip allows the user to manipulate a point constrained to move around a circle, in order to specify a sweep angle. This manipulator generates a single floating point value corresponding to the sweep angle.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnCircleSweepManip ()`  
  Destructor.
- `MFnCircleSweepManip ()`  
  Default constructor.
- `MFnCircleSweepManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnCircleSweepManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new CircleSweepManip.
- `MObject create (const MString &manipName, const MString &angleName, MStatus *ReturnStatus=NULL)`  
  Creates a new CircleSweepManip.
- `MStatus connectToAnglePlug (MPlug &anglePlug)`  
  Connect to the angle plug.
- `MStatus setCenterPoint (const MPoint &centerPoint)`  
  Sets the center point of the CircleSweepManip.
- `MStatus setStartPoint (const MPoint &startPoint)`  
  Sets the start point of the CircleSweepManip.
- `MStatus setEndPoint (const MPoint &endPoint)`  
  Sets the end point of the CircleSweepManip.
- `MStatus setNormal (const MVector &normal)`  
  Sets the normal of the CircleSweepManip.
- `MStatus setRadius (double radius)`  
  Sets the radius of the CircleSweepManip.
- `MStatus setAngle (MAngle angle)`  
  Sets the angle of the CircleSweepManip.
- `MStatus setDrawAsArc (bool state)`  
  Sets whether or not to draw as arc.
- `MPoint startPoint (MStatus *ReturnStatus=NULL) const`  
  Returns the start point.
- `MPoint endPoint (MStatus *ReturnStatus=NULL) const`  
  Returns the end point.
- `unsigned int centerIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index for the center of the CircleSweepManip.
- `unsigned int axisIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index for the axis of CircleSweepManip.
- `unsigned int angleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index for the angle of CircleSweepManip.
- `unsigned int startCircleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index for the start of the circle of CircleSweepManip.
- `unsigned int endCircleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index for the end of the circle of CircleSweepManip.
- `MFnCircleSweepManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnCurveSegmentManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_curve_segment_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_curve_segment_manip.html).

CurveSegmentManip function set. The CurveSegmentManip allows the user to manipulate two points on a curve, in order to specify a curve segment. This manipulator generates two floating point values, which correspond to the parameters of the start and end of the curve segment.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnCurveSegmentManip ()`  
  Destructor.
- `MFnCurveSegmentManip ()`  
  Default constructor.
- `MFnCurveSegmentManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnCurveSegmentManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new CurveSegmentManip.
- `MObject create (const MString &manipName, const MString &startParamName, const MString &endParamName, MStatus *ReturnStatus=NULL)`  
  Creates a new CurveSegmentManip.
- `MStatus connectToCurvePlug (MPlug &curvePlug)`  
  Connect to the curve plug.
- `MStatus connectToStartParamPlug (MPlug &startParamPlug)`  
  Connect to the startParam plug.
- `MStatus connectToEndParamPlug (MPlug &endParamPlug)`  
  Connect to the endParam plug.
- `MStatus setStartParameter (double startParameter)`  
  Sets the start parameter.
- `MStatus setEndParameter (double endParameter)`  
  Sets the end parameter.
- `double startParameter (MStatus *ReturnStatus=NULL) const`  
  Returns the start parameter.
- `double endParameter (MStatus *ReturnStatus=NULL) const`  
  Returns the end parameter.
- `unsigned int curveIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the curve.
- `unsigned int startParamIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the start parameter of the CurveSegmentManip.
- `unsigned int endParamIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the end parameter of the CurveSegmentManip.
- `MFnCurveSegmentManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnDirectionManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_direction_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_direction_manip.html).

DirectionManip function set. The DirectionManip allows the user to specify a direction, as defined by the vector from the start point to the manipulator position. It uses a FreePointTriadManip to specify the end point of a vector relative to a given start point. This manipulator generates a vector from the start point to the end point.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnDirectionManip ()`  
  Destructor.
- `MFnDirectionManip ()`  
  Default constructor.
- `MFnDirectionManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnDirectionManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new DirectionManip.
- `MObject create (const MString &manipName, const MString &directionName, MStatus *ReturnStatus=NULL)`  
  Creates a new DirectionManip.
- `MStatus connectToDirectionPlug (MPlug &directionPlug)`  
  Connect to the direction plug.
- `MStatus setNormalizeDirection (bool state)`  
  Sets whether or not to the direction should be normalized.
- `MStatus setDrawStart (bool state)`  
  Sets whether or not to draw the start of the DirectionManip.
- `MStatus setStartPoint (MPoint &startPoint)`  
  Sets the start point of the DirectionManip.
- `MStatus setDirection (MVector &direction)`  
  Sets the direction of the DirectionManip.
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the start point of the DirectionManip.
- `unsigned int endPointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the end point of the DirectionManip.
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the direction.
- `MFnDirectionManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnDiscManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_disc_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_disc_manip.html).

DiscManip function set. The DiscManip allows the user to rotate a disc in order to specify a rotation about an axis. This manipulator generates a single floating point value corresponding to the rotation.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnDiscManip ()`  
  Destructor.
- `MFnDiscManip ()`  
  Default constructor.
- `MFnDiscManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnDiscManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new DiscManip.
- `MObject create (const MString &manipName, const MString &angleName, MStatus *ReturnStatus=NULL)`  
  Creates a new DiscManip.
- `MStatus connectToAnglePlug (MPlug &anglePlug)`  
  Connect to the angle plug.
- `MStatus setCenterPoint (const MPoint &centerPoint)`  
  Sets the center point of the DiscManip.
- `MStatus setNormal (const MVector &normal)`  
  Sets the normal of the DiscManip.
- `MStatus setRadius (double radius)`  
  Sets the radius of the DiscManip.
- `MStatus setAngle (MAngle angle)`  
  Sets the angle of the DiscManip.
- `unsigned int centerIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the center of the DiscManip.
- `unsigned int axisIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the axis of the DiscManip.
- `unsigned int angleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the angle.
- `MFnDiscManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnDistanceManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_distance_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_distance_manip.html).

DistanceManip function set. The DistanceManip allows the user to manipulate a point that is constrained to move along a line. This manipulator generates a single floating point value. Scaling factors can be used to determine how int the manipulator appears when it is drawn.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnDistanceManip ()`  
  Destructor.
- `MFnDistanceManip ()`  
  Default constructor.
- `MFnDistanceManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnDistanceManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new DistanceManip.
- `MObject create (const MString &manipName, const MString &distanceName, MStatus *ReturnStatus=NULL)`  
  Creates a new DistanceManip.
- `MStatus connectToDistancePlug (MPlug &distancePlug)`  
  Connect to the distance plug.
- `MStatus setStartPoint (const MPoint &point)`  
  Sets the start point of the DistanceManip.
- `MStatus setDirection (const MVector &vector)`  
  Sets the direction of the DistanceManip.
- `MStatus setDrawStart (bool state)`  
  Sets whether or not to draw the start of the DistanceManip.
- `MStatus setDrawLine (bool state)`  
  Sets whether or not to draw a line from the start to the end of the DistanceManip.
- `MStatus setScalingFactor (double scalingFactor)`  
  Sets the scaling factor.
- `bool isDrawStartOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the start of the DistanceManip is being drawn.
- `bool isDrawLineOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not a line is being drawn from the start to the end of the DistanceManip.
- `double scalingFactor (MStatus *ReturnStatus=NULL) const`  
  Returns the scaling factor.
- `unsigned int distanceIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the distance.
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the direction.
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the start point of the DistanceManip.
- `unsigned int currentPointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the current point of the DistanceManip.
- `MFnDistanceManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnFreePointTriadManip

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_free_point_triad_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_free_point_triad_manip.html).

FreePointTriadManip function set. The FreePointTriadManip provides a moveable point, which can be moved anywhere, and has axes for constrained x, y, and z movement and obeys grid snapping, point snapping, and curve snapping. The FreePointTriadManip generates the 3D position of the moveable point. It is useful for specifying the position of an object in space. Note that only the MFnNumericData::k3Double data type is supported when connecting to a pointPlug via connectToPointPlug.

**Public Types**

- `enum ManipPlane { kYZPlane, kXZPlane, kXYPlane, kViewPlane }`  
  Interaction plane for the manip center.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnFreePointTriadManip ()`  
  Destructor.
- `MFnFreePointTriadManip ()`  
  Default constructor.
- `MFnFreePointTriadManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnFreePointTriadManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new FreePointTriadManip.
- `MObject create (const MString &manipName, const MString &pointName, MStatus *ReturnStatus=NULL)`  
  Creates a new FreePointTriadManip.
- `MStatus connectToPointPlug (MPlug &pointPlug)`  
  Connect to the point plug.
- `MStatus setDrawAxes (bool state)`  
  Sets whether or not to draw the axes of the FreePointTriadManip.
- `MStatus setSnapMode (bool state)`  
  Sets whether or not to the FreePointTriadManip should be in snap mode.
- `MStatus setKeyframeAll (bool state)`  
  Sets whether or not keyframeAll is on.
- `MStatus setDrawArrowHead (bool state)`  
  Sets whether or not drawArrowHead is on.
- `MStatus setGlobalTriadPlane (ManipPlane whichPlane)`  
  Sets which plane to use as the global triad plane.
- `MStatus setPoint (const MPoint &pointValue)`  
  Set the point manipulator value to the given vector.
- `MStatus setDirection (const MVector &direction)`  
  Sets the orientation of the FreePointTriadManip.
- `bool isDrawAxesOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the axes of the FreePointTriadManip are being drawn.
- `bool isSnapModeOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the FreePointTriadManip is in snap mode.
- `bool isKeyframeAllOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the FreePointTriadManip is in keyframeAll mode.
- `unsigned int pointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the point of the FreePointTriadManip.
- `MFnFreePointTriadManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnManip3D

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_manip3_d.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_manip3_d.html).

3D manipulator function set MFnManip3D allows the creation and manipulation of 3D manipulators. MFnManip3D is the base class from which MFnFreePointTriadManip, MFnDirectionManip, MFnDistanceManip, MFnPointOnCurveManip, MFnPointOnSurfaceManip, MFnDiscManip, MFnCircleSweepManip, MFnToggleManip, MFnStateManip, and MFnCurveSegmentManip are derived.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnManip3D ()`  
  Destructor.
- `MFnManip3D ()`  
  Default constructor.
- `MFnManip3D (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnManip3D (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `bool isVisible (MStatus *ReturnStatus) const`  
  Returns whether or not the manipulator is visible.
- `MStatus setVisible (bool isVisible)`  
  Sets whether or not the manipulator is visible.
- `float manipScale (MStatus *ReturnStatus) const`  
  Returns the manipulator scale.
- `MStatus setManipScale (float size)`  
  Sets the manipulator scale.
- `bool isOptimizePlaybackOn (MStatus *ReturnStatus) const`  
  Returns whether or not optimize playback is on.
- `MStatus setOptimizePlayback (bool optimizePlayback)`  
  Sets whether or not to optimize the playback.
- `MEulerRotation rotateXYZValue (const unsigned int valIndex, MStatus *ReturnStatus=NULL)`  
  Gets the rotation for the active manipulator.
- `MFnManip3D (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static float globalSize ()`  
  Returns the global manipulator size.
- `static void setGlobalSize (float size)`  
  Sets the global manipulator size.
- `static float handleSize ()`  
  Returns the manipulator handle size.
- `static void setHandleSize (float size)`  
  Sets the manipulator handle size.
- `static float lineSize ()`  
  Returns the manipulator line size.
- `static void setLineSize (float size)`  
  Sets the manipulator line size.
- `static MStatus deleteManipulator (MObject &manip)`  
  Delete a manipulator.
- `static bool drawPlaneHandles ()`  
  This method returns the global option that says if the planar manipulator handles should be drawn or not.
- `static void setDrawPlaneHandles (bool drawPlaneHandles)`  
  Sets the global option to display planar handles or not on supported manipulators.


### MFnPointOnCurveManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_point_on_curve_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_on_curve_manip.html).

PointOnCurveManip function set. The PointOnCurveManip allows the user to manipulate a point constrained to move along a curve, in order to specify the "u" curve parameter value. This manipulator generates a single floating point value corresponding to the curve parameter.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnPointOnCurveManip ()`  
  Destructor.
- `MFnPointOnCurveManip ()`  
  Default constructor.
- `MFnPointOnCurveManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnPointOnCurveManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new PointOnCurveManip.
- `MObject create (const MString &manipName, const MString &paramName, MStatus *ReturnStatus=NULL)`  
  Creates a new PointOnCurveManip.
- `MStatus connectToCurvePlug (MPlug &curvePlug)`  
  Connect to the curve plug.
- `MStatus connectToParamPlug (MPlug &paramPlug)`  
  Connect to the param plug.
- `MStatus setDrawCurve (bool state)`  
  Sets whether or not the curve is drawn.
- `MStatus setParameter (double parameter)`  
  Sets the parameter.
- `bool isDrawCurveOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the curve is drawn.
- `double parameter (MStatus *ReturnStatus=NULL) const`  
  Returns the parameter.
- `MPoint curvePoint (MStatus *ReturnStatus=NULL) const`  
  Returns the curve point.
- `unsigned int curveIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the curve.
- `unsigned int paramIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the parameter of the PointOnCurveManip.
- `MFnPointOnCurveManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnPointOnSurfaceManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_point_on_surface_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_point_on_surface_manip.html).

PointOnSurfaceManip function set. The PointOnSurfaceManip allows the user to manipulate a point constrained to move along a surface, in order to specify the (u, v) surface parameter values. This manipulator generates two floating point values corresponding to the surface (u, v) parameters.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnPointOnSurfaceManip ()`  
  Destructor.
- `MFnPointOnSurfaceManip ()`  
  Default constructor.
- `MFnPointOnSurfaceManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnPointOnSurfaceManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new PointOnSurfaceManip.
- `MObject create (const MString &manipName, const MString &paramName, MStatus *ReturnStatus=NULL)`  
  Creates a new PointOnSurfaceManip.
- `MStatus connectToSurfacePlug (MPlug &surfacePlug)`  
  Connect to the surface plug.
- `MStatus connectToParamPlug (MPlug &paramPlug)`  
  Connect to the param plug.
- `MStatus setDrawSurface (bool state)`  
  Sets whether or not the surface is drawn.
- `MStatus setDrawArrows (bool state)`  
  Sets whether or not the arrows should be drawn.
- `MStatus setParameters (double u, double v)`  
  Sets the u and v parameters.
- `MStatus getParameters (double &u, double &v)`  
  Returns the parameter.
- `bool isDrawSurfaceOn (MStatus *ReturnStatus=NULL) const`  
  Returns whether or not the surface is drawn.
- `unsigned int surfaceIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the surface.
- `unsigned int paramIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the parameter of the PointOnSurfaceManip.
- `MFnPointOnSurfaceManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnRotateManip

Module `OpenMayaUI`. Inherits `MFnTransform`. Source: [class_m_fn_rotate_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_rotate_manip.html).

RotateManip function set. This class provides access to the built-in Maya rotate manipulator. The manipulator consists of three constrained-axis rotation rings, a view rotation ring, as well as an invisible trackball that allows the user to rotate in arbitrary directions on the sphere. The manipulator provides data to the plugin through the rotation manipVal. The rotation value is a vector consisting of x, y, and z rotations. Rotations are measured from the initial rotation (usually <0,0,0>) of the manipulator. The manipulator can be configured either to display with an object (which must be a DAG node) or to display at an arbitrary point using the rotationCenter manipVal.

**Public Types**

- `enum RotateMode { kObjectSpace, kWorldSpace, kGimbal }`  
  Manipulator rotation modes.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnRotateManip ()`  
  Destructor.
- `MFnRotateManip ()`  
  Default constructor.
- `MFnRotateManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnRotateManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new RotateManip, and attaches this function set to the new manipulator.
- `MObject create (const MString &manipName, const MString &rotationName, MStatus *ReturnStatus=NULL)`  
  Creates a new RotateManip, and attaches this function set to the new manipulator.
- `MStatus connectToRotationPlug (const MPlug &rotationPlug)`  
  Create a 1-1 connection from the rotation manipVal to the rotationPlug parameter.
- `MStatus connectToRotationCenterPlug (const MPlug &rotationCenterPlug)`  
  Create a 1-1 association of the rotation center on the manipulator and the rotationCenterPlug parameter.
- `MStatus setInitialRotation (const MEulerRotation &rotation)`  
  Sets the initial rotation for the rotate manipulator.
- `MStatus displayWithNode (const MObject &node)`  
  Configures the manipulator to display with the node, causing the position of the manipulator to follow the position of the node whenever the node is moved.
- `MStatus setRotateMode (RotateMode mode)`  
  Sets the mode for the rotation manipulator.
- `RotateMode rotateMode () const`  
  Returns the current rotation mode.
- `MStatus setSnapMode (bool snapEnabled)`  
  Sets the snap mode.
- `bool isSnapModeOn () const`  
  Returns true when snap mode is on.
- `MStatus setSnapIncrement (double snapInc)`  
  Sets the snap increment.
- `double snapIncrement () const`  
  Returns the snapping increment in degrees.
- `unsigned int rotationIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the rotation manipVal for the manipulator.
- `unsigned int rotationCenterIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the rotation center for the manipulator.
- `MStatus setRotationCenter (const MPoint &rotationCenter)`  
  Sets the position of the rotation center for the manipulator.
- `MFnRotateManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnScaleManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_scale_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_scale_manip.html).

ScaleManip function set. This class provides access to the built-in Maya scale manipulator. The manipulator consists of three constrained-axis scale handles for non-proportional scaling, and a central handle for proportional scaling. The manipulator provides data to the plugin through the scale manipVal. The scale value is a vector consisting of X, Y, and Z scale values. Each scale value represents a factor controlling how much an object should be extended along that dimension. The scale values are absolute and the initial scale value has a default of <1.0,1.0,1.0>. The manipulator can be configured either to display with an object (which must be a DAG node) or to display at an arbitrary point using the scaleCenter manipVal.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnScaleManip ()`  
  Destructor.
- `MFnScaleManip ()`  
  Default constructor.
- `MFnScaleManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnScaleManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new ScaleManip, and attaches this function set to the new manipulator.
- `MObject create (const MString &manipName, const MString &scaleName, MStatus *ReturnStatus=NULL)`  
  Creates a new ScaleManip, and attaches this function set to the new manipulator.
- `MStatus connectToScalePlug (const MPlug &scalePlug)`  
  Create a 1-1 connection from the scale manipVal to the scalePlug parameter.
- `MStatus connectToScaleCenterPlug (const MPlug &scaleCenterPlug)`  
  Create a 1-1 association of the scale center on the manipulator and the scaleCenterPlug parameter.
- `MStatus setInitialScale (const MVector &scale)`  
  Sets the initial scale for the scale manipulator.
- `MStatus displayWithNode (const MObject &node)`  
  Configures the manipulator to display with the node, causing the position of the manipulator to follow the position of the node whenever the node is moved.
- `MStatus setSnapMode (bool snapEnabled)`  
  Sets the snap mode.
- `bool isSnapModeOn () const`  
  Returns true when snap mode is on.
- `MStatus setSnapIncrement (double snapInc)`  
  Sets the snap increment.
- `double snapIncrement () const`  
  Returns the snapping increment in working units.
- `MStatus setOrientation (const MEulerRotation &direction)`  
  Sets the arbitrary orientation of the MFnScaleManip.
- `MEulerRotation getOrientation (MStatus *ReturnStatus=NULL) const`  
  Returns the orientation used by the manip when its orientationMode is set to kArbitraryOrientation.
- `MStatus setOrientationMode (ScaleOrientationMode mode)`  
  Sets the orientation mode of the MFnScaleManip.
- `ScaleOrientationMode getOrientationMode (MStatus *ReturnStatus=NULL) const`  
  Gets the orientation mode of the MFnScaleManip.
- `unsigned int scaleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the scale manipVal for this manipulator.
- `unsigned int scaleCenterIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the scale center manipVal for this manipulator.
- `MFnScaleManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnStateManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_state_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_state_manip.html).

StateManip function set. The StateManip allows the user to switch between multiple states. It is drawn as a circle with a notch. Each click on the circle increments the value of the state (modulo the maximum number of states). This manipulator generates an integer value corresponding to the state of the manip.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnStateManip ()`  
  Destructor.
- `MFnStateManip ()`  
  Default constructor.
- `MFnStateManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnStateManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new StateManip.
- `MObject create (const MString &manipName, const MString &stateName, MStatus *ReturnStatus=NULL)`  
  Creates a new StateManip.
- `MStatus connectToStatePlug (MPlug &statePlug)`  
  Connect to the state plug.
- `MStatus setInitialState (unsigned int initialState)`  
  Sets the initial state of the StateManip.
- `MStatus setMaxStates (unsigned int numStates)`  
  Sets the maximum number of states that the StateManip will have.
- `unsigned int maxStates (MStatus *ReturnStatus=NULL) const`  
  Returns the number of maximum states.
- `unsigned int state (MStatus *ReturnStatus=NULL) const`  
  Returns the current state.
- `unsigned int positionIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the position of the StateManip.
- `unsigned int stateIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the state.
- `MFnStateManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnToggleManip

Module `OpenMayaUI`. Inherits `MFnManip3D`. Source: [class_m_fn_toggle_manip.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_toggle_manip.html).

ToggleManip function set. The ToggleManip allows the user to switch between two modes or some on/off state. It is drawn as a circle with or without a dot. When the mode is on, the dot is drawn in the circle; when the mode is off, the circle is drawn without the dot. This manipulator generates a boolean value corresponding to whether or not the mode is on or off.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnToggleManip ()`  
  Destructor.
- `MFnToggleManip ()`  
  Default constructor.
- `MFnToggleManip (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnToggleManip (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  Creates a new ToggleManip.
- `MObject create (const MString &manipName, const MString &toggleName, MStatus *ReturnStatus=NULL)`  
  Creates a new ToggleManip.
- `MStatus connectToTogglePlug (MPlug &togglePlug)`  
  Connect to the toggle plug.
- `MPoint startPoint (MStatus *ReturnStatus=NULL) const`  
  Returns the start point.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction.
- `double length (MStatus *ReturnStatus=NULL) const`  
  Returns the length.
- `bool toggle (MStatus *ReturnStatus=NULL) const`  
  Returns the toggle.
- `unsigned int startPointIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the start point of the ToggleManip.
- `unsigned int directionIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the direction.
- `unsigned int lengthIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the length of the ToggleManip.
- `unsigned int toggleIndex (MStatus *ReturnStatus=NULL) const`  
  Returns the index of the toggle of the ToggleManip.
- `MStatus setStartPoint (MPoint &startPoint)`  
  Sets the start point of the ToggleManip.
- `MStatus setDirection (MVector &direction)`  
  Sets the direction of the ToggleManip.
- `MStatus setLength (double length)`  
  Sets the length of the ToggleManip.
- `MStatus setToggle (bool toggle)`  
  Sets the toggle of the ToggleManip.
- `MFnToggleManip (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MGraphEditorInfo

Module `OpenMayaUI`. Inherits `MPanelCanvasInfo`. Source: [class_m_graph_editor_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_graph_editor_info.html).

Graph Editor state information with manipulation capabilities. This class provides methods to obtain UI related information from a specific Graph Editor. Support is included for obtaining/setting viewport bounds and obtaining animation curve nodes. Note: the following are built-in GraphEditor-specific event callback names: graphEditorChanged Triggered whenever there is a Graph Editor refresh/resize event graphEditorParamCurveSelected Triggered whenever an animation curve is selected or de-selected graphEditorOutlinerHighlightChanged Triggered whenever there is a change to the Outliner list selection graphEditorOutlinerListChanged Triggered whenever there is a change to the Outliner list contents Python 1.0 Example: (getting the names of the curves selected in the default Graph Editor) import maya.OpenMaya as om

**Public Types**

- `enum AnimCurveQuery { kAnimCurveOutlinerOnly = 0, kAnimCurveHighlighted, kAnimCurveSelected, kAnimCurveAllKnown }`  
  Defines the query criteria for animation curves specifically with respect to the attached Graph Editor's own viewport.

**Public Member Functions**

- `MGraphEditorInfo (MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MGraphEditorInfo (const MString &graphEditorName, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `~MGraphEditorInfo () override`  
  Destructor.
- `MStatus getAnimCurveNodes (MObjectArray &animCurveNodeArray, AnimCurveQuery animCurveQuery) const`  
  Returns an array of animCurve nodes, based on the attached Graph Editor's state information, which captures:
- `bool isStackedViewportMode () const`  
  Returns whether or not the Graph Editor is in Stacked view mode.
- `bool isNormalizedViewportMode () const`  
  Returns whether or not the Graph Editor is in Normalized view mode.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MHWShaderSwatchGenerator

Module `OpenMayaUI`. Inherits `MSwatchRenderBase`. Source: [class_m_h_w_shader_swatch_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_shader_swatch_generator.html).

Hardware shader swatch generator utility class. Derived from the MSwatchRenderBase class as a utility for generating a swatch for a plugin hardware shader class. This class supports hardware shaders derived from both MPxHardwareShader and the older MPxHwShaderNode base classes.

**Public Member Functions**

- `MHWShaderSwatchGenerator (MObject obj, MObject renderObj, int res)`  
  Class constructor.
- `bool doIteration () override`  
  This method will be called from the MSwatchRenderRegister for generating a swatch image.

**Static Public Member Functions**

- `static MSwatchRenderBase * createObj (MObject obj, MObject renderObj, int res)`  
  Class constructor.
- `static const MString & initialize ()`  
  This method sets a swatch name, and registers a new swatch generator creation function for the swatch name.
- `static void getSwatchBackgroundColor (float &r, float &g, float &b, float &a)`  
  This method returns the default background color for the hardware rendered swatch.


### MManipData

Module `OpenMayaUI`. Source: [class_m_manip_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_manip_data.html).

Manipulator Data. This class encapulates manipulator data which is returned from the manipulator conversion functions. MManipData is used to represent data that is either simple or complex. Simple data is used to represent bool, short, int, unsigned int, float, and double types. Complex data is used to represent MObjects created by MFnData, or classes derived from MFnData.

**Public Member Functions**

- `MManipData ()`  
  The default class constructor.
- `~MManipData ()`  
  The destructor.
- `MManipData (const MManipData &)`  
  The copy constructor.
- `MManipData (bool)`  
  Constructor for bool types.
- `MManipData (short)`  
  Constructor for short types.
- `MManipData (int)`  
  Constructor for int types.
- `MManipData (unsigned int)`  
  Constructor for unsigned int types.
- `MManipData (float)`  
  Constructor for float types.
- `MManipData (double)`  
  Constructor for double types.
- `MManipData (const MObject &)`  
  Constructor for MObject types created by MFnData, or classes derived from MFnData.
- `MManipData & operator= (const MManipData &)`  
  The assignment operator.
- `MManipData & operator= (bool)`  
  The assignment operator for bool types.
- `MManipData & operator= (short)`  
  The assignment operator for short types.
- `MManipData & operator= (int)`  
  The assignment operator for int types.
- `MManipData & operator= (unsigned int)`  
  The assignment operator for unsigned int types.
- `MManipData & operator= (float)`  
  The assignment operator for float types.
- `MManipData & operator= (double)`  
  The assignment operator for double types.
- `MManipData & operator= (const MObject &)`  
  The assignment operator for MObject types created by MFnData, or classes derived from MFnData.
- `bool isSimple () const`  
  Returns whether or not the manipulator data is simple or complex.
- `bool asBool () const`  
  Returns the manipulator data as a bool.
- `short asShort () const`  
  Returns the manipulator data as a short.
- `int asLong () const`  
  Returns the manipulator data as a int.
- `unsigned int asUnsigned () const`  
  Returns the manipulator data as an unsigned int.
- `float asFloat () const`  
  Returns the manipulator data as a float.
- `double asDouble () const`  
  Returns the manipulator data as a double.
- `MObject asMObject () const`  
  Returns the manipulator data as an MObject.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MMaterial

Module `OpenMayaUI`. Source: [class_m_material.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_material.html).

This class is used in the draw functions of user defined shapes (see MPxSurfaceShapeUI) for setting up and querying materials used in shaded mode drawing. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MShaderInstance or MHWRender::MFragmentManager instead.

**Public Types**

- `enum MtextureChannel { kColor, kTransparency, kAmbientColor, kIncandescence, kBumpMap, kDiffuse, kTransluscence, kRoughness, kHighlightSize, kWhiteness, kCosinePower, kEccentricity, kSpecularRollOff, kSpecularColor, kReflectivity, kReflectedColor }`  
  Channels which can be queried.

**Public Member Functions**

- `MMaterial ()`  
  Constructor.
- `MMaterial (const MMaterial &in)`  
  Copy constructor.
- `MMaterial (const MObject &, MStatus *ReturnStatus=NULL)`  
  Constructor from shader set MObject.
- `~MMaterial ()`  
  Destructor.
- `MStatus evaluateMaterial (M3dView &, const MDagPath &)`  
  Evaluate a material.
- `MStatus evaluateShininess ()`  
  Perform necessary evaluation to be able to get shininess back.
- `MStatus evaluateDiffuse ()`  
  Perform necessary evaluation to be able to get diffuse back.
- `MStatus evaluateEmission ()`  
  Perform necessary evaluation to be able to get emission back.
- `MStatus evaluateSpecular ()`  
  Perform necessary evaluation to be able to get specular back.
- `MStatus evaluateTexture (MDrawData &data)`  
  Evaluate texturing related information.
- `bool materialIsTextured () const`  
  Do we have a texture (evaluated or not).
- `MStatus setMaterial (const MDagPath &, bool hasTransparency)`  
  Set the current GL material.
- `MStatus getShininess (float &)`  
  Get the GL shininess.
- `MStatus getDiffuse (MColor &)`  
  Get the GL diffuse color.
- `MStatus getEmission (MColor &)`  
  Get the GL emission color.
- `MStatus getSpecular (MColor &)`  
  Get the GL specular color.
- `MStatus getHasTransparency (bool &)`  
  Determine if material or texture has transparency.
- `MStatus getTextureTransformation (const MDrawData &data, MMatrix &texXform)`  
  Get the current textures transformation.
- `MStatus getTextureTransformation (const MDrawData &data, float &rotateUV, float &scaleU, float &scaleV, float &translateU, float &translateV, float &rotateFrame)`  
  Get the current textures transformation.
- `void applyTexture (M3dView &, const MDrawData &)`  
  For materials that have texture, this method must be used before the OpenGL drawing to apply the texture to the current view.
- `MStatus textureImage (MImage &image, MColor &color, MtextureChannel chan, bool &mapped, MDagPath &dagPath, int xRes=-1, int yRes=-1)`  
  For materials that have texture, this method will attempt to retrieve the pixel map for a given mapped channel of that material.
- `MPxHwShaderNode * getHwShaderNode (MStatus *ReturnStatus=NULL)`  
  Get the hardware shader node.
- `MObject shadingEngine ()`  
  Get the shading engined associated with this material.
- `MStatus evaluateTextureTransformation ()`  
  This method is obsolete.
- `MStatus getTextureTransformation (float &scaleU, float &scaleV, float &translateU, float &translateV, float &rotate)`  
  This method is obsolete.

**Static Public Member Functions**

- `static MMaterial defaultMaterial ()`  
  Get the default material.
- `static const char * className ()`  
  Returns the name of this class.


### MMaterialArray

Module `OpenMayaUI`. Source: [class_m_material_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_material_array.html).

Array of pointers of MMaterial data type. This class implements an array of MMaterial. Common convenience functions are available, and the implementation is compatible with the internal Maya implementation so that it can be passed efficiently between plugins and internal maya data structures.

**Public Member Functions**

- `MMaterialArray ()`  
  Default constructor.
- `MMaterialArray (const MMaterialArray &other)`  
  Copy Constructor.
- `~MMaterialArray ()`  
  The class destructor.
- `const MMaterial operator[] (unsigned index) const`  
  Returns the value of the element at the given index.
- `MMaterialArray & operator= (const MMaterialArray &other)`  
  Assignment operator copies all of the elements of the other array into this one.
- `MStatus set (const MMaterial &element, unsigned index)`  
  Sets the value of the indicated element to the indicated MMaterial value.
- `MStatus setLength (unsigned length)`  
  Set the length of the array.
- `unsigned length () const`  
  Returns the number of elements in the instance.
- `MStatus remove (unsigned index)`  
  Remove the array element at the given index.
- `MStatus insert (const MMaterial &element, unsigned index)`  
  Inserts a new value into the array at the given index.
- `MStatus append (const MMaterial &element)`  
  Adds a new element to the end of the array.
- `MStatus copy (const MMaterialArray &source)`  
  Copy the contents of the source array to this array.
- `MStatus clear ()`  
  Clear the contents of the array.
- `void setSizeIncrement (unsigned newIncrement)`  
  Set the size by which the array will be expanded whenever expansion is necessary.
- `unsigned sizeIncrement () const`  
  Return the size by which the array will be expanded whenever expansion is necessary.
- `MMaterial operator[] (unsigned index)`  
  Returns the value of the element at the given index.

**Static Public Member Functions**

- `static const char * className ()`  
  NO SCRIPT SUPPORT.


### MObjectListFilter

Module `OpenMayaUI`. Source: [class_m_object_list_filter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_object_list_filter.html).

Class for defining a scene list filter. MObjectListFilter provides an interface to define a list of selection items which can be used to filter the display of items for interactive 3D scene rendering. The selection list can either have the meaning of rendering only the items in that list (an inclusion list) or only rendering items which are not in the list (an exclusion list). Programmers using this interface can derive from this class and implement the required methods. The derived class is responsible for Registering and deregistering the filter via the registerFilter() and deregisterFilter() methods. Maintaining the selection list to be used for filtering. Indicating when this list has been modified via the requireListUpdate() method. Note that it is valid to always mark the list as being modified. Return the selection list when asked via the getList() method.

**Public Types**

- `enum MFilterType { kInclusionList = 0, kExclusionList, kNumberOfFilterTypes }`  
  Type of filter list.
- `enum MSceneUpdateType { kNone = 0, kAddRemoveObjects = 1<<(0) }`  
  Type of scene update.

**Public Member Functions**

- `MObjectListFilter (const MString &name)`  
  Constructor for a filter.
- `virtual bool requireListUpdate ()=0`  
  This method is called by Maya to determine whether the contents of the object list for the filter has changed since the last requireLastUpdate() call was made.
- `virtual MStatus getList (MSelectionList &list)=0`  
  This method will return the selection list to use for filtering scene rendering.
- `virtual MSceneUpdateType dependentOnSceneUpdates ()`  
  Return whether the update of the filter list is dependent on scene updates.
- `void setFilterType (MFilterType filterType)`  
  Set the filter type.
- `MFilterType filterType () const`  
  Query the filter type.
- `const MString & name () const`  
  Query the name identifier.
- `const MString & UIname () const`  
  Query the UI name.
- `void setUIName (const MString &name)`  
  Set the UI name.

**Static Public Member Functions**

- `static MStatus registerFilter (const MObjectListFilter &filter)`  
  Register the object filter as one of an available set of filters.
- `static MStatus deregisterFilter (const MObjectListFilter &filter)`  
  Deregister the object filter from the list of available filters.
- `static const char * className ()`  
  Returns the name of this class.


### MPanelCanvas

Module `OpenMayaUI`. Source: [class_m_panel_canvas.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_panel_canvas.html).

Custom Panel user drawing capabilities for supported panels. This class provides methods for a user to draw additional primitives to a supporting control. The user can construct this class from the name of a panel which supports such drawing.

**Public Types**

- `enum DrawLayer`  
  Principal layer values.
- `enum GraphEditorReservedLayer`  
  Graph Editor reserved layers.
- `typedef void(* MDrawUIFunction) (MHWRender::MUIDrawManager &drawMgr, void *userData)`  
  UI callback function definition.

**Public Member Functions**

- `MPanelCanvas (const MString &panelControlName, MStatus *stat=NULL)`  
  Constructor.
- `~MPanelCanvas ()`  
  Destructor.
- `bool supportsUIDrawing ()`  
  Returns whether the attached panel control supports drawing primitives in screen space.
- `bool isValid ()`  
  Returns true if MPanelCanvas has a valid pointer to a Graph Editor object, false otherwise.
- `int createVertexBuffer (const MTimeArray &xvals, const MFloatArray &yvals, const MColorArray *colors=NULL, MStatus *stat=NULL)`  
  Create a vertex buffer with time values as the x-coordinate.
- `int createVertexBuffer (const MFloatArray &xvals, const MFloatArray &yvals, const MColorArray *colors=NULL, MStatus *stat=NULL)`  
  Create a vertex buffer with screen space values as the x and y coordinates.
- `int createPrimitive (MHWRender::MGeometry::Primitive primType, int vertexBufferId, int startIndex, int numVertices, const MDrawProperties &props, MStatus *stat=NULL)`  
  Create a primitive of the given type using the vertex buffer specified by the given id, the range of vertices used from the buffer, and a drawing style.
- `MStatus addPrimitive (int primId, int layer)`  
  Add the primitive referred to by the given id to the list of primitives to be drawn at the given layer.
- `MStatus removePrimitive (int primId, int layer)`  
  Remove the primitive referred to by the given id from the list of primitives to be drawn at the given layer.
- `MStatus destroyPrimitive (int primId)`  
  Destroy the primitive referred to by the given id.
- `MStatus destroyVertexBuffer (int vertId)`  
  Destroy the vertex buffer referred to by the given id.
- `MStatus refresh ()`  
  Force the associated Graph Editor to refresh.
- `MStatus setAutoRefresh (bool enable)`  
  Set whether the associated editor will be automatically refreshed.
- `bool isAutoRefresh ()`  
  Returns whether the associated editor will automatically refresh.
- `MStatus setLayerVisible (int layer, bool isVisible)`  
  Set whether the given layer will be drawn.
- `bool isLayerVisible (int layer, MStatus *stat=NULL)`  
  Return whether the given layer is visible.
- `int registerDrawUICallback (int layer, MDrawUIFunction func, void *clientData=NULL, MStatus *returnStatus=NULL)`  
  Register a callback to be called when the given panel is drawing the given layer.
- `void unregisterDrawUICallback (int callbackId, MStatus *returnStatus=NULL)`  
  Unregister the callback specified by the given id.


### MPanelCanvasInfo

Module `OpenMayaUI`. Source: [class_m_panel_canvas_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_panel_canvas_info.html).

Panel state information setting and retrieval. This class provides methods for setting and retrieving common information for all supported panels. Currently, the Graph Editor is the only supported editor.

**Public Member Functions**

- `MPanelCanvasInfo (const MString &editorName, MStatus *stat=NULL)`  
  The constructor.
- `virtual ~MPanelCanvasInfo ()`  
  Destructor.
- `MStatus getViewportBounds (double &left, double &right, double &bottom, double &top) const`  
  Return the viewport bounds.
- `MStatus getViewportBounds (MDoubleArray &boundsArray) const`  
  Get the attached Graph Editor's viewport bounds, using the currently defined animation time units.
- `MStatus getViewportSize (int &width, int &height) const`  
  Return the viewport size.
- `MStatus setViewportBounds (double left, double right, double bottom, double top) const`  
  Set the viewport bounds.
- `MStatus setViewportBounds (const MDoubleArray &boundsArray) const`  
  Set the attached Graph Editor's viewport bounds to those that are supplied using the currently defined animation time units as seen in the viewport.
- `bool supportsUIDrawing ()`  
  Returns whether the attached panel control supports drawing primitives in screen space.
- `void reset ()`  
  Resets all stored state information to be empty, thereby detaching from the currently attached Graph Editor (if any)
- `MString name () const`  
  Return the name of the currently attached panel.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Friends**

- `class MGraphEditorInfo`


### MProgressWindow

Module `OpenMayaUI`. Source: [class_m_progress_window.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_progress_window.html).

Create and manipulate progress windows. The MProgressWindow class manages a window containing a status message, a graphical progress gauge, and optionally a "Hit ESC to Cancel" label for interruptable operations. Only a single progress window may be displayed at any time. To reserve the use of the progress window, use the reserve() method in this class. Any methods that change the state of the progress window will fail unless the progress window has first been successfully reserved. The startProgress() and endProgress() functions show and hide the progress window. endProgress() also has the effect of unreserving the progress window, allowing it to be reserved for another use. The MEL command "progressWindow" provides equivalent functionality to this class. Attempting to manipulate a progress window that is in use by MEL will cause the methods in this class to fail.

**Static Public Member Functions**

- `static bool reserve ()`  
  Reserves a progress window for use through this class.
- `static MStatus startProgress ()`  
  Displays the progress window on the screen.
- `static MStatus endProgress ()`  
  Destroys the progress window and removes it from the screen.
- `static MStatus setProgressRange (const int minValue, const int maxValue)`  
  Sets the range (minValue, maxValue) of the progress indicator.
- `static MStatus setProgressMin (const int minValue)`  
  Sets the minimum value for the progress.
- `static MStatus setProgressMax (const int maxValue)`  
  Sets the maximum value for the progress.
- `static int progressMin (MStatus *ReturnStatus=NULL)`  
  Get the minimum progress value.
- `static int progressMax (MStatus *ReturnStatus=NULL)`  
  Get the maximum progress value.
- `static MStatus setProgress (const int progress)`  
  Sets the progress value.
- `static MStatus advanceProgress (const int amount)`  
  Increases the progress value by amount.
- `static int progress (MStatus *ReturnStatus=NULL)`  
  Get the progress value.
- `static MStatus setTitle (const MString &title)`  
  Sets the title of the progress window.
- `static MString title (MStatus *ReturnStatus=NULL)`  
  Get the window title.
- `static MStatus setProgressStatus (const MString &progressStatus)`  
  Sets the progress status string.
- `static MString progressStatus (MStatus *ReturnStatus=NULL)`  
  Get the progress status string.
- `static MStatus setInterruptable (const bool value)`  
  Sets whether the progress window is interruptable.
- `static bool isInterruptable (MStatus *ReturnStatus=NULL)`  
  Determine whether the progress window is interruptable.
- `static bool isCancelled (MStatus *ReturnStatus=NULL)`  
  Determine whether the user has tried to cancel an interruptable progress window.
- `static const char * className ()`  
  Returns the name of this class.


### MPx3dModelView

Module `OpenMayaUI`. Source: [class_m_px3d_model_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px3d_model_view.html).

MPx3dModelView is the class for user defined model views. The MPx3dModelView class works with the MPxModelEditorCommand class to create a user defined model editor that may be used in a window or in a scripted panel. When registering the model editor with the MFnPlugin::registerModelEditorCommand() method, an appropriate MPx3dModelView::creator() method is required. This class works for interactive Maya views and is not designed for rendering. One of the interesting uses of a MPx3dModelView is that it allows multiple cameras to be drawn into the same window. Like a normal model view, this view has a main camera associated with it. This camera is obtained with the getCamera() method and is the camera used for manipulations and selection. To setup a multiple camera draw, first the number of passes must be set by an overloaded multipleDrawPassCount() method. The preMultipleDraw() method allows any setup to be performed. The preMultipleDrawPass(unsigned int) is called for each pass, with the argument indicating which pass is currently being used. The camera for the specific pass may be set with the setCameraInDraw() method. The postMultipleDrawPass(unsigned int) method is called after the drawing for the indicated pass is complete. Finally any cleanup may be done with the postMultipleDraw() method. During the drawing, a filter exists to determine which items should be drawn. The okForMultipleDraw(MdagPath &) allows filtering of what should be drawn. Another approach which is faster is to tuen on the view selected mode (setViewSelected()) and use the setObjectsToView() or viewSelectedSet() to specify which items should be drawn. These values may be set per pass so that each camera has control over what gets drawn. This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MRenderOverride instead.

**Public Types**

- `enum LightingMode { kLightAll, kLightSelected, kLightActive, kLightDefault, kLightNone, kLightQuality }`  
  Lighting mode used in this view.
- `enum FogSource { kFogFragment, kFogCoordinate }`  
  Fog computation modes.
- `enum FogMode { kFogLinear, kFogExponential, kFogExponentialSquared }`  
  Drop-off modes for fog.

**Public Member Functions**

- `MPx3dModelView ()`  
  class constructor
- `virtual ~MPx3dModelView ()`  
  class destructor
- `MString name (MStatus *ReturnStatus=NULL) const`  
  Returns the name of the view.
- `virtual MString viewType () const`  
  Returns a string specifying the view type.
- `virtual void preMultipleDraw ()`  
  This method is called before any drawing is performed in the model view.
- `virtual void postMultipleDraw ()`  
  This method is called after the drawing is finished.
- `virtual void preMultipleDrawPass (unsigned int index)`  
  This method is called immediately before a specific pass is about to be drawn.
- `virtual void postMultipleDrawPass (unsigned int index)`  
  This method is called when a specified pass is finshed.
- `virtual bool okForMultipleDraw (const MDagPath &)`  
  This method provides some filter capabilities as to what is drawn.
- `virtual unsigned int multipleDrawPassCount ()`  
  This method returns the number of multiple draw passes that are going to be made.
- `bool multipleDrawEnabled () const`  
  This method returns the multiple draw enable state for this view.
- `void setMultipleDrawEnable (bool enable)`  
  This method turns enables/disables multiple camera drawing for this view.
- `bool destroyOnPanelDestruction () const`  
  This method queries the destruction setting for this MPx3dModelView which is employed when the panel associated with this view is destroyed.
- `void setDestroyOnPanelDestruction (bool how)`  
  This method enables/disables destruction of the MPx3dModelView object when the panel is destroyed.
- `MStatus updateViewingParameters ()`  
  This method tells the camera to set the view's transformation matrix.
- `virtual void removingCamera (MDagPath &cameraPath)`  
  This method should be overloaded in MPx3dModelView derived classes.
- `MStatus setDoUpdateOnMove (bool value)`  
  Some viewports require a refresh when the user has moved the top level window.
- `bool doUpdateOnMove (MStatus *ReturnStatus=NULL) const`  
  Returns the state of the doUpdateOnMove flag.
- `MStatus refresh (bool all=false, bool force=false)`  
  Refresh the this view.
- `MStatus drawText (const MString &text, const MPoint position, M3dView::TextPosition textPosition=M3dView::kLeft)`  
  Draws the given text at the given spot in the default font.
- `MStatus beginGL ()`  
  Setup port for native OpenGL drawing calls.
- `MStatus endGL ()`  
  End OpenGL drawing.
- `MStatus setCameraInDraw (MDagPath &camera)`  
  Sets the camera during a draw.
- `MStatus setDrawCameraOverride (const MMatrix &worldMatrix, const MMatrix &projectionMatrix, double left, double right, double bottom, double top, double nearpt, double farpt, bool isOrtho=false)`  
  Sets the camera during a draw.
- `MStatus setDrawCameraOverride (const MMatrix &worldMatrix, const MMatrix &projectionMatrix, const MPointArray &frustum)`  
  Sets the camera during a draw.
- `MStatus setCamera (MDagPath &camera)`  
  Set the camera for this view.
- `MStatus getCamera (MDagPath &camera)`  
  Get the camera for this view.
- `MStatus setCameraSet (MObject &cameraSet)`  
  Set the cameraSet for this view.
- `MStatus getCameraSet (MObject &cameraSet)`  
  Get the cameraSet for this view.
- `MStatus setCurrentCameraSetCamera (const MString &cameraName)`  
  Set a camera used by the currently specified cameraSet as the controlled camera for this view.
- `MStatus getCurrentCameraSetCamera (MString &cameraName)`  
  Get the cameraSet for this view.
- `virtual MString getCameraHUDName ()`  
  Return the name to use for the camera in the heads up display.
- `MStatus setDisplayHUD (bool display)`  
  Enables or disables the drawing the heads up display in this view.
- `bool displayHUD (MStatus *ReturnStatus=NULL) const`  
  Returns the heads up display state for this view.
- `MStatus drawHUDNow ()`  
  Forces the HUD viewport elements to be drawn immediately.
- `MStatus setDrawAdornments (bool display)`  
  Toggles the control of how adornments are drawn in the view.
- `bool drawAdornments (MStatus *ReturnStatus=NULL)`  
  Returns the state of the adornment drawing for this view.
- `MStatus drawAdornmentsNow ()`  
  Forces the adornment viewport elements to be drawn immediately.
- `MStatus setDisplayAxis (bool display)`  
  Sets the axis display in the MPx3dModelView.
- `bool displayAxisOn (MStatus *ReturnStatus=NULL) const`  
  Returns the axis display state for this MPx3dModelView.
- `MStatus setDisplayAxisAtOrigin (bool display)`  
  Sets the origin axis display in the MPx3dModelView.
- `bool displayAxisAtOriginOn (MStatus *ReturnStatus=NULL) const`  
  Returns the origin axis display state for this MPx3dModelView.
- `MStatus setDisplayCameraAnnotation (bool display)`  
  Sets the camera annotation display in the MPx3dModelView.
- `bool displayCameraAnnotationOn (MStatus *ReturnStatus=NULL) const`  
  Returns the camera annotation display state for this MPx3dModelView.
- `bool isVisible (MStatus *ReturnStatus=NULL) const`  
  This method returns true if this view is visible, otherwise false is returned.
- `M3dView::DisplayStyle displayStyle (MStatus *ReturnStatus=NULL) const`  
  Return the display style for this 3d view.
- `bool isShadeActiveOnly (MStatus *ReturnStatus=NULL) const`  
  Returns true if this view's display style is shaded for objects that are active and wireframe otherwise.
- `MStatus setDisplayStyle (M3dView::DisplayStyle style, bool activeOnly=false)`  
  Sets the display style for this view.
- `int portWidth (MStatus *ReturnStatus=NULL)`  
  Returns the width of the current viewport.
- `int portHeight (MStatus *ReturnStatus=NULL)`  
  Returns the height of the current viewport.
- `MStatus beginXorDrawing (bool drawOrthographic=true, bool disableDepthTesting=true, float lineWidth=1.0f, M3dView::LineStipplePattern stipplePattern=M3dView::kStippleNone, const MColor &lineColor=MColor(1, 1, 1))`  
  Setup the context for exclusive-or (XOR) drawing.
- `MStatus endXorDrawing ()`  
  Reset the context to non-exclusive-or (non-XOR) screen drawing.
- `MStatus setDrawColor (unsigned int index, M3dView::ColorTable table=M3dView::kActiveColors)`  
  Set the color to draw in.
- `MStatus setDrawColor (const MColor &color)`  
  Set the color to draw in.
- `unsigned int numDormantColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of dormant object colors in the internal application color table.
- `unsigned int numActiveColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of active object colors in the internal application color table.
- `unsigned int numUserDefinedColors (MStatus *ReturnStatus=NULL)`  
  Returns the number of user defined colors in the internal application color table.
- `MStatus setUserDefinedColor (unsigned int index, const MColor &color)`  
  Sets the user defined color at the given index.
- `unsigned int userDefinedColorIndex (unsigned int index, MStatus *ReturnStatus=NULL)`  
  Returns the index for the given user-defined color.
- `bool isBackgroundGradient (MStatus *ReturnStatus=NULL)`  
  Returns whether a gradient is being used as the background color.
- `MColor templateColor (MStatus *ReturnStatus=NULL)`  
  Returns the value of the template color.
- `MColor backgroundColor (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background color.
- `MColor backgroundColorTop (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background gradient top color.
- `MColor backgroundColorBottom (MStatus *ReturnStatus=NULL)`  
  Returns the value of the background gradient bottom color.
- `MColor colorAtIndex (unsigned int index, M3dView::ColorTable table=M3dView::kActiveColors, MStatus *ReturnStatus=NULL)`  
  Returns the value of the color at the given index in the application's color table.
- `MStatus getColorIndexAndTable (unsigned int glindex, unsigned int &index, M3dView::ColorTable &table) const`  
  Returns the color table and index representing the given OpenGL color-index value.
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &worldPt, MVector &worldVector) const`  
  Takes a point in port coordinates and returns a corresponding ray in world coordinates.
- `MStatus viewToWorld (short x_pos, short y_pos, MPoint &nearClipPt, MPoint &farClipPt) const`  
  Takes a point in port coordinates and returns a point on the near and far clipping planes.
- `MStatus viewToObjectSpace (short x_pos, short y_pos, const MMatrix &localMatrixInverse, MPoint &oPt, MVector &oVector) const`  
  Takes a point in port coordinates and returns a corresponding ray in object coordinates.
- `bool worldToView (const MPoint &worldPt, short &x_pos, short &y_pos, MStatus *ReturnStatus=NULL) const`  
  converts a point in world space to port space.
- `MStatus setObjectDisplay (M3dView::DisplayObjects, bool)`  
  Sets the display option for various types of objects.
- `bool objectDisplay (M3dView::DisplayObjects, MStatus *ReturnStatus=NULL)`  
  Test whether specific types of objects are to be displayed.
- `MStatus setBackfaceCulling (bool cull)`  
  Sets backface culling.
- `bool isBackfaceCulling (MStatus *ReturnStatus=NULL) const`  
  Returns the state of backface culling.
- `MStatus setWireframeOnShaded (bool on)`  
  Displays as wireframe on shaded.
- `bool isWireframeOnShaded (MStatus *ReturnStatus=NULL) const`  
  Returns the state of wireframe on shaded.
- `MStatus setXrayEnabled (bool xray)`  
  Sets xray display state.
- `bool isXrayEnabled (MStatus *ReturnStatus=NULL) const`  
  Returns the state of xray display.
- `MStatus setTextureDisplayEnabled (bool texture)`  
  Enables texture display.
- `bool isTextureDisplayEnabled (MStatus *ReturnStatus=NULL) const`  
  Returns the enable state of texture display.
- `MStatus setTwoSidedLighting (bool twoSided)`  
  Enables two sided lighting.
- `bool isTwoSidedLighting (MStatus *ReturnStatus=NULL) const`  
  Returns the state of two sided lighting.
- `MStatus setLightingMode (MPx3dModelView::LightingMode)`  
  Sets the lighting mode.
- `MPx3dModelView::LightingMode lightingMode (MStatus *ReturnStatus=NULL) const`  
  Returns the lighting mode.
- `MStatus setFogEnabled (bool state)`  
  Enables and disables fog.
- `bool isFogEnabled (MStatus *ReturnStatus=NULL) const`  
  Returns true if fog is enabled.
- `MPx3dModelView::FogSource fogSource (MStatus *ReturnStatus=NULL) const`  
  Returns the algorithm used to compute fog.
- `MStatus setFogSource (MPx3dModelView::FogSource)`  
  Sets the type of fog algorithm to use.
- `MPx3dModelView::FogMode fogMode (MStatus *ReturnStatus=NULL) const`  
  Return the type of drop off used with fog.
- `MStatus setFogMode (MPx3dModelView::FogMode)`  
  Sets the drop-off mode for fog.
- `double fogDensity (MStatus *ReturnStatus=NULL) const`  
  Returns the fog density.
- `MStatus setFogDensity (double)`  
  Determines the density of hardware fogging.
- `double fogStart (MStatus *ReturnStatus=NULL) const`  
  Returns the fog start position.
- `MStatus setFogStart (double)`  
  Determines the start location of hardware fogging.
- `double fogEnd (MStatus *ReturnStatus=NULL) const`  
  Returns the fog end position.
- `MStatus setFogEnd (double)`  
  Determines the end location of hardware fogging.
- `MColor fogColor (MStatus *ReturnStatus=NULL) const`  
  Returns the fog color.
- `MStatus setFogColor (const MColor &)`  
  Sets the color used for hardware fogging.
- `bool isBackgroundFogEnabled (MStatus *ReturnStatus=NULL) const`  
  Returns true if the background fog is enabled.
- `MStatus setBackgroundFogEnabled (bool enable)`  
  Enables and disables background fog.
- `MString viewSelectedPrefix (MStatus *ReturnStatus) const`  
  Returns the prefix used when displaying the camera name in the heads up display when view selected in on.
- `MStatus setViewSelectedPrefix (const MString &prefix)`  
  Sets the prefix for the camera name as displayed in the heads up display when view selected is enabled.
- `bool viewSelected (MStatus *ReturnStatus=NULL) const`  
  Returns the state of view selected for this view.
- `MStatus setViewSelected (bool viewSelected)`  
  Enables the view selected mode.
- `MObject viewSelectedSet (MStatus *ReturnStatus=NULL) const`  
  Returns an MObject for the set used by view selected.
- `MStatus setViewSelectedSet (const MObject &set)`  
  Sets the list of objects used by view selected as an object set.
- `MStatus getObjectsToView (MSelectionList &list) const`  
  Returns a selection list containing all of the objects on the view selected list.
- `MStatus setObjectsToView (const MSelectionList &list)`  
  Sets the list of objects used by view selected as a selection list.
- `bool viewIsFiltered (MStatus *ReturnStatus=NULL) const`  
  Returns the state of view filtering for this view.
- `MStatus filteredObjectList (MSelectionList &list) const`  
  Returns a selection list containing all of the objects that remain after filtering is applied to the view.
- `bool hasStereoBufferSupport () const`  
  Returns true if this 3dModelView is running in stereo buffer mode.
- `MStatus getAsM3dView (M3dView &view)`  
  Get this MPx3dModelView as a M3dView.
- `virtual bool wantStereoGLBuffer () const`  
  Users should override this method if they want a stereo buffer enabled MPx3dModelView.
- `MStatus setInStereoDrawMode (bool flag)`  
  Derived classes should call this method to indicate to Maya whether the view is currently drawing in stereo.
- `virtual bool requestOkForDraw (const MDrawRequest &)`  
  This method provides some filter capabilities as to what is drawn.

**Static Public Member Functions**

- `static MPx3dModelView * getModelView (const MString &name, MStatus *ReturnStatus=NULL)`  
  Returns a pointer to a MPx3dModelView that has the passed name with the specified type (the same typed when registering the control).
- `static const char * className ()`  
  Returns the name of this class.


### MPxClientDeviceNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_client_device_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_client_device_node.html).

Base class for client device creation. MPxClientDeviceNode is an extension of MPxThreadedDevice node, and is intended for creating maya devices that act as clients. An example of this will be a Maya device that listens for animation data that is sent from another application. See MPxThreadedDevice node for details on how to use the memory queue system Plug-ins should create the MPxClientDeviceNode with a kCustomClient parameter. This class contains the serverName and deviceName attributes. If these attributes are changed, the node will restart the thread so that processing will continue with the new settings. NOTE: This class relies on the idle event queue to perform the node attribute updating. As a result, it will not work in Maya batch mode.

**Public Types**

- `enum ClientType { kUser, kInternal }`  
  Defines the type client server.

**Public Member Functions**

- `MPxClientDeviceNode (ClientType typ=kUser)`  
  Constructor.
- `~MPxClientDeviceNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  Returns the type of node that this is.
- `ClientType clientType ()`  
  Returns the client type of this node.
- `virtual void threadHandler (const char *serverName, const char *deviceName)`  
  This method should be overridden in user defined nodes rather than the parameterless version that exists in the base class.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject serverName`  
  Server.
- `static MObject deviceName`  
  Device.


### MPxContext

Module `OpenMayaUI`. Source: [class_m_px_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_context.html).

Base class for user defined contexts. This is the base class for user defined contexts. Contexts provide a way to create interactive tools in Maya. A context class defines what happens when interactive events, such as mouse events, occur within an interactive panel in Maya. Since there are default actions for all tools in Maya, such as the right mouse button event which brings up an options menu, only as subset of the events that occur in a view can be overridden. The events that can be overridden are: doPress - mouse button press event (left & middle only) doRelease - mouse buttons release event (left & middle only) doDrag - mouse button drag event (left & middle only) doHold - mouse button hold event (left & middle only) deleteAction - delete/backspace key event completeAction - complete key event abortAction - abort/escape key event A context is activated by pressing the toolButton for that context. Once the context is activated, it will handle the events that occur within 3d panel. A context is deactivated when some other tool button is pressed. There can be more than one instance of a context in Maya, for example, dragging a tool icon into the shelf creates another instance of that context. Since there can be multiple instances of the same context there is a command class, MPxContextCommand, which is responsible for the creation of contexts. See MPxContextCommand for more information.

**Public Types**

- `enum ImageIndex { kImage1, kImage2, kImage3 }`  
  Used to select between the three possible images associated with the context.

**Public Member Functions**

- `MPxContext ()`  
  Class contstructor.
- `virtual ~MPxContext ()`  
  Default destructor.
- `virtual void toolOnSetup (MEvent &event)`  
  This method is called when the context is activated, i.e when the toolButton for the context is pressed.
- `virtual void toolOffCleanup ()`  
  This method is called when the context is deactivated, i.e when another context is activated.
- `virtual MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is not available in Python.
- `virtual MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is not available in Python.
- `virtual MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is not available in Python.
- `virtual MStatus doHold (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is not available in Python.
- `virtual MStatus drawFeedback (MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is not available in Python.
- `virtual MStatus doPtrMoved (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context)`  
  This method is called when a mouse move event occurs.
- `virtual MStatus doPress (MEvent &event)`  
  This method is called when any mouse button is pressed.
- `virtual MStatus doRelease (MEvent &event)`  
  This method is called when any mouse button is released.
- `virtual MStatus doDrag (MEvent &event)`  
  This method is called when a mouse drag event occurs.
- `virtual MStatus doHold (MEvent &event)`  
  This method is called when a mouse button is pressed but before the mouse is dragged.
- `virtual MStatus doPtrMoved (MEvent &event)`  
  This method is called when a mouse move event occurs.
- `virtual MStatus doEnterRegion (MEvent &event)`  
  This method is called when the mouse pointer enters a screen panel region.
- `virtual MStatus helpStateHasChanged (MEvent &event)`  
  This method is called whenever the help state may need to be updated.
- `virtual void deleteAction ()`  
  This method is called when the delete or backspace key is pressed.
- `virtual void completeAction ()`  
  This method is called when the complete key is pressed.
- `virtual MStatus addManipulator (const MObject &manipulator)`  
  This method adds a manipulator to the context.
- `virtual MStatus deleteManipulators ()`  
  This method deletes all the manipulators that belong to the context.
- `MStatus setImage (const MString &image, ImageIndex index)`  
  This method is used to set an XPM icon image that is to be used to represent this tool context in various places including the tool bar and can be queried from mel using the contextInfo command.
- `MString image (ImageIndex index, MStatus *ReturnStatus=NULL) const`  
  This method is used to retrieve an XPM icon image that has previously been set for this tool context.
- `virtual void abortAction ()`  
  This method is called when the abort key is pressed.
- `virtual bool processNumericalInput (const MDoubleArray &values, const MIntArray &flags, bool isAbsolute)`  
  This method processes the input from the numerical input field.
- `virtual bool feedbackNumericalInput () const`  
  This method is called to update the numerical feedback.
- `virtual MSyntax::MArgType argTypeNumericalInput (unsigned int index) const`  
  This method is used by the feedback line to determine what units to display.
- `virtual MString stringClassName () const`  
  This method is called to determine the name that uniquely identifies the context.
- `virtual void getClassName (MString &name) const`  
  NO SCRIPT SUPPORT.
- `MStatus getImage (MString &image, ImageIndex index) const`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus setHelpString (const MString &str)`  
  USE _setHelpString() IN SCRIPT.
- `MStatus setTitleString (const MString &str)`  
  USE _setTitleString() IN SCRIPT.
- `MStatus setCursor (const MCursor &newCursor)`  
  USE _setCursor() IN SCRIPT.
- `MStatus beginMarquee (MEvent &event)`  
  USE _beginMarquee() IN SCRIPT.
- `MStatus dragMarquee (MEvent &event)`  
  USE _dragMarquee() IN SCRIPT.
- `MStatus releaseMarquee (MEvent &event, short &top, short &left, short &bottom, short &right)`  
  USE _releaseMarquee() IN SCRIPT.
- `virtual MPxToolCommand * newToolCommand ()`  
  CALL _newToolCommand() IN SCRIPT.

**Static Protected Member Functions**

- `static bool ignoreEntry (const MIntArray &flags, unsigned int entry)`  
  USE _ignoreEntry() IN SCRIPT.


### MPxContextCommand

Module `OpenMayaUI`. Source: [class_m_px_context_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_context_command.html).

Base class for context creation commands. This is the base class for context creation commands. The purpose of this command class is to create instances of user contexts derived from MPxContext, and to allow the MEL programmer to edit and query various properties of the context related to this command. The user will derive off of this class and override the makeObj method which will be called when this command is invoked in Maya. The context command is registered in Maya using MFnPlugin::registerContextCommand

**Public Member Functions**

- `MPxContextCommand ()`  
  Class constructor.
- `virtual ~MPxContextCommand ()`  
  Class destructor.
- `virtual MStatus doEditFlags ()`  
  This method is called when the command is called in edit mode.
- `virtual MStatus doQueryFlags ()`  
  This method is called when the command is called in query mode.
- `virtual MPxContext * makeObj ()`  
  This function is used to instantiate a proxy context.
- `virtual MStatus appendSyntax ()`  
  This method should be overridden to append syntax to the context command.
- `MStatus setResult (bool result)`  
  This method should be called when the result of the context command is a boolean.
- `MStatus setResult (int result)`  
  This method should be called when the result of the context command is an integer.
- `MStatus setResult (double result)`  
  This method should be called when the result of the context command is a double.
- `MStatus setResult (const MString &result)`  
  This method should be called when the result of the context command a string.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`  
  USE _syntax() IN SCRIPT.
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`  
  USE _parser() IN SCRIPT.


### MPxControlCommand

Module `OpenMayaUI`. Source: [class_m_px_control_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_control_command.html).

Base class for control creation commands. MPxControlCommand is the base class for user defined UI control commands. This command gives all of the flags and options of the base control command in addition to any user defined flags or behaviours. When registering this command, use the MFnPlugin::registerControlCommand() method. All control commands have a corresponding MPxUIControl. It is important to note that a given MPxControlCommand is reponsible for only ONE MPxUIControl.

**Public Member Functions**

- `MPxControlCommand ()`  
  Class constructor.
- `virtual ~MPxControlCommand ()`  
  Class destructor.
- `virtual MPxUIControl * makeControl ()=0`  
  This method is called when the UI control should be created.
- `virtual MStatus doEditFlags ()`  
  This method is called when the command is called in edit mode.
- `virtual MStatus doQueryFlags ()`  
  This method is invoked during query mode, and the default method should be overridden in user-defined control commands to determine which query flags are set in conjunction with the argument parser for the command.
- `virtual MStatus appendSyntax ()`  
  This method should be overridden to append syntax to the control command.
- `virtual bool skipFlagForCreate (const MString &longFlag)`  
  Returns true if the passed long flag name should be skipped during the creation portion of the command.
- `void clearResult ()`  
  This method should be called to clear the result to be output by the command.
- `MStatus setResult (bool result)`  
  This method should be called when the result of the control command is a boolean.
- `MStatus setResult (int result)`  
  This method should be called when the result of the control command is an integer.
- `MStatus setResult (double result)`  
  This method should be called when the result of the control command is a double.
- `MStatus setResult (const MString &result)`  
  This method should be called when the result of the control command is a string.
- `MStatus setResult (const MStringArray &result)`  
  This method should be called when the result of the control command is a string array.
- `MStatus setResult (const MIntArray &result)`  
  This method should be called when the result of the control command is a integer array.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MPxUIControl * control (MStatus *ReturnStatus=NULL)`  
  USE _control() IN SCRIPT.
- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`  
  USE _syntax() IN SCRIPT.
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`  
  USE _parser() IN SCRIPT.


### MPxDragAndDropBehavior

Module `OpenMayaUI`. Source: [class_m_px_drag_and_drop_behavior.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_drag_and_drop_behavior.html).

Drag and Drop Behavior. This is the base class for user defined drag and drop behaviors. This class allows a plugin to override the behavior of drag and drop connections from nodes in the hyperGraph/hyperShade to other nodes or other UI. These behaviors are defined by the type of relationship that is trying to be resolved. These are connectAttrToAttr(), connectAttrToNode(), connectNodeToAttr(), connectNodeToNode(). In order for the overridden methods to be executed the shouldBeUsedFor() method must be overridden in order to decide which behavior to use given the source and destination node.

**Public Member Functions**

- `MPxDragAndDropBehavior ()`  
  Class constructor.
- `virtual ~MPxDragAndDropBehavior ()`  
  Class destructor.
- `MPxDragAndDropBehavior (void *init)`  
  This method is obsolete.
- `virtual bool shouldBeUsedFor (MObject &sourceNode, MObject &destinationNode, MPlug &sourcePlug, MPlug &destinationPlug)`  
  This method must be overridden in order to use a drag and drop behavior.
- `virtual MStatus connectAttrToAttr (MPlug &sourcePlug, MPlug &destinationPlug, bool force)`  
  This method is called by the defaultNavigation command to connect a source attribute to a destination attribute.
- `virtual MStatus connectAttrToNode (MPlug &sourcePlug, MObject &destinationNode, bool force)`  
  This method is called by the defaultNavigation command to connect a source attribute to a destination node.You should override this method if you can determine from the type of source node and attribute and the type of destination node what the user is trying to do and you know the appropriate connections that must be made for the end result to be what the user expects.
- `virtual MStatus connectNodeToAttr (MObject &sourceNode, MPlug &destinationPlug, bool force)`  
  This method is called by the defaultNavigation command to connect a source node to a destination attribute.
- `virtual MStatus connectNodeToNode (MObject &sourceNode, MObject &destinationNode, bool force)`  
  This method is called by the defaultNavigation command to connect a source node to a destination node.You should override this method if you can determine from the type of source node and the type of destination node what the user is trying to do and you know the appropriate connections that must be made for the end result to be what the user expects.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxGlBuffer

Module `OpenMayaUI`. Source: [class_m_px_gl_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_gl_buffer.html).

Historically this class was used to created offscreen buffers on Linux. This class is now using FBO. Invoke openFbo() method to create a Frame Buffer Object. The contents of the frame buffer object (FBO) can be read back by using the bindFbo() method and OpenGl calls to read pixels. After rendering and reading pixels, the frame buffer object can be destroyed by calling closeFbo(). This method is obsolete. [from before Maya 2016] Deprecated:Use MHWRender::MRenderOverride and MHWRender::MRenderTarget instead.

**Public Member Functions**

- `MPxGlBuffer ()`  
  Construct an MPxGlBuffer for use with the currently active view.
- `MPxGlBuffer (M3dView &view)`  
  Construct an MPxGlBuffer for use with a specified view.
- `MStatus openFbo (short width, short height, M3dView &)`  
  Create a frame buffer object where the renderer result will be stored.
- `MStatus closeFbo (M3dView &)`  
  Destroy a frame buffer object that was created by createFbo.
- `MStatus bindFbo ()`  
  If a frame buffer object was created using the method openFbo, then this method can be used to bind that Fbo.
- `MStatus unbindFbo ()`  
  If a frame buffer object was created using the method openFbo, then this method can be used to unbind that Fbo.
- `virtual void beginBufferNotify ()`  
  This method is called when the GL buffer is being setup by the viewport renderer.
- `virtual void endBufferNotify ()`  
  This method is called when the GL buffer is being shutdown by the viewport renderer.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxHardwareShader

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_hardware_shader.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_hardware_shader.html).

Base class for user defined hardware shaders. MPxHardwareShader allows the creation of user-defined hardware shaders. A hardware shader allows the plug-in writer to control the on-screen display of an object in Maya. You must derive a hardware shader node from MPxHardwareShader for it to have any effect on the on-screen display at all. In addition to their effect on the on-screen display of objects, hardware shaders function as surface shader nodes. This allows you to connect any shading network up to the hardware shader's outColor attribute to handle the shading during a software render. To create a working hardware shader, derive from this class and override the render() and optionally the populateRequirements() methods. The other methods of the parent class MPxNode may also be overridden to perform dependency node capabilities. NOTE: Plug-in hardware shaders are fully supported for polygonal mesh shapes. NURBS surfaces are only supported in the High Quality Interactive viewport and Hardware Renderer.

**Classes**

- `struct RenderParameters`  
  Provides information on how to render the image.
- `struct ShaderContext`  
  Provides contextual information about the current invocation of the shader.

**Public Types**

- `enum TransparencyOptions { kIsTransparent = 0x0001, kNoTransparencyFrontBackCull = 0x0002, kNoTransparencyPolygonSort = 0x0004 }`  
  Transparency option bitmasks.

**Public Member Functions**

- `MPxHardwareShader ()`  
  Class constructor.
- `~MPxHardwareShader () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `MStatus setVaryingParameters (const MVaryingParameterList &parameters, bool remapCurrentValues=true, MDagModifier *dagModifier=NULL)`  
  Call this method to set the list of varying parameters this shader uses.
- `MStatus setUniformParameters (const MUniformParameterList &parameters, bool remapCurrentValues=true, MDagModifier *dagModifier=NULL)`  
  Call this method to set the list of uniform parameters this shader uses.
- `virtual MStatus render (MGeometryList &iterator)`  
  Override this method to render geometry using your hardware shader.
- `virtual unsigned int transparencyOptions ()`  
  This method returns transparency options for usage as hints for Maya's internal draw during a given rendering pass.
- `virtual const MRenderProfile & profile ()`  
  Override this method to specify the renderers your shader supports.
- `virtual MStatus populateRequirements (const MPxHardwareShader::ShaderContext &context, MGeometryRequirementsLegacy &requirements)`  
  This method is not available in Python.
- `virtual MStatus getAvailableImages (const MPxHardwareShader::ShaderContext &context, const MString &uvSetName, MStringArray &imageNames)`  
  This method is not available in Python.
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, const MString &imageName, floatRegion region, int &imageWidth, int &imageHeight)`  
  This method is obsolete.
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, const MString &imageName, floatRegion region, const MPxHardwareShader::RenderParameters &parameters, int &imageWidth, int &imageHeight)`  
  This method is not available in Python.
- `virtual MStatus renderImage (const MPxHardwareShader::ShaderContext &context, MHWRender::MUIDrawManager &uiDrawManager, const MString &imageName, floatRegion region, const MPxHardwareShader::RenderParameters &parameters, int &imageWidth, int &imageHeight)`  
  This method is not available in Python.
- `virtual MStatus renderSwatchImage (MImage &image)`  
  If the shader specifies to override swatch rendering, then this method must be overridden in order to draw anything into a swatch.

**Static Public Member Functions**

- `static MPxHardwareShader * getHardwareShaderPtr (MObject &object)`  
  This is a static convenience method to be able to get an MPxHardwareShader from an MObject provided by a swatch generator class (Class derived from MSwatchRenderRegister).
- `static MString findResource (const MString &name, const MString &shaderPath, MStatus *status=NULL)`  
  This is a static utility to find the full path to a shader resource (typically a texture).
- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject outColor`  
  output color value
- `static MObject outColorR`  
  output color red
- `static MObject outColorG`  
  output color green
- `static MObject outColorB`  
  output color blue


### MPxHwShaderNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_hw_shader_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_hw_shader_node.html).

Base class for user defined hardware shaders. MPxHwShaderNode allows the creation of user-defined hardware shaders. A hardware shader allows the plug-in writer to control the on-screen display of an object in Maya. You must derive a hardware shader node from MPxHwShaderNode for it to have any affect on the on-screen display at all. In addition to their affect on the on-screen display of objects, hardware shaders function as surface shader nodes. This allows you to connect any shading network up to the hardware shader's outColor attribute to handle the shading during a software render. To create a working hardware shader, derive from this class and override the bind(), unbind(), and geometry() methods. If your hardware shader uses texture coordinates from Maya, you also need to override either texCoordsPerVertex or getTexCoordSetNames(). The other methods of the parent class MPxNode may also be overridden to perform dependency node capabilities. NOTE: Plug-in hardware shaders are fully supported for polygonal mesh shapes. NURBS surfaces are only supported in the High Quality Interactive viewport and Hardware Renderer if the glBind/glGeometry/glUnbind methods of this class are implemented.

**Classes**

- `struct RenderParameters`  
  Provides information on how to render the image.

**Public Types**

- `enum Writeable { kWriteNone = 0x0000, kWriteVertexArray = 0x0001, kWriteNormalArray = 0x0002, kWriteColorArrays = 0x0004, kWriteTexCoordArrays = 0x0008, kWriteAll = 0x000f }`  
  Bit masks used in conjuction with the 'writeable' parameter passed to the geometry() method to determine which arrays the shader is allowed to write to.
- `enum DirtyMask { kDirtyNone = 0x0000, kDirtyVertexArray = 0x0001, kDirtyNormalArray = 0x0002, kDirtyColorArrays = 0x0004, kDirtyTexCoordArrays = 0x0008, kDirtyAll = 0x000f }`  
  Bit masks used in combination with the return value of the dirtyMask() method to determine which portions of the geometry are dirty.
- `enum TransparencyOptions { kIsTransparent = 0x0001, kNoTransparencyFrontBackCull = 0x0002, kNoTransparencyPolygonSort = 0x0004 }`  
  Bit masks to be returned by the shader's transparencyOptions() method.

**Public Member Functions**

- `MPxHwShaderNode ()`  
  Class constructor.
- `~MPxHwShaderNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual MStatus bind (const MDrawRequest &request, M3dView &view)`  
  This method is invoked for hardware rendering to Maya's 3D view.
- `virtual MStatus unbind (const MDrawRequest &request, M3dView &view)`  
  This method is invoked for hardware rendering to Maya's 3D view.
- `virtual MStatus geometry (const MDrawRequest &request, M3dView &view, int prim, unsigned int writable, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays)`  
  This method is obsolete.
- `virtual MStatus geometry (const MDrawRequest &request, M3dView &view, int prim, unsigned int writable, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays, const int *faceIDs, const float *localUVCoord)`  
  This method is invoked for hardware rendering to Maya's 3D view.
- `virtual MStatus glBind (const MDagPath &shapePath)`  
  This method should only be overridden for hardware rendering.
- `virtual MStatus glUnbind (const MDagPath &shapePath)`  
  This method should only be overridden for hardware rendering.
- `virtual MStatus glGeometry (const MDagPath &shapePath, int glPrim, unsigned int writeMask, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays)`  
  This method is obsolete.
- `virtual MStatus glGeometry (const MDagPath &shapePath, int glPrim, unsigned int writeMask, int indexCount, const unsigned int *indexArray, int vertexCount, const int *vertexIDs, const float *vertexArray, int normalCount, floatArrayPtr normalArrays, int colorCount, floatArrayPtr colorArrays, int texCoordCount, floatArrayPtr texCoordArrays, const int *faceIDs, const float *localUVCoord)`  
  This method should only be overridden for hardware rendering.
- `virtual bool supportsBatching () const`  
  Specifies whether or not this shader supports batched rendering of shapes.
- `virtual bool invertTexCoords () const`  
  Specifies whether this shader requires inverted texture coordinates.
- `const MDagPath & currentPath () const`  
  This method returns a reference to the current path that the shader is invoked for.
- `unsigned int dirtyMask () const`  
  This method returns a "dirty" mask that indicates which geometry items have changed from the last invocation of the plugin to draw.
- `virtual int normalsPerVertex ()`  
  Specifies how many normals per vertex the HW shader would like Maya to provide.
- `virtual int colorsPerVertex ()`  
  This method returns the number of color values per vertex that the hw shader node would like to receive from Maya.
- `virtual int getColorSetNames (MStringArray &names)`  
  NO SCRIPT SUPPORT.
- `virtual int texCoordsPerVertex ()`  
  This method returns the number of texture coordinate values per vertex that the hw shader node would like to receive from Maya.
- `virtual int getTexCoordSetNames (MStringArray &names)`  
  NO SCRIPT SUPPORT.
- `virtual bool hasTransparency ()`  
  This method returns a boolean value that indicates whether the object will be drawn transparently or not.
- `virtual bool provideVertexIDs ()`  
  This method returns a boolean value that indicates whether a map of the vertex IDs will be provided to the geometry method.
- `virtual bool provideFaceIDs ()`  
  This method returns a boolean value that indicates whether a map of the face IDs will be provided to the geometry method.
- `virtual bool provideLocalUVCoord ()`  
  This method returns a boolean value that indicates whether the local uv coordinates of the subdivided face vertices will be provided to the geometry method.
- `virtual unsigned int transparencyOptions ()`  
  This method returns transparency options for usage as hints for Maya's internal draw during a given rendering pass.
- `virtual MStatus getAvailableImages (const MString &uvSetName, MStringArray &imageNames)`  
  This method is not available in Python.
- `virtual MStatus renderImage (const MString &imageName, floatRegion region, int &imageWidth, int &imageHeight)`  
  This method is obsolete.
- `virtual MStatus renderImage (const MString &imageName, floatRegion region, const MPxHwShaderNode::RenderParameters &parameters, int &imageWidth, int &imageHeight)`  
  This method is not available in Python.
- `virtual MStatus renderImage (MHWRender::MUIDrawManager &uiDrawManager, const MString &imageName, floatRegion region, const MPxHwShaderNode::RenderParameters &parameters, int &imageWidth, int &imageHeight)`  
  This method is not available in Python.
- `virtual MStatus renderSwatchImage (MImage &image)`  
  If the shader specifies to override swatch rendering, then this method must be overridden in order to draw anything into a swatch.
- `MObject currentShadingEngine () const`  
  This method returns an MObject to the shading engine that is currently being rendered.

**Static Public Member Functions**

- `static MPxHwShaderNode * getHwShaderNodePtr (MObject &object)`  
  This is a static convenience method to be able to get an MPxHwShaderNode from an MObject provided by a swatch generator class (Class derived from MSwatchRenderRegister).
- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject outColor`  
  output color value
- `static MObject outColorR`  
  output color red
- `static MObject outColorG`  
  output color green
- `static MObject outColorB`  
  output color blue
- `static MObject outTransparency`  
  output transparency value
- `static MObject outTransparencyR`  
  output transparency red
- `static MObject outTransparencyG`  
  output transparency green
- `static MObject outTransparencyB`  
  output transparency blue
- `static MObject outMatteOpacity`  
  output matte opacity value
- `static MObject outMatteOpacityR`  
  output matte opacity red
- `static MObject outMatteOpacityG`  
  output matte opacity green
- `static MObject outMatteOpacityB`  
  output matte opacity blue
- `static MObject outGlowColor`  
  output glow color value
- `static MObject outGlowColorR`  
  output glow color red
- `static MObject outGlowColorG`  
  output glow color green
- `static MObject outGlowColorB`  
  output glow color blue


### MPxLocatorNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_locator_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_locator_node.html).

Base class for user defined locators. MPxLocatorNode allows the creation of user-defined locators. A locator is a DAG shape that is drawn on the screen, but that is not rendered. Locators are full dependency nodes and can have attributes and a compute method. To create a locator, derive off of this class and override the draw method. The draw method can be overridden to draw custom geometry using standard OpenGL calls. The other methods of the parent class MPxNode may also be overridden to perform dependency node capabilities as well.

**Public Member Functions**

- `MPxLocatorNode ()`  
  Constructor
- `~MPxLocatorNode () override`  
  Destructor
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`  
  Overriding this method allows the drawing of custom geometry using standard OpenGL calls.
- `virtual bool isBounded () const`  
  This method should be overridden to return true if the user supplies a bounding box routine.
- `virtual MBoundingBox boundingBox () const`  
  This method should be overridden to return a bounding box for the locator.
- `void getCacheSetup (const MEvaluationNode &evalNode, MNodeCacheDisablingInfo &disablingInfo, MNodeCacheSetupInfo &cacheSetupInfo, MObjectArray &monitoredAttributes) const override`  
  Disables Cached Playback support by default.
- `unsigned int color (M3dView::DisplayStatus displayStatus)`  
  This method returns the index of the color that is the default draw color for the given display status.
- `MColor colorRGB (M3dView::DisplayStatus displayStatus)`  
  This method returns the RGB values of the default draw color for the given display status.
- `virtual bool excludeAsLocator () const`  
  When the modelPanel is set to not draw locators, returing true will also not draw the custom locator.
- `virtual bool isTransparent () const`  
  Indicates that this locator uses transparency during draw() method calls.
- `virtual bool drawLast () const`  
  Indicates that this locator should be the last item draw in a given refresh cycle.
- `virtual bool useClosestPointForSelection () const`  
  Determines whether Maya should call closestPoint() when doing single selection.
- `virtual MPoint closestPoint (MPoint cursorRayPoint, MVector cursorRayDir) const`  
  Returns the point on the locator, in the locator's local space, which is closest along the specified ray.
- `virtual MSelectionMask getShapeSelectionMask () const`  
  This routine can be overridden to provide information about the selection mask of the locator.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject underWorldObject`  
  under world attribute
- `static MObject localPosition`  
  local position attribute
- `static MObject localPositionX`  
  X component of localPosition.
- `static MObject localPositionY`  
  Y component of localPosition.
- `static MObject localPositionZ`  
  Z component of localPosition.
- `static MObject worldPosition`  
  world position attribute
- `static MObject worldPositionX`  
  X component of worldPosition.
- `static MObject worldPositionY`  
  Y component of worldPosition.
- `static MObject worldPositionZ`  
  Z component of worldPosition.
- `static MObject localScale`  
  local scale attribute
- `static MObject localScaleX`  
  X component of localScale.
- `static MObject localScaleY`  
  Y component of localScale.
- `static MObject localScaleZ`  
  Z component of localScale.
- `static MObject nodeBoundingBox`  
  bounding box attribute
- `static MObject nodeBoundingBoxMin`  
  bounding box minimum point
- `static MObject nodeBoundingBoxMinX`  
  X component of boundingBoxMin.
- `static MObject nodeBoundingBoxMinY`  
  Y component of boundingBoxMin.
- `static MObject nodeBoundingBoxMinZ`  
  Z component of boundingBoxMin.
- `static MObject nodeBoundingBoxMax`  
  bounding box maximum point
- `static MObject nodeBoundingBoxMaxX`  
  X component of boundingBoxMax.
- `static MObject nodeBoundingBoxMaxY`  
  Y component of boundingBoxMax.
- `static MObject nodeBoundingBoxMaxZ`  
  Z component of boundingBoxMax.
- `static MObject nodeBoundingBoxSize`  
  bounding box size vector
- `static MObject nodeBoundingBoxSizeX`  
  X component of boundingBoxSize.
- `static MObject nodeBoundingBoxSizeY`  
  Y component of boundingBoxSize.
- `static MObject nodeBoundingBoxSizeZ`  
  Z component of boundingBoxSize.
- `static MObject center`  
  object center attribute
- `static MObject boundingBoxCenterX`  
  X component of boundingBoxCenter.
- `static MObject boundingBoxCenterY`  
  Y component of boundingBoxCenter.
- `static MObject boundingBoxCenterZ`  
  Z component of boundingBoxCenter.
- `static MObject matrix`  
  matrix attribute
- `static MObject inverseMatrix`  
  inverse matrix attribute
- `static MObject worldMatrix`  
  world matrix attribute
- `static MObject worldInverseMatrix`  
  inverse world matrix attribute
- `static MObject parentMatrix`  
  parent matrix attribute
- `static MObject parentInverseMatrix`  
  inverse parent matrix attribute
- `static MObject visibility`  
  visibility attribute
- `static MObject intermediateObject`  
  intermediate object attribute
- `static MObject isTemplated`  
  template attribute
- `static MObject instObjGroups`  
  instances object group info attribute
- `static MObject objectGroups`  
  object groups attributes
- `static MObject objectGrpCompList`  
  component in object groups attribute
- `static MObject objectGroupId`  
  group id attribute
- `static MObject objectGroupColor`  
  group id attribute
- `static MObject useObjectColor`  
  controls choice of wireframe dormant object color
- `static MObject objectColor`  
  the per object dormant wireframe color


### MPxManipContainer

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_manip_container.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_manip_container.html).

Base class for user defined manipulator containers. MPxManipContainer is the base class for user-defined container manipulators. This class is derived from MPxNode since manipulators in Maya are dependency nodes. MPxManipContainer is a container manipulator that has at least one base manipulator. MPxManipContainer has methods for adding the following base manipulators types to the container: FreePointTriadManip, DirectionManip, DistanceManip, PointOnCurveManip, PointOnSurfaceManip, DiscManip, CircleSweepManip, ToggleManip, StateManip, and CurveSegmentManip. A container manipulator has one converter which is the interface between the container's children manipulators and the node plugs they affect. The values on the converter that are related to children manipulators are called converterManipValues, and the values on the converter that are related to the node plugs are called converterPlugValues. The conversion between converterManipValues and converterPlugValues are performed through conversion callback methods, except when there is a one-to-one connection between a converterManipValue and a converterPlugValue. There are two kinds of conversion callback methods: manipToPlug and plugToManip. A plugToManipConversionCallback is used to calculate a converterManipValue from various converterPlugValues. This callback has access to all the converterPlugValues and returns the value of a converterManipValue. A manipToPlugConversionCallback is used to calculate a converterPlugValue from various converterManipValues. This callback has access to all the converterManipValues and returns the value of a converterPlugValue. In Viewport 2.0, all child manipulators will draw automatically. However for custom drawing (things done in the draw() method for the default viewport), the plugin must also implement preDrawUI() and drawUI(). Note that selection relies on the default viewport draw pass so the draw() method must still be implemented even if the manipulator is not intended for use in the default viewport. preDrawUI() is called before drawUI() and should be used to prepare and cache data which will be needed to perform the custom drawing in drawUI(). In order for an MPxManipContainer to be able to run, the manipulator needs to know that the initialization is finished through the finishAddingManips method.

**Public Types**

- `enum baseType { kFreePointTriadManip, kDirectionManip, kDistanceManip, kPointOnCurveManip, kPointOnSurfaceManip, kDiscManip, kCircleSweepManip, kToggleManip, kStateManip, kCurveSegmentManip, kCustomManip }`  
  Built-in manipulator types.
- `typedef MManipData(MPxManipContainer::* plugToManipConversionCallback) (unsigned int manipIndex)`  
  Pointer to a plug-to-manip conversion callback function.
- `typedef MManipData(MPxManipContainer::* manipToPlugConversionCallback) (unsigned int plugIndex)`  
  Pointer to a manip-to-plug conversion callback function.

**Public Member Functions**

- `MPxManipContainer ()`  
  Constructor.
- `~MPxManipContainer () override`  
  Destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual void preDrawUI (const M3dView &view)`  
  This method is not available in Python.
- `virtual void drawUI (MHWRender::MUIDrawManager &drawManager, const MHWRender::MFrameContext &frameContext) const`  
  This method is not available in Python.
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus status)`  
  This method can be overloaded to customize the drawing of the child manipulators.
- `virtual MStatus connectToDependNode (const MObject &dependNode)`  
  This method connects the manipulator to the dependency node.
- `virtual MStatus createChildren ()`  
  This method should be overridden in user defined manipulators.
- `MDagPath addFreePointTriadManip (const MString &manipName, const MString &pointName)`  
  This method creates a FreePointTriadManip and adds it to the MPxManipContainer container.
- `MDagPath addDirectionManip (const MString &manipName, const MString &directionName)`  
  This method creates a DirectionManip and adds it to the MPxManipContainer container.
- `MDagPath addDistanceManip (const MString &manipName, const MString &distanceName)`  
  This method creates a DistanceManip and adds it to the MPxManipContainer container.
- `MDagPath addPointOnCurveManip (const MString &manipName, const MString &paramName)`  
  This method creates a PointOnCurveManip and adds it to the MPxManipContainer container.
- `MDagPath addPointOnSurfaceManip (const MString &manipName, const MString &paramName)`  
  This method creates a PointOnSurfaceManip and adds it to the MPxManipContainer container.
- `MDagPath addDiscManip (const MString &manipName, const MString &angleName)`  
  This method creates a DiscManip and adds it to the MPxManipContainer container.
- `MDagPath addCircleSweepManip (const MString &manipName, const MString &angleName)`  
  This method creates a CircleSweepManip and adds it to the MPxManipContainer container.
- `MDagPath addToggleManip (const MString &manipName, const MString &toggleName)`  
  This method creates a ToggleManip and adds it to the MPxManipContainer container.
- `MDagPath addStateManip (const MString &manipName, const MString &stateName)`  
  This method creates a StateManip and adds it to the MPxManipContainer container.
- `MDagPath addCurveSegmentManip (const MString &manipName, const MString &startParamName, const MString &endParamName)`  
  This method creates a CurveSegmentManip and adds it to the MPxManipContainer container.
- `MDagPath addRotateManip (const MString &manipName, const MString &rotationName)`  
  This method creates a rotate manipulator and adds it to the MPxManipContainer container.
- `MDagPath addScaleManip (const MString &manipName, const MString &scaleName)`  
  This method creates a scale manipulator and adds it to the MPxManipContainer container.
- `MStatus addMPxManipulatorNode (const MString &manipTypeName, const MString &manipName, MPxManipulatorNode *&proxyManip)`  
  This method creates a custom MPxManipulatorNode and adds it to the MPxManipContainer container.
- `bool isManipActive (const MFn::Type &manipType, MObject &manipObject)`  
  This method determines if custom manip is active & gets the current manip object.
- `MStatus finishAddingManips ()`  
  This method should be called from the user-defined manipulator plug-in near the end of the connectToDependNode method so that the converter in the manipulator can be initialized.
- `virtual MManipData plugToManipConversion (unsigned int manipIndex)`  
  This virtual method calculates and returns the requested manipulator value, based upon the values of plugs on the nodes being manipulated.
- `virtual MManipData manipToPlugConversion (unsigned int manipIndex)`  
  This virtual method calculates and returns the requested plug value, based upon the container's manipulator values.
- `void addPlugToManipConversion (unsigned int manipIndex)`  
  This method adds a plug to manipulator converter for the specified manipulator value (e.g.
- `unsigned int addManipToPlugConversion (MPlug &plug)`  
  This method adds a manipulator to plug converter for the specified plug.
- `void addPlugToInViewEditor (const MPlug &plug)`  
  Adds a plug to the In-View Editor.
- `void addPlugToManipConversionCallback (unsigned int manipIndex, plugToManipConversionCallback callback)`  
  NO SCRIPT SUPPORT.
- `unsigned int addManipToPlugConversionCallback (MPlug &plug, manipToPlugConversionCallback callback)`  
  NO SCRIPT SUPPORT.
- `MStatus getConverterManipValue (unsigned int manipIndex, unsigned int &value)`  
  This method retrieves the value of a converterManipValue of type unsigned int at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, double &value)`  
  This method retrieves the value of a converterManipValue of type double at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, double &x, double &y)`  
  This method retrieves the x and y values of a converterManipValue of type double at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, MPoint &point)`  
  This method retrieves the value of a converterManipValue of type MPoint at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, MVector &vector)`  
  This method retrieves the value of a converterManipValue of type MVector at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, MMatrix &matrix)`  
  This method retrieves the value of a converterManipValue of type MMatrix at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, MEulerRotation &rotation)`  
  This method retrieves the value of a converterManipValue of type MEulerRotation at a given index from the converter.
- `MStatus getConverterManipValue (unsigned int manipIndex, MTransformationMatrix &xform)`  
  This method retrieves the value of a converterManipValue of type MTransformationMatrix at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, double &value)`  
  This method retrieves the value of a converterPlugValue of type double at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, double &x, double &y)`  
  This method retrieves the x and y values of a converterPlugValue of type double at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, MPoint &point)`  
  This method retrieves the value of a converterPlugValue of type MPoint at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, MVector &vector)`  
  This method retrieves the value of a converterPlugValue of type MVector at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, MMatrix &matrix)`  
  This method retrieves the value of a converterPlugValue of type MMatrix at a given index from the converter.
- `MStatus getConverterPlugValue (unsigned int plugIndex, MEulerRotation &rotation)`  
  This method retrieves the value of a converterPlugValue of type MEulerRotation at a given index from the converter.
- `virtual MStatus doPress ()`  
  This method gets called when the manipulator receives a mouse down event.
- `virtual MStatus doDrag ()`  
  This method gets called when the manipulator receives a mouse drag event.
- `virtual MStatus doRelease ()`  
  This method gets called when the manipulator receives a mouse release event.
- `MObject thisMObject () const override`  
  Returns the MObject associated with this user defined node.

**Static Public Member Functions**

- `static MStatus initialize ()`  
  This method initializes the manipulator, and should be overriden in user-defined manipulators.
- `static MPxManipContainer * newManipulator (const MString &manipName, MObject &manipObject, MStatus *ReturnStatus=NULL)`  
  This static function is used to create a user-defined manipulator.
- `static MStatus addToManipConnectTable (MTypeId &)`  
  This method adds the user defined node as an entry in the manipConnectTable so that when this node is selected the user can use the show manip tool to get the user defined manipulator associated with this node.
- `static MStatus removeFromManipConnectTable (MTypeId &)`  
  This method removes the user defined node entry from the manipConnectTable.
- `static const char * className ()`  
  Returns the name of this class.


### MPxManipulatorNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_manipulator_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_manipulator_node.html).

Base class for manipulator creation. MPxManipulatorNode is the base class used for creating user-defined manipulators. This class is derived from MPxNode since manipulators in Maya are dependency nodes. An MPxManipulatorNode should implement the following method: virtual void draw(M3dView &view, const MDagPath &path,M3dView::DisplayStyle style, M3dView::DisplayStatus status);

**Public Member Functions**

- `MPxManipulatorNode ()`  
  Class constructor.
- `~MPxManipulatorNode () override`  
  Class destructor.
- `virtual MStatus connectToDependNode (const MObject &dependNode)`  
  This method connects the manipulator to the dependency node.
- `virtual void preDrawUI (const M3dView &view)`  
  This method is not available in Python.
- `virtual void drawUI (MHWRender::MUIDrawManager &drawManager, const MHWRender::MFrameContext &frameContext) const`  
  This method is not available in Python.
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus status)`  
  This method is overloaded to draw the manipulators.
- `virtual MStatus doPress (M3dView &view)`  
  This method gets called when the manipulator receives a mouse down event.
- `virtual MStatus doDrag (M3dView &view)`  
  This method gets called when the manipulator receives a mouse drag event.
- `virtual MStatus doRelease (M3dView &view)`  
  This method gets called when the manipulator receives a mouse release event.
- `virtual MStatus doMove (M3dView &view, bool &refresh)`  
  This method gets called when the manipulator receives a mouse move event, if the manipulator registered for mouse move events.
- `MStatus finishAddingManips ()`  
  This method should be called from the user-defined manipulator plug-in near the end of the connectToDependNode method so that the converter in the manipulator can be initialized.
- `MStatus colorAndName (M3dView &view, MGLuint glName, bool glNameIsPickable, short colorIndex) const`  
  This method is used to set the color of the GL component that is being drawn next.
- `MStatus shouldDrawHandleAsSelected (unsigned int name, bool &useSelectedColor) const`  
  This method is obsolete.
- `MStatus setHandleColor (MHWRender::MUIDrawManager &drawManager, unsigned int handleName, short colorIndex) const`  
  This method is used to set the color of component that is being drawn next.
- `MStatus glFirstHandle (MGLuint &firstHandle)`  
  This method is used to find the unsigned int value that should be used for the first GL handle.
- `MStatus glActiveName (MGLuint &glName)`  
  This method returns the unsigned int value which specifies the current active handle.
- `MStatus mouseRay (MPoint &linePoint, MVector &lineDirection) const`  
  This method returns the location of the mouse within a view.
- `MStatus mouseRayWorld (MPoint &linePoint, MVector &lineDirection) const`  
  This method returns the location of the mouse within a view.
- `MStatus mousePosition (short &x_pos, short &y_pos)`  
  This method returns the current mouse position within a view.
- `MStatus mouseDown (short &x_pos, short &y_pos)`  
  This method returns the mouse down position within a view.
- `MStatus mouseUp (short &x_pos, short &y_pos)`  
  This method returns the mouse up position within a view.
- `MStatus registerForMouseMove ()`  
  This method registers this manipulator to receive mouse move events.
- `MStatus deregisterForMouseMove ()`  
  This method deregisters this manipulator from receiving mouse move events.
- `MStatus addDoubleValue (const MString &valueName, double defaultValue, int &valueIndex)`  
  Manipulators which call connectPlugToValue() must first create the value on the node.
- `MStatus addPointValue (const MString &valueName, const MPoint &defaultValue, int &valueIndex)`  
  Manipulators which call connectPlugToValue() must first create the value on the node.
- `MStatus addVectorValue (const MString &valueName, const MVector &defaultValue, int &valueIndex)`  
  Manipulators which call connectPlugToValue() must first create the value on the node.
- `MStatus setDoubleValue (int valueIndex, double value)`  
  This method is used for setting a double value associated with the manipulator.
- `MStatus setPointValue (int valueIndex, const MPoint &value)`  
  This method is used for setting an MPoint value associated with the manipulator.
- `MStatus setVectorValue (int valueIndex, const MVector &value)`  
  This method is used for setting a MVector value associated with the manipulator.
- `MStatus getDoubleValue (int valueIndex, bool previousValue, double &value)`  
  This method is used for getting a double value associated with the manipulator.
- `MStatus getPointValue (int valueIndex, bool previousValue, MPoint &value)`  
  This method is used for getting a MPoint value associated with the manipulator.
- `MStatus getVectorValue (int valueIndex, bool previousValue, MVector &value)`  
  This method is used for getting a MVector value associated with the manipulator.
- `MStatus connectPlugToValue (const MPlug &plug, int valueIndex, int &plugIndex)`  
  This method is called in the connectToDependNode() virtual if it is implemented for the custom manipulator.
- `MStatus addDependentPlug (const MPlug &plug)`  
  This method adds the plug to the list of those to be keyframed.
- `MStatus dependentPlugsReset ()`  
  This method resets the list of dependent plugs for this manipulator.
- `short mainColor () const`  
  This method returns the main color index.
- `short xColor () const`  
  This method returns the color index of the x axis.
- `short yColor () const`  
  This method returns the color index of the y axis.
- `short zColor () const`  
  This method returns the color index of the z axis.
- `short prevColor () const`  
  This method returns the previously color used by the colorAndName() method.
- `short lineColor () const`  
  This method returns the color index of a line.
- `short dimmedColor () const`  
  This method returns the color index for a dimmed or unselectable component.
- `short selectedColor () const`  
  This method returns the color index of a selected component.
- `short labelColor () const`  
  This method returns the color index of a label.
- `short labelBackgroundColor () const`  
  This method returns the color index of a label background.

**Static Public Member Functions**

- `static MPxManipulatorNode * newManipulator (const MString &manipName, MObject &manipObject, MStatus *ReturnStatus=NULL)`  
  This static function is used to create a user-defined manipulator.
- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject connectedNodes`  
  Connected node, message attribute.


### MPxMidiInputDevice

Module `OpenMayaUI`. Source: [class_m_px_midi_input_device.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_midi_input_device.html).

Midi input device. This is the base class for user defined MIDI input devices. Child classes of MPxMidiInputDevice should define: a constructor which defines number of buttons and axes nameAttributes() which names the buttons and axes (optional) deviceState() which turns a midi event into an MDeviceState or returns NULL if it is not a midi event from this device

**Public Member Functions**

- `virtual MStatus openDevice ()`  
  Open the midi device.
- `virtual void closeDevice ()`  
  Close the midi device.
- `virtual void nameAxes ()`  
  Assign names to the axes of the device.
- `virtual void nameButtons ()`  
  Assign names to the buttons of the device.
- `virtual MDeviceState * deviceState ()`  
  Return the current state of the input device.
- `virtual MStatus sendMessage (const char *const messageType, const char *const messageParams)`  
  If this midi event belongs to this device then fiil up the MDeviceState.
- `virtual char * getMessage (const char *const messageType, char *messageResponse)`  
  User should override this method.
- `virtual void doButtonEvents (bool=true)`  
  This method is used to specify whether this device is accepting button events from its child.
- `virtual void doMovementEvents (bool=true)`  
  This method is used to specify whether this device is accepting movement input from its child.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus setNamedButton (const MString &, unsigned short)`  
  Set the name of the specified button.
- `MStatus addChannel (MDeviceChannel &)`  
  Add the given channel to this device.
- `MStatus setDegreesOfFreedom (int freedom)`  
  Set the degrees of freedom for this device.
- `MStatus setNumberOfButtons (int buttons)`  
  Set the number of buttons for this device.


### MPxModelEditorCommand

Module `OpenMayaUI`. Source: [class_m_px_model_editor_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_model_editor_command.html).

Base class for editor creation commands. MPxModelEditorCommand is the base class for user defined model editor commands. This command gives all of the flags and options of the modelEditor command in addition to any user defined flags or behaviours. When registering this command, use the MFnPlugin::MFnPlugin::registerModelEditorCommand() method. A MPx3dModelView is also required to be used with MPxModelEditorCommand and is specified when the MPxModelEditorCommand is registered. This method is obsolete. [as of Maya 2022] Deprecated:Use MHWRender::MRenderOverride instead.

**Public Member Functions**

- `MPxModelEditorCommand ()`  
  Class constructor.
- `virtual ~MPxModelEditorCommand ()`  
  Class destructor.
- `virtual MStatus doEditFlags ()`  
  This method is called when the command is called in edit mode.
- `virtual MStatus doQueryFlags ()`  
  This method is called when the command is called in query mode.
- `virtual MStatus appendSyntax ()`  
  This method should be overridden to append syntax to the panel command.
- `virtual bool skipFlagForCreate (const MString &longFlag)`  
  Returns true if the passed long flag name should be skipped during the creation portion of the command.
- `MStatus setResult (bool result)`  
  This method should be called when the result of the panel command is a boolean.
- `MStatus setResult (int result)`  
  This method should be called when the result of the panel command is an integer.
- `MStatus setResult (double result)`  
  This method should be called when the result of the panel command is a double.
- `MStatus setResult (const MString &result)`  
  This method should be called when the result of the panel command is a string.
- `MStatus setResult (const MStringArray &result)`  
  This method should be called when the result of the panel command is a string array.
- `MStatus setResult (const MDoubleArray &result)`  
  This method should be called when the result of the panel command is a double array.
- `MStatus setResult (const MIntArray &result)`  
  This method should be called when the result of the panel command is an integer array.
- `virtual MString editorCommandName () const`  
  Returns the name of editor command.
- `virtual MString editorMenuScriptName () const`  
  Returns the name of the script that should get executed to construct the menu for the editor.
- `virtual MPx3dModelView * makeModelView (MStatus *ReturnStatus=NULL)`  
  This method is called when the modelEditor is being created and it can be overriden to allow the user to handle the allocation of the modelView directly.
- `MPx3dModelView * modelView (MStatus *ReturnStatus=NULL) const`  
  Returns a pointer to the MPx3dModelView created by this command.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MSyntax syntax (MStatus *ReturnStatus=NULL) const`  
  USE _syntax() IN SCRIPT.
- `MArgParser parser (MStatus *ReturnStatus=NULL) const`  
  USE _parser() IN SCRIPT.


### MPxPolyTweakUVInteractiveCommand

Module `OpenMayaUI`. Inherits `MPxToolCommand`. Source: [class_m_px_poly_tweak_u_v_interactive_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_poly_tweak_u_v_interactive_command.html).

Base class used for moving polygon UV's. This is the base class for UV editing interactive commands on polygonal objects. The purpose of this tool command class is to simplify the process of moving UVs on a polygonal object. The use is only required to provide the new positions of the UVs that being modified, and finalize at the end of editing.

**Public Member Functions**

- `MPxPolyTweakUVInteractiveCommand ()`  
  Class constructor.
- `~MPxPolyTweakUVInteractiveCommand () override`  
  Class destructor.
- `void setUVs (const MObject &mesh, MIntArray &uvList, MFloatArray &uPos, MFloatArray &vPos, const MString *uvSet=NULL)`  
  This method sets the uv to new values.
- `bool isUndoable () const override`  
  This method is used to specify whether or not the command is undoable.
- `MStatus doIt (const MArgList &args) override`  
  This method should perform a command by setting up internal class data and then calling the redoIt method.
- `MStatus cancel () override`  
  This method cancels the command.
- `MStatus finalize () override`  
  This method finishs the editing of uv.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxSelectionContext

Module `OpenMayaUI`. Inherits `MPxContext`. Source: [class_m_px_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_selection_context.html).

Base class for interative selection tools. This class is used in creating user defined tools that use the internal selection mechanism in maya.

**Public Member Functions**

- `MPxSelectionContext ()`  
  Class contstructor.
- `~MPxSelectionContext () override`  
  Default destructor.
- `MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doHold (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doPress (MEvent &event) override`  
  This method is called when any mouse button is pressed.
- `MStatus doRelease (MEvent &event) override`  
  This method is called when any mouse button is released.
- `MStatus doDrag (MEvent &event) override`  
  This method is called when a mouse drag event occurs.
- `MStatus doHold (MEvent &event) override`  
  This method is called when after a mouse button is pressed but before the mouse is dragged.
- `MStatus helpStateHasChanged (MEvent &) override`  
  This method is called whenever the help state may need to be updated.
- `MStatus addManipulator (const MObject &manipulator) override`  
  This method adds a manipulator to the context, and also adds the manipulator to the DAG.
- `MStatus deleteManipulators () override`  
  This method deletes all the manipulators that belong to the context.
- `MStatus setAllowPreSelectHilight ()`  
  This method enables the support of pre-selection highlight for this context.
- `MStatus setAllowSoftSelect ()`  
  This method enables the support of soft selection for this context.
- `MStatus setAllowSymmetry ()`  
  This method enables the support of symmetrical selection for this context.
- `MStatus setAllowDoubleClickAction ()`  
  This method enables the support of double click smart selection for this context.
- `MStatus setImage (const MString &image, ImageIndex index)`  
  This method is used to set an XPM icon image that is to be used to represent this tool context in various places including the tool bar and can be queried from mel using the contextInfo command.
- `MString image (ImageIndex index, MStatus *ReturnStatus=NULL) const`  
  This method is used to retrieve an XPM icon image that has previously been set for this tool context.
- `MStatus getImage (MString &image, ImageIndex index) const`  
  NO SCRIPT SUPPORT.
- `void abortAction () override`  
  This method is called when the abort key is pressed.
- `bool processNumericalInput (const MDoubleArray &values, const MIntArray &flags, bool isAbsolute) override`  
  This method processes the input from the numerical input field.
- `bool feedbackNumericalInput () const override`  
  This method is called to update the numerical feedback.
- `MSyntax::MArgType argTypeNumericalInput (unsigned int index) const override`  
  This method is used by the feedback line to determine what units to display.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `bool isSelecting ()`  
  USE _isSelecting() IN SCRIPT.
- `MPoint startPoint ()`  
  USE _startPoint() IN SCRIPT.
- `MPoint lastDragPoint ()`  
  USE _lastDragPoint() IN SCRIPT.
- `MPxToolCommand * newToolCommand () override`  
  CALL _newToolCommand() IN SCRIPT.


### MPxSurfaceShapeUI

Module `OpenMayaUI`. Source: [class_m_px_surface_shape_u_i.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_surface_shape_u_i.html).

drawing and selection for user defined shapes The base class for the UI portion of all user defined shapes.

**Public Types**

- `enum UVSelectionType { kSelectMeshUVs, kSelectMeshVerts, kSelectMeshFaces, kSelectMeshEdges }`  
  Selection modes for UVs.

**Public Member Functions**

- `MPxSurfaceShapeUI ()`  
  Class constructor.
- `virtual ~MPxSurfaceShapeUI ()`  
  Class destructor.
- `void getDrawData (const void *geom, MDrawData &)`  
  This method is obsolete.
- `virtual void getDrawRequests (const MDrawInfo &, bool objectAndActiveOnly, MDrawRequestQueue &requests)`  
  This method is obsolete.
- `virtual void draw (const MDrawRequest &, M3dView &view) const`  
  This method is obsolete.
- `virtual bool select (MSelectInfo &selectInfo, MSelectionList &selectionList, MPointArray &worldSpaceSelectPts) const`  
  This routine must be overriden if the shape is to support interactive object and/or component selection.
- `virtual bool snap (MSelectInfo &snapInfo) const`  
  Maya calls this method when snapping to the shape's vertices.
- `virtual bool canDrawUV () const`  
  Called by Maya to determine if this surface shape supports UV drawing.
- `virtual void drawUV (M3dView &view, const MTextureEditorDrawInfo &info) const`  
  This method is called when the surface shape is selected and the texture view is open.
- `virtual bool selectUV (M3dView &view, UVSelectionType selType, int xmin, int ymin, int xmax, int ymax, bool singleSelect, MSelectionList &selList) const`  
  This method is called when the user performs a selection within the texture view.
- `MPxSurfaceShape * surfaceShape () const`  
  Returns the non-ui shape associated with current instance of this class.
- `MMaterial material (MDagPath &path) const`  
  Returns the material associated with this shape.
- `void materials (MDagPath &path, MObjectArray &componentFilter, MMaterialArray &materials, MObjectArray *componentSet=NULL) const`  
  Returns a list of materials associated with this shape and a given list of components.

**Static Public Member Functions**

- `static MPxSurfaceShapeUI * surfaceShapeUI (const MDagPath &path, MStatus *ReturnStatus=NULL)`  
  This is a static method that can be used to find the corresponding MPxSurfaceShapeUI for the specified path.
- `static const char * className ()`  
  Returns the name of this class.


### MPxTexContext

Module `OpenMayaUI`. Inherits `MPxContext`. Source: [class_m_px_tex_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_tex_context.html).

Base class for user defined contexts working on uv editor. This is the base class for user defined contexts working on uv editor.

**Public Member Functions**

- `MPxTexContext ()`  
  Class contstructor.
- `~MPxTexContext () override`  
  Default destructor.
- `MStatus doPress (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doRelease (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doDrag (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `MStatus doPtrMoved (MEvent &event, MHWRender::MUIDrawManager &drawMgr, const MHWRender::MFrameContext &context) override`  
  This method is not available in Python.
- `void viewToPort (double xView, double yView, short &xPort, short &yPort) const`  
  This method is used to convert view coordinates to port (window) coordinates.
- `void portToView (short xPort, short yPort, double &xView, double &yView) const`  
  This method is used to convert port (window) coordinates to view coordinates.
- `void viewRect (double &left, double &right, double &bottom, double &top) const`  
  This method is used to get the current uv viewport dimensions.
- `void portSize (double &width, double &height) const`  
  This method is used to get the window dimension of the current uv viewport.
- `MPxToolCommand * newToolCommand () override`  
  Create a new instance of the tool command associated with this context.

**Static Public Member Functions**

- `static bool getMarqueeSelection (double xMin, double yMin, double xMax, double yMax, const MSelectionMask &mask, bool bPickSingle, bool bIgnoreSelectionMode, MSelectionList &selectionList)`  
  This method is called when the user performs a selection within the uv editor.
- `static const char * className ()`  
  Returns the name of this class.


### MPxThreadedDeviceNode

Module `OpenMayaUI`. Inherits `MPxNode`. Source: [class_m_px_threaded_device_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_threaded_device_node.html).

Base class for threaded node creation. MPxThreadedDeviceNode is the base class used for creating threaded maya device nodes. Maya handles thread management. The plug-in needs to implement the threadHandler() and threadShutdownHandler(). Maya will call these handlers as it manages the thread. The interface implements a memory queue system where data pointers are acquired, filled, then pushed into the node using mutex locking. This is done from a separate, secondary thread. When the data has been acquired, the thread will mark the output attribute as dirty so that the compute in the main thread will be called. The main thread will pop the oldest data buffer, process it, then release the thread data. The helper class MCharBuffer is used with the memory methods. The "live" attribute allows finer control over the secondary thread. It can be used to stop the handling of incoming data without disconnecting the output attributes. Note that the thread continues to run even if the the live attribute is set to false. To implement an MPxThreadedDeviceNode plug-in, you must: 1. Use the post constructor to call setRefreshOutputAttributes() and createMemoryPools() 2. Implement the destructor to destroy the memory pools 3. Implement the threadHandler() virtual. Maya will call this handler when it creates the thread. Typically the thread handler will implement a loop for collecting data. This class contains a "done" state which is accessible via the isDone() and setDone() methods. Loop while isDone() is false 4. Implement the threadShutdownHandler(). This method would be called by Maya on File -> New and Exit for example. Call setDone( true ) to stop the isDone() loop implemented in the threadHandler if the plug-in uses the done state 5. The secondary thread should call beginThreadLoop() and endThreadLoop() with the data operations happening in-between. The endThreadLoop() method will perform the throttling based on the frame rate attribute and additionally cause the output attribute to be marked dirty. This in turn will cause compute() to be called in the main thread once Maya processes the dirty message 6. The thread loop should acquireDataStorage() to get a buffer, populate the buffer with information and then call pushThreadData() 8. The compute method should popThreadData() and if it's valid, process the data and release the data pointer by calling releaseDataStorage(). The live and frameRate attributes are included in this class. Changes to these attributes will cause the thread to be shutdown and restarted so that the new settings can be in effect. If this node is deleted then the thread will automatically be shutdown when the node is placed on the undo queue. Undoing the delete will restart the thread if it was running at the time of the delete. NOTE: This class relies on the idle event queue to perform the node attribute updating. As a result, it will not work in Maya batch mode.

**Public Member Functions**

- `MPxThreadedDeviceNode ()`  
  Constructor.
- `~MPxThreadedDeviceNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  Returns the type of node that this is.
- `virtual void threadHandler ()`  
  This method should be overridden in user defined nodes.
- `virtual void threadShutdownHandler ()`  
  This method should be overridden in user defined nodes.
- `bool isLive ()`  
  Returns the true if the node is live.
- `bool isDone ()`  
  Returns the true if the node has finished running.
- `MStatus setDone (bool done)`  
  This method is used to set done state of the thread.
- `MStatus beginThreadLoop ()`  
  This method should be called at the beginning of the loop in the secondary thread.
- `MStatus endThreadLoop ()`  
  This method should be called at the end of the loop in the secondary thread.
- `MStatus createMemoryPools (int nCount, int nElements, long nSize)`  
  This method should be called from the post constructor to create the memory buffers required.
- `MStatus destroyMemoryPools ()`  
  This method should be called to destroy the memory pools.
- `MStatus acquireDataStorage (MCharBuffer &buffer)`  
  This method is used to acquire a pointer to a block of storage that was allocated by the createMemoryPools() method.
- `MStatus pushThreadData (MCharBuffer &buffer)`  
  This method is used to push the block of storage onto the internal queue.
- `MStatus popThreadData (MCharBuffer &buffer)`  
  This method is used to pop the block of storage from the internal queue.
- `long threadDataCount ()`  
  This method returns the number of pieces of thread data that are currently in the node's queue.
- `MStatus releaseDataStorage (MCharBuffer &buffer)`  
  After popThreadData() is called, the pointer should be released so that it can be used again.
- `MStatus setRefreshOutputAttributes (const MObjectArray &attributeList)`  
  This method is used to set the list of output attributes which will be marked dirty by the secondary thread.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject output`  
  Output data, generic attribute.
- `static MObject live`  
  Makes node live, boolean attribute.
- `static MObject frameRate`  
  Capture frame rate, double attribute.


### MPxToolCommand

Module `OpenMayaUI`. Inherits `MPxCommand`. Source: [class_m_px_tool_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_tool_command.html).

Base class for interactive tool commands. This is the base class for interactive tool commands. An interactive tool command is a command that can be invoked as a MEL command or from within a user defined context (see MPxContext). Tool commands have the same functionality as MPxCommands, but include several additional methods for use in interactive contexts: setUpInteractive, cancel, finalize, and doFinalize.

**Public Member Functions**

- `MPxToolCommand ()`  
  Class constructor.
- `~MPxToolCommand () override`  
  Class Destructor.
- `MStatus doIt (const MArgList &args) override=0`  
  This method should perform a command by setting up internal class data and then calling the redoIt method.
- `virtual MStatus cancel ()`  
  This method cancels the command.
- `virtual MStatus finalize ()`  
  This method is used to create a string representing the command and its arguments.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus doFinalize (MArgList &command)`  
  USE _doFinalize() IN SCRIPT.


### MPxUIControl

Module `OpenMayaUI`. Source: [class_m_px_u_i_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_u_i_control.html).

Base class for control creation. MPxUIControl is the base class for user defined UI. This class should never be extended. Extend the derived classes of this class.

**Public Member Functions**

- `virtual ~MPxUIControl ()`  
  Class destructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxUITableControl

Module `OpenMayaUI`. Inherits `MPxUIControl`. Source: [class_m_px_u_i_table_control.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_u_i_table_control.html).

Base class for creating new spreadsheet controls. MPxUITableControl is the base class for creating new spreadsheet like interfaces. Every UI control has an associated command that is 'in-charge' of that control. Therefore, when creating a new table control, you will need to create an associated MPxControlCommand. Please refer to the MPxControlCommand documentation for details on how to use these class together. At a minimum, when creating a new table control, you must overload getCell( ... ). Other methods available for overloading are getLabel(...), allowEdit( ... ), allowSelection( ... ), and collapseOrExpandRow( ... ).

**Public Types**

- `enum MLabelType { kNoLabel, kRowLabel, kColumnLabel, kAllLabels }`  
  Types of labels available in the control.

**Public Member Functions**

- `MPxUITableControl (MPxControlCommand &)`  
  Class constructor.
- `~MPxUITableControl () override`  
  Class destructor.
- `MStatus redrawLabels (MLabelType lt=kAllLabels)`  
  Tells the table control to redraw the label entries.
- `MStatus redrawCells ()`  
  Tells the UI control to redraw the cells of the table.
- `MStatus addToSelection (unsigned int row, unsigned int col)`  
  Add the follow cell to the selection data structure.
- `MStatus removeFromSelection (unsigned int row, unsigned int column)`  
  Removes the specified cell entry from the internal selection list.
- `MStatus clearSelection ()`  
  Clears the selection table.
- `MStatus setSelection (unsigned int row, unsigned int column)`  
  Sets the table selection list based on the row & column entry.
- `MStatus setSelection (unsigned int firstRow, unsigned int lastRow, unsigned int firstCol, unsigned int lastCol)`  
  Sets the selection to be the range of cells specified.
- `MStatus setNumberOfRows (unsigned int count)`  
  Specifies the number of rows for the table.
- `unsigned int numberOfRows (MStatus *ReturnStatus=nullptr)`  
  Returns the number of rows in the table.
- `MStatus setNumberOfColumns (unsigned int count)`  
  Specifies the number of columns for the table.
- `unsigned int numberOfColumns (MStatus *ReturnStatus=nullptr) const`  
  Returns the number of columns in the table.
- `bool suspendUpdates (bool update, MStatus *ReturnStatue=nullptr)`  
  When set to true, it prevents the UI from updating the table.
- `bool isSelected (unsigned int row, unsigned int col, MStatus *status=nullptr)`  
  Returns true if the specified cell is selected.
- `virtual bool collapseOrExpandRow (unsigned int row)`  
  This is called when a row should be collapsed or expanded (depending on its current state).

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `virtual MString cellString (unsigned int r, unsigned int c, bool &ValidCell)`  
  Returns the contents of a cell.
- `virtual MString labelString (MLabelType labelType, unsigned int n)`  
  Returns the text of the label for the given row or column.
- `virtual bool getCell (unsigned int r, unsigned int c, MString &value)`  
  NO SCRIPT SUPPORT.
- `virtual void getLabel (MLabelType labelType, unsigned int n, MString &value)`  
  NO SCRIPT SUPPORT.
- `virtual bool allowEdit () const`  
  Tells the base UI table control class if this control is editable.
- `virtual bool allowSelection (int top, int left, int bottom, int right)`  
  This method is called when a selection occurs.
- `virtual bool getCellColor (unsigned int r, unsigned int c, int &red, int &green, int &blue)`  
  Returns the background color of a cell.


### MQtUtil

Module `OpenMayaUI`. Source: [class_m_qt_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_qt_util.html).

Qt Utility Methods. This class provides some basic methods for accessing the Qt controls which underlie Maya's UI. The safest way to use the Qt API from within Maya is to create your own Qt window and populate it with your own controls. While it is possible to use the Qt API to modify existing Maya UI elements, such as those created using Maya commands from MEL or Python, such modifications are not supported by Autodesk and could lead to Maya becoming unstable or inoperable. In practice, you will likely find a number of modifications which appear safe, such as changing a control's text or reorganizing the items on a menu. However, Autodesk provides no guarantees that the underlying implementations of those UI elements won't change from one release of Maya to another, potentially in ways that may make formerly safe usage become unsafe. So if you choose to incorporate such actions into your plug-in be aware that you may have to modify your code in future versions of Maya. Layouts In Maya Vs. Qt In Qt a layout (i.e. a QLayout or something derived from it) is not a widget. It is attached to a widget to control the positioning of that widget's children but is not itself a child of the widget and does not appear in the widget hierarchy. In Maya, layouts are also widgets which are part of the hierarchy. For example, if you have a window named "myWin" which contains a layout named "myLayout" which in turn holds a button named "myButton", then the button's full hierarchical name would be "myWin|myLayout|myButton". To bridge this difference, each Maya layout is given its own placeholder QWidget to which the corresponding QLayout is applied and bearing the same name as the layout. The children of the layout are then parented beneath the placeholder widget. When you call the findLayout() method, it is the placeholder widget which is returned. The one exception to this use of placeholders is the top-level layout of a window. Rather than introduce a superfluous placeholder widget, the top-level QLayout is attached directly to the window. So a call to findLayout() for a top-level layout will return the window. Note, however, that the top-level layout's hierarchical name is still structured as if it had its own dedicated widget in the hierarchy. E.g. "myWin|myLayout". This can lead to the somewhat incongruous result that calling fullName() on the result of a findLayout() call for a top-level layout will return the name of the window rather than the layout. E.g: QWidget* layoutWidget = MQtUtil::findLayout("myWin|myLayout");

**Public Types**

- `typedef QWidget *(* UITypeCreatorFn) (QWidget *parent, const QString &name)`  
  Pointer to a widget creator function.

**Static Public Member Functions**

- `static QWidget * findControl (MString controlName, const QWidget *ancestor=NULL)`  
  Returns the QWidget for the named Maya control.
- `static QWidget * findLayout (MString layoutName, const QWidget *ancestor=NULL)`  
  Returns the QWidget for the named Maya layout.
- `static QAction * findMenuItem (MString itemName)`  
  Returns the QAction for the named Maya menuItem.
- `static QWidget * findWindow (MString windowName)`  
  Returns the QWidget for the named Maya window.
- `static MString fullName (const QObject *uiElement)`  
  Returns the full, hierarchical name of a UI element.
- `static QWidget * getCurrentParent ()`  
  Returns the placeholder widget for the current layout if there is one.
- `static QPixmap * createPixmap (const MString &imageName, bool autoScale=true)`  
  Looks for a pixmap first in the Qt resources, and then in a file on disk.
- `static QCursor * createCursor (const MString &cursorName, bool autoScale=true)`  
  Looks for a cursor first in the Qt resources, and then in a file on disk.
- `static QIcon * createIcon (const MString &iconName, bool autoScale=true)`  
  Looks for a icon first in the Qt resources, and then in a file on disk.
- `static QList< QObject * > getLayoutChildren (const QWidget *layout)`  
  Returns a list of all the Maya UI elements parented directly beneath the specified Maya layout.
- `static QObject * getParent (const QObject *uiElement)`  
  Returns a pointer to a UI element's parent element within Maya's UI hierarchy.
- `static MString addWidgetToMayaLayout (QWidget *control, QWidget *layout, const MString &uiType=MString())`  
  Adds a QWidget to an existing Maya layout, such as that returned by getCurrentParent() or getLayout().
- `static bool registerUIType (const MString &className, UITypeCreatorFn, const MString &command)`  
  Registers a class name (UI-type) with a function to create the QWidget of that type, and a command to edit it with.
- `static bool deregisterUIType (const MString &className)`  
  De-registers a widget class (UI-type) which has been previously registered with registerUIType().
- `static QWidget * mainWindow ()`  
  Returns Maya's main window.
- `static QGLContext * resourceGLContext ()`  
  Returns Maya internal QGLContext that the plug-in may use to create resource-sharing OpenGL context.
- `static MNativeWindowHdl nativeWindow (const QWidget *control)`  
  /*! Returns a platform-specific native window handle for the specified control.
- `static MString toMString (const QString &qstr)`  
  Convenience utility to convert a QString to an MString.
- `static QString toQString (const MString &mstr)`  
  Convenience utility to convert an MString to a QString.
- `static int dpiScale (int size)`  
  Get the scaled size for Maya interface scaling.
- `static float dpiScale (float size)`  
  Get the scaled size for Maya interface scaling.
- `static QDoubleValidator * newClocaleValidator (double bottom, double top, int decimals, QObject *parent=nullptr)`  
  Creates a overloaded QDoubleValidator instance that will automatically convert decimal separator to dot to facilidate interfacing with Maya.
- `static const char * className ()`  
  Returns the name of this class.


### MSelectInfo

Module `OpenMayaUI`. Inherits `MDrawInfo`. Source: [class_m_select_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_select_info.html).

Selection state information used in MPxSurfaceShapeUI::select. MSelectInfo is used with user defined shape selection and is passed as an argument to the MPxSurfaceShapeUI::select method. This class encapsulates all the selection state information for selected objects.

**Public Member Functions**

- `MSelectInfo ()`  
  Constructor.
- `MSelectInfo (const MSelectInfo &in)`  
  Copy constructor.
- `~MSelectInfo () override`  
  Destructor.
- `M3dView view ()`  
  Returns the view that the current selection is taking place in.
- `bool singleSelection () const`  
  This method determines if we want to select a single object.
- `bool selectClosest () const`  
  Determines if we want to select the closest object.
- `bool selectable (MSelectionMask &mask) const`  
  Given the selection mask, this method determines if the object is selectable.
- `bool selectableComponent (bool displayed, MSelectionMask &mask) const`  
  Given the selection mask, this method determines if the component is selectable.
- `void selectRect (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`  
  Get the current selection rectangle dimensions.
- `bool isRay () const`  
  Is there a selection ray.
- `MMatrix getAlignmentMatrix () const`  
  Returns the alignment matrix.
- `void getLocalRay (MPoint &, MVector &) const`  
  Get the selection ray.
- `bool selectForHilite (const MSelectionMask &) const`  
  Give the selection mask, can this object be selected for the hilite list.
- `bool selectOnHilitedOnly () const`  
  This means that you can only select components if the object is hilited.
- `int highestPriority () const`  
  Returns the highest selection priority value.
- `void setHighestPriority (int)`  
  Sets the highest selection priority value.
- `void addSelection (const MSelectionList &item, const MPoint &point, MSelectionList &list, MPointArray &points, const MSelectionMask &mask, bool isComponent)`  
  Adds components or objects to the active selection list.
- `bool setSnapPoint (const MPoint &point)`  
  When a snapping operation is being performed the shape's overridden MPxSurfaceShapeUI::snap() method can call this method to set the point to be snapped to.
- `MDagPath selectPath () const`  
  Returns a path to the item that is being selected.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MTemplateContextCommand&lt; ContextCommandName, ContextCommandClass, ContextClass &gt;

Module `OpenMayaUI`. Inherits `MPxContextCommand`. Source: [class_m_template_context_command.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_context_command.html).

template<const char * ContextCommandName, class ContextCommandClass, class ContextClass> class MTemplateContextCommand< ContextCommandName, ContextCommandClass, ContextClass > Template class for context commands. Template helper to build command that is used to create context. Parameters [in]ContextCommandName: string name of context command [in]ContextCommandClass: context command derived from MPxContextCommand [in]ContextClass: context derived from MPxSelectionContext

**Public Member Functions**

- `MTemplateContextCommand ()`  
  Constructor.
- `MPxContext * makeObj () override`  
  Command function for make the context.

**Static Public Member Functions**

- `static void * creator ()`  
  Creator of the context command.
- `static MStatus registerContextCommand (MObject obj)`  
  Plug-in registration method.
- `static MStatus deregisterContextCommand (MObject obj)`  
  Plug-in deregistration method.


### MTemplateSelectionContext&lt; ContextNameString, ContextClass, NodeType, ManipulatorClass, ManipulatorNodeName &gt;

Module `OpenMayaUI`. Inherits `MPxSelectionContext`. Source: [class_m_template_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_template_selection_context.html).

template<const char * ContextNameString, class ContextClass, MFn::Type NodeType, class ManipulatorClass, const char * ManipulatorNodeName> class MTemplateSelectionContext< ContextNameString, ContextClass, NodeType, ManipulatorClass, ManipulatorNodeName > Template class for selection context. Template helper to build selection contexts. Parameters: ContextNameString : string name of context command ContextClass : context command derived from MPxContextCommand NodeType : MFn type that selection context should operate on ManipulatorClass : derived from MPxManipulatorNode ManipulatorNodeName : name of the manipulator node

**Public Member Functions**

- `MTemplateSelectionContext ()`  
  Constructor.
- `~MTemplateSelectionContext () override`  
  Destructor.
- `void toolOnSetup (MEvent &event) override`  
  Sets up a active list modified message which will trigger manipulator create/delete operations.
- `void toolOffCleanup () override`  
  Removes the callback.
- `virtual void namesOfAttributes (MStringArray &namesOfAttributes)`  
  Override to specify which attributes are required on the selected geometry.
- `virtual void setInitialState ()`  
  Override to set manipulator initial state.
- `bool validGeometrySelected ()`  
  Ensure that valid geometry is selected.

**Static Public Member Functions**

- `static void updateManipulators (void *data)`  
  Callback that creates the manipulator if valid geometry is selected.


### MTextureEditorDrawInfo

Module `OpenMayaUI`. Source: [class_m_texture_editor_draw_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_texture_editor_draw_info.html).

Drawing state for drawing to the UV texture window with custom shapes. This class is used by drawUV method of MPxSurfaceShapeUI to specify the current UV drawing state for a user defined shape. API users must override the canDrawUV method on MPxSurfaceShapeUI to recieve drawUV calls. The only situation where the drawing style can change is during a selection event. However, selection events are currently not passed onto the API user. Therefore, most of the functionality in this class is place holder for future work. See alsoMPxSurfaceShapeUI

**Public Types**

- `enum DrawingFunction { kDrawFunctionFirst = 1, kDrawWireframe = kDrawFunctionFirst, kDrawEverything, kDrawVertexForSelect, kDrawEdgeForSelect, kDrawFacetForSelect, kDrawUVForSelect, kDrawFunctionLast = kDrawUVForSelect }`  
  Draw modes.

**Public Member Functions**

- `MTextureEditorDrawInfo ()`  
  Constructor.
- `MTextureEditorDrawInfo (const MTextureEditorDrawInfo &in)`  
  Copy constructor.
- `virtual ~MTextureEditorDrawInfo ()`  
  Destructor.
- `DrawingFunction drawingFunction () const`  
  Indicates the current drawing state for a drawUV method call.
- `void setDrawingFunction (DrawingFunction func)`  
  Sets the current drawing state.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MToolsInfo

Module `OpenMayaUI`. Source: [class_m_tools_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_tools_info.html).

Tool information. MToolsInfo is a caretaker class used to keep track of the state of the current tool property sheet. The tool writer should make sure to call the setDirtyFlag method when any of the values are modified. The dirty flag is used to indicate that the UI needs to be updated when the value of a tool property sheet option has changed.

**Static Public Member Functions**

- `static void setDirtyFlag (const MPxContext &context)`  
  This method should be called by a tool when the value of a tool property sheet option has changed.
- `static void resetDirtyFlag ()`  
  This method resets the dirty flag, such that the state becomes clean.
- `static bool isDirty ()`  
  This method returns whether or not the dirty flag is set.
- `static const char * className ()`  
  Returns the name of this class.


### MUiMessage

Module `OpenMayaUI`. Inherits `MMessage`. Source: [class_m_ui_message.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_ui_message.html).

UI messages. This class is used to register callbacks to track the deletion of UI objects. The first parameter passed to the add callback method is the name of the UI that will trigger the callback. The method returns an id which is used to remove the callback. To remove a callback use MMessage::removeCallback. All callbacks that are registered by a plug-in must be removed by that plug-in when it is unloaded. Failure to do so will result in a fatal error.

**Static Public Member Functions**

- `static MCallbackId addUiDeletedCallback (const MString &uiName, MMessage::MBasicFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for UI deleted messages.
- `static MCallbackId addCameraChangedCallback (const MString &panelName, MMessage::MStringNode func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for cameras being changed in 3d views.
- `static MCallbackId add3dViewDestroyMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when a particular 3d view gets destroyed.
- `static MCallbackId add3dViewPreRenderMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when a particular 3d view is about to render it's contents.
- `static MCallbackId add3dViewPostRenderMsgCallback (const MString &panelName, MMessage::MStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when the 3d view is about to display it's rendered contents to the viewport.
- `static MCallbackId add3dViewPreMultipleDrawPassMsgCallback (const MString &panelName, MUiMessage::MStringIndexFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when a particular 3d view's specific pass is about to be drawn when multiple drawing is enabled.
- `static MCallbackId add3dViewPostMultipleDrawPassMsgCallback (const MString &panelName, MUiMessage::MStringIndexFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when a particular 3d view's specified pass is finshed when multiple drawing is enabled.
- `static MCallbackId add3dViewRendererChangedCallback (const MString &panelName, MMessage::MThreeStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when the renderer for a particular 3d view changes.
- `static MCallbackId add3dViewRenderOverrideChangedCallback (const MString &panelName, MMessage::MThreeStringFunction func, void *clientData=NULL, MStatus *ReturnStatus=NULL)`  
  This method registers a callback for when the render override for a particular 3d view changes.
- `static const char * className ()`  
  Returns the name of this class.

