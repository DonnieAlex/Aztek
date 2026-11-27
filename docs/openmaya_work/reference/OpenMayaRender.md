# OpenMayaRender, API module for rendering

Transcribed from the reference's 2022 C++ API reference, module page https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_render.html.

This is a class and member transcription: names, inheritance, signatures and the one-line descriptions on the class pages. Parameter and return prose that only appears in the per-member detail sections is not reproduced; follow the source link on a class to read it.

Classes in this module: 120.

## Classes

### MAttributeParameterMapping

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_attribute_parameter_mapping.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_attribute_parameter_mapping.html).

Class for defining relationship between Maya attributes and fragment parameters. MAttributeParameterMapping allows implementations of MPxShadingNodeOverride to describe which attribute on a Maya node drives which parameter on the corresponding shader fragment or fragment graph. The plugin can provide a list of mappings in getCustomMappings(). Additional mappings will be automatically created by Maya after getCustomMappings() is called for any attributes that have a parameter on the fragment or fragment graph with the same name and type.

**Public Member Functions**

- `MAttributeParameterMapping (const MString &paramName, const MString &attrName, bool allowConnection, bool allowRename)`  
  Constructor.
- `~MAttributeParameterMapping ()`  
  Destructor.
- `const MString & parameterName () const`  
  Get the parameter name for this mapping.
- `const MString & resolvedParameterName () const`  
  Get the resolved parameter name for this mapping.
- `const MString & attributeName () const`  
  Get the attribute name for this mapping.
- `bool allowConnection () const`  
  This method returns true if Maya is allowed to connect other shade fragments to the parameter named by this mapping.
- `bool allowRename () const`  
  This method returns true if the parameter named by this mapping may be renamed in the final shading effect.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MAttributeParameterMappingList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_attribute_parameter_mapping_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_attribute_parameter_mapping_list.html).

A list of MAttributeParameterMapping objects. A list of MAttributeParameterMapping objects. Ownership of mapping objects added to the list remains with the caller; the list makes a copy.

**Public Member Functions**

- `MAttributeParameterMappingList ()`  
  Constructor.
- `~MAttributeParameterMappingList ()`  
  Destructor.
- `unsigned int length () const`  
  Get the length of the list.
- `const MAttributeParameterMapping * getMapping (unsigned int index) const`  
  Get the mapping at the specified index.
- `const MAttributeParameterMapping * findByAttributeName (const MString &attributeName) const`  
  Find a mapping by attribute name.
- `const MAttributeParameterMapping * findByParameterName (const MString &parameterName) const`  
  Find a mapping by parameter name.
- `void append (const MAttributeParameterMapping &mapping)`  
  Add a mapping to the list.
- `void clear ()`  
  Clear all mappings from the list.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MBlendState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_blend_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_blend_state.html).

Container class for an acquired GPU blend state. MBlendState wraps an acquired composite GPU blend state. This blend state can be used to set the current GPU blend state using MStateManager::setBlendState(myBlendState). Instances of MBlendState cannot be created or modified by users. You can read the descriptor for an MBlendState using MBlendState::desc(), but you cannot set or alter the descriptor directly. MBlendState can only be obtained via MStateManager::acquireBlendState(). To use MStateManager::acquireBlendState(), create and fill an MBlendStateDesc instance with the requested state and pass it to MStateManager::acquireBlendState() which will return the cached unique blend state of that description. If the unique blend state did not previously exist in the cache, it is created. The information included in the blend state includes alpha to coverage enable, independent blend enable, a global float4 blend factor, and an array of blending descriptors, one for each target. The advantages of using this class rather than setting blend states explicitly are: 1. Since the states are cached internally, state setting is much faster than setting individual states. 2. Since state is set through the Maya viewport API, Maya can maintain accurate knowledge of the current GPU state, even in the presence of arbitrary plug-ins. This also speeds state setup by reducing redundant state setting and OGL queries. 3. The information is device aware, meaning that it will return the correct results based on the current active device. For example it will return the appropriate values for DirectX versus an OpenGL device.

**Public Types**

- `enum BlendOperation { kAdd = 0x1, kSubtract = 0x2, kReverseSubtract = 0x3, kMin = 0x4, kMax = 0x5 }`  
  Blend operations for hardware target blending.
- `enum BlendOption { kZero = 1, kOne = 2, kSourceColor = 3, kInvSourceColor = 4, kSourceAlpha = 5, kInvSourceAlpha = 6, kSourceAlphaSat = 11, kDestinationColor = 9, kInvDestinationColor = 10, kDestinationAlpha = 7, kInvDestinationAlpha = 8, kBothSourceAlpha = 12, kBothInvSourceAlpha = 13, kBlendFactor = 14, kInvBlendFactor = 15 }`  
  Specifies what value to use as blend factor with an optional pre-operation.
- `enum ChannelMask { kNoChannels = 0, kRedChannel = 0x1, kGreenChannel = 0x2, kBlueChannel = 0x4, kAlphaChannel = 0x8, kRGBChannels = 0x7, kRGBAChannels = 0xf }`  
  A bitfield that indicates which color components(red, green, blue, alpha) are writable.

**Public Member Functions**

- `const MBlendStateDesc & desc (MStatus *ReturnStatus=NULL) const`  
  Get the blend state descriptor that was used to create the state object.
- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a blend state.
- `~MBlendState ()`  
  Destructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static const int kMaxTargets = 8`  
  Maximum number of simulatanious render targets.


### MBlendStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_blend_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_blend_state_desc.html).

Descriptor for a complete blend state. This class describes a complete blending state, including the blending state for all targets.

**Public Member Functions**

- `MBlendStateDesc ()`  
  Constructor, builds a default blend state.
- `~MBlendStateDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the blend state to their default values.
- `MBlendStateDesc (const MBlendStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MBlendStateDesc & operator= (const MBlendStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `bool alphaToCoverageEnable`  
  Enable alpha to coverage.
- `bool independentBlendEnable`  
  Use TargetBlends[0] if false or individual blend states if true.
- `float blendFactor [4]`  
  Global blend factor, default (1,1,1,1).
- `unsigned int multiSampleMask`  
  Each bit in this mask, starting at the least significant bit (LSB), controls modification of one of the samples in a multisample render target.
- `MTargetBlendDesc targetBlends [MBlendState::kMaxTargets]`  
  An array of 8 MTargetBlendDesc, one for each target.


### MClearOperation

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_clear_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_clear_operation.html).

Class which defines the operation of clearing render target channels.

**Public Types**

- `enum ClearMask { kClearNone = 0, kClearColor = 1, kClearDepth = 1 << 1, kClearStencil = 1 << 2, kClearAll = ~0 }`  
  ClearMask describes the set of channels to clear If the mask value is set then that given channel will be cleared.

**Public Member Functions**

- `MClearOperation (const MString &name)`  
  Constructor for a named clear operation.
- `~MClearOperation () override`  
  Default destructor.
- `bool overridesColors () const`  
  Query whether clear colors are set by the override or come from Maya's preferences.
- `unsigned int mask () const`  
  Query the clear mask.
- `const float * clearColor () const`  
  Query the first clear color value.
- `bool clearGradient () const`  
  Query if the clear should clear with a gradient.
- `const float * clearColor2 () const`  
  Query the second clear color value.
- `int clearStencil () const`  
  Query the stencil clear value.
- `float clearDepth () const`  
  Query the clear depth value.
- `void setOverridesColors (bool override)`  
  Set the enabled state to control whether the clear operation overrides Maya's color preferences.
- `void setMask (unsigned int mask)`  
  Set the clear mask to define which channels to clear.
- `void setClearColor (float value[4])`  
  Set the clear color value.
- `void setClearGradient (bool value)`  
  Set whether to clear with a vertical color gradient.
- `void setClearColor2 (float value[4])`  
  Set the clear second color value.
- `void setClearStencil (int value)`  
  Set the clear stencil value.
- `void setClearDepth (float value)`  
  Set the clear depth value.

**Public Attributes**

- `unsigned int mClearMask`  
  Clear mask.
- `float mClearColor [4]`  
  Clear color value.
- `float mClearColor2 [4]`  
  Secondary clear color value.
- `bool mClearGradient`  
  Flag to indicate whether to clear the gradient.
- `int mClearStencil`  
  Clear stencil value.
- `float mClearDepth`  
  Clear depth value.


### MColorManagementNodes

Module `OpenMayaRender`. Source: [class_m_color_management_nodes.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_nodes.html).

Utilities for manipulating color manageable nodes.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static bool isColorManageable (const MObject &object)`  
  Returns whether the object is color manageable or not.
- `static bool isColorManaged (const MObject &object)`  
  Returns whether the object is color managed or not.
- `static MStatus colorManage (MObject &object, const MString &filename, MDGModifier &dgModifier)`  
  Color manage one specific input node.
- `static MStatus colorManageAllNodes (MDGModifier &dgModifier)`  
  Color manage all input nodes.


### MColorManagementUtilities

Module `OpenMayaRender`. Source: [class_m_color_management_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_utilities.html).

Utilities for obtaining color management information.

**Classes**

- `class MColorTransformData`  
  Utility class that retrieves the opaque data containing the color transform information needed to render the scene.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static MStatus getColorTransformCacheIdForInputSpace (const MString &inputSpaceName, MString &transformId)`  
  Utility function to retrieve the id of a color transform based on the input color space name provided.
- `static MStatus getColorTransformCacheIdForOutputTransform (MString &transformId)`  
  Utility function to retrieve the id of the color transform to be applied on the final output.
- `static bool isColorManagementEnabled ()`  
  Returns whether color management is enabled for the current scene.
- `static bool isColorManagementAvailable ()`  
  Returns whether color management is available for the current product.
- `static MStatus connectDependencyNodeToColorManagement (MObject &object)`  
  This method is obsolete.


### MColorManagementUtilities::MColorTransformData

Module `OpenMayaRender`. Source: [class_m_color_management_utilities_1_1_m_color_transform_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_utilities_1_1_m_color_transform_data.html).

Utility class that retrieves the opaque data containing the color transform information needed to render the scene. This class manages internally a buffer that contains data about color transformation used in the scene. The buffer managed herein whose pointer is returned by the 'getData' method is only valid during the lifetime of this object's instance.

**Public Member Functions**

- `MColorTransformData ()`  
  MColorTransformData constructor.
- `~MColorTransformData ()`  
  MColorTransformData destructor.
- `unsigned getSize () const`  
  Obtain the size of the opaque data block containing the color transform information needed to render the scene.
- `const void * getData () const`  
  Obtain a reference to the opaque data block containing the color transform information needed to render the scene.


### MColorPickerCallback

Module `OpenMayaRender`. Source: [class_m_color_picker_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_picker_callback.html).

Helper class for the color picking mechanism. When using the Color Picking mechanism one should create a callback class inhereting from MColorPickerCallback in order to implement the pure virtual method getcolor(). By having a class (instead of a plain function callback), some custom data could be added to the callback instance.

**Public Member Functions**

- `MColorPickerCallback (QWidget *key)`  
  Constructor.
- `virtual ~MColorPickerCallback ()`  
  Default Destructor.
- `virtual MColor getColor (QWidget *, const QPoint &, bool) const =0`
- `QWidget * getKey () const`  
  Returns the widget used for the registration.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MColorPickerUtilities

Module `OpenMayaRender`. Source: [class_m_color_picker_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_picker_utilities.html).

Utilities for the Color Picker.

**Static Public Member Functions**

- `static bool isRegisteredToColorPicking (QWidget *)`  
  Returns whether the widget registered to the Color Picking mechanism.
- `static MStatus doRegisterToColorPicking (QWidget *, MColorPickerCallback *)`  
  Register the widget to the Color Picking mechanism.
- `static MStatus unregisterFromColorPicking (QWidget *)`  
  Unregister the widget from the Color Picking mechanism.
- `static const char * className ()`  
  Returns the name of this class.


### MCommonRenderSettingsData

Module `OpenMayaRender`. Source: [class_m_common_render_settings_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_common_render_settings_data.html).

Data container for common rendering settings. This class is a container that encapsulates the data for common rendering globals. The data is intended to be accessed using the following method: MCommonRenderSettingsData data;

**Public Types**

- `enum MpathType { kRelativePath, kFullPathImage, kFullPathTmp }`  
  Defines the types file paths.

**Public Member Functions**

- `MCommonRenderSettingsData ()`  
  Default contructor.
- `~MCommonRenderSettingsData ()`  
  Destructor.
- `bool isAnimated () const`  
  Determines if there is animation.
- `bool isMovieFormat () const`  
  Determines if the output format is a single movie file.
- `void setPassName (const MString &pass)`  
  Used with getImageName. Sets the pass name for later use.
- `void setFieldName (const MString &field)`  
  Used with getImageName. Sets the field name for later use.
- `const MString getImageName (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const bool createDirectory=true, MStatus *ReturnStatus=NULL) const`  
  Obtains the image name with the proper subdirectory structure.
- `const MString getImageName (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const MString &customTokenString, const bool createDirectory=true, MStatus *ReturnStatus=NULL) const`  
  Obtains the image name with the proper subdirectory structure (supports custom tokens).
- `bool shouldRenderFrameAtTime (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const MString &customTokenString) const`  
  Indicates whether render the frame or not. If true, render it. If false, skip it.
- `const MString getBufferName (const MObject &renderPass, const MObject &layer, const MString &cameraName, const MString &customTokenString="", const bool leaveUnmatchedTokens=false, MStatus *ReturnStatus=NULL) const`  
  Get image buffer name.

**Static Public Member Functions**

- `static MString getPreRenderFrameCmd ()`  
  Return the preRenderFrame MEL command, which is run immediately before rendering a frame.
- `static MString getPostRenderFrameCmd ()`  
  Return the postRenderFrame MEL command, which is run immediately after rendering a frame.
- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `MString name`  
  Base name for the rendered image.
- `unsigned int namingScheme`  
  File naming scheme.
- `unsigned int framePadding`  
  Number of padding zeros for the frame number.
- `MString namePattern`  
  Single string that defines the whole naming convention.
- `unsigned int imageFormat`  
  Predefined supported image format.
- `MString customImageFormat`  
  Custom image format name.
- `MTime frameStart`  
  First frame number.
- `MTime frameEnd`  
  Last frame number.
- `float frameBy`  
  Frame number step.
- `bool skipExistingFrames`  
  When true, skip existing frames.
- `bool renderAll`  
  When false, only the selected objects should be rendered.
- `bool useCustomExt`  
  When true, overrides the default image format extension.
- `MString customExt`  
  Custom format extension, when useCustomExt is true.
- `bool renumberFrames`  
  When true, images need to be renumbered.
- `float renumberStart`  
  First image number, when renumbering.
- `float renumberBy`  
  Image number step, when renumbering.
- `unsigned int width`  
  Image width in pixels.
- `unsigned int height`  
  Image height in pixels.
- `float deviceAspectRatio`  
  Image aspect ratio. This is equal to pixelAspect * width / height.
- `float pixelAspectRatio`  
  Pixel aspect ratio.
- `float dotPerInch`  
  Dot per Inch.
- `bool enableDefaultLight`  
  When on, a default light should be created when no light is available in the scene.
- `MString preMel`  
  The mel string to be executed before a scene is rendered.
- `MString postMel`  
  The mel string to be executed after a scene is rendered.
- `MString preRenderLayerMel`  
  The mel string to be executed before a render layer is rendered.
- `MString postRenderLayerMel`  
  The mel string to be executed after a render layer is rendered.
- `MString preRenderMel`  
  The mel string to be executed before a frame is rendered.
- `MString postRenderMel`  
  The mel string to be executed after a frame is rendered.

**Friends**

- `class MRenderUtil`


### MComponentDataIndexing

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_component_data_indexing.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_component_data_indexing.html).

Class for storing index mapping when vertices are shared.

**Public Member Functions**

- `MUintArray & indices ()`  
  Get the array of vertex indices for the component.
- `const MUintArray & indices () const`  
  Get the array of vertex indices for the component.
- `MComponentType componentType () const`  
  Get the component type that the vertex indices represent.
- `void setComponentType (MComponentType type)`  
  Set the component type that the vertex indices represent.


### MComponentDataIndexingList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_component_data_indexing_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_component_data_indexing_list.html).

A list of MIndexBufferDescriptor objects. A list of MIndexBufferDescriptor objects.

**Public Member Functions**

- `MComponentDataIndexingList ()`  
  Constructor.
- `~MComponentDataIndexingList ()`  
  Destructor.
- `int length () const`  
  Get the number of MComponentDataIndexing in the list.
- `const MComponentDataIndexing * operator[] (int index) const`  
  Gets the MComponentDataIndexing at the requested index.
- `MComponentDataIndexing * operator[] (int index)`  
  Gets the MComponentDataIndexing at the requested index.
- `bool append (const MComponentDataIndexing &desc)`  
  Add a MComponentDataIndexing to the list.
- `bool removeAt (int index)`  
  Remove a MComponentDataIndexing from the list.
- `void clear ()`  
  Clears the list.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MD3D9Renderer

Module `OpenMayaRender`. Source: [class_m_d3_d9_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d3_d9_renderer.html).

MD3D9Renderer is a wrapper class to provide access to Maya's DirectX resources.

**Public Member Functions**

- `MStatus makeSwatchContextCurrent (unsigned int &width, unsigned int &height)`  
  Create a directX render target (or context) to render into for swatches.
- `MStatus setBackgroundColor (const MColor &color)`  
  Set the back ground color of the swatch.
- `MStatus readSwatchContextPixels (MImage &image)`  
  Reads out the current swatch context color buffer into an image format.

**Static Public Member Functions**

- `static MD3D9Renderer * theRenderer ()`  
  Returns a static pointer to the hardware renderer wrapper class.


### MDepthStencilState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_depth_stencil_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_depth_stencil_state.html).

Container class for an acquired complete GPU depth stencil state. MDepthStencilState wraps an acquired composite GPU depth stencil state. This depth stencil state can be used to set the current GPU depth stencil state using MStateManager::setDepthStencilState(myDepthState). Instances of MDepthStencilState cannot be created or modified by users. You can read the descriptor for an MDepthStencilState using MDepthStencilState::desc(), but you cannot set or alter the descriptor directly. MDepthStencilState can only be obtained via MStateManager::acquireDepthStencilState(). To use MStateManager::acquireDepthStencilState(), create and fill an MDepthStencilStateDesc instance with the requested state and pass it to MStateManager::acquireDepthStencilState() which will return the cached unique depth stencil state of that description. If the unique depth stencil state did not previously exist in the cache, it is created. The information included in the depth stencil state includes depth buffer enable, depth buffer write enable, stencil enable, and separate stencil operations for front faces and back faces. The advantages of using this class rather than setting depth stencil states explicitly are: 1. Since the states are cached internally, state setting is much faster than setting individual states. 2. Since state is set through the Maya viewport API, Maya can maintain accurate knowledge of the current GPU state, even in the presence of arbitrary plug-ins. This also speeds state setup by reducing redundant state setting and OGL queries. 3. The information is device aware, meaning that it will return the correct results based on the current active device. For example it will return the appropriate values for DirectX versus an OpenGL device.

**Public Types**

- `enum StencilOperation { kKeepStencil = 1, kZeroStencil = 2, kReplaceStencil = 3, kIncrementStencilSat = 4, kDecrementStencilSat = 5, kInvertStencil = 6, kIncrementStencil = 7, kDecrementStencil = 8 }`  
  Describes the choices for stencil buffer operations.

**Public Member Functions**

- `const MDepthStencilStateDesc & desc (MStatus *ReturnStatus=NULL) const`  
  Get the depth stencil state descriptor that was used to create the state object.
- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a depth-stencil state.
- `~MDepthStencilState ()`  
  Destructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDepthStencilStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_depth_stencil_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_depth_stencil_state_desc.html).

Descriptor for a complete depth-stencil state. This class encapsulates a complete depth-stencil state, like z buffer enable, z write enable, z comparison function, stencil enable, stencil masks and reference values. It also contains MStencilOpDesc stencil operation descriptors for front and back faces.

**Public Member Functions**

- `MDepthStencilStateDesc ()`  
  Constructor, builds a default depth stencil state.
- `~MDepthStencilStateDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the depth stencil state to their default values.
- `MDepthStencilStateDesc (const MDepthStencilStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MDepthStencilStateDesc & operator= (const MDepthStencilStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `bool depthEnable`  
  Enables depth buffer reads and compares, default true.
- `bool depthWriteEnable`  
  Enables depth buffer writes, default true.
- `MStateManager::CompareMode depthFunc`  
  Sets the depth buffer comparison function, default less than.
- `bool stencilEnable`  
  Enables stencil buffer operation.
- `unsigned char stencilReadMask`  
  Sets a bitwise stencil buffer read mask, default 0xff.
- `unsigned char stencilWriteMask`  
  Sets a bitwise stencil buffer write mask, default 0xff.
- `int stencilReferenceVal`  
  Sets the stencil reference value.
- `MStencilOpDesc frontFace`  
  Sets the stencil op for the front facing fragments.
- `MStencilOpDesc backFace`  
  Sets the stencil op for the back facing fragments.


### MDrawContext

Module `OpenMayaRender`. Inherits `MFrameContext`. Source: [class_m_h_w_render_1_1_m_draw_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_draw_context.html).

Class to allow access to hardware draw context information. MDrawContext provides access to current hardware draw context information. It is read-only and cannot be instantiated by the plugin writer. The information provided includes various transformation matrices, camera information, and render target information such as output buffer size. It is also possible to access and alter GPU state through this class. There are a few main advantages for using this class: 1. The data is computed and cached as required. This can avoid work for the plugin writer to extract this information themselves. 2. The plugin writer should not need to directly access the device to extract this information. Any data extraction from the device has a performance penalty which can be avoided. 3. The information is device aware, meaning that it will return the correct results based on the current active device. For example it will return the appropriate values for DirectX versus an OpenGL device. MDrawContext is derived from MFrameContext and provides its own implementation for all virtual methods. The values returned from these methods may differ slightly between MFrameContext and MDrawContext as MFrameContext retrieves the values from Maya and MDrawContext retrieves the values from the GPU device state. Some information will vary from one draw to the next draw such as the object-to-world matrix which will change as different objects are being drawn.

**Public Types**

- `enum LightFilter { kFilteredToLightLimit, kFilteredIgnoreLightLimit }`  
  Options on which lights to return information for from light query methods.
- `enum InternalTexture { kDepthPeelingTranspDepthTexture, kDepthPeelingOpaqueDepthTexture, kIrradianceEnvironment, kSpecularEnvironment, kNumberOfInternalTextures }`  
  Options on which internal texture to get.

**Public Member Functions**

- `MMatrix getMatrix (MFrameContext::MatrixType mtype, MStatus *ReturnStatus=NULL) const override`  
  Get a matrix value of a certain type.
- `MDoubleArray getTuple (MFrameContext::TupleType ttype, MStatus *ReturnStatus=NULL) const override`  
  Get a tuple (vector, position, or single) value of a certain type.
- `MStatus getViewportDimensions (int &originX, int &originY, int &width, int &height) const override`  
  Get the viewport dimensions.
- `MUint64 getFrameStamp () const`  
  Returns the current frame stamp.
- `MBoundingBox getSceneBox (MStatus *ReturnStatus=NULL) const`  
  Get a bounding box of the scene in world space.
- `MBoundingBox getFrustumBox (MStatus *ReturnStatus) const`  
  Get the bounding box of the current view frustum in world space.
- `MStatus getRenderTargetSize (int &width, int &height) const`  
  Get the size of the render target (output buffer) being rendered into.
- `MStatus getDepthRange (float &nearVal, float &farVal) const`  
  Get the depth range which specifies the mapping of depth values from normalized device coordinates to window coordinates.
- `bool viewDirectionAlongNegZ (MStatus *ReturnStatus) const`  
  Return whether the view direction is pointing down the -Z axis.
- `unsigned int numberOfActiveLights (LightFilter lightFilter=kFilteredToLightLimit, MStatus *ReturnStatus=NULL) const`  
  Return the number of available lights to render the scene.
- `MStatus getLightInformation (unsigned int lightNumber, MFloatPointArray &positions, MFloatVector &direction, float &intensity, MColor &color, bool &hasDirection, bool &hasPosition, LightFilter lightFilter=kFilteredToLightLimit) const`  
  Return common lighting information for a given active light.
- `MLightParameterInformation * getLightParameterInformation (unsigned int lightNumber, LightFilter lightFilter=kFilteredToLightLimit) const`  
  Return parameter information for a given active light.
- `MHWRender::MStateManager * getStateManager () const`  
  Access the GPU state manager for the current draw context.
- `const MHWRender::MPassContext & getPassContext () const`  
  Access the current pass context.
- `const MRenderTarget * copyCurrentColorRenderTarget (const MString &name) const`  
  Get a copy of the current color render target.
- `const MRenderTarget * copyCurrentDepthRenderTarget (const MString &name) const`  
  Get a copy of the current depth render target.
- `MTexture * copyCurrentColorRenderTargetToTexture () const`  
  Get a copy of the current color render target as a texture.
- `MTexture * copyCurrentDepthRenderTargetToTexture () const`  
  Get a copy of the current depth render target as a texture.
- `const MTexture * getInternalTexture (InternalTexture internalTexture) const`  
  Returns the internal texture as specified.
- `bool isMultiDraw () const`  
  Returns true if the context is rendering in MultiDraw mode.
- `unsigned int numberOfActiveLights (MStatus *ReturnStatus) const`  
  This method is obsolete.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDrawProcedureBase

Module `OpenMayaRender`. Source: [class_m_draw_procedure_base.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_procedure_base.html).

This class provides an interface through which a plug-in can be writen to implement a class to provide custom hardware drawing effects. The derived class can be added, removed, or reordered in a list of draw procedures used by the hardware renderer. Please refer to documentation for MHardwareRenderer for more details. Each procedure has a user defined string name and can be enabled or disabled. Name, and enabling methods must be defined. All derived classes must over the execute() method. This is the method that will be called by the hardware renderer to which the procedure is attached. The call will only be made if the procedure is enabled.

**Public Member Functions**

- `MDrawProcedureBase (const MString &name)`  
  Constructor for a draw procedure.
- `virtual ~MDrawProcedureBase ()`  
  Default destructor for a draw procedure.
- `virtual bool execute ()=0`  
  This method gets called by the renderer to execture the draw procedure.
- `void setEnabled (bool value)`  
  This method sets whether the draw procedure is enabled or not.
- `bool enabled () const`  
  This method returns whether the draw procedure is enabled or not.
- `void setName (const MString &name)`  
  This method sets the name for the draw procedure.
- `const MString & name () const`  
  This method returns the name of the draw procedure.


### MDrawRegistry

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_draw_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_draw_registry.html).

Access the registry associating node types with custom implementations. This class provides a means to register custom implementations to be used by Viewport 2.0. Custom draw overrides : Used by Viewport 2.0 to draw dependency nodes. The registry is based on classification strings. Each draw override must be registered with a classification string and any node type with a matching classification string will be drawn using the registered draw override. Custom stream buffer generators/mutators : Used by Viewport 2.0 to generate custom streams. The registry is based on identification name. Each generator/mutator must be registered with a identification name and any stream with a matching name will be filled using the registered generator or altered using the registered mutator. Custom index buffer generators/mutators : Used by Viewport 2.0 to generate custom index buffers. The registry is based on primitive type. Each generator/mutator must be registered with a primitive type and any index buffer with a matching type will be filled using the registered generator or altered using the registered mutator. Custom selection component converters : Used by Viewport 2.0 to perform custom component selection. The registry is based on identification name. Each convert must be registered with a identification name and any render item with a matching name will be processed by the converter during a viewport 2.0 selection.

**Static Public Member Functions**

- `static MStatus registerShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId, ShadingNodeOverrideCreator creator)`  
  Register an implementation of MPxShadingNodeOverride to use with nodes that have the specified, draw-specific classification string.
- `static MStatus deregisterShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxShadingNodeOverride.
- `static MStatus registerSurfaceShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId, SurfaceShadingNodeOverrideCreator creator)`  
  Register an implementation of MPxSurfaceShadingNodeOverride to use with surface shaders that have the specified, draw-specific classification string.
- `static MStatus deregisterSurfaceShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxSurfaceShadingNodeOverride.
- `static MStatus registerShaderOverrideCreator (const MString &drawClassification, const MString &registrantId, ShaderOverrideCreator creator)`  
  Register an implementation of MPxShaderOverride to use with nodes that have the specified, draw-specific classification string.
- `static MStatus deregisterShaderOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxShaderOverride.
- `static MStatus registerGeometryOverrideCreator (const MString &drawClassification, const MString &registrantId, GeometryOverrideCreator creator)`  
  Register an implementation of MPxGeometryOverride to use with nodes that have the specified, draw-specific classification string.
- `static MStatus deregisterGeometryOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxGeometryOverride.
- `static MStatus registerDrawOverrideCreator (const MString &drawClassification, const MString &registrantId, DrawOverrideCreator creator)`  
  Register an implementation of MPxDrawOverride to use with DAG objects that have the specified, draw-specific classification string.
- `static MStatus deregisterDrawOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxDrawOverride.
- `static MStatus registerSubSceneOverrideCreator (const MString &drawClassification, const MString &registrantId, SubSceneOverrideCreator creator)`  
  Register an implementation of MPxSubSceneOverride to use with DAG objects that have the specified, draw-specific classification string.
- `static MStatus deregisterSubSceneOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxSubSceneOverride.
- `static MStatus registerImagePlaneOverrideCreator (const MString &drawClassification, const MString &registrantId, ImagePlaneOverrideCreator creator)`  
  Register an implementation of MPxImagePlaneOverride to use with nodes that have the specified, draw-specific classification string.
- `static MStatus deregisterImagePlaneOverrideCreator (const MString &drawClassification, const MString &registrantId)`  
  Deregister an implementation of MPxImagePlaneOverride.
- `static MStatus registerVertexBufferGenerator (const MString &bufferName, VertexBufferGeneratorCreator creator)`  
  Register an implementation of MPxVertexBufferGenerator to provide custom vertex streams for shapes.
- `static MStatus deregisterVertexBufferGenerator (const MString &bufferName)`  
  Deregister an implementation of MPxVertexBufferGenerator.
- `static MStatus registerVertexBufferMutator (const MString &bufferName, VertexBufferMutatorCreator creator)`  
  Register an implementation of MPxVertexBufferMutator to provide custom vertex streams for shapes.
- `static MStatus deregisterVertexBufferMutator (const MString &bufferName)`  
  Deregister an implementation of MPxVertexBufferMutator.
- `static MStatus registerPrimitiveGenerator (const MString &primitiveType, PrimitiveGeneratorCreator creator)`  
  Register an implementation of MPxPrimitiveGenerator to generate custom primitive types for shapes.
- `static MStatus deregisterPrimitiveGenerator (const MString &primitiveType)`  
  Deregister an implementation of MPxPrimitiveGenerator.
- `static MStatus registerIndexBufferMutator (const MString &primitiveType, IndexBufferMutatorCreator creator)`  
  Register an implementation of MPxIndexBufferMutator to generate custom primitive types for shapes.
- `static MStatus deregisterIndexBufferMutator (const MString &primitiveType)`  
  Deregister an implementation of MPxIndexBufferMutator.
- `static MStatus registerComponentConverter (const MString &renderItemName, ComponentConverterCreator creator)`  
  Register an implementation of MPxComponentConverter to use with render items that have the specified name.
- `static MStatus deregisterComponentConverter (const MString &renderItemName)`  
  Deregister a previously registered MPxComponentConverter for given render item name.
- `static void setCallableInfo (MPluginCallableInfo *info)`  
  Mark the current registration operation as one that will be passed callbacks defined in script.
- `static MPluginCallableInfo * getCallableInfo ()`  
  Return true if the current registration operation has its callbacks defined in script.
- `static const char * className ()`  
  Returns the name of this class.


### MFnImageSource

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_image_source.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_image_source.html).

Function set for image sources. An image source is a node that is used in render graphs, e.g. compositing graphs, shading networks. It provides access to the images produced by 3D scene renderers, or 2D image processing tasks. This class provides functionality for working with image sources, e.g. retrieving the path of rendered image source files.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnImageSource ()`  
  Destructor.
- `MFnImageSource ()`  
  Default constructor.
- `MFnImageSource (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MString getImageName (MCommonRenderSettingsData::MpathType pathType, double frameNumber, const MString &sceneName, const MObject camera, const MObject renderLayer, MString *bufferName=NULL, MStatus *ReturnStatus=NULL) const`  
  Return image path associated with this image source.
- `MFnImageSource (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnRenderLayer

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_render_layer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_render_layer.html).

Function set for render layer. Provide functionalities for working with render layers such as getting the objects present in the render layer or checking whether the given object is in the current layer.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnRenderLayer ()`  
  Destructor.
- `MFnRenderLayer ()`  
  Default constructor.
