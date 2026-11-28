# OpenMayaRender

Class names, bases and signatures of the module, checked against the reference's 2022 C++ API reference (https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_render.html). Descriptions are not kept here; follow the source link on a class for them.

Classes in this module: 120.

## Classes

### MAttributeParameterMapping

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_attribute_parameter_mapping.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_attribute_parameter_mapping.html).

**Public Member Functions**

- `MAttributeParameterMapping (const MString &paramName, const MString &attrName, bool allowConnection, bool allowRename)`
- `~MAttributeParameterMapping ()`
- `const MString & parameterName () const`
- `const MString & resolvedParameterName () const`
- `const MString & attributeName () const`
- `bool allowConnection () const`
- `bool allowRename () const`

**Static Public Member Functions**

- `static const char * className ()`

### MAttributeParameterMappingList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_attribute_parameter_mapping_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_attribute_parameter_mapping_list.html).

**Public Member Functions**

- `MAttributeParameterMappingList ()`
- `~MAttributeParameterMappingList ()`
- `unsigned int length () const`
- `const MAttributeParameterMapping * getMapping (unsigned int index) const`
- `const MAttributeParameterMapping * findByAttributeName (const MString &attributeName) const`
- `const MAttributeParameterMapping * findByParameterName (const MString &parameterName) const`
- `void append (const MAttributeParameterMapping &mapping)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MBlendState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_blend_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_blend_state.html).

**Public Types**

- `enum BlendOperation { kAdd = 0x1, kSubtract = 0x2, kReverseSubtract = 0x3, kMin = 0x4, kMax = 0x5 }`
- `enum BlendOption { kZero = 1, kOne = 2, kSourceColor = 3, kInvSourceColor = 4, kSourceAlpha = 5, kInvSourceAlpha = 6, kSourceAlphaSat = 11, kDestinationColor = 9, kInvDestinationColor = 10, kDestinationAlpha = 7, kInvDestinationAlpha = 8, kBothSourceAlpha = 12, kBothInvSourceAlpha = 13, kBlendFactor = 14, kInvBlendFactor = 15 }`
- `enum ChannelMask { kNoChannels = 0, kRedChannel = 0x1, kGreenChannel = 0x2, kBlueChannel = 0x4, kAlphaChannel = 0x8, kRGBChannels = 0x7, kRGBAChannels = 0xf }`

**Public Member Functions**

- `const MBlendStateDesc & desc (MStatus *ReturnStatus=NULL) const`
- `void * resourceHandle () const`
- `~MBlendState ()`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static const int kMaxTargets = 8`

### MBlendStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_blend_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_blend_state_desc.html).

**Public Member Functions**

- `MBlendStateDesc ()`
- `~MBlendStateDesc ()`
- `void setDefaults ()`
- `MBlendStateDesc (const MBlendStateDesc &rDesc)`
- `MBlendStateDesc & operator= (const MBlendStateDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `bool alphaToCoverageEnable`
- `bool independentBlendEnable`
- `float blendFactor [4]`
- `unsigned int multiSampleMask`
- `MTargetBlendDesc targetBlends [MBlendState::kMaxTargets]`

### MClearOperation

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_clear_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_clear_operation.html).

**Public Types**

- `enum ClearMask { kClearNone = 0, kClearColor = 1, kClearDepth = 1 << 1, kClearStencil = 1 << 2, kClearAll = ~0 }`

**Public Member Functions**

- `MClearOperation (const MString &name)`
- `~MClearOperation () override`
- `bool overridesColors () const`
- `unsigned int mask () const`
- `const float * clearColor () const`
- `bool clearGradient () const`
- `const float * clearColor2 () const`
- `int clearStencil () const`
- `float clearDepth () const`
- `void setOverridesColors (bool override)`
- `void setMask (unsigned int mask)`
- `void setClearColor (float value[4])`
- `void setClearGradient (bool value)`
- `void setClearColor2 (float value[4])`
- `void setClearStencil (int value)`
- `void setClearDepth (float value)`

**Public Attributes**

- `unsigned int mClearMask`
- `float mClearColor [4]`
- `float mClearColor2 [4]`
- `bool mClearGradient`
- `int mClearStencil`
- `float mClearDepth`

### MColorManagementNodes

Module `OpenMayaRender`. Source: [class_m_color_management_nodes.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_nodes.html).

**Static Public Member Functions**

- `static const char * className ()`
- `static bool isColorManageable (const MObject &object)`
- `static bool isColorManaged (const MObject &object)`
- `static MStatus colorManage (MObject &object, const MString &filename, MDGModifier &dgModifier)`
- `static MStatus colorManageAllNodes (MDGModifier &dgModifier)`

### MColorManagementUtilities

Module `OpenMayaRender`. Source: [class_m_color_management_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_utilities.html).

**Classes**

- `class MColorTransformData`

**Static Public Member Functions**

- `static const char * className ()`
- `static MStatus getColorTransformCacheIdForInputSpace (const MString &inputSpaceName, MString &transformId)`
- `static MStatus getColorTransformCacheIdForOutputTransform (MString &transformId)`
- `static bool isColorManagementEnabled ()`
- `static bool isColorManagementAvailable ()`
- `static MStatus connectDependencyNodeToColorManagement (MObject &object)`

### MColorManagementUtilities::MColorTransformData

Module `OpenMayaRender`. Source: [class_m_color_management_utilities_1_1_m_color_transform_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_management_utilities_1_1_m_color_transform_data.html).

**Public Member Functions**

- `MColorTransformData ()`
- `~MColorTransformData ()`
- `unsigned getSize () const`
- `const void * getData () const`

### MColorPickerCallback

Module `OpenMayaRender`. Source: [class_m_color_picker_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_picker_callback.html).

**Public Member Functions**

- `MColorPickerCallback (QWidget *key)`
- `virtual ~MColorPickerCallback ()`
- `virtual MColor getColor (QWidget *, const QPoint &, bool) const =0`
- `QWidget * getKey () const`

**Static Public Member Functions**

- `static const char * className ()`

### MColorPickerUtilities

Module `OpenMayaRender`. Source: [class_m_color_picker_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_color_picker_utilities.html).

**Static Public Member Functions**

- `static bool isRegisteredToColorPicking (QWidget *)`
- `static MStatus doRegisterToColorPicking (QWidget *, MColorPickerCallback *)`
- `static MStatus unregisterFromColorPicking (QWidget *)`
- `static const char * className ()`

### MCommonRenderSettingsData

Module `OpenMayaRender`. Source: [class_m_common_render_settings_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_common_render_settings_data.html).

**Public Types**

- `enum MpathType { kRelativePath, kFullPathImage, kFullPathTmp }`

**Public Member Functions**

- `MCommonRenderSettingsData ()`
- `~MCommonRenderSettingsData ()`
- `bool isAnimated () const`
- `bool isMovieFormat () const`
- `void setPassName (const MString &pass)`
- `void setFieldName (const MString &field)`
- `const MString getImageName (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const bool createDirectory=true, MStatus *ReturnStatus=NULL) const`
- `const MString getImageName (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const MString &customTokenString, const bool createDirectory=true, MStatus *ReturnStatus=NULL) const`
- `bool shouldRenderFrameAtTime (MpathType pathType, double frameNumber, const MString &sceneName, const MString &cameraName, const MString &fileFormat, const MObject layer, const MString &customTokenString) const`
- `const MString getBufferName (const MObject &renderPass, const MObject &layer, const MString &cameraName, const MString &customTokenString="", const bool leaveUnmatchedTokens=false, MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static MString getPreRenderFrameCmd ()`
- `static MString getPostRenderFrameCmd ()`
- `static const char * className ()`

**Public Attributes**

- `MString name`
- `unsigned int namingScheme`
- `unsigned int framePadding`
- `MString namePattern`
- `unsigned int imageFormat`
- `MString customImageFormat`
- `MTime frameStart`
- `MTime frameEnd`
- `float frameBy`
- `bool skipExistingFrames`
- `bool renderAll`
- `bool useCustomExt`
- `MString customExt`
- `bool renumberFrames`
- `float renumberStart`
- `float renumberBy`
- `unsigned int width`
- `unsigned int height`
- `float deviceAspectRatio`
- `float pixelAspectRatio`
- `float dotPerInch`
- `bool enableDefaultLight`
- `MString preMel`
- `MString postMel`
- `MString preRenderLayerMel`
- `MString postRenderLayerMel`
- `MString preRenderMel`
- `MString postRenderMel`

**Friends**

- `class MRenderUtil`

### MComponentDataIndexing

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_component_data_indexing.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_component_data_indexing.html).

**Public Member Functions**

- `MUintArray & indices ()`
- `const MUintArray & indices () const`
- `MComponentType componentType () const`
- `void setComponentType (MComponentType type)`

### MComponentDataIndexingList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_component_data_indexing_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_component_data_indexing_list.html).

**Public Member Functions**

- `MComponentDataIndexingList ()`
- `~MComponentDataIndexingList ()`
- `int length () const`
- `const MComponentDataIndexing * operator[] (int index) const`
- `MComponentDataIndexing * operator[] (int index)`
- `bool append (const MComponentDataIndexing &desc)`
- `bool removeAt (int index)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MD3D9Renderer

Module `OpenMayaRender`. Source: [class_m_d3_d9_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_d3_d9_renderer.html).

**Public Member Functions**

- `MStatus makeSwatchContextCurrent (unsigned int &width, unsigned int &height)`
- `MStatus setBackgroundColor (const MColor &color)`
- `MStatus readSwatchContextPixels (MImage &image)`

**Static Public Member Functions**

- `static MD3D9Renderer * theRenderer ()`

### MDepthStencilState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_depth_stencil_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_depth_stencil_state.html).

**Public Types**

- `enum StencilOperation { kKeepStencil = 1, kZeroStencil = 2, kReplaceStencil = 3, kIncrementStencilSat = 4, kDecrementStencilSat = 5, kInvertStencil = 6, kIncrementStencil = 7, kDecrementStencil = 8 }`

**Public Member Functions**

- `const MDepthStencilStateDesc & desc (MStatus *ReturnStatus=NULL) const`
- `void * resourceHandle () const`
- `~MDepthStencilState ()`

**Static Public Member Functions**

- `static const char * className ()`

### MDepthStencilStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_depth_stencil_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_depth_stencil_state_desc.html).

**Public Member Functions**

- `MDepthStencilStateDesc ()`
- `~MDepthStencilStateDesc ()`
- `void setDefaults ()`
- `MDepthStencilStateDesc (const MDepthStencilStateDesc &rDesc)`
- `MDepthStencilStateDesc & operator= (const MDepthStencilStateDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `bool depthEnable`
- `bool depthWriteEnable`
- `MStateManager::CompareMode depthFunc`
- `bool stencilEnable`
- `unsigned char stencilReadMask`
- `unsigned char stencilWriteMask`
- `int stencilReferenceVal`
- `MStencilOpDesc frontFace`
- `MStencilOpDesc backFace`

### MDrawContext

Module `OpenMayaRender`. Inherits `MFrameContext`. Source: [class_m_h_w_render_1_1_m_draw_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_draw_context.html).

**Public Types**

- `enum LightFilter { kFilteredToLightLimit, kFilteredIgnoreLightLimit }`
- `enum InternalTexture { kDepthPeelingTranspDepthTexture, kDepthPeelingOpaqueDepthTexture, kIrradianceEnvironment, kSpecularEnvironment, kNumberOfInternalTextures }`

**Public Member Functions**

- `MMatrix getMatrix (MFrameContext::MatrixType mtype, MStatus *ReturnStatus=NULL) const override`
- `MDoubleArray getTuple (MFrameContext::TupleType ttype, MStatus *ReturnStatus=NULL) const override`
- `MStatus getViewportDimensions (int &originX, int &originY, int &width, int &height) const override`
- `MUint64 getFrameStamp () const`
- `MBoundingBox getSceneBox (MStatus *ReturnStatus=NULL) const`
- `MBoundingBox getFrustumBox (MStatus *ReturnStatus) const`
- `MStatus getRenderTargetSize (int &width, int &height) const`
- `MStatus getDepthRange (float &nearVal, float &farVal) const`
- `bool viewDirectionAlongNegZ (MStatus *ReturnStatus) const`
- `unsigned int numberOfActiveLights (LightFilter lightFilter=kFilteredToLightLimit, MStatus *ReturnStatus=NULL) const`
- `MStatus getLightInformation (unsigned int lightNumber, MFloatPointArray &positions, MFloatVector &direction, float &intensity, MColor &color, bool &hasDirection, bool &hasPosition, LightFilter lightFilter=kFilteredToLightLimit) const`
- `MLightParameterInformation * getLightParameterInformation (unsigned int lightNumber, LightFilter lightFilter=kFilteredToLightLimit) const`
- `MHWRender::MStateManager * getStateManager () const`
- `const MHWRender::MPassContext & getPassContext () const`
- `const MRenderTarget * copyCurrentColorRenderTarget (const MString &name) const`
- `const MRenderTarget * copyCurrentDepthRenderTarget (const MString &name) const`
- `MTexture * copyCurrentColorRenderTargetToTexture () const`
- `MTexture * copyCurrentDepthRenderTargetToTexture () const`
- `const MTexture * getInternalTexture (InternalTexture internalTexture) const`
- `bool isMultiDraw () const`
- `unsigned int numberOfActiveLights (MStatus *ReturnStatus) const`

**Static Public Member Functions**

- `static const char * className ()`

### MDrawProcedureBase

Module `OpenMayaRender`. Source: [class_m_draw_procedure_base.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_draw_procedure_base.html).

**Public Member Functions**

- `MDrawProcedureBase (const MString &name)`
- `virtual ~MDrawProcedureBase ()`
- `virtual bool execute ()=0`
- `void setEnabled (bool value)`
- `bool enabled () const`
- `void setName (const MString &name)`
- `const MString & name () const`

### MDrawRegistry

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_draw_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_draw_registry.html).

**Static Public Member Functions**

- `static MStatus registerShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId, ShadingNodeOverrideCreator creator)`
- `static MStatus deregisterShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerSurfaceShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId, SurfaceShadingNodeOverrideCreator creator)`
- `static MStatus deregisterSurfaceShadingNodeOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerShaderOverrideCreator (const MString &drawClassification, const MString &registrantId, ShaderOverrideCreator creator)`
- `static MStatus deregisterShaderOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerGeometryOverrideCreator (const MString &drawClassification, const MString &registrantId, GeometryOverrideCreator creator)`
- `static MStatus deregisterGeometryOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerDrawOverrideCreator (const MString &drawClassification, const MString &registrantId, DrawOverrideCreator creator)`
- `static MStatus deregisterDrawOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerSubSceneOverrideCreator (const MString &drawClassification, const MString &registrantId, SubSceneOverrideCreator creator)`
- `static MStatus deregisterSubSceneOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerImagePlaneOverrideCreator (const MString &drawClassification, const MString &registrantId, ImagePlaneOverrideCreator creator)`
- `static MStatus deregisterImagePlaneOverrideCreator (const MString &drawClassification, const MString &registrantId)`
- `static MStatus registerVertexBufferGenerator (const MString &bufferName, VertexBufferGeneratorCreator creator)`
- `static MStatus deregisterVertexBufferGenerator (const MString &bufferName)`
- `static MStatus registerVertexBufferMutator (const MString &bufferName, VertexBufferMutatorCreator creator)`
- `static MStatus deregisterVertexBufferMutator (const MString &bufferName)`
- `static MStatus registerPrimitiveGenerator (const MString &primitiveType, PrimitiveGeneratorCreator creator)`
- `static MStatus deregisterPrimitiveGenerator (const MString &primitiveType)`
- `static MStatus registerIndexBufferMutator (const MString &primitiveType, IndexBufferMutatorCreator creator)`
- `static MStatus deregisterIndexBufferMutator (const MString &primitiveType)`
- `static MStatus registerComponentConverter (const MString &renderItemName, ComponentConverterCreator creator)`
- `static MStatus deregisterComponentConverter (const MString &renderItemName)`
- `static void setCallableInfo (MPluginCallableInfo *info)`
- `static MPluginCallableInfo * getCallableInfo ()`
- `static const char * className ()`

### MFnImageSource

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_image_source.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_image_source.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnImageSource ()`
- `MFnImageSource ()`
- `MFnImageSource (MObject &object, MStatus *ReturnStatus=NULL)`
- `MString getImageName (MCommonRenderSettingsData::MpathType pathType, double frameNumber, const MString &sceneName, const MObject camera, const MObject renderLayer, MString *bufferName=NULL, MStatus *ReturnStatus=NULL) const`
- `MFnImageSource (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnRenderLayer

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_render_layer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_render_layer.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnRenderLayer ()`
- `MFnRenderLayer ()`
- `MFnRenderLayer (MObject &object, MStatus *ReturnStatus=NULL)`
- `bool inLayer (const MObject &transform, MStatus *ReturnStatus=NULL)`
- `MStatus layerChildren (MObjectArray &children, bool recurse=false)`
- `bool inCurrentRenderLayer (const MDagPath &objectPath, MStatus *ReturnStatus=NULL)`
- `MStatus listMembers (MObjectArray &objectsInLayer) const`
- `bool isPlugAdjusted (const MPlug &scenePlug, MStatus *ReturnStatus=NULL) const`
- `MPlug adjustmentPlug (const MPlug &scenePlug, MStatus *ReturnStatus=NULL) const`
- `MStatus externalRenderPasses (MObjectArray &renderPassArray) const`
- `bool passHasObject (const MObject &renderPass, const MDagPath &objectPath, MStatus *ReturnStatus=NULL) const`
- `bool passHasLight (const MObject &renderPass, const MObject &light, MStatus *ReturnStatus=NULL) const`
- `MFnRenderLayer (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`
- `static MObject findLayerByName (const MString &renderLayer, MStatus *ReturnStatus=NULL)`
- `static MObject defaultRenderLayer (MStatus *ReturnStatus=NULL)`
- `static MStatus listAllRenderLayers (MObjectArray &rl)`
- `static MObject currentLayer (MStatus *ReturnStatus=NULL)`

### MFnRenderPass

