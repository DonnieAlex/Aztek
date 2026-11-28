# OpenMayaFX

Class names, bases and signatures of the module, checked against the reference's 2022 C++ API reference (https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_f_x.html). Descriptions are not kept here; follow the source link on a class for them.

Classes in this module: 33.

## Classes

### MDynSweptLine

Module `OpenMayaFX`. Source: [class_m_dyn_swept_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dyn_swept_line.html).

**Public Member Functions**

- `MDynSweptLine ()`
- `~MDynSweptLine ()`
- `MVector vertex (int vertexId, double t=1.0)`
- `MVector normal (double x, double y, double z, double t=1.0)`
- `MVector tangent (double t=1.0)`
- `double length (double t=1.0)`

**Static Public Member Functions**

- `static const char * className ()`

### MDynSweptTriangle

Module `OpenMayaFX`. Source: [class_m_dyn_swept_triangle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dyn_swept_triangle.html).

**Public Member Functions**

- `MDynSweptTriangle ()`
- `~MDynSweptTriangle ()`
- `MVector vertex (int vertexId, double t=1.0)`
- `MVector normal (double t=1.0)`
- `MVector normalToPoint (double x, double y, double z, double t=1.0)`
- `MVector uvPoint (int vertexId)`
- `double area ()`

**Static Public Member Functions**

- `static const char * className ()`

### MDynamicsUtil

Module `OpenMayaFX`. Source: [class_m_dynamics_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dynamics_util.html).

**Static Public Member Functions**

- `static bool hasValidDynamics2dTexture (const MObject &node, const MObject &texAttr, MStatus *status=NULL)`
- `static MStatus evalDynamics2dTexture (const MObject &node, const MObject &texAttr, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`
- `static bool inRunup ()`
- `static bool runupIfRequired ()`
- `static bool addNodeTypeToRunup (const MString &nodeTypeName)`
- `static bool removeNodeTypeFromRunup (const MString &nodeTypeName)`