- `MFnRenderLayer (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `bool inLayer (const MObject &transform, MStatus *ReturnStatus=NULL)`  
  Returns true if the given shape is in this layer.
- `MStatus layerChildren (MObjectArray &children, bool recurse=false)`  
  Returns the container layers for this layer.
- `bool inCurrentRenderLayer (const MDagPath &objectPath, MStatus *ReturnStatus=NULL)`  
  The function checks if the given object is present in the current render layer or not.
- `MStatus listMembers (MObjectArray &objectsInLayer) const`  
  Returns the objects present in the render layer.
- `bool isPlugAdjusted (const MPlug &scenePlug, MStatus *ReturnStatus=NULL) const`  
  The function checks if the specified plug is adjusted or not; returns true if the plug is adjusted, else returns false.
- `MPlug adjustmentPlug (const MPlug &scenePlug, MStatus *ReturnStatus=NULL) const`  
  Returns the render layer adjustment value plug for the specified plug.
- `MStatus externalRenderPasses (MObjectArray &renderPassArray) const`  
  The function builds an array of unique render pass nodes (MObject) that are connected to the renderPass attribute.
- `bool passHasObject (const MObject &renderPass, const MDagPath &objectPath, MStatus *ReturnStatus=NULL) const`  
  Returns true if the specified object instance contributes to the given render pass, based on the pass contribution maps attached to the layer.
- `bool passHasLight (const MObject &renderPass, const MObject &light, MStatus *ReturnStatus=NULL) const`  
  Returns true if the specified light contributes to the given render pass, based on the pass contribution maps attached to this layer.
- `MFnRenderLayer (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static MObject findLayerByName (const MString &renderLayer, MStatus *ReturnStatus=NULL)`  
  This function returns an MObject to a render layer that matches the specified name.
- `static MObject defaultRenderLayer (MStatus *ReturnStatus=NULL)`  
  Returns the MObject for the defaultRenderLayer.
- `static MStatus listAllRenderLayers (MObjectArray &rl)`  
  Returns the list of render layers currently in the system.
- `static MObject currentLayer (MStatus *ReturnStatus=NULL)`  
  Returns the MObject for the current render layer.


### MFnRenderPass

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_render_pass.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_render_pass.html).

Function set for render passes. Provide functionalities for working with render passes such as retrieving renderer-specific implementations.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnRenderPass ()`  
  Destructor.
- `MFnRenderPass ()`  
  Default constructor.
- `MFnRenderPass (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MPxRenderPassImpl * setImplementation (const MString &renderer, MStatus *ReturnStatus=NULL)`  
  Returns the first render pass implementation compatible with the specified renderer and that supports the currently selected frame buffer data type and the selected number of channels.
- `MPxRenderPassImpl * getImplementation () const`  
  Returns the render pass implementation that was resolved by the previous call to setImplementation.
- `MPxRenderPassImpl::PassTypeBit frameBufferType (MStatus *ReturnStatus=NULL) const`  
  Returns the data type of the frame buffer associated with the render pass.
- `unsigned int frameBufferChannels (MStatus *ReturnStatus=NULL) const`  
  Returns the number of channels of the frame buffer associated with the render pass.
- `bool usesFiltering (MStatus *ReturnStatus=NULL) const`  
  Returns true if frame buffer filtering is requested.
- `MString passID (MStatus *ReturnStatus=NULL) const`  
  Returns the passID string that uniquely identifies the renderpass definition associated with the pass.
- `MString customTokenString (MStatus *ReturnStatus=NULL) const`  
  Returns a custom token string that can be passed to MCommonRenderSettingsData::getImageName() to generate a file name for the pass.
- `MFnRenderPass (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFragmentManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_fragment_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_fragment_manager.html).

Provides facilities for managing fragments for use with Viewport 2.0. MFragmentManager provides functionality for dealing with Viewport 2.0 shade fragments and fragment graphs. Both are defined using an XML format and can be specified using either a file or a string buffer. The XML format describes input and output parameters for the fragment as well as default values for parameters. It also allows the specification of multiple implementations of the fragment (e.g. a Cg implementation and an HLSL implementation) so that fragments can be created that work in both the OpenGL and DirectX versions of Viewport 2.0. The format can also be used to define how several fragments should be connected together into a single fragment graph. Please see the Viewport 2.0 section of the Maya API guide for a more detailed description of the XML along with a complete XML Schema (XSD) for both fragments and fragment graphs. In addition to the description in the API guide MFragmentManager provides several methods for examining how Maya works with fragments and graphs. The XML for many of the fragments and fragment graphs used for Maya's internal nodes are available for inspection. fragmentList() will return a list of the names of all non-hidden fragments and fragment graphs. For any of these names, getFragmentXML() may be called to retrieve the XML for that fragment. While combining fragments, Maya often creates intermediate fragment graphs on the fly. These are not stored in the manager so they may not be queried through getFragmentXML(). To see the XML for these intermediate graphs, call setIntermediateGraphOutputDirectory() with a directory and Maya will dump the XML to files in that directory every time it creates such a graph. When all fragments have been connected and joined together, Maya will create a final effect. This effect is compiled and used to drive geometry requirements and final shading. The text for this effect can be dumped to disk every time it is created by calling setEffectOutputDirectory() with a valid directory. This text can be used to see how final effects are created out of fragment graphs. Note that for the intermediate graph and effect output directories, Maya will use whatever value was set last. So multiple plugins that set these values may conflict with each other. These methods are intended to be used as debug tools only during development of a plugin. The usage of them incurs a performance penalty and so it is strongly discouraged to use them in a production setting.

**Public Types**

- `enum ShaderStage { kVertexShader, kPixelShader, kGeometryShader, kHullShader, kHullConstantShader, kDomainShader }`  
  Specifies shader stage in GPU pipeline.

**Public Member Functions**

- `bool hasFragment (const MString &fragmentName) const`  
  Returns true if a fragment of the given name has been registered with the fragment manager.
- `void fragmentList (MStringArray &fragmentNames) const`  
  Returns a list of the names of all registered fragments and fragment graphs (both internal and external) which are not hidden.
- `bool getFragmentXML (const MString &fragmentName, MString &buffer) const`  
  Fills the buffer with the XML representation of the named fragment or fragment graph (if it exists).
- `bool getFragmentXML (const MObject &shadingNode, MString &buffer, bool includeUpstreamNodes=false, const MDagPath *objectContext=NULL) const`  
  Fills the buffer with the XML code for the fragment graph Maya would use to represent the given shading node in Viewport 2.0.
- `MString addShadeFragmentFromBuffer (const char *buffer, bool hidden)`  
  Add a new fragment to the manager.
- `MString addShadeFragmentFromFile (const MString &fileName, bool hidden)`  
  Add a new fragment to the manager.
- `MString addFragmentGraphFromBuffer (const char *buffer)`  
  Add a new fragment graph to the manager.
- `MString addFragmentGraphFromFile (const MString &fileName)`  
  Add a new fragment graph to the manager.
- `bool removeFragment (const MString &fragmentName)`  
  Remove a named fragment or fragment graph from the fragment manager.
- `bool addFragmentPath (const MString &path)`  
  Add a path to the list of fragment search paths used when parsing the file path for any methods which add fragments to the manager from files on disk.
- `bool addAutomaticShaderStageInput (ShaderStage shaderStage, const MString &parameterName, const MString &semantic, MShaderInstance::ParameterType parameterType, bool isVaryingInput)`  
  Add a parameter to the list of automatic input parameters for a specified shader stage.
- `bool removeAutomaticShaderStageInput (ShaderStage shaderStage, const MString &parameterName)`  
  Remove a parameter from the list of automatic input parameters for a specified shader stage.
- `bool addDomainShaderInputNameMapping (const MString &realParamName, const MString &domainParamName)`  
  Add a mapping between a parameter name (realParamName) and a transient name (domainParamName) which is used in domain shader.
- `bool removeDomainShaderInputNameMapping (const MString &realParamName)`  
  Remove a mapping between a parameter name (realParamName) and a transient name (domainParamName) which is used in domain shader.
- `MString findDomainShaderInputName (const MString &realParamName)`  
  Find the transient name which is used in domain shader.
- `void setEffectOutputDirectory (const MString &dir)`  
  Set the path to use for dumping final effect files.
- `MString getEffectOutputDirectory () const`  
  Get the directory to be used for effect file output.
- `void setIntermediateGraphOutputDirectory (const MString &dir)`  
  Set the path to use for dumping intermediate fragment graph XML files.
- `MString getIntermediateGraphOutputDirectory () const`  
  Get the directory to be used for intermediate fragment graph output.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFrameContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_frame_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_frame_context.html).

This class contains some global information for the current render frame. MFrameContext is designed to provide information which is available per frame render. This includes information such as render targets, viewport size and camera information. In terms of relative scope, MFrameContext can be thought of as encompassing the time period for a "pass" (MPassContext) and the time period for actual drawing (MDrawContext). MDrawContext is derived from MFrameContext and provides its own implementation for all virtual methods. The values returned from these methods may differ slightly between MFrameContext and MDrawContext as MFrameContext retrieves the values from Maya and MDrawContext retrieves the values from the GPU device state. Also, MFrameContext::getMatrix() is not able to return values for any matrix type requiring the object-to-world matrix as that information is only available at draw time.

**Classes**

- `struct BackgroundParams`  
  Background parameters.
- `struct DOFParams`  
  Depth-of-field parameters.
- `struct EnvironmentParams`  
  Struct for environment map parameters.
- `struct HwFogParams`  
  Struct for hardware fog parameters.
- `struct RenderOverrideInformation`  
  Render override information.

**Public Types**

- `enum MatrixType { kWorldMtx, kWorldTransposeMtx, kWorldInverseMtx, kWorldTranspInverseMtx, kViewMtx, kViewTransposeMtx, kViewInverseMtx, kViewTranspInverseMtx, kProjectionMtx, kProjectionTranposeMtx, kProjectionInverseMtx, kProjectionTranspInverseMtx, kViewProjMtx, kViewProjTranposeMtx, kViewProjInverseMtx, kViewProjTranspInverseMtx, kWorldViewMtx, kWorldViewTransposeMtx, kWorldViewInverseMtx, kWorldViewTranspInverseMtx, kWorldViewProjMtx, kWorldViewProjTransposeMtx, kWorldViewProjInverseMtx, kWorldViewProjTranspInverseMtx }`  
  Matrices that can be accessed.
- `enum TupleType { kViewPosition, kViewDirection, kViewUp, kViewRight, kViewportPixelSize, kViewNearClipValue, kViewFarClipValue, kViewUnnormlizedNearClipValue, kViewUnnormalizedFarClipValue }`  
  Tuples that can be accessed. May be one or more values returned depending on the tuple requested.
- `enum DisplayStyle { kGouraudShaded = (0x1), kWireFrame = (0x1 << 1), kBoundingBox = (0x1 << 2), kTextured = (0x1 << 3), kDefaultMaterial = (0x1 << 4), kXrayJoint = (0x1 << 5), kXray = (0x1 << 6), kTwoSidedLighting = (0x1 << 7), kFlatShaded = (0x1 << 8), kShadeActiveOnly = (0x1 << 9), kXrayActiveComponents = (0x1 << 10), kBackfaceCulling = (0x1 << 11), kSmoothWireframe = (0x1 << 12) }`  
  Display styles.
- `enum LightingMode { kNoLighting, kAmbientLight, kLightDefault, kSelectedLights, kSceneLights, kCustomLights }`  
  Lighting modes.
- `enum PostEffectType { kAmbientOcclusion, kMotionBlur, kGammaCorrection, kViewColorTransformEnabled = kGammaCorrection, kDepthOfField, kAntiAliasing }`  
  Types of post effects which may be enabled during rendering.
- `enum FogMode { kFogLinear, kFogExp, kFogExp2 }`  
  Types of the fog mode.
- `enum TransparencyAlgorithm { kUnsorted, kObjectSorting, kWeightedAverage, kDepthPeeling }`  
  Options for transparency algorithm.
- `enum WireOnShadedMode { kWireframeOnShadedFull, kWireFrameOnShadedReduced, kWireFrameOnShadedNone }`  
  Wireframe on shaded modes.
- `enum RenderingDestination { k3dViewport, k2dViewport, kImage }`  
  Rendering destinations.

**Public Member Functions**

- `virtual MMatrix getMatrix (MFrameContext::MatrixType mtype, MStatus *ReturnStatus=NULL) const`  
  Get a matrix value of a certain type.
- `virtual MDoubleArray getTuple (MFrameContext::TupleType ttype, MStatus *ReturnStatus=NULL) const`  
  Get a tuple (vector or position) value of a certain type.
- `virtual MStatus getViewportDimensions (int &originX, int &originY, int &width, int &height) const`  
  Get the viewport dimensions.
- `virtual float getGlobalLineWidth () const`  
  Get global line width.
- `MDagPath getCurrentCameraPath (MStatus *ReturnStatus=NULL) const`  
  Get the path to the camera being used to render the current frame.
- `const MRenderTarget * getCurrentColorRenderTarget () const`  
  Get current color render target.
- `const MRenderTarget * getCurrentDepthRenderTarget () const`  
  Get current depth render target.
- `unsigned int getDisplayStyle () const`  
  Get the current display style as a bit field.
- `LightingMode getLightingMode () const`  
  Get the current light mode.
- `unsigned int getLightLimit () const`  
  Get the current light limit.
- `EnvironmentParams getEnvironmentParameters () const`  
  Get parameters for currently used environment.
- `bool getPostEffectEnabled (PostEffectType postEffectType) const`  
  Returns if a given post effect is currently enabled.
- `HwFogParams getHwFogParameters () const`  
  Get all the hardware fog parameters.
- `TransparencyAlgorithm getTransparencyAlgorithm () const`  
  Get the current transparency algoritm.
- `RenderingDestination renderingDestination (MString &destinationName) const`  
  Return value indicates the destination that the renderer is drawing to.
- `MUint64 objectTypeExclusions () const`  
  Get the object type exclusions as a bitfield.
- `void classificationExclusions (MStringArray &classification) const`  
  Get a list of drawdb strings for object which are excluded from display.
- `void getBackgroundParameters (MFrameContext::BackgroundParams &params) const`  
  Get parameters related to how the background is cleared.
- `void getDOFParameters (MFrameContext::DOFParams &params) const`  
  Get the parameters generated by Maya for the circle-of-confusion depth shader used for a pass used when computing depth of field.
- `void getRenderOverrideInformation (RenderOverrideInformation &info) const`  
  Get information about any render override.
- `const MSelectionInfo * getSelectionInfo () const`  
  Get the selection state information if the method is called in selection pass; nullptr will be returned otherwise.
- `void worldToViewport (const MPoint &inPoint, double &xPos, double &yPos) const`  
  Convert coordinate in world space to coordinate in viewport.
- `void viewportToWorld (double xPos, double yPos, MPoint &worldNearPt, MPoint &worldFarPt) const`  
  Convert coordinate in viewport to near cliping plane coordinate in world space.

**Static Public Member Functions**

- `static MFrameContext::MatrixType semanticToMatrixType (const MString &value, MStatus *ReturnStatus=NULL)`  
  It is assumed that these flags match between MFrameContext and TexcludeObjectDisplay, but that is not enforced elsewhere.
- `static MFrameContext::TupleType semanticToTupleType (const MString &value, MStatus *ReturnStatus=NULL)`  
  Given a semantic name return the corresponding tuple enumeration that can be used to retrieve a value via the getTuple() method.
- `static bool inUserInteraction ()`  
  Returns true during any interactive refresh, as when user is interacting with the scene in any way including camera changes, object or component TRS changes, etc.
- `static bool userChangingViewContext ()`  
  Returns true during any interactive refresh, as when user is changing the view using view context tools such as tumble, dolly or track.
- `static WireOnShadedMode wireOnShadedMode ()`  
  Returns the global user display preference which indicates how wireframe should be drawn on top of objects while in shaded mode.
- `static bool shadeTemplates ()`  
  Returns the display preference indicating whether templated objects should be drawn shaded.
- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static constexpr MUint64 kExcludeNone = MUint64(0)`  
  Bit flags specifying types of objects excluded from displaying in the viewport.
- `static constexpr MUint64 kExcludeNurbsCurves = MUint64(1) << (0)`  
  Exclude NURBS curves.
- `static constexpr MUint64 kExcludeNurbsSurfaces = MUint64(1) << (1)`  
  Exclude NURBS surface.
- `static constexpr MUint64 kExcludeMeshes = MUint64(1) << (2)`  
  Exclude polygonal meshes.
- `static constexpr MUint64 kExcludePlanes = MUint64(1) << (3)`  
  Exclude planes.
- `static constexpr MUint64 kExcludeLights = MUint64(1) << (4)`  
  Exclude lights.
- `static constexpr MUint64 kExcludeCameras = MUint64(1) << (5)`  
  Exclude camera.
- `static constexpr MUint64 kExcludeJoints = MUint64(1) << (6)`  
  Exclude joints.
- `static constexpr MUint64 kExcludeIkHandles = MUint64(1) << (7)`  
  Exclude IK handles.
- `static constexpr MUint64 kExcludeDeformers = MUint64(1) << (8)`  
  Exclude all deformations.
- `static constexpr MUint64 kExcludeDynamics = MUint64(1) << (9)`  
  Exclude all dynamics objects (emiiters, cloth)
- `static constexpr MUint64 kExcludeParticleInstancers = MUint64(1) << (10)`  
  Exclude all Particle Instancers.
- `static constexpr MUint64 kExcludeLocators = MUint64(1) << (11)`  
  Exclude locators.
- `static constexpr MUint64 kExcludeDimensions = MUint64(1) << (12)`  
  Exclude all measurement objects.
- `static constexpr MUint64 kExcludeSelectHandles = MUint64(1) << (13)`  
  Exclude selection handles.
- `static constexpr MUint64 kExcludePivots = MUint64(1) << (14)`  
  Exclude pivots.
- `static constexpr MUint64 kExcludeTextures = MUint64(1) << (15)`  
  Exclude texure placement objects.
- `static constexpr MUint64 kExcludeGrid = MUint64(1) << (16)`  
  Exclude grid drawing.
- `static constexpr MUint64 kExcludeCVs = MUint64(1) << (17)`  
  Exclude NURBS control vertices.
- `static constexpr MUint64 kExcludeHulls = MUint64(1) << (18)`  
  Exclude NURBS hulls.
- `static constexpr MUint64 kExcludeStrokes = MUint64(1) << (19)`  
  Exclude PaintFX strokes.
- `static constexpr MUint64 kExcludeSubdivSurfaces = MUint64(1) << (20)`  
  Exclude subdivision surfaces.
- `static constexpr MUint64 kExcludeFluids = MUint64(1) << (21)`  
  Exclude fluid objects.
- `static constexpr MUint64 kExcludeFollicles = MUint64(1) << (22)`  
  Exclude hair follicles.
- `static constexpr MUint64 kExcludeHairSystems = MUint64(1) << (23)`  
  Exclude hair system.
- `static constexpr MUint64 kExcludeImagePlane = MUint64(1) << (24)`  
  Exclude image planes.
- `static constexpr MUint64 kExcludeNCloths = MUint64(1) << (25)`  
  Exclude N-cloth objects.
- `static constexpr MUint64 kExcludeNRigids = MUint64(1) << (26)`  
  Exclude rigid-body objects.
- `static constexpr MUint64 kExcludeDynamicConstraints = MUint64(1) << (27)`  
  Exclude rigid-body constraints.
- `static constexpr MUint64 kExcludeManipulators = MUint64(1) << (28)`  
  Exclude manipulators.
- `static constexpr MUint64 kExcludeNParticles = MUint64(1) << (29)`  
  Exclude N-particle objects.
- `static constexpr MUint64 kExcludeMotionTrails = MUint64(1) << (30)`  
  Exclude motion trails.
- `static constexpr MUint64 kExcludeHoldOuts = MUint64(1) << (31)`  
  Exclude Hold-Outs.
- `static constexpr MUint64 kExcludePluginShapes = MUint64(1) << (32)`  
  Exclude plug-in shapes.
- `static constexpr MUint64 kExcludeHUD = MUint64(1) << (33)`  
  Exclude HUD.
- `static constexpr MUint64 kExcludeClipGhosts = MUint64(1) << (34)`  
  Exclude animation clip ghosts.
- `static constexpr MUint64 kExcludeGreasePencils = MUint64(1) << (35)`  
  Exclude grease-pencil draw.
- `static constexpr MUint64 kExcludeControllers = MUint64(1) << (36)`  
  Exclude Controllers.
- `static constexpr MUint64 kExcludeAll = ~MUint64(0)`  
  Exclude all listed object types.


### MGLFunctionTable

Module `OpenMayaRender`. Source: [class_m_g_l_function_table.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_l_function_table.html).

The best cross platform alternative for drawing in Viewport 2.0 is via MHWRender::MVertexBuffer, MHWRender::MPxGeometryOverride, and other classes providing an abstraction from the underlying hardware API. The MHWRender::MUIDrawManager offers handy options for very simple drawing commands. Drawing in raw OpenGL mode will still be allowed, but we recommend you use the core profile subset of the OpenGL API to ensure compatibility with future versions of Maya. MGLFunctionTable is a utility class which provides wrappers for the basic functions in the OpenGL API. Core functions up to OpenGL 2.0 are provided here, as well as a number of ARB/EXT and vendor specific extensions. Refer to the MGLExtension enumeration for extensions which are checked. Please refer to an OpenGL reference for usage of the OpenGL functions provided in this wrapper class. When using the functions provided, the standard GL* type and constant definitions to be used can be found in MGLDefinitions.h. MGLDefinitions.h basically provides a wrapper for what would normally be found in gl.h and glext.h files. The naming convention used in this file is to take the regular GL* definitions and add a "M" prefix. This is to avoid conflicts with existing type and constant declarations which may be found in files such as gl.h and glext.h. It is recommended that externally provided files which have GL definitions not be included in the same C++ file to avoid conflicts. Mixing GL code wusing MGLFunctionTable and external code is possible, just not recommended. MGLFunctionTable cannot be created on its own, and must be retrieved via a method on the MHardwareRenderer class. It is possible that this class will not be available, if the hardware renderer class cannot be instantiated. This would be due to insufficient graphics hardware support. Below is an example of initializing and using the function table to draw a simple 3-line axis. Note the usage of the definitions from MGLDefinitions such as MGL_LIGHTING versus GL_LIGHTING, and MGL_LINES and MGL_DEPTH_TEST. #include <maya/MHardwareRenderer.h>

**Public Types**

- `enum MGLversion { kMGL_Version11, kMGL_Version12, kMGL_Version121, kMGL_Version13, kMGL_Version14, kMGL_Version15, kMGL_Version20 }`  
  OpenGL versions checked.

**Public Member Functions**

- `bool extensionExists (MGLExtension extension)`  
  Provides information as to whether a given OpenGL extension exists.
- `unsigned int numTexUnits () const`  
  Get information about the maximum number of texture units.
- `unsigned int numTexInterpolants () const`  
  Get information about the maximum number of texture interpolants.
- `unsigned int numTexImageUnits () const`  
  Get information about the maximum number of texture image units available.
- `unsigned int maxTextureSize () const`  
  Get information about the maximum texture size.
- `unsigned int maxVertexAttributes () const`  
  Get information about the maximum number of vertex attributes available.

**Friends**

- `class MHardwareRenderer`


### MGeometry

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry.html).

Class for working with geometric structures used to draw objects. This class provides a means to specify and access geometric data used to draw objects in Viewport 2.0. Objects of type MGeometry cannot be created or destroyed. They are only accessed through methods on draw override classes such as MPxShaderOverride or MPxGeometryOverride. Each MGeometry object contains zero or more vertex buffers and zero or more index buffers. In read-only scenarios, these buffers describe geometry that is about to be drawn and can be used to bind custom shaders to the actual geometry streams on the GPU (see MPxShaderOverride for more details). In the non-read only scenario, MGeometry can be used to provide vertex and index data to Maya to use to draw Maya DAG objects with arbitrary shaders (see MPxGeometryOverride). In all cases the MGeometry instance owns the vertex and index buffers and manages their lifetimes.

**Public Types**

- `enum DataType { kInvalidType, kFloat, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`  
  Specifies data types to use with buffers.
- `enum Semantic { kInvalidSemantic, kPosition, kNormal, kTexture, kColor, kTangent, kBitangent, kTangentWithSign }`  
  Specifies the semantic of a data buffer.
- `enum Primitive { kInvalidPrimitive, kPoints, kLines, kLineStrip, kTriangles, kTriangleStrip, kAdjacentTriangles, kAdjacentTriangleStrip, kAdjacentLines, kAdjacentLineStrip, kPatch }`  
  Specifies the data primitive type constructed by the indexing array.
- `enum DrawMode { kWireframe = (0x1 << 0), kShaded = (0x1 << 1), kTextured = (0x1 << 2), kBoundingBox = (0x1 << 3), kSelectionOnly = (0x1 << 4), kSelectionHighlighting = (0x1 << 5), kAll = (kWireframe | kShaded | kTextured | kBoundingBox) }`  
  Specifies draw mode for render items.

**Public Member Functions**

- `int vertexBufferCount () const`  
  Get the number of vertex buffers contained in this MGeometry object.
- `const MVertexBuffer * vertexBuffer (int index) const`  
  Get the vertex buffer stored at the given index.
- `MVertexBuffer * vertexBuffer (int index)`  
  Get the vertex buffer stored at the given index.
- `MVertexBuffer * createVertexBuffer (const MVertexBufferDescriptor &desc)`  
  Creates a vertex buffer which is owned by this object.
- `bool addVertexBuffer (MVertexBuffer *buffer)`  
  Add a pre-created vertex buffer to this MGeometry object.
- `bool deleteVertexBuffer (int index)`  
  Remove a vertex buffer from this object.
- `int indexBufferCount () const`  
  Get the number of index buffers contained in this MGeometry object.
- `const MIndexBuffer * indexBuffer (int index) const`  
  Get the index buffer stored at the given index.
- `MIndexBuffer * indexBuffer (int index)`  
  Get the index buffer stored at the given index.
- `MIndexBuffer * createIndexBuffer (MGeometry::DataType type)`  
  Creates an index buffer which is owned by this object.
- `bool addIndexBuffer (MIndexBuffer *buffer)`  
  Add a pre-created index buffer to this MGeometry object.
- `bool deleteIndexBuffer (int index)`  
  Remove an index buffer from this object.
- `MGeometry (const MGeometry &)`  
  Constructor.
- `~MGeometry ()`  
  Destructor.

**Static Public Member Functions**

- `static const MString & dataTypeString (MGeometry::DataType d)`  
  Get a logical string name for the data type.
- `static const MString & semanticString (MGeometry::Semantic s)`  
  Get a logical string name for the semantic.
- `static const MString & primitiveString (MGeometry::Primitive p)`  
  Get a logical string name for the primitive.
- `static const MString drawModeString (MGeometry::DrawMode d)`  
  Get a logical string name for the draw mode.
- `static const char * className ()`  
  Returns the name of this class.


### MGeometryData

Module `OpenMayaRender`. Source: [class_m_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_data.html).

This class allows storage of arbitrary data which is formated to be specifically suitable for usage using a 3D display interface such as OpenGL. Format options include: Currently, each element can be of a 1, 2, 3, 4 tuple. Element type can be IEEE single float or double float, signed or unsigned byte (character), 16 or 32 bit signed unsigned integers. There are specific "type" identifiers to provide a semantic on for the data. These include: position : vertex position vector normal : vertex normal vector texCoord : vertextexture coordinate vector color : vertex color vector weight : vertex skin weight vector tangent : vertex tangent vector binormal : vertex binormal vector velocity : vertex velocity vector primitiveCenter : center a primitive (e.g. a triangle) colorMask : colour existance mask. 1:1 correspondance with color useData : some user defined data. A semantic that Maya does not understand. Currently Maya only interprets a fixed format subset for data with recongnized semantics, This does not mean that the user cannot create any arbitrary format for data storage. Support formats with semantics includes: 3 float position interpreted as (x,y,z). 3 float normal interpreted as (x,y,z). 2 float texture coordinate. Coorindate is interpreted as a (U,V) tuple. 4 float color. Color is interpreted as (Red, Green, Blue, Alpha) tuple. 3 float tangent interpreted as (x,y,z). 3 float binormal interpreted as (x,y,z). 3 float velocity interpreted as (x,y,z). 3 float primitive center position interpreted as (x,y,z). 1 float color mask interpreted as 1=mapped, and 0 = unmapped. Memory allocation of the correct size is left up to the user of this class. Memory can be marked as "owned" by this class or the user of this class. Ownership by this class is the default behaviour specified in the constructor. If the data is marked as being owned by the class, it is assumed that the data is created using a "new" operation, as the destructor of this class will use a "delete" operation to free memory. Internal Maya data which is passed to the user via this class is always assumed to be non-modifiable. If modified, stability cannot be ensured.

**Public Types**

- `enum ElementSize { kInvalidElementSize = 0, kOne = 1, kTwo, kThree, kFour }`  
  Specifies the size or dimension of each data element of the storage.
- `enum ElementType { kInvalidElementType = -1, kFloat = 0, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`  
  Specifies the data type of each data element of the storage.
- `enum DataType { kInvalidDataType = 0, kPosition, kNormal, kTexCoord, kColor, kWeight, kAPISupported, kTangent, kBiNormal, kVelocity, kPrimitiveCenter, kColorMask, kUserData, kMaxDataTypeIndex }`  
  Specifies the data type of the storage array.

**Public Member Functions**

- `MGeometryData (const char *dataName, DataType dataType, ElementType elementType, ElementSize elementSize, unsigned int elementCount, void *dataPtr=NULL, bool ownsData=true)`  
  Geometry data constructor.
- `MGeometryData (const MGeometryData &)`  
  Copy constructor.
- `~MGeometryData ()`  
  Class destructor.
- `const char * objectName () const`  
  Return the logical name of the geometry.
- `int uniqueID () const`  
  Return the per session unique identifier.
- `DataType dataType () const`  
  Get the data type for the data.
- `ElementType elementType () const`  
  Returns the data type.
- `unsigned int elementTypeSize () const`  
  Return the element type size.
- `ElementSize elementSize () const`  
  Return element size.
- `unsigned int elementCount () const`  
  Return element count.
- `void * data () const`  
  Retrieve a pointer to the internal data.
- `void setCollectionNumber (int cn)`  
  Set the collection number for the object.
- `int collectionNumber () const`  
  Get the collection number of the data.
- `void setObjectOwnsData (bool val)`  
  Set ownship of the interal data.
- `bool objectOwnsData () const`  
  Return if the MGeometryData object owns the internal data or not.

**Friends**

- `class MGeometryLegacy`
- `class MHardwareRenderer`
- `class MD3D9Renderer`


### MGeometryExtractor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_extractor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_extractor.html).

Class for extracting renderable geometry. Construct an instance of this class to populate buffers with vertex and indexing data. Buffers are filled based on the supplied requirements. Use the MIndexBufferDescriptor and MVertexBufferDescriptor classes to request a broad range of data types, formats, and content. Buffers are filled in-place so you should request the primitive count and vertex count before filling the buffers. Use the count multiplied by the element stride to construct data buffers large enough to fill properly prior to calling one of the populate methods.

**Public Member Functions**

- `MGeometryExtractor (const MHWRender::MGeometryRequirements &requirements, const MDagPath &path,::MHWRender::MPolyGeomOptions options, MStatus *status=NULL)`  
  Constructor.
- `~MGeometryExtractor ()`  
  Destructor.
- `unsigned int primitiveCount (const MHWRender::MIndexBufferDescriptor &indexDesc) const`  
  Returns the number of primitives (triangles, lines, points, etc.) that will be produced for the given indexing requirements.
- `MStatus populateIndexBuffer (void *data, const unsigned int primitiveCount, const MHWRender::MIndexBufferDescriptor &indexDesc) const`  
  Fill a buffer with geometry indexing data.
- `unsigned int vertexCount () const`  
  Returns the number of vertices that will be produced for the vertex requirement.
- `MStatus populateVertexBuffer (void *data, const unsigned int vertexCount, const MHWRender::MVertexBufferDescriptor &bufferDesc) const`  
  Fill a buffer with vertex data.
- `MGeometryExtractor (const MHWRender::MGeometryRequirements &requirements, const MDagPath &path, const bool sharing=true, MStatus *status=NULL)`  
  This method is obsolete.

**Static Public Member Functions**

- `static unsigned int minimumBufferSize (unsigned int primitiveCount, MGeometry::Primitive primitive, unsigned int primitiveStride=0)`  
  Get the minimum buffer size required by populateIndexBuffer().


### MGeometryIndexMapping

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_index_mapping.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_index_mapping.html).

A mapping of geometry index. This class represents the index mapping of the multiple objects concatenated into a single render item. When multiple objects are compatible, their geometry can be consolidated into a single render item, to provide better performance by concatening their index and vertex buffers. The Geometry Index Mapping can be used to get the indexing details of the concatenated geometries to be able to render them separately.

**Public Member Functions**

- `MGeometryIndexMapping ()`  
  Constructor.
- `~MGeometryIndexMapping ()`  
  Destructor.
- `int geometryCount () const`  
  Get the number of geometry described by the mapping.
- `MDagPath dagPath (int geometryIdx) const`  
  Get the MDagPath of a geometry.
- `MObject component (int geometryIdx) const`  
  Get the component of a geometry.
- `int indexStart (int geometryIdx) const`  
  Get the index start of a geometry.
- `int indexLength (int geometryIdx) const`  
  Get the index length of a geometry.
- `int vertexStart (int geometryIdx) const`  
  Get the vertex start of the geometry.
- `int vertexLength (int geometryIdx) const`  
  Get the vertex length of the geometry.
- `MRenderItem * sourceRenderItem (int geometryIdx) const`  
  Get the source render item.
- `void updateSource (int geometryIdx, int indexStart, int indexLength, int vertexStart, int vertexLength)`  
  Update an existing entry in an MGeometryIndexMapping.


### MGeometryLegacy

Module `OpenMayaRender`. Source: [class_m_geometry_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_legacy.html).

MGeometryLegacy stores the collection of MGeometryData arrays which describe a Maya surface, including per-component data such as UV mapping and colour. Various methods are provided for returning MGeometryData containing the data for different properties of the geometry: the position() method returns position data, the normal() method returns normals, and so on. The primitiveArray() method returns MGeometryPrimitive's which provide indexing into those property arrays for each primitive to be drawn.

**Public Member Functions**

- `~MGeometryLegacy ()`  
  Class destructor.
- `MGeometryLegacy (const MGeometryLegacy &geom)`  
  Copy constructor.
- `MGeometryLegacy & operator= (const MGeometryLegacy &rhs)`  
  Copy constructor.
- `unsigned int primitiveArrayCount () const`  
  Get the number of primitive arrays for this surface.
- `const MGeometryPrimitive primitiveArray (unsigned int arrayNumber) const`  
  Get the primitive indexing data for this surface.
- `const MGeometryData position ()`  
  Get the position data for this surface.
- `const MGeometryData normal ()`  
  Get the normal data for this surface.
- `const MGeometryData componentId ()`  
  Get the Maya component id data for this surface.
- `const MGeometryData texCoord (const MString &name)`  
  Get a texture coordinate set by name.
- `const MGeometryData color (const MString &name)`  
  Get a color set by name.
- `const MGeometryData tangent (const MString &name)`  
  Get a tangent set by name.
- `const MGeometryData binormal (const MString &name)`  
  Get a binormal set by name.
- `const MGeometryData data (MGeometryData::DataType what, const MString &name)`  
  Get arbitrary surface data by name.

**Friends**

- `class MGeometryManager`
- `class MGeometryList`
- `class MVaryingParameter`


### MGeometryList

Module `OpenMayaRender`. Source: [class_m_geometry_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_list.html).

This class holds the set of data elements which represent a Maya surface. It provides iterated access to a list of geometry items, along with the rendering context require to render them (e.g. matrix, etc).

**Public Types**

- `enum { kNone = 0, kMatrices = 1 << 0, kFixedFunctionLighting = 1 << 1, kCulling = 1 << 2, kAll = -1 }`  
  Bit flags for the geometry method that govern which OpenGL state Maya sets for you.
- `enum MCullMode { kCullNone, kCullCW, kCullCCW }`  
  Defines the culling modes to use when rendering this geometry.

**Public Member Functions**

- `MGeometryList (const MRenderingInfo &info, int initialCapacity=1)`  
  Create an emtpy GeometryIterator for the specified rendering context.
- `~MGeometryList ()`  
  Class destructor.
- `bool isDone () const`  
  Tests to see if the iterator has reached the end of the elements it contains.
- `void next ()`  
  Advance to the next element in the iterator.
- `void reset ()`  
  Reset this iterator to the first element.
- `int length () const`  
  Query the total number of elements available through this iterator.
- `void setCurrentElement (int i)`  
  Randomly access the ith element in the iterator.
- `MGeometryLegacy & geometry (int setupFlags=kAll)`  
  Get the geometry for the current element in the iterator.
- `const MMatrix & objectToWorldMatrix () const`  
  Get the object to world matrix for the current element in the iterator.
- `const MMatrix & projectionMatrix () const`  
  Get the camera project matrix for the current element in the iterator.
- `const MMatrix & viewMatrix () const`  
  Get the camera view matrix for the current element in the iterator.
- `MDagPath path () const`  
  Get the dag path for the current element in the iterator.
- `MGeometryList::MCullMode cullMode () const`  
  Get the rendering cull mode to use for current element in the iterator.
- `bool addLast (const MDagPath &path, const MObject *components=NULL)`  
  Add the geometry representing a shape to this geometry iterator.
- `bool addLast (const MGeometryLegacy &geometry, const MMatrix &matrix)`  
  Add arbitrary geometry to this geometry iterator.

**Public Attributes**

- `enum MGeometryList:: { ... } MSetupFlags`  
  Bit flags for the geometry method that govern which OpenGL state Maya sets for you.


### MGeometryManager

Module `OpenMayaRender`. Source: [class_m_geometry_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_manager.html).

This class provides methods for managing MGeometry resources. It provides an interface for loading and using hardware textures.

**Public Types**

- `enum GeometricShape { kDefaultSphere = 0, kDefaultPlane, kDefaultCube }`  
  Default geometry shapes.

**Static Public Member Functions**

- `static MGeometryLegacy getGeometry (const MDagPath &shape, const MGeometryRequirementsLegacy &requirements, MObject *components=NULL)`  
  Access the Geometry cache for a shape.
- `static MGeometryList * referenceDefaultGeometry (const MGeometryManager::GeometricShape geomShape, const MGeometryRequirementsLegacy &requirements)`  
  Obtain a reference to geometry for some "default" shapes maintained by the manager such that the geometry returned will match a set of geometric requirements (MGeometryRequirementsLegacy).
- `static MStatus dereferenceDefaultGeometry (MGeometryList *data)`  
  This is the companion method to referenceDefaultGeometry() and must always be called immediately after usage of data supplied by the reference call.
- `static const char * className ()`  
  Returns the name of this class.


### MGeometryPrimitive

Module `OpenMayaRender`. Source: [class_m_geometry_primitive.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_primitive.html).

MGeometryPrimitive is a class describes the topology used for accessing MGeometryData. Topology is specified as a set of index values which references into data elements in an MGeometryData. Index values can be assumed to be stored in contiguous memory. A "draw primitive type" indicates the format of the indexing as follows: kPoint : individual unconnected points. kLine : individual unconnected line segments. kLineStrip : connected line strip. kLineLoop : closed line loop. kTriangle : filled triangle. kTriangleStrip : filled triangle strip. kTriangleFan : filled triangle fan kQuad : filled quadrilateral (quad). kQuadStrip : filled quad strip. kPolygon : filled N-sided polygon Internal Maya data which is passed to the user via this class is always assumed to be non-modifiable. If modified, stability cannot be ensured.

**Public Types**

- `enum DrawPrimitiveType { kInvalidIndexType = 0, kPoints, kLines, kLineStrip, kLineLoop, kTriangles, kTriangleStrip, kTriangleFan, kQuads, kQuadStrip, kPolygon, kMaxDrawPrimitiveTypeIndex }`  
  Specifies the data primitive type constructed by the indexing array.

**Public Member Functions**

- `MGeometryPrimitive (const MGeometryPrimitive &other)`  
  Copy constructor.
- `~MGeometryPrimitive ()`  
  Destructor.
- `int uniqueID () const`  
  Return the per session unique identifier.
- `DrawPrimitiveType drawPrimitiveType () const`  
  Get the data type for the data.
- `unsigned int elementCount () const`  
  Return element count.
- `MGeometryData::ElementType dataType () const`  
  Return element data type.
- `const void * data () const`  
  Retrieve a pointer to the internal data.

**Friends**

- `class MGeometryLegacy`
- `class MVaryingParameter`
- `class MHardwareRenderer`


### MGeometryRequirements

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_requirements.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_requirements.html).

Geometry requirements. This class is used to describe the geometry requirements to be fulfilled by an implementation of MPxGeometryOverride. It represents the union of all vertex requirements from all shaders assigned to the object being evaluated by the MPxGeometryOverride instance. There are two parts to the requirements. First, the vertex requirements are accessed through the vertexRequirements() method. For each such requirement, implementations of MPxGeometryOverride should create a vertex buffer on the MGeometry instance which matches the vertex buffer descriptor from the requirement and fill it with data. Second, the indexing requirements are accessed through the indexingRequirements() method. This list is populated when making geometry requests using classes like MGeometryExtractor.

**Public Member Functions**

- `MGeometryRequirements ()`  
  Constructor.
- `~MGeometryRequirements ()`  
  Destructor.
- `void addVertexRequirement (const MVertexBufferDescriptor &descriptor)`  
  Add a new vertex requirement to the list of vertex requirements.
- `void addIndexingRequirement (const MIndexBufferDescriptor &descriptor)`  
  Add a new indexing requirement to the list of indexing requirements.