Module `OpenMayaRender`. Inherits `MFnDependencyNode`. Source: [class_m_fn_render_pass.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_render_pass.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnRenderPass ()`
- `MFnRenderPass ()`
- `MFnRenderPass (MObject &object, MStatus *ReturnStatus=NULL)`
- `MPxRenderPassImpl * setImplementation (const MString &renderer, MStatus *ReturnStatus=NULL)`
- `MPxRenderPassImpl * getImplementation () const`
- `MPxRenderPassImpl::PassTypeBit frameBufferType (MStatus *ReturnStatus=NULL) const`
- `unsigned int frameBufferChannels (MStatus *ReturnStatus=NULL) const`
- `bool usesFiltering (MStatus *ReturnStatus=NULL) const`
- `MString passID (MStatus *ReturnStatus=NULL) const`
- `MString customTokenString (MStatus *ReturnStatus=NULL) const`
- `MFnRenderPass (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFragmentManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_fragment_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_fragment_manager.html).

**Public Types**

- `enum ShaderStage { kVertexShader, kPixelShader, kGeometryShader, kHullShader, kHullConstantShader, kDomainShader }`

**Public Member Functions**

- `bool hasFragment (const MString &fragmentName) const`
- `void fragmentList (MStringArray &fragmentNames) const`
- `bool getFragmentXML (const MString &fragmentName, MString &buffer) const`
- `bool getFragmentXML (const MObject &shadingNode, MString &buffer, bool includeUpstreamNodes=false, const MDagPath *objectContext=NULL) const`
- `MString addShadeFragmentFromBuffer (const char *buffer, bool hidden)`
- `MString addShadeFragmentFromFile (const MString &fileName, bool hidden)`
- `MString addFragmentGraphFromBuffer (const char *buffer)`
- `MString addFragmentGraphFromFile (const MString &fileName)`
- `bool removeFragment (const MString &fragmentName)`
- `bool addFragmentPath (const MString &path)`
- `bool addAutomaticShaderStageInput (ShaderStage shaderStage, const MString &parameterName, const MString &semantic, MShaderInstance::ParameterType parameterType, bool isVaryingInput)`
- `bool removeAutomaticShaderStageInput (ShaderStage shaderStage, const MString &parameterName)`
- `bool addDomainShaderInputNameMapping (const MString &realParamName, const MString &domainParamName)`
- `bool removeDomainShaderInputNameMapping (const MString &realParamName)`
- `MString findDomainShaderInputName (const MString &realParamName)`
- `void setEffectOutputDirectory (const MString &dir)`
- `MString getEffectOutputDirectory () const`
- `void setIntermediateGraphOutputDirectory (const MString &dir)`
- `MString getIntermediateGraphOutputDirectory () const`

**Static Public Member Functions**

- `static const char * className ()`

### MFrameContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_frame_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_frame_context.html).

**Classes**

- `struct BackgroundParams`
- `struct DOFParams`
- `struct EnvironmentParams`
- `struct HwFogParams`
- `struct RenderOverrideInformation`

**Public Types**

- `enum MatrixType { kWorldMtx, kWorldTransposeMtx, kWorldInverseMtx, kWorldTranspInverseMtx, kViewMtx, kViewTransposeMtx, kViewInverseMtx, kViewTranspInverseMtx, kProjectionMtx, kProjectionTranposeMtx, kProjectionInverseMtx, kProjectionTranspInverseMtx, kViewProjMtx, kViewProjTranposeMtx, kViewProjInverseMtx, kViewProjTranspInverseMtx, kWorldViewMtx, kWorldViewTransposeMtx, kWorldViewInverseMtx, kWorldViewTranspInverseMtx, kWorldViewProjMtx, kWorldViewProjTransposeMtx, kWorldViewProjInverseMtx, kWorldViewProjTranspInverseMtx }`
- `enum TupleType { kViewPosition, kViewDirection, kViewUp, kViewRight, kViewportPixelSize, kViewNearClipValue, kViewFarClipValue, kViewUnnormlizedNearClipValue, kViewUnnormalizedFarClipValue }`
- `enum DisplayStyle { kGouraudShaded = (0x1), kWireFrame = (0x1 << 1), kBoundingBox = (0x1 << 2), kTextured = (0x1 << 3), kDefaultMaterial = (0x1 << 4), kXrayJoint = (0x1 << 5), kXray = (0x1 << 6), kTwoSidedLighting = (0x1 << 7), kFlatShaded = (0x1 << 8), kShadeActiveOnly = (0x1 << 9), kXrayActiveComponents = (0x1 << 10), kBackfaceCulling = (0x1 << 11), kSmoothWireframe = (0x1 << 12) }`
- `enum LightingMode { kNoLighting, kAmbientLight, kLightDefault, kSelectedLights, kSceneLights, kCustomLights }`
- `enum PostEffectType { kAmbientOcclusion, kMotionBlur, kGammaCorrection, kViewColorTransformEnabled = kGammaCorrection, kDepthOfField, kAntiAliasing }`
- `enum FogMode { kFogLinear, kFogExp, kFogExp2 }`
- `enum TransparencyAlgorithm { kUnsorted, kObjectSorting, kWeightedAverage, kDepthPeeling }`
- `enum WireOnShadedMode { kWireframeOnShadedFull, kWireFrameOnShadedReduced, kWireFrameOnShadedNone }`
- `enum RenderingDestination { k3dViewport, k2dViewport, kImage }`

**Public Member Functions**

- `virtual MMatrix getMatrix (MFrameContext::MatrixType mtype, MStatus *ReturnStatus=NULL) const`
- `virtual MDoubleArray getTuple (MFrameContext::TupleType ttype, MStatus *ReturnStatus=NULL) const`
- `virtual MStatus getViewportDimensions (int &originX, int &originY, int &width, int &height) const`
- `virtual float getGlobalLineWidth () const`
- `MDagPath getCurrentCameraPath (MStatus *ReturnStatus=NULL) const`
- `const MRenderTarget * getCurrentColorRenderTarget () const`
- `const MRenderTarget * getCurrentDepthRenderTarget () const`
- `unsigned int getDisplayStyle () const`
- `LightingMode getLightingMode () const`
- `unsigned int getLightLimit () const`
- `EnvironmentParams getEnvironmentParameters () const`
- `bool getPostEffectEnabled (PostEffectType postEffectType) const`
- `HwFogParams getHwFogParameters () const`
- `TransparencyAlgorithm getTransparencyAlgorithm () const`
- `RenderingDestination renderingDestination (MString &destinationName) const`
- `MUint64 objectTypeExclusions () const`
- `void classificationExclusions (MStringArray &classification) const`
- `void getBackgroundParameters (MFrameContext::BackgroundParams &params) const`
- `void getDOFParameters (MFrameContext::DOFParams &params) const`
- `void getRenderOverrideInformation (RenderOverrideInformation &info) const`
- `const MSelectionInfo * getSelectionInfo () const`
- `void worldToViewport (const MPoint &inPoint, double &xPos, double &yPos) const`
- `void viewportToWorld (double xPos, double yPos, MPoint &worldNearPt, MPoint &worldFarPt) const`

**Static Public Member Functions**

- `static MFrameContext::MatrixType semanticToMatrixType (const MString &value, MStatus *ReturnStatus=NULL)`
- `static MFrameContext::TupleType semanticToTupleType (const MString &value, MStatus *ReturnStatus=NULL)`
- `static bool inUserInteraction ()`
- `static bool userChangingViewContext ()`
- `static WireOnShadedMode wireOnShadedMode ()`
- `static bool shadeTemplates ()`
- `static const char * className ()`

**Static Public Attributes**

- `static constexpr MUint64 kExcludeNone = MUint64(0)`
- `static constexpr MUint64 kExcludeNurbsCurves = MUint64(1) << (0)`
- `static constexpr MUint64 kExcludeNurbsSurfaces = MUint64(1) << (1)`
- `static constexpr MUint64 kExcludeMeshes = MUint64(1) << (2)`
- `static constexpr MUint64 kExcludePlanes = MUint64(1) << (3)`
- `static constexpr MUint64 kExcludeLights = MUint64(1) << (4)`
- `static constexpr MUint64 kExcludeCameras = MUint64(1) << (5)`
- `static constexpr MUint64 kExcludeJoints = MUint64(1) << (6)`
- `static constexpr MUint64 kExcludeIkHandles = MUint64(1) << (7)`
- `static constexpr MUint64 kExcludeDeformers = MUint64(1) << (8)`
- `static constexpr MUint64 kExcludeDynamics = MUint64(1) << (9)`
- `static constexpr MUint64 kExcludeParticleInstancers = MUint64(1) << (10)`
- `static constexpr MUint64 kExcludeLocators = MUint64(1) << (11)`
- `static constexpr MUint64 kExcludeDimensions = MUint64(1) << (12)`
- `static constexpr MUint64 kExcludeSelectHandles = MUint64(1) << (13)`
- `static constexpr MUint64 kExcludePivots = MUint64(1) << (14)`
- `static constexpr MUint64 kExcludeTextures = MUint64(1) << (15)`
- `static constexpr MUint64 kExcludeGrid = MUint64(1) << (16)`
- `static constexpr MUint64 kExcludeCVs = MUint64(1) << (17)`
- `static constexpr MUint64 kExcludeHulls = MUint64(1) << (18)`
- `static constexpr MUint64 kExcludeStrokes = MUint64(1) << (19)`
- `static constexpr MUint64 kExcludeSubdivSurfaces = MUint64(1) << (20)`
- `static constexpr MUint64 kExcludeFluids = MUint64(1) << (21)`
- `static constexpr MUint64 kExcludeFollicles = MUint64(1) << (22)`
- `static constexpr MUint64 kExcludeHairSystems = MUint64(1) << (23)`
- `static constexpr MUint64 kExcludeImagePlane = MUint64(1) << (24)`
- `static constexpr MUint64 kExcludeNCloths = MUint64(1) << (25)`
- `static constexpr MUint64 kExcludeNRigids = MUint64(1) << (26)`
- `static constexpr MUint64 kExcludeDynamicConstraints = MUint64(1) << (27)`
- `static constexpr MUint64 kExcludeManipulators = MUint64(1) << (28)`
- `static constexpr MUint64 kExcludeNParticles = MUint64(1) << (29)`
- `static constexpr MUint64 kExcludeMotionTrails = MUint64(1) << (30)`
- `static constexpr MUint64 kExcludeHoldOuts = MUint64(1) << (31)`
- `static constexpr MUint64 kExcludePluginShapes = MUint64(1) << (32)`
- `static constexpr MUint64 kExcludeHUD = MUint64(1) << (33)`
- `static constexpr MUint64 kExcludeClipGhosts = MUint64(1) << (34)`
- `static constexpr MUint64 kExcludeGreasePencils = MUint64(1) << (35)`
- `static constexpr MUint64 kExcludeControllers = MUint64(1) << (36)`
- `static constexpr MUint64 kExcludeAll = ~MUint64(0)`

### MGLFunctionTable

Module `OpenMayaRender`. Source: [class_m_g_l_function_table.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_g_l_function_table.html).

**Public Types**

- `enum MGLversion { kMGL_Version11, kMGL_Version12, kMGL_Version121, kMGL_Version13, kMGL_Version14, kMGL_Version15, kMGL_Version20 }`

**Public Member Functions**

- `bool extensionExists (MGLExtension extension)`
- `unsigned int numTexUnits () const`
- `unsigned int numTexInterpolants () const`
- `unsigned int numTexImageUnits () const`
- `unsigned int maxTextureSize () const`
- `unsigned int maxVertexAttributes () const`

**Friends**

- `class MHardwareRenderer`

### MGeometry

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry.html).

**Public Types**

- `enum DataType { kInvalidType, kFloat, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`
- `enum Semantic { kInvalidSemantic, kPosition, kNormal, kTexture, kColor, kTangent, kBitangent, kTangentWithSign }`
- `enum Primitive { kInvalidPrimitive, kPoints, kLines, kLineStrip, kTriangles, kTriangleStrip, kAdjacentTriangles, kAdjacentTriangleStrip, kAdjacentLines, kAdjacentLineStrip, kPatch }`
- `enum DrawMode { kWireframe = (0x1 << 0), kShaded = (0x1 << 1), kTextured = (0x1 << 2), kBoundingBox = (0x1 << 3), kSelectionOnly = (0x1 << 4), kSelectionHighlighting = (0x1 << 5), kAll = (kWireframe | kShaded | kTextured | kBoundingBox) }`

**Public Member Functions**

- `int vertexBufferCount () const`
- `const MVertexBuffer * vertexBuffer (int index) const`
- `MVertexBuffer * vertexBuffer (int index)`
- `MVertexBuffer * createVertexBuffer (const MVertexBufferDescriptor &desc)`
- `bool addVertexBuffer (MVertexBuffer *buffer)`
- `bool deleteVertexBuffer (int index)`
- `int indexBufferCount () const`
- `const MIndexBuffer * indexBuffer (int index) const`
- `MIndexBuffer * indexBuffer (int index)`
- `MIndexBuffer * createIndexBuffer (MGeometry::DataType type)`
- `bool addIndexBuffer (MIndexBuffer *buffer)`
- `bool deleteIndexBuffer (int index)`
- `MGeometry (const MGeometry &)`
- `~MGeometry ()`

**Static Public Member Functions**

- `static const MString & dataTypeString (MGeometry::DataType d)`
- `static const MString & semanticString (MGeometry::Semantic s)`
- `static const MString & primitiveString (MGeometry::Primitive p)`
- `static const MString drawModeString (MGeometry::DrawMode d)`
- `static const char * className ()`

### MGeometryData

Module `OpenMayaRender`. Source: [class_m_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_data.html).

**Public Types**

- `enum ElementSize { kInvalidElementSize = 0, kOne = 1, kTwo, kThree, kFour }`
- `enum ElementType { kInvalidElementType = -1, kFloat = 0, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`
- `enum DataType { kInvalidDataType = 0, kPosition, kNormal, kTexCoord, kColor, kWeight, kAPISupported, kTangent, kBiNormal, kVelocity, kPrimitiveCenter, kColorMask, kUserData, kMaxDataTypeIndex }`

**Public Member Functions**

- `MGeometryData (const char *dataName, DataType dataType, ElementType elementType, ElementSize elementSize, unsigned int elementCount, void *dataPtr=NULL, bool ownsData=true)`
- `MGeometryData (const MGeometryData &)`
- `~MGeometryData ()`
- `const char * objectName () const`
- `int uniqueID () const`
- `DataType dataType () const`
- `ElementType elementType () const`
- `unsigned int elementTypeSize () const`
- `ElementSize elementSize () const`
- `unsigned int elementCount () const`
- `void * data () const`
- `void setCollectionNumber (int cn)`
- `int collectionNumber () const`
- `void setObjectOwnsData (bool val)`
- `bool objectOwnsData () const`

**Friends**

- `class MGeometryLegacy`
- `class MHardwareRenderer`
- `class MD3D9Renderer`

### MGeometryExtractor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_extractor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_extractor.html).

**Public Member Functions**

- `MGeometryExtractor (const MHWRender::MGeometryRequirements &requirements, const MDagPath &path,::MHWRender::MPolyGeomOptions options, MStatus *status=NULL)`
- `~MGeometryExtractor ()`
- `unsigned int primitiveCount (const MHWRender::MIndexBufferDescriptor &indexDesc) const`
- `MStatus populateIndexBuffer (void *data, const unsigned int primitiveCount, const MHWRender::MIndexBufferDescriptor &indexDesc) const`
- `unsigned int vertexCount () const`
- `MStatus populateVertexBuffer (void *data, const unsigned int vertexCount, const MHWRender::MVertexBufferDescriptor &bufferDesc) const`
- `MGeometryExtractor (const MHWRender::MGeometryRequirements &requirements, const MDagPath &path, const bool sharing=true, MStatus *status=NULL)`

**Static Public Member Functions**

- `static unsigned int minimumBufferSize (unsigned int primitiveCount, MGeometry::Primitive primitive, unsigned int primitiveStride=0)`

### MGeometryIndexMapping

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_index_mapping.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_index_mapping.html).

**Public Member Functions**

- `MGeometryIndexMapping ()`
- `~MGeometryIndexMapping ()`
- `int geometryCount () const`
- `MDagPath dagPath (int geometryIdx) const`
- `MObject component (int geometryIdx) const`
- `int indexStart (int geometryIdx) const`
- `int indexLength (int geometryIdx) const`
- `int vertexStart (int geometryIdx) const`
- `int vertexLength (int geometryIdx) const`
- `MRenderItem * sourceRenderItem (int geometryIdx) const`
- `void updateSource (int geometryIdx, int indexStart, int indexLength, int vertexStart, int vertexLength)`

### MGeometryLegacy

Module `OpenMayaRender`. Source: [class_m_geometry_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_legacy.html).

**Public Member Functions**

- `~MGeometryLegacy ()`
- `MGeometryLegacy (const MGeometryLegacy &geom)`
- `MGeometryLegacy & operator= (const MGeometryLegacy &rhs)`
- `unsigned int primitiveArrayCount () const`
- `const MGeometryPrimitive primitiveArray (unsigned int arrayNumber) const`
- `const MGeometryData position ()`
- `const MGeometryData normal ()`
- `const MGeometryData componentId ()`
- `const MGeometryData texCoord (const MString &name)`
- `const MGeometryData color (const MString &name)`
- `const MGeometryData tangent (const MString &name)`
- `const MGeometryData binormal (const MString &name)`
- `const MGeometryData data (MGeometryData::DataType what, const MString &name)`

**Friends**

- `class MGeometryManager`
- `class MGeometryList`
- `class MVaryingParameter`

### MGeometryList

Module `OpenMayaRender`. Source: [class_m_geometry_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_list.html).

**Public Types**

- `enum { kNone = 0, kMatrices = 1 << 0, kFixedFunctionLighting = 1 << 1, kCulling = 1 << 2, kAll = -1 }`
- `enum MCullMode { kCullNone, kCullCW, kCullCCW }`

**Public Member Functions**

- `MGeometryList (const MRenderingInfo &info, int initialCapacity=1)`
- `~MGeometryList ()`
- `bool isDone () const`
- `void next ()`
- `void reset ()`
- `int length () const`
- `void setCurrentElement (int i)`
- `MGeometryLegacy & geometry (int setupFlags=kAll)`
- `const MMatrix & objectToWorldMatrix () const`
- `const MMatrix & projectionMatrix () const`
- `const MMatrix & viewMatrix () const`
- `MDagPath path () const`
- `MGeometryList::MCullMode cullMode () const`
- `bool addLast (const MDagPath &path, const MObject *components=NULL)`
- `bool addLast (const MGeometryLegacy &geometry, const MMatrix &matrix)`

**Public Attributes**

- `enum MGeometryList:: { ... } MSetupFlags`

### MGeometryManager