### MFnAirField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_air_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_air_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnAirField ()`
- `MFnAirField ()`
- `MFnAirField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnAirField (const MDagPath &object, MStatus *ret=NULL)`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `MStatus setDirection (const MVector &airDirection)`
- `double speed (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpeed (double value)`
- `double inheritVelocity (MStatus *ReturnStatus=NULL) const`
- `MStatus setInheritVelocity (double velocity)`
- `bool inheritRotation (MStatus *ReturnStatus=NULL) const`
- `MStatus setInheritRotation (bool enable)`
- `bool componentOnly (MStatus *ReturnStatus=NULL) const`
- `MStatus setComponentOnly (bool enable)`
- `double spread (MStatus *ReturnStatus=NULL) const`
- `MStatus setSpread (double value)`
- `bool enableSpread (MStatus *ReturnStatus=NULL) const`
- `MStatus setEnableSpread (bool enable)`
- `MFnAirField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDragField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_drag_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_drag_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDragField ()`
- `MFnDragField ()`
- `MFnDragField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnDragField (const MDagPath &object, MStatus *ret=NULL)`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `MStatus setDirection (const MVector &dragDirection)`
- `bool useDirection (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseDirection (bool enable)`
- `MFnDragField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnDynSweptGeometryData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_dyn_swept_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_dyn_swept_geometry_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnDynSweptGeometryData ()`
- `MFnDynSweptGeometryData ()`
- `MFnDynSweptGeometryData (MObject &object, MStatus *ReturnStatus=NULL)`
- `int lineCount (MStatus *ReturnStatus=NULL) const`
- `int triangleCount (MStatus *ReturnStatus=NULL) const`
- `MDynSweptLine sweptLine (int index, MStatus *ReturnStatus=NULL) const`
- `MDynSweptTriangle sweptTriangle (int index, MStatus *ReturnStatus=NULL) const`
- `MObject create (MStatus *ReturnStatus=NULL)`
- `MFnDynSweptGeometryData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnField

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnField ()`
- `MFnField ()`
- `MFnField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnField (const MDagPath &object, MStatus *ret=NULL)`
- `MStatus getForceAtPoint (const MPointArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`
- `MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`
- `double magnitude (MStatus *ReturnStatus=NULL) const`
- `MStatus setMagnitude (double mag)`
- `double attenuation (MStatus *ReturnStatus=NULL) const`
- `MStatus setAttenuation (double atten)`
- `double maxDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus setMaxDistance (double maxDist)`
- `bool perVertex (MStatus *ReturnStatus=NULL) const`
- `MStatus setPerVertex (bool enable)`
- `bool useMaxDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus setUseMaxDistance (bool enable)`
- `double falloffCurve (const double param, MStatus *ReturnStatus=NULL)`
- `bool isFalloffCurveConstantOne (MStatus *ReturnStatus=NULL)`
- `MFnField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnFluid

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_fluid.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_fluid.html).

**Public Types**

- `enum FluidMethod { kZero, kStaticGrid, kDynamicGrid, kGradient }`
- `enum FluidGradient { kConstant, kXGradient, kYGradient, kZGradient, kNegXGradient, kNegYGradient, kNegZGradient, kCenterGradient }`
- `enum FalloffMethod { kNoFalloffGrid, kStaticFalloffGrid }`
- `enum ColorMethod { kUseShadingColor, kStaticColorGrid, kDynamicColorGrid }`
- `enum CoordinateMethod { kFixed, kGrid }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnFluid ()`
- `MFnFluid ()`
- `MFnFluid (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnFluid (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create3D (unsigned int Xres, unsigned int Yres, unsigned int Zres, double Xdim, double Ydim, double Zdim, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MObject create2D (unsigned int Xres, unsigned int Yres, double Xdim, double Ydim, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`
- `MStatus getResolution (unsigned int &Xres, unsigned int &Yres, unsigned int &Zres) const`
- `MStatus getResolution (unsigned int &Xres, unsigned int &Yres) const`
- `MStatus getDimensions (double &Xdim, double &Ydim, double &Zdim) const`
- `MStatus setSize (unsigned int Xres, unsigned int Yres, unsigned int Zres, double Xdim, double Ydim, double Zdim, bool resample)`
- `MStatus setSize (unsigned int Xres, unsigned int Yres, double Xdim, double Ydim, bool resample)`
- `unsigned int gridSize (MStatus *ReturnStatus=NULL)`
- `MStatus velocityGridSizes (int &xsize, int &ysize, int &zsize)`
- `float * falloff (MStatus *ReturnStatus=NULL)`
- `float * density (MStatus *ReturnStatus=NULL)`
- `MStatus getVelocity (float *&Xvel, float *&Yvel, float *&Zvel)`
- `float * pressure (MStatus *ReturnStatus=NULL)`
- `float * temperature (MStatus *ReturnStatus=NULL)`
- `float * fuel (MStatus *ReturnStatus=NULL)`
- `MStatus getCoordinates (float *&u, float *&v, float *&w)`
- `MStatus getColors (float *&r, float *&g, float *&b)`
- `MStatus setFalloffMode (FalloffMethod mode)`
- `MStatus getFalloffMode (FalloffMethod &mode)`
- `MStatus setDensityMode (FluidMethod mode, FluidGradient gradient)`
- `MStatus getDensityMode (FluidMethod &mode, FluidGradient &gradient)`
- `MStatus setVelocityMode (FluidMethod mode, FluidGradient gradient)`
- `MStatus getVelocityMode (FluidMethod &mode, FluidGradient &gradient)`
- `MStatus setTemperatureMode (FluidMethod mode, FluidGradient gradient)`
- `MStatus getTemperatureMode (FluidMethod &mode, FluidGradient &gradient)`
- `MStatus setFuelMode (FluidMethod mode, FluidGradient gradient)`
- `MStatus getFuelMode (FluidMethod &mode, FluidGradient &gradient)`
- `MStatus setCoordinateMode (CoordinateMethod mode)`
- `MStatus getCoordinateMode (CoordinateMethod &mode)`
- `MStatus setColorMode (ColorMethod mode)`
- `MStatus getColorMode (ColorMethod &mode)`
- `MStatus getForceAtPoint (const MPointArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`
- `MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`
- `bool toGridIndex (const MPoint &objectSpacePoint, int3 &gridCoords, MStatus *status=NULL)`
- `MStatus voxelCenterPosition (int xi, int yi, int zi, MPoint &objectSpacePoint)`
- `MStatus updateGrid ()`
- `void emitIntoArrays (float val, int x, int y, int z, float density, float heat, float fuel, bool doColor, const MColor &emitColor)`
- `int index (int xi, int yi)`
- `int index (int xi, int yi, int zi)`
- `void index (int ai, int &xi, int &yi, int &zi)`
- `int index (int xi, int yi, int zi, int xres, int yres, int zres)`
- `void index (int ai, int xres, int yres, int zres, int &xi, int &yi, int &zi)`
- `bool isAutoResize ()`
- `bool isResizeToEmitter ()`
- `MStatus expandToInclude (const MPoint &min, const MPoint &max)`
- `MFnFluid (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnGravityField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_gravity_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_gravity_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnGravityField ()`
- `MFnGravityField ()`
- `MFnGravityField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnGravityField (const MDagPath &object, MStatus *ret=NULL)`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `MStatus setDirection (const MVector &gravityDirection)`
- `MFnGravityField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnInstancer

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_instancer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_instancer.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnInstancer ()`
- `MFnInstancer ()`
- `MFnInstancer (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnInstancer (const MDagPath &object, MStatus *ret=NULL)`
- `int particleCount () const`
- `int instancesForParticle (int p, MDagPathArray &paths, MMatrix &instancerMatrix, MStatus *ReturnStatus=NULL) const`
- `MStatus allInstances (MDagPathArray &paths, MMatrixArray &matrices, MIntArray &particlePathStartIndices, MIntArray &pathIndices) const`
- `MFnInstancer (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNIdData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_n_id_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_n_id_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNIdData ()`
- `MFnNIdData ()`
- `MFnNIdData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create () const`
- `MStatus getObjectPtr (MnObject *&ptr) const`
- `MnObject * getObjectPtr (MStatus *status=NULL) const`
- `MFnNIdData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNObjectData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_n_object_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_n_object_data.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNObjectData ()`
- `MFnNObjectData ()`
- `MFnNObjectData (MObject &object, MStatus *ReturnStatus=NULL)`
- `MObject create () const`
- `MStatus getObjectPtr (MnCloth *&ptr) const`
- `MStatus getObjectPtr (MnRigid *&ptr) const`
- `MStatus getObjectPtr (MnParticle *&ptr) const`
- `MnCloth * getClothObjectPtr (MStatus *status=NULL) const`
- `MnRigid * getRigidObjectPtr (MStatus *status=NULL) const`
- `MnParticle * getParticleObjectPtr (MStatus *status=NULL) const`
- `MStatus setObjectPtr (MnCloth *ptr)`
- `MStatus setObjectPtr (MnRigid *ptr)`
- `MStatus setObjectPtr (MnParticle *ptr)`
- `MStatus getCollide (bool &) const`
- `MStatus getCached (bool &cached) const`
- `bool isCached (MStatus *status=NULL) const`
- `MStatus setCached (bool cached)`
- `MFnNObjectData (const MObject &object, MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnNewtonField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_newton_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_newton_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnNewtonField ()`
- `MFnNewtonField ()`
- `MFnNewtonField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnNewtonField (const MDagPath &object, MStatus *ret=NULL)`
- `double minDistance (MStatus *ReturnStatus=NULL) const`
- `MStatus setMinDistance (double distance)`
- `MFnNewtonField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnParticleSystem

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_particle_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_particle_system.html).

**Public Types**

- `enum RenderType { kCloud, kTube, kBlobby, kMultiPoint, kMultiStreak, kNumeric, kPoints, kSpheres, kSprites, kStreak }`

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnParticleSystem ()`
- `MFnParticleSystem ()`
- `MFnParticleSystem (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnParticleSystem (const MDagPath &object, MStatus *ret=NULL)`
- `MObject create (MStatus *status=NULL)`
- `MObject create (MObject parent, MStatus *status=NULL)`
- `MStatus emit (const MPoint &position)`
- `MStatus emit (const MPointArray &positionArray)`
- `MStatus emit (const MPoint &position, const MVector &velocity)`
- `MStatus emit (const MPointArray &positionArray, const MVectorArray &velocityArray)`
- `MStatus saveInitialState () const`
- `void evaluateDynamics (MTime &to, bool runupFromStart)`
- `bool isValid () const`
- `MString particleName () const`
- `unsigned int count () const`
- `void setCount (unsigned int) const`
- `RenderType renderType () const`
- `void position (MVectorArray &) const`
- `void velocity (MVectorArray &) const`
- `void acceleration (MVectorArray &) const`
- `void position0 (MVectorArray &) const`
- `void position1 (MVectorArray &) const`
- `void radius (MDoubleArray &) const`
- `void radius0 (MDoubleArray &) const`
- `void radius1 (MDoubleArray &) const`
- `double surfaceShading () const`
- `double threshold () const`
- `bool betterIllum () const`
- `bool flatShaded () const`
- `bool disableCloudAxis () const`
- `double tailSize () const`
- `void particleIds (MIntArray &) const`
- `void age (MDoubleArray &) const`
- `void lifespan (MDoubleArray &) const`
- `void rgb (MVectorArray &) const`
- `void opacity (MDoubleArray &) const`
- `void mass (MDoubleArray &) const`
- `void emission (MVectorArray &) const`
- `bool hasLifespan () const`
- `bool hasRgb () const`
- `bool hasOpacity () const`
- `bool hasEmission () const`
- `bool primaryVisibility () const`
- `bool visibleInReflections () const`
- `bool visibleInRefractions () const`
- `bool castsShadows () const`
- `bool receiveShadows () const`
- `unsigned int getPerParticleAttribute (const MString &attrName, MIntArray &, MStatus *status=NULL) const`
- `unsigned int getPerParticleAttribute (const MString &attrName, MVectorArray &, MStatus *status=NULL) const`
- `unsigned int getPerParticleAttribute (const MString &attrName, MDoubleArray &, MStatus *status=NULL) const`
- `void setPerParticleAttribute (const MString &attrName, MVectorArray &, MStatus *status=NULL) const`
- `void setPerParticleAttribute (const MString &attrName, MDoubleArray &, MStatus *status=NULL) const`
- `bool isPerParticleIntAttribute (const MString &attrName, MStatus *status=NULL) const`
- `bool isPerParticleDoubleAttribute (const MString &attrName, MStatus *status=NULL) const`
- `bool isPerParticleVectorAttribute (const MString &attrName, MStatus *status=NULL) const`
- `bool isDeformedParticleShape (MStatus *status=NULL) const`
- `MObject deformedParticleShape (MStatus *status=NULL) const`
- `MObject originalParticleShape (MStatus *status=NULL) const`
- `MFnParticleSystem (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnPfxGeometry

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_pfx_geometry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_pfx_geometry.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnPfxGeometry ()`
- `MFnPfxGeometry ()`
- `MFnPfxGeometry (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnPfxGeometry (const MDagPath &object, MStatus *ret=NULL)`
- `MStatus getLineData (MRenderLineArray &mainLines, MRenderLineArray &leafLines, MRenderLineArray &flowerLines, bool doLines, bool doTwist, bool doWidth, bool doFlatness, bool doParameter, bool doColor, bool doIncandescence, bool doTransparency, bool worldSpace)`
- `MStatus getBoundingBox (double *min, double *max)`
- `MFnPfxGeometry (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnRadialField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_radial_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_radial_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnRadialField ()`
- `MFnRadialField ()`
- `MFnRadialField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnRadialField (const MDagPath &object, MStatus *ret=NULL)`
- `double radialType (MStatus *ReturnStatus=NULL) const`
- `MStatus setType (double value)`
- `MFnRadialField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnTurbulenceField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_turbulence_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_turbulence_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnTurbulenceField ()`
- `MFnTurbulenceField ()`
- `MFnTurbulenceField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnTurbulenceField (const MDagPath &object, MStatus *ret=NULL)`
- `double frequency (MStatus *ReturnStatus=NULL) const`
- `MStatus setFrequency (double value)`
- `double phase (MStatus *ReturnStatus=NULL) const`
- `MStatus setPhase (double value)`
- `MFnTurbulenceField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnUniformField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_uniform_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_uniform_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnUniformField ()`
- `MFnUniformField ()`
- `MFnUniformField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnUniformField (const MDagPath &object, MStatus *ret=NULL)`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `MStatus setDirection (const MVector &uniformDirection)`
- `MFnUniformField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnVolumeAxisField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_volume_axis_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_volume_axis_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnVolumeAxisField ()`
- `MFnVolumeAxisField ()`
- `MFnVolumeAxisField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnVolumeAxisField (const MDagPath &object, MStatus *ret=NULL)`
- `bool invertAttenuation (MStatus *ReturnStatus=NULL) const`
- `MVector direction (MStatus *ReturnStatus=NULL) const`
- `double speedAlongAxis (MStatus *ReturnStatus=NULL) const`
- `double speedAroundAxis (MStatus *ReturnStatus=NULL) const`
- `double speedAwayFromAxis (MStatus *ReturnStatus=NULL) const`
- `double speedAwayFromCenter (MStatus *ReturnStatus=NULL) const`
- `double directionalSpeed (MStatus *ReturnStatus=NULL) const`
- `double turbulence (MStatus *ReturnStatus=NULL) const`
- `double turbulenceSpeed (MStatus *ReturnStatus=NULL) const`
- `MVector turbulenceFrequency (MStatus *ReturnStatus=NULL) const`
- `MVector turbulenceOffset (MStatus *ReturnStatus=NULL) const`
- `double detailTurbulence (MStatus *ReturnStatus=NULL) const`
- `MStatus setInvertAttenuation (bool enable)`
- `MStatus setDirection (const MVector &direction)`
- `MStatus setSpeedAlongAxis (double value)`
- `MStatus setSpeedAroundAxis (double value)`
- `MStatus setSpeedAwayFromAxis (double value)`
- `MStatus setSpeedAwayFromCenter (double value)`
- `MStatus setDirectionalSpeed (double value)`
- `MStatus setTurbulence (double value)`
- `MStatus setTurbulenceSpeed (double value)`
- `MStatus setTurbulenceFrequency (const MVector &value)`
- `MStatus setTurbulenceOffset (const MVector &value)`
- `MStatus detailTurbulence (double value)`
- `MFnVolumeAxisField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MFnVortexField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_vortex_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_vortex_field.html).

**Public Member Functions**

- `virtual MFn::Type type () const`
- `virtual ~MFnVortexField ()`
- `MFnVortexField ()`
- `MFnVortexField (MObject &object, MStatus *ReturnStatus=NULL)`
- `MFnVortexField (const MDagPath &object, MStatus *ret=NULL)`
- `MVector axis (MStatus *ReturnStatus=NULL) const`
- `MStatus setAxis (const MVector &axisVector)`
- `MFnVortexField (const MObject &object, MStatus *ret=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

### MHairSystem

Module `OpenMayaFX`. Source: [class_m_hair_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hair_system.html).

**Static Public Member Functions**

- `static MStatus registerCollisionSolverCollide (MHairSystemCollisionSolverCollideFnPtr)`
- `static MStatus registerCollisionSolverPreFrame (MHairSystemCollisionSolverPreFrameFnPtr)`
- `static MStatus unregisterCollisionSolverCollide ()`
- `static MStatus unregisterCollisionSolverPreFrame ()`
- `static MObject getCollisionObject (const MObject hairSystem, const int index, MStatus *status)`
- `static MStatus getCollisionObject (const MObject hairSystem, MObjectArray &objects, MIntArray &logicalIndices)`
- `static MObject getFollicle (const MObject hairSystem, const int index, MStatus *status)`
- `static MStatus getFollicle (const MObject hairSystem, MObjectArray &follicles, MIntArray &logicalIndices)`
- `static void setRegisteringCallableScript ()`
- `static bool registeringCallableScript ()`
- `static const char * className ()`

### MPxEmitterNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_emitter_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_emitter_node.html).

**Public Types**

- `enum MEmitterType { kDirectional = 0, kOmni, kSurface, kCurve, kVolume }`

**Public Member Functions**

- `MPxEmitterNode ()`
- `~MPxEmitterNode () override`
- `MPxNode::Type type () const override`
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`
- `MEmitterType getEmitterType (MDataBlock &block)`
- `MObject getOwnerShape ()`
- `MTime getCurrentTime (MDataBlock &block)`
- `MTime getStartTime (int plugIndex, MDataBlock &block)`
- `MTime getDeltaTime (int plugIndex, MDataBlock &block)`
- `int getRandomSeed (int plugIndex, MDataBlock &block)`
- `void getRandomState (int plugIndex, MDataBlock &block)`
- `void setRandomState (int plugIndex, MDataBlock &block)`
- `void resetRandomState (int plugIndex, MDataBlock &block)`
- `double randgen ()`
- `double getRate (MDataBlock &block)`
- `double getMinDistance (MDataBlock &block)`
- `double getMaxDistance (MDataBlock &block)`
- `MPoint getWorldPosition ()`
- `MMatrix getWorldMatrix ()`
- `bool volumePrimitiveBoundingBox (MBoundingBox &box)`
- `bool volumePrimitivePointInside (const MPoint &worldPoint, const MMatrix &emitterWorldMatrix)`
- `double volumePrimitiveDistanceFromAxis (const MPoint &worldPoint, const MMatrix &emitterWorldMatrix)`
- `bool hasValidEmission2dTexture (const MObject &texAttr, MStatus *status=NULL)`
- `MStatus evalEmission2dTexture (const MObject &texAttr, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject mEmitterType`
- `static MObject mRate`
- `static MObject mSpeed`
- `static MObject mDirection`
- `static MObject mDirectionX`
- `static MObject mDirectionY`
- `static MObject mDirectionZ`
- `static MObject mOwnerPosData`
- `static MObject mOwnerVelData`
- `static MObject mOwnerCentroid`
- `static MObject mOwnerCentroidX`
- `static MObject mOwnerCentroidY`
- `static MObject mOwnerCentroidZ`
- `static MObject mSweptGeometry`
- `static MObject mWorldMatrix`
- `static MObject mStartTime`
- `static MObject mDeltaTime`
- `static MObject mIsFull`
- `static MObject mInheritFactor`
- `static MObject mSeed`
- `static MObject mRandState`
- `static MObject mCurrentTime`
- `static MObject mOutput`
- `static MObject mMinDistance`
- `static MObject mMaxDistance`

### MPxFieldNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_field_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_field_node.html).

**Public Member Functions**

- `MPxFieldNode ()`
- `~MPxFieldNode () override`
- `MPxNode::Type type () const override`
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`
- `virtual MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime)`
- `virtual MStatus iconSizeAndOrigin (GLuint &width, GLuint &height, GLuint &xbo, GLuint &ybo)`
- `virtual MStatus iconBitmap (GLubyte *bitmap)`
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`
- `virtual double falloffCurve (const double param, MStatus *ReturnStatus=NULL)`
- `virtual bool isFalloffCurveConstantOne (MStatus *ReturnStatus=NULL)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject mMagnitude`
- `static MObject mAttenuation`
- `static MObject mMaxDistance`
- `static MObject mUseMaxDistance`
- `static MObject mApplyPerVertex`
- `static MObject mInputData`
- `static MObject mInputPositions`
- `static MObject mInputVelocities`
- `static MObject mInputMass`
- `static MObject mDeltaTime`
- `static MObject mInputForce`
- `static MObject mOutputForce`
- `static MObject mOwnerCentroidX`
- `static MObject mOwnerCentroidY`
- `static MObject mOwnerCentroidZ`
- `static MObject mOwnerCentroid`
- `static MObject mOwnerPosData`
- `static MObject mOwnerVelData`
- `static MObject mWorldMatrix`
- `static MObject mInputPPData`
- `static MObject mOwnerPPData`

### MPxFluidEmitterNode

Module `OpenMayaFX`. Inherits `MPxEmitterNode`. Source: [class_m_px_fluid_emitter_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_fluid_emitter_node.html).

**Public Member Functions**

- `MPxFluidEmitterNode ()`
- `~MPxFluidEmitterNode () override`
- `MPxNode::Type type () const override`
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`
- `virtual MStatus fluidEmitter (const MObject &fluidObj, const MMatrix &worldMatrix, int plugIndex)`
- `double fluidDensityEmission (MDataBlock &block)`
- `double fluidHeatEmission (MDataBlock &block)`
- `double fluidFuelEmission (MDataBlock &block)`
- `bool fluidEmitColor (MDataBlock &block)`
- `MColor fluidColor (MDataBlock &block)`
- `double fluidDropoff (MDataBlock &block)`
- `double turbulence (MDataBlock &block)`
- `bool fluidJitter (MDataBlock &block)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject mFluidDensityEmission`
- `static MObject mFluidHeatEmission`
- `static MObject mFluidFuelEmission`
- `static MObject mEmitFluidColor`
- `static MObject mFluidColor`
- `static MObject mFluidColorR`
- `static MObject mFluidColorG`
- `static MObject mFluidColorB`
- `static MObject mFluidDropoff`
- `static MObject mTurbulence`
- `static MObject mFluidJitter`
- `static MObject mEmissionFunction`

### MPxParticleAttributeMapperNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_particle_attribute_mapper_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_particle_attribute_mapper_node.html).

**Public Member Functions**

- `MPxParticleAttributeMapperNode ()`
- `~MPxParticleAttributeMapperNode () override`
- `MPxNode::Type type () const override`
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject computeNode`
- `static MObject uCoordPP`
- `static MObject vCoordPP`
- `static MObject outMinValue`
- `static MObject outMaxValue`
- `static MObject computeNodeColor`
- `static MObject computeNodeColorR`
- `static MObject computeNodeColorG`
- `static MObject computeNodeColorB`
- `static MObject time`
- `static MObject outColorPP`
- `static MObject outValuePP`

### MPxSpringNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_spring_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_spring_node.html).

**Public Member Functions**

- `MPxSpringNode ()`
- `~MPxSpringNode () override`
- `MPxNode::Type type () const override`
- `virtual MStatus applySpringLaw (double stiffness, double damping, double restLength, double endMass1, double endMass2, const MVector &endP1, const MVector &endP2, const MVector &endV1, const MVector &endV2, MVector &forceV1, MVector &forceV2)`

**Static Public Member Functions**

- `static const char * className ()`

**Static Public Attributes**

- `static MObject mEnd1Weight`
- `static MObject mEnd2Weight`
- `static MObject mDeltaTime`

### MRenderLine

Module `OpenMayaFX`. Source: [class_m_render_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_line.html).

**Public Member Functions**

- `MRenderLine ()`
- `~MRenderLine ()`
- `MRenderLine & operator= (const MRenderLine &other)`
- `MVectorArray getLine () const`
- `MVectorArray getTwist () const`
- `MDoubleArray getWidth () const`
- `MDoubleArray getFlatness () const`
- `MDoubleArray getParameter () const`
- `MVectorArray getColor () const`
- `MVectorArray getIncandescence () const`
- `MVectorArray getTransparency () const`

**Static Public Member Functions**

- `static const char * className ()`

### MRenderLineArray

Module `OpenMayaFX`. Source: [class_m_render_line_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_line_array.html).

**Public Member Functions**

- `MRenderLineArray ()`
- `~MRenderLineArray ()`
- `MRenderLineArray & operator= (const MRenderLineArray &other)`
- `void deleteArray ()`
- `int length () const`
- `MRenderLine renderLine (unsigned int index, MStatus *status) const`

**Static Public Member Functions**

- `static const char * className ()`

### MnCloth

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_cloth.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_cloth.html).

**Public Member Functions**

- `MnCloth ()`
- `~MnCloth () override`
- `MStatus createNCloth ()`
- `MStatus setTopology (const int numFaces, int *numVertsPerFace, int *faces, const int numEdges, int *edges)`
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`
- `MStatus setVelocities (const MFloatPointArray &velocities)`
- `MStatus setBendRestAngleFromPositions (const MFloatPointArray &positions)`
- `MStatus setLinksRestLengthFromPositions (const MFloatPointArray &positions)`
- `MStatus setInputMeshAttractPositions (const MFloatPointArray &positions)`
- `MStatus setInputMeshAttractDamping (float damping)`
- `MStatus setInputMeshAttractAndRigidStrength (float *inputAttractArray, float *rigidArray, float *deformArray)`
- `MStatus setComputeRestLength (const bool b)`
- `MStatus setComputeRestAngles (const bool b)`
- `MStatus setThickness (float radius)`
- `MStatus setThickness (float *radius)`
- `MStatus setInverseMass (float invMass)`
- `MStatus setInverseMass (float *invMass)`
- `MStatus setBounce (float bounce)`
- `MStatus setBounce (float *bounce)`
- `MStatus setFriction (float friction)`
- `MStatus setFriction (float *friction)`
- `MStatus setDamping (float damping)`
- `MStatus setDamping (float *damping)`
- `MStatus setSelfCollisionFlags (const bool vertToVert, const bool vertToEdge=false, const bool vertToFace=false, const bool edgeToEdge=false, const bool edgeToFace=false)`
- `MStatus setCollisionFlags (const bool vertToVert, const bool edgeToEdge=false, const bool faceToFace=false)`
- `MStatus setDisableGravity (const bool b)`
- `MStatus setMaxIterations (const int it)`
- `MStatus setMaxSelfCollisionIterations (const int it)`
- `MStatus setSelfCollisionSoftness (float softness)`
- `MStatus setSelfTrappedCheck (bool on)`
- `MStatus setSelfCrossoverPush (float val)`
- `MStatus setDragAndLift (float drag, float lift)`
- `MStatus setTangentialDrag (float tangentialDrag)`
- `MStatus setStartPressure (float startPressure)`
- `MStatus setSealHoles (bool seal)`
- `MStatus setPressure (float pressure)`
- `MStatus setTrackVolume (bool track)`
- `MStatus setIncompressibility (float v)`
- `MStatus setPressureDamping (float damp)`
- `MStatus setPumpRate (float pump)`
- `MStatus setAirTightness (float airTightness)`
- `MStatus setAddCrossLinks (bool addCrossLinks)`
- `MStatus setBendAngleDropoff (float dropoff)`
- `MStatus setShearResistance (float resistance)`
- `MStatus setStretchAndCompressionResistance (float stretchResist, float compressionResist)`
- `MStatus setStretchAndCompressionResistance (float *stretchResist, float *compressionResist)`
- `MStatus setBendAngleScale (float scale)`
- `MStatus setRestitutionAngle (float angle)`
- `MStatus setRestitutionTension (float tension)`
- `MStatus setSelfCollideWidth (float width)`
- `MStatus setBendResistance (float strength)`
- `MStatus getNumVertices (unsigned int &numVerts) const`
- `MStatus getPositions (MFloatPointArray &positions) const`
- `MStatus getVelocities (MFloatPointArray &velocities) const`
- `MStatus getThickness (MFloatArray &radius) const`
- `MStatus getInverseMass (MFloatArray &inverseMass) const`
- `MStatus getBounce (MFloatArray &bounce) const`
- `MStatus getFriction (MFloatArray &friction) const`

**Protected Member Functions**

- `void setNCloth (TnCloth *nObj, bool own)`

### MnObject

Module `OpenMayaFX`. Source: [class_mn_object.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_object.html).

**Public Member Functions**

- `MnObject ()`
- `virtual ~MnObject ()`

**Protected Member Functions**

- `void setNObject (TnObject *nObj, bool own)`

### MnParticle

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_particle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_particle.html).

**Public Member Functions**

- `MnParticle ()`
- `~MnParticle () override`
- `MStatus createNParticle ()`
- `MStatus setTopology (const int numPoints)`
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`
- `MStatus setVelocities (const MFloatPointArray &velocities)`
- `MStatus setThickness (float radius)`
- `MStatus setThickness (float *radius)`
- `MStatus setInverseMass (float invMass)`
- `MStatus setInverseMass (float *invMass)`
- `MStatus setBounce (float bounce)`
- `MStatus setBounce (float *bounce)`
- `MStatus setFriction (float friction)`
- `MStatus setFriction (float *friction)`
- `MStatus setDamping (float damping)`
- `MStatus setDamping (float *damping)`
- `MStatus setDisableGravity (const bool b)`
- `MStatus setLiquidSimulation (const bool b)`
- `MStatus setIncompressibility (float incompressibility)`
- `MStatus setRestDensity (float restDensity)`
- `MStatus setLiquidRadiusScale (float liquidRadiusScale)`
- `MStatus setViscosity (float viscosity)`
- `MStatus setViscosity (float *viscosity)`
- `MStatus setSurfaceTension (float surfaceTension)`
- `MStatus setSurfaceTension (float *surfaceTension)`
- `MStatus setMaxIterations (const int it)`
- `MStatus setMaxSelfCollisionIterations (const int it)`
- `MStatus setSelfCollisionSoftness (float softness)`
- `MStatus setDragAndLift (float drag, float lift)`
- `MStatus setCollide (const bool b)`
- `MStatus setSelfCollide (const bool b)`
- `MStatus setSelfCollideWidth (float width)`
- `MStatus getNumVertices (unsigned int &numVerts) const`
- `MStatus getPositions (MFloatPointArray &positions) const`
- `MStatus getVelocities (MFloatPointArray &velocities) const`
- `MStatus getThickness (MFloatArray &radius) const`
- `MStatus getInverseMass (MFloatArray &inverseMass) const`
- `MStatus getBounce (MFloatArray &bounce) const`
- `MStatus getFriction (MFloatArray &friction) const`

**Protected Member Functions**

- `void setNParticle (TnParticle *nObj, bool own)`

### MnRigid

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_rigid.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_rigid.html).

**Public Member Functions**

- `MnRigid ()`
- `~MnRigid () override`
- `MStatus setTopology (const int numFaces, int *numVertsPerFace, int *faces, const int numEdges, int *edges)`
- `MStatus createNRigid ()`
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`
- `MStatus setVelocities (const MFloatPointArray &velocities)`
- `MStatus setThickness (float radius)`
- `MStatus setThickness (float *radius)`
- `MStatus setBounce (float bounce)`
- `MStatus setBounce (float *bounce)`
- `MStatus setFriction (float friction)`
- `MStatus setFriction (float *friction)`
- `MStatus setCollisionFlags (const bool vertToVert, const bool edgeToEdge=false, const bool faceToFace=false)`
- `MStatus getNumVertices (unsigned int &numVerts) const`
- `MStatus getPositions (MFloatPointArray &positions) const`
- `MStatus getVelocities (MFloatPointArray &velocities) const`
- `MStatus getThickness (MFloatArray &radius) const`
- `MStatus getInverseMass (MFloatArray &inverseMass) const`
- `MStatus getBounce (MFloatArray &bounce) const`
- `MStatus getFriction (MFloatArray &friction) const`

**Protected Member Functions**

- `void setNRigid (TnRigid *nObj, bool own)`

### MnSolver

Module `OpenMayaFX`. Source: [class_mn_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_solver.html).

**Public Member Functions**

- `MnSolver ()`
- `virtual ~MnSolver ()`
- `MStatus createNSolver ()`
- `MStatus addNObject (MnObject *obj)`
- `MStatus removeNObject (MnObject *obj)`
- `MStatus setGravity (float grav)`
- `MStatus setGravityDir (float gravX, float gravY, float gravZ)`
- `MStatus setAirDensity (float dens)`
- `MStatus setWindSpeed (float speed)`
- `MStatus setWindDir (float windX, float windY, float windZ)`
- `MStatus setWindNoiseIntensity (float noise)`
- `MStatus setDisabled (bool disabled)`
- `MStatus setSubsteps (int substeps)`
- `MStatus setMaxIterations (int maxIter)`
- `MStatus setStartTime (float startTime)`
- `MStatus makeAllCollide ()`
- `MStatus removeAllCollisions ()`
- `MStatus solve (float solveTime)`