- `const MVertexBufferDescriptorList & vertexRequirements () const`  
  Get a descriptor list with descriptors that specify the vertex geometry requirements of this object.
- `const MIndexBufferDescriptorList & indexingRequirements () const`  
  Get a descriptor list with descriptors that specify the geometry indexing requirements of an object.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MGeometryRequirementsLegacy

Module `OpenMayaRender`. Source: [class_m_geometry_requirements_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_requirements_legacy.html).

MGeometryRequirementsLegacy stores the collection of MGeometryRequirementsData arrays which describe a Maya surface, including per-component data such as UV mapping and colour.

**Public Member Functions**

- `MGeometryRequirementsLegacy ()`  
  Default constructor.
- `~MGeometryRequirementsLegacy ()`  
  Destructor.
- `void addPosition (int dimension=3)`  
  Add position to the set of required elements.
- `void addComponentId ()`  
  Add component id to the set of required elements.
- `void addNormal (int dimension=3)`  
  Add normal to the set of required elements.
- `void addTexCoord (const MString &uvSetName)`  
  Add a texture coordinate set to the list of required elements.
- `void addTangent (const MString &uvSetName, int dimension=3)`  
  Add a tangent set to the list of required elements.
- `void addBinormal (const MString &uvSetName, int dimension=3)`  
  Add a binormal set to the list of required elements.
- `void addColor (const MString &colorSetName)`  
  Add a color set to the list of required elements.
- `void addFaceOffsets ()`  
  Add face offsets to the set of required elements.


### MGeometryUtilities

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_utilities.html).

Utilities for Viewport 2.0. This class is a utility class for rendering geometry in Viewport 2.0

**Public Types**

- `enum GeometricShape { kDefaultSphere, kDefaultPlane, kDefaultCube }`  
  Default geometry shapes.

**Static Public Member Functions**

- `static MColor wireframeColor (const MDagPath &path)`  
  Returns the wireframe color used in Viewport 2.0 for the given DAG path.
- `::MHWRender::DisplayStatus displayStatus (const MDagPath &path, MStatus *returnStatus=NULL)`  
  Returns the display status of the given DAG path.
- `static MGeometry * acquireReferenceGeometry (GeometricShape shape, const MGeometryRequirements &requirements)`  
  Acquire reference geometry with required buffers.
- `static void releaseReferenceGeometry (MGeometry *geometry)`  
  Release a generated reference geometry.
- `static const char * className ()`  
  Returns the name of this class.


### MHUDRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_h_u_d_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_h_u_d_render.html).

Class which defines rendering the 2D heads-up-display.

**Public Member Functions**

- `MHUDRender ()`  
  Default constructor for a HUD render operation.
- `~MHUDRender () override`  
  Default destructor for a HUD render operation.
- `const MString & name () const override`  
  Name for a HUD render operation.
- `virtual bool hasUIDrawables () const`  
  addUIdrawables() will be called only if this method is overridden to return true.
- `virtual void addUIDrawables (MUIDrawManager &drawManager2D, const MFrameContext &frameContext)`  
  Provides access to the 2D version of MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.


### MHardwareRenderer

Module `OpenMayaRender`. Source: [class_m_hardware_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hardware_renderer.html).

Static hardware renderer interface class. MHardwareRenderer is an interface class which wraps the hardware renderer.

**Public Types**

- `enum DrawProcedureStatusCode { kSuccess = 0, kFailure, kItemExists, kItemNotFound, kLocationNotFound }`  
  Draw Procedure status codes.
- `enum BufferPixelFormat { kRGBA_Fix8, kRGBA_Float16, kDepth_Float32 }`  
  Specify the pixel format of the current buffer.
- `enum CallLocation { kPreRendering = 0, kPreExposure = 1, kPostExposure = 2, kPostRendering = 3 }`  
  Draw Procedure call locations.
- `enum GeometricShape { kDefaultSphere = 0, kDefaultPlane, kDefaultCube }`  
  Default geometry shapes.

**Public Member Functions**

- `const MString & backEndString () const`  
  This method is obsolete.
- `DrawProcedureStatusCode addDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedureptr, CallLocation location)`  
  This method is obsolete.
- `DrawProcedureStatusCode removeDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedureptr, CallLocation location)`  
  This method is obsolete.
- `DrawProcedureStatusCode insertDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedurePtr, CallLocation location, unsigned int listIndex, bool moveExistingItem=false)`  
  This method is obsolete.
- `DrawProcedureStatusCode getDrawProcedureCount (const MString &backEndString, CallLocation location, unsigned int &count) const`  
  This method is obsolete.
- `DrawProcedureStatusCode getDrawProcedureListNames (const MString &backEndString, CallLocation location, MStringArray &namesOfProcs) const`  
  This method is obsolete.
- `MDrawProcedureBase * findDrawProcedure (const MString &backEndString, CallLocation location, const MString &procName) const`  
  This method is obsolete.
- `MStatus makeResourceContextCurrent (const MString &backEndString)`  
  This method is obsolete.
- `MStatus getBufferSize (const MString &backEndString, unsigned int &width, unsigned int &height) const`  
  This method is obsolete.
- `MStatus getColorBufferPixelFormat (const MString &backEndString, BufferPixelFormat &fmt) const`  
  This method is obsolete.
- `MStatus getDepthBufferPixelFormat (const MString &backEndString, BufferPixelFormat &fmt) const`  
  This method is obsolete.
- `MStatus getCurrentExposureNumber (const MString &backEndString, unsigned int &number) const`  
  This method returns the current exposure number during rendering.
- `MStatus getTotalExposureCount (const MString &backEndString, unsigned int &number) const`  
  This method returns the total number of exposures for rendering a single frame.
- `MStatus restoreCurrent (const MString &backEndString) const`  
  This method is obsolete.
- `MStatus makeSwatchContextCurrent (const MString &backEndString, unsigned int &width, unsigned int &height)`  
  The swatch context is a "scratch" context which can be used to hardware render an image for swatch display.
- `MStatus readSwatchContextPixels (const MString &backEndString, MImage &image)`  
  Read the contents of the swatch context into an MImage.
- `MGeometryData * referenceDefaultGeometry (MHardwareRenderer::GeometricShape geomShape, unsigned int &numberOfData, unsigned int *&pIndexing, unsigned int &indexLength)`  
  This method is obsolete.
- `MStatus dereferenceGeometry (MGeometryData *pGeomData, unsigned int numberOfData)`  
  This method is obsolete.
- `void getSwatchOrthoCameraSetting (double &l, double &r, double &b, double &t, double &n, double &f) const`  
  This method returns an orthographics camera clipping plane values which form the camera viewing volume.
- `void getSwatchPerspectiveCameraSetting (double &l, double &r, double &b, double &t, double &n, double &f) const`  
  This method returns a perspective camera clipping plane values which form the camera viewing volume.
- `void getSwatchPerspectiveCameraTranslation (float &x, float &y, float &z, float &w) const`  
  This method returns a perspective camera 3D translation values.
- `void getSwatchLightDirection (float &x, float &y, float &z, float &w) const`  
  This method returns the direction vector of the default directional light used for rendering the swatch.
- `void drawSwatchBackGroundQuads (const MColor &quadColor=kDefaultQuadColor, bool textured=false, unsigned int numberOfRepeats=8) const`  
  This draws a set of quads in a checkboard pattern.
- `MGLFunctionTable * glFunctionTable () const`  
  This method is obsolete.

**Static Public Member Functions**

- `static MHardwareRenderer * theRenderer ()`  
  This method is obsolete.


### MHwTextureManager

Module `OpenMayaRender`. Source: [class_m_hw_texture_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hw_texture_manager.html).

The MHwTextureManager provides an interface for loading and using hardware textures.

**Static Public Member Functions**

- `static MStatus glBind (const MObject &fileTextureObject, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`  
  Bind the contents of a file texture node to the currently active OpenGL texture unit.
- `static MStatus glBind (const MPlug &fileTextureConnection, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`  
  Bind the contents of a specific file texture node attribute (e.g.
- `static MStatus registerTextureFile (const MString &fileName, MHwTextureFileHandle &hTexture)`  
  Register a named texture file.
- `static MStatus deregisterTextureFile (const MHwTextureFileHandle &hTexture)`  
  Deregisters the given handle to a texture entry in the manager.
- `static MStatus textureFile (const MHwTextureFileHandle &hTexture, MString &fileName)`  
  Return the file name associated with a given texture file handle.
- `static MStatus glBind (const MHwTextureFileHandle &hTexture, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`  
  Bind the contents of a specific file to the currently active OpenGL texture unit.
- `static const char * className ()`  
  Returns the name of this class.


### MHwrCallback

Module `OpenMayaRender`. Source: [class_m_hwr_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hwr_callback.html).

Rendering Callbacks. This class is used to register callbacks to gain access to Maya's Hardware Rendering device status. You can be notified of device creation, lost reset and deletion. To register callbacks, inherit from this class and override deviceNew, deviceLost, deviceReset, deviceDeleted. Any number of these methods can be overridden by the callback. Then register the callbacks by calling the addCallback() method. If multiple callbacks need to be registered, the order of invocation can be set by adding each callback with a priority number, 0 being the highest priority.

**Public Member Functions**

- `MHwrCallback ()`  
  Default Constructor.
- `virtual ~MHwrCallback ()`  
  Default Destructor.
- `virtual void deviceNew ()`  
  Method to override for being notified of new device creation.
- `virtual void deviceLost ()`  
  Method to override for being notified of device Lost.
- `virtual void deviceReset ()`  
  Method to override for being notified of device Reset.
- `virtual void deviceDeleted ()`  
  Method to override for being notified of device Deletion.

**Static Public Member Functions**

- `static void addCallback (MHwrCallback *, int priority=0)`  
  Static procedure to add a rendering callback.
- `static void removeCallback (MHwrCallback *)`  
  Static procedure to remove a rendering callback.


### MIndexBuffer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer.html).

Index buffer for use with MGeometry. This class represents an index buffer with a specific data type. When retrieving an index buffer for binding to custom shaders (MPxShaderOverride), resourceHandle() may be called to get the device dependent handle to the index buffer on the GPU. When creating an index buffer to supply geometric data to Maya (MPxGeometryOverride), acquire() may be called to get a pointer to a block of memory to fill with said data. Once filled, commit() must be called to apply the data to the buffer.

**Public Member Functions**

- `MIndexBuffer (MGeometry::DataType type, bool softwareStaged=true)`  
  Constructor.
- `MIndexBuffer (const MIndexBuffer &)`  
  Constructor.
- `~MIndexBuffer ()`  
  Destructor.
- `MGeometry::DataType dataType () const`  
  Get the data type of the buffer.
- `unsigned int size () const`  
  Get the size of the buffer in units of dataType().
- `void * resourceHandle () const`  
  Gets a graphics device dependent handle to the vertex indexing data.
- `void resourceHandle (void *handle, unsigned int size)`  
  Directly set the graphics-device-dependent hardware buffer resource handle.
- `bool hasCustomResourceHandle () const`  
  Returns true if this index buffer is using a custom resource handle set by the plugin using MIndexBuffer::resourceHandle(void*, unsigned int).
- `void lockResourceHandle ()`  
  Lock the resource handle.
- `void unlockResourceHandle ()`  
  Unlock the resource handle.
- `void * acquire (unsigned int size, bool writeOnly)`  
  Get a pointer to memory for the buffer.
- `void commit (void *buffer)`  
  Commit the data stored in the memory given by acquire() to the buffer.
- `void * map ()`  
  Get a read-only pointer to the existing content of the buffer.
- `void unmap ()`  
  Release the data exposed by map().
- `MStatus update (const void *buffer, unsigned int destOffset, unsigned int numIndices, bool truncateIfSmaller)`  
  Set a portion (or all) of the contents of the MIndexBuffer using the data in the provided software buffer.
- `void unload ()`  
  If the buffer is resident in GPU memory, calling this method will move it to system memory and free the GPU memory.
- `void * acquire (unsigned int size)`  
  This method is obsolete.
- `void * getIndexBuffer () const`  
  Private friend class only access to the internal OGS index buffer.
- `void setIndexBuffer (void *ib)`  
  Private friend class only to set the internal OGS index buffer.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MIndexBufferDescriptor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer_descriptor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer_descriptor.html).

MIndexBufferDescriptor describes an indexing scheme. This class represents a description of an indexing scheme. It provides the indexing type (including custom named types), primitive type, primitive stride (for use with patch primitives), and component information.

**Public Member Functions**

- `MIndexBufferDescriptor ()`  
  Constructor.
- `MIndexBufferDescriptor (MIndexBufferDescriptor::IndexType type, const MString &name, MGeometry::Primitive primitive, unsigned int primitiveStride=0, MObject component=MObject::kNullObj, MGeometry::DataType dataType=MGeometry::kUnsignedInt32)`  
  Constructor with arguments.
- `~MIndexBufferDescriptor ()`  
  Destructor.
- `MString name () const`  
  Get the name used to describe the type when the type() returns kCustom.
- `void setName (MString &val)`  
  Set the name used to describe the type when the type() is set to kCustom.
- `MIndexBufferDescriptor::IndexType indexType () const`  
  Get the indexing type describing what the buffer is used for.
- `void setIndexType (MIndexBufferDescriptor::IndexType val)`  
  Set the indexing type describing what the buffer is used for.
- `MGeometry::Primitive primitive () const`  
  Get the primitive describing the input layout for each drawable.
- `void setPrimitive (MGeometry::Primitive val)`  
  Set the primitive describing the input layout for each drawable.
- `unsigned int primitiveStride () const`  
  Get the number of points per primitive.
- `void setPrimitiveStride (unsigned int id)`  
  Set the number of control points used for patch primitives.
- `MObject component () const`  
  Get the components associated with the index buffer.
- `void setComponent (MObject component)`  
  Set the components to use when building the index buffer.
- `MGeometry::DataType dataType () const`  
  Get the type of data expected to be in the index buffer.
- `void dataType (MGeometry::DataType dataType)`  
  Set the type of data expected to be in the index buffer.


### MIndexBufferDescriptorList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer_descriptor_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer_descriptor_list.html).

A list of MIndexBufferDescriptor objects. A list of MIndexBufferDescriptor objects.

**Public Member Functions**

- `MIndexBufferDescriptorList ()`  
  Constructor.
- `~MIndexBufferDescriptorList ()`  
  Destructor.
- `int length () const`  
  Get the number of descriptors in the list.
- `bool getDescriptor (int index, MIndexBufferDescriptor &desc) const`  
  Gets the descriptor at the requested index and copies data into desc.
- `bool append (const MIndexBufferDescriptor &desc)`  
  Add a descriptor to the list.
- `bool removeAt (int index)`  
  Remove a descriptor from the list and delete it.
- `void clear ()`  
  Clears the list.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MIntersection

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_intersection.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_intersection.html).

Describes the intersection of a selection hit. This class gives a description of an intersection when a selection hit occurs.

**Public Member Functions**

- `MSelectionContext::SelectionLevel selectionLevel (MStatus *ReturnStatus=nullptr) const`  
  Get the selection level of the intersection.
- `int index (MStatus *ReturnStatus=nullptr) const`  
  Get the index of the hit vertex, edge or triangle.
- `MStatus barycentricCoordinates (float &a, float &b) const`  
  Get the barycentric coordinates.
- `float edgeInterpolantValue (MStatus *ReturnStatus=nullptr) const`  
  Get the edge interpolant value.
- `MFloatPoint intersectionPoint (MStatus *ReturnStatus=nullptr) const`  
  Get the intersection point in object space.
- `int instanceID (MStatus *ReturnStatus=nullptr) const`  
  Get the draw instance ID of the render item.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MLightLinks

Module `OpenMayaRender`. Source: [class_m_light_links.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_light_links.html).

Class providing read-only Light Linking API functions. This class provides read-only access to the light linking information expressed in Maya's light linker nodes. Usage To extract Maya's light linking information, use the following sequence of calls: Call parseLinks() once. MLightLinks will examine the connections to the scene's light linker nodes, and compile the information into a table of light-object links. parseLinks() needs to be recalled whenever the scene's light linking changes (objects/lights are added or deleted, or light links are made or broken). Call getLinkedLights() or getLinkedObjects() to query the lights linked to a particular object, or the objects (and components) linked to a particular light. Lights can be linked either to DAG hierarchies or to shading groups. Additionnally, light linking information can store a list of illuminating lights and a list excluded lights. The algorithm used to determine which light should contribute can be described by the pseudo code below. If the entire object is all in one shading group, then only consider object-level light links. The code to compute lightList will look like: MObject nullObj; // A void component object

**Public Member Functions**

- `MLightLinks ()`  
  Constructor.
- `~MLightLinks ()`  
  Destructor.
- `MStatus parseLinks (const MObject &linkNode=MObject::kNullObj, bool verbose=false, std::ostream *stream=NULL, bool useIgnore=false, bool componentSupport=true)`  
  Compiles the information contained in a light linker node into a table of object/light links.
- `MStatus getLinkedLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`  
  Queries the light link table to find the lights that are linked to a particular object or object component.
- `MStatus getLinkedObjects (const MDagPath &light, MSelectionList &objects)`  
  Queries the light link table to find the objects or object components that are linked to a particular light.
- `MStatus getIgnoredLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`  
  Queries the light link table to find the lights that ignores a particular object or object component.
- `MStatus getIgnoredObjects (const MDagPath &light, MSelectionList &objects)`  
  Queries the light link table to find the objects or object components that are ignored to a particular light.
- `MStatus getShadowLinkedLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`  
  Queries the light link table to find the lights that are shadow linked to a particular object or object component.
- `MStatus getShadowLinkedObjects (const MDagPath &light, MSelectionList &objects)`  
  Queries the light link table to find the objects or object components that are shadow linked to a particular light.
- `MStatus getShadowIgnoredLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`  
  Queries the light link table to find the lights that ignore a particular object or object component when casting shadows.
- `MStatus getShadowIgnoredObjects (const MDagPath &light, MSelectionList &objects)`  
  Queries the light link table to find the objects or object components that are ignored to a particular light when calculating shadows.


### MLightParameterInformation

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_light_parameter_information.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_light_parameter_information.html).

A class for providing lighting information that may be used with Viewport 2.0. This class allows for access to various per-light information accessible via the MDrawContext class in Viewport 2.0.

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture2, kSampler, kTextureCube }`  
  Specifies light parameter data types.
- `enum StockParameterSemantic { kNoSemantic, kLightEnabled, kWorldPosition, kWorldDirection, kIntensity, kColor, kEmitsDiffuse, kEmitsSpecular, kDecayRate, kDropoff, kCosConeAngle, kStartShadowParameters , kShadowMap, kShadowSamp, kShadowBias, kShadowMapSize, kShadowViewProj, kShadowColor, kGlobalShadowOn, kShadowOn, kShadowDirty, kDepthRange }`  
  Specifies semantic meanings for predefined names for common or "stock" light parameters.

**Public Member Functions**

- `void parameterList (MStringArray &list) const`  
  Get the names of all light parameters that are accessible.
- `ParameterType parameterType (const MString &parameterName) const`  
  Get the type of the named parameter, returns kInvalid if parameter is not found.
- `StockParameterSemantic parameterSemantic (const MString &parameterName) const`  
  Get the stock semantic for a named parameter.
- `void parameterNames (StockParameterSemantic semantic, MStringArray &list) const`  
  Get the name of all parameters on the light which are tagged with the stock semantic.
- `unsigned int arrayParameterCount (const MString &parameterName) const`  
  Return the array size of a parameter.
- `MStatus getParameter (const MString &parameterName, MIntArray &value)`  
  Get a named boolean or integer parameter.
- `MStatus getParameter (const MString &parameterName, MFloatArray &value)`  
  Get a named float parameter.
- `MStatus getParameter (const MString &parameterName, MMatrix &value)`  
  Get a named matrix parameter.
- `MStatus getParameter (const MString &parameterName, MSamplerStateDesc &samplerDesc)`  
  Get a description of a sampler parameter.
- `void * getParameterTextureHandle (const MString &parameterName)`  
  Get a resource handle for a texture parameter.
- `MStatus getParameter (const MString &parameterName, MTextureAssignment &value)`  
  Get a named texture parameter.
- `MStatus getParameter (StockParameterSemantic semantic, MIntArray &value)`  
  Get a boolean or integer parameter by semantic.
- `MStatus getParameter (StockParameterSemantic semantic, MFloatArray &value)`  
  Get a float parameter by semantic.
- `MStatus getParameter (StockParameterSemantic semantic, MMatrix &value)`  
  Get a matrix parameter by semantic.
- `MStatus getParameter (StockParameterSemantic semantic, MSamplerStateDesc &samplerDesc)`  
  Get a sampler parameter by semantic.
- `void * getParameterTextureHandle (StockParameterSemantic semantic)`  
  Get a resource handle for a texture parameter by semantic.
- `MStatus getParameter (StockParameterSemantic semantic, MTextureAssignment &value)`  
  Get a texture parameter by semantic.
- `MString lightType () const`  
  Get the classification of the light node.
- `MDagPath lightPath (MStatus *ReturnStatus=NULL) const`  
  Returns the DagPath to the scene light.

**Protected Member Functions**

- `~MLightParameterInformation ()`  
  Destructor.


### MPassContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_pass_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_pass_context.html).

Class to allow access to pass context information. MPassContext provides access to current pass context information. The term "pass context" is used to denote a logical breakdown of the render loop within a single frame to render. A "pass" denotes the highest level elements of the breakdown. Each pass has a unique identifier and a list of semantics. The identifier can either be an internally specified identifier or the name of a custom scene or user render operation (MSceneRender, MUserRenderOperation respectively). The semantics provide a description of what is being rendered. The list of semantics is ordered, from first to last element, such that the first element provides the broadest description and additional elements refine (provides additional details) about the previous element. The following is a list of semantics which may be returned as the first element of the semantic list. Possible refining semantics are also described. A "color pass" semantic implies rendering for color output. Rendering for a color pass can be further broken down into the follow logical categorizations: Color drawing of opaque surfaces. For this, an additional semantic can describe if a material override is being used. Color drawing of transparent surfaces. For this, a breakdown of whether the drawing is occurring with back or front-face culling may be specified. Color drawing of the UI for surfaces. A "shadow pass" semantic implies rendering to create a shadow map. A "depth" pass semantic implies rendering for depth output only and not color. A "depth and normal" semantic implies rendering of depth and object normals as output. A "user pass" semantic implies that a user operation (MUserRenderOperation) is the current operation being executed for the active render override (MRenderOverride) As an example, the semantic list returned may contain these elements: [ color pass, opaque surface drawing, material override ] This indicates that the active pass is drawing to a color output. Also that the render items being draw are opaque, and that those render items are being drawing with a material override. As a second example, the semantic list returned may contain these elements: [ color pass, transparent surface drawing, back-face culling ] This means that the active pass is drawing to a color output. Also that the render items being drawn are transparent, and that those render items are being drawing with back-face culling enabled.

**Public Member Functions**

- `const MString & passIdentifier () const`  
  Return the identifier for the pass context.
- `const MStringArray & passSemantics () const`  
  Return an array of semantics for the pass context.
- `bool hasShaderOverride () const`  
  Return if there is a shader instance override set for the current pass.
- `MShaderInstance * shaderOverrideInstance () const`  
  Return the shader instance override set for the current pass.

**Static Public Attributes**

- `static const MString kColorPassSemantic`  
  Semantic for a color pass.
- `static const MString kShadowPassSemantic`  
  Semantic for a shadow pass.
- `static const MString kPointLightShadowPassSemantic`  
  Semantic for a point light shadow pass.
- `static const MString kDepthPassSemantic`  
  Semantic for a depth pass, which pre-renders an intermediate normalized depth target and a linear depth texture that can be used for depth testing during later passes.
- `static const MString kNormalDepthPassSemantic`  
  Semantic for a normal and depth pass, whose result will be used for SSAO.
- `static const MString kOpaqueGeometrySemantic`  
  Semantic to indicate opaque surfaces are being drawn.
- `static const MString kPreUIGeometrySemantic`  
  Semantic to indicate pre-scene UI is being drawn. E.g. The grid.
- `static const MString kPostUIGeometrySemantic`  
  Semantic to indicate post-scene UI for surfaces is being drawn. e.g. the wireframe or components.
- `static const MString kUIGeometrySemantic`  
  Semantic to indicate UI for surfaces is being drawn. e.g. the wireframe or components.
- `static const MString kOpaqueUISemantic`  
  Semantic for opaque UI.
- `static const MString kTransparentUISemantic`  
  Semantic for transparent UI.
- `static const MString kXrayUISemantic`  
  Semantic for UI in xray mode.
- `static const MString kTransparentGeometrySemantic`  
  Semantic to indicate transparent surfaces are being drawn.
- `static const MString kCullBackSemantic`  
  Semantic to indicate that back-face culling is being applied.
- `static const MString kCullFrontSemantic`  
  Semantic to indicate that front-face culling is being applied.
- `static const MString kMaterialOverrideSemantic`  
  Semantic to indicate that a material override is being used.
- `static const MString kTransparentPeelSemantic`  
  Semantic for a transparent peel pass.
- `static const MString kTransparentPeelAndAvgSemantic`  
  Semantic for a transparent peel-and-average pass.
- `static const MString kTransparentWeightedAvgSemantic`  
  Semantic for a transparent weighted-average pass.
- `static const MString kUserPassSemantic`  
  Semantic to indicate a user defined pass (MUserRenderOperation)
- `static const MString kBeginRenderSemantic`  
  Semantic to indicate the start of rendering. See MRenderer::addNotification() for usage.
- `static const MString kBeginSceneRenderSemantic`  
  Semantic to indicate the beginning of scene draw pass. See MRenderer::addNotification() for usage.
- `static const MString kEndSceneRenderSemantic`  
  Semantic to indicate the end of a scene draw pass. See MRenderer::addNotification() for usage.
- `static const MString kEndRenderSemantic`  
  Semantic to indicate the end of rendering. See MRenderer::addNotification() for usage.
- `static const MString kSelectionPassSemantic`  
  Semantic for a selection pass.
- `static const MString kDOFPassSemantic`  
  Semantic for depth-of-field pass.
- `static const MString kMotionVectorPassSemantic`  
  Semantic for motion vector pass, whose result will be used for motion blur.
- `static const MString kPEPatternPassSemantic`  
  Semantic for post-effect pattern pass, which fills an intermediate depth target with depth values of surfaces included in post effects. The depth target will be used for depth test in non-post-effect pattern pass later.
- `static const MString kNonPEPatternPassSemantic`  
  Semantic for non-post-effect pattern pass, which produces alpha mask for surfaces excluded from post effects in order to composite the beauty pass output without post effects over the one with post effects later.


### MPresentTarget

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_present_target.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_present_target.html).

Class which defines the operation of presenting a target for final output.

**Public Types**

- `enum MTargetBackBuffer { kCenterBuffer, kLeftBuffer, kRightBuffer }`  
  Supported output target back-buffer options.

**Public Member Functions**

- `MPresentTarget (const MString &name)`  
  Constructor for a named target presentation operation.
- `~MPresentTarget () override`  
  Default destructor for target presentation operation.
- `bool presentDepth () const`  
  Query whether the present operation will display depth values.
- `void setPresentDepth (bool val)`  
  Set whether the operation will present depth values.
- `bool enableSRGBWrite () override`  
  The present operation never enables SRGB.
- `MTargetBackBuffer targetBackBuffer () const`  
  Query the desired back-buffer to use on the output target.
- `void setTargetBackBuffer (MTargetBackBuffer backBuffer)`  
  Set the desired back-buffer to use on the output target.

**Protected Attributes**

- `MTargetBackBuffer mTargetBackBuffer`  
  Back-buffer of output target to render to.
- `bool mPresentDepth`  
  Present depth.


### MPxComponentConverter

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_component_converter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_component_converter.html).

Base class for user defined component converter. Implementations of MPxComponentConverter must be registered with Maya through MDrawRegistry. MPxComponentConverters are registered with a unique render item name. A registered MPxComponentConverter will be used to perform custom viewport 2.0 selection. They are used to convert selection intersection data to a list of object components (MFnComponent), by matching the index buffer positions (MIntersection::index()) to valid component ids. The same converter can be registered against different render item name, meaning that different render items can share the same component converter.

**Public Member Functions**

- `MPxComponentConverter ()`  
  Construct a MPxComponentConverter.
- `virtual ~MPxComponentConverter ()`  
  Destructor.
- `virtual void initialize (const MRenderItem &renderItem)=0`  
  Maya calls this function to allow the converter to initialize itself for the selection on the given render item.
- `virtual void addIntersection (MIntersection &intersection)=0`  
  Maya calls this function for every selection hit on the render item.
- `virtual MObject component ()=0`  
  Once all of the geometry hits have been passed to the converter through calls to addIntersection(), Maya will call this method to retrieve the components corresponding to those hits.
- `virtual MSelectionMask selectionMask () const =0`  
  Maya calls this function to allow the converter to specify the type of components it can handle.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxDrawOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_draw_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_draw_override.html).

Base class for user defined drawing of nodes. MPxDrawOverride allows the user to define custom draw code to be used to draw all instances of a specific DAG object type in Maya when using Viewport 2.0. transform() should always return the world space transformation matrix for the object to be drawn in the custom draw code. The default implementation simply returns the transformation defined by the parent transform nodes. isBounded() can return true or false to indicate whether the object is bounded or not respectively. If a false value is returned then camera frustum culling will not be performed against the bounding box. In this situation the boundingBox() method will not be called since the bounding box is not required. boundingBox() should always return the object space bounding box for whatever is to be drawn in the custom draw code. If the bounding box is incorrect the node may be culled at the wrong time and the custom draw method might not be called. When the object associated with the draw override changes, prepareForDraw() is called which allows the user to pull data from Maya to be used in the draw phase. It is invalid to query attribute values from Maya nodes during the draw callback and doing so may result in instability. The method excludedFromPostEffects() allows for the plug-in to indicate whether to contribute to passes which are required for post effects. If so, the plug-in's draw() may be called multiple times, once for each pass. Note that even without overriding this method that the plug-in's draw can be called more than once (e.g. for a shadow map pass). The method isTransparent() allows for the plug-in to indicate whether the object should be drawn as transparency. If so, the plug-in's draw() will be called two times, one for front-culling pass and one for back-culling pass. It will never be called into weighted average or depth peeling. Note that without overriding this method or make the method return false means the object will only be drawn once in opaque pass. addUIDrawables() provides access to the MUIDrawManager, which can be used to queue up operations for drawing simple UI elements such as lines, circles and text. It is called just after prepareForDraw() and carries the same restrictions on the sorts of operations it can perform. To enable addUIDrawables(), override hasUIDrawables() and make it return true. UI elements are not affected by post effects, and hence are not affected by the excludedFromPostEffects() return value. At draw time, the user defined callback will be invoked at which point any custom OpenGL drawing may occur. Data needed from the Maya dependency graph must have been cached in the prepareForDraw() stage as it is invalid to query such data during the draw callback. Implementations of MPxDrawOverride must be registered with Maya through MDrawRegistry. If the registration contains "/includePostEffects" this will override the value returned from excludedFromPostEffects(). If the registration contains "/isTransparent" this will override the value returned from isTransparent(). For example, a classification starts with "/drawdb/geometry/testNode/includePostEffects/isTransparent" means testNode should contribute to passes required for post effect and it is a transparent object for draw. When using the MHWRender::MRenderer::setGeometryDrawDirty() interface to mark a DAG object associated with an MPxDrawOverride object dirty, the optional topologyChanged parameter must be set to true.

**Public Types**

- `typedef void(* GeometryDrawOverrideCb) (const MDrawContext &, const MUserData *)`  
  User draw callback definition, draw context and blind user data are parameters.

**Public Member Functions**

- `MPxDrawOverride (const MObject &obj, GeometryDrawOverrideCb callback, bool isAlwaysDirty=true)`  
  Construct an MPxDrawOverride.
- `virtual ~MPxDrawOverride ()`  
  Destructor.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool hasUIDrawables () const`  
  In order for any override for the addUIDrawables() method to be called this method must also be overridden to return true.
- `virtual MMatrix transform (const MDagPath &objPath, const MDagPath &cameraPath) const`  
  Called by Maya whenever the world space transform is needed for the object to be drawn by the draw callback.
- `virtual MBoundingBox boundingBox (const MDagPath &objPath, const MDagPath &cameraPath) const`  
  Called by Maya whenever the bounding box of the drawable object is needed.
- `virtual bool isBounded (const MDagPath &objPath, const MDagPath &cameraPath) const`  
  Called by Maya to determine if the drawable object is bounded or not.
- `virtual bool disableInternalBoundingBoxDraw () const`  
  Indicates whether to disable the automatic drawing of bounding boxes when the display mode has been set to bounding box display.
- `virtual bool excludedFromPostEffects () const`  
  Indicates whether or not the draw code should be called for any additional passes required to perform post effects.
- `virtual bool isTransparent () const`  
  Indicates whether or not the draw method should be called for each transparency pass(front-culling and back-culling).
- `virtual MUserData * prepareForDraw (const MDagPath &objPath, const MDagPath &cameraPath, const MFrameContext &frameContext, MUserData *oldData)=0`  
  Called by Maya whenever the object is dirty and needs to update for draw.
- `virtual void addUIDrawables (const MDagPath &objPath, MUIDrawManager &drawManager, const MFrameContext &frameContext, const MUserData *data)`  
  Provides access to the MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.
- `virtual bool refineSelectionPath (const MSelectionInfo &selectInfo, const MRenderItem &hitItem, MDagPath &path, MObject &geomComponents, MSelectionMask &objectMask)`  
  Maya calls this function during the selection interpretation phase of Viewport 2.0 selection and can be used to override the selected path, the selected components or simply reject the selection.
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`  
  Maya calls this function during the pre-filtering phase of Viewport 2.0 selection and is used to setup the selection context of the given DAG object.
- `virtual bool traceCallSequence () const`  
  This method allows a way for a plug-in to examine the basic call sequence for a draw override.
- `virtual void handleTraceMessage (const MString &message) const`  
  When debug tracing is enabled via MPxDrawOverride::traceCallSequence(), this method will be called for each trace message.
- `virtual bool wantUserSelection () const`  
  This method is called during the hit test phase of Viewport 2.0 selection and is used to indicate whether or not the userSelect() method should be called to override the default hit test implementation for the associated DAG object.
- `virtual bool userSelect (const MSelectionInfo &selectInfo, const MDrawContext &context, const MDagPath &objPath, const MUserData *data, MSelectionList &selectionList, MPointArray &worldSpaceHitPts)`  
  This method is called during the hit test phase of Viewport 2.0 selection if wantUserSelection() returns true, in order to override the default hit test implementation for the associated DAG object.
- `virtual bool userSelect (MSelectionInfo &selectInfo, const MDrawContext &context, MPoint &hitPoint, const MUserData *data)`  
  This method is obsolete.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus setGeometryForRenderItem (MRenderItem &renderItem, const MVertexBufferArray &vertexBuffers, const MIndexBuffer *indexBuffer, const MBoundingBox *objectBox) const`  
  Call this method to provide the geometry for a render item.
- `virtual void invokeTheCallback (MDrawContext &drawContext, MUserData *userData)`  
  This callback is to allow a Python script to be called instead of the C++ function.

**Static Protected Member Functions**

- `static MStatus drawRenderItem (const MHWRender::MDrawContext &context, MRenderItem &item)`  
  Call this method to draw a render item constructed for the MPxDrawOverride.
- `static bool pointSnappingActive ()`  
  This utility function can be called by a draw override to query whether Viewport 2.0 selection has been launched to find points for snapping.


### MPxGeometryOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_geometry_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_geometry_override.html).