Module `OpenMayaRender`. Source: [class_m_geometry_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_manager.html).

**Public Types**

- `enum GeometricShape { kDefaultSphere = 0, kDefaultPlane, kDefaultCube }`

**Static Public Member Functions**

- `static MGeometryLegacy getGeometry (const MDagPath &shape, const MGeometryRequirementsLegacy &requirements, MObject *components=NULL)`
- `static MGeometryList * referenceDefaultGeometry (const MGeometryManager::GeometricShape geomShape, const MGeometryRequirementsLegacy &requirements)`
- `static MStatus dereferenceDefaultGeometry (MGeometryList *data)`
- `static const char * className ()`

### MGeometryPrimitive

Module `OpenMayaRender`. Source: [class_m_geometry_primitive.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_primitive.html).

**Public Types**

- `enum DrawPrimitiveType { kInvalidIndexType = 0, kPoints, kLines, kLineStrip, kLineLoop, kTriangles, kTriangleStrip, kTriangleFan, kQuads, kQuadStrip, kPolygon, kMaxDrawPrimitiveTypeIndex }`

**Public Member Functions**

- `MGeometryPrimitive (const MGeometryPrimitive &other)`
- `~MGeometryPrimitive ()`
- `int uniqueID () const`
- `DrawPrimitiveType drawPrimitiveType () const`
- `unsigned int elementCount () const`
- `MGeometryData::ElementType dataType () const`
- `const void * data () const`

**Friends**

- `class MGeometryLegacy`
- `class MVaryingParameter`
- `class MHardwareRenderer`

### MGeometryRequirements

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_requirements.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_requirements.html).

**Public Member Functions**

- `MGeometryRequirements ()`
- `~MGeometryRequirements ()`
- `void addVertexRequirement (const MVertexBufferDescriptor &descriptor)`
- `void addIndexingRequirement (const MIndexBufferDescriptor &descriptor)`
- `const MVertexBufferDescriptorList & vertexRequirements () const`
- `const MIndexBufferDescriptorList & indexingRequirements () const`

**Static Public Member Functions**

- `static const char * className ()`

### MGeometryRequirementsLegacy

Module `OpenMayaRender`. Source: [class_m_geometry_requirements_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_geometry_requirements_legacy.html).

**Public Member Functions**

- `MGeometryRequirementsLegacy ()`
- `~MGeometryRequirementsLegacy ()`
- `void addPosition (int dimension=3)`
- `void addComponentId ()`
- `void addNormal (int dimension=3)`
- `void addTexCoord (const MString &uvSetName)`
- `void addTangent (const MString &uvSetName, int dimension=3)`
- `void addBinormal (const MString &uvSetName, int dimension=3)`
- `void addColor (const MString &colorSetName)`
- `void addFaceOffsets ()`

### MGeometryUtilities

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_geometry_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_geometry_utilities.html).

**Public Types**

- `enum GeometricShape { kDefaultSphere, kDefaultPlane, kDefaultCube }`

**Static Public Member Functions**

- `static MColor wireframeColor (const MDagPath &path)`
- `::MHWRender::DisplayStatus displayStatus (const MDagPath &path, MStatus *returnStatus=NULL)`
- `static MGeometry * acquireReferenceGeometry (GeometricShape shape, const MGeometryRequirements &requirements)`
- `static void releaseReferenceGeometry (MGeometry *geometry)`
- `static const char * className ()`

### MHUDRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_h_u_d_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_h_u_d_render.html).

**Public Member Functions**

- `MHUDRender ()`
- `~MHUDRender () override`
- `const MString & name () const override`
- `virtual bool hasUIDrawables () const`
- `virtual void addUIDrawables (MUIDrawManager &drawManager2D, const MFrameContext &frameContext)`

### MHardwareRenderer

Module `OpenMayaRender`. Source: [class_m_hardware_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hardware_renderer.html).

**Public Types**

- `enum DrawProcedureStatusCode { kSuccess = 0, kFailure, kItemExists, kItemNotFound, kLocationNotFound }`
- `enum BufferPixelFormat { kRGBA_Fix8, kRGBA_Float16, kDepth_Float32 }`
- `enum CallLocation { kPreRendering = 0, kPreExposure = 1, kPostExposure = 2, kPostRendering = 3 }`
- `enum GeometricShape { kDefaultSphere = 0, kDefaultPlane, kDefaultCube }`

**Public Member Functions**

- `const MString & backEndString () const`
- `DrawProcedureStatusCode addDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedureptr, CallLocation location)`
- `DrawProcedureStatusCode removeDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedureptr, CallLocation location)`
- `DrawProcedureStatusCode insertDrawProcedure (const MString &backEndString, MDrawProcedureBase *drawProcedurePtr, CallLocation location, unsigned int listIndex, bool moveExistingItem=false)`
- `DrawProcedureStatusCode getDrawProcedureCount (const MString &backEndString, CallLocation location, unsigned int &count) const`
- `DrawProcedureStatusCode getDrawProcedureListNames (const MString &backEndString, CallLocation location, MStringArray &namesOfProcs) const`
- `MDrawProcedureBase * findDrawProcedure (const MString &backEndString, CallLocation location, const MString &procName) const`
- `MStatus makeResourceContextCurrent (const MString &backEndString)`
- `MStatus getBufferSize (const MString &backEndString, unsigned int &width, unsigned int &height) const`
- `MStatus getColorBufferPixelFormat (const MString &backEndString, BufferPixelFormat &fmt) const`
- `MStatus getDepthBufferPixelFormat (const MString &backEndString, BufferPixelFormat &fmt) const`
- `MStatus getCurrentExposureNumber (const MString &backEndString, unsigned int &number) const`
- `MStatus getTotalExposureCount (const MString &backEndString, unsigned int &number) const`
- `MStatus restoreCurrent (const MString &backEndString) const`
- `MStatus makeSwatchContextCurrent (const MString &backEndString, unsigned int &width, unsigned int &height)`
- `MStatus readSwatchContextPixels (const MString &backEndString, MImage &image)`
- `MGeometryData * referenceDefaultGeometry (MHardwareRenderer::GeometricShape geomShape, unsigned int &numberOfData, unsigned int *&pIndexing, unsigned int &indexLength)`
- `MStatus dereferenceGeometry (MGeometryData *pGeomData, unsigned int numberOfData)`
- `void getSwatchOrthoCameraSetting (double &l, double &r, double &b, double &t, double &n, double &f) const`
- `void getSwatchPerspectiveCameraSetting (double &l, double &r, double &b, double &t, double &n, double &f) const`
- `void getSwatchPerspectiveCameraTranslation (float &x, float &y, float &z, float &w) const`
- `void getSwatchLightDirection (float &x, float &y, float &z, float &w) const`
- `void drawSwatchBackGroundQuads (const MColor &quadColor=kDefaultQuadColor, bool textured=false, unsigned int numberOfRepeats=8) const`
- `MGLFunctionTable * glFunctionTable () const`

**Static Public Member Functions**

- `static MHardwareRenderer * theRenderer ()`

### MHwTextureManager

Module `OpenMayaRender`. Source: [class_m_hw_texture_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hw_texture_manager.html).

**Static Public Member Functions**

- `static MStatus glBind (const MObject &fileTextureObject, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`
- `static MStatus glBind (const MPlug &fileTextureConnection, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`
- `static MStatus registerTextureFile (const MString &fileName, MHwTextureFileHandle &hTexture)`
- `static MStatus deregisterTextureFile (const MHwTextureFileHandle &hTexture)`
- `static MStatus textureFile (const MHwTextureFileHandle &hTexture, MString &fileName)`
- `static MStatus glBind (const MHwTextureFileHandle &hTexture, MImageFileInfo::MHwTextureType &targetType, MImageFileInfo::MImageType imageType=MImageFileInfo::kImageTypeUnknown)`
- `static const char * className ()`

### MHwrCallback

Module `OpenMayaRender`. Source: [class_m_hwr_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hwr_callback.html).

**Public Member Functions**

- `MHwrCallback ()`
- `virtual ~MHwrCallback ()`
- `virtual void deviceNew ()`
- `virtual void deviceLost ()`
- `virtual void deviceReset ()`
- `virtual void deviceDeleted ()`

**Static Public Member Functions**

- `static void addCallback (MHwrCallback *, int priority=0)`
- `static void removeCallback (MHwrCallback *)`

### MIndexBuffer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer.html).

**Public Member Functions**

- `MIndexBuffer (MGeometry::DataType type, bool softwareStaged=true)`
- `MIndexBuffer (const MIndexBuffer &)`
- `~MIndexBuffer ()`
- `MGeometry::DataType dataType () const`
- `unsigned int size () const`
- `void * resourceHandle () const`
- `void resourceHandle (void *handle, unsigned int size)`
- `bool hasCustomResourceHandle () const`
- `void lockResourceHandle ()`
- `void unlockResourceHandle ()`
- `void * acquire (unsigned int size, bool writeOnly)`
- `void commit (void *buffer)`
- `void * map ()`
- `void unmap ()`
- `MStatus update (const void *buffer, unsigned int destOffset, unsigned int numIndices, bool truncateIfSmaller)`
- `void unload ()`
- `void * acquire (unsigned int size)`
- `void * getIndexBuffer () const`
- `void setIndexBuffer (void *ib)`

**Static Public Member Functions**

- `static const char * className ()`

### MIndexBufferDescriptor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer_descriptor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer_descriptor.html).

**Public Member Functions**

- `MIndexBufferDescriptor ()`
- `MIndexBufferDescriptor (MIndexBufferDescriptor::IndexType type, const MString &name, MGeometry::Primitive primitive, unsigned int primitiveStride=0, MObject component=MObject::kNullObj, MGeometry::DataType dataType=MGeometry::kUnsignedInt32)`
- `~MIndexBufferDescriptor ()`
- `MString name () const`
- `void setName (MString &val)`
- `MIndexBufferDescriptor::IndexType indexType () const`
- `void setIndexType (MIndexBufferDescriptor::IndexType val)`
- `MGeometry::Primitive primitive () const`
- `void setPrimitive (MGeometry::Primitive val)`
- `unsigned int primitiveStride () const`
- `void setPrimitiveStride (unsigned int id)`
- `MObject component () const`
- `void setComponent (MObject component)`
- `MGeometry::DataType dataType () const`
- `void dataType (MGeometry::DataType dataType)`

### MIndexBufferDescriptorList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_index_buffer_descriptor_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_index_buffer_descriptor_list.html).

**Public Member Functions**

- `MIndexBufferDescriptorList ()`
- `~MIndexBufferDescriptorList ()`
- `int length () const`
- `bool getDescriptor (int index, MIndexBufferDescriptor &desc) const`
- `bool append (const MIndexBufferDescriptor &desc)`
- `bool removeAt (int index)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MIntersection

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_intersection.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_intersection.html).

**Public Member Functions**

- `MSelectionContext::SelectionLevel selectionLevel (MStatus *ReturnStatus=nullptr) const`
- `int index (MStatus *ReturnStatus=nullptr) const`
- `MStatus barycentricCoordinates (float &a, float &b) const`
- `float edgeInterpolantValue (MStatus *ReturnStatus=nullptr) const`
- `MFloatPoint intersectionPoint (MStatus *ReturnStatus=nullptr) const`
- `int instanceID (MStatus *ReturnStatus=nullptr) const`

**Static Public Member Functions**

- `static const char * className ()`

### MLightLinks

Module `OpenMayaRender`. Source: [class_m_light_links.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_light_links.html).

**Public Member Functions**

- `MLightLinks ()`
- `~MLightLinks ()`
- `MStatus parseLinks (const MObject &linkNode=MObject::kNullObj, bool verbose=false, std::ostream *stream=NULL, bool useIgnore=false, bool componentSupport=true)`
- `MStatus getLinkedLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`
- `MStatus getLinkedObjects (const MDagPath &light, MSelectionList &objects)`
- `MStatus getIgnoredLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`
- `MStatus getIgnoredObjects (const MDagPath &light, MSelectionList &objects)`
- `MStatus getShadowLinkedLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`
- `MStatus getShadowLinkedObjects (const MDagPath &light, MSelectionList &objects)`
- `MStatus getShadowIgnoredLights (const MDagPath &path, const MObject &component, MDagPathArray &lights)`
- `MStatus getShadowIgnoredObjects (const MDagPath &light, MSelectionList &objects)`

### MLightParameterInformation

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_light_parameter_information.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_light_parameter_information.html).

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture2, kSampler, kTextureCube }`
- `enum StockParameterSemantic { kNoSemantic, kLightEnabled, kWorldPosition, kWorldDirection, kIntensity, kColor, kEmitsDiffuse, kEmitsSpecular, kDecayRate, kDropoff, kCosConeAngle, kStartShadowParameters , kShadowMap, kShadowSamp, kShadowBias, kShadowMapSize, kShadowViewProj, kShadowColor, kGlobalShadowOn, kShadowOn, kShadowDirty, kDepthRange }`

**Public Member Functions**

- `void parameterList (MStringArray &list) const`
- `ParameterType parameterType (const MString &parameterName) const`
- `StockParameterSemantic parameterSemantic (const MString &parameterName) const`
- `void parameterNames (StockParameterSemantic semantic, MStringArray &list) const`
- `unsigned int arrayParameterCount (const MString &parameterName) const`
- `MStatus getParameter (const MString &parameterName, MIntArray &value)`
- `MStatus getParameter (const MString &parameterName, MFloatArray &value)`
- `MStatus getParameter (const MString &parameterName, MMatrix &value)`
- `MStatus getParameter (const MString &parameterName, MSamplerStateDesc &samplerDesc)`
- `void * getParameterTextureHandle (const MString &parameterName)`
- `MStatus getParameter (const MString &parameterName, MTextureAssignment &value)`
- `MStatus getParameter (StockParameterSemantic semantic, MIntArray &value)`
- `MStatus getParameter (StockParameterSemantic semantic, MFloatArray &value)`
- `MStatus getParameter (StockParameterSemantic semantic, MMatrix &value)`
- `MStatus getParameter (StockParameterSemantic semantic, MSamplerStateDesc &samplerDesc)`
- `void * getParameterTextureHandle (StockParameterSemantic semantic)`
- `MStatus getParameter (StockParameterSemantic semantic, MTextureAssignment &value)`
- `MString lightType () const`
- `MDagPath lightPath (MStatus *ReturnStatus=NULL) const`

**Protected Member Functions**

- `~MLightParameterInformation ()`

### MPassContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_pass_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_pass_context.html).

**Public Member Functions**

- `const MString & passIdentifier () const`
- `const MStringArray & passSemantics () const`
- `bool hasShaderOverride () const`
- `MShaderInstance * shaderOverrideInstance () const`

**Static Public Attributes**

- `static const MString kColorPassSemantic`
- `static const MString kShadowPassSemantic`
- `static const MString kPointLightShadowPassSemantic`
- `static const MString kDepthPassSemantic`
- `static const MString kNormalDepthPassSemantic`
- `static const MString kOpaqueGeometrySemantic`
- `static const MString kPreUIGeometrySemantic`
- `static const MString kPostUIGeometrySemantic`
- `static const MString kUIGeometrySemantic`
- `static const MString kOpaqueUISemantic`
- `static const MString kTransparentUISemantic`
- `static const MString kXrayUISemantic`
- `static const MString kTransparentGeometrySemantic`
- `static const MString kCullBackSemantic`
- `static const MString kCullFrontSemantic`
- `static const MString kMaterialOverrideSemantic`
- `static const MString kTransparentPeelSemantic`
- `static const MString kTransparentPeelAndAvgSemantic`
- `static const MString kTransparentWeightedAvgSemantic`
- `static const MString kUserPassSemantic`
- `static const MString kBeginRenderSemantic`
- `static const MString kBeginSceneRenderSemantic`
- `static const MString kEndSceneRenderSemantic`
- `static const MString kEndRenderSemantic`
- `static const MString kSelectionPassSemantic`
- `static const MString kDOFPassSemantic`
- `static const MString kMotionVectorPassSemantic`
- `static const MString kPEPatternPassSemantic`
- `static const MString kNonPEPatternPassSemantic`

### MPresentTarget

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_present_target.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_present_target.html).

**Public Types**

- `enum MTargetBackBuffer { kCenterBuffer, kLeftBuffer, kRightBuffer }`

**Public Member Functions**

- `MPresentTarget (const MString &name)`
- `~MPresentTarget () override`
- `bool presentDepth () const`
- `void setPresentDepth (bool val)`
- `bool enableSRGBWrite () override`
- `MTargetBackBuffer targetBackBuffer () const`
- `void setTargetBackBuffer (MTargetBackBuffer backBuffer)`

**Protected Attributes**

- `MTargetBackBuffer mTargetBackBuffer`
- `bool mPresentDepth`

### MPxComponentConverter

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_component_converter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_component_converter.html).

**Public Member Functions**

- `MPxComponentConverter ()`
- `virtual ~MPxComponentConverter ()`
- `virtual void initialize (const MRenderItem &renderItem)=0`
- `virtual void addIntersection (MIntersection &intersection)=0`
- `virtual MObject component ()=0`
- `virtual MSelectionMask selectionMask () const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxDrawOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_draw_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_draw_override.html).

**Public Types**

- `typedef void(* GeometryDrawOverrideCb) (const MDrawContext &, const MUserData *)`

**Public Member Functions**

- `MPxDrawOverride (const MObject &obj, GeometryDrawOverrideCb callback, bool isAlwaysDirty=true)`
- `virtual ~MPxDrawOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool hasUIDrawables () const`
- `virtual MMatrix transform (const MDagPath &objPath, const MDagPath &cameraPath) const`
- `virtual MBoundingBox boundingBox (const MDagPath &objPath, const MDagPath &cameraPath) const`
- `virtual bool isBounded (const MDagPath &objPath, const MDagPath &cameraPath) const`
- `virtual bool disableInternalBoundingBoxDraw () const`
- `virtual bool excludedFromPostEffects () const`
- `virtual bool isTransparent () const`
- `virtual MUserData * prepareForDraw (const MDagPath &objPath, const MDagPath &cameraPath, const MFrameContext &frameContext, MUserData *oldData)=0`
- `virtual void addUIDrawables (const MDagPath &objPath, MUIDrawManager &drawManager, const MFrameContext &frameContext, const MUserData *data)`
- `virtual bool refineSelectionPath (const MSelectionInfo &selectInfo, const MRenderItem &hitItem, MDagPath &path, MObject &geomComponents, MSelectionMask &objectMask)`
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`
- `virtual bool traceCallSequence () const`
- `virtual void handleTraceMessage (const MString &message) const`
- `virtual bool wantUserSelection () const`
- `virtual bool userSelect (const MSelectionInfo &selectInfo, const MDrawContext &context, const MDagPath &objPath, const MUserData *data, MSelectionList &selectionList, MPointArray &worldSpaceHitPts)`
- `virtual bool userSelect (MSelectionInfo &selectInfo, const MDrawContext &context, MPoint &hitPoint, const MUserData *data)`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus setGeometryForRenderItem (MRenderItem &renderItem, const MVertexBufferArray &vertexBuffers, const MIndexBuffer *indexBuffer, const MBoundingBox *objectBox) const`
- `virtual void invokeTheCallback (MDrawContext &drawContext, MUserData *userData)`

**Static Protected Member Functions**

- `static MStatus drawRenderItem (const MHWRender::MDrawContext &context, MRenderItem &item)`
- `static bool pointSnappingActive ()`

### MPxGeometryOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_geometry_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_geometry_override.html).

**Public Member Functions**

- `MPxGeometryOverride (const MObject &obj)`
- `virtual ~MPxGeometryOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool hasUIDrawables () const`
- `virtual bool requiresGeometryUpdate () const`
- `virtual bool requiresUpdateRenderItems (const MDagPath &path) const`
- `virtual bool supportsEvaluationManagerParallelUpdate () const`
- `virtual bool supportsVP2CustomCaching () const`
- `virtual void configCache (const MEvaluationNode &, MCacheSchema &) const`
- `virtual void updateDG ()=0`
- `virtual bool isIndexingDirty (const MRenderItem &item)`
- `virtual bool isStreamDirty (const MVertexBufferDescriptor &desc)`
- `virtual void updateRenderItems (const MDagPath &path, MRenderItemList &list)=0`
- `virtual void addUIDrawables (const MDagPath &path, MUIDrawManager &drawManager, const MFrameContext &frameContext)`
- `virtual void populateGeometry (const MGeometryRequirements &requirements, const MRenderItemList &renderItems, MGeometry &data)=0`
- `virtual void cleanUp ()=0`
- `virtual bool refineSelectionPath (const MSelectionInfo &selectInfo, const MRenderItem &hitItem, MDagPath &multipath, MObject &geomComponents, MSelectionMask &objectMask)`
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`
- `virtual bool traceCallSequence () const`
- `virtual void handleTraceMessage (const MString &message) const`
- `const MFrameContext * getFrameContext () const`

**Static Public Member Functions**

- `static bool pointSnappingActive ()`
- `static const char * className ()`

### MPxImagePlaneOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_image_plane_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_image_plane_override.html).

**Public Member Functions**

- `MPxImagePlaneOverride (const MObject &obj)`
- `virtual ~MPxImagePlaneOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `MString getFileName (bool useImageSequence)`
- `virtual void updateDG ()`
- `virtual void updateColorTexture ()`

**Static Public Member Functions**

- `static const char * className ()`

### MPxIndexBufferMutator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_index_buffer_mutator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_index_buffer_mutator.html).

**Public Member Functions**

- `MPxIndexBufferMutator ()`
- `virtual ~MPxIndexBufferMutator ()`
- `virtual MGeometry::Primitive mutateIndexing (const MHWRender::MComponentDataIndexingList &sourceIndexBuffers, const MVertexBufferArray &vertexBuffers, MIndexBuffer &indexBuffer, int &primitiveStride) const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxPrimitiveGenerator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_primitive_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_primitive_generator.html).

**Public Member Functions**

- `MPxPrimitiveGenerator ()`
- `virtual ~MPxPrimitiveGenerator ()`
- `virtual unsigned int computeIndexCount (const MObject &object, const MObject &component) const =0`
- `virtual MGeometry::Primitive generateIndexing (const MObject &object, const MObject &component, const MComponentDataIndexingList &sourceIndexing, const MComponentDataIndexingList &targetIndexing, MIndexBuffer &indexBuffer, int &primitiveStride) const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxRenderPassImpl

Module `OpenMayaRender`. Source: [class_m_px_render_pass_impl.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_render_pass_impl.html).

**Public Types**

- `enum PassTypeBit { kUInt8 = 0x1, kUInt16 = 0x2, kUInt32 = 0x4, kUInt64 = 0x8, kInt8 = 0x10, kInt16 = 0x20, kInt32 = 0x40, kInt64 = 0x80, kFloat16 = 0x100, kFloat32 = 0x200, kFloat64 = 0x400, kBit = 0x800, kOther = 0x1000 }`
- `enum PassSemantic { kInvalidSemantic = 0, kColorSemantic, kVectorSemantic, kDirectionVectorSemantic, kDepthSemantic, kLabelSemantic, kMaskSemantic, kOtherSemantic }`
- `typedef unsigned int PassTypeMask`

**Public Member Functions**

- `MPxRenderPassImpl ()`
- `virtual ~MPxRenderPassImpl ()`
- `virtual bool isCompatible (const MString &renderer) const =0`
- `virtual PassTypeMask typesSupported () const =0`
- `virtual PassTypeBit getDefaultType () const =0`
- `virtual unsigned int getNumChannels () const =0`
- `virtual PassSemantic frameBufferSemantic () const =0`
- `virtual bool perLightPassContributionSupported () const =0`

### MPxRenderer

Module `OpenMayaRender`. Source: [class_m_px_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer.html).

**Classes**

- `class JobParams`
- `class ProgressParams`
- `class RefreshParams`

**Public Types**

- `enum EnvironmentType { IBL_ENVIRONMENT }`
- `enum JobType { SWATCH_RENDER_JOB }`

**Public Member Functions**

- `virtual ~MPxRenderer ()`
- `virtual MStatus startAsync (const JobParams &params)`
- `virtual MStatus stopAsync ()`
- `virtual bool isRunningAsync ()`
- `virtual MStatus beginSceneUpdate ()`
- `virtual MStatus translateMesh (const MUuid &id, const MObject &node)`
- `virtual MStatus translateLightSource (const MUuid &id, const MObject &node)`
- `virtual MStatus translateCamera (const MUuid &id, const MObject &node)`
- `virtual MStatus translateEnvironment (const MUuid &id, EnvironmentType type)`
- `virtual MStatus translateTransform (const MUuid &id, const MUuid &childId, const MMatrix &matrix)`
- `virtual MStatus translateShader (const MUuid &id, const MObject &node)`
- `virtual MStatus setProperty (const MUuid &id, const MString &name, bool value)`
- `virtual MStatus setProperty (const MUuid &id, const MString &name, int value)`
- `virtual MStatus setProperty (const MUuid &id, const MString &name, float value)`
- `virtual MStatus setProperty (const MUuid &id, const MString &name, const MString &value)`
- `virtual MStatus setShader (const MUuid &id, const MUuid &shaderId)`
- `virtual MStatus setResolution (unsigned int width, unsigned int height)`
- `virtual MStatus endSceneUpdate ()`
- `virtual MStatus destroyScene ()`
- `virtual bool isSafeToUnload ()=0`

**Protected Member Functions**

- `MPxRenderer ()`
- `void refresh (const RefreshParams &params)`
- `void progress (const ProgressParams &params)`

### MPxRenderer::JobParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_job_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_job_params.html).

**Public Attributes**

- `JobType type`
- `MString description`
- `unsigned int maxThreads`
- `unsigned int priority`
- `MUuid cameraId`

### MPxRenderer::ProgressParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_progress_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_progress_params.html).

**Public Attributes**

- `MString description`
- `float progress`

### MPxRenderer::RefreshParams

Module `OpenMayaRender`. Source: [class_m_px_renderer_1_1_refresh_params.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_renderer_1_1_refresh_params.html).

**Public Attributes**

- `unsigned int width`
- `unsigned int height`
- `unsigned int left`
- `unsigned int right`
- `unsigned int bottom`
- `unsigned int top`
- `unsigned int channels`
- `unsigned int bytesPerChannel`
- `void * data`

### MPxShaderOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_shader_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_shader_override.html).

**Classes**

- `struct MInitContext`
- `struct MInitFeedback`

**Public Member Functions**

- `MPxShaderOverride (const MObject &obj)`
- `virtual ~MPxShaderOverride ()`
- `virtual MString initialize (MObject shader)`
- `virtual MString initialize (const MInitContext &initContext, MInitFeedback &initFeedback)`
- `virtual MString initialize (const MInitContext &initContext, MSharedPtr< MUserData > &userData)`
- `virtual void updateDG (MObject object)`
- `virtual void updateDevice ()`
- `virtual void endUpdate ()`
- `virtual bool handlesDraw (MDrawContext &context)`
- `virtual void activateKey (MDrawContext &context, const MString &key)`
- `virtual bool draw (MDrawContext &context, const MRenderItemList &renderItemList) const =0`
- `virtual void terminateKey (MDrawContext &context, const MString &key)`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool isTransparent ()`
- `virtual bool supportsAdvancedTransparency () const`
- `virtual bool overridesDrawState ()`
- `virtual bool rebuildAlways ()`
- `virtual double boundingBoxExtraScale () const`
- `virtual bool overridesNonMaterialItems () const`
- `virtual bool handlesConsolidatedGeometry () const`
- `virtual bool requiresUnsharedGeometricStreams () const`
- `virtual bool supportsMultiDraw () const`
- `virtual MHWRender::MShaderInstance * shaderInstance (MDrawContext &context) const`
- `virtual MHWRender::MShaderInstance * nonTexturedShaderInstance (bool &monitorNode) const`
- `virtual MHWRender::MShaderInstance * multiDrawNonTexturedShaderInstance (bool &monitorNode) const`
- `virtual void activateKey (MDrawContext &context)`
- `virtual void terminateKey (MDrawContext &context)`
- `virtual MHWRender::MShaderInstance * shaderInstance () const`

**Static Public Member Functions**

- `static const char * className ()`

**Protected Member Functions**

- `MStatus addGeometryRequirement (const MVertexBufferDescriptor &desc)`
- `MStatus addGeometryRequirements (const MVertexBufferDescriptorList &list)`
- `MStatus setGeometryRequirements (const MShaderInstance &shaderInstance)`
- `MStatus addShaderSignature (void *signature, size_t signatureSize)`
- `MStatus addShaderSignature (const MShaderInstance &shaderInsance)`
- `MStatus addIndexingRequirement (const MIndexBufferDescriptor &desc)`
- `void drawGeometry (const MDrawContext &context) const`

### MPxShadingNodeOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_shading_node_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_shading_node_override.html).

**Public Member Functions**

- `MPxShadingNodeOverride (const MObject &obj)`
- `virtual ~MPxShadingNodeOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool allowConnections () const`
- `virtual MString fragmentName () const =0`
- `virtual void getCustomMappings (MAttributeParameterMappingList &mappings)`
- `virtual MString outputForConnection (const MPlug &sourcePlug, const MPlug &destinationPlug)`
- `virtual bool valueChangeRequiresFragmentRebuild (const MPlug *plug) const`
- `virtual void updateDG ()`
- `virtual void updateShader (MShaderInstance &shader, const MAttributeParameterMappingList &mappings)`

**Static Public Member Functions**

- `static const char * className ()`

### MPxSubSceneOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_sub_scene_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_sub_scene_override.html).

**Public Member Functions**

- `MPxSubSceneOverride (const MObject &obj)`
- `virtual ~MPxSubSceneOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool requiresUpdate (const MSubSceneContainer &container, const MFrameContext &frameContext) const =0`
- `virtual void update (MSubSceneContainer &container, const MFrameContext &frameContext)=0`
- `virtual bool furtherUpdateRequired (const MFrameContext &frameContext)`
- `virtual bool areUIDrawablesDirty () const`
- `virtual bool hasUIDrawables () const`
- `virtual void addUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`
- `virtual bool enableUpdateForSelection () const`
- `virtual bool getSelectionPath (const MRenderItem &renderItem, MDagPath &dagPath) const`
- `virtual bool getInstancedSelectionPath (const MRenderItem &renderItem, const MIntersection &intersection, MDagPath &dagPath) const`
- `virtual void updateSelectionGranularity (const MDagPath &path, MSelectionContext &selectionContext)`
- `MStatus setGeometryForRenderItem (MRenderItem &renderItem, const MVertexBufferArray &vertexBuffers, const MIndexBuffer &indexBuffer, const MBoundingBox *objectBox)`
- `unsigned int addInstanceTransform (MRenderItem &renderItem, const MMatrix &transform)`
- `MStatus setInstanceTransformArray (MRenderItem &renderItem, const MMatrixArray &matrixArray)`
- `MStatus updateInstanceTransform (MRenderItem &renderItem, unsigned int instanceId, const MMatrix &transform)`
- `MStatus removeInstance (MRenderItem &renderItem, unsigned int instanceId)`
- `MStatus removeAllInstances (MRenderItem &renderItem)`
- `MStatus setExtraInstanceData (MRenderItem &renderItem, const MString &parameterName, const MFloatArray &data)`
- `MStatus setExtraInstanceData (MRenderItem &renderItem, unsigned int instanceId, const MString &parameterName, const MFloatArray &data)`
- `MStatus removeExtraInstanceData (MRenderItem &renderItem, const MString &parameterName)`
- `MStatus setAllowTransparentInstances (MRenderItem &renderItem, bool allow)`

**Static Public Member Functions**

- `static bool pointSnappingActive ()`
- `static const char * className ()`

### MPxSurfaceShadingNodeOverride

Module `OpenMayaRender`. Inherits `MPxShadingNodeOverride`. Source: [class_m_h_w_render_1_1_m_px_surface_shading_node_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_surface_shading_node_override.html).

**Public Member Functions**

- `MPxSurfaceShadingNodeOverride (const MObject &obj)`
- `~MPxSurfaceShadingNodeOverride () override`
- `virtual MString primaryColorParameter () const`
- `virtual MString transparencyParameter () const`
- `virtual MString bumpAttribute () const`

**Static Public Member Functions**

- `static const char * className ()`

### MPxVertexBufferGenerator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_vertex_buffer_generator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_vertex_buffer_generator.html).

**Public Member Functions**

- `MPxVertexBufferGenerator ()`
- `virtual ~MPxVertexBufferGenerator ()`
- `virtual bool getSourceIndexing (const MObject &object, MComponentDataIndexing &sourceIndexing) const =0`
- `virtual bool getSourceStreams (const MObject &object, MStringArray &sourceStreams) const =0`
- `virtual void createVertexStream (const MObject &object, MVertexBuffer &vertexBuffer, const MComponentDataIndexing &targetIndexing, const MComponentDataIndexing &sharedIndexing, const MVertexBufferArray &sourceStreams) const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxVertexBufferMutator

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_vertex_buffer_mutator.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_vertex_buffer_mutator.html).

**Public Member Functions**

- `MPxVertexBufferMutator ()`
- `virtual ~MPxVertexBufferMutator ()`
- `virtual void modifyVertexStream (const MObject &object, MVertexBuffer &vertexBuffer, const MComponentDataIndexing &targetIndexing) const =0`

**Static Public Member Functions**

- `static const char * className ()`

### MPxViewportComputeItem

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_px_viewport_compute_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_px_viewport_compute_item.html).

**Classes**

- `struct Actions`

**Public Types**

- `enum Action { kAccessVirtualDevice = 0x1 << 0, kAccessConsolidation = 0x1 << 1, kModifyVertexBufferSize = 0x1 << 2, kModifyMatrix = 0x1 << 3, kModifyConsolidation = 0x1 << 4, kModifyBounds = 0x1 << 5, kAddVertexStream = 0x1 << 6, kModifyVertexBufferData = 0x1 << 7, kModifyPrimitive = 0x1 << 8, kActionFinal = 0x1 <<9 }`

**Public Member Functions**

- `MPxViewportComputeItem (bool hasExternalOwner)`
- `virtual ~MPxViewportComputeItem ()`
- `bool hasExternalOwner () const`
- `void setHasExternalOwner (bool hasExternalOwner)`
- `MStatus setRequiredAction (Action action, bool required)`
- `bool hasRequiredAction (Action action) const`
- `Actions requiredActions () const`
- `virtual bool execute (const Actions &availableActions, MRenderItem &renderItem)=0`

**Static Public Member Functions**

- `static const char * className ()`

### MQuadRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_quad_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_quad_render.html).

**Public Member Functions**

- `MQuadRender (const MString &name)`
- `~MQuadRender () override`
- `virtual const MShaderInstance * shader ()`
- `virtual MClearOperation & clearOperation ()`
- `virtual const MDepthStencilState * depthStencilStateOverride ()`
- `virtual const MRasterizerState * rasterizerStateOverride ()`
- `virtual const MBlendState * blendStateOverride ()`

**Protected Attributes**

- `MClearOperation mClearOperation`

### MRasterizerState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_rasterizer_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_rasterizer_state.html).

**Public Types**

- `enum FillMode { kFillSolid = 3, kFillWireFrame = 2 }`
- `enum CullMode { kCullNone = 1, kCullFront = 2, kCullBack = 3 }`

**Public Member Functions**

- `const MRasterizerStateDesc & desc (MStatus *ReturnStatus=NULL) const`
- `void * resourceHandle () const`
- `~MRasterizerState ()`

**Static Public Member Functions**

- `static const char * className ()`

### MRasterizerStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_rasterizer_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_rasterizer_state_desc.html).

**Public Member Functions**

- `MRasterizerStateDesc ()`
- `~MRasterizerStateDesc ()`
- `void setDefaults ()`
- `MRasterizerStateDesc (const MRasterizerStateDesc &rDesc)`
- `MRasterizerStateDesc & operator= (const MRasterizerStateDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `MRasterizerState::FillMode fillMode`
- `MRasterizerState::CullMode cullMode`
- `bool frontCounterClockwise`
- `bool depthBiasIsFloat`
- `float depthBias`
- `float depthBiasClamp`
- `float slopeScaledDepthBias`
- `bool depthClipEnable`
- `bool scissorEnable`
- `bool multiSampleEnable`
- `bool antialiasedLineEnable`

### MRenderCallback

Module `OpenMayaRender`. Source: [class_m_render_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_callback.html).

**Public Member Functions**

- `MRenderCallback ()`
- `virtual ~MRenderCallback ()`
- `virtual bool shadowCastCallback (const MRenderShadowData &data)`
- `virtual bool renderCallback (const MRenderData &data)`
- `virtual bool postProcessCallback (const MRenderData &data)`

**Static Public Member Functions**

- `static void addCallback (MRenderCallback *, int priority=0)`
- `static void removeCallback (MRenderCallback *)`
- `static MCallbackId addRenderTileCallback (MMessage::MRenderTileFunction func, MStatus *ReturnStatus)`
- `static const char * className ()`

### MRenderData

Module `OpenMayaRender`. Source: [class_m_render_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_data.html).

**Public Member Functions**

- `MRenderData ()`
- `void worldToScreen (const MFloatPoint &worldPoint, MFloatPoint &screenPoint) const`
- `void screenToWorld (const MFloatPoint &screenPoint, MFloatPoint &worldPoint) const`

**Public Attributes**

- `bool perspective`
- `unsigned short resX`
- `unsigned short resY`
- `unsigned short left`
- `unsigned short bottom`
- `unsigned short right`
- `unsigned short top`
- `unsigned short bytesPerChannel`
- `unsigned short xsize`
- `unsigned short ysize`
- `float fieldOfView`
- `float aspectRatio`
- `MFloatVector viewDirection`
- `MFloatPoint eyePoint`
- `MFloatMatrix worldToEyeMatrix`
- `unsigned char * rgbaArr`
- `float * depthArr`

### MRenderItem

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_item.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_item.html).