Base for user-defined classes to prepare geometry for drawing. MPxGeometryOverride allows the user to create an override to prepare vertex data that will be used to draw a specific Maya DAG object type with an arbitrary shader (standard Maya or custom) in Viewport 2.0. This class is designed to be a high level data-translator independent of any specific hardware draw API. Once registered through MDrawRegistry, an instance will be created for each node with a matching classification string. That instance will be used to generate the vertex streams needed by the assigned shaders in order to draw the object. The intent of this class is that it be used to provide data for plugin shape types (MPxSurfaceShape) however it can also be used to override the geometry translation for any Maya geometry type. If a more low-level interface to the Viewport 2.0 draw loop is needed, look at either MPxDrawOverride or MPxShaderOverride. The override is only triggered when the associated DAG object has changed and that object is about to be drawn. So it is not invoked if the user is simply tumbling the scene, or if the object is not within the current view frustum. Implementations of MPxGeometryOverride must be registered with Maya through MDrawRegistry. Users of this interface must implement several virtual methods which will be called at specific times during the draw-preparation phase. 1) updateDG() : In the updateDG() call, all data needed to compute the indexing and geometry data must be pulled from Maya and cached. It is invalid to query attribute values from Maya nodes in any later stage and doing so may result in instability. 2) updateRenderItems() : For each shader assigned to the instance of the object Maya will assign a render item. A render item is a single atomic renderable entity containing a shader and some geometry. In updateRenderItems(), implementations of this class may enable or disable the automatic shader-based render items and they may add or remove custom user defined render items in order to cause additional things to be drawn. Look at the MRenderItem interface for more details. 3) addUIDrawables() : For each instance of the object, besides the render items updated in 'updateRenderItems()' for the geometry rendering, there is also a render item list there for render the simple UI elements. 'addUIDrawables()' happens just after normal geometry item updating, The original design for this stage is to allow user accessing 'MUIDrawManager' which helps drawing the simple geometry easily like line, circle, rectangle, text, etc. Overriding this methods is not always necessary, but if you want to override it, please also override 'hasUIDrawables()' to make it return true or the overrided method will not be called. 4) populateGeometry() : In this method the implementation is expected to fill the MGeometry data structure with the vertex and index buffers required to draw the object as indicated by the data in the geometry requirements instance passed to this method. Failure to fulfill the geometry requirements may result in incorrect drawing or possibly complete failure to draw the object. 5) cleanUp() : Delete any cached data generated in the earlier phases that is no longer needed. Additionally, this interface includes some optional virtual methods which may be implemented to extend the functionality of MPxGeometryOverride. 1) requiresGeometryUpdate() : If this method returns true then all of the other methods will be called for this draw-preparation phase. This allows your MPxGeometryOverride to depend on data outside of associated DAG object. By default this method returns false. 2) requiresUpdateRenderItems() : If this method returns false for an MDagPath instance of the associated DAG object then updateRenderItems() will not be called for that MDagPath draw-preparation phase. If requiresUpdateRenderItems() returns false for all MDagPaths then updateRenderItems() will not be called at all for the draw-preparation phase. 3) supportsEvaluationManagerParallelUpdate() : If this method returns true then the MPxGeometryOverride will be considered for Evaluation Manager Parallel Update. If the MPxGeometryOverride object returns false from hasUIDrawables and requiresUpdateRenderItems during the draw-preparation phase then the MPxGeometryOverride is eligible for Evaluation Manager Parallel Update during that draw-preparation phase. Evaluation Manager Parallel Update is a performance optimization which allows the updateDG() and populateGeometry() step of the draw update to occur during Evaluation Graph Execution rather than during VP2 scene rendering. The improves performance by allowing updateDG() and populateGeometry() calls of several plug-ins to run in parallel with each other and with Evaluation Manager dependency graph evaluation. Using Evaluation Manager Parallel Update incurs some additional overhead updating each object. For drawable objects where updateDG() and populateGeometry() calls are short (such as for UI objects) it may be faster to not support Evaluation Manager Parallel Update. Enabling Evaluation Manager Parallel Update for MPxGeometryOverride does add some additional restrictions to the implementation of updateDG() and populateGeometry(). When Evaluation Manager Parallel Update is enabled, updateDG() and populateGeometry() may be called from a worker thread which may not safely access the graphics API. To populate geometry you must use the methods on MIndexBuffer and MVertexBuffer which explicitly state they are safe to use on a worker thread. Furthermore, updateDG() and populateGeometry() may only access values on the MObject associated with the MPxGeometryOverride. Accessing values on other objects will result in undefined behavior. Using Evaluation Manager Parallel Update with Evaluation Caching enabled adds additional limitations on Evaluation Manager Parallel Update. During Evaluation Manager Parallel Update it is only safe to read attributes which are not animated, or attributes which are animated and cached. For more information on how to ensure attributes are cached with Evaluation Caching please refer to the Evaluation Caching documentation. MPxGeometryOverride automatically supports Evaluation Caching. No changes to MPxGeometryOverride are necessary to enable Evaluation Caching support. If MPxGeometryOverride is attached to an user-defined node then some changes to the user-defined node may be necessary for Evaluation Caching to work correct with the user-defined node and MPxGeometryOverride. Specifically, all values read by MPxGeometryOverride must be cached by the user-defined node. MPxGeometryOverride includes optional support for VP2 Custom Caching. To support VP2 Custom Caching a node must also support Evaluation Manager Parallel Update. When VP2 Custom Caching is supported three new potential behaviors will occur. 1) During evaluation when the VP2 Custom Cache is not ready the MPxGeometryOverride will be invoked as normal. After populateGeometry() the VP2 Custom Cache will save references to the MVertexBuffers and MIndexBuffers used to draw the object for the current time. It is not safe to hold references to these buffers after returning from populateGometry(). 2) During evaluation when the VP2 Custom Cache is ready a subset of the MPxGeomtryOverride methods will be invoked. The plug-in will have a chance to choose which render items draw and to add UI render items (hasUIDrawable(), updateRenderItems(), addUIDrawables() will all be called). However, no updateDG() or populateGeometry() call will be made. The pre-existing buffers from an earlier evaluation will be used to draw. 3) When Maya is idle Maya will fill the VP2 Custom Cache using background evaluation. updateDG(), populateGeometry() and cleanUp() will be invoked in a worker thread which does not have the normal context as the current context. During background evaluation there are additional restrictions on updateDG(), populateGeometry() and cleanUp(): it is not safe to access the graphics API and it is only safe to access values on the MObject associated with the MPxGeometryOverride. After the populateGeometry() call the MVertexBuffers and MIndexBuffers prepared will be stored. It is not safe to hold references to these buffers after returning from populateGeometry(). When MPxGeometryOverride is invoked from background evaluation for VP2 Custom Caching the current context will already be set correctly for the frame being background evaluated. When VP2 Custom Caching is enabled and the desired content of the MVertexBuffers used to draw the MPxGeometryOverride is changing then requiresGeometryUpdate() must return true.

**Public Member Functions**

- `MPxGeometryOverride (const MObject &obj)`  
  Construct an MPxGeometryOverride.
- `virtual ~MPxGeometryOverride ()`  
  Destructor.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool hasUIDrawables () const`  
  In order for any override for the addUIDrawables() method to be called this method must also be overridden to return true.
- `virtual bool requiresGeometryUpdate () const`  
  This method is called one during each draw-preparation phase.
- `virtual bool requiresUpdateRenderItems (const MDagPath &path) const`  
  This method is called for each instance of the associated DAG object whenever the object changes.
- `virtual bool supportsEvaluationManagerParallelUpdate () const`  
  This method is called once each MPxGeometryOverride in the scene to determine if the MPxGeometryOverride is eligible for Evaluation Manager Parallel Update.
- `virtual bool supportsVP2CustomCaching () const`  
  This method is called at any time during evaluation to determine if VP2 Custom Caching is supported.
- `virtual void configCache (const MEvaluationNode &, MCacheSchema &) const`  
  Defines the node's behavior when participating in Cached Playback in VP2 Custom Caching.
- `virtual void updateDG ()=0`  
  Perform any work required to translate the geometry data that needs to get information from the dependency graph.
- `virtual bool isIndexingDirty (const MRenderItem &item)`  
  This method is called for each render item on the assocated DAG object whenever the object changes.
- `virtual bool isStreamDirty (const MVertexBufferDescriptor &desc)`  
  This method is called for each geometry stream on the assocated DAG object whenever the object changes.
- `virtual void updateRenderItems (const MDagPath &path, MRenderItemList &list)=0`  
  This method is called for each instance of the associated DAG object whenever the object changes.
- `virtual void addUIDrawables (const MDagPath &path, MUIDrawManager &drawManager, const MFrameContext &frameContext)`  
  For each instance of the object, besides the render items updated in updateRenderItems() there is also a render item list for rendering simple UI elements.
- `virtual void populateGeometry (const MGeometryRequirements &requirements, const MRenderItemList &renderItems, MGeometry &data)=0`  
  Implementations of this method should create and populate vertex and index buffers on the MGeometry instance 'data' in order to fulfill all of the geometry requirements defined by the 'requirements' parameter.
- `virtual void cleanUp ()=0`  
  Called after all other stages are completed.
- `virtual bool refineSelectionPath (const MSelectionInfo &selectInfo, const MRenderItem &hitItem, MDagPath &multipath, MObject &geomComponents, MSelectionMask &objectMask)`  
  Maya calls this function during the selection interpretation phase of Viewport 2.0 selection and can be used to override the selected path, the selected components or simply reject the selection.
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`  
  Maya calls this function during the pre-filtering phase of Viewport 2.0 selection and is used to setup the selection context of the given DAG object.
- `virtual bool traceCallSequence () const`  
  This method allows a way for a plug-in to examine the basic call sequence for a geometry override.
- `virtual void handleTraceMessage (const MString &message) const`  
  When debug tracing is enabled via MPxGeometryOverride::traceCallSequence(), this method will be called for each trace message.
- `const MFrameContext * getFrameContext () const`  
  Returns the frame context.

**Static Public Member Functions**

- `static bool pointSnappingActive ()`  
  Returns true if selection has been launched to find snap points.
- `static const char * className ()`  
  Returns the name of this class.


### MPxImagePlaneOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_image_plane_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_image_plane_override.html).

Base class for user defined image plane draw overrides.

**Public Member Functions**

- `MPxImagePlaneOverride (const MObject &obj)`  
  Construct an MPxImagePlaneOverride.
- `virtual ~MPxImagePlaneOverride ()`  
  Destructor.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `MString getFileName (bool useImageSequence)`  
  This method can get the exact file name of image sequence or just return the original name if input is not sequence.
- `virtual void updateDG ()`  
  This method is called every time Maya needs to update the parameter which will later be used in method updateTexture().
- `virtual void updateColorTexture ()`  
  This method is used for update texture which will be used to draw image plane.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxIndexBufferMutator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_index_buffer_mutator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_index_buffer_mutator.html).

Base class for user defined index buffer mutators. Implementations of MPxIndexBufferMutator must be registered with Maya through MDrawRegistry. MPxIndexBufferMutators are registered with a unique mutator type name. The name signifies the name of the custom index buffer the mutator can produce. When an MPxShaderOverride or custom renderer uses an effect with custom index buffer mutator they can provide a MIndexBufferDescriptor with the desired mutator name. A registered MPxIndexBufferMutator will be used to mutate the existing index buffer.

**Public Member Functions**

- `MPxIndexBufferMutator ()`  
  Construct a MPxIndexBufferMutator.
- `virtual ~MPxIndexBufferMutator ()`  
  Destructor.
- `virtual MGeometry::Primitive mutateIndexing (const MHWRender::MComponentDataIndexingList &sourceIndexBuffers, const MVertexBufferArray &vertexBuffers, MIndexBuffer &indexBuffer, int &primitiveStride) const =0`  
  This method gets called to allow the generator to mutate the data for a custom index stream using information stored in the vertex buffers.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxPrimitiveGenerator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_primitive_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_primitive_generator.html).

Base class for user defined primitive generators. Implementations of MPxPrimitiveGenerator must be registered with Maya through MDrawRegistry. MPxPrimitiveGenerators are registered with a unique primitive type name. The name signifies the name of the custom primitive the generator can produce. When an MPxShaderOverride or custom renderer uses an effect with custom input primitive they can provide a MIndexBufferDescriptor with the desired custom primitive name. A registered MPxPrimitiveGenerator will be used to produce the custom primitive.

**Public Member Functions**

- `MPxPrimitiveGenerator ()`  
  Construct a MPxPrimitiveGenerator.
- `virtual ~MPxPrimitiveGenerator ()`  
  Destructor.
- `virtual unsigned int computeIndexCount (const MObject &object, const MObject &component) const =0`  
  This function is called to allow the primitive generator to provide the number of vertices it will use.
- `virtual MGeometry::Primitive generateIndexing (const MObject &object, const MObject &component, const MComponentDataIndexingList &sourceIndexing, const MComponentDataIndexingList &targetIndexing, MIndexBuffer &indexBuffer, int &primitiveStride) const =0`  
  This method gets called to allow the generator to fill in the data for a custom index stream.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxRenderPassImpl

Module `OpenMayaRender`. Source: [class_m_px_render_pass_impl.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_render_pass_impl.html).

Render pass implementation. The MPxRenderPassImpl class is an abstract base class which provides a common interface for pass implementations. Extend this class to create custom pass implementations.

**Public Types**

- `enum PassTypeBit { kUInt8 = 0x1, kUInt16 = 0x2, kUInt32 = 0x4, kUInt64 = 0x8, kInt8 = 0x10, kInt16 = 0x20, kInt32 = 0x40, kInt64 = 0x80, kFloat16 = 0x100, kFloat32 = 0x200, kFloat64 = 0x400, kBit = 0x800, kOther = 0x1000 }`  
  Data types that implementation may support.
- `enum PassSemantic { kInvalidSemantic = 0, kColorSemantic, kVectorSemantic, kDirectionVectorSemantic, kDepthSemantic, kLabelSemantic, kMaskSemantic, kOtherSemantic }`  
  Description of the nature of the data stored in the frame buffer.
- `typedef unsigned int PassTypeMask`  
  Use to create a mask of PassTypeBit types.

**Public Member Functions**

- `MPxRenderPassImpl ()`  
  Default constructor.
- `virtual ~MPxRenderPassImpl ()`  
  Destructor.
- `virtual bool isCompatible (const MString &renderer) const =0`  
  Called by Maya check whether this pass implementation is compatible with the given renderer.
- `virtual PassTypeMask typesSupported () const =0`  
  Called by Maya to determine which types are supported by this pass.
- `virtual PassTypeBit getDefaultType () const =0`  
  Called by Maya to determine the default type for this pass.
- `virtual unsigned int getNumChannels () const =0`  
  Called by Maya to get the number of channels supported by this pass.
- `virtual PassSemantic frameBufferSemantic () const =0`  
  Called by Maya to get the frame buffer semantic.
- `virtual bool perLightPassContributionSupported () const =0`  
  Called by Maya to determine if this pass implementation supports per-light contributions defined by pass contribution maps.


### MPxRenderer

Module `OpenMayaRender`. Source: [class_m_px_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer.html).

Base class for plugin renderers. A base class providing an interface through which a plugin can implement a renderer to provide rendered images of a scene. LIMITATION: Currently this class is only used for rendering into the Material Viewer in the Hypershade editor. For renderer integration into the Render View, please refer to the API class MRenderView and the MEL command "renderer". For rendering of shader swatches, please refer to the API class MSwatchRenderBase. The derived class needs to be registered and deregistered with Maya using calls to MFnPlugin::registerRenderer() and MFnPlugin::deregisterRenderer(). Translation of the scene data to render is done by Maya calling the corresponding translate and set methods. The translate methods (translateMesh(), translateLightSource(), translateCamera(), translateTransform(), translateShader(), translateEnvironment) should implement translation of a Maya object to a corresponding renderer specific object. When Maya calls translate or set methods it is always done in a thread-safe context. This means that the Maya handle sent in (and associated data) is valid and thread-safe for the duration of the call, but not guaranteed to be valid or thread-safe after the call, so the renderer should not save the Maya handle. If it needs to save the handle, it should do so using an MObjectHandle and never save the MObject directly. If the handle is saved it is up to the renderer to make sure the data is accessed in a thread-safe way. When Maya translates a shape object (mesh, light, camera), the call to the translate method is always followed by one or more calls to translateTransform(), which instantiates the shape and assigns it as a child to one or more transform nodes. Calls to all translate and set methods are always preceded by a call to beginSceneUpdate(). This is to let the renderer know that scene updates are about to happen, to let it cancel the current rendering or otherwise prepare for scene updates. The update is then followed by a call to endSceneUpdate(), to let the renderer know that no more updates are comming (until next beginSceneUpdate()). The renderer can then do any scene data post-processing and restart the rendering. Calls to translate and set methods can occur before a render job is started, for initial scene translation. But it can also occur during rendering for scene changes. So the renderer must support these methods being called at any time. However they are always encapsulated by calls to beginSceneUpdate() and endSceneUpdate(). The translate methods are also used to update already existing objects. So if a translate method is called with an uuid pointing to an already translated object, that means the object should be updated. A render session is started by Maya calling startAsync(). This call should start a render control thread running asynchronously from Maya's main thread. The control thread is in turn allowed to spawn as many render worker threads as given by the JobParams sent in. During rendering the refresh() method should be called to send new image data back to Maya. The progress() method could be called to report the progress of the current image. While the job is running it is up to the renderer to decide when the rendered image is finished. When finished the render worker threads should terminate or sleep, and not cause unnecessary CPU usage. If scene updates occur the control thread should restart the rendering to produce a new image with the new scene data. The job should be running in this manner until a call to stopAsync() is received. At that point both worker threads and the control thread should terminate. A call to destroyScene() should clear out the scene and destroy all allocated scene data. The steps below outlines the events for a render job session and which calls Maya sends to the renderer. A scene update consists of a block of calls, named a Scene Update Block below: beginSceneUpdate() translateMesh() translateTransform() setProperty() setShader() ... endSceneUpdate() 1. Maya will start the render session with 0 or more Scene Update Blocks. The renderer should use them to build its internal representation for the scene graph. 2. Maya will call startAsync(). The renderer should start a separate thread. 3. The renderer should start rendering the internal scene graph that it now has, in the separate thread or by starting other worker threads. 4. The renderer should call progress() to indicate to Maya how far along it is. A value between 0.0 and 1.0 indicates the rendering is in progress. A value of 1.0 or higher indicates the rendering is completed. A negative value indicates the task is done and scene graph destroyed (see point 12). 5. The renderer should call refresh() during rendering to present successive image results that Maya can display. 6. When the rendering is completed the renderer should drop into a waiting state. Notification of this is done by calling progress() with a value of 1.0 or higher. 7. Maya may modify the scene graph through additional Scene Update Blocks. If these occur while the renderer is in its waiting state then it must start up a new render. If these occur while the renderer is already rendering, it must incorporate the changes into the current render, typically by restarting it. 8. Steps 3 through 7 may be repeated multiple times. 9. Maya calls stopAsync(). The renderer should terminate its separate threads but retains its internal scene graph. 10. Steps 1 through 9 may be repeated to produce renderings of variations of the same scene. 11. Maya calls destroyScene(). The renderer should destroy its internal scene graph. 12. The renderer should then finish by calling progress() with a negative value, to let Maya know the scene graph is destroyed and the renderer is shut down.

**Classes**

- `class JobParams`  
  Parameters for starting a new job.
- `class ProgressParams`  
  Parameters for reporting progress of a task.
- `class RefreshParams`  
  Parameters for doing refresh of the view when new image data is available.

**Public Types**

- `enum EnvironmentType { IBL_ENVIRONMENT }`  
  Identifier for environment types.
- `enum JobType { SWATCH_RENDER_JOB }`  
  Identifier for job types.

**Public Member Functions**

- `virtual ~MPxRenderer ()`  
  Class destructor.
- `virtual MStatus startAsync (const JobParams &params)`  
  Start an asynchronous render job session.
- `virtual MStatus stopAsync ()`  
  Stop an asynchronous render job session.
- `virtual bool isRunningAsync ()`  
  Returns true if an asynchronous job is running.
- `virtual MStatus beginSceneUpdate ()`  
  Notify the renderer that scene updates are about to happen.
- `virtual MStatus translateMesh (const MUuid &id, const MObject &node)`  
  Translate a Maya mesh node to a corresponding renderer specific object.
- `virtual MStatus translateLightSource (const MUuid &id, const MObject &node)`  
  Translate a Maya light source node to a corresponding renderer specific object.
- `virtual MStatus translateCamera (const MUuid &id, const MObject &node)`  
  Translate a Maya camera node to a corresponding renderer specific object.
- `virtual MStatus translateEnvironment (const MUuid &id, EnvironmentType type)`  
  Translate an environment of given type to a corresponding renderer specific object.
- `virtual MStatus translateTransform (const MUuid &id, const MUuid &childId, const MMatrix &matrix)`  
  Translate a Maya transform to a corresponding renderer specific object.
- `virtual MStatus translateShader (const MUuid &id, const MObject &node)`  
  Translate a Maya shader node to a corresponding renderer specific object.
- `virtual MStatus setProperty (const MUuid &id, const MString &name, bool value)`  
  Set a boolean property value on a node.
- `virtual MStatus setProperty (const MUuid &id, const MString &name, int value)`  
  Set an integer property value on a node.
- `virtual MStatus setProperty (const MUuid &id, const MString &name, float value)`  
  Set a float property value on a node.
- `virtual MStatus setProperty (const MUuid &id, const MString &name, const MString &value)`  
  Set a string property value on a node.
- `virtual MStatus setShader (const MUuid &id, const MUuid &shaderId)`  
  Assign a shader to a node.
- `virtual MStatus setResolution (unsigned int width, unsigned int height)`  
  Sets rendering resolution.
- `virtual MStatus endSceneUpdate ()`  
  Notify the renderer that scene update is completed.
- `virtual MStatus destroyScene ()`  
  A call to this method should clear out the scene and destroy all allocated scene data.
- `virtual bool isSafeToUnload ()=0`  
  Returns true if it is safe to unload the plugin for this renderer.

**Protected Member Functions**

- `MPxRenderer ()`  
  Class constructor.
- `void refresh (const RefreshParams &params)`  
  Send new image data to Maya.
- `void progress (const ProgressParams &params)`  
  This should be called during rendering to notify Maya about the progress of a task.


### MPxRenderer::JobParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_job_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_job_params.html).

Parameters for starting a new job.

**Public Attributes**

- `JobType type`  
  Type of job to start.
- `MString description`  
  Job description.
- `unsigned int maxThreads`  
  Maximum number of threads allowed for the job.
- `unsigned int priority`  
  Hint of the priority for this job.
- `MUuid cameraId`  
  Id of camera transform node, for the render camera.


### MPxRenderer::ProgressParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_progress_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_progress_params.html).

Parameters for reporting progress of a task.

**Public Attributes**

- `MString description`  
  Description of current task.
- `float progress`  
  Progess of current task.


### MPxRenderer::RefreshParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_refresh_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_refresh_params.html).

Parameters for doing refresh of the view when new image data is available.

**Public Attributes**

- `unsigned int width`  
  Width of the image frame.
- `unsigned int height`  
  Height of the image frame.
- `unsigned int left`  
  Left edge of the image tile.
- `unsigned int right`  
  Right edge of the image tile.
- `unsigned int bottom`  
  Bottom edge of the image tile.
- `unsigned int top`  
  Top edge of the image tile.
- `unsigned int channels`  
  Number of channels in the image data.
- `unsigned int bytesPerChannel`  
  Size of each channel in bytes.
- `void * data`  
  The image data.


### MPxShaderOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_shader_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_shader_override.html).

Base class for user defined shading effect draw overrides. MPxShaderOverride allows the user to create a custom override for associating a "full shading effect" with a shading node (custom or standard) in Maya. Its primary use is for associating hardware effects with pre-existing plugin software shaders. A "full shading effect" defines the complete shading and lighting involved to render a given object. Input resources for shading such as geometry, textures, and lights are defined and bound to the shading effect via the override as required. The override is fully responsible for these tasks. As an example, for hardware shading, this can be thought of as implementing a CgFx or HLSL effect file renderer which can use the resources defined within a Maya scene. There are three main phases that the override must implement: 1) Initialization Phase : This phase occurs when Maya determines that the hardware shader generated through this override needs to be rebuilt. This may be due to a new object using the shader, or because something has changed on the shader that requires a complete rebuild. For example a value change with rebuildAlways() returning true will require a complete rebuild. During the initialization phase the geometric stream requirements can be specified. If an MShaderInstance is used as the shader for rendering then setGeometryRequirements() should be called to extract the requirements from the MShaderInstance. Otherwise, addGeometryRequirement() or addGeometryRequirements() should be used. The requirements will determine which geometric streams are required from objects to which the given shading effect is assigned. If no requirements are specified then a single position stream requirement will be used. The initialize() method must return a string representing the shader key. It often happens that different instances of the MPxShaderOverride represent essentially the same shader, but with different shader parameters. The shader key is used to identify the MPxShaderOverride instances representing the same shader. To optimize rendering, the Viewport 2.0 will make an effort to regroup the rendering of objects with the same MPxShaderOverride shader key. This allows the plug-in to perform its setup only once for the entire sequence. It is up to each plug-in to decide what the meaning of representing the same shader is. There are two possible versions of initialize() to implement (however it is only necessary to implement one, the default implementation of the second calls the first). The first simply receives the shader node that is being initialized. The second receives additional context (such as the DAG path for the object bound to the shader) and also allows the implementation to send back custom user data to attach to the MRenderItem representing the DAG-object/shader association. The custom user data must be derived from MUserData. If the deleteAfterUse() method returns true for the MUserData instance then the data object will automatically be deleted when the render item is deleted. Otherwise the lifetime of the user data object is the responsibility of the caller. This user data may be accessed in the draw phase from the render item being drawn. Note that Maya objects which have render items that have custom user data can only be consolidated when the user data pointer on each item references the same user data instance. Draw performance will suffer without consolidation, however if the user data object is shared then memory management must be handled manually (ie. deleteAfterUse() should return false). During initialization, if the current display mode is non-textured display mode, an internally defined static shader instance is used for all render items which use the shading node associated with a given shader override. This is a performance optimization to avoid any additional node monitoring as well as allow for render items which share this shader instance to be consolidated. If it is desirable to override this behavior, the nonTexturedShaderInstance() method can be overridden to return a custom shader instance to be specified instead of the default shared instance. If this shader instance needs to be updated on node attribute changes, then a return parameter can be set to indicate this. If no monitoring is required Maya will attempt to skip the update phase while in non-textured mode. It is still possible that update is required for the shader which used for textured mode display. For example if rebuildAlways() returns true then the update phase would be called regardless of the options set in this method. The handling of properties related to the non-textured shader instance such as transparency are not directly tied to the update phases isTransparent() method. That method is for the status of textured mode display shader. The implementation can control transparency by directly calling the MShaderInstance::setIsTransparent() with the appropriate value. If this needs to be done at shader initialization time then the call should be made within the nonTexturedShaderInstance() method before returning the shader instance. 2) Data Update Phase : In this phase, updating of all data values required for shading is performed. The interface has an explicit split of when the dependency graph can be accessed (updateDG()), and when the draw API (e.g. OpenGL) can be accessed (updateDevice()). Any intermediate data can be cleaned up when endUpdate() is called. As an example the override may require input from an attribute on a given node. updateDG() would evaluate the plug for the attribute and cache its value temporarily on the override. updateDevice() would take the temporarily cached value and bind it to a shader stored on the graphic device. endUpdate() would reset the temporary data on the override. If an MShaderInstance is being used, then the parameters on that instance should be updated during updateDevice(). Note that the override can provide a draw hint as to whether shading will involve semi-transparency. This hint can be provided by overriding the isTransparent() method which gets called between updateDevice() and endUpdate(). 3) Drawing Phase : The actual drawing using the shader is performed in the pure virtual draw() method. The callback method should return true if it was able to draw successfully. If false is returned then drawing will be done using the default shader used for unsupported materials. Drawing is explicitly not intermixed with the data update on purpose. By the time draw is being called all evaluation should have been completed. If there is user data that needs to be passed between the update and drawing phases the override must cache that data itself. It is an error to access the Maya dependency graph during draw and attempts to do so may result in instability. Although it is possible for implementations of the draw() method to handle all shader setup and geometry draw, the expected use of the draw() method is to do shader setup only. Then drawGeometry() can be called to allow Maya to handle the actual geometry drawing. If manual geometry binding is required however, it is possible to query the hardware resource handles through the geometry on each render item in the render item list passed into the draw() method. The handlesDraw() method will be invoked at the beginning of the draw phase to allow the plugin to indicate whether it will handle drawing based on the current draw context information. If this method returns false then shaderInstance(), activateKey(), draw() and terminateKey() will not be called. The default implementation will return true if the shader is being used for a color pass and there is no per-frame shader override specified. An example of when such a shader override is used is when all objects in the scene are set to draw with the "default material". The activateKey() and terminateKey() method will also be invoked in the draw phase each time a render item is drawn with a different shader key (see the discussion above about shader keys). The activateKey() and terminateKey() methods can be used to optimize rendering by configuring the rendering state only once for a batch of draw() calls that are sharing the same shader key. The sequence of invocations will look like: shaderOverrideA->activateKey(...) shaderOverrideA->draw(...) shaderOverrideB->draw(...) shaderOverrideC->draw(...) ... shaderOverrideA->terminateKey(...) shaderOverrideX->activateKey(...) Note that the terminateKey() callback is always invoked on the same MPxShaderOverride instance as the one used to invoked the activateKey() callback. If an MShaderInstance is being used for drawing, in order to take advatange of the batch drawing optimization, the shaderInstance() method should return the shader instance to use. The shader instance should then be bound during activateKey() and unbound during terminateKey(). An MShaderInstance can be bound and unbound from within the draw() method but will not be optimized for batch drawing. The sequence using an MShaderInstance and the batch optimization will look like: shaderOverrideA->shaderInstance(...) shaderOverrideA->activateKey(...) shaderOverrideA->draw(...) shaderOverrideB->draw(...) shaderOverrideC->draw(...) ... shaderOverrideA->terminateKey(...) shaderOverrideX->shaderInstance(...) shaderOverrideX->activateKey(...) Note if full draw control is desired, the proxy class MPxDrawOverride may be more appropriate. Implementations of MPxShaderOverride must be registered with Maya through MDrawRegistry.

**Classes**

- `struct MInitContext`  
  Initialization context used by advanced initalization method.
- `struct MInitFeedback`  
  Data to pass back to Maya after initialization.

**Public Member Functions**

- `MPxShaderOverride (const MObject &obj)`  
  Construct an MPxShaderOverride.
- `virtual ~MPxShaderOverride ()`  
  Destructor.
- `virtual MString initialize (MObject shader)`  
  Initialization occurs when Maya determines that the hardware shader needs to be rebuilt.
- `virtual MString initialize (const MInitContext &initContext, MInitFeedback &initFeedback)`  
  This method is obsolete.
- `virtual MString initialize (const MInitContext &initContext, MSharedPtr< MUserData > &userData)`  
  Initialization occurs when Maya determines that the hardware shader needs to be rebuilt.
- `virtual void updateDG (MObject object)`  
  This is the first part of the update phase.
- `virtual void updateDevice ()`  
  This is the second part of the update phase.
- `virtual void endUpdate ()`  
  This is the final part of the update phase.
- `virtual bool handlesDraw (MDrawContext &context)`  
  This method indicates whether the shader will handle the drawing based on the context passed in.
- `virtual void activateKey (MDrawContext &context, const MString &key)`  
  This is the activateKey callback.
- `virtual bool draw (MDrawContext &context, const MRenderItemList &renderItemList) const =0`  
  This is the draw callback, the method is called during the draw phase.
- `virtual void terminateKey (MDrawContext &context, const MString &key)`  
  This is the terminateKey callback.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool isTransparent ()`  
  During the update phase the override will be called to return whether it will be drawing with semi-transparency.
- `virtual bool supportsAdvancedTransparency () const`  
  During the update phase the override will be called to return whether it supports advanced transparency algorithms (such as depth peeling).
- `virtual bool overridesDrawState ()`  
  During the draw phase this method will be called to determine whether the override will override the draw state when drawing.
- `virtual bool rebuildAlways ()`  
  If this method returns true, it will force shader and geometry data to be rebuilt on any change to the shader.
- `virtual double boundingBoxExtraScale () const`  
  Override this method to supply an extra scale factor to be applied to the object space bounding box of any objects which use the shader associated with this override.
- `virtual bool overridesNonMaterialItems () const`  
  Override this method if the shader instance should also be used to render non material items such as the wireframe and the selected edges/vertices components.
- `virtual bool handlesConsolidatedGeometry () const`  
  Override this method if the shader instance should disable the consolidation for the geometry it is applied to.
- `virtual bool requiresUnsharedGeometricStreams () const`  
  Override this method if the shader instance requires the unshared geometric streams.
- `virtual bool supportsMultiDraw () const`  
  Override this method if the shader supports MultiDraw rendering.
- `virtual MHWRender::MShaderInstance * shaderInstance (MDrawContext &context) const`  
  Override this method if a shader instance (MShaderInstance) is to be used for drawing.
- `virtual MHWRender::MShaderInstance * nonTexturedShaderInstance (bool &monitorNode) const`  
  Return an override shader instance to be used when drawing in non-textured mode.
- `virtual MHWRender::MShaderInstance * multiDrawNonTexturedShaderInstance (bool &monitorNode) const`  
  Return an override shader instance to be used when drawing MultiDraw non-textured render items.
- `virtual void activateKey (MDrawContext &context)`  
  This method is obsolete.
- `virtual void terminateKey (MDrawContext &context)`  
  This method is obsolete.
- `virtual MHWRender::MShaderInstance * shaderInstance () const`  
  This method is obsolete.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Protected Member Functions**

- `MStatus addGeometryRequirement (const MVertexBufferDescriptor &desc)`  
  During the initialization phase the geometry requirements for the shading effect can be updated.
- `MStatus addGeometryRequirements (const MVertexBufferDescriptorList &list)`  
  During the initialization phase the geometry requirements for the shading effect can be updated.
- `MStatus setGeometryRequirements (const MShaderInstance &shaderInstance)`  
  During the initialization phase the geometry requirements for the shading effect can be updated.
- `MStatus addShaderSignature (void *signature, size_t signatureSize)`  
  During the initialization phase, the "signature" for the shader may be set.
- `MStatus addShaderSignature (const MShaderInstance &shaderInsance)`  
  During the initialization phase, the "signature" for the shader may be set.
- `MStatus addIndexingRequirement (const MIndexBufferDescriptor &desc)`  
  During the initialization phase the indexing requirements for the shading effect can be updated.
- `void drawGeometry (const MDrawContext &context) const`  
  This method may be called from draw() and will cause Maya to immediately draw the current geometry using the current state of the draw API.


### MPxShadingNodeOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_shading_node_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_shading_node_override.html).

Base class for user defined shading node overrides. MPxShadingNodeOverride allows the user to specify how a plugin shading node in Maya should interact with other shading nodes in Viewport 2.0. Specifically this class lets the user inform Maya which shading fragment (or fragment graph) to use to represent the node in the Viewport 2.0 shading graph. Using this system, plugin nodes may function seamlessly with internal nodes as well as other plugin nodes from unrelated plugins (both geometry and shaders). MPxShadingNodeOverride differs from MPxShaderOverride in that implementations of MPxShaderOverride are required to produce the whole shading effect for a shader graph (including lighting) while MPxShadingNodeOverride is only required to produce a small fragment for an individual node. Shade fragments and graphs are managed by MFragmentManager. New fragments and/or graphs may be defined using XML and registered with Maya through MFragmentManager. These fragments and graphs may then be referenced by implementations of MPxShadingNodeOverride. For the purposes of MPxShadingNodeOverride, fragments and fragment graphs can be used interchangeably. A fragment graph is merely a special case of a fragment. It still has input and output parameters it is just composed of other fragments instead of directly defining shading code itself. In the following descriptions assume that any reference to a fragment can also refer to a fragment graph. Implementations of MPxShadingNodeOverride have two main responsibilities. One is to define the fragment to use and how the parameters of that fragment are related to the attributes on the Maya node. The second (optional) responsibility is to manually set the values of the parameters for the fragment on the final shader when the associated Maya shading node changes. When Maya needs to create a shading effect for a particular shading node graph, it will create a shading fragment for each node and connect them together to form the full effect. Any node in the node graph whose node type is associated with an implementation of MPxShadingNodeOverride will use the shade fragment named by the fragmentName() method. This will happen any time the Maya shading node graph changes in such a way as to require a rebuild of the full effect (for example, if connections are changed). The parameters on the shading fragment are automatically driven by attributes on the Maya node which have matching names and types. The plugin may also specify associations between attributes and parameters of the same type but with different names by overriding the getCustomMappings() function. This method is called immediately after the fragment is created and before the automatic mappings are done. No automatic mapping will be created for any parameter on the fragment that already has a custom mapping. All functionality is driven through these attribute parameter mappings. When Maya is traversing the shading node graph building and connecting fragments it will only traverse connections where the input attribute on the node has a defined mapping (custom or automatic). Also, as fragments are combined for all the nodes in the Maya shading graph, their parameters are renamed in order to avoid collisions (allowing the same fragment type to be used multiple times in a graph). Only parameters with mappings will be renamed, all others may suffer name collisions which will produce unpredictable results. In addition to informing Maya about the relationship between attributes and parameters, custom mappings can be used to prevent Maya from trying to connect other fragments to a particular parameter or to prevent a parameter from being renamed (name collisions become the responsibility of the user). Custom mappings may also be used to tell Maya to rename a parameter to avoid name collisions but not to associate it with any attribute (when the attribute name of the mapping is the empty string). The values for such a parameter must be set manually in updateShader(). This can be useful when a parameter is not directly driven by an attribute but must still be set with a computed value. An implementation of MPxShadingNodeOverride which specifies a shade fragment to use along with optional custom mappings will function well in the Viewport 2.0 shading system. The parameter values on the final effect will automatically be set to the values of the attributes on the Maya node whenever the Maya node changes. However, if additional control is required, the implementation can also override the updateDG() and updateShader() methods. These two methods will be called when Maya needs to update the parameter values on the final shader. In updateDG() the implementation should query and cache any information it needs from the Maya dependency graph. It is an error to attempt to access the DG in updateShader() and doing so may result in instability. In updateShader() the implementation is given the MShaderInstance of which the fragment it specified is a part. It is also given the full list of attribute parameter mappings known to Maya for the node (both automatic and custom). Since most parameters are renamed from the original names on the fragment, the implementation must use the "resolved" name from the mappings to set values on the MShaderInstance. The implementation may set the value of any parameter on the shader instance but any parameter with a mapping that defines an attribute will be set automatically. Only parameters that need special computation to set their values need to be handled. Although it is possible to set values on the MShaderInstance for parameters from other fragments in the Maya shading node graph, this behaviour is not recommended or supported. Such values may get overwritten and behaviour will be unpredictable. Implementations of MPxShadingNodeOverride must be registered with Maya through MDrawRegistry.

**Public Member Functions**

- `MPxShadingNodeOverride (const MObject &obj)`  
  Construct an MPxShadingNodeOverride.
- `virtual ~MPxShadingNodeOverride ()`  
  Destructor.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool allowConnections () const`  
  An override may prevent Maya from connecting fragments to specific parameters of the fragment for this override by providing custom attribute parameter mappings.