**Public Types**

- `enum RenderItemType { MaterialSceneItem, NonMaterialSceneItem, DecorationItem, InternalItem, InternalMaterialItem, InternalTexturedMaterialItem, InternalUnsupportedMaterialItem, OverrideNonMaterialItem }`
- `enum DefaultMaterialFiltering { IgnoreDefaultMaterialMode, DrawOnlyWhenDefaultMaterialActive, SkipWhenDefaultMaterialActive }`
- `typedef void(* LinkLostCallback) (MUserData *userData)`

**Public Member Functions**

- `void setDrawMode (MGeometry::DrawMode mode)`
- `bool setMatrix (const MMatrix *mat)`
- `bool setShader (const MShaderInstance *shader, const MString *customStreamName=NULL)`
- `MStatus setShaderFromNode2 (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback linkLostCb=nullptr, const MSharedPtr< MUserData > &linkLostUserData=nullptr, bool nonTextured=false)`
- `bool setShaderFromNode (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback=NULL, MUserData *linkLostUserData=NULL, bool nonTextured=false)`
- `void setExcludedFromPostEffects (bool exclude)`
- `void setSupportsAdvancedTransparency (bool support)`
- `void setCustomData (MUserData *userData)`
- `void setCustomData (MSharedPtr< MUserData > userData)`
- `MStatus addViewportComputeItem (MSharedPtr< MPxViewportComputeItem > viewportComputeItem)`
- `void enable (bool on)`
- `void castsShadows (bool on)`
- `void receivesShadows (bool on)`
- `void depthPriority (unsigned int val)`
- `MUint64 InternalObjectId () const`
- `bool isShaderFromNode () const`
- `MShaderInstance * getShader ()`
- `const MShaderInstance * getShader () const`
- `bool associateWithIndexBuffer (const MIndexBuffer *buffer) const`
- `bool excludedFromPostEffects () const`
- `bool supportsAdvancedTransparency () const`
- `MUserData * customData () const`
- `MSharedPtr< MUserData > getCustomData () const`
- `MSharedPtr< MPxViewportComputeItem > viewportComputeItem () const`
- `bool isEnabled () const`
- `bool castsShadows () const`
- `bool receivesShadows () const`
- `unsigned int depthPriority () const`
- `MString name () const`
- `void setPrimitive (MGeometry::Primitive primitive, int stride=0)`
- `MGeometry::DrawMode drawMode () const`
- `MDagPath sourceDagPath () const`
- `MObject component () const`
- `MObject shadingComponent () const`
- `void setAllowIsolateSelectCopy (bool state)`
- `bool allowIsolateSelectCopy () const`
- `bool isIsolateSelectCopy () const`
- `const MVertexBufferDescriptorList & requiredVertexBuffers () const`
- `const MGeometry * geometry () const`
- `MGeometry * geometry ()`
- `MBoundingBox boundingBox (MSpace::Space space=MSpace::kObject, MStatus *ReturnStatus=NULL) const`
- `MStatus setBoundingBox (const MBoundingBox &bounds)`
- `bool isConsolidated () const`
- `bool isMultiDraw () const`
- `void sourceIndexMapping (MGeometryIndexMapping &geometryIndexMapping) const`
- `bool isSourceIndexMappingValid (const MGeometryIndexMapping &geometryIndexMapping) const`
- `void setSourceIndexMapping (const MGeometryIndexMapping &geometryIndexMapping)`
- `bool wantConsolidation () const`
- `void setWantConsolidation (bool state)`
- `bool isCompatibleWithMayaInstancer () const`
- `void setCompatibleWithMayaInstancer (bool state)`
- `void setTreatAsTransparent (bool val)`
- `DefaultMaterialFiltering getDefaultMaterialHandling () const`
- `void setDefaultMaterialHandling (DefaultMaterialFiltering state)`
- `MSelectionMask selectionMask () const`
- `void setSelectionMask (const MSelectionMask &mask)`
- `MUint64 objectTypeExclusionFlag () const`
- `MStatus setObjectTypeExclusionFlag (MUint64 flag)`
- `unsigned int availableShaderParameters (MStringArray &parameterNames, MStatus *ReturnStatus=NULL) const`
- `bool getShaderBoolParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`
- `int getShaderIntParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`
- `float getShaderFloatParameter (const MString &parameterName, MStatus *ReturnStatus=NULL) const`
- `const float * getShaderFloatArrayParameter (const MString &parameterName, unsigned int &size, MStatus *ReturnStatus=NULL) const`
- `bool wantSubSceneConsolidation () const`
- `void setWantSubSceneConsolidation (bool state)`
- `~MRenderItem ()`
- `MRenderItem ()`
- `MStatus setShaderFromNodePython2 (const MObject &shaderNode, const MDagPath &shapePath, PythonCallbackWrapper *linkLostCb=nullptr, const MSharedPtr< MUserData > &linkLostUserData=nullptr, bool nonTextured=false)`
- `bool setShaderFromNodePython (const MObject &shaderNode, const MDagPath &shapePath, PythonCallbackWrapper *=nullptr, MUserData *linkLostUserData=nullptr, bool nonTextured=false)`

**Static Public Member Functions**

- `static MRenderItem * Create (const MString &name, RenderItemType type, MGeometry::Primitive primitive)`
- `static MRenderItem * Create (const MString &name, RenderItemType type, MGeometry::Primitive primitive, int stride)`
- `static MRenderItem * Create (const MRenderItem &item)`
- `static void Destroy (MRenderItem *&item)`
- `static const char * className ()`
- `static MRenderItem * Create (const MString &name, MGeometry::Primitive primitive, MGeometry::DrawMode mode, bool raiseAboveShaded)`

**Public Attributes**

- `OPENMAYA_PRIVATE RenderItemType type`
- `OPENMAYA_PRIVATE RenderItemType MGeometry::Primitive primitive`

**Static Public Attributes**

- `static unsigned int sDormantFilledDepthPriority`
- `static unsigned int sDormantWireDepthPriority`
- `static unsigned int sHiliteWireDepthPriority`
- `static unsigned int sActiveWireDepthPriority`
- `static unsigned int sActiveLineDepthPriority`
- `static unsigned int sDormantPointDepthPriority`
- `static unsigned int sActivePointDepthPriority`
- `static unsigned int sSelectionDepthPriority`

### MRenderItemList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_item_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_item_list.html).

**Public Member Functions**

- `int length () const`
- `int indexOf (const MString &name) const`
- `int indexOf (const MString &name, MRenderItem::RenderItemType type) const`
- `int indexOf (const MString &name, MGeometry::Primitive primitive, MGeometry::DrawMode mode) const`
- `const MRenderItem * itemAt (int index) const`
- `MRenderItem * itemAt (int index)`
- `bool append (MRenderItem *item)`
- `bool removeAt (int index)`
- `void clear ()`
- `MRenderItemList (const MRenderItemList &)`
- `~MRenderItemList ()`

**Static Public Member Functions**

- `static const char * className ()`

### MRenderOperation

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_operation.html).

**Public Types**

- `enum MRenderOperationType { kClear, kSceneRender, kQuadRender, kUserDefined, kHUDRender, kPresentTarget }`

**Public Member Functions**

- `virtual bool enableSRGBWrite ()`
- `virtual const MFloatPoint * viewportRectangleOverride ()`
- `virtual const MString & name () const`
- `virtual bool getInputTargetDescription (const MString &name, MRenderTargetDescription &description)`
- `virtual int writableTargets (unsigned int &count)`
- `virtual MRenderTarget *const * targetOverrideList (unsigned int &listSize)`
- `const MStringArray & inputTargets ()`
- `const MStringArray & outputTargets ()`
- `void renameOutputTarget (const MString &name, const MString &newName)`
- `MRenderOperationType operationType () const`

**Protected Member Functions**

- `MRenderOperation (const MString &name)`
- `MRenderTarget * getInputTarget (const MString &name)`

**Protected Attributes**

- `MRenderOperationType mOperationType`
- `MString mName`

### MRenderOperationList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_operation_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_operation_list.html).

**Public Member Functions**

- `MRenderOperationList ()`
- `~MRenderOperationList ()`
- `int length () const`
- `const MRenderOperation * operator[] (int index) const`
- `MRenderOperation * operator[] (int index)`
- `bool replace (const MString &name, MRenderOperation *operation)`
- `bool append (MRenderOperation *operation)`
- `bool insertBefore (const MString &name, MRenderOperation *operation)`
- `bool insertAfter (const MString &name, MRenderOperation *operation)`
- `bool remove (const MString &name)`
- `bool removeAt (int index)`
- `MRenderOperation * take (const MString &name)`
- `MRenderOperation * takeAt (int index)`
- `int indexOf (const MString &name)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MRenderOverride

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_override.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_override.html).

**Public Member Functions**

- `MRenderOverride (const MString &name)`
- `virtual ~MRenderOverride ()`
- `virtual ::MHWRender::DrawAPI supportedDrawAPIs () const`
- `virtual bool startOperationIterator ()`
- `virtual MRenderOperation * renderOperation ()`
- `virtual bool nextRenderOperation ()`
- `const MString & name () const`
- `virtual MString uiName () const`
- `virtual MStatus setup (const MString &destination)`
- `virtual MStatus cleanup ()`
- `virtual bool select (const MFrameContext &frameContext, const MSelectionInfo &selectInfo, bool useDepth, MSelectionList &selectionList, MPointArray &worldSpaceHitPts)`
- `const MFrameContext * getFrameContext () const`

### MRenderParameters

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_parameters.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_parameters.html).

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture1, kTexture2, kTexture3, kTextureCube, kSampler, kRenderTarget, kString }`

**Public Member Functions**

- `MRenderParameters ()`
- `MRenderParameters (void *)`
- `MRenderParameters (const MRenderParameters &copy)`
- `virtual ~MRenderParameters ()`
- `MRenderParameters & operator= (const MRenderParameters &other)`
- `void parameterList (MStringArray &list) const`
- `ParameterType parameterType (const MString &parameterName) const`
- `bool isArrayParameter (const MString &parameterName) const`
- `MString semantic (const MString &parameterName) const`
- `MStatus setParameter (const MString &parameterName, bool value)`
- `MStatus setParameter (const MString &parameterName, int value)`
- `MStatus setParameter (const MString &parameterName, float value)`
- `MStatus setParameter (const MString &parameterName, const float *value)`
- `MStatus setParameter (const MString &parameterName, const MFloatVector &value)`
- `MStatus setParameter (const MString &parameterName, const MMatrix &value)`
- `MStatus setParameter (const MString &parameterName, const MFloatMatrix &value)`
- `MStatus setParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`
- `MStatus setParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`
- `MStatus setParameter (const MString &parameterName, const MSamplerState &sampler)`
- `MStatus setParameter (const MString &parameterName, const MString &str)`
- `MStatus setArrayParameter (const MString &parameterName, const bool *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const int *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const float *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const MMatrix *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const MString *values, unsigned int count)`
- `MStatus getParameter (const MString &parameterName, bool &value)`
- `MStatus getParameter (const MString &parameterName, int &value)`
- `MStatus getParameter (const MString &parameterName, float &value)`
- `MStatus getParameter (const MString &parameterName, float *&value)`
- `MStatus getParameter (const MString &parameterName, MFloatVector &value)`
- `MStatus getParameter (const MString &parameterName, MMatrix &value)`
- `MStatus getParameter (const MString &parameterName, MFloatMatrix &value)`
- `MStatus getParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`
- `MStatus getParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`
- `MStatus getParameter (const MString &parameterName, MSamplerStateDesc &samplerDesc)`
- `MStatus getParameter (const MString &parameterName, MString &str)`
- `MStatus getParameter (const MString &parameterName, MIntArray &values)`
- `MStatus getParameter (const MString &parameterName, MFloatArray &values)`
- `MStatus getParameter (const MString &parameterName, MMatrixArray &values)`
- `MStatus getParameter (const MString &parameterName, MStringArray &values)`

### MRenderPassDef

Module `OpenMayaRender`. Source: [class_m_render_pass_def.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_pass_def.html).

**Public Member Functions**

- `virtual ~MRenderPassDef ()`
- `MPxRenderPassImpl * getImplementation (const MString &renderer, MPxRenderPassImpl::PassTypeBit fbType, unsigned int numChannels, MStatus *ReturnStatus=NULL)`
- `MPxRenderPassImpl * getImplementation (const MString &renderer, MStatus *ReturnStatus=NULL)`
- `MString getID (MStatus *ReturnStatus=NULL) const`
- `MString getName (MStatus *ReturnStatus=NULL) const`
- `MString getGroup (MStatus *ReturnStatus=NULL) const`
- `MString getDescription (MStatus *ReturnStatus=NULL) const`
- `MString getAttributeType (const MString &attributeName, MStatus *ReturnStatus=NULL) const`
- `MStatus addFloatParameter (const MString &longName, const MString &shortName, const MString &UIName, const float &minValue, const float &maxValue, const float &defaultValue)`
- `MStatus addDoubleParameter (const MString &longName, const MString &shortName, const MString &UIName, const double &minValue, const double &maxValue, const double &defaultValue)`
- `MStatus addIntParameter (const MString &longName, const MString &shortName, const MString &UIName, const int &minValue, const int &maxValue, const int &defaultValue)`

**Static Public Member Functions**

- `static const char * className ()`

**Friends**

- `class MRenderPassRegistry`
- `class MFnPlugin`

### MRenderPassRegistry

Module `OpenMayaRender`. Source: [class_m_render_pass_registry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_pass_registry.html).

**Static Public Member Functions**

- `static MRenderPassDef getRenderPassDefinition (const MString &passID, MStatus *ReturnStatus=NULL)`
- `static MRenderPassDef registerRenderPassDefinition (const MString &passID, const MString &passName, const MString &passGroup, const MString &description, bool overload=false, MStatus *ReturnStatus=NULL)`
- `static const char * className ()`

### MRenderProfile

Module `OpenMayaRender`. Source: [class_m_render_profile.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_profile.html).

**Public Types**

- `enum MStandardRenderer { kMayaSoftware, kMayaOpenGL, kMayaD3D }`

**Public Member Functions**

- `MRenderProfile ()`
- `~MRenderProfile ()`
- `unsigned int numberOfRenderers () const`
- `void addRenderer (MStandardRenderer renderer)`
- `void addRenderer (const MString &name, float version)`
- `bool hasRenderer (MStandardRenderer renderer) const`
- `bool hasRenderer (const MString &name, float version) const`

### MRenderScriptCallback

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_script_callback.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_script_callback.html).

**Public Member Functions**

- `MRenderScriptCallback ()`
- `virtual ~MRenderScriptCallback ()`
- `virtual MString name () const =0`
- `virtual bool execute (MRenderParameters *params, MString arg1, MString arg2, MString arg3)=0`

### MRenderSetup

Module `OpenMayaRender`. Source: [class_m_render_setup.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_setup.html).

**Static Public Member Functions**

- `static const char * className ()`
- `static MStringArray getEnabledSelectedNodeNames (const MString &renderLayerName, MStatus *status=nullptr)`

### MRenderShadowData

Module `OpenMayaRender`. Source: [class_m_render_shadow_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_shadow_data.html).

**Public Types**

- `enum LightType { kInvalid, kPoint, kDirectional, kSpot }`

**Public Member Functions**

- `MRenderShadowData ()`
- `void worldToZbuffer (const MFloatPoint &worldPoint, MFloatPoint &screenPoint) const`
- `void zbufferToWorld (const MFloatPoint &screenPoint, MFloatPoint &worldPoint) const`

**Public Attributes**

- `bool perspective`
- `bool useMidDistMap`
- `LightType lightType`
- `unsigned short shadowResX`
- `unsigned short shadowResY`
- `MFloatPoint lightPosition`
- `MFloatMatrix projectionMatrix`
- `MFloatMatrix perspectiveMatrix`
- `float * depthMaps`
- `float * midDistMaps`

### MRenderTarget

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target.html).

**Public Member Functions**

- `MStatus updateDescription (const MRenderTargetDescription &targetDescription)`
- `void targetDescription (MRenderTargetDescription &desc) const`
- `void * resourceHandle () const`
- `void * rawData (int &rowPitch, size_t &slicePitch)`

**Static Public Member Functions**

- `static const char * className ()`
- `static void freeRawData (void *data)`

### MRenderTargetDescription

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target_description.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target_description.html).

**Public Member Functions**

- `MRenderTargetDescription ()`
- `MRenderTargetDescription (const MString &name, unsigned int width, unsigned int height, unsigned int multiSampleCount,::MHWRender::MRasterFormat rasterFormat, unsigned int arraySliceCount, bool isCubeMap)`
- `virtual ~MRenderTargetDescription ()`
- `const MString & name () const`
- `unsigned int width () const`
- `unsigned int height () const`
- `unsigned int multiSampleCount () const`
- `::MHWRender::MRasterFormat rasterFormat () const`
- `unsigned int arraySliceCount () const`
- `bool isCubeMap () const`
- `bool allowsUnorderedAccess () const`
- `void setName (const MString &name)`
- `void setWidth (unsigned int val)`
- `void setHeight (unsigned int val)`
- `void setMultiSampleCount (unsigned int val)`
- `void setRasterFormat (::MHWRender::MRasterFormat val)`
- `void setArraySliceCount (unsigned int val)`
- `void setIsCubeMap (bool val)`
- `void setAllowsUnorderedAccess (bool val)`
- `bool compatibleWithDescription (const MRenderTargetDescription &desc)`

### MRenderTargetLegacy

Module `OpenMayaRender`. Source: [class_m_render_target_legacy.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_target_legacy.html).

**Public Member Functions**

- `unsigned int width () const`
- `unsigned int height () const`
- `void makeTargetCurrent () const`
- `MStatus writeColorBuffer (const MImage &image, signed short x=0, signed short y=0, bool writeDepth=false) const`

### MRenderTargetManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_target_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_target_manager.html).

**Public Member Functions**

- `MRenderTarget * acquireRenderTarget (const MRenderTargetDescription &targetDescription) const`
- `MRenderTarget * acquireRenderTargetFromScreen (const MString &targetName) const`
- `bool formatSupportsSRGBWrite (::MHWRender::MRasterFormat format) const`
- `void releaseRenderTarget (const MRenderTarget *target) const`

**Static Public Member Functions**

- `static const char * className ()`

### MRenderUtil

Module `OpenMayaRender`. Source: [class_m_render_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_util.html).

**Public Types**

- `enum MRenderState { kNotRendering, kBatchRender, kInteractiveRender, kIprRender, kHardwareRender }`
- `enum MRenderPass { kAll, kColorOnly, kShadowOnly, kAmbientOnly, kDiffuseOnly, kSpecularOnly }`

**Static Public Member Functions**

- `static MRenderState mayaRenderState ()`
- `static MStatus raytrace (const MFloatVector &rayOrigin, const MFloatVector &rayDirection, const void *objectId, const void *raySampler, const short rayDepth, MFloatVector &resultColor, MFloatVector &resultTransparency, const bool isReflectedRays=true)`
- `static MStatus raytrace (const MFloatVectorArray &rayOrigins, const MFloatVectorArray &rayDirections, const void *objectId, const void *raySampler, const short rayDepth, MFloatVectorArray &resultColors, MFloatVectorArray &resultTransparencies, const bool isReflectedRays=true)`
- `static MStatus raytraceFirstGeometryIntersections (const MFloatVectorArray &rayOrigins, const MFloatVectorArray &rayDirections, const void *objectId, const void *raySampler, MFloatVectorArray &resultIntersections, MIntArray &resultIntersected)`
- `static MStatus sampleShadingNetwork (MString shadingNodeName, int numSamples, bool useShadowMaps, bool reuseMaps, MFloatMatrix &cameraMatrix, MFloatPointArray *points, MFloatArray *uCoords, MFloatArray *vCoords, MFloatVectorArray *normals, MFloatPointArray *refPoints, MFloatVectorArray *tangentUs, MFloatVectorArray *tangentVs, MFloatArray *filterSizes, MFloatVectorArray &resultColors, MFloatVectorArray &resultTransparencies)`
- `static bool generatingIprFile ()`
- `static MString relativeFileName (const MString &absFileName, MStatus *ReturnStatus=NULL)`
- `static bool exactFileTextureName (const MObject &fileNode, MString &texturePath, MStatus *ReturnStatus=NULL)`
- `static bool exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, MString &exactName, MStatus *ReturnStatus=NULL)`
- `static bool exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, const MString &contextNodeFullName, MString &exactName, MStatus *ReturnStatus=NULL)`
- `static bool exactFileTextureUvTileData (const MObject &fileNode, MStringArray &tilePaths, MFloatArray &tilePositions, MStatus *ReturnStatus=NULL)`
- `static bool convertPsdFile (const MObject &fileNode, MString &texturePath, const bool &forExport=false, MStatus *ReturnStatus=NULL)`
- `static bool exactImagePlaneFileName (const MObject &imagePlaneNode, MString &texturePath, MStatus *ReturnStatus=NULL)`
- `static MString exactFileTextureName (const MObject &fileNode, MStatus *ReturnStatus=NULL)`
- `static MString exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, MStatus *ReturnStatus=NULL)`
- `static MString exactFileTextureName (const MString &baseName, bool useFrameExt, const MString &currentFrameExt, const MString &contextNodeFullName, MStatus *ReturnStatus=NULL)`
- `static MString convertPsdFile (const MObject &fileNode, const bool &forExport=false, MStatus *ReturnStatus=NULL)`
- `static MString exactImagePlaneFileName (const MObject &imagePlaneNode, MStatus *ReturnStatus=NULL)`
- `static bool inCurrentRenderLayer (const MDagPath &objectPath, MStatus *ReturnStatus=NULL)`
- `static MRenderPass renderPass (void)`
- `static MString mainBeautyPassCustomTokenString ()`
- `static MString mainBeautyPassName ()`
- `static float diffuseReflectance (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &normal, bool lightBackFace, MStatus *ReturnStatus=NULL)`
- `static MFloatVector maximumSpecularReflection (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &normal, const MFloatVector &rayDirection, MStatus *ReturnStatus=NULL)`
- `static float lightAttenuation (const void *lightBlindData, const MFloatVector &pointCamera, const MFloatVector &normal, bool lightBackFace, MStatus *ReturnStatus=NULL)`
- `static float hemisphereCoverage (const void *lightBlindData, const MFloatVector &lightDirection, const MFloatVector &pointCamera, const MFloatVector &rayDirection, bool lightBackFace, MStatus *ReturnStatus=NULL)`
- `static void sendRenderProgressInfo (const MString &pixFile, int percentageDone)`
- `static void getCommonRenderSettings (MCommonRenderSettingsData &rgData)`
- `static MStatus renderObjectItem (const void *objectId, MSelectionList &item)`
- `static const char * className ()`
- `static MStatus eval2dTexture (const MObject &texNode, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`
- `static unsigned int noiseTableSize ()`
- `static unsigned int noiseTableCubeSide ()`
- `static float valueInNoiseTable (unsigned int index)`
- `static float noise1 (float x)`
- `static float noise2 (float x, float y)`
- `static float noise3 (float x, float y, float z)`
- `static float noise4 (float x, float y, float z, float t, unsigned short numTimeSteps)`

### MRenderUtilities

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_render_utilities.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_render_utilities.html).

**Public Types**

- `enum MaterialViewerCamera { kPerspectiveCamera = 0, kOrthogonalCameraWithMargin, kOrthogonalCameraCloseUp }`
- `enum MaterialViewerLights { kDefaultLights = 0, kSwatchLight, kAmbientLight }`

**Static Public Member Functions**

- `static MDrawContext * acquireSwatchDrawContext (const MRenderTarget *colorTarget=NULL, const MRenderTarget *depthTarget=NULL)`
- `static MDrawContext * acquireUVTextureDrawContext (const MRenderTarget *colorTarget=NULL, const MRenderTarget *depthTarget=NULL)`
- `static void releaseDrawContext (MDrawContext *context, bool releaseTargets=true)`
- `static void swatchBackgroundColor (float &r, float &g, float &b, float &a)`
- `static MStatus blitTargetToImage (const MRenderTarget *target, MImage &image)`
- `static MStatus blitTargetToGL (const MRenderTarget *target, const float region[2][2], bool unfiltered)`
- `static MStatus drawSimpleMesh (MDrawContext &context, const MVertexBuffer &vertexBuffer, const MIndexBuffer &indexBuffer, MGeometry::Primitive primitiveType, int start, int count)`
- `static MStatus renderMaterialViewerGeometry (const MString &shape, MObject shaderNode, MImage &image, MaterialViewerCamera cameraMode=kPerspectiveCamera, MaterialViewerLights lightRig=kDefaultLights)`
- `static const char * className ()`

### MRenderView

Module `OpenMayaRender`. Source: [class_m_render_view.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_view.html).

**Static Public Member Functions**

- `static bool doesRenderEditorExist ()`
- `static MStatus setCurrentCamera (MDagPath &camera)`
- `static MStatus getRenderRegion (unsigned int &left, unsigned int &right, unsigned int &bottom, unsigned int &top)`
- `static MStatus startRender (unsigned int width, unsigned int height, bool doNotClearBackground=false, bool immediateFeedback=false)`
- `static MStatus startRegionRender (unsigned int imageWidth, unsigned int imageHeight, unsigned int regionLeft, unsigned int regionRight, unsigned int regionBottom, unsigned int regionTop, bool doNotClearBackground=false, bool immediateFeedback=false)`
- `static MStatus updatePixels (unsigned int left, unsigned int right, unsigned int bottom, unsigned int top, RV_PIXEL *pPixels, bool isHdr=false, const unsigned int numberOfAOVs=0, const RV_AOV *const pAOVs=NULL)`
- `static MStatus refresh (unsigned int left, unsigned int right, unsigned int bottom, unsigned int top)`
- `static MStatus endRender ()`
- `static const char * className ()`

### MRenderer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_renderer.html).

**Public Types**

- `enum MGPUMemType { kMemTexture, kMemAll }`
- `typedef void(* NotificationCallback) (MDrawContext &context, void *clientData)`

**Public Member Functions**

- `::MHWRender::DrawAPI drawAPI () const`
- `bool drawAPIIsOpenGL () const`
- `unsigned int drawAPIVersion () const`
- `void * GPUDeviceHandle () const`
- `unsigned int GPUmaximumVertexBufferSize () const`
- `unsigned int GPUmaximumPrimitiveCount () const`
- `void GPUmaximumOutputTargetSize (unsigned int &w, unsigned int &h) const`
- `MInt64 GPUtotalMemorySize () const`
- `MInt64 GPUUsedMemorySize (MGPUMemType type) const`
- `MFragmentManager * getFragmentManager () const`
- `const MShaderManager * getShaderManager () const`
- `const MRenderTargetManager * getRenderTargetManager () const`
- `bool copyTargetToScreen (const MRenderTarget *renderTarget)`
- `bool render (const MString &sourceName, MRenderTarget **targets, unsigned int count)`
- `MTextureManager * getTextureManager () const`
- `MStatus registerOverride (const MRenderOverride *roverride)`
- `MStatus deregisterOverride (const MRenderOverride *roverride)`
- `const MRenderOverride * findRenderOverride (const MString &name)`
- `const MString activeRenderOverride () const`
- `unsigned int renderOverrideCount ()`
- `MStatus setRenderOverrideName (const MString &name)`
- `MString renderOverrideName (MStatus *ReturnStatus=NULL) const`
- `MStatus registerScriptCallback (MRenderScriptCallback *callback)`
- `MStatus deregisterScriptCallback (MRenderScriptCallback *callback)`
- `MStatus holdGPUMemory (MInt64 sizeInBytes, MInt64 *evictedGPUMemSize=NULL)`
- `MStatus releaseGPUMemory (MInt64 sizeInBytes)`
- `void getStandardViewportOperations (MRenderOperationList &operations) const`
- `void setOutputTargetOverrideSize (unsigned int w, unsigned int h)`
- `void getOutputTargetOverrideSize (int &w, int &h)`
- `void unsetOutputTargetOverrideSize ()`
- `MStatus outputTargetSize (unsigned int &w, unsigned int &h) const`
- `bool setLightRequiresShadows (MObject obj, bool flag)`
- `MStatus addNotification (NotificationCallback notification, const MString &name, const MString semanticLocation, void *clientData)`
- `MStatus removeNotification (const MString &name, const MString &semanticLocation)`
- `unsigned int notificationCount (const MString &semanticLocation) const`
- `bool presentOnScreen () const`
- `void setPresentOnScreen (bool val)`
- `bool useGradient () const`
- `MColor clearColor () const`
- `MColor clearColor2 () const`

**Static Public Member Functions**

- `static MRenderer * theRenderer (bool initializeRenderer=true)`
- `static void disableChangeManagementUntilNextRefresh ()`
- `static void setGeometryDrawDirty (MObject obj, bool topologyChanged=true)`
- `static void needEvaluateAllLights ()`
- `static void setLightsAndShadowsDirty ()`

### MRenderingInfo

Module `OpenMayaRender`. Source: [class_m_rendering_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_rendering_info.html).

**Public Member Functions**

- `unsigned int originX () const`
- `unsigned int originY () const`
- `unsigned int width () const`
- `unsigned int height () const`
- `MViewportRenderer::RenderingAPI renderingAPI () const`
- `float renderingVersion () const`
- `const MDagPath & cameraPath () const`
- `const MRenderTargetLegacy & renderTarget () const`
- `const MMatrix & viewMatrix () const`
- `const MMatrix & projectionMatrix () const`

### MSamplerState

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sampler_state.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sampler_state.html).

**Public Types**

- `enum TextureFilter { kMinMagMipPoint = 0, kMinMagPoint_MipLinear = 1, kMinPoint_MagLinear_MipPoint = 0x4, kMinPoint_MagMipLinear = 0x5, kMinLinear_MagMipPoint = 0x10, kMinLinear_MagPoint_MipLinear = 0x11, kMinMagLinear_MipPoint = 0x14, kMinMagMipLinear = 0x15, kAnisotropic = 0x55 }`
- `enum TextureAddress { kTexWrap = 1, kTexMirror = 2, kTexClamp = 3, kTexBorder = 4 }`

**Public Member Functions**

- `const MSamplerStateDesc & desc (MStatus *ReturnStatus=NULL) const`
- `void * resourceHandle () const`

**Static Public Member Functions**

- `static const char * className ()`

### MSamplerStateDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sampler_state_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sampler_state_desc.html).

**Public Member Functions**