- `virtual MString fragmentName () const =0`  
  Override this method to return the name of the fragment or fragment graph to use for rendering the shading node associated with this override.
- `virtual void getCustomMappings (MAttributeParameterMappingList &mappings)`  
  Maya will automatically match parameters on the shade fragment specified by this override with attributes on the associated Maya node as long as the names and types match.
- `virtual MString outputForConnection (const MPlug &sourcePlug, const MPlug &destinationPlug)`  
  When Maya attempts to connect the fragment for this override to the fragment for another node in the shading network, it will call this method to get the name of the output on the fragment to use for the connection.
- `virtual bool valueChangeRequiresFragmentRebuild (const MPlug *plug) const`  
  This method will be called by Maya when it detects changes in the attribute values of nodes in the shading network.
- `virtual void updateDG ()`  
  This method is called every time Maya needs to update the parameter values on the final shading effect of which the fragment produced by this override is a part.
- `virtual void updateShader (MShaderInstance &shader, const MAttributeParameterMappingList &mappings)`  
  This method is called every time Maya needs to update the parameter values on the final shading effect of which the fragment produced by this override is a part.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxSubSceneOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_sub_scene_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_sub_scene_override.html).

Base class for Viewport 2.0 drawing of DAG nodes which represent sub-scenes. MPxSubSceneOverride allows the user to completely define the renderable units (render items) needed to draw a specific DAG object type in Maya when using Viewport 2.0. This class is independent of any specific hardware draw API. Implementations operate using the MRenderItem interface and provide a list of the render items needed to draw all instances of the associated DAG object. One implementation of the override will be created for each object in the Maya scene whose classification string satisfies the classification string used when registering the override. If the associated DAG object is instanced, the override is responsible for drawing all instances. The collection of render items is stored in an MSubSceneContainer object. This container is suitable for storing and accessing large numbers of render items (thousands or tens of thousands) in a high performance manner. Implementations are responsible for assigning a shader and an optional matrix to each render item and must also provide the geometry for each render item as required by its shader. Geometry may be shared between render items and management of the geometry is up to the implementation. MPxSubSceneOverride falls in between MPxDrawOverride and MPxGeometryOverride with respect to the amount of control given to the implementation. MPxDrawOverride allows full control over the entire draw of the object, but as a result the implementation is entirely responsible for the draw. MPxGeometryOverride is relatively simple to work with but as a result only limited control of the draw is available. MPxSubSceneOverride is allowed to fully define all render items, geometry and shaders providing a high degree of control. However this definition is abstracted from the hardware draw API so only one implementation is needed to get support for both DirectX and OpenGL. Furthermore, Maya will handle actually drawing the render items and thus the items can participate fully in the Maya rendering pipeline (including screen-space effects like SSAO, transparency sorting, shadow casting/receiving, etc.). Finally, render items may specify which draw modes they should draw in (wireframe, shaded, textured, bounding box) so there is no need for the implementation to track these modes manually. MPxSubSceneOverride can be used to provide Viewport 2.0 support for any type of DAG object, however it is optimized for use in cases where a single DAG object needs to produce a large number of renderables which are not necessarily located at the same point in space. For example, this interface would be a good choice for implementing support for a custom node that needs to read and draw a full scene based on a description from a cache file. MPxSubSceneOverride has three stages. In each stage, an instance of MFrameContext is provided to give some information about the current state of the renderer. The implementation is also free to query any information it needs from Maya. 1) requiresUpdate() : In this call, the implementation is given constant access to the set of render items for the object. On the first call it will be empty. If the method returns true, Maya will call update() and the implementation will be given the opportunity to add/remove render items from the set as well as to modify those items. Render item state is maintained as long as the render item is stored in the MSubSceneContainer object. Thus, if nothing needs to be changed this method may return false to skip the update phase. 2) update() : In this call, the implementation may add/remove render items to/from the set, and it may modify existing render items. For each render item the implementation is required to call setGeometryForRenderItem() to provide the geometry for the render item. Render items without geometry will not draw. 3) addUIDrawables() : In this call, the implementation may use MUIDrawManager to draw simple UI shapes like lines, circles, text, etc. If areUIDrawablesDirty() method returns true, the UI drawables would be cleared and re-added every refresh. If the dirty is false, the UI drawables are being persistently kept until either the DAG object associated with the override is destroyed or the override is deregistered. Note that the UI drawables don't support instancing draw. If at the end of these three stages further update is required then furtherUpdateRequired() can be overridden to indicate whether the three stages need to be called again. If one implementation requires to split and defer the update of selection-related render items until they are needed, enableUpdateForSelection() can be overridden to indicate whether the three stages need to be called in selection pass. If so, the instance of MFrameContext provided for each stage will also allow access to the selection state information. Implementations of MPxSubSceneOverride must be registered with Maya through MDrawRegistry.

**Public Member Functions**

- `MPxSubSceneOverride (const MObject &obj)`  
  Construct an MPxSubSceneOverride.
- `virtual ~MPxSubSceneOverride ()`  
  Destructor.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool requiresUpdate (const MSubSceneContainer &container, const MFrameContext &frameContext) const =0`  
  On each frame Maya will give each instantiated MPxSubSceneOverride object a chance to update its set of render items.
- `virtual void update (MSubSceneContainer &container, const MFrameContext &frameContext)=0`  
  This method is called by Maya on each frame as long as the implementation of MPxSubSceneOverride::requiresUpdate() returns true.
- `virtual bool furtherUpdateRequired (const MFrameContext &frameContext)`  
  This method is called by Maya following update() to allow the override to indicate whether further processing is required.
- `virtual bool areUIDrawablesDirty () const`  
  Determines whether addUIDrawables() should be called on the next refresh.
- `virtual bool hasUIDrawables () const`  
  Tells Maya whether addUIDrawables() should be called.
- `virtual void addUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`  
  Provides access to a MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.
- `virtual bool enableUpdateForSelection () const`  
  The method is called by Maya when creating the subscene.
- `virtual bool getSelectionPath (const MRenderItem &renderItem, MDagPath &dagPath) const`  
  This method is called by Maya during the selection interpretation phase of Viewport 2.0 selection to allow specifying the selection path for a single render item.
- `virtual bool getInstancedSelectionPath (const MRenderItem &renderItem, const MIntersection &intersection, MDagPath &dagPath) const`  
  This method is called by Maya during the selection interpretation phase of Viewport 2.0 selection to allow specifying the selection path for a single render item.
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`  
  Maya calls this function during the pre-filtering phase of Viewport 2.0 selection and is used to allow derived classes to modify the selection context of the given DAG object.
- `MStatus setGeometryForRenderItem (MRenderItem &renderItem, const MVertexBufferArray &vertexBuffers, const MIndexBuffer &indexBuffer, const MBoundingBox *objectBox)`  
  Call this method to provide the geometry for a render item.
- `unsigned int addInstanceTransform (MRenderItem &renderItem, const MMatrix &transform)`  
  Add an additional instance for a render item.
- `MStatus setInstanceTransformArray (MRenderItem &renderItem, const MMatrixArray &matrixArray)`  
  Sets the entire instance array for a render item.
- `MStatus updateInstanceTransform (MRenderItem &renderItem, unsigned int instanceId, const MMatrix &transform)`  
  Update the instance transform matrix for one instance of a render item.
- `MStatus removeInstance (MRenderItem &renderItem, unsigned int instanceId)`  
  Remove one instance of a render item.
- `MStatus removeAllInstances (MRenderItem &renderItem)`  
  Remove all instances for a render item.
- `MStatus setExtraInstanceData (MRenderItem &renderItem, const MString &parameterName, const MFloatArray &data)`  
  Adds an extra stream of instanced data to an instanced render item.
- `MStatus setExtraInstanceData (MRenderItem &renderItem, unsigned int instanceId, const MString &parameterName, const MFloatArray &data)`  
  Set the value for the specified extra instance data stream for a particular instance of the instanced render item.
- `MStatus removeExtraInstanceData (MRenderItem &renderItem, const MString &parameterName)`  
  Remove an entire extra instance data stream from the instanced render item.
- `MStatus setAllowTransparentInstances (MRenderItem &renderItem, bool allow)`  
  Instancing is disabled automatically by default when the shader is transparent.

**Static Public Member Functions**

- `static bool pointSnappingActive ()`  
  Returns true if selection has been launched to find snap points.
- `static const char * className ()`  
  Returns the name of this class.


### MPxSurfaceShadingNodeOverride

Module `OpenMayaRender`. Inherits `MPxShadingNodeOverride`. Source: [class_m_h_w_render_1_1_m_px_surface_shading_node_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_surface_shading_node_override.html).

Base class for user defined surface shading node overrides. MPxSurfaceShadingNodeOverride is an extension of MPxShadingNodeOverride which is specialized for surface shaders. Plugin surface shader nodes that may be connected directly to a Maya shading engine should define an override which derives from this class instead of MPxShadingNodeOverride when providing support for Viewport 2.0. In addition to providing all the functionality of MPxShadingNodeOverride, MPxSurfaceShadingNodeOverride also lets the override flag certain parameters and attributes for special processing. Maya also treats MPxSurfaceShadingNodeOverride objects differently than plain MPxShadingNodeOverride objects. In particular, Maya will attempt to connect the shade fragments for lights to the fragment provided by an implementation of MPxSurfaceShadingNodeOverride. The following named input parameters on the fragment will be recognized and have the specified Maya lighting parameters automatically connected to them. Parameter name Parameter type Automatically connected value from lights Lw 3-float World space light direction vector for diffuse lighting HLw and SLw 3-float World space light direction vector for specular lighting diffuseI 3-float Diffuse irradiance for light specularI 3-float Specular irradiance for light ambientIn 3-float Total contribution from all ambient lights When there are multiple lights in the scene, the contribution from the fragments that make up the final shading effect is computed once for each light and then the results are accumulated. So the parameters above are all for the whichever light is currently being computed. Please examine the fragments and fragment graphs for the Maya phong shader (mayaPhongSurface) for examples of how light information can be used. Implementations of MPxSurfaceShadingNodeOverride must be registered with Maya through MDrawRegistry.

**Public Member Functions**

- `MPxSurfaceShadingNodeOverride (const MObject &obj)`  
  Construct an MPxSurfaceShadingNodeOverride.
- `~MPxSurfaceShadingNodeOverride () override`  
  Destructor.
- `virtual MString primaryColorParameter () const`  
  This method is called just after getCustomMappings() to allow the plugin to provide extra surface shader-specific information.
- `virtual MString transparencyParameter () const`  
  This method is called just after getCustomMappings() to allow the plugin to provide extra surface shader-specific information.
- `virtual MString bumpAttribute () const`  
  This method is called after getCustomMappings() to allow the plugin to provide surface-shader specific information.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxVertexBufferGenerator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_vertex_buffer_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_vertex_buffer_generator.html).

Base class for user defined vertex buffer generators. Implementations of MPxVertexBufferGenerator must be registered with Maya through MDrawRegistry. MPxVertexGenerators are registered with a unique buffer name. The name signifies the name of the custom stream the generator can produce. When an MPxShaderOverride or custom renderer uses an effect with custom input streams they can provide a unique name in the MVertexBufferDescriptor and a registered MPxVertexBufferGenerator will be used to produce the custom stream for a given shape using the incoming requirements held in the MVertexBuffer. createVertexStream() is called to allow the generator to fill in the missing vertex data Access the requirements for the requested stream through vertexBuffer.descriptor(). Populate the stream using vertexBuffer.acquire() and vertexBuffer.commit().

**Public Member Functions**

- `MPxVertexBufferGenerator ()`  
  Construct an MPxVertexBufferGenerator.
- `virtual ~MPxVertexBufferGenerator ()`  
  Destructor.
- `virtual bool getSourceIndexing (const MObject &object, MComponentDataIndexing &sourceIndexing) const =0`  
  This function is called to allow the vertex buffer generator to provide its vertex indexing information as well as the space the vertices are in.
- `virtual bool getSourceStreams (const MObject &object, MStringArray &sourceStreams) const =0`  
  This function is called to allow the vertex buffer generator to provide the list of stream names that it requires.
- `virtual void createVertexStream (const MObject &object, MVertexBuffer &vertexBuffer, const MComponentDataIndexing &targetIndexing, const MComponentDataIndexing &sharedIndexing, const MVertexBufferArray &sourceStreams) const =0`  
  This method gets called to allow the generator to fill in the data for a custom vertex stream.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxVertexBufferMutator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_vertex_buffer_mutator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_vertex_buffer_mutator.html).

Base class for user defined vertex buffer generators. Implementations of MPxVertexBufferMutator must be registered with Maya through MDrawRegistry. MPxVertexBufferMutators are registered with a unique buffer name. The name signifies the name of the custom stream the mutator can alter. When an MPxShaderOverride or custom renderer uses an effect with custom input streams they can provide a unique name in the MVertexBufferDescriptor and a registered MPxVertexBufferMutator will be used to alter the custom stream for a given shape using the incoming requirements held in the MVertexBuffer. modifyVertexStream() is called to allow the mutator to alter the vertex data Access the requirements for the requested stream through vertexBuffer.descriptor(). Populate the stream using vertexBuffer.acquire() and vertexBuffer.commit().

**Public Member Functions**

- `MPxVertexBufferMutator ()`  
  Construct an MPxVertexBufferMutator.
- `virtual ~MPxVertexBufferMutator ()`  
  Destructor.
- `virtual void modifyVertexStream (const MObject &object, MVertexBuffer &vertexBuffer, const MComponentDataIndexing &targetIndexing) const =0`  
  This method gets called to allow the mutator to alter the data for a custom vertex stream.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MPxViewportComputeItem

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_viewport_compute_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_viewport_compute_item.html).

Virtual base class for user viewport compute. MPxViewportComputeItem is a virtual base class meant to provide a means for users to attach compute to certain Maya object types such that the lifetime of the compute may be managed by Maya. When hasExternalOwner is false then the MPxViewportComputeItem is owned by the Maya object it is attached to and the MPxViewportComputeItem will be deleted when that object is deleted. When hasExternalOwner is true the lifetime of the MPxViewportComputeItem object is the responsibility of the allocator. A typical use case for MPxViewportComputeItem would be to compute stream data for an MRenderItem. For example, a shaded MRenderItem could be created with only a postion buffer & an MPxViewportComputeItem attached to compute & fill the normal buffer. When MPxViewportComputeItem is attached to an MRenderItem it is visible to consolidated world. Use methods canConsolidate() and cloneForConsolidation() to control how MPxViewportComputeItem interacts with consolidation. An MPxViewportComputeItem must declare the action(s) it will take during a call to execute. These actions reflect access to resources or modification of the associated MRenderItem, or access to global resources. See setRequiredAction() and execute() for more information.

**Classes**

- `struct Actions`  
  A bitmask storing a set of actions.

**Public Types**

- `enum Action { kAccessVirtualDevice = 0x1 << 0, kAccessConsolidation = 0x1 << 1, kModifyVertexBufferSize = 0x1 << 2, kModifyMatrix = 0x1 << 3, kModifyConsolidation = 0x1 << 4, kModifyBounds = 0x1 << 5, kAddVertexStream = 0x1 << 6, kModifyVertexBufferData = 0x1 << 7, kModifyPrimitive = 0x1 << 8, kActionFinal = 0x1 <<9 }`  
  Actions an MPxViewportComputeItem may perform during its operator() call. Some MRenderItem functions can never be called during execute(): setDrawMode, setShader, setShaderFromNode, setExcludedFromPostEffect, supportsAdvancedTransparency, setCustomData, enable, castsShadows, recievesShadows, depthPriority, setAllowIsolateSelectCopy, setWAntConsolidation, setCompatibleWithMayaInstancer, setTreatAsTransparent, setSelectionMask, setWantSubSceneConsolidation may never be called.

**Public Member Functions**

- `MPxViewportComputeItem (bool hasExternalOwner)`  
  Construct an MPxViewportComputeItem.
- `virtual ~MPxViewportComputeItem ()`  
  Destructor.
- `bool hasExternalOwner () const`  
  Return whether or not this MPxViewportComputeItem should be deleted when the Maya object it is attached to is deleted.
- `void setHasExternalOwner (bool hasExternalOwner)`  
  Sets whether or not this MPxViewportComputeItem should be deleted when the Maya object it is attached to is deleted.
- `MStatus setRequiredAction (Action action, bool required)`  
  Sets actions which must be available to this MPxViewportComputeItem when execute() is called.
- `bool hasRequiredAction (Action action) const`  
  Returns if a action is required by the MPxViewportComputeItem.
- `Actions requiredActions () const`  
  Returns the set of required actions for this MPxViewportComputeItem.
- `virtual bool execute (const Actions &availableActions, MRenderItem &renderItem)=0`  
  Perform the user defined compute operations.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MQuadRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_quad_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_quad_render.html).

Class which defines a 2d geometry quad render.

**Public Member Functions**

- `MQuadRender (const MString &name)`  
  Constructor for a named quad render.
- `~MQuadRender () override`  
  Default destructor for a quad render.
- `virtual const MShaderInstance * shader ()`  
  Shader to use when rendering a quad.
- `virtual MClearOperation & clearOperation ()`  
  Get the scene clear operation.
- `virtual const MDepthStencilState * depthStencilStateOverride ()`  
  Writing to the depth buffer is disabled by default for a quad operation.
- `virtual const MRasterizerState * rasterizerStateOverride ()`  
  No backface culling is performed by default for a quad operation.
- `virtual const MBlendState * blendStateOverride ()`  
  Blending is disabled by default for a quad operation.

**Protected Attributes**

- `MClearOperation mClearOperation`  
  Clear operation.


### MRasterizerState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_rasterizer_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_rasterizer_state.html).

Container class for an acquired complete GPU rasterizer state. MRasterizerState wraps an acquired composite GPU rasterizer state. This rasterizer state can be used to set the current GPU rasterizer state using MStateManager::setRaserizerState(myRaserizerState). Instances of MRasterizerState cannot be created or modified by users. You can read the descriptor for an MRasterizerState using MRasterizerState::desc(), but you cannot set or alter the descriptor directly. MRasterizerState can only be obtained via MStateManager::acquireRaserizerState(). To use MStateManager::acquireRaserizerState(), create and fill an MRasterizerStateDesc instance with the requested state and pass it to MStateManager::acquireRaserizerState() which will return the cached unique rasterizer state of that description. If the unique rasterizer state did not previously exist in the cache, it is created. The information included in the rasterizer state includes fill mode, culling mode, anti-aliasing enables, depth bias information, depth clipping enable, scissor clip enable, and whether CW or CCW winding is "front". The advantages of using this class rather than setting rasterizer states explicitly are: 1. Since the states are cached internally, state setting is much faster than setting individual states. 2. Since state is set through the Maya viewport API, Maya can maintain accurate knowledge of the current GPU state, even in the presence of arbitrary plug-ins. This also speeds state setup by reducing redundant state setting and OGL queries. 3. The information is device aware, meaning that it will return the correct results based on the current active device. For example it will return the appropriate values for DirectX versus an OpenGL device.

**Public Types**

- `enum FillMode { kFillSolid = 3, kFillWireFrame = 2 }`  
  Indicates how to fill various primitives.
- `enum CullMode { kCullNone = 1, kCullFront = 2, kCullBack = 3 }`  
  Indicates how to cull primitive faces during drawing.

**Public Member Functions**

- `const MRasterizerStateDesc & desc (MStatus *ReturnStatus=NULL) const`  
  Get the rasterizer state descriptor that was used to create the state object.
- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a rasteriser state.
- `~MRasterizerState ()`  
  Destructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MRasterizerStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_rasterizer_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_rasterizer_state_desc.html).

Descriptor for a complete rasterizer state. This class describes a complete rasterizer state, like fill mode, cull mode, depth bias, multisampling, point size, etc.

**Public Member Functions**

- `MRasterizerStateDesc ()`  
  Constructor, builds a default rasterizer state.
- `~MRasterizerStateDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the rasterizer state to their default values.
- `MRasterizerStateDesc (const MRasterizerStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MRasterizerStateDesc & operator= (const MRasterizerStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `MRasterizerState::FillMode fillMode`  
  Select the primitive fill mode, default kFillSolid.
- `MRasterizerState::CullMode cullMode`  
  Select the face culling mode, default kCullNone.
- `bool frontCounterClockwise`  
  Select whether CW or CCW winding is used for "front" face, default false.
- `bool depthBiasIsFloat`  
  Indicates that DepthBias is a float value, default false.
- `float depthBias`  
  DepthBias adds the given bias value to the rasterizer z value prior to depth testing.
- `float depthBiasClamp`  
  Maximum value scaled depth bias can attain, default 0.
- `float slopeScaledDepthBias`  
  Slope scaled depth bias value, default 0.
- `bool depthClipEnable`  
  Enables HW automatic depth clipping, default true.
- `bool scissorEnable`  
  Enables HW scissor clip rectangle, default false.
- `bool multiSampleEnable`  
  Enables HW full screen multi-sample anti-aliasing, default false.
- `bool antialiasedLineEnable`  
  Enables HW anti-aliased lines, auto disabled by multi-sample AA, default false.


### MRenderCallback

Module `OpenMayaRender`. Source: [class_m_render_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_callback.html).

Rendering Callbacks. This class is used to register callbacks to gain access to Maya's rendering information during software rendering. You can modify Maya's shadow maps, RGB pixmap, and depth map to composite your own rendering effects into Maya's rendering. To register callbacks, inherit from this class and override renderCallback(), shadowCastCallback(), or postProcessCallback(). Any number of these methods can be overridden by the callback. Then register the callbacks by calling the addCallback() method. Each of the callback method gets passed a MRenderData which contains the information. MRenderData also provides utility methods for converting between world space and screen space. In the case of shadowCastCallback, MRenderShadowData is provided and it also has utility methods for converting between world space and shadow map space. If there are callbacks registered, prior to shadow maps being written out, shadowCastCallback() will be invoked with light information and a pointer to the shadow map passed in. Then immediately after software rendering completes, renderCallback() will be invoked with the rendering's dimension info and image passed in. Lastly, during post-processing, postProcessCallback() will be invoked with the rendering's dimension and pointers to the rgb pixmap and depthmap passed in. If multiple callbacks need to be registered, the order of invocation can be set by adding each callback with a priority number, 0 being the highest priority.

**Public Member Functions**

- `MRenderCallback ()`  
  Default Constructor.
- `virtual ~MRenderCallback ()`  
  Default Destructor.
- `virtual bool shadowCastCallback (const MRenderShadowData &data)`  
  Method to override for plugging into shadow map creation.
- `virtual bool renderCallback (const MRenderData &data)`  
  Method to override for plugging into image rendering.
- `virtual bool postProcessCallback (const MRenderData &data)`  
  Method to override for plugging into the post-process.

**Static Public Member Functions**

- `static void addCallback (MRenderCallback *, int priority=0)`  
  Static procedure to add a rendering callback.
- `static void removeCallback (MRenderCallback *)`  
  Static procedure to remove a rendering callback.
- `static MCallbackId addRenderTileCallback (MMessage::MRenderTileFunction func, MStatus *ReturnStatus)`  
  Static procedure to add a callback to receive tiles as they get rendered and before they get displayed in the Render View Window.
- `static const char * className ()`  
  Returns the name of this class.


### MRenderData

Module `OpenMayaRender`. Source: [class_m_render_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_data.html).

Access Rendering Data. The rendered image and depth map can be changed following the software render by instanciating a MRenderCallback and overriding renderCallback() or postProcessCallback(). When these methods are invoked, a MRenderData is passed as an argument; the rgbaArr and depthArr can then be changed by this API. Methods and data are provided to assist in transforming back and forth from world space to image space. Paint Effects and Fur are two examples which use this mechanism to change the rendered image.

**Public Member Functions**

- `MRenderData ()`  
  Default class Constructor.
- `void worldToScreen (const MFloatPoint &worldPoint, MFloatPoint &screenPoint) const`  
  Converts world space point to screen space.
- `void screenToWorld (const MFloatPoint &screenPoint, MFloatPoint &worldPoint) const`  
  Converts screen space point to world space.

**Public Attributes**

- `bool perspective`  
  set if the camera has a perspective projection
- `unsigned short resX`  
  the x resolution of the rendered image
- `unsigned short resY`  
  the y resolution of the rendered image
- `unsigned short left`  
  the left (low) render region dimension
- `unsigned short bottom`  
  the bottom (low) render region dimension
- `unsigned short right`  
  the right (high) render region dimension
- `unsigned short top`  
  the top (high) render region dimension
- `unsigned short bytesPerChannel`  
  number of bytes per channel of the rgbaArr
- `unsigned short xsize`  
  the actual x resolution of rgbaArr and depthArr
- `unsigned short ysize`  
  the actual y resolution of rgbaArr and depthArr
- `float fieldOfView`  
  camera horizontal field of view in radians
- `float aspectRatio`  
  camera pixel aspect ratio
- `MFloatVector viewDirection`  
  view direction of the camera (object space)
- `MFloatPoint eyePoint`  
  position of the camera (object space)
- `MFloatMatrix worldToEyeMatrix`  
  camera matrix to transform to eye space
- `unsigned char * rgbaArr`  
  this is a 1d array representing the output image buffer.
- `float * depthArr`  
  this is a 1d array representing the output depth buffer.


### MRenderItem

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_item.html).

A single renderable entity. MRenderItem represents a single renderable entity. Each such entity is defined by a name, a render item type and a primitive type. The render item type affects how Maya treats the render item under various contexts. A list of MRenderItem objects is passed to the draw method of implementations of MPxShaderOverride to indicate the objects that should be drawn immediately by the override. If the implementation wishes to manually bind data streams it can call MRenderItem::geometry() to access the vertex buffers and index buffers that have already been generate for the render item. MRenderItem objects are used by implementations of MPxGeometryOverride for two purposes. First, MPxGeometryOverride::updateRenderItems() can create and add new render items to the list of items that will be drawn for a specific instance of a DAG object. This method can also enable/disable render items in the list and set a custom solid color shader on any user defined items. Later, a list of render items is provided to MPxGeometryOverride::populateGeometry(), to indicate index buffers that need to be generated. This list contains both the automatically generated render items Maya creates for shader assignments as well as the render items created in MPxGeometryOverride::updateRenderItems(). MRenderItem objects are also used by MPxSubSceneOverride for specifying all information about all drawable objects produced by the override.

**Public Types**

- `enum RenderItemType { MaterialSceneItem, NonMaterialSceneItem, DecorationItem, InternalItem, InternalMaterialItem, InternalTexturedMaterialItem, InternalUnsupportedMaterialItem, OverrideNonMaterialItem }`  
  Type descriptor for render items which drives how Maya treats them in the render pipeline.
- `enum DefaultMaterialFiltering { IgnoreDefaultMaterialMode, DrawOnlyWhenDefaultMaterialActive, SkipWhenDefaultMaterialActive }`  
  Specifies how to handle drawing when the default material mode is active.
- `typedef void(* LinkLostCallback) (MUserData *userData)`  
  Definition for callback function triggered when shader instance assigned by setShaderFromNode() is no longer valid.

**Public Member Functions**

- `void setDrawMode (MGeometry::DrawMode mode)`  
  Set the draw mode for the render item.
- `bool setMatrix (const MMatrix *mat)`  
  Override the object to world transformation matrix to use when drawing this render item.
- `bool setShader (const MShaderInstance *shader, const MString *customStreamName=NULL)`  
  Set shader to use when drawing this render item.
- `MStatus setShaderFromNode2 (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback linkLostCb=nullptr, const MSharedPtr< MUserData > &linkLostUserData=nullptr, bool nonTextured=false)`  
  Set shader to use when drawing this render item.
- `bool setShaderFromNode (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback=NULL, MUserData *linkLostUserData=NULL, bool nonTextured=false)`  
  This method is obsolete.
- `void setExcludedFromPostEffects (bool exclude)`  
  Set whether this item should be excluded from post-effects like SSAO and depth-of-field.
- `void setSupportsAdvancedTransparency (bool support)`  
  Set whether this item supports advanced transparency like depth peeling.
- `void setCustomData (MUserData *userData)`  
  This method is obsolete.
- `void setCustomData (MSharedPtr< MUserData > userData)`  
  Associate custom user data with this render item.
- `MStatus addViewportComputeItem (MSharedPtr< MPxViewportComputeItem > viewportComputeItem)`  
  Associate custom viewport compute with this render item.
- `void enable (bool on)`  
  Enable or disable the render item for rendering.
- `void castsShadows (bool on)`  
  Enable or disable the ability of the render item to cast shadows.
- `void receivesShadows (bool on)`  
  Enable or disable the ability of the render item to receive shadows.
- `void depthPriority (unsigned int val)`  
  Set the depth priority of the render item.
- `MUint64 InternalObjectId () const`  
  A unique identifier for this render item.
- `bool isShaderFromNode () const`  
  Return true if the shader instance was set by evaluating the shading network of a surface shader node (either standard or custom) in the scene via setShaderFromNode() method.
- `MShaderInstance * getShader ()`  
  Get the shader used when drawing this render item.
- `const MShaderInstance * getShader () const`  
  Get a const pointer to the shader used by this render item.
- `bool associateWithIndexBuffer (const MIndexBuffer *buffer) const`  
  Use to indicate that a particular index buffer should be used with this render item.
- `bool excludedFromPostEffects () const`  
  Get whether this item is excluded from post-effects like SSAO and depth-of-field.
- `bool supportsAdvancedTransparency () const`  
  Returns true if this item supports advanced transparency like depth peeling.
- `MUserData * customData () const`  
  This method is obsolete.
- `MSharedPtr< MUserData > getCustomData () const`  
  Retrieve custom data from the render item, returns an empty MSharedPtr if no such data has ever been set on the render item.
- `MSharedPtr< MPxViewportComputeItem > viewportComputeItem () const`  
  Retrieve custom data from the render item, returns NULL if no such data has ever been set on the render item.
- `bool isEnabled () const`  
  Get the enable state of the render item.
- `bool castsShadows () const`  
  Get the castsShadows state of the render item.
- `bool receivesShadows () const`  
  Get the receivesShadows state of the render item.
- `unsigned int depthPriority () const`  
  Get the depth priority value for the render item.
- `MString name () const`  
  Get the name of the render item.
- `void setPrimitive (MGeometry::Primitive primitive, int stride=0)`  
  Set the primitive type of the render item.
- `MGeometry::DrawMode drawMode () const`  
  Get the draw mode for the render item.
- `MDagPath sourceDagPath () const`  
  Retrieve the MDagPath for the instance of the object that generated this render item.
- `MObject component () const`  
  Get the optional component for the render item if the render item represents the drawing of a component as a result of per-face shader assignment, component selection highlighting etc.
- `MObject shadingComponent () const`  
  Get the optional shading component for the render item.
- `void setAllowIsolateSelectCopy (bool state)`  
  When a 3D model view activates Isolate Select for components, Viewport 2.0 will create and maintain necessary render items to represent the drawing of the isolate selected components specifically for that view.
- `bool allowIsolateSelectCopy () const`  
  Returns whether or not the render item allows its copies to be created for the drawing of isolate selected components.
- `bool isIsolateSelectCopy () const`  
  Returns whether or not the render item is a copy created to represent the drawing of an isolate selected component.
- `const MVertexBufferDescriptorList & requiredVertexBuffers () const`  
  Get a list of vertex buffer descriptors that describe the buffers required to draw the given render item.
- `const MGeometry * geometry () const`  
  Access full geometry data for the render item.
- `MGeometry * geometry ()`  
  Access full geometry data for the render item.
- `MBoundingBox boundingBox (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`  
  Returns the bounding box for the geometry data of the render item.
- `MStatus setBoundingBox (const MBoundingBox &bounds)`  
  Sets the bounding box for the geometry data of the render item.
- `bool isConsolidated () const`  
  Return whether or not this render item is a consolidated item.
- `bool isMultiDraw () const`
- `void sourceIndexMapping (MGeometryIndexMapping &geometryIndexMapping) const`  
  Get the geometry index mapping of the multiples objects contained by this consolidated render item.
- `bool isSourceIndexMappingValid (const MGeometryIndexMapping &geometryIndexMapping) const`  
  Validate an existing MGeometryIndexMapping for *this MRenderItem.
- `void setSourceIndexMapping (const MGeometryIndexMapping &geometryIndexMapping)`  
  Get the geometry index mapping of the multiples objects contained by this consolidated render item.
- `bool wantConsolidation () const`  
  Return whether or not this render item wants to participate in consolidation.
- `void setWantConsolidation (bool state)`  
  Set whether or not this render item wants to participate in consolidation.
- `bool isCompatibleWithMayaInstancer () const`  
  Returns whether or not this render item can be used as an instance object with Maya Instancer.
- `void setCompatibleWithMayaInstancer (bool state)`  
  Set whether or not this render item can be used as an instance object with Maya Instancer.
- `void setTreatAsTransparent (bool val)`  
  Set whether or not this object should be treated as a transparent item.
- `DefaultMaterialFiltering getDefaultMaterialHandling () const`  
  Returns whether or not this render item will draw when default material mode is active.
- `void setDefaultMaterialHandling (DefaultMaterialFiltering state)`  
  Set whether or not this object should be drawn when default material mode is active.
- `MSelectionMask selectionMask () const`  
  Get the render item selection mask.
- `void setSelectionMask (const MSelectionMask &mask)`  
  Set the render item selection mask.
- `MUint64 objectTypeExclusionFlag () const`  
  Query the bit flag which is used in display filtering based on object types.
- `MStatus setObjectTypeExclusionFlag (MUint64 flag)`  
  Set a bit flag for use in display filtering based on object types.
- `unsigned int availableShaderParameters (MStringArray &parameterNames, MStatus *ReturnStatus=NULL) const`  
  Get the list of available shader parameters.
- `bool getShaderBoolParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`  
  Get the boolean value of a shader parameter.
- `int getShaderIntParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`  
  Get the integer value of a shader parameter.
- `float getShaderFloatParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`  
  Get the float value of a shader parameter.
- `const float * getShaderFloatArrayParameter (const MString &parameterName, unsigned int &size, MStatus *ReturnStatus=NULL) const`  
  Get the float values of a shader parameter.
- `bool wantSubSceneConsolidation () const`  
  This method is obsolete.
- `void setWantSubSceneConsolidation (bool state)`  
  This method is obsolete.
- `~MRenderItem ()`  
  Destructor.
- `MRenderItem ()`  
  Constructor.
- `MStatus setShaderFromNodePython2 (const MObject &shaderNode, const MDagPath &shapePath, PythonCallbackWrapper *linkLostCb=nullptr, const MSharedPtr< MUserData > &linkLostUserData=nullptr, bool nonTextured=false)`  
  This overload is only used to interface with the python bindings for compatibility reasons.
- `bool setShaderFromNodePython (const MObject &shaderNode, const MDagPath &shapePath, PythonCallbackWrapper *=nullptr, MUserData *linkLostUserData=nullptr, bool nonTextured=false)`  
  This overload is only used to interface with the python bindings for compatibility reasons.

**Static Public Member Functions**

- `static MRenderItem * Create (const MString &name, RenderItemType type, MGeometry::Primitive primitive)`  
  Static MRenderItem creation utility.
- `static MRenderItem * Create (const MString &name, RenderItemType type, MGeometry::Primitive primitive, int stride)`  
  Static MRenderItem creation utility.
- `static MRenderItem * Create (const MRenderItem &item)`  
  Static MRenderItem creation utility.
- `static void Destroy (MRenderItem *&item)`  
  Static MRenderItem destruction utility.
- `static const char * className ()`  
  Returns the name of this class.
- `static MRenderItem * Create (const MString &name, MGeometry::Primitive primitive, MGeometry::DrawMode mode, bool raiseAboveShaded)`  
  This method is obsolete.

**Public Attributes**

- `OPENMAYA_PRIVATE RenderItemType type`  
  Get the type of the render item.
- `OPENMAYA_PRIVATE RenderItemType MGeometry::Primitive primitive`  
  Get the primitive type drawn by the render item.

**Static Public Attributes**

- `static unsigned int sDormantFilledDepthPriority`  
  Dormant filled primitive depth priority.
- `static unsigned int sDormantWireDepthPriority`  
  Dormant wireframe primitive depth priority.
- `static unsigned int sHiliteWireDepthPriority`  
  Hilite wireframe primitive depth priority.
- `static unsigned int sActiveWireDepthPriority`  
  Active wireframe primitive depth priority.
- `static unsigned int sActiveLineDepthPriority`  
  Active line / edge depth priority.
- `static unsigned int sDormantPointDepthPriority`  
  Dormant point depth priority.
- `static unsigned int sActivePointDepthPriority`  
  Active point depth priority.
- `static unsigned int sSelectionDepthPriority`  
  Selection depth priority.


### MRenderItemList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_item_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_item_list.html).

A list of MRenderItem objects. A list of MRenderItem objects. All items in the list are owned by the list. This class cannot be created or destroyed, it is only passed to the user through various interfaces.

**Public Member Functions**

- `int length () const`  
  Get the number of items in the list.
- `int indexOf (const MString &name) const`  
  Find the index of the first render item in the list matching the given search parameters.
- `int indexOf (const MString &name, MRenderItem::RenderItemType type) const`  
  Find the index of the first render item in the list matching the given search parameters.
- `int indexOf (const MString &name, MGeometry::Primitive primitive, MGeometry::DrawMode mode) const`  
  Find the index of the first render item in the list matching the given search parameters.
- `const MRenderItem * itemAt (int index) const`  
  Get a read-only pointer to the item at the specified index.
- `MRenderItem * itemAt (int index)`  
  Get the item at the specified index.
- `bool append (MRenderItem *item)`  
  Add the item to the list.
- `bool removeAt (int index)`  
  Remove the item at the specified index.
- `void clear ()`  
  Clear the list.
- `MRenderItemList (const MRenderItemList &)`  
  Constructor.
- `~MRenderItemList ()`  
  Destructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MRenderOperation

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_operation.html).