- `MSamplerStateDesc ()`
- `~MSamplerStateDesc ()`
- `void setDefaults ()`
- `MSamplerStateDesc (const MSamplerStateDesc &rDesc)`
- `MSamplerStateDesc & operator= (const MSamplerStateDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `MSamplerState::TextureFilter filter`
- `MStateManager::CompareMode comparisonFn`
- `MSamplerState::TextureAddress addressU`
- `MSamplerState::TextureAddress addressV`
- `MSamplerState::TextureAddress addressW`
- `float borderColor [4]`
- `float mipLODBias`
- `unsigned int minLOD`
- `unsigned int maxLOD`
- `unsigned int maxAnisotropy`
- `unsigned int coordCount`
- `unsigned int elementIndex`

### MSceneRender

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_scene_render.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_scene_render.html).

**Public Types**

- `enum MObjectTypeExclusions { kExcludeNone = 0, kExcludeNurbsCurves = 1<<(0), kExcludeNurbsSurfaces = 1<<(1), kExcludeMeshes = 1<<(2), kExcludePlanes = 1<<(3), kExcludeLights = 1<<(4), kExcludeCameras = 1<<(5), kExcludeJoints = 1<<(6), kExcludeIkHandles = 1<<(7), kExcludeDeformers = 1<<(8), kExcludeDynamics = 1<<(9), kExcludeParticleInstancers = 1<<(10), kExcludeLocators = 1<<(11), kExcludeDimensions = 1<<(12), kExcludeSelectHandles = 1<<(13), kExcludePivots = 1<<(14), kExcludeTextures = 1<<(15), kExcludeGrid = 1<<(16), kExcludeCVs = 1<<(17), kExcludeHulls = 1<<(18), kExcludeStrokes = 1<<(19), kExcludeSubdivSurfaces = 1<<(20), kExcludeFluids = 1<<(21), kExcludeFollicles = 1<<(22), kExcludeHairSystems = 1<<(23), kExcludeImagePlane = 1<<(24), kExcludeNCloths = 1<<(25), kExcludeNRigids = 1<<(26), kExcludeDynamicConstraints = 1<<(27), kExcludeManipulators = 1<<(28), kExcludeNParticles = 1<<(29), kExcludeMotionTrails = 1<<(30), kExcludeHoldOuts = 1<<(31), kExcludeAll = ~0 }`
- `enum MSceneFilterOption { , kRenderPreSceneUIItems = 1 << (0), kRenderOpaqueShadedItems = 1 << (1), kRenderTransparentShadedItems = 1 << (2), kRenderShadedItems = (kRenderOpaqueShadedItems | kRenderTransparentShadedItems), kRenderPostSceneUIItems = 1 << (3) , kRenderAllItems = ~0 }`
- `enum MDisplayMode { kNoDisplayModeOverride = 0, kWireFrame = 1<<(0), kShaded = 1<<(1), kFlatShaded = 1<<(2), kShadeActiveOnly = 1<<(3), kBoundingBox = 1<<(4), kDefaultMaterial = 1<<(5), kTextured = 1<<(6) }`
- `enum MLightingMode { kNoLightingModeOverride = 0, kNoLight, kAmbientLight, kLightDefault, kSelectedLights, kSceneLights }`
- `enum MPostEffectsOverride { kPostEffectDisableNone = 0, kPostEffectDisableSSAO = 1 << (0), kPostEffectDisableMotionBlur = 1 << (1), kPostEffectDisableDOF = 1 << (2), kPostEffectDisableAll = ~0 }`
- `enum MCullingOption { kNoCullingOverride = 0, kCullNone, kCullBackFaces, kCullFrontFaces }`

**Public Member Functions**

- `MSceneRender (const MString &name)`
- `MSceneRender (const MString &name, const MString &fragmentName)`
- `~MSceneRender () override`
- `virtual void preRender ()`
- `virtual void postRender ()`
- `virtual void preSceneRender (const MDrawContext &context)`
- `virtual void postSceneRender (const MDrawContext &context)`
- `virtual MSceneFilterOption renderFilterOverride ()`
- `virtual const MCameraOverride * cameraOverride ()`
- `virtual const MSelectionList * objectSetOverride ()`
- `virtual const MShaderInstance * shaderOverride ()`
- `virtual MUint64 getObjectTypeExclusions ()`
- `virtual MDisplayMode displayModeOverride ()`
- `virtual MLightingMode lightModeOverride ()`
- `virtual const bool * shadowEnableOverride ()`
- `virtual MPostEffectsOverride postEffectsOverride ()`
- `virtual MCullingOption cullingOverride ()`
- `virtual MClearOperation & clearOperation ()`
- `virtual bool hasUIDrawables () const`
- `virtual void addPreUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`
- `virtual void addPostUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`
- `MRenderParameters * getParameters ()`
- `MString fragmentName ()`
- `virtual MObjectTypeExclusions objectTypeExclusions ()`

**Protected Attributes**

- `MClearOperation mClearOperation`

### MSelectionContext

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_selection_context.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_selection_context.html).

**Public Types**

- `enum SelectionLevel { kNone, kObject, kComponent, kFace, kEdge, kVertex }`

**Public Member Functions**

- `SelectionLevel selectionLevel (MStatus *ReturnStatus=nullptr) const`
- `MStatus setSelectionLevel (SelectionLevel level)`

**Static Public Member Functions**

- `static const char * className ()`

### MSelectionInfo

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_selection_info.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_selection_info.html).

**Public Member Functions**

- `bool singleSelection (MStatus *ReturnStatus=nullptr) const`
- `bool selectClosest (MStatus *ReturnStatus=nullptr) const`
- `bool selectable (const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`
- `bool selectableComponent (bool displayed, const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`
- `MStatus selectRect (unsigned int &x, unsigned int &y, unsigned int &width, unsigned int &height) const`
- `MStatus cursorPoint (int &x, int &y) const`
- `bool isRay (MStatus *ReturnStatus=nullptr) const`
- `MMatrix getAlignmentMatrix (MStatus *ReturnStatus=nullptr) const`
- `MStatus getLocalRay (MPoint &pnt, MVector &vec) const`
- `bool selectForHilite (const MSelectionMask &mask, MStatus *ReturnStatus=nullptr) const`
- `bool selectOnHilitedOnly (MStatus *ReturnStatus=nullptr) const`
- `bool pointSnapping (MStatus *ReturnStatus=nullptr) const`
- `bool snapToActive (MStatus *ReturnStatus=nullptr) const`

**Static Public Member Functions**

- `static const char * className ()`

### MShaderInstance

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_shader_instance.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_shader_instance.html).

**Public Types**

- `enum ParameterType { kInvalid, kBoolean, kInteger, kFloat, kFloat2, kFloat3, kFloat4, kFloat4x4Row, kFloat4x4Col, kTexture1, kTexture2, kTexture3, kTextureCube, kSampler }`
- `enum FragmentUsage { kPixelShader, kNormalShader, kNormalShader2, kVertexShader, kGeometryShader, kGlossShader, kGlossShader2, kRotationAngleShader, kRotationAngleShader2, kReflectanceShader, kReflectanceShader2, kRoughnessShader, kRoughnessShader2, kAnisotropyShader, kAnisotropyShader2, kDisplacementPosShader }`
- `typedef void(* DrawCallback) (MDrawContext &context, const MRenderItemList &renderItemList, MShaderInstance *shaderInstance)`

**Public Member Functions**

- `DrawCallback preDrawCallback () const`
- `DrawCallback postDrawCallback () const`
- `void parameterList (MStringArray &list) const`
- `ParameterType parameterType (const MString &parameterName) const`
- `bool isArrayParameter (const MString &parameterName) const`
- `int getArraySize (const MString &parameterName) const`
- `MString semantic (const MString &parameterName) const`
- `bool isVaryingParameter (const MString &parameterName) const`
- `MStatus setParameter (const MString &parameterName, bool value)`
- `MStatus setParameter (const MString &parameterName, int value)`
- `MStatus setParameter (const MString &parameterName, float value)`
- `MStatus setParameter (const MString &parameterName, const float *value)`
- `MStatus setParameter (const MString &parameterName, const MFloatVector &value)`
- `MStatus setParameter (const MString &parameterName, const MMatrix &value)`
- `MStatus setParameter (const MString &parameterName, const MFloatMatrix &value)`
- `MStatus setParameter (const MString &parameterName, MTextureAssignment &textureAssignment)`
- `MStatus setParameter (const MString &parameterName, MRenderTargetAssignment &targetAssignment)`
- `MStatus setParameter (const MString &parameterName, const MSamplerState &sampler)`
- `MStatus setArrayParameter (const MString &parameterName, const bool *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const int *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const float *values, unsigned int count)`
- `MStatus setArrayParameter (const MString &parameterName, const MMatrix *values, unsigned int count)`
- `MStatus addInputFragment (const MString &fragmentName, const MString &outputName, const MString &inputName, const MString promotedInputName=MString())`
- `MStatus addInputFragmentForMultiParams (const MString &fragmentName, const MString &uniqueName, const MStringArray &outputNames, const MStringArray &inputNames, MUintArray *invalidParameterIndices=nullptr, FragmentUsage fragmentUsage=kPixelShader)`
- `MStatus addOutputFragment (const MString &fragmentName, const MString &inputName)`
- `MStatus renameParameter (const MString &oldName, const MString &newName)`
- `MStatus setSemantic (const MString &paramName, const MString &semantic)`
- `MStatus setAsVarying (const MString &paramName, bool)`
- `MStatus bind (const MDrawContext &context) const`
- `unsigned int getPassCount (const MDrawContext &context, MStatus *status=NULL) const`
- `MStatus activatePass (const MDrawContext &context, unsigned int pass) const`
- `MStatus unbind (const MDrawContext &context) const`
- `MStatus updateParameters (const MDrawContext &context) const`
- `bool isTransparent () const`
- `MStatus setIsTransparent (bool value)`
- `MShaderInstance * clone () const`
- `MShaderInstance * createShaderInstanceWithColorManagementFragment (const MString &inputColorSpace)`
- `MStatus requiredVertexBuffers (MVertexBufferDescriptorList &list) const`
- `int annotationAsInt (const MString &parameterName, const MString &annotationName, MStatus &status)`
- `float annotationAsFloat (const MString &parameterName, const MString &annotationName, MStatus &status)`
- `MString annotationAsString (const MString &parameterName, const MString &annotationName, MStatus &status)`
- `MString parameterSemantic (const MString &parameterName, MStatus &status)`
- `void * parameterDefaultValue (const MString &parameterName, MStatus &status)`
- `MString resourceName (const MString &parameterName, MStatus &status)`
- `MString uiWidget (const MString &parameterName, MStatus &status)`
- `MString uiName (const MString &parameterName, MStatus &uiName)`
- `MStatus techniqueNames (MStringArray &techniqueNames) const`
- `int techniqueAnnotationAsInt (const MString &annotationName, MStatus &status)`
- `float techniqueAnnotationAsFloat (const MString &annotationName, MStatus &status)`
- `MString techniqueAnnotationAsString (const MString &annotationName, MStatus &status)`
- `int passAnnotationAsInt (unsigned int pass, const MString &annotationName, MStatus &status)`
- `float passAnnotationAsFloat (unsigned int pass, const MString &annotationName, MStatus &status)`
- `MString passAnnotationAsString (unsigned int pass, const MString &annotationName, MStatus &status)`
- `MStatus writeEffectSourceToFile (const MString &filePath) const`

**Static Public Member Functions**

- `static const char * className ()`

### MShaderManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_shader_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_shader_manager.html).

**Public Types**

- `enum MStockShader { k3dSolidShader, k3dBlinnShader, k3dDefaultMaterialShader, k3dSolidTextureShader, k3dCPVFatPointShader, k3dColorLookupFatPointShader, k3dOpacityLookupFatPointShader, k3dColorOpacityLookupFatPointShader, k3dShadowerShader, k3dFatPointShader, k3dThickLineShader, k3dCPVThickLineShader, k3dDashLineShader, k3dCPVDashLineShader, k3dStippleShader, k3dThickDashLineShader, k3dCPVThickDashLineShader, k3dDepthShader, k3dCPVSolidShader, k3dIntegerNumericShader, k3dFloatNumericShader, k3dFloat2NumericShader, k3dFloat3NumericShader, k3dPointVectorShader, k3dPointLightShadowerShader, k3dStandardSurfaceShader }`
- `typedef void(* LinkLostCallback) (MShaderInstance *shaderInstance, MUserData *userData)`

**Public Member Functions**

- `MStatus addShaderPath (const MString &path) const`
- `MStatus shaderPaths (MStringArray &paths) const`
- `MStatus addShaderIncludePath (const MString &path) const`
- `MStatus shaderIncludePaths (MStringArray &paths) const`
- `MStatus clearEffectCache () const`
- `MStatus removeEffectFromCache (const MString &effectsFileName, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0) const`
- `void getEffectsTechniques (const MString &effectsFileName, MStringArray &techniqueNames, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true) const`
- `MShaderInstance * getEffectsFileShader (const MString &effectsFileName, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`
- `MShaderInstance * getEffectsBufferShader (const void *buffer, unsigned int size, const MString &techniqueName, MShaderCompileMacro *macros=0, unsigned int numberOfMacros=0, bool useEffectCache=true, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`
- `MShaderInstance * getFragmentShader (const MString &fragmentName, const MString &structOutputName, bool decorateFragment, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`
- `MShaderInstance * getShaderFromNode (const MObject &shaderNode, const MDagPath &shapePath, LinkLostCallback linkLostCb=0, MUserData *linkLostUserData=0, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0, bool nonTextured=false) const`
- `MShaderInstance * getStockShader (MStockShader name, MShaderInstance::DrawCallback preCb=0, MShaderInstance::DrawCallback postCb=0) const`
- `void releaseShader (MShaderInstance *shader) const`

**Static Public Member Functions**

- `static bool isSupportedShaderSemantic (const MString &value)`
- `static MString getLastError ()`
- `static MString getLastErrorSource (bool displayLineNumber=false, bool filterSource=false, unsigned int numSurroundingLines=2)`
- `static const char * className ()`

### MStateManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_state_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_state_manager.html).

**Public Types**

- `enum CompareMode { kCompareNever = 1, kCompareLess = 2, kCompareEqual = 3, kCompareLessEqual = 4, kCompareGreater = 5, kCompareNotEqual = 6, kCompareGreaterEqual = 7, kCompareAlways = 8 }`
- `enum ShaderType { kNoShader, kVertexShader, kGeometryShader, kPixelShader, kHullShader, kDomainShader }`

**Public Member Functions**

- `MStatus setBlendState (const MBlendState *blendState)`
- `const MBlendState * getBlendState (MStatus *ReturnStatus=NULL) const`
- `MStatus setRasterizerState (const MRasterizerState *rasterizerState)`
- `const MRasterizerState * getRasterizerState (MStatus *ReturnStatus=NULL) const`
- `MStatus setDepthStencilState (const MDepthStencilState *depthStencilState)`
- `const MDepthStencilState * getDepthStencilState (MStatus *ReturnStatus=NULL) const`
- `MStatus setSamplerState (ShaderType shader, int samplerIndex, const MSamplerState *samplerState)`
- `const MSamplerState * getSamplerState (ShaderType shader, int samplerIndex, MStatus *ReturnStatus=NULL) const`

**Static Public Member Functions**

- `static const MBlendState * acquireBlendState (const MBlendStateDesc &blendStateDesc, MStatus *ReturnStatus=NULL)`
- `static void releaseBlendState (const MBlendState *state)`
- `static const MRasterizerState * acquireRasterizerState (const MRasterizerStateDesc &rasterizerStateDesc, MStatus *ReturnStatus=NULL)`
- `static void releaseRasterizerState (const MRasterizerState *state)`
- `static const MDepthStencilState * acquireDepthStencilState (const MDepthStencilStateDesc &desc, MStatus *ReturnStatus=NULL)`
- `static void releaseDepthStencilState (const MDepthStencilState *state)`
- `static int getMaxSamplerCount ()`
- `static const MSamplerState * acquireSamplerState (const MSamplerStateDesc &samplerStateDesc, MStatus *ReturnStatus=NULL)`
- `static void releaseSamplerState (const MSamplerState *state)`
- `static const char * className ()`

### MStencilOpDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_stencil_op_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_stencil_op_desc.html).

**Public Member Functions**

- `MStencilOpDesc ()`
- `~MStencilOpDesc ()`
- `void setDefaults ()`
- `MStencilOpDesc (const MStencilOpDesc &rDesc)`
- `MStencilOpDesc & operator= (const MStencilOpDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `MDepthStencilState::StencilOperation stencilPassOp`
- `MDepthStencilState::StencilOperation stencilFailOp`
- `MDepthStencilState::StencilOperation stencilDepthFailOp`
- `MStateManager::CompareMode stencilFunc`

### MSubSceneContainer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_sub_scene_container.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_sub_scene_container.html).

**Public Member Functions**

- `bool add (MRenderItem *item)`
- `bool remove (const MString &name)`
- `void clear ()`
- `MRenderItem * find (const MString &name)`
- `unsigned int count () const`
- `const MRenderItem * find (const MString &name) const`
- `Iterator * getIterator ()`
- `ConstIterator * getConstIterator () const`

**Static Public Member Functions**

- `static const char * className ()`

### MSwatchRenderBase

Module `OpenMayaRender`. Source: [class_m_swatch_render_base.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_swatch_render_base.html).

**Public Member Functions**

- `MSwatchRenderBase (MObject obj, MObject renderObj, int res)`
- `virtual ~MSwatchRenderBase ()`
- `virtual bool doIteration ()`
- `virtual bool renderParallel ()`
- `virtual void cancelParallelRendering ()`
- `MObject swatchNode () const`
- `MObject node () const`
- `int resolution () const`
- `MImage & image ()`
- `void setRenderQuality (int)`
- `int renderQuality () const`

**Static Public Member Functions**

- `static void cancelCurrentSwatchRender ()`
- `static void enableSwatchRender (bool enable)`

**Protected Member Functions**

- `void finishParallelRender ()`

### MSwatchRenderRegister

Module `OpenMayaRender`. Source: [class_m_swatch_render_register.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_swatch_render_register.html).

**Static Public Member Functions**

- `static MStatus registerSwatchRender (MString swatchGenName, MSwatchRenderCreatorFnPtr fnPtr)`
- `static MStatus unregisterSwatchRender (MString swatchGenName)`

### MTargetBlendDesc

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_target_blend_desc.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_target_blend_desc.html).

**Public Member Functions**

- `MTargetBlendDesc ()`
- `~MTargetBlendDesc ()`
- `void setDefaults ()`
- `MTargetBlendDesc (const MTargetBlendDesc &rDesc)`
- `MTargetBlendDesc & operator= (const MTargetBlendDesc &rDesc)`

**Static Public Member Functions**

- `static const char * className ()`

**Public Attributes**

- `bool blendEnable`
- `MBlendState::BlendOption sourceBlend`
- `MBlendState::BlendOption destinationBlend`
- `MBlendState::BlendOperation blendOperation`
- `MBlendState::BlendOption alphaSourceBlend`
- `MBlendState::BlendOption alphaDestinationBlend`
- `MBlendState::BlendOperation alphaBlendOperation`
- `MBlendState::ChannelMask targetWriteMask`

### MTexture

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture.html).

**Public Member Functions**

- `void * resourceHandle () const`
- `const MString & name () const`
- `void textureDescription (MTextureDescription &desc) const`
- `bool hasAlpha () const`
- `MStatus setHasAlpha (bool value)`
- `bool hasZeroAlpha () const`
- `MStatus setHasZeroAlpha (bool value)`
- `bool hasTransparentAlpha () const`
- `MStatus setHasTransparentAlpha (bool value)`
- `unsigned int bytesPerPixel () const`
- `void * rawData (int &rowPitch, size_t &slicePitch) const`
- `MStatus update (const void *pixelData, bool generateMipMaps, unsigned int rowPitch=0, MHWRender::MTextureUpdateRegion *region=NULL)`
- `MStatus update (const MImage &image, bool generateMipMaps)`
- `MStatus update (const MObject &textureNode)`

**Static Public Member Functions**

- `static const char * className ()`
- `static void freeRawData (void *)`

### MTextureArguments

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture_arguments.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture_arguments.html).

**Public Member Functions**

- `MTextureArguments (const MString &filePath, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`
- `void setContextNodeFullName (const MString &contextNodeFullName)`
- `void setFileTextureNode (MObject node)`

### MTextureManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_texture_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_texture_manager.html).

**Public Member Functions**

- `MStatus addImagePath (const MString &path)`
- `MStatus imagePaths (MStringArray &paths) const`
- `MTexture * acquireTexture (const MString &filePath, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`
- `MTexture * acquireTexture (const MTextureArguments &args)`
- `MTexture * acquireTexture (const MString &filePath, const MString &contextNodeFullName, int mipmapLevels=0, bool useExposureControl=true, const MString &layerName=MString(), int alphaChannelIdx=-1)`
- `MTexture * acquireTexture (const MString &textureName, const MPlug &plug, int width, int height, bool generateMipMaps=true)`
- `MTexture * acquireTexture (const MObject &textureNode, bool allowBackgroundLoad=false)`
- `MTexture * acquireTexture (const MString &textureName, const MHWRender::MTextureDescription &textureDesc, const void *pixelData, bool generateMipMaps=true)`
- `MTexture * acquireTiledTexture (const MString textureName, const MStringArray &tilePaths, const MFloatArray &tilePositions, const MColor &undefinedColor, unsigned int width, unsigned int height, MStringArray &failedTilePaths, MFloatArray &uvScaleOffset)`
- `MTexture * acquireDepthTexture (const MString &textureName, const MImage &image, bool generateMipMaps, const MDepthNormalizationDescription *normalizationDesc)`
- `MTexture * acquireDepthTexture (const MString &textureName, float *pixelData, unsigned int width, unsigned int height, bool generateMipMaps, const MDepthNormalizationDescription *normalizationDesc)`
- `void releaseTexture (MTexture *texture) const`
- `MStatus saveTexture (MTexture *texture, const MString &filePath)`
- `MTexture * findTexture (const MString &textureName)`

**Static Public Member Functions**

- `static const char * className ()`

### MUIDrawManager

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_u_i_draw_manager.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_u_i_draw_manager.html).

**Public Types**

- `enum FontSize { kDefaultFontSize = 12, kSmallFontSize = 9 }`
- `enum TextAlignment { kLeft, kCenter, kRight }`
- `enum TextIncline { kInclineNormal = 0, kInclineItalic = 1, kInclineOblique = 2 }`
- `enum TextWeight { kWeightLight = 25, kWeightNormal = 50, kWeightDemiBold = 63, kWeightBold = 75, kWeightBlack = 87 }`
- `enum TextStretch { kStretchUltraCondensed = 50, kStretchExtraCondensed = 62, kStretchCondensed = 75, kStretchSemiCondensed = 87, kStretchUnstretched = 100, kStretchSemiExpanded = 112, kStretchExpanded = 125, kStretchExtraExpanded = 150, kStretchUltraExpanded = 200 }`
- `enum TextLine { kLineNone, kLineOverline, kLineUnderline, kLineStrikeoutLine }`
- `enum LineStyle { kSolid, kShortDotted, kShortDashed, kDashed, kDotted }`
- `enum PaintStyle { kFlat, kStippled, kShaded }`
- `enum Primitive { kPoints, kLines, kLineStrip, kClosedLine, kTriangles, kTriStrip }`
- `enum Selectability { kNonSelectable, kSelectable, kAutomatic }`

**Public Member Functions**

- `void beginDrawable (Selectability selectability=kAutomatic, unsigned int selectionName=0)`
- `void beginDrawable (unsigned int name, bool nameIsPickable)`
- `void endDrawable ()`
- `void beginDrawInXray ()`
- `void endDrawInXray ()`
- `void setColor (const MColor &color)`
- `void setColorIndex (const short index)`
- `void setPointSize (float value)`
- `void setLineWidth (float value)`
- `void setLineStyle (LineStyle style)`
- `void setLineStyle (unsigned int factor, unsigned short pattern)`
- `void setPaintStyle (PaintStyle style)`
- `unsigned int depthPriority () const`
- `void setDepthPriority (unsigned int priority)`
- `void line (const MPoint &startPoint, const MPoint &endPoint)`
- `void line2d (const MPoint &startPoint, const MPoint &endPoint)`
- `MStatus lineList (const MPointArray &points, bool draw2D)`
- `MStatus lineStrip (const MPointArray &points, bool draw2D)`
- `void point (const MPoint &point)`
- `void point2d (const MPoint &point)`
- `MStatus points (const MPointArray &points, bool draw2D)`
- `void rect (const MPoint &center, const MVector &up, const MVector &normal, double scaleX, double scaleY, bool filled=false)`
- `void rect2d (const MPoint &center, const MVector &up, double scaleX, double scaleY, bool filled=false)`
- `void sphere (const MPoint &center, double radius, bool filled=false)`
- `void sphere (const MPoint &center, double radius, int subdivisionsAxis, int subdivisionsHeight, bool filled=false)`
- `void circle (const MPoint &center, const MVector &normal, double radius, bool filled=false)`
- `void circle2d (const MPoint &center, double radius, bool filled=false)`
- `void circle (const MPoint &center, const MVector &normal, double radius, int numSubdivision, bool filled)`
- `void circle2d (const MPoint &center, double radius, int numSubdivision, bool filled)`
- `void arc (const MPoint &center, const MVector &start, const MVector &end, const MVector &normal, double radius, bool filled=false)`
- `void arc2d (const MPoint &center, const MVector &start, const MVector &end, double radius, bool filled=false)`
- `void arc (const MPoint &center, const MVector &start, const MVector &end, const MVector &normal, double radius, int numSubdivisions, bool filled)`
- `void arc2d (const MPoint &center, const MVector &start, const MVector &end, double radius, int numSubdivisions, bool filled)`
- `void mesh (Primitive mode, const MPointArray &position, const MVectorArray *normal=NULL, const MColorArray *color=NULL, const MUintArray *index=NULL, const MPointArray *texcoord=NULL)`
- `void mesh (Primitive mode, const MFloatPointArray &position, const MFloatVectorArray *normal=NULL, const MColorArray *color=NULL, const MUintArray *index=NULL, const MFloatPointArray *texcoord=NULL)`
- `void mesh2d (Primitive mode, const MPointArray &position, const MColorArray *color=NULL, const MUintArray *index=NULL, const MPointArray *texcoord=NULL)`
- `void cone (const MPoint &base, const MVector &direction, double radius, double height, bool filled=false)`
- `void cone (const MPoint &base, const MVector &direction, double radius, double height, int subdivisionsCap, bool filled)`
- `void cylinder (const MPoint &center, const MVector &up, double radius, double height, int subdivisionsAxis, bool filled=false)`
- `void capsule (const MPoint &center, const MVector &up, double radius, double height, int subdivisionsAxis, int subdivisionsHeight, bool filled=false)`
- `void box (const MPoint &center, const MVector &up, const MVector &right, double scaleX=1.0, double scaleY=1.0, double scaleZ=1.0, bool filled=false)`
- `void setFontIncline (const int fontIncline)`
- `void setFontWeight (const int fontWeight)`
- `void setFontStretch (const int fontStretch)`
- `void setFontLine (const int fontLine)`
- `void setFontSize (const unsigned int fontSize)`
- `void setFontName (const MString &faceName)`
- `void text (const MPoint &position, const MString &text, TextAlignment alignment=kLeft, const int *backgroundSize=NULL, const MColor *backgroundColor=NULL, bool dynamic=false)`
- `void text2d (const MPoint &position, const MString &text, TextAlignment alignment=kLeft, const int *backgroundSize=NULL, const MColor *backgroundColor=NULL, bool dynamic=false)`
- `void setTexture (MHWRender::MTexture *texture)`
- `MStatus setTextureSampler (MHWRender::MSamplerState::TextureFilter filter, MHWRender::MSamplerState::TextureAddress address)`
- `MStatus setTextureMask (MHWRender::MBlendState::ChannelMask mask)`
- `MStatus icon (const MPoint &position, const MString &iconName, float scale)`

**Static Public Member Functions**

- `static unsigned int getFontList (MStringArray &list, bool usingOGSPainter=true)`
- `static unsigned int getIconNames (MStringArray &names, bool usingOGSPainter=true)`

### MUniformParameter

Module `OpenMayaRender`. Source: [class_m_uniform_parameter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uniform_parameter.html).

**Public Types**

- `enum DataType { kTypeUnknown, kTypeBool, kTypeInt, kTypeFloat, kType1DTexture, kType2DTexture, kType3DTexture, kTypeCubeTexture, kTypeEnvTexture, kTypeString, kTypeEnum }`
- `enum DataSemantic { kSemanticUnknown, kSemanticObjectDir, kSemanticWorldDir, kSemanticViewDir, kSemanticProjectionDir, kSemanticObjectPos, kSemanticWorldPos, kSemanticViewPos, kSemanticProjectionPos, kSemanticColor, kSemanticNormal, kSemanticBump, kSemanticEnvironment, kSemanticWorldMatrix, kSemanticWorldInverseMatrix, kSemanticWorldInverseTransposeMatrix, kSemanticViewMatrix, kSemanticViewInverseMatrix, kSemanticViewInverseTransposeMatrix, kSemanticProjectionMatrix, kSemanticProjectionInverseMatrix, kSemanticProjectionInverseTransposeMatrix, kSemanticWorldViewMatrix, kSemanticWorldViewInverseMatrix, kSemanticWorldViewInverseTransposeMatrix, kSemanticWorldViewProjectionMatrix, kSemanticWorldViewProjectionInverseMatrix, kSemanticWorldViewProjectionInverseTransposeMatrix, kSemanticColorTexture, kSemanticNormalTexture, kSemanticBumpTexture, kSemanticNormalizationTexture, kSemanticTranspDepthTexture, kSemanticOpaqueDepthTexture, kSemanticTime, kSemanticWorldTransposeMatrix, kSemanticViewTransposeMatrix, kSemanticProjectionTransposeMatrix, kSemanticWorldViewTransposeMatrix, kSemanticWorldViewProjectionTransposeMatrix, kSemanticViewProjectionMatrix, kSemanticViewProjectionInverseMatrix, kSemanticViewProjectionTransposeMatrix, kSemanticViewProjectionInverseTransposeMatrix, kSemanticLocalViewer, kSemanticViewportPixelSize, kSemanticBackgroundColor, kSemanticFrameNumber, kSemanticNearClipPlane, kSemanticFarClipPlane, kSemanticHWSPrimitiveBase, kSemanticHWSPrimitiveCountPerInstance, kSemanticHWSObjectLevel, kSemanticHWSFaceLevel, kSemanticHWSEdgeLevel, kSemanticHWSVertexLevel, kSemanticHWSOccluder, kSemanticHWSFrontCCW, kSemanticHWSInstancedDraw, kSemanticHWSHighlighting }`

**Public Member Functions**

- `MUniformParameter ()`
- `MUniformParameter (const MString &name, DataType type, DataSemantic semantic, unsigned int numRows=1, unsigned int numColumns=1, void *userData=NULL)`
- `MUniformParameter (const MUniformParameter &src)`
- `~MUniformParameter ()`
- `const MUniformParameter & operator= (const MUniformParameter &other)`
- `MString name () const`
- `MUniformParameter::DataType type () const`
- `MUniformParameter::DataSemantic semantic () const`
- `unsigned int numRows () const`
- `unsigned int numColumns () const`
- `unsigned int numElements () const`
- `void * userData () const`
- `void setRangeMin (double rangeMin) const`
- `void setRangeMax (double rangeMax) const`
- `void setSoftRangeMin (double softRangeMin) const`
- `void setSoftRangeMax (double softRangeMax) const`
- `void setUIHidden (bool hiddenState) const`
- `bool UIHidden () const`
- `void setEnumFieldNames (const MString &fieldNames)`
- `void setKeyable (bool keyable) const`
- `bool keyable () const`
- `bool isATexture () const`
- `bool hasChanged (const MGeometryList &iterator) const`
- `bool hasChanged (const MHWRender::MDrawContext &context) const`
- `const float * getAsFloatArray (const MGeometryList &iterator) const`
- `const float * getAsFloatArray (const MHWRender::MDrawContext &context) const`
- `void setAsFloatArray (const float *value, unsigned int maxElements) const`
- `float getAsFloat (const MGeometryList &iterator) const`
- `float getAsFloat (const MHWRender::MDrawContext &context) const`
- `void setAsFloat (float value) const`
- `MString getAsString (const MGeometryList &iterator) const`
- `MString getAsString (const MHWRender::MDrawContext &context) const`
- `void setAsString (const MString &value) const`
- `bool getAsBool (const MGeometryList &iterator) const`
- `bool getAsBool (const MHWRender::MDrawContext &context) const`
- `void setAsBool (bool value) const`
- `int getAsInt (const MGeometryList &iterator) const`
- `int getAsInt (const MHWRender::MDrawContext &context) const`
- `void setAsInt (int value) const`
- `void setUINiceName (const MString &name)`
- `MPlug getPlug () const`
- `MPlug getSource () const`
- `void setDirty ()`

### MUniformParameterList

Module `OpenMayaRender`. Source: [class_m_uniform_parameter_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_uniform_parameter_list.html).

**Public Member Functions**

- `MUniformParameterList ()`
- `~MUniformParameterList ()`
- `MUniformParameterList (const MUniformParameterList &src)`
- `void append (const MUniformParameter &element)`
- `int length () const`
- `void setLength (int n)`
- `MUniformParameter getElement (int n) const`
- `void setElement (int n, const MUniformParameter &p)`
- `const MUniformParameterList & operator= (const MUniformParameterList &other)`

### MUserData

Module `OpenMayaRender`. Source: [class_m_user_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_user_data.html).

**Public Member Functions**

- `MUserData (bool deleteAfterUse)`
- `MUserData ()`
- `virtual ~MUserData ()`
- `bool deleteAfterUse ()`
- `void setDeleteAfterUse (bool shouldDelete)`

**Static Public Member Functions**

- `static const char * className ()`

### MUserRenderOperation

Module `OpenMayaRender`. Inherits `MRenderOperation`. Source: [class_m_h_w_render_1_1_m_user_render_operation.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_user_render_operation.html).

**Public Member Functions**

- `MUserRenderOperation (const MString &name)`
- `~MUserRenderOperation () override`
- `virtual const MCameraOverride * cameraOverride ()`
- `virtual MStatus execute (const MDrawContext &drawContext)=0`
- `virtual bool hasUIDrawables () const`
- `virtual void addUIDrawables (MUIDrawManager &drawManager, const MFrameContext &frameContext)`
- `virtual bool requiresLightData () const`

### MVaryingParameter

Module `OpenMayaRender`. Source: [class_m_varying_parameter.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_varying_parameter.html).

**Public Types**

- `enum MVaryingParameterType { kInvalidParameter = -1, kStructure = 0, kFloat, kDouble, kChar, kUnsignedChar, kInt16, kUnsignedInt16, kInt32, kUnsignedInt32 }`
- `enum MVaryingParameterSemantic { kNoSemantic = 0, kPosition, kNormal, kTexCoord, kColor, kWeight, kTangent = kWeight + 2, kBinormal }`

**Public Member Functions**

- `MVaryingParameter ()`
- `MVaryingParameter (const MString &name, MVaryingParameterType type, int minDimension=1, int maxDimension=1, MVaryingParameterSemantic semantic=kNoSemantic, bool invertTexCoords=false, const MString &semanticName="")`
- `MVaryingParameter (const MString &name, MVaryingParameterType type, int minDimension, int maxDimension, int dimension, MVaryingParameterSemantic semantic=kNoSemantic, const MString &destinationSet="", bool invertTexCoords=false, const MString &semanticName="")`
- `MVaryingParameter (const MVaryingParameter &src)`
- `~MVaryingParameter ()`
- `const MVaryingParameter & operator= (const MVaryingParameter &other)`
- `const MString name () const`
- `MVaryingParameter::MVaryingParameterType type () const`
- `MVaryingParameter::MVaryingParameterSemantic semantic () const`
- `const MString semanticName () const`
- `MString destinationSet () const`
- `int dimension () const`
- `MStatus getBuffer (MGeometryLegacy &geometry, const void *&data, unsigned int &elements, unsigned int &count) const`
- `MVaryingParameter::MVaryingParameterSemantic getSourceType () const`
- `MString getSourceSetName () const`
- `MStatus setSource (MVaryingParameter::MVaryingParameterSemantic type, const MString &setName)`
- `unsigned int getElementSize () const`
- `unsigned int getMaximumStride () const`
- `MStatus addElement (const MVaryingParameter &child)`
- `unsigned int numElements () const`
- `MVaryingParameter getElement (unsigned int i) const`
- `MStatus removeElements ()`
- `unsigned int getUpdateId () const`

### MVaryingParameterList

Module `OpenMayaRender`. Source: [class_m_varying_parameter_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_varying_parameter_list.html).

**Public Member Functions**

- `MVaryingParameterList ()`
- `~MVaryingParameterList ()`
- `MVaryingParameterList (const MVaryingParameterList &src)`
- `void append (const MVaryingParameter &element)`
- `int length () const`
- `void setLength (int n)`
- `MVaryingParameter getElement (int n) const`
- `void setElement (int n, const MVaryingParameter &p)`
- `const MVaryingParameterList & operator= (const MVaryingParameterList &other)`

### MVertexBuffer

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer.html).

**Public Member Functions**

- `MVertexBuffer (const MVertexBufferDescriptor &descriptor, bool softwareStaged=true)`
- `MVertexBuffer (const MVertexBuffer &)`
- `~MVertexBuffer ()`
- `const MVertexBufferDescriptor & descriptor () const`
- `void * resourceHandle () const`
- `void resourceHandle (void *handle, unsigned int size)`
- `bool hasCustomResourceHandle () const`
- `void lockResourceHandle ()`
- `void unlockResourceHandle ()`
- `void * acquire (unsigned int size, bool writeOnly)`
- `void commit (void *buffer)`
- `void * map ()`
- `void unmap ()`
- `MStatus update (const void *buffer, unsigned int destOffset, unsigned int numVerts, bool truncateIfSmaller)`
- `void unload ()`
- `unsigned int vertexCount () const`
- `void * acquire (unsigned int size)`
- `void * getVertexBuffer () const`

**Static Public Member Functions**

- `static const char * className ()`

### MVertexBufferArray

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_array.html).