Class which defines a rendering operation.

**Public Types**

- `enum MRenderOperationType { kClear, kSceneRender, kQuadRender, kUserDefined, kHUDRender, kPresentTarget }`  
  Supported render operation types.

**Public Member Functions**

- `virtual bool enableSRGBWrite ()`  
  Return whether to enable GPU based gamma correction during pixel writes.
- `virtual const MFloatPoint * viewportRectangleOverride ()`  
  Query for a viewport rectangle override.
- `virtual const MString & name () const`  
  Query the name of a render operator.
- `virtual bool getInputTargetDescription (const MString &name, MRenderTargetDescription &description)`  
  Return the description to use when creating the target for the named input.
- `virtual int writableTargets (unsigned int &count)`  
  Return the index of the first writable target in the inputTargets list.
- `virtual MRenderTarget *const * targetOverrideList (unsigned int &listSize)`  
  Return a list of pointers to render target which will be used as the target overrides for the operation.
- `const MStringArray & inputTargets ()`  
  An array of input target names that semantically declares the type of targets the operation requires as input.
- `const MStringArray & outputTargets ()`  
  An array of output target names that semantically declares the type of targets the operation can produce.
- `void renameOutputTarget (const MString &name, const MString &newName)`  
  Renames an output target of an operation.
- `MRenderOperationType operationType () const`  
  Query the type (MRenderOperationType) of a render operator.

**Protected Member Functions**

- `MRenderOperation (const MString &name)`  
  Protected constructor.
- `MRenderTarget * getInputTarget (const MString &name)`  
  Gets the target associated with this input.

**Protected Attributes**

- `MRenderOperationType mOperationType`  
  Operation type.
- `MString mName`  
  Identifier for a sub render.


### MRenderOperationList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_operation_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_operation_list.html).

Base class for holding a list of MRenderOperations. The list is the owner of the operations.

**Public Member Functions**

- `MRenderOperationList ()`  
  Constructor.
- `~MRenderOperationList ()`  
  Destructor.
- `int length () const`  
  Get the number of MRenderOperation in the list.
- `const MRenderOperation * operator[] (int index) const`  
  Gets the MRenderOperation at the requested index.
- `MRenderOperation * operator[] (int index)`  
  Gets the MRenderOperation at the requested index.
- `bool replace (const MString &name, MRenderOperation *operation)`  
  Replace the operation with the given name by a new operation.
- `bool append (MRenderOperation *operation)`  
  Add a MRenderOperation to the list.
- `bool insertBefore (const MString &name, MRenderOperation *operation)`  
  Insert a MRenderOperation in the list before the operation with a name matching the input.
- `bool insertAfter (const MString &name, MRenderOperation *operation)`  
  Insert a MRenderOperation in the list after the operation with a name matching the input.
- `bool remove (const MString &name)`  
  Remove a MRenderOperation from the list.
- `bool removeAt (int index)`  
  Remove a MRenderOperation from the list.
- `MRenderOperation * take (const MString &name)`  
  Remove a MRenderOperation from the list and return it.
- `MRenderOperation * takeAt (int index)`  
  Remove a MRenderOperation from the list and return it.
- `int indexOf (const MString &name)`  
  get the index of the operation with a name matching the input.
- `void clear ()`  
  Clears the list.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MRenderOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_override.html).

Base class for defining a rendering override.

**Public Member Functions**

- `MRenderOverride (const MString &name)`  
  Constructor for a name render override.
- `virtual ~MRenderOverride ()`  
  Default destructor for a render override.
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`  
  Returns the draw API supported by this override.
- `virtual bool startOperationIterator ()`  
  In order for an override to indicate the set of operations that are to be performed it must implement overrides for the iterator methods on the override:
- `virtual MRenderOperation * renderOperation ()`  
  Return the current operation being iterated over.
- `virtual bool nextRenderOperation ()`  
  Iterate to the next operation.
- `const MString & name () const`  
  Query the name of the override.
- `virtual MString uiName () const`  
  Query the user interface name for the override.
- `virtual MStatus setup (const MString &destination)`  
  Perform any setup required before render operations are to be executed.
- `virtual MStatus cleanup ()`  
  Perform any cleanup required following the execution of render operations.
- `virtual bool select (const MFrameContext &frameContext, const MSelectionInfo &selectInfo, bool useDepth, MSelectionList &selectionList, MPointArray &worldSpaceHitPts)`  
  The method is called by Maya to override the default Viewport 2.0 selection.
- `const MFrameContext * getFrameContext () const`  
  Return frame context.


### MRenderParameters

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_parameters.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_parameters.html).

Class which defines a set of render parameters.

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture1, kTexture2, kTexture3, kTextureCube, kSampler, kRenderTarget, kString }`  
  Specifies parameter types.

**Public Member Functions**

- `MRenderParameters ()`  
  Constructor.
- `MRenderParameters (void *)`  
  Constructor (private)
- `MRenderParameters (const MRenderParameters &copy)`  
  Copy Constructor.
- `virtual ~MRenderParameters ()`  
  Destructor (virtual)
- `MRenderParameters & operator= (const MRenderParameters &other)`  
  Operator overload (=)
- `void parameterList (MStringArray &list) const`  
  Get the names of all parameters that are settable on this instance.
- `ParameterType parameterType (const MString &parameterName) const`  
  Get the type of the named parameter, returns kInvalid if parameter is not found.
- `bool isArrayParameter (const MString &parameterName) const`  
  Determine whether the named parameter is an array.
- `MString semantic (const MString &parameterName) const`  
  Return the semantic for a named parameter.
- `MStatus setParameter (const MString &parameterName, bool value)`  
  Set a named, boolean parameter on this instance.
- `MStatus setParameter (const MString &parameterName, int value)`  
  Set a named, integer parameter on this instance.
- `MStatus setParameter (const MString &parameterName, float value)`  
  Set a named, float parameter on this instance.
- `MStatus setParameter (const MString &parameterName, const float *value)`  
  Set a named, float, float2, float3 or float4 parameter on this instance.
- `MStatus setParameter (const MString &parameterName, const MFloatVector &value)`  
  Set a named, float, float2, float3 parameter on this instance.
- `MStatus setParameter (const MString &parameterName, const MMatrix &value)`  
  Set a named, matrix parameter on this instance.
- `MStatus setParameter (const MString &parameterName, const MFloatMatrix &value)`  
  Set a named, matrix parameter on this instance.
- `MStatus setParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`  
  Set a texture parameter on this instance.
- `MStatus setParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`  
  Set a named, texture parameter on this instance to a render target.
- `MStatus setParameter (const MString &parameterName, const MSamplerState &sampler)`  
  Set a sampler parameter on this instance.
- `MStatus setParameter (const MString &parameterName, const MString &str)`  
  Set a named, string parameter on this instance.
- `MStatus setArrayParameter (const MString &parameterName, const bool *values, unsigned int count)`  
  Set a named, bool array parameter on this instance.
- `MStatus setArrayParameter (const MString &parameterName, const int *values, unsigned int count)`  
  Set a named, integer array parameter on this instance.
- `MStatus setArrayParameter (const MString &parameterName, const float *values, unsigned int count)`  
  Set a named, float array parameter on this instance.
- `MStatus setArrayParameter (const MString &parameterName, const MMatrix *values, unsigned int count)`  
  Set a named, matrix array parameter on this instance.
- `MStatus setArrayParameter (const MString &parameterName, const MString *values, unsigned int count)`  
  Set a named, integer array parameter on this instance.
- `MStatus getParameter (const MString &parameterName, bool &value)`  
  Get a named boolean parameter.
- `MStatus getParameter (const MString &parameterName, int &value)`  
  Get a named int parameter.
- `MStatus getParameter (const MString &parameterName, float &value)`  
  Get a named float parameter.
- `MStatus getParameter (const MString &parameterName, float *&value)`  
  Get a named float pointer parameter.
- `MStatus getParameter (const MString &parameterName, MFloatVector &value)`  
  Get a named float vector parameter.
- `MStatus getParameter (const MString &parameterName, MMatrix &value)`  
  Get a named matrix parameter.
- `MStatus getParameter (const MString &parameterName, MFloatMatrix &value)`  
  Get a named float matrix parameter.
- `MStatus getParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`  
  Get a named texture parameter.
- `MStatus getParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`  
  Get a named target parameter.
- `MStatus getParameter (const MString &parameterName, MSamplerStateDesc &samplerDesc)`  
  Get a named sampler state parameter.
- `MStatus getParameter (const MString &parameterName, MString &str)`  
  Get a named string parameter.
- `MStatus getParameter (const MString &parameterName, MIntArray &values)`  
  Get a named int array parameter.
- `MStatus getParameter (const MString &parameterName, MFloatArray &values)`  
  Get a named float array parameter.
- `MStatus getParameter (const MString &parameterName, MMatrixArray &values)`  
  Get a named matrix array parameter.
- `MStatus getParameter (const MString &parameterName, MStringArray &values)`  
  Get a named string array parameter.


### MRenderPassDef

Module `OpenMayaRender`. Source: [class_m_render_pass_def.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_pass_def.html).

Render pass information. This class provides a means to access information about a specific render pass. Initialize by passing to MRenderPassRegistry::getRenderPassDefinition(), or by calling create() to generate and register a new pass.

**Public Member Functions**

- `virtual ~MRenderPassDef ()`  
  Destructor.
- `MPxRenderPassImpl * getImplementation (const MString &renderer, MPxRenderPassImpl::PassTypeBit fbType, unsigned int numChannels, MStatus *ReturnStatus=NULL)`  
  Retrieve an implementation of this render pass for the given renderer.
- `MPxRenderPassImpl * getImplementation (const MString &renderer, MStatus *ReturnStatus=NULL)`  
  Retrieve an implementation of this render pass for the given renderer.
- `MString getID (MStatus *ReturnStatus=NULL) const`  
  Retrieve the pass ID for this pass.
- `MString getName (MStatus *ReturnStatus=NULL) const`  
  Retrieve the pass name for this pass.
- `MString getGroup (MStatus *ReturnStatus=NULL) const`  
  Retrieve the pass group for this pass.
- `MString getDescription (MStatus *ReturnStatus=NULL) const`  
  Retrieve the description for this pass.
- `MString getAttributeType (const MString &attributeName, MStatus *ReturnStatus=NULL) const`  
  Retrieve the type of a given render pass attribute.
- `MStatus addFloatParameter (const MString &longName, const MString &shortName, const MString &UIName, const float &minValue, const float &maxValue, const float &defaultValue)`  
  Define a single-precision floating-point parameter for the pass.
- `MStatus addDoubleParameter (const MString &longName, const MString &shortName, const MString &UIName, const double &minValue, const double &maxValue, const double &defaultValue)`  
  Define a double-precision floating-point parameter for the pass.
- `MStatus addIntParameter (const MString &longName, const MString &shortName, const MString &UIName, const int &minValue, const int &maxValue, const int &defaultValue)`  
  Define an integer parameter for the pass.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Friends**

- `class MRenderPassRegistry`
- `class MFnPlugin`


### MRenderPassRegistry

Module `OpenMayaRender`. Source: [class_m_render_pass_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_pass_registry.html).

Access the render pass registry. This class provides a means to register custom render pass definitions with Maya's internal render pass registry. It can also be used to query pass definitions.

**Static Public Member Functions**

- `static MRenderPassDef getRenderPassDefinition (const MString &passID, MStatus *ReturnStatus=NULL)`  
  Retrieve the definition of the specified render pass.
- `static MRenderPassDef registerRenderPassDefinition (const MString &passID, const MString &passName, const MString &passGroup, const MString &description, bool overload=false, MStatus *ReturnStatus=NULL)`  
  Creates a new render pass definition and registers it with the internal render pass registry.
- `static const char * className ()`  
  Returns the name of this class.


### MRenderProfile

Module `OpenMayaRender`. Source: [class_m_render_profile.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_profile.html).

Render profile. The MRenderProfile class describes the rendering APIs and algorithms supported by a given rendering entity (e.g. a shading node, a renderer). A single profile can contain multiple entries allowing, for example, a shading node to specify that it supports both OpenGL and Direct3D rendering. The profile entries refer to renderers rather than rendering APIs as the rendering elements may depend on specific services, information or algorithms implemented by the renderer (e.g. a global light table, or render state cache).

**Public Types**

- `enum MStandardRenderer { kMayaSoftware, kMayaOpenGL, kMayaD3D }`  
  Maya's internal renderers.

**Public Member Functions**

- `MRenderProfile ()`  
  Create a new empty render profile.
- `~MRenderProfile ()`  
  Class destructor.
- `unsigned int numberOfRenderers () const`  
  Find the number of renderers in this profile.
- `void addRenderer (MStandardRenderer renderer)`  
  Add one of Maya's internal renderers to this render profile.
- `void addRenderer (const MString &name, float version)`  
  Add a custom entry to this render profile.
- `bool hasRenderer (MStandardRenderer renderer) const`  
  See if a Maya renderer is listed in this profile.
- `bool hasRenderer (const MString &name, float version) const`  
  Find a custom entry in this render profile.


### MRenderScriptCallback

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_script_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_script_callback.html).

Class which defines a callback that can be executed from a script fragment. An MRenderScriptCallback is a class that can be implemented and executed from a script fragment. Derived instances of this class can be registered and deregistered with the MRenderer class. Once registered they can be called by name using the "Call" script command in script fragments used by an MSceneRender. The renderer's parameter set plus up to three additional script arguments can be passed into the callback from the script.

**Public Member Functions**

- `MRenderScriptCallback ()`  
  Default constructor for this base class.
- `virtual ~MRenderScriptCallback ()`  
  Virtual destructor for this base class.
- `virtual MString name () const =0`  
  Method used to acquire the name of the script callback.
- `virtual bool execute (MRenderParameters *params, MString arg1, MString arg2, MString arg3)=0`  
  A virtual method callable from a script fragment.


### MRenderSetup

Module `OpenMayaRender`. Source: [class_m_render_setup.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_setup.html).

Utilities for obtaining render setup information.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static MStringArray getEnabledSelectedNodeNames (const MString &renderLayerName, MStatus *status=nullptr)`  
  Return the members of a render setup render layer.


### MRenderShadowData

Module `OpenMayaRender`. Source: [class_m_render_shadow_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_shadow_data.html).

Access Rendering Shadow Map Data. The shadow map can be changed by instanciating a MRenderCallback and overriding shadowCastCallback(). When this is invoked, a MRenderShadowData is passed as an argument; the depthMaps and midDistMaps members can then be changed by this API. Methods and data are provided to assist in transforming back and forth from world space to z buffer space. Paint Effects and Fur are two examples which use this mechanism to change the shadow map. To prevent self shadowing, Maya uses a mid distance map to resolve the ambiguity. Details of this technique can be obtained from Graphics Gems III, "The Shadow Depth Map Revisited".

**Public Types**

- `enum LightType { kInvalid, kPoint, kDirectional, kSpot }`  
  Light Types.

**Public Member Functions**

- `MRenderShadowData ()`  
  Default Constructor.
- `void worldToZbuffer (const MFloatPoint &worldPoint, MFloatPoint &screenPoint) const`  
  Converts world space point to shadow map.
- `void zbufferToWorld (const MFloatPoint &screenPoint, MFloatPoint &worldPoint) const`  
  Converts shadow map point to world space.

**Public Attributes**

- `bool perspective`  
  set if the light has a perspective projection
- `bool useMidDistMap`  
  set if the light generates a depth and midDist map
- `LightType lightType`  
  the type of light the shadow map is generated for
- `unsigned short shadowResX`  
  the x resolution of shadow map
- `unsigned short shadowResY`  
  the y resolution of the shadow map
- `MFloatPoint lightPosition`  
  position of the light in world space
- `MFloatMatrix projectionMatrix`  
  projection matrix for the light
- `MFloatMatrix perspectiveMatrix`  
  perspective matrix for the light
- `float * depthMaps`  
  the output depth map
- `float * midDistMaps`  
  the output mid distance map


### MRenderTarget

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target.html).

An instance of a render target that may be used with Viewport 2.0. This class represents a render target that may be used with the MRenderOperation class for rendering in Viewport 2.0.

**Public Member Functions**

- `MStatus updateDescription (const MRenderTargetDescription &targetDescription)`  
  Change the description of a render target.
- `void targetDescription (MRenderTargetDescription &desc) const`  
  Get target description.
- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a render target.
- `void * rawData (int &rowPitch, size_t &slicePitch)`  
  Get a copy of the raw data mapped to the target.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static void freeRawData (void *data)`  
  Convenience method that simply deletes the data that is allocated in the MRenderTarget::rawData() method.


### MRenderTargetDescription

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target_description.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target_description.html).

Class which provides a description of a hardware render target The name is the unique identifier for a render target.

**Public Member Functions**

- `MRenderTargetDescription ()`  
  Default constructor, initalizes description with zero height and width and other simple default values.
- `MRenderTargetDescription (const MString &name, unsigned int width, unsigned int height, unsigned int multiSampleCount,::MHWRender::MRasterFormat rasterFormat, unsigned int arraySliceCount, bool isCubeMap)`  
  Constructor for a render target description.
- `virtual ~MRenderTargetDescription ()`  
  Default destructor.
- `const MString & name () const`  
  Query the name identifier for the target description.
- `unsigned int width () const`  
  Query the width of a 2D render target slice.
- `unsigned int height () const`  
  Query the height of a 2D render target slice.
- `unsigned int multiSampleCount () const`  
  Query the multi-sample count defined by the description.
- `::MHWRender::MRasterFormat rasterFormat () const`  
  Query the raster format defined by the description.
- `unsigned int arraySliceCount () const`  
  Query the number of array slices defined by the description.
- `bool isCubeMap () const`  
  Query whether this is a cube map target.
- `bool allowsUnorderedAccess () const`  
  Query whether unordered access is supported.
- `void setName (const MString &name)`  
  Set name of the target.
- `void setWidth (unsigned int val)`  
  Set width of the target.
- `void setHeight (unsigned int val)`  
  Set height of the target.
- `void setMultiSampleCount (unsigned int val)`  
  Set multisample count of the target.
- `void setRasterFormat (::MHWRender::MRasterFormat val)`  
  Set the raster format of the target.
- `void setArraySliceCount (unsigned int val)`  
  Set array slice count of the target.
- `void setIsCubeMap (bool val)`  
  Set cube map flag for the target.
- `void setAllowsUnorderedAccess (bool val)`  
  Set the flag for unordered data access for the target.
- `bool compatibleWithDescription (const MRenderTargetDescription &desc)`  
  Determine if another target with a given description is 'compatible' with a target using this description.


### MRenderTargetLegacy

Module `OpenMayaRender`. Source: [class_m_render_target_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_target_legacy.html).

MRenderTargetLegacy is a class contains information about a given hardware render target.

**Public Member Functions**

- `unsigned int width () const`  
  Width of render target in pixels.
- `unsigned int height () const`  
  Height of render target in pixels.
- `void makeTargetCurrent () const`  
  Make the render target current for rendering.
- `MStatus writeColorBuffer (const MImage &image, signed short x=0, signed short y=0, bool writeDepth=false) const`  
  Write a 2d image directly to the color buffer, without any transformation.


### MRenderTargetManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target_manager.html).

Provides access to MRenderTarget objects for use in Viewport 2.0. This class generates MRenderTarget objects for use with MRenderOperation objects. Any MRenderTarget objects created by this class are owned by the caller.

**Public Member Functions**

- `MRenderTarget * acquireRenderTarget (const MRenderTargetDescription &targetDescription) const`  
  Acquire an instance of a render target.
- `MRenderTarget * acquireRenderTargetFromScreen (const MString &targetName) const`  
  Acquire an instance of a render target with the same characteristics as the current on-screen target.
- `bool formatSupportsSRGBWrite (::MHWRender::MRasterFormat format) const`  
  This method will perform a check to determine whether gamma correction can be performed by the GPU when writing pixels to a render target of a given format.
- `void releaseRenderTarget (const MRenderTarget *target) const`  
  Deletes the MRenderTarget and releases the reference to the underlying target which is held by the MRenderTarget object.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MRenderUtil

Module `OpenMayaRender`. Source: [class_m_render_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_util.html).

Common API rendering functions. MRenderUtil is a class which provides static API methods to access Maya's rendering functionalities.

**Public Types**

- `enum MRenderState { kNotRendering, kBatchRender, kInteractiveRender, kIprRender, kHardwareRender }`  
  Defines the current rendering state of Maya.
- `enum MRenderPass { kAll, kColorOnly, kShadowOnly, kAmbientOnly, kDiffuseOnly, kSpecularOnly }`  
  Defines the current pass of Maya.

**Static Public Member Functions**

- `static MRenderState mayaRenderState ()`  
  Returns an enumerated type specifying the current state of the Maya renderer.
- `static MStatus raytrace (const MFloatVector &rayOrigin, const MFloatVector &rayDirection, const void *objectId, const void *raySampler, const short rayDepth, MFloatVector &resultColor, MFloatVector &resultTransparency, const bool isReflectedRays=true)`  
  This utility method provides functionality to raytrace from within a shader plug-in.
- `static MStatus raytrace (const MFloatVectorArray &rayOrigins, const MFloatVectorArray &rayDirections, const void *objectId, const void *raySampler, const short rayDepth, MFloatVectorArray &resultColors, MFloatVectorArray &resultTransparencies, const bool isReflectedRays=true)`  
  This utility method provides functionality to raytrace from within a shader plug-in.
- `static MStatus raytraceFirstGeometryIntersections (const MFloatVectorArray &rayOrigins, const MFloatVectorArray &rayDirections, const void *objectId, const void *raySampler, MFloatVectorArray &resultIntersections, MIntArray &resultIntersected)`  
  This utility method provides functionality to find the location of the first geometry intersected by a ray from within a shader plug-in.
- `static MStatus sampleShadingNetwork (MString shadingNodeName, int numSamples, bool useShadowMaps, bool reuseMaps, MFloatMatrix &cameraMatrix, MFloatPointArray *points, MFloatArray *uCoords, MFloatArray *vCoords, MFloatVectorArray *normals, MFloatPointArray *refPoints, MFloatVectorArray *tangentUs, MFloatVectorArray *tangentVs, MFloatArray *filterSizes, MFloatVectorArray &resultColors, MFloatVectorArray &resultTransparencies)`  
  This utility method allows you to sample a shading node/shading engine.
- `static bool generatingIprFile ()`  
  Returns true if an IPR file is being created.
- `static MString relativeFileName (const MString &absFileName, MStatus *ReturnStatus=NULL)`  
  Extract relative file name from the given (absolute) file name.
- `static bool exactFileTextureName (const MObject &fileNode, MString &texturePath, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file texture name specified on a file texture node into an absolute path to an image file on disk.
- `static bool exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, MString &exactName, MStatus *ReturnStatus=NULL)`  
  This method is obsolete.
- `static bool exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, const MString &contextNodeFullName, MString &exactName, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file name specified into an absolute path to a file on disk.
- `static bool exactFileTextureUvTileData (const MObject &fileNode, MStringArray &tilePaths, MFloatArray &tilePositions, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve all the paths to UV tiles for the given file texture node into absolute paths to image files on disk.
- `static bool convertPsdFile (const MObject &fileNode, MString &texturePath, const bool &forExport=false, MStatus *ReturnStatus=NULL)`  
  Internal use only.
- `static bool exactImagePlaneFileName (const MObject &imagePlaneNode, MString &texturePath, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file texture name specified on an image plane node into an absolute path to an image file on disk.
- `static MString exactFileTextureName (const MObject &fileNode, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file texture name specified on a file texture node into an absolute path to an image file on disk.
- `static MString exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, MStatus *ReturnStatus=NULL)`  
  This method is obsolete.
- `static MString exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, const MString &contextNodeFullName, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file name specified into an absolute path to a file on disk.
- `static MString convertPsdFile (const MObject &fileNode, const bool &forExport=false, MStatus *ReturnStatus=NULL)`  
  Internal use only.
- `static MString exactImagePlaneFileName (const MObject &imagePlaneNode, MStatus *ReturnStatus=NULL)`  
  Attempts to resolve the file texture name specified on an image plane node into an absolute path to an image file on disk.
- `static bool inCurrentRenderLayer (const MDagPath &objectPath, MStatus *ReturnStatus=NULL)`  
  Returns true if the specified path is in the current render layer.
- `static MRenderPass renderPass (void)`  
  Returns an enumerated type specifying the current pass the Maya renderer is executing.
- `static MString mainBeautyPassCustomTokenString ()`  
  Returns the custom token string of the main beauty pass for use by renderers in file naming.
- `static MString mainBeautyPassName ()`  
  Returns the name of the main beauty pass for use by renderers for token substitution.
- `static float diffuseReflectance (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &normal, bool lightBackFace, MStatus *ReturnStatus=NULL)`  
  This utility function computes and returns the diffuse or lambertian reflectance for a given light source and surface.
- `static MFloatVector maximumSpecularReflection (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &normal, const MFloatVector &rayDirection, MStatus *ReturnStatus=NULL)`  
  This utility function computes and returns the vector corresponding to the point on the light source that provides the maximum specular reflection.
- `static float lightAttenuation (const void *lightBlindData, const MFloatVector &pointCamera, const MFloatVector &normal, bool lightBackFace, MStatus *ReturnStatus=NULL)`  
  This utility function computes and returns light attentuation factor of a light.
- `static float hemisphereCoverage (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &rayDirection, bool lightBackFace, MStatus *ReturnStatus=NULL)`  
  This utility function computes and returns the fraction of the light that is transmitted by an object.
- `static void sendRenderProgressInfo (const MString &pixFile, int percentageDone)`  
  This utility function sends batch rendering status to Maya.
- `static void getCommonRenderSettings (MCommonRenderSettingsData &rgData)`  
  Fill the set of common render setting values.
- `static MStatus renderObjectItem (const void *objectId, MSelectionList &item)`  
  Procedure to look up a selection item for a given sample object id.
- `static const char * className ()`  
  Returns the name of this class.
- `static MStatus eval2dTexture (const MObject &texNode, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`  
  If a supported texture node is given, this method can be called to evaluate that texture at a number of (u,v) texture coordinate values.
- `static unsigned int noiseTableSize ()`  
  Access the size of Maya's noise table.
- `static unsigned int noiseTableCubeSide ()`  
  The size of Maya's noise table is a perfect cube.
- `static float valueInNoiseTable (unsigned int index)`  
  Returns a raw value from Maya's noise table at the given index.
- `static float noise1 (float x)`  
  Generates continous non-repeating one dimensional noise based on Maya's internal noise table.
- `static float noise2 (float x, float y)`  
  Generates continous non-repeating two dimensional noise based on Maya's internal noise table.
- `static float noise3 (float x, float y, float z)`  
  Generates continous non-repeating three dimensional noise based on Maya's internal noise table.
- `static float noise4 (float x, float y, float z, float t, unsigned short numTimeSteps)`  
  Generates continous four dimensional (space + time) noise based on Maya's internal noise table.


### MRenderUtilities

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_utilities.html).

Utilities for Viewport 2.0. This class is a utility class for rendering in Viewport 2.0

**Public Types**

- `enum MaterialViewerCamera { kPerspectiveCamera = 0, kOrthogonalCameraWithMargin, kOrthogonalCameraCloseUp }`  
  Camera positions supported by the Material Viewer.
- `enum MaterialViewerLights { kDefaultLights = 0, kSwatchLight, kAmbientLight }`  
  Light rigs supported by the Material Viewer.

**Static Public Member Functions**

- `static MDrawContext * acquireSwatchDrawContext (const MRenderTarget *colorTarget=NULL, const MRenderTarget *depthTarget=NULL)`  
  Acquire a draw context fit for rendering a swatch into the provided render targets.
- `static MDrawContext * acquireUVTextureDrawContext (const MRenderTarget *colorTarget=NULL, const MRenderTarget *depthTarget=NULL)`  
  Acquire a draw context fit for rendering a texture for the UV editor into the provided render targets.
- `static void releaseDrawContext (MDrawContext *context, bool releaseTargets=true)`  
  Release a draw context.
- `static void swatchBackgroundColor (float &r, float &g, float &b, float &a)`  
  This method returns the default background color for the hardware rendered swatch.
- `static MStatus blitTargetToImage (const MRenderTarget *target, MImage &image)`  
  Copy the data from a target to an image.
- `static MStatus blitTargetToGL (const MRenderTarget *target, const float region[2][2], bool unfiltered)`  
  Blit the data from a target to current GL context.
- `static MStatus drawSimpleMesh (MDrawContext &context, const MVertexBuffer &vertexBuffer, const MIndexBuffer &indexBuffer, MGeometry::Primitive primitiveType, int start, int count)`  
  Render a simple mesh.
- `static MStatus renderMaterialViewerGeometry (const MString &shape, MObject shaderNode, MImage &image, MaterialViewerCamera cameraMode=kPerspectiveCamera, MaterialViewerLights lightRig=kDefaultLights)`  
  Do an off-screen render replicating the results shown by the Material Viewer window of Hypershade.
- `static const char * className ()`  
  Returns the name of this class.


### MRenderView

Module `OpenMayaRender`. Source: [class_m_render_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_view.html).

Static class providing Render View API functions. This class provides access to the Maya Render View. The class allows plugins to send image data to the Render View in the same way that the Maya renderer does. Either a "full render" or a "region render" can be performed. In a full render, the Render View expects to receive pixel data that fills the entire image, while a region render expects only updates to a specified image region. Usage To send an image to the Render View, use the following sequence of calls: Call doesRenderEditorExist() to make sure that a Render View exists. If this returns false, then Maya is operating in batch mode. Call setCurrentCamera() to specify the name of the camera that is being rendered. Call startRender() or startRegionRender() to inform the Render View that you are about to start sending either a full image or a region update. When region rendering, the getRenderRegion() method can be used to retrieve the currently selected marquee region in the Render View. The image or image region is sent to the Render View as a series of one or more tiles. For each tile, send the tile's image data using the updatePixels() method. Call the refresh() method to refresh the Render View after each tile has been sent. Call endRender() to signal the Render View that all image data has been sent.

**Static Public Member Functions**

- `static bool doesRenderEditorExist ()`  
  Determines whether or not a Render View exists to receive image data.
- `static MStatus setCurrentCamera (MDagPath &camera)`  
  Informs the render client of the camera that will be performing the rendering.
- `static MStatus getRenderRegion (unsigned int &left, unsigned int &right, unsigned int &bottom, unsigned int &top)`  
  Retrieves the currently selected Render Region in Maya's Render View.
- `static MStatus startRender (unsigned int width, unsigned int height, bool doNotClearBackground=false, bool immediateFeedback=false)`  
  Informs the Render View that a full image render is about to begin.
- `static MStatus startRegionRender (unsigned int imageWidth, unsigned int imageHeight, unsigned int regionLeft, unsigned int regionRight, unsigned int regionBottom, unsigned int regionTop, bool doNotClearBackground=false, bool immediateFeedback=false)`  
  Informs the Render View that a region render is about to begin.
- `static MStatus updatePixels (unsigned int left, unsigned int right, unsigned int bottom, unsigned int top, RV_PIXEL *pPixels, bool isHdr=false, const unsigned int numberOfAOVs=0, const RV_AOV *const pAOVs=NULL)`  
  Sends a block of pixels to the Render View.
- `static MStatus refresh (unsigned int left, unsigned int right, unsigned int bottom, unsigned int top)`  
  Requests that the Render View refresh the display of a particular region of the displayed image.
- `static MStatus endRender ()`  
  Informs the Render View that the current render has completed.
- `static const char * className ()`  
  Returns the name of this class.


### MRenderer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_renderer.html).

Main interface class to the Viewport 2.0 renderer.

**Public Types**

- `enum MGPUMemType { kMemTexture, kMemAll }`  
  GPU memory type.
- `typedef void(* NotificationCallback) (MDrawContext &context, void *clientData)`  
  Definition for callback function for addNotification()

**Public Member Functions**

- `::MHWRender::DrawAPI drawAPI () const`  
  Returns the current drawing API.
- `bool drawAPIIsOpenGL () const`  
  Method will indicate whether the current drawing API is OpenGL or not.
- `unsigned int drawAPIVersion () const`  
  Method will return the version of drawing API.
- `void * GPUDeviceHandle () const`  
  Method will return a pointer to the GPU "device".
- `unsigned int GPUmaximumVertexBufferSize () const`  
  Method to return the maximum number of vertices allowed in a vertex buffer by the GPU device.
- `unsigned int GPUmaximumPrimitiveCount () const`  
  Method to return the maximum number of primitives that can be drawn per draw call by the GPU device.
- `void GPUmaximumOutputTargetSize (unsigned int &w, unsigned int &h) const`  
  Retrieve the maximum allowable dimensions for output target by the GPU device.
- `MInt64 GPUtotalMemorySize () const`  
  Retrieve the total amount of memory on the GPU device in bytes.
- `MInt64 GPUUsedMemorySize (MGPUMemType type) const`  
  Retrieve the amount of Maya occupied GPU memory in bytes.
- `MFragmentManager * getFragmentManager () const`  
  Access the fragment manager.
- `const MShaderManager * getShaderManager () const`  
  Access the shader manager.
- `const MRenderTargetManager * getRenderTargetManager () const`  
  Access the render target manager.
- `bool copyTargetToScreen (const MRenderTarget *renderTarget)`  
  Copy a render target to the screen.
- `bool render (const MString &sourceName, MRenderTarget **targets, unsigned int count)`  
  Render into render targets with given source name.
- `MTextureManager * getTextureManager () const`  
  Access the texture manager.
- `MStatus registerOverride (const MRenderOverride *roverride)`  
  Register the override as being usable by the renderer.
- `MStatus deregisterOverride (const MRenderOverride *roverride)`  
  Deregister an existing render override on the renderer.
- `const MRenderOverride * findRenderOverride (const MString &name)`  
  Return a reference to an existing render override registered with the renderer.
- `const MString activeRenderOverride () const`  
  Get the name of the active override.
- `unsigned int renderOverrideCount ()`  
  Get the number of registered render overrides.
- `MStatus setRenderOverrideName (const MString &name)`  
  Set the name of a render override (MHWRender::MRenderOverride) for batch rendering.
- `MString renderOverrideName (MStatus *ReturnStatus=NULL) const`  
  Get the current render override name used for batch rendering.
- `MStatus registerScriptCallback (MRenderScriptCallback *callback)`  
  Register a MRenderScriptCallback that can be then from a script fragment.
- `MStatus deregisterScriptCallback (MRenderScriptCallback *callback)`  
  Deregister a MRenderScriptCallback and delete the registered instance.
- `MStatus holdGPUMemory (MInt64 sizeInBytes, MInt64 *evictedGPUMemSize=NULL)`  
  Inform Maya that the total amount of GPU memory allocated by a plug-in is increasing by sizeInBytes.
- `MStatus releaseGPUMemory (MInt64 sizeInBytes)`  
  Inform Maya that the total amount of GPU memory allocated by a plug-in is decreasing by sizeInBytes.
- `void getStandardViewportOperations (MRenderOperationList &operations) const`  
  Return a list of new MRenderOperations representing the standard viewport pipeline.
- `void setOutputTargetOverrideSize (unsigned int w, unsigned int h)`  
  This method allows rendering to occur to an offscreen target which differs from the interactive 3d viewport size.
- `void getOutputTargetOverrideSize (int &w, int &h)`  
  Get the override dimensions set for the final output target.
- `void unsetOutputTargetOverrideSize ()`  
  Will unset any override dimensions set fot the final output target.
- `MStatus outputTargetSize (unsigned int &w, unsigned int &h) const`  
  Return the current dimensions for the output target.
- `bool setLightRequiresShadows (MObject obj, bool flag)`  
  The Viewport 2.0 renderer has a light limit option which is used to determine the number of lights used for computing illumination.
- `MStatus addNotification (NotificationCallback notification, const MString &name, const MString semanticLocation, void *clientData)`  
  Add a notification callback to the rendering pipeline.
- `MStatus removeNotification (const MString &name, const MString &semanticLocation)`  
  Remove a notification operation from the rendering pipeline.
- `unsigned int notificationCount (const MString &semanticLocation) const`  
  Returns the number of notifications that have been set for a given semantic location.
- `bool presentOnScreen () const`  
  Returns whether the contents rendered by VP2 will be presented to on-screen buffers when running Maya in interactive mode.
- `void setPresentOnScreen (bool val)`  
  Set whether the contents rendered by VP2 will be presented to on-screen buffers when running Maya in interactive mode.
- `bool useGradient () const`  
  Returns whether a gradient is being used to clear the background color.