**Public Member Functions**

- `MVertexBufferArray ()`
- `~MVertexBufferArray ()`
- `unsigned int count () const`
- `MVertexBuffer * getBuffer (const MString &name) const`
- `MVertexBuffer * getBuffer (unsigned int index) const`
- `MString getName (unsigned int index) const`
- `MStatus addBuffer (const MString &name, MVertexBuffer *buffer)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MVertexBufferDescriptor

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_descriptor.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_descriptor.html).

**Public Member Functions**

- `MVertexBufferDescriptor ()`
- `MVertexBufferDescriptor (const MString &name, MGeometry::Semantic semantic, MGeometry::DataType dataType, int dimension)`
- `MVertexBufferDescriptor (const MString &name, MGeometry::Semantic semantic, const MString &semanticName, MGeometry::DataType dataType, int dimension)`
- `~MVertexBufferDescriptor ()`
- `MVertexBufferDescriptor (const MVertexBufferDescriptor &other)`
- `MVertexBufferDescriptor & operator= (const MVertexBufferDescriptor &other)`
- `MString name () const`
- `void setName (const MString &n)`
- `MGeometry::Semantic semantic () const`
- `void setSemantic (MGeometry::Semantic s)`
- `MString semanticName () const`
- `void setSemanticName (const MString &n)`
- `MGeometry::DataType dataType () const`
- `void setDataType (MGeometry::DataType d)`
- `unsigned int dataTypeSize () const`
- `int dimension () const`
- `void setDimension (int d)`
- `int offset () const`
- `void setOffset (int o)`
- `int stride () const`

**Static Public Member Functions**

- `static const char * className ()`

### MVertexBufferDescriptorList

Module `OpenMayaRender`. Source: [class_m_h_w_render_1_1_m_vertex_buffer_descriptor_list.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_h_w_render_1_1_m_vertex_buffer_descriptor_list.html).

**Public Member Functions**

- `MVertexBufferDescriptorList ()`
- `~MVertexBufferDescriptorList ()`
- `int length () const`
- `bool getDescriptor (int index, MVertexBufferDescriptor &desc) const`
- `bool append (const MVertexBufferDescriptor &desc)`
- `bool removeAt (int index)`
- `void clear ()`

**Static Public Member Functions**

- `static const char * className ()`

### MViewportRenderer

Module `OpenMayaRender`. Source: [class_m_viewport_renderer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_viewport_renderer.html).

**Public Types**

- `enum RenderingAPI { kOpenGL, kDirect3D, kSoftware }`
- `enum RenderingOverride { kNoOverride, kOverrideAllDrawing, kOverrideThenStandard, kOverrideThenUI }`
- `enum RenderingExclusion`

**Public Member Functions**

- `MViewportRenderer (const MString &name)`
- `virtual ~MViewportRenderer ()`
- `virtual MStatus initialize ()=0`
- `virtual MStatus uninitialize ()=0`
- `virtual MStatus render (const MRenderingInfo &info)=0`
- `virtual bool nativelySupports (MViewportRenderer::RenderingAPI api, float version)=0`
- `virtual bool override (MViewportRenderer::RenderingOverride override)=0`
- `virtual unsigned int overrideThenStandardExclusion () const`
- `MStatus registerRenderer () const`
- `MStatus deregisterRenderer () const`
- `const MString & name () const`
- `const MString & UIname () const`
- `void setUIName (const MString &name)`
- `MViewportRenderer::RenderingOverride renderingOverride () const`
- `void setRenderingOverride (RenderingOverride override)`

**Friends**

- `class MHardwareRenderer`