- `MColor clearColor () const`  
  Returns the first clear color.
- `MColor clearColor2 () const`  
  Return the second clear color.

**Static Public Member Functions**

- `static MRenderer * theRenderer (bool initializeRenderer=true)`  
  Returns a static pointer to the renderer.
- `static void disableChangeManagementUntilNextRefresh ()`  
  Calling this method will cause Viewport 2.0 to stop processing all changes to the Maya scene until the next viewport refresh.
- `static void setGeometryDrawDirty (MObject obj, bool topologyChanged=true)`  
  Notify the Viewport 2.0 renderer that the geometry (size, shape, etc.) of obj has changed, causing the object to be updated in the viewport.
- `static void needEvaluateAllLights ()`  
  Notify the Viewport 2.0 renderer that it should evaluate all lights marked dirty, regardless of the light limit.
- `static void setLightsAndShadowsDirty ()`  
  Notify the Viewport 2.0 renderer that something has changed which requires re-evaluation of lighting and shadows.


### MRenderingInfo

Module `OpenMayaRender`. Source: [class_m_rendering_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_rendering_info.html).

Information to perform rendering into a hardware render target. MRenderingInfo is a class which holds information about rendering into hardware render targets.

**Public Member Functions**

- `unsigned int originX () const`  
  Origin (x) of region to render to in pixels.
- `unsigned int originY () const`  
  Origin (y) of region to render to in pixels.
- `unsigned int width () const`  
  Width of region to render in pixels.
- `unsigned int height () const`  
  Height of region to render in pixels.
- `MViewportRenderer::RenderingAPI renderingAPI () const`  
  Native target rendering API.
- `float renderingVersion () const`  
  Native target rendering version.
- `const MDagPath & cameraPath () const`  
  Current camera being used to render with.
- `const MRenderTargetLegacy & renderTarget () const`  
  Current render target.
- `const MMatrix & viewMatrix () const`  
  Current view matrix.
- `const MMatrix & projectionMatrix () const`  
  Current projection matrix.


### MSamplerState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sampler_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sampler_state.html).

Container class for an acquired complete GPU sampler state. MSamplerState wraps an acquired composite GPU sampler state. This sampler state can be used to set the current GPU sampler state using MStateManager::setSamplerState(myShader, 0, mySamplerState). Instances of MSamplerState cannot be created or modified by users. You can read the descriptor for an MSamplerState using MSamplerState::desc(), but you cannot set or alter the descriptor directly. MSamplerState can only be obtained via MStateManager::acquireSamplerState(). To use MStateManager::acquireSamplerState(), create and fill an MSamplerStateDesc instance with the requested state and pass it to MStateManager::acquireSamplerState() which will return the cached unique sampler state of that description. If the unique sampler state did not previously exist in the cache, it is created. The information included in the sampler state includes texture filter selection, texture address mode selection, mip LOD controls, anisotropy control, and the border color. Any single given sampler state can be bound to as many GPU sampler stages as needed. The advantages of using this class rather than setting sampler states explicitly are: 1. Since the states are cached internally, state setting is much faster than setting individual states. 2. Since state is set through the Maya viewport API, Maya can maintain accurate knowledge of the current GPU state, even in the presence of arbitrary plug-ins. This also speeds state setup by reducing redundant state setting and OGL queries. 3. The information is device aware, meaning that it will return the correct results based on the current active device. For example it will return the appropriate values for DirectX versus an OpenGL device.

**Public Types**

- `enum TextureFilter { kMinMagMipPoint = 0, kMinMagPoint_MipLinear = 1, kMinPoint_MagLinear_MipPoint = 0x4, kMinPoint_MagMipLinear = 0x5, kMinLinear_MagMipPoint = 0x10, kMinLinear_MagPoint_MipLinear = 0x11, kMinMagLinear_MipPoint = 0x14, kMinMagMipLinear = 0x15, kAnisotropic = 0x55 }`  
  Indicates how to filter a texture sample.
- `enum TextureAddress { kTexWrap = 1, kTexMirror = 2, kTexClamp = 3, kTexBorder = 4 }`  
  Indicates how to handle texture addresses beyond the canonical (0..1, 0..1) range.

**Public Member Functions**

- `const MSamplerStateDesc & desc (MStatus *ReturnStatus=NULL) const`  
  Get the sampler state descriptor that was used to create the state object.
- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a sampler state.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MSamplerStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sampler_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sampler_state_desc.html).

Descriptor for a complete sampler state. This class encapsulates a complete Sampler state.

**Public Member Functions**

- `MSamplerStateDesc ()`  
  Constructor, builds a default sampler state.
- `~MSamplerStateDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the sampler state to their default values.
- `MSamplerStateDesc (const MSamplerStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MSamplerStateDesc & operator= (const MSamplerStateDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `MSamplerState::TextureFilter filter`  
  Selects how to filter a texture sample, default kMinMagMipPoint.
- `MStateManager::CompareMode comparisonFn`  
  Selects the filter comparison function, default kCompareAlways.
- `MSamplerState::TextureAddress addressU`  
  Select the u coordinate addressing mode, default kTexWrap.
- `MSamplerState::TextureAddress addressV`  
  Select the v coordinate addressing mode, default kTexWrap.
- `MSamplerState::TextureAddress addressW`  
  Select the w coordinate addressing mode, default kTexWrap.
- `float borderColor [4]`  
  Set border color used for accesses beyond texture 0..1, default(0,0,0,0).
- `float mipLODBias`  
  Set a float bias to be added to the computed mip LOD level, default 0.
- `unsigned int minLOD`  
  Set minimum mip LOD level accessed, default 0.
- `unsigned int maxLOD`  
  Set maximum mip LOD level accessed, default 16.
- `unsigned int maxAnisotropy`  
  Set the maximum anisotropy permitted for anisotropic filters.
- `unsigned int coordCount`  
  Set the number of texture coordinates, default 2.
- `unsigned int elementIndex`  
  When using texture arrays, selects array element, default 0.


### MSceneRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_scene_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_scene_render.html).

Class which defines a scene render.

**Public Types**

- `enum MObjectTypeExclusions { kExcludeNone = 0, kExcludeNurbsCurves = 1<<(0), kExcludeNurbsSurfaces = 1<<(1), kExcludeMeshes = 1<<(2), kExcludePlanes = 1<<(3), kExcludeLights = 1<<(4), kExcludeCameras = 1<<(5), kExcludeJoints = 1<<(6), kExcludeIkHandles = 1<<(7), kExcludeDeformers = 1<<(8), kExcludeDynamics = 1<<(9), kExcludeParticleInstancers = 1<<(10), kExcludeLocators = 1<<(11), kExcludeDimensions = 1<<(12), kExcludeSelectHandles = 1<<(13), kExcludePivots = 1<<(14), kExcludeTextures = 1<<(15), kExcludeGrid = 1<<(16), kExcludeCVs = 1<<(17), kExcludeHulls = 1<<(18), kExcludeStrokes = 1<<(19), kExcludeSubdivSurfaces = 1<<(20), kExcludeFluids = 1<<(21), kExcludeFollicles = 1<<(22), kExcludeHairSystems = 1<<(23), kExcludeImagePlane = 1<<(24), kExcludeNCloths = 1<<(25), kExcludeNRigids = 1<<(26), kExcludeDynamicConstraints = 1<<(27), kExcludeManipulators = 1<<(28), kExcludeNParticles = 1<<(29), kExcludeMotionTrails = 1<<(30), kExcludeHoldOuts = 1<<(31), kExcludeAll = ~0 }`  
  Object type exclusions This enumeration is marked to be deprecated.
- `enum MSceneFilterOption { , kRenderPreSceneUIItems = 1 << (0), kRenderOpaqueShadedItems = 1 << (1), kRenderTransparentShadedItems = 1 << (2), kRenderShadedItems = (kRenderOpaqueShadedItems | kRenderTransparentShadedItems), kRenderPostSceneUIItems = 1 << (3) , kRenderAllItems = ~0 }`  
  Render filter options.
- `enum MDisplayMode { kNoDisplayModeOverride = 0, kWireFrame = 1<<(0), kShaded = 1<<(1), kFlatShaded = 1<<(2), kShadeActiveOnly = 1<<(3), kBoundingBox = 1<<(4), kDefaultMaterial = 1<<(5), kTextured = 1<<(6) }`  
  Display modes.
- `enum MLightingMode { kNoLightingModeOverride = 0, kNoLight, kAmbientLight, kLightDefault, kSelectedLights, kSceneLights }`  
  Lighting mode.
- `enum MPostEffectsOverride { kPostEffectDisableNone = 0, kPostEffectDisableSSAO = 1 << (0), kPostEffectDisableMotionBlur = 1 << (1), kPostEffectDisableDOF = 1 << (2), kPostEffectDisableAll = ~0 }`  
  Post effect override.
- `enum MCullingOption { kNoCullingOverride = 0, kCullNone, kCullBackFaces, kCullFrontFaces }`  
  Culling option.

**Public Member Functions**

- `MSceneRender (const MString &name)`  
  Constructor for a named scene render.
- `MSceneRender (const MString &name, const MString &fragmentName)`  
  Constructor for a named scene render that uses a custom fragment to render with.
- `~MSceneRender () override`  
  Default destructor for a scene render.
- `virtual void preRender ()`  
  Method to allow for the operation to update itself before being executed.
- `virtual void postRender ()`  
  Method to allow for the operation to clean up itself after being executed.
- `virtual void preSceneRender (const MDrawContext &context)`  
  Method to allow for the operation to update itself before a scene rendering begins.
- `virtual void postSceneRender (const MDrawContext &context)`  
  Method to allow for the operation to update itself after a scene rendering ends.
- `virtual MSceneFilterOption renderFilterOverride ()`  
  This method allows for the programmer to provide feedback on determining which elements of a scene render will be drawn based on semantic meaning.
- `virtual const MCameraOverride * cameraOverride ()`  
  Query for a camera override.
- `virtual const MSelectionList * objectSetOverride ()`  
  Query for override for the set of objects to view.
- `virtual const MShaderInstance * shaderOverride ()`  
  Query for a scene level shader override.
- `virtual MUint64 getObjectTypeExclusions ()`  
  Query for any object type exclusions.
- `virtual MDisplayMode displayModeOverride ()`  
  Query for any display mode override.
- `virtual MLightingMode lightModeOverride ()`  
  Query for any lighting mode override.
- `virtual const bool * shadowEnableOverride ()`  
  Query for shadow display override.
- `virtual MPostEffectsOverride postEffectsOverride ()`  
  Query for post effects override.
- `virtual MCullingOption cullingOverride ()`  
  Query for a face culling override.
- `virtual MClearOperation & clearOperation ()`  
  Get the scene clear operation.
- `virtual bool hasUIDrawables () const`  
  If you've overridden addPreUIDrawables() or addPostUIDrawables() then you must override this method to return true in order for them to have any effect.
- `virtual void addPreUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`  
  Provides access to the MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.
- `virtual void addPostUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`  
  Provides access to the MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.
- `MRenderParameters * getParameters ()`  
  Get the block of input parameters used to control the renderer.
- `MString fragmentName ()`  
  Get the name of the fragment to use when rendering the scene.
- `virtual MObjectTypeExclusions objectTypeExclusions ()`  
  This method is obsolete.

**Protected Attributes**

- `MClearOperation mClearOperation`  
  Clear operation.


### MSelectionContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_selection_context.html).

Class to allow control on Viewport 2.0 selection behavior. MSelectionContext provides control on selection behavior for a given DAG object. It cannot be instantiated by plug-ins. During the pre-filtering phase of Viewport 2.0 selection, an MSelectionContext instance is passed to the following functions for plug-ins to set up selection context for a custom object: MPxGeometryOverride::updateSelectionGranularity MPxSubSceneOverride::updateSelectionGranularity MPxDrawOverride::updateSelectionGranularity The MSelectionContext instance is only valid inside these functions, thus plug-ins should not save its pointer.

**Public Types**

- `enum SelectionLevel { kNone, kObject, kComponent, kFace, kEdge, kVertex }`  
  Specifies granularity level to use for the selection.

**Public Member Functions**

- `SelectionLevel selectionLevel (MStatus *ReturnStatus=nullptr) const`  
  Get the selection level used to pick items.
- `MStatus setSelectionLevel (SelectionLevel level)`  
  Set the selection level to use to pick items.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MSelectionInfo

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_selection_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_selection_info.html).

Class to allow access to the selection state information. MSelectionInfo encapsulates the selection state information. It is read-only and cannot be instantiated by plug-ins. During the hit test phase or the selection interpretation phase of Viewport 2.0 selection, an MSelectionInfo instance is passed to the following functions for plug-ins to query the current selection state information: MPxGeometryOverride::refineSelectionPath MPxDrawOverride::refineSelectionPath MPxDrawOverride::userSelect MRenderOverride::select The MSelectionInfo instance is only valid inside these functions, thus plug-ins should not save its pointer.

**Public Member Functions**

- `bool singleSelection (MStatus *ReturnStatus=nullptr) const`  
  Determines if we want to select a single shape or component.
- `bool selectClosest (MStatus *ReturnStatus=nullptr) const`  
  Determines if we want to select the closest shape or component.
- `bool selectable (const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`  
  Given the selection mask, determines if the shape is selectable.
- `bool selectableComponent (bool displayed, const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`  
  Given the selection mask, determines if the component is selectable.
- `MStatus selectRect (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`  
  Get the current selection rectangle dimensions.
- `MStatus cursorPoint (int &x, int &y) const`  
  Get the cursor point relative to the lower left corner of the viewport.
- `bool isRay (MStatus *ReturnStatus=nullptr) const`  
  Get whether or not there is a selection ray.
- `MMatrix getAlignmentMatrix (MStatus *ReturnStatus=nullptr) const`  
  Returns the alignment matrix.
- `MStatus getLocalRay (MPoint &pnt, MVector &vec) const`  
  Get the selection ray.
- `bool selectForHilite (const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`  
  Given the selection mask, determines if this shape can be selected for the hilite list.
- `bool selectOnHilitedOnly (MStatus *ReturnStatus=nullptr) const`  
  Determines if components can only be selected if the shape is hilited.
- `bool pointSnapping (MStatus *ReturnStatus=nullptr) const`  
  Determines if selection is launched to find snap points.
- `bool snapToActive (MStatus *ReturnStatus=nullptr) const`  
  Determines if active objects should be snapped to.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MShaderInstance

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_shader_instance.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_shader_instance.html).

An instance of a shader that may be used with Viewport 2.0. This class represents a shader that may be used with the MRenderItem class for rendering in Viewport 2.0.

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture1, kTexture2, kTexture3, kTextureCube, kSampler }`  
  Specifies parameter types.
- `enum FragmentUsage { kPixelShader, kNormalShader, kNormalShader2, kVertexShader, kGeometryShader, kGlossShader, kGlossShader2, kRotationAngleShader, kRotationAngleShader2, kReflectanceShader, kReflectanceShader2, kRoughnessShader, kRoughnessShader2, kAnisotropyShader, kAnisotropyShader2, kDisplacementPosShader }`  
  Specifies where a fragment will be connected and used.
- `typedef void(* DrawCallback) (MDrawContext &context, const MRenderItemList &renderItemList, MShaderInstance *shaderInstance)`  
  Definition for pre/post draw callback functions.

**Public Member Functions**

- `DrawCallback preDrawCallback () const`  
  Access the pre-draw callback function pointer set for the this shader instance.
- `DrawCallback postDrawCallback () const`  
  Access the post-draw callback function pointer set for the this shader instance.
- `void parameterList (MStringArray &list) const`  
  Get the names of all parameters that are settable on this shader instance.
- `ParameterType parameterType (const MString &parameterName) const`  
  Get the type of the named parameter, returns kInvalid if parameter is not found.
- `bool isArrayParameter (const MString &parameterName) const`  
  Determine whether the named parameter is an array.
- `int getArraySize (const MString &parameterName) const`  
  Get the size of the array.
- `MString semantic (const MString &parameterName) const`  
  Return the semantic for a named parameter.
- `bool isVaryingParameter (const MString &parameterName) const`  
  Determine whether the named parameter is varying or is uniform.
- `MStatus setParameter (const MString &parameterName, bool value)`  
  Set a named, boolean parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, int value)`  
  Set a named, integer parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, float value)`  
  Set a named, float parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, const float *value)`  
  Set a named, float, float2, float3 or float4 parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, const MFloatVector &value)`  
  Set a named, float, float2, float3 parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, const MMatrix &value)`  
  Set a named, matrix parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, const MFloatMatrix &value)`  
  Set a named, matrix parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`  
  Set a texture parameter on the shader instance.
- `MStatus setParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`  
  Set a named, texture parameter on the shader instance to a render target.
- `MStatus setParameter (const MString &parameterName, const MSamplerState &sampler)`  
  Set a sampler parameter on the shader instance.
- `MStatus setArrayParameter (const MString &parameterName, const bool *values, unsigned int count)`  
  Set a named, bool array parameter on the shader instance.
- `MStatus setArrayParameter (const MString &parameterName, const int *values, unsigned int count)`  
  Set a named, integer array parameter on the shader instance.
- `MStatus setArrayParameter (const MString &parameterName, const float *values, unsigned int count)`  
  Set a named, float array parameter on the shader instance.
- `MStatus setArrayParameter (const MString &parameterName, const MMatrix *values, unsigned int count)`  
  Set a named, matrix array parameter on the shader instance.
- `MStatus addInputFragment (const MString &fragmentName, const MString &outputName, const MString &inputName, const MString promotedInputName=MString())`  
  Connect a fragment that has been registered with the fragment manager to an input on the existing MShaderInstance.
- `MStatus addInputFragmentForMultiParams (const MString &fragmentName, const MString &uniqueName, const MStringArray &outputNames, const MStringArray &inputNames, MUintArray *invalidParameterIndices=nullptr, FragmentUsage fragmentUsage=kPixelShader)`  
  Connect a named fragment that has been registered with the MFragmentManager to this MShaderInstance.
- `MStatus addOutputFragment (const MString &fragmentName, const MString &inputName)`  
  Connect a fragment that has been registered with the fragment manager to an output on the existing MShaderInstance.
- `MStatus renameParameter (const MString &oldName, const MString &newName)`  
  Rename the parameter.
- `MStatus setSemantic (const MString &paramName, const MString &semantic)`  
  Set the semantic for a fragment based parameter.
- `MStatus setAsVarying (const MString &paramName, bool)`  
  Set the parameter to be a varying parameter or not for a fragment based parameter.
- `MStatus bind (const MDrawContext &context) const`  
  Binds the shader instance to the draw context, so that it is the active shader.
- `unsigned int getPassCount (const MDrawContext &context, MStatus *status=NULL) const`  
  Returns the number of draw passes defined by the shader.
- `MStatus activatePass (const MDrawContext &context, unsigned int pass) const`  
  Activates the given pass of the shader.
- `MStatus unbind (const MDrawContext &context) const`  
  Unbinds the shader instance from the draw context.
- `MStatus updateParameters (const MDrawContext &context) const`  
  Updates the bound shader instance with the current parameter data.
- `bool isTransparent () const`  
  Return whether the shader will render with transparency.
- `MStatus setIsTransparent (bool value)`  
  Set whether the shader will render with transparency.
- `MShaderInstance * clone () const`  
  Clone the shader.
- `MShaderInstance * createShaderInstanceWithColorManagementFragment (const MString &inputColorSpace)`  
  Return a new shader instance with Color Management fragment added, which is based on the callee.
- `MStatus requiredVertexBuffers (MVertexBufferDescriptorList &list) const`  
  Get the vertex buffer descriptors that describe the buffers required by a given shader instance.
- `int annotationAsInt (const MString &parameterName, const MString &annotationName, MStatus &status)`  
  Returns annotation value for a given parameter as integer.
- `float annotationAsFloat (const MString &parameterName, const MString &annotationName, MStatus &status)`  
  Returns annotation value for a given parameter as float.
- `MString annotationAsString (const MString &parameterName, const MString &annotationName, MStatus &status)`  
  Returns annotation value for a given parameter as MString.
- `MString parameterSemantic (const MString &parameterName, MStatus &status)`  
  Returns the semantic associated to a given parameter.
- `void * parameterDefaultValue (const MString &parameterName, MStatus &status)`  
  Returns the default value of a given parameter.
- `MString resourceName (const MString &parameterName, MStatus &status)`  
  Returns the resource name of a given texture parameter.
- `MString uiWidget (const MString &parameterName, MStatus &status)`  
  Returns the UI widget type associated with a given parameter.
- `MString uiName (const MString &parameterName, MStatus &uiName)`  
  Returns the UI name associated with a given parameter.
- `MStatus techniqueNames (MStringArray &techniqueNames) const`  
  Returns a list of the technique names for the effect.
- `int techniqueAnnotationAsInt (const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique annotation as integer.
- `float techniqueAnnotationAsFloat (const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique annotation as float.
- `MString techniqueAnnotationAsString (const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique annotation as MString.
- `int passAnnotationAsInt (unsigned int pass, const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique's pass annotation as int.
- `float passAnnotationAsFloat (unsigned int pass, const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique's pass annotation as float.
- `MString passAnnotationAsString (unsigned int pass, const MString &annotationName, MStatus &status)`  
  Returns the value of the current technique's pass annotation as MString.
- `MStatus writeEffectSourceToFile (const MString &filePath) const`  
  Write the source of the final OGSFX/HLSL/CgFX effect to a specified file.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MShaderManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_shader_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_shader_manager.html).

Provides access to MShaderInstance objects for use in Viewport 2.0. This class generates MShaderInstance objects for use with user-created MRenderItem objects. Any MShaderInstance objects created by this class are owned by the caller. The methods for adding shader and shader include paths will allow for additional search paths to be used with the "getEffectsFileShader()" method which searches for shader files on disk. The default search path is in the "Cg" and "HLSL" folders found in the runtime directory. It is recommended that user defined shaders not reside at this location to avoid any potential shader conflicts. When acquiring a shader instance the caller may also specify pre-draw and post-draw callback functions. These functions will be invoked any time a render item that uses the shader instance is about to be drawn. These callbacks are passed an MDrawContext object as well as the render item being drawn and may alter the draw state in order to get different rendering effects. Accessing Maya nodes from within these callbacks is an error and may result in instability.

**Public Types**

- `enum MStockShader { k3dSolidShader, k3dBlinnShader, k3dDefaultMaterialShader, k3dSolidTextureShader, k3dCPVFatPointShader, k3dColorLookupFatPointShader, k3dOpacityLookupFatPointShader, k3dColorOpacityLookupFatPointShader, k3dShadowerShader, k3dFatPointShader, k3dThickLineShader, k3dCPVThickLineShader, k3dDashLineShader, k3dCPVDashLineShader, k3dStippleShader, k3dThickDashLineShader, k3dCPVThickDashLineShader, k3dDepthShader, k3dCPVSolidShader, k3dIntegerNumericShader, k3dFloatNumericShader, k3dFloat2NumericShader, k3dFloat3NumericShader, k3dPointVectorShader, k3dPointLightShadowerShader, k3dStandardSurfaceShader }`  
  Name of an available "stock" shader.
- `typedef void(* LinkLostCallback) (MShaderInstance *shaderInstance, MUserData *userData)`  
  Definition for callback function triggered when a MShaderInstance acquired by calling getShaderFromNode() is no longer valid for that node.

**Public Member Functions**

- `MStatus addShaderPath (const MString &path) const`  
  Add a path to the list of shader search paths.
- `MStatus shaderPaths (MStringArray &paths) const`  
  Query the list of shader search paths.
- `MStatus addShaderIncludePath (const MString &path) const`  
  Add a path to the list of paths used for searching for shader include files.
- `MStatus shaderIncludePaths (MStringArray &paths) const`  
  Query the list of search paths user for searching for shader include files.
- `MStatus clearEffectCache () const`  
  Clear the effect cache.
- `MStatus removeEffectFromCache (const MString &effectsFileName, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0) const`  
  Remove an effect from the cache.
- `void getEffectsTechniques (const MString &effectsFileName, MStringArray &techniqueNames, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true) const`  
  Analyzes a given effect file to extract the names of the techniques that are defined.
- `MShaderInstance * getEffectsFileShader (const MString &effectsFileName, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`  
  Get a new instance of a shader generated from an effects file stored on disk.
- `MShaderInstance * getEffectsBufferShader (const void *buffer, unsigned int size, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`  
  Get a new instance of a shader generated from a block of memory containing device-specific source code (as char*).
- `MShaderInstance * getFragmentShader (const MString &fragmentName, const MString &structOutputName, bool decorateFragment, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`  
  Get a new instance of a shader generated from a named shader fragment or fragment graph.
- `MShaderInstance * getShaderFromNode (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback linkLostCb=0, MUserData *linkLostUserData=0, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0, bool nonTextured=false) const`  
  Get the shader instance by evaluating the shading network of a surface shader node (either standard or custom) in the scene.
- `MShaderInstance * getStockShader (MStockShader name, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`  
  Get a new instance of a stock shader.
- `void releaseShader (MShaderInstance *shader) const`  
  Deletes the MShaderInstance and releases its reference to the underlying shader which is held by the MShaderInstance object.

**Static Public Member Functions**

- `static bool isSupportedShaderSemantic (const MString &value)`  
  Return if a given string is a supported shader semantic.
- `static MString getLastError ()`  
  Get the description of the last error encountered by the shader manager regarding an effect.
- `static MString getLastErrorSource (bool displayLineNumber=false, bool filterSource=false, unsigned int numSurroundingLines=2)`  
  Get the source of the shader that generated the last error.
- `static const char * className ()`  
  Returns the name of this class.


### MStateManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_state_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_state_manager.html).

Class to allow efficient access to GPU state information. MStateManager can be used to provide efficient access to GPU state information when drawing in MPxShaderOverride or MPxDrawOverride. It cannot be created by the user, only accessed through MDrawContext. MStateManager works with GPU state in blocks for efficiency. These blocks are: blend state, rasterizer state, depth stencil state and sampler state. Each state block is represented by a different class. Users may acquire immutable instances of such classes by calling the relevant acquire methods on the state manager using a descriptor object to indicate what parameters the state block should have. Once acquired a state block may be held throughout the session and used to set the state where needed. Acquiring state blocks is relatively slow and expensive so it is recommended to avoid acquiring them during draw. State blocks are not large and so holding the memory should not be overly burdensome. For more details about the specific state block types, see MBlendState, MRasterizerState, MDepthStencilState and MSamplerState.

**Public Types**

- `enum CompareMode { kCompareNever = 1, kCompareLess = 2, kCompareEqual = 3, kCompareLessEqual = 4, kCompareGreater = 5, kCompareNotEqual = 6, kCompareGreaterEqual = 7, kCompareAlways = 8 }`  
  Indicates how to perform comparisons for buffers.
- `enum ShaderType { kNoShader, kVertexShader, kGeometryShader, kPixelShader, kHullShader, kDomainShader }`  
  Indicates which shader the sampler applies to.

**Public Member Functions**

- `MStatus setBlendState (const MBlendState *blendState)`  
  Sets the active blend state on the device.
- `const MBlendState * getBlendState (MStatus *ReturnStatus=NULL) const`  
  Gets the current active blend state from the device.
- `MStatus setRasterizerState (const MRasterizerState *rasterizerState)`  
  Sets the active rasterizer state on the device.
- `const MRasterizerState * getRasterizerState (MStatus *ReturnStatus=NULL) const`  
  Gets the current active rasterizer state from the device.
- `MStatus setDepthStencilState (const MDepthStencilState *depthStencilState)`  
  Sets the active depth stencil state on the device.
- `const MDepthStencilState * getDepthStencilState (MStatus *ReturnStatus=NULL) const`  
  Gets the current active DepthStencil state from the device.
- `MStatus setSamplerState (ShaderType shader, int samplerIndex, const MSamplerState *samplerState)`  
  Sets the active sampler state for any of the texture samplers on the device.
- `const MSamplerState * getSamplerState (ShaderType shader, int samplerIndex, MStatus *ReturnStatus=NULL) const`  
  Gets the current active sampler state from the device.

**Static Public Member Functions**

- `static const MBlendState * acquireBlendState (const MBlendStateDesc &blendStateDesc, MStatus *ReturnStatus=NULL)`  
  Acquires an immutable unique blend state matching the blend state descriptor, and returns it in a disposable container.
- `static void releaseBlendState (const MBlendState *state)`  
  Deletes the MBlendState and releases the reference to the underlying state object which is held by the MBlendState object.
- `static const MRasterizerState * acquireRasterizerState (const MRasterizerStateDesc &rasterizerStateDesc, MStatus *ReturnStatus=NULL)`  
  Acquires an immutable unique rasterizer state matching the rasterizer state descriptor and returns it in a disposable container.
- `static void releaseRasterizerState (const MRasterizerState *state)`  
  Deletes the MRasterizerState and releases the reference to the underlying state object which is held by the MRasterizerState object.
- `static const MDepthStencilState * acquireDepthStencilState (const MDepthStencilStateDesc &desc, MStatus *ReturnStatus=NULL)`  
  Acquires an immutable unique DepthStencil state matching the DepthStencil state descriptor and returns it in a disposable container.
- `static void releaseDepthStencilState (const MDepthStencilState *state)`  
  Deletes the MDepthStencilState and releases the reference to the underlying state object which is held by the MDepthStencilState object.
- `static int getMaxSamplerCount ()`  
  Get the maximum number of simulataneous texture coordinate interpolation channels.
- `static const MSamplerState * acquireSamplerState (const MSamplerStateDesc &samplerStateDesc, MStatus *ReturnStatus=NULL)`  
  Acquires an immutable unique Sampler state matching the Sampler state descriptor and returns it in a disposable container.
- `static void releaseSamplerState (const MSamplerState *state)`  
  Deletes the MSamplerState and releases the reference to the underlying state object which is held by the MSamplerState object.
- `static const char * className ()`  
  Returns the name of this class.


### MStencilOpDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_stencil_op_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_stencil_op_desc.html).

Descriptor for a depth-stencil operation. This class encapsulates a depth-stencil operation descriptor. There are separate MStencilOpDesc stencil operation descriptors for front and back faces in the depth stencil state.

**Public Member Functions**

- `MStencilOpDesc ()`  
  Constructor, builds a default stencil operation state.
- `~MStencilOpDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the stencil operation state to their default values.
- `MStencilOpDesc (const MStencilOpDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MStencilOpDesc & operator= (const MStencilOpDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `MDepthStencilState::StencilOperation stencilPassOp`  
  Stencil op to use when the fragment passes the stencil test, default kKeepStencil.
- `MDepthStencilState::StencilOperation stencilFailOp`  
  Stencil op to use when the fragment fails the stencil test, default kKeepStencil.
- `MDepthStencilState::StencilOperation stencilDepthFailOp`  
  Stencil op to use when the fragment passes the depth test, default kKeepStencil.
- `MStateManager::CompareMode stencilFunc`  
  Sets the stencil buffer comparison function, default kCompareAlways.


### MSubSceneContainer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sub_scene_container.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sub_scene_container.html).

Container for render items generated by MPxSubSceneOverride. An instance of this class is associated with each instance of MPxSubSceneOverride and is used to manage and store the set of render items required by the override to draw its associated DAG object. The container is optimized to be used with large numbers of render items. The container will assume ownership of any render item added to it. Implementations of MPxSubSceneOverride are free to maintain separate pointers to render items stored in the container, but those pointers will become invalid as soon as the associated render item is removed from the container.

**Public Member Functions**

- `bool add (MRenderItem *item)`  
  Add a render item to the set of render items that will be used to draw the DAG object associated with the override that owns this container.
- `bool remove (const MString &name)`  
  Remove a render item by name from the set of render items used to draw the object associated with the override that owns this container.
- `void clear ()`  
  Remove all render items from this container.
- `MRenderItem * find (const MString &name)`  
  Get a render item by name from the container.
- `unsigned int count () const`  
  Get the number of render items in the container.
- `const MRenderItem * find (const MString &name) const`  
  Get a render item by name from the container.
- `Iterator * getIterator ()`  
  Get an iterator for the container.
- `ConstIterator * getConstIterator () const`  
  Get a const iterator for the container.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MSwatchRenderBase

Module `OpenMayaRender`. Source: [class_m_swatch_render_base.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_swatch_render_base.html).

Swatch Render Base class. A class providing an interface through which a plugin can implement a class to provide custom rendered images for swatches which are displayed in hypershade and the Attribute Editor. By deriving from this class and implementing the doIteration function, swatches for nodes (having the required classification) can be custom rendered by the plugin. The derived class needs to be registered with Maya using the interface provided by MSwatchRenderRegister. Please refer to documentation of MSwatchRenderRegister for more details. To indicate that swatch for a certain node type will be generated by the plugin the classfication string of the node needs to contain the following string : "swatch/<swatch generator name>" The swatch generator name string should match the name used to register the swatch generator class.

**Public Member Functions**

- `MSwatchRenderBase (MObject obj, MObject renderObj, int res)`  
  Class constructor.
- `virtual ~MSwatchRenderBase ()`  
  Class destructor.
- `virtual bool doIteration ()`  
  Method called from the MSwatchRenderRegister for generation of swatch image.
- `virtual bool renderParallel ()`  
  Method indicates if the swatch is rendered parallel.
- `virtual void cancelParallelRendering ()`  
  Method to cancel the parallel rendering.
- `MObject swatchNode () const`  
  This method returns the node for which the swatch is required to be generated.
- `MObject node () const`  
  This method returns the node that is used to compute the swatch.
- `int resolution () const`  
  This method returns the expected resolution of the swatch.
- `MImage & image ()`  
  This method returns the the render swatch as an image.
- `void setRenderQuality (int)`  
  Method to set the render quality in which the swatch will be rendered.
- `int renderQuality () const`  
  Returns the quality in which the swatch will be rendered.

**Static Public Member Functions**

- `static void cancelCurrentSwatchRender ()`  
  The method cancels the swatch which is being rendered in parallel, and push the swatch render item back to the render queue after.
- `static void enableSwatchRender (bool enable)`  
  Enable/disable swatch rendering.

**Protected Member Functions**

- `void finishParallelRender ()`  
  Method to update the swatch image when the parallel rendering is finished.


### MSwatchRenderRegister

Module `OpenMayaRender`. Source: [class_m_swatch_render_register.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_swatch_render_register.html).

Manages swatch generators. Provides an interface for plugins to register/unregister swatch generator classes (derived from MSwatchRenderBase) with Maya. Whenever a swatch needs to be generated for a node, it checks the classfication string of the node for the preferred swatch generator. If a match is found, it creates and manages the swatch generator object (that is it deletes the swatch generator object once the image has been generated). The doIteration function is called for generating the swatch. The doIteration function is called repeatedly (during idle events) till it returns true. This allows for generation of the swatch in stages.

**Static Public Member Functions**

- `static MStatus registerSwatchRender (MString swatchGenName, MSwatchRenderCreatorFnPtr fnPtr)`  
  registers a new swatch generator creation function by name.
- `static MStatus unregisterSwatchRender (MString swatchGenName)`  
  removes the previously registered swatch generator


### MTargetBlendDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_target_blend_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_target_blend_desc.html).

Descriptor for a blend state for a single render target. This class holds the complete blend state for a single render target. The blended result will be: result = !BlendEnable ? source : blendOperation(sourceBlendOption(source), destinationBlendOption(destination)); MBlendStateDesc maintains an array of MTargetBlendDesc of size MBlendState::kMaxTargets.

**Public Member Functions**

- `MTargetBlendDesc ()`  
  Constructor, builds a default target blend descriptor.
- `~MTargetBlendDesc ()`  
  Destructor.
- `void setDefaults ()`  
  Set all values for the target blend state to their default values.
- `MTargetBlendDesc (const MTargetBlendDesc &rDesc)`  
  NO SCRIPT SUPPORT.
- `MTargetBlendDesc & operator= (const MTargetBlendDesc &rDesc)`  
  NO SCRIPT SUPPORT.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Public Attributes**

- `bool blendEnable`  
  Enable blending on this target, default is false.
- `MBlendState::BlendOption sourceBlend`  
  The blend factor for the source color, default is one.
- `MBlendState::BlendOption destinationBlend`  
  The blend factor for the destination color, default is zero.
- `MBlendState::BlendOperation blendOperation`  
  The blend operation, default is add.
- `MBlendState::BlendOption alphaSourceBlend`  
  The blend factor for the source alpha, default is one.
- `MBlendState::BlendOption alphaDestinationBlend`  
  The blend factor for the destination alpha, default is zero.
- `MBlendState::BlendOperation alphaBlendOperation`  
  The blend operation for alpha, default is add.
- `MBlendState::ChannelMask targetWriteMask`  
  Indicates what color components(red, green, blue, alpha) are writable, the default is RGBA.


### MTexture

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture.html).

Class which includes texture data.

**Public Member Functions**

- `void * resourceHandle () const`  
  This method allows access to the draw API dependent handle for a texture.
- `const MString & name () const`  
  Get the name of the texture.
- `void textureDescription (MTextureDescription &desc) const`  
  Get texture description.
- `bool hasAlpha () const`  
  Get whether the texture has an alpha channel.
- `MStatus setHasAlpha (bool value)`  
  Specify that the texture has an alpha channel.
- `bool hasZeroAlpha () const`  
  Get whether the texture has any texels with an alpha value of 0.0.
- `MStatus setHasZeroAlpha (bool value)`  
  Specify that the texture has texels with an alpha value of 0.0.
- `bool hasTransparentAlpha () const`  
  Get whether the texture has semi-transparent texels.
- `MStatus setHasTransparentAlpha (bool value)`  
  Specify that the texture has texels with an alpha value greater than or equal to 0.0 and less than 1.0.
- `unsigned int bytesPerPixel () const`  
  Get the number of bytes per pixel in the texture.
- `void * rawData (int &rowPitch, size_t &slicePitch) const`  
  Get a copy of the raw data mapped to the texture.
- `MStatus update (const void *pixelData, bool generateMipMaps, unsigned int rowPitch=0, MHWRender::MTextureUpdateRegion *region=NULL)`  
  Update the contents of an image with new data.
- `MStatus update (const MImage &image, bool generateMipMaps)`  
  Update the contents of an image with new data.
- `MStatus update (const MObject &textureNode)`  
  Attempt to update the texture given a file texture node.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.
- `static void freeRawData (void *)`  
  Convenience method that simply deletes the data that is allocated in the MTexture::rawData() method.


### MTextureArguments

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture_arguments.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture_arguments.html).

Class which includes texture arguments. This class encapsulates the information needed by the MTextureManager to acquire a texture from a file, such as the file path and mipmap levels. Encapsulating the texture information in an object allows you to store the information for easy re-use, and can simplify your texture loading code. MTextureArgments also accepts a file texture node, which is required if you want to activate background loading for the texture. You can set the file texture node through the MTextureArguments() constructor, or by calling MTextureArguments::setFileTextureNode(). To use this class, call the constructor to create an instance and set it up with the necessary parameters. When you need to get the texture, pass the object in a call to MTextureManager::acquireTexture(const MTextureArgments& args).

**Public Member Functions**

- `MTextureArguments (const MString &filePath, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`  
  Create the structure which includes the arguments to acquire a texture from disk.
- `void setContextNodeFullName (const MString &contextNodeFullName)`  
  Set the full name of the texture owner node which is provided as a context to the Maya resolver.
- `void setFileTextureNode (MObject node)`  
  Set the file texture node associated with the texture.


### MTextureManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture_manager.html).

Class which manages texture.

**Public Member Functions**

- `MStatus addImagePath (const MString &path)`  
  This method adds an additional search path for looking up images on disk.
- `MStatus imagePaths (MStringArray &paths) const`  
  The current set of image search paths can be retrieved using this method.
- `MTexture * acquireTexture (const MString &filePath, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`  
  This method is obsolete.
- `MTexture * acquireTexture (const MTextureArguments &args)`  
  Ask the renderer to acquire a hardware texture.
- `MTexture * acquireTexture (const MString &filePath, const MString &contextNodeFullName, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`  
  Ask the renderer to acquire a hardware texture.
- `MTexture * acquireTexture (const MString &textureName, const MPlug &plug, int width, int height, bool generateMipMaps=true)`  
  Sample the shading network connected to a plug to produce a texture.
- `MTexture * acquireTexture (const MObject &textureNode, bool allowBackgroundLoad=false)`  
  Acquire the texture associated with a given texture node.
- `MTexture * acquireTexture (const MString &textureName, const MHWRender::MTextureDescription &textureDesc, const void *pixelData, bool generateMipMaps=true)`  
  Ask the renderer to acquire a hardware texture by providing a texture description and a block of system memory data which matches the texture description.
- `MTexture * acquireTiledTexture (const MString textureName, const MStringArray &tilePaths, const MFloatArray &tilePositions, const MColor &undefinedColor, unsigned int width, unsigned int height, MStringArray &failedTilePaths, MFloatArray &uvScaleOffset)`  
  Ask the renderer to acquire a tiled hardware texture.
- `MTexture * acquireDepthTexture (const MString &textureName, const MImage &image, bool generateMipMaps, const MDepthNormalizationDescription *normalizationDesc)`  
  Ask the renderer to acquire a hardware texture from an MImage's depth buffer.
- `MTexture * acquireDepthTexture (const MString &textureName, float *pixelData, unsigned int width, unsigned int height, bool generateMipMaps, const MDepthNormalizationDescription *normalizationDesc)`  
  Ask the renderer to acquire a hardware texture from an array of depth values.
- `void releaseTexture (MTexture *texture) const`  
  Deletes the MTexture and releases the reference to the underlying texture which is held by the MTexture object.
- `MStatus saveTexture (MTexture *texture, const MString &filePath)`  
  Ask the renderer to save a hardware texture to disk.
- `MTexture * findTexture (const MString &textureName)`  
  Search the texture caching system for a previously cached texture with the given name.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MUIDrawManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_u_i_draw_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_u_i_draw_manager.html).

Main interface for drawing basic UI drawables in Viewport 2.0 and Hardware Renderer 2.0. The MUIDrawManager class provides a straight forward way to draw basic UI drawables like lines, icons, texts and other basic 2D/3D primitives. MUIDrawManager is not designed for accessing at arbitrary time or arbitrary places. Access to the interface is restricted to certain methods on various API interfaces. An instance of MUIDrawManager is constructed and passed to these methods together with any required data and context information. All drawing operations, including setting state like color and font size, must occur between calls to beginDrawable() and endDrawable(). For example, to draw a pair of red lines you would do the following: MColor red(1.0f, 0.0f, 0.0f);

**Public Types**

- `enum FontSize { kDefaultFontSize = 12, kSmallFontSize = 9 }`  
  Font size for drawing the text.
- `enum TextAlignment { kLeft, kCenter, kRight }`  
  Text alignment.
- `enum TextIncline { kInclineNormal = 0, kInclineItalic = 1, kInclineOblique = 2 }`  
  Text incline.
- `enum TextWeight { kWeightLight = 25, kWeightNormal = 50, kWeightDemiBold = 63, kWeightBold = 75, kWeightBlack = 87 }`  
  Text weight.
- `enum TextStretch { kStretchUltraCondensed = 50, kStretchExtraCondensed = 62, kStretchCondensed = 75, kStretchSemiCondensed = 87, kStretchUnstretched = 100, kStretchSemiExpanded = 112, kStretchExpanded = 125, kStretchExtraExpanded = 150, kStretchUltraExpanded = 200 }`  
  Text stretch.
- `enum TextLine { kLineNone, kLineOverline, kLineUnderline, kLineStrikeoutLine }`  
  Text line.
- `enum LineStyle { kSolid, kShortDotted, kShortDashed, kDashed, kDotted }`  
  Line style.
- `enum PaintStyle { kFlat, kStippled, kShaded }`  
  Paint style.
- `enum Primitive { kPoints, kLines, kLineStrip, kClosedLine, kTriangles, kTriStrip }`  
  Primitive.
- `enum Selectability { kNonSelectable, kSelectable, kAutomatic }`  
  Selectability.

**Public Member Functions**

- `void beginDrawable (Selectability selectability=kAutomatic, unsigned int selectionName=0)`  
  Resets all draw state, such as color and line style, to defaults and indicates the start of a sequence of drawing operations.
- `void beginDrawable (unsigned int name, bool nameIsPickable)`  
  This method is obsolete.
- `void endDrawable ()`  
  Indicates the end of a sequence of drawing operations.
- `void beginDrawInXray ()`  
  The drawables to be drawn between calls to beginDrawInXray() and endDrawInXray() will display on the top of other geometries in the scene, as the depth test is disabled for these drawables.
- `void endDrawInXray ()`  
  Pair with "MUIDrawManager::beginDrawInXray()".
- `void setColor (const MColor &color)`  
  Set the draw color.
- `void setColorIndex (const short index)`  
  Set the color index for the later primitive and text drawing.
- `void setPointSize (float value)`  
  Set the point size for the point drawing.
- `void setLineWidth (float value)`  
  Set the line width for the primitive drawing (line, rect, box...)
- `void setLineStyle (LineStyle style)`  
  Set the line style for the primitive drawing (line, rect, box...)
- `void setLineStyle (unsigned int factor, unsigned short pattern)`  
  Set the dashed line pattern for the primitive drawing (line, rect, box...)
- `void setPaintStyle (PaintStyle style)`  
  Set the paint style for filled primitive drawing.
- `unsigned int depthPriority () const`  
  Get the current depth priority value for primitive drawing.
- `void setDepthPriority (unsigned int priority)`  
  Set the depth priority for primitive drawing.
- `void line (const MPoint &startPoint, const MPoint &endPoint)`  
  Draw a straight line between two points.
- `void line2d (const MPoint &startPoint, const MPoint &endPoint)`  
  Draw a straight line between two points.
- `MStatus lineList (const MPointArray &points, bool draw2D)`  
  Draw a series of line segments in 3D or 2D.
- `MStatus lineStrip (const MPointArray &points, bool draw2D)`  
  Draw a series of connected line segments in 3D or 2D.
- `void point (const MPoint &point)`  
  Draw a point.
- `void point2d (const MPoint &point)`  
  Draw a point.
- `MStatus points (const MPointArray &points, bool draw2D)`  
  Draw a series of points in 3D or 2D.
- `void rect (const MPoint &center, const MVector &up, const MVector &normal, double scaleX, double scaleY, bool filled=false)`  
  Draw a rectangle.
- `void rect2d (const MPoint &center, const MVector &up, double scaleX, double scaleY, bool filled=false)`  
  Draw a 2D rectangle on the screen.
- `void sphere (const MPoint &center, double radius, bool filled=false)`  
  Draw a sphere.
- `void sphere (const MPoint &center, double radius, int subdivisionsAxis, int subdivisionsHeight, bool filled=false)`  
  Draw a sphere.
- `void circle (const MPoint &center, const MVector &normal, double radius, bool filled=false)`  
  Draw a circle.
- `void circle2d (const MPoint &center, double radius, bool filled=false)`  
  Draw a 2D circle on the screen.
- `void circle (const MPoint &center, const MVector &normal, double radius, int numSubdivision, bool filled)`  
  Draw a circle.
- `void circle2d (const MPoint &center, double radius, int numSubdivision, bool filled)`  
  Draw a 2D circle on the screen.
- `void arc (const MPoint &center, const MVector &start, const MVector &end, const MVector &normal, double radius, bool filled=false)`  
  Draw an arc.
- `void arc2d (const MPoint &center, const MVector &start, const MVector &end, double radius, bool filled=false)`  
  Draw a 2D arc on the screen.
- `void arc (const MPoint &center, const MVector &start, const MVector &end, const MVector &normal, double radius, int numSubdivisions, bool filled)`  
  Draw an arc.
- `void arc2d (const MPoint &center, const MVector &start, const MVector &end, double radius, int numSubdivisions, bool filled)`  
  Draw a 2D arc on the screen.
- `void mesh (Primitive mode, const MPointArray &position, const MVectorArray *normal=NULL, const MColorArray *color=NULL, const MUintArray *index=NULL, const MPointArray *texcoord=NULL)`  
  Draw custom geometric shapes from an array of vertices.
- `void mesh (Primitive mode, const MFloatPointArray &position, const MFloatVectorArray *normal=NULL, const MColorArray *color=NULL, const MUintArray *index=NULL, const MFloatPointArray *texcoord=NULL)`  
  Draw custom geometric shapes from an array of vertices.
- `void mesh2d (Primitive mode, const MPointArray &position, const MColorArray *color=NULL, const MUintArray *index=NULL, const MPointArray *texcoord=NULL)`  
  Draw custom 2d geometric shapes from an array of vertices.
- `void cone (const MPoint &base, const MVector &direction, double radius, double height, bool filled=false)`  
  Draw a cone.
- `void cone (const MPoint &base, const MVector &direction, double radius, double height, int subdivisionsCap, bool filled)`  
  Draw a cone.
- `void cylinder (const MPoint &center, const MVector &up, double radius, double height, int subdivisionsAxis, bool filled=false)`  
  Draw a cylinder.
- `void capsule (const MPoint &center, const MVector &up, double radius, double height, int subdivisionsAxis, int subdivisionsHeight, bool filled=false)`  
  Draw a capsule.
- `void box (const MPoint &center, const MVector &up, const MVector &right, double scaleX=1.0, double scaleY=1.0, double scaleZ=1.0, bool filled=false)`  
  Draw a box.
- `void setFontIncline (const int fontIncline)`  
  Set the incline of font to be used when drawing text.
- `void setFontWeight (const int fontWeight)`  
  Set the weight of font to be used when drawing text.
- `void setFontStretch (const int fontStretch)`  
  Set the stretch of font to be used when drawing text.
- `void setFontLine (const int fontLine)`  
  Set the line of font to be used when drawing text.
- `void setFontSize (const unsigned int fontSize)`  
  Set the size of font to be used when drawing text.
- `void setFontName (const MString &faceName)`  
  Set the face name of font to be used when drawing text.
- `void text (const MPoint &position, const MString &text, TextAlignment alignment=kLeft, const int *backgroundSize=NULL, const MColor *backgroundColor=NULL, bool dynamic=false)`  
  Draw a screen facing and horizontal aligned text in viewport 2.0.
- `void text2d (const MPoint &position, const MString &text, TextAlignment alignment=kLeft, const int *backgroundSize=NULL, const MColor *backgroundColor=NULL, bool dynamic=false)`  
  Draw a text on the screen.
- `void setTexture (MHWRender::MTexture *texture)`  
  Set the active texture to apply when drawing a mesh.
- `MStatus setTextureSampler (MHWRender::MSamplerState::TextureFilter filter, MHWRender::MSamplerState::TextureAddress address)`  
  Set the filter and address mode used when applying a texture to a mesh.
- `MStatus setTextureMask (MHWRender::MBlendState::ChannelMask mask)`  
  Set the channel mask to used when applying a texture to a mesh.
- `MStatus icon (const MPoint &position, const MString &iconName, float scale)`  
  Draw an icon at a given 3d position.

**Static Public Member Functions**

- `static unsigned int getFontList (MStringArray &list, bool usingOGSPainter=true)`  
  Get the names of all font faces that are available on current system.
- `static unsigned int getIconNames (MStringArray &names, bool usingOGSPainter=true)`  
  Get list of icon names.


### MUniformParameter

Module `OpenMayaRender`. Source: [class_m_uniform_parameter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uniform_parameter.html).

Uniform parameter. The MUniformParameter class provides a high-level interface to hardware shader uniform parameters. By defining your shader's uniform parameters through this class, you allow Maya to handle the attributes, editing, serialisation, and caching for you in a standard way that ensure you'll be able to leverage future performance and functionlity improvements. At setup time (either initial load or when the effect/technique is changed), your shader simply creates the list of parameters it requires, specifying the name, type, semantic of the parameters. At render time, you can then use the parameters to directly access the appropriate buffers for that surface data. If you include a custom Attribute Editor template for your shader node, you can include these surface parameters by calling the AEhwUniformTemplateParameters script function. The following sample code provides a basic template you can modify - however your AE template can use as much or as little of this as you like: global int $yourUniformNameTemplateInitialised = 0;

**Public Types**

- `enum DataType { kTypeUnknown, kTypeBool, kTypeInt, kTypeFloat, kType1DTexture, kType2DTexture, kType3DTexture, kTypeCubeTexture, kTypeEnvTexture, kTypeString, kTypeEnum }`  
  Parameter data types.
- `enum DataSemantic { kSemanticUnknown, kSemanticObjectDir, kSemanticWorldDir, kSemanticViewDir, kSemanticProjectionDir, kSemanticObjectPos, kSemanticWorldPos, kSemanticViewPos, kSemanticProjectionPos, kSemanticColor, kSemanticNormal, kSemanticBump, kSemanticEnvironment, kSemanticWorldMatrix, kSemanticWorldInverseMatrix, kSemanticWorldInverseTransposeMatrix, kSemanticViewMatrix, kSemanticViewInverseMatrix, kSemanticViewInverseTransposeMatrix, kSemanticProjectionMatrix, kSemanticProjectionInverseMatrix, kSemanticProjectionInverseTransposeMatrix, kSemanticWorldViewMatrix, kSemanticWorldViewInverseMatrix, kSemanticWorldViewInverseTransposeMatrix, kSemanticWorldViewProjectionMatrix, kSemanticWorldViewProjectionInverseMatrix, kSemanticWorldViewProjectionInverseTransposeMatrix, kSemanticColorTexture, kSemanticNormalTexture, kSemanticBumpTexture, kSemanticNormalizationTexture, kSemanticTranspDepthTexture, kSemanticOpaqueDepthTexture, kSemanticTime, kSemanticWorldTransposeMatrix, kSemanticViewTransposeMatrix, kSemanticProjectionTransposeMatrix, kSemanticWorldViewTransposeMatrix, kSemanticWorldViewProjectionTransposeMatrix, kSemanticViewProjectionMatrix, kSemanticViewProjectionInverseMatrix, kSemanticViewProjectionTransposeMatrix, kSemanticViewProjectionInverseTransposeMatrix, kSemanticLocalViewer, kSemanticViewportPixelSize, kSemanticBackgroundColor, kSemanticFrameNumber, kSemanticNearClipPlane, kSemanticFarClipPlane, kSemanticHWSPrimitiveBase, kSemanticHWSPrimitiveCountPerInstance, kSemanticHWSObjectLevel, kSemanticHWSFaceLevel, kSemanticHWSEdgeLevel, kSemanticHWSVertexLevel, kSemanticHWSOccluder, kSemanticHWSFrontCCW, kSemanticHWSInstancedDraw, kSemanticHWSHighlighting }`  
  Parameter semantics (i.e. what the parameter represents).

**Public Member Functions**

- `MUniformParameter ()`  
  Create an uninitialised (i.e.
- `MUniformParameter (const MString &name, DataType type, DataSemantic semantic, unsigned int numRows=1, unsigned int numColumns=1, void *userData=NULL)`  
  Create a new uniform parameter.
- `MUniformParameter (const MUniformParameter &src)`  
  Copy Constructor.
- `~MUniformParameter ()`  
  Class destructor.
- `const MUniformParameter & operator= (const MUniformParameter &other)`  
  Assignment operator.
- `MString name () const`  
  Get the name of this parameter.
- `MUniformParameter::DataType type () const`  
  Get the type of this parameter.
- `MUniformParameter::DataSemantic semantic () const`  
  Get the semantic of this parameter.
- `unsigned int numRows () const`  
  Get the number of rows in this parameter.
- `unsigned int numColumns () const`  
  Get the number of columns in this parameter.
- `unsigned int numElements () const`  
  Get the number of elements in this parameter (including rows and columns)
- `void * userData () const`  
  Get the user data for this parameter.
- `void setRangeMin (double rangeMin) const`  
  Sets the hard range lower bound for a numeric uniform parameter.
- `void setRangeMax (double rangeMax) const`  
  Sets the hard range upper bound for a numeric uniform parameter.
- `void setSoftRangeMin (double softRangeMin) const`  
  Sets the soft range lower bound for a numeric uniform parameter.
- `void setSoftRangeMax (double softRangeMax) const`  
  Sets the soft range upper bound for a numeric uniform parameter.
- `void setUIHidden (bool hiddenState) const`  
  Sets the hidden state of this parameter.
- `bool UIHidden () const`  
  Gets the hidden state of this parameter.
- `void setEnumFieldNames (const MString &fieldNames)`  
  Sets the field names of an enum attribute.
- `void setKeyable (bool keyable) const`  
  Sets the keyable state of this parameter.
- `bool keyable () const`  
  Gets the keyable state of this parameter.
- `bool isATexture () const`  
  Test if this parameter stores a texture.
- `bool hasChanged (const MGeometryList &iterator) const`  
  Has the value of this parameter changed since the last time it was accessed? This allows your shader to minimise state changes by only updating modified parameters.
- `bool hasChanged (const MHWRender::MDrawContext &context) const`  
  Has the value of this parameter changed since the last time it was accessed? This allows your shader to minimise state changes by only updating modified parameters.
- `const float * getAsFloatArray (const MGeometryList &iterator) const`  
  Get the value of this uniform parameter as one or more floating point values.
- `const float * getAsFloatArray (const MHWRender::MDrawContext &context) const`  
  Get the value of this uniform parameter as one or more floating point values.
- `void setAsFloatArray (const float *value, unsigned int maxElements) const`  
  Set the value of this uniform parameter as one or more floating point values.
- `float getAsFloat (const MGeometryList &iterator) const`  
  Get the value of this uniform parameter as a float.
- `float getAsFloat (const MHWRender::MDrawContext &context) const`  
  Get the value of this uniform parameter as a float.
- `void setAsFloat (float value) const`  
  Set the value of this uniform parameter as a float.
- `MString getAsString (const MGeometryList &iterator) const`  
  Get the value of this uniform parameter as a string.
- `MString getAsString (const MHWRender::MDrawContext &context) const`  
  Get the value of this uniform parameter as a string.
- `void setAsString (const MString &value) const`  
  Set the value of this uniform parameter as a string.
- `bool getAsBool (const MGeometryList &iterator) const`  
  Get the value of this uniform parameter as a boolean value.
- `bool getAsBool (const MHWRender::MDrawContext &context) const`  
  Get the value of this uniform parameter as a boolean value.
- `void setAsBool (bool value) const`  
  Set the value of this uniform parameter as a boolean value.
- `int getAsInt (const MGeometryList &iterator) const`  
  Get the value of this uniform parameter as an integer.
- `int getAsInt (const MHWRender::MDrawContext &context) const`  
  Get the value of this uniform parameter as an integer.
- `void setAsInt (int value) const`  
  Set the value of this uniform parameter as an integer value.
- `void setUINiceName (const MString &name)`  
  Sets the UI Nice Name for the attribute.
- `MPlug getPlug () const`  
  Get the plug managed by this parameter.
- `MPlug getSource () const`  
  Get the source plug connected to this parameter.
- `void setDirty ()`  
  Mark the data for this parameter as dirty.


### MUniformParameterList

Module `OpenMayaRender`. Source: [class_m_uniform_parameter_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uniform_parameter_list.html).

Uniform Parameter. MUniformParameterArray specify the list of uniform shader parameters used by a hardware shader, allowing Maya to handle setting up the node and user interfaces to the data, the population and access of cached data, etc.

**Public Member Functions**

- `MUniformParameterList ()`  
  Create a new, empty list of uniform parameters.
- `~MUniformParameterList ()`  
  Class destructor.
- `MUniformParameterList (const MUniformParameterList &src)`  
  Copy constructor.
- `void append (const MUniformParameter &element)`  
  Append a new parameter to this end of this list.
- `int length () const`  
  Get the number of parameters in this list.
- `void setLength (int n)`  
  Set the number of parameters in this list.
- `MUniformParameter getElement (int n) const`  
  Get the nth parameter in this list.
- `void setElement (int n, const MUniformParameter &p)`  
  Get the nth parameter in this list.
- `const MUniformParameterList & operator= (const MUniformParameterList &other)`  
  Assignment operator.


### MUserData

Module `OpenMayaRender`. Source: [class_m_user_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_user_data.html).

Virtual base class for user data caching. MUserData is a virtual base class meant to provide a means for users to attach blind data to certain Maya object types such that the lifetime of the blind data is managed by Maya.

**Public Member Functions**

- `MUserData (bool deleteAfterUse)`  
  This method is obsolete.
- `MUserData ()`  
  Construct an MUserData.
- `virtual ~MUserData ()`  
  Destructor.
- `bool deleteAfterUse ()`  
  This method is obsolete.
- `void setDeleteAfterUse (bool shouldDelete)`  
  This method is obsolete.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MUserRenderOperation

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_user_render_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_user_render_operation.html).

Class which defines a user defined rendering operation.

**Public Member Functions**

- `MUserRenderOperation (const MString &name)`  
  Constructor for named user render operation.
- `~MUserRenderOperation () override`  
  Default destructor for a user render operation.
- `virtual const MCameraOverride * cameraOverride ()`  
  Query for a camera override.
- `virtual MStatus execute (const MDrawContext &drawContext)=0`  
  Each user operation is responsible for "executing" itself.
- `virtual bool hasUIDrawables () const`  
  addUIdrawables() will be called only if this method is overridden to return true.
- `virtual void addUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`  
  Provides access to the MUIDrawManager, which can be used to queue up operations to draw simple UI shapes like lines, circles, text, etc.
- `virtual bool requiresLightData () const`  
  Indicates whether light data from the renderer is required for this user operation.


### MVaryingParameter

Module `OpenMayaRender`. Source: [class_m_varying_parameter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_varying_parameter.html).

Geometric data cache. The MVaryingParameter class provides a high-level interface to hardware shader varying parameters. By defining your shader's varying data through this class, you allow Maya to handle the attributes, editing, serialisation, requirements setup, and cache management for you in a standard way that ensure you'll be able to leverage future performance and functionlity improvements. At setup time (either initial load or when the effect/technique is changed), your shader simply creates the list of parameters it requires, specifying the name, type, semantic of the parameters. At render time, you can then use the parameters to directly access the appropriate buffers for that surface data. If you include a custom Attribute Editor template for your shader node, you can include these surface parameters by calling the AEhwShaderTemplateParameters script function. The following sample code provides a basic template you can modify - however your AE template can use as much or as little of this as you like: global int $yourShaderNameTemplateInitialised = 0;

**Public Types**

- `enum MVaryingParameterType { kInvalidParameter = -1, kStructure = 0, kFloat, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`  
  Parameter types.
- `enum MVaryingParameterSemantic { kNoSemantic = 0, kPosition, kNormal, kTexCoord, kColor, kWeight, kTangent = kWeight + 2, kBinormal }`  
  Parameter semantics (i.e. what the parameter represents).

**Public Member Functions**

- `MVaryingParameter ()`  
  Create an uninitialised (i.e.
- `MVaryingParameter (const MString &name, MVaryingParameterType type, int minDimension=1, int maxDimension=1, MVaryingParameterSemantic semantic=kNoSemantic, bool invertTexCoords=false, const MString &semanticName="")`  
  Create a new varying parameter.
- `MVaryingParameter (const MString &name, MVaryingParameterType type, int minDimension, int maxDimension, int dimension, MVaryingParameterSemantic semantic=kNoSemantic, const MString &destinationSet="", bool invertTexCoords=false, const MString &semanticName="")`  
  Create a new varying parameter.
- `MVaryingParameter (const MVaryingParameter &src)`  
  Copy Constructor.
- `~MVaryingParameter ()`  
  Class destructor.
- `const MVaryingParameter & operator= (const MVaryingParameter &other)`  
  Assignment operator.
- `const MString name () const`  
  Get the name of this parameter.
- `MVaryingParameter::MVaryingParameterType type () const`  
  Get the type of this parameter.
- `MVaryingParameter::MVaryingParameterSemantic semantic () const`  
  Get the semantic of this parameter.
- `const MString semanticName () const`  
  Get the semantic name assigned to this parameter The semanticName is used to identify a custom vertex stream request in order to fill the stream with the appropriate data requested by a shader override.
- `MString destinationSet () const`  
  Get the destination Set of this parameter.
- `int dimension () const`  
  Get the dimension of this parameter.
- `MStatus getBuffer (MGeometryLegacy &geometry, const void *&data, unsigned int &elements, unsigned int &count) const`  
  Get the data for this parameter as a system memory buffer.
- `MVaryingParameter::MVaryingParameterSemantic getSourceType () const`  
  Get the type of data (e.g.
- `MString getSourceSetName () const`  
  If the current data type supports data sets (e.g.
- `MStatus setSource (MVaryingParameter::MVaryingParameterSemantic type, const MString &setName)`  
  While the source of geometry parameters is usually configured by the artist through Maya's user interface, this method allows you to programatically set the source of a geometry parameter, including both the data type (e.g.
- `unsigned int getElementSize () const`  
  Get the size in bytes of one element of this parameter.
- `unsigned int getMaximumStride () const`  
  Get the maximum stride of this parameter in bytes.
- `MStatus addElement (const MVaryingParameter &child)`  
  Add a child element to this parameter.
- `unsigned int numElements () const`  
  Get the number of elements in this structure.
- `MVaryingParameter getElement (unsigned int i) const`  
  Get an element within a structure.
- `MStatus removeElements ()`  
  Remove all child elements from a structure.
- `unsigned int getUpdateId () const`  
  Get the update id.


### MVaryingParameterList

Module `OpenMayaRender`. Source: [class_m_varying_parameter_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_varying_parameter_list.html).

Geometric data cache. MVaryingParameterList specify the surface component level data used by a hardware shader, allowing Maya to handle setting up the node and user interfaces to the data, the population and access of cached data, etc.

**Public Member Functions**

- `MVaryingParameterList ()`  
  Create a new, empty list of varying parameters.
- `~MVaryingParameterList ()`  
  Class destructor.
- `MVaryingParameterList (const MVaryingParameterList &src)`  
  Copy constructor.
- `void append (const MVaryingParameter &element)`  
  Append a new parameter to this end of this list.
- `int length () const`  
  Get the number of parameters in this list.
- `void setLength (int n)`  
  Set the number of parameters in this list.
- `MVaryingParameter getElement (int n) const`  
  Get the nth parameter in this list.
- `void setElement (int n, const MVaryingParameter &p)`  
  Set the nth parameter in this list.
- `const MVaryingParameterList & operator= (const MVaryingParameterList &other)`  
  Assignment operator.


### MVertexBuffer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer.html).

Vertex buffer for use with MGeometry. This class represents a vertex buffer with attributes described by the descriptor member. When retrieving a vertex buffer for binding to custom shaders (MPxShaderOverride), resourceHandle() may be called to get the device dependent handle to the vertex buffer on the GPU. When creating a vertex buffer to supply geometric data to Maya (MPxGeometryOverride), acquire() may be called to get a pointer to a block of memory to fill with said data. Once filled, commit() must be called to apply the data to the buffer. When exploring the contents of an already filled vertex buffer, map() may be called to get a pointer to the read-only block of memory containing the data. Once done exploring, unmap() must be called to allow the buffer to be reused.

**Public Member Functions**

- `MVertexBuffer (const MVertexBufferDescriptor &descriptor, bool softwareStaged=true)`  
  Constructor.
- `MVertexBuffer (const MVertexBuffer &)`  
  Copy constructor.
- `~MVertexBuffer ()`  
  Destructor.
- `const MVertexBufferDescriptor & descriptor () const`  
  Get the the buffer descriptor.
- `void * resourceHandle () const`  
  Gets a graphics device dependent handle to a vertex buffer.
- `void resourceHandle (void *handle, unsigned int size)`  
  Directly set the graphics-device-dependent hardware buffer resource handle.
- `bool hasCustomResourceHandle () const`  
  Returns true if this vertex buffer is using a custom resource handle set by the plugin using MVertexBuffer::resourceHandle(void*, unsigned int).
- `void lockResourceHandle ()`  
  Lock the resource handle.
- `void unlockResourceHandle ()`  
  Unlock the resource handle.
- `void * acquire (unsigned int size, bool writeOnly)`  
  Get a pointer to memory for the buffer.
- `void commit (void *buffer)`  
  Commit the data stored in the memory given by acquire() to the buffer.
- `void * map ()`  
  Get a read-only pointer to the existing content of the buffer.
- `void unmap ()`  
  Release the data exposed by map().
- `MStatus update (const void *buffer, unsigned int destOffset, unsigned int numVerts, bool truncateIfSmaller)`  
  Set a portion (or all) of the contents of the MVertexBuffer using the data in the provided software buffer.
- `void unload ()`  
  If the buffer is resident in GPU memory, calling this method will move it to system memory and free the GPU memory.
- `unsigned int vertexCount () const`  
  Get the size of the vertex buffer.
- `void * acquire (unsigned int size)`  
  This method is obsolete.
- `void * getVertexBuffer () const`  
  Private friend class only access to the internal OGS vertex buffer.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MVertexBufferArray

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_array.html).

Array of Vertex buffers. This class represents an array of vertex buffers.

**Public Member Functions**

- `MVertexBufferArray ()`  
  Constructor.
- `~MVertexBufferArray ()`  
  Destructor.
- `unsigned int count () const`  
  Get num of vertex buffers in list.
- `MVertexBuffer * getBuffer (const MString &name) const`  
  Get vertex buffer based on name.
- `MVertexBuffer * getBuffer (unsigned int index) const`  
  Get vertex buffer based at desired index.
- `MString getName (unsigned int index) const`  
  Get the name of the buffer at desired index.
- `MStatus addBuffer (const MString &name, MVertexBuffer *buffer)`  
  Add a new vertex buffer to the list.
- `void clear ()`  
  Clear the array.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MVertexBufferDescriptor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_descriptor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_descriptor.html).

Describes properties of a vertex buffer. This class gives a complete description of a vertex buffer and is used by MGeometry, MVertexBuffer and MGeometryRequirements to simplify management of vertex buffer attributes. Note that not all combinations of semantics, data types and dimensions are currently supported. Attempts to create vertex buffers with unsupported attributes will fail.

**Public Member Functions**

- `MVertexBufferDescriptor ()`  
  Constructor.
- `MVertexBufferDescriptor (const MString &name, MGeometry::Semantic semantic, MGeometry::DataType dataType, int dimension)`  
  Constructor.
- `MVertexBufferDescriptor (const MString &name, MGeometry::Semantic semantic, const MString &semanticName, MGeometry::DataType dataType, int dimension)`  
  Constructor.
- `~MVertexBufferDescriptor ()`  
  Destructor.
- `MVertexBufferDescriptor (const MVertexBufferDescriptor &other)`  
  NO SCRIPT SUPPORT.
- `MVertexBufferDescriptor & operator= (const MVertexBufferDescriptor &other)`  
  NO SCRIPT SUPPORT.
- `MString name () const`  
  Get the name of the buffer.
- `void setName (const MString &n)`  
  Set the name of the buffer The buffer name is used to determine which render item this buffer belongs to.
- `MGeometry::Semantic semantic () const`  
  Get the semantic of the buffer The semantic is used by the hardware effects system and must be one of the predefined types.
- `void setSemantic (MGeometry::Semantic s)`  
  Set the semantic of the buffer.
- `MString semanticName () const`  
  Get the semantic name of the buffer The semanticName is used to identify a custom vertex stream request in order to fill the stream with the appropriate data requested by a shader override.
- `void setSemanticName (const MString &n)`  
  Set the semantic name of the buffer The semanticName is used to identify a custom vertex stream request in order to fill the stream with the appropriate data requested by a shader override.
- `MGeometry::DataType dataType () const`  
  Get the data type of the buffer.
- `void setDataType (MGeometry::DataType d)`  
  Set the data type of the buffer.
- `unsigned int dataTypeSize () const`  
  Get the size in bytes of the data type of the buffer.
- `int dimension () const`  
  Get the dimension of the buffer.
- `void setDimension (int d)`  
  Set the dimension of the buffer.
- `int offset () const`  
  Get the offset of the vertex buffer.
- `void setOffset (int o)`  
  Set the offset of the vertex element in an interleaved vertex buffer.
- `int stride () const`  
  Get the stride of the vertex buffer.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MVertexBufferDescriptorList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_descriptor_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_descriptor_list.html).

A list of MVertexBufferDescriptor objects. A list of MVertexBufferDescriptor objects.

**Public Member Functions**

- `MVertexBufferDescriptorList ()`  
  Constructor.
- `~MVertexBufferDescriptorList ()`  
  Destructor.
- `int length () const`  
  Get the number of descriptors in the list.
- `bool getDescriptor (int index, MVertexBufferDescriptor &desc) const`  
  Gets the descriptor at the requested index and copies data into desc.
- `bool append (const MVertexBufferDescriptor &desc)`  
  Add a descriptor to the list.
- `bool removeAt (int index)`  
  Remove a descriptor from the list and delete it.
- `void clear ()`  
  Clears the list.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MViewportRenderer

Module `OpenMayaRender`. Source: [class_m_viewport_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_viewport_renderer.html).

MViewportRenderer is a class which represents a hardware viewport renderer.

**Public Types**

- `enum RenderingAPI { kOpenGL, kDirect3D, kSoftware }`  
  API used for rendering.
- `enum RenderingOverride { kNoOverride, kOverrideAllDrawing, kOverrideThenStandard, kOverrideThenUI }`  
  Override status.
- `enum RenderingExclusion`  
  Rendering exclusions when RenderingOverride is kOverrideThenStandard.

**Public Member Functions**

- `MViewportRenderer (const MString &name)`  
  Constructor.
- `virtual ~MViewportRenderer ()`  
  Destructor.
- `virtual MStatus initialize ()=0`  
  Renderer initialization.
- `virtual MStatus uninitialize ()=0`  
  Renderer cleanup.
- `virtual MStatus render (const MRenderingInfo &info)=0`  
  Method which does the actual rendering.
- `virtual bool nativelySupports (MViewportRenderer::RenderingAPI api, float version)=0`  
  Query the native rendering API's supported by this renderer.
- `virtual bool override (MViewportRenderer::RenderingOverride override)=0`  
  Check if override exists.
- `virtual unsigned int overrideThenStandardExclusion () const`  
  Rendering exclusion for standard pass of kOverrideThenStandard.
- `MStatus registerRenderer () const`  
  Register the renderer.
- `MStatus deregisterRenderer () const`  
  Deregister the renderer.
- `const MString & name () const`  
  Return the internal name of the renderer.
- `const MString & UIname () const`  
  Return the name of the renderer as it should appear to users.
- `void setUIName (const MString &name)`  
  Set the name of the renderer as it should appear to users.
- `MViewportRenderer::RenderingOverride renderingOverride () const`  
  Return the override status.
- `void setRenderingOverride (RenderingOverride override)`  
  Set the rendering override for the renderer.

**Friends**

- `class MHardwareRenderer`

