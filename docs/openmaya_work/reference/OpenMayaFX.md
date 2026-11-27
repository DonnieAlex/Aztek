# OpenMayaFX, API module for effects

Transcribed from the reference's 2022 C++ API reference, module page https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/group___open_maya_f_x.html.

This is a class and member transcription: names, inheritance, signatures and the one-line descriptions on the class pages. Parameter and return prose that only appears in the per-member detail sections is not reproduced; follow the source link on a class to read it.

Classes in this module: 33.

## Classes

### MDynSweptLine

Module `OpenMayaFX`. Source: [class_m_dyn_swept_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dyn_swept_line.html).

Class for evaluating curve segments as lines over time. A MDynSweptLine provides access to a curve segment defined as a line. It can only be accessed with the MFnDynSweptGeometryData class that is provided as an output from the geoConnector node. The class provides parametric time evaluation for a curve. Time is in the range 0 to 1, where 1 represents the current frame and 0 represents the previous frame. In this way you can get interpolated values of a curve in motion.

**Public Member Functions**

- `MDynSweptLine ()`  
  The class constructor.
- `~MDynSweptLine ()`  
  The class destructor.
- `MVector vertex (int vertexId, double t=1.0)`  
  Return the vertex requested by id, at the parametric time value.
- `MVector normal (double x, double y, double z, double t=1.0)`  
  Given a parametric time specified by 't' and a vector, returns a normalized vector perpendicular to the tangent, and rotated into the plane defined by the tangent and vector argument.
- `MVector tangent (double t=1.0)`  
  Given a parametric time specified by 't', returns normalized tangent of the line.
- `double length (double t=1.0)`  
  Given a parametric time specified by 't', returns the total length of the line.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDynSweptTriangle

Module `OpenMayaFX`. Source: [class_m_dyn_swept_triangle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dyn_swept_triangle.html).

Class for evaluating surfaces as triangles over time. MDynSweptTriangle provides access to surfaces (Nurbs and Polys) defined as triangles. It can only be accessed with the MFnDynSweptGeometryData class that is provided as an output from the geoConnector node. The class provides parametric time evaluation for a triangle. Time is in the range 0 to 1, where 1 represents the current frame and 0 represents the previous frame. In this way you can get interpolated values of a triangle in motion.

**Public Member Functions**

- `MDynSweptTriangle ()`  
  The class constructor.
- `~MDynSweptTriangle ()`  
  The class destructor.
- `MVector vertex (int vertexId, double t=1.0)`  
  Return the vertex requested by id, at the parametric time value.
- `MVector normal (double t=1.0)`  
  Given a parametric time specified by 't', returns the normal of the triangle.
- `MVector normalToPoint (double x, double y, double z, double t=1.0)`  
  Given a point, returns the normal of the triangle in the direction towards the point.
- `MVector uvPoint (int vertexId)`  
  Given a vertex id, this method returns the UV point for the vertex as a MVector.
- `double area ()`  
  This method returns the area.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MDynamicsUtil

Module `OpenMayaFX`. Source: [class_m_dynamics_util.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_dynamics_util.html).

Utility class for Maya dynamics. MDynamicsUtil provides utility methods related to Maya dynamics (eg. particles and fluids).

**Static Public Member Functions**

- `static bool hasValidDynamics2dTexture (const MObject &node, const MObject &texAttr, MStatus *status=NULL)`  
  Certain aspects of Maya's dynamics can be textured using 2d textures.
- `static MStatus evalDynamics2dTexture (const MObject &node, const MObject &texAttr, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`  
  If a supported 2d texture (see hasValidDynamics2dTexture() method documentation) is connected to the specified attribute, this method can be called to evaluate that texture at a number of (u,v) texture coordinate values.
- `static bool inRunup ()`  
  Is Maya's dynamics system currently doing a runup?
- `static bool runupIfRequired ()`  
  If the dynamics runup prefs are set, do a runup.
- `static bool addNodeTypeToRunup (const MString &nodeTypeName)`  
  Add this node to the list of nodes participating in runup.
- `static bool removeNodeTypeFromRunup (const MString &nodeTypeName)`  
  Remove this node from the list of nodes participating in runup.


### MFnAirField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_air_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_air_field.html).

Function set for Air Fields. Function set for creation, edit, and query of Air Fields. An air field simulates the effects of moving air.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnAirField ()`  
  Destructor.
- `MFnAirField ()`  
  Default constructor.
- `MFnAirField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnAirField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction the air is blowing.
- `MStatus setDirection (const MVector &airDirection)`  
  Sets the direction vector for the air to blow.
- `double speed (MStatus *ReturnStatus=NULL) const`  
  Returns the control setting on how quickly objects match the velocity of the air field.
- `MStatus setSpeed (double value)`  
  Sets the control setting on how quickly the objects match the velocity of the air field.
- `double inheritVelocity (MStatus *ReturnStatus=NULL) const`  
  Returns the amount of the moving air field's velocity that is added to the direction and magnitude of the wind.
- `MStatus setInheritVelocity (double velocity)`  
  Sets the amount of the moving air field's velocity that is added to the direction and magnitude of the wind.
- `bool inheritRotation (MStatus *ReturnStatus=NULL) const`  
  Returns true if the air field is rotating or parented to a rotating object, and will undergo that same rotation.
- `MStatus setInheritRotation (bool enable)`  
  Enables the air field to undergo rotations and effect the direction that the air field points.
- `bool componentOnly (MStatus *ReturnStatus=NULL) const`  
  Returns true if the air field will apply force only in the direction specified by the combination of its direction, speed, and inherit velocity attributes.
- `MStatus setComponentOnly (bool enable)`  
  Enables the air field to apply force specified as a combination of its direction, speed, and inherit veloicty attributes.
- `double spread (MStatus *ReturnStatus=NULL) const`  
  Returns a value that represents an angle which objects are affected by the air fields direction setting.
- `MStatus setSpread (double value)`  
  Sets the value representing an angle which objects are affected by the air fields direction setting.
- `bool enableSpread (MStatus *ReturnStatus=NULL) const`  
  Returns true if the air field is using the spread angle to define the influence of the air field.
- `MStatus setEnableSpread (bool enable)`  
  Enables the air field to influence objects based on the spread angle setting.
- `MFnAirField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnDragField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_drag_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_drag_field.html).

Function set for Drag Fields. Function set for creation, edit, and query of Drag Fields. A drag field exerts a friction, or braking force, proportional to the speed of a moving object.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnDragField ()`  
  Destructor.
- `MFnDragField ()`  
  Default constructor.
- `MFnDragField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnDragField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction of the drag force's influence along the x, y, and z axes.
- `MStatus setDirection (const MVector &dragDirection)`  
  Sets the direction of the drag force's influence along the x, y, and z axes.
- `bool useDirection (MStatus *ReturnStatus=NULL) const`  
  Returns true if the braking force is exerted only against the component of the object's velocity that lies along the direction of the drag force.
- `MStatus setUseDirection (bool enable)`  
  Enables the braking force to be exerted only against the component of the object's velocity that lies along the direction setting of the drag force.
- `MFnDragField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnDynSweptGeometryData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_dyn_swept_geometry_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_dyn_swept_geometry_data.html).

Swept Geometry function set for dependency node data. MFnDynSweptGeometryData provides access to the MDynSweptLine and MDynSweptTriangle data for use in a user defined dependency graph node. The data is provided as an output from the geoConnector node and is primarily used to determine positional information over time. If a user written dependency node accepts MFnDynSweptGeometryData, then this class is used to extract data that comes from the geoConnector node. The MDataHandle::type method will return kDynSweptGeometry when data of this type is present. To access it, the MDataHandle::data() method is used to get an MObject for the data and this should then be used to initialize an instance of MFnDynSweptGeometryData. Important note: Users can create the data for connections but cannot produce the contents of the data as this is reserved for the Maya Dynamics internals.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnDynSweptGeometryData ()`  
  Destructor.
- `MFnDynSweptGeometryData ()`  
  Default constructor.
- `MFnDynSweptGeometryData (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `int lineCount (MStatus *ReturnStatus=NULL) const`  
  Return the number of line segments contained in the data.
- `int triangleCount (MStatus *ReturnStatus=NULL) const`  
  Return the number of triangles contained in the data.
- `MDynSweptLine sweptLine (int index, MStatus *ReturnStatus=NULL) const`  
  Return data for a swept line.
- `MDynSweptTriangle sweptTriangle (int index, MStatus *ReturnStatus=NULL) const`  
  Return data for a swept triangle.
- `MObject create (MStatus *ReturnStatus=NULL)`  
  This method create a new swept geometry data object for use with the dependency graph.
- `MFnDynSweptGeometryData (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnField

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_field.html).

Function set for Dynamic Fields. Function set for creation, edit, and query of Dynamic Fields. There are several types of dynamic fields: Air, Drag, Gravity, Newton Radial, Turbulence, Uniform, and Vortex.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnField ()`  
  Destructor.
- `MFnField ()`  
  Default constructor.
- `MFnField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MStatus getForceAtPoint (const MPointArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`  
  Compute the force of a field on an array of points, given their position, velocity, and mass.
- `MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`  
  Compute the force of a field on an array of points, given their position, velocity, and mass.
- `double magnitude (MStatus *ReturnStatus=NULL) const`  
  Returns the strength of the field.
- `MStatus setMagnitude (double mag)`  
  Sets the strength of the field.
- `double attenuation (MStatus *ReturnStatus=NULL) const`  
  Returns the rate of change where the strength of the field changes as the distance between the field and the affected object increases.
- `MStatus setAttenuation (double atten)`  
  Sets the rate of change where the strength of the field changes as the distance between the field and the affected object increases.
- `double maxDistance (MStatus *ReturnStatus=NULL) const`  
  Returns the maximum distance from the field at which the force of the field is exerted.
- `MStatus setMaxDistance (double maxDist)`  
  Sets the maximum distance from the field at which the force of the field is exerted.
- `bool perVertex (MStatus *ReturnStatus=NULL) const`  
  Returns true if the field exerts its force on each individual point (cv, particle, vertex) equally.
- `MStatus setPerVertex (bool enable)`  
  Enables the field to exert its force on each individual point (cv, particle, vertex) equally.
- `bool useMaxDistance (MStatus *ReturnStatus=NULL) const`  
  Returns true if the field will use the maximum distance setting to determine the area of influence.
- `MStatus setUseMaxDistance (bool enable)`  
  Enables the field to use the maximum distance setting to determine the area of influence.
- `double falloffCurve (const double param, MStatus *ReturnStatus=NULL)`  
  Returns falloff given the param in [0,1].
- `bool isFalloffCurveConstantOne (MStatus *ReturnStatus=NULL)`  
  Returns true if falloffCurve is a constant one (default) or false if not.
- `MFnField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnFluid

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_fluid.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_fluid.html).

Fluid node function set. This is the function set for fluid objects. A fluid object is a node in the dependency graph that contains a grid which is made up of cells. Each cell has a variety of values assigned to it such as density, velocity, temperature, and color. The grid may be 2D or 3D. See the methods below for full details. Fluid objects may be used for simulation purposes. This function set allows read and write access to the values in the cells of the fluid object.

**Public Types**

- `enum FluidMethod { kZero, kStaticGrid, kDynamicGrid, kGradient }`  
  Defines how voxel values are computed for most types of fluid data.
- `enum FluidGradient { kConstant, kXGradient, kYGradient, kZGradient, kNegXGradient, kNegYGradient, kNegZGradient, kCenterGradient }`  
  Defines the orientation of the gradient.
- `enum FalloffMethod { kNoFalloffGrid, kStaticFalloffGrid }`  
  Falloff data only supports a subset of the available fluid methods.
- `enum ColorMethod { kUseShadingColor, kStaticColorGrid, kDynamicColorGrid }`  
  Color data has its own set of methods for computing voxel data.
- `enum CoordinateMethod { kFixed, kGrid }`  
  Coordinate data has its own set of methods for computing voxel data.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnFluid ()`  
  Destructor.
- `MFnFluid ()`  
  Default constructor.
- `MFnFluid (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnFluid (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create3D (unsigned int Xres, unsigned int Yres, unsigned int Zres, double Xdim, double Ydim, double Zdim, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Creates a fluid object from the specified data and sets this function set to operate on the new fluid object.
- `MObject create2D (unsigned int Xres, unsigned int Yres, double Xdim, double Ydim, MObject parentOrOwner=MObject::kNullObj, MStatus *ReturnStatus=NULL)`  
  Creates a fluid object from the specified data and sets this function set to operate on the new fluid object.
- `MStatus getResolution (unsigned int &Xres, unsigned int &Yres, unsigned int &Zres) const`  
  Gets the resolution of the fluid.
- `MStatus getResolution (unsigned int &Xres, unsigned int &Yres) const`  
  Gets the resolution of the fluid.
- `MStatus getDimensions (double &Xdim, double &Ydim, double &Zdim) const`  
  Gets the dimensions of the fluid.
- `MStatus setSize (unsigned int Xres, unsigned int Yres, unsigned int Zres, double Xdim, double Ydim, double Zdim, bool resample)`  
  Sets the size and resolution of the grid.
- `MStatus setSize (unsigned int Xres, unsigned int Yres, double Xdim, double Ydim, bool resample)`  
  Sets the size and resolution of the grid.
- `unsigned int gridSize (MStatus *ReturnStatus=NULL)`  
  Returns the number of elements in the grid.
- `MStatus velocityGridSizes (int &xsize, int &ysize, int &zsize)`  
  Returns the number of elements in the velocity grids.
- `float * falloff (MStatus *ReturnStatus=NULL)`  
  This method returns a pointer to the storage for the falloff data in the fluid.
- `float * density (MStatus *ReturnStatus=NULL)`  
  This method returns a pointer to the storage for the density data in the fluid.
- `MStatus getVelocity (float *&Xvel, float *&Yvel, float *&Zvel)`  
  This method returns pointers to the storage for the velocity data in the fluid.
- `float * pressure (MStatus *ReturnStatus=NULL)`  
  This method returns a pointer to the storage for the pressure data in the fluid.
- `float * temperature (MStatus *ReturnStatus=NULL)`  
  This method returns a pointer to the storage for the temperature data in the fluid.
- `float * fuel (MStatus *ReturnStatus=NULL)`  
  This method returns a pointer to the storage for the fuel data in the fluid.
- `MStatus getCoordinates (float *&u, float *&v, float *&w)`  
  This method returns pointers to the storage for the uvw coordinate data in the fluid.
- `MStatus getColors (float *&r, float *&g, float *&b)`  
  This method returns pointers to the storage for the color data in the fluid.
- `MStatus setFalloffMode (FalloffMethod mode)`  
  Set the modes by which the shader falloff values in the grid are determined.
- `MStatus getFalloffMode (FalloffMethod &mode)`  
  Get the modes by which the falloff values in the grid are determined.
- `MStatus setDensityMode (FluidMethod mode, FluidGradient gradient)`  
  Set the modes by which the density values in the grid are determined.
- `MStatus getDensityMode (FluidMethod &mode, FluidGradient &gradient)`  
  Get the modes by which the density values in the grid are determined.
- `MStatus setVelocityMode (FluidMethod mode, FluidGradient gradient)`  
  Set the modes by which the velocity values in the grid are determined.
- `MStatus getVelocityMode (FluidMethod &mode, FluidGradient &gradient)`  
  Get the modes by which the velocity values in the grid are determined.
- `MStatus setTemperatureMode (FluidMethod mode, FluidGradient gradient)`  
  Set the modes by which the temperature values in the grid are determined.
- `MStatus getTemperatureMode (FluidMethod &mode, FluidGradient &gradient)`  
  Get the modes by which the temperature values in the grid are determined.
- `MStatus setFuelMode (FluidMethod mode, FluidGradient gradient)`  
  Set the modes by which the fuel values in the grid are determined.
- `MStatus getFuelMode (FluidMethod &mode, FluidGradient &gradient)`  
  Get the modes by which the fuel values in the grid are determined.
- `MStatus setCoordinateMode (CoordinateMethod mode)`  
  Set the modes by which the UVW coordinate values in the grid are determined.
- `MStatus getCoordinateMode (CoordinateMethod &mode)`  
  Get the modes by which the UVW coordinates values in the grid are determined.
- `MStatus setColorMode (ColorMethod mode)`  
  Set the modes by which the color values in the grid are determined.
- `MStatus getColorMode (ColorMethod &mode)`  
  Get the modes by which the color values in the grid are determined.
- `MStatus getForceAtPoint (const MPointArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`  
  Compute the force of the fluid as a field on an array of points, given their position, velocity, and mass.
- `MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime=1.0/24.0)`  
  Compute the force of a field on an array of points, given their position, velocity, and mass.
- `bool toGridIndex (const MPoint &objectSpacePoint, int3 &gridCoords, MStatus *status=NULL)`  
  For the given point in object space, get the grid indices of the voxel that it happens to lie in.
- `MStatus voxelCenterPosition (int xi, int yi, int zi, MPoint &objectSpacePoint)`  
  For the given voxel, get the location of the center in object space.
- `MStatus updateGrid ()`  
  Tells the fluid shape that the contents of the fluid grid has changed.
- `void emitIntoArrays (float val, int x, int y, int z, float density, float heat, float fuel, bool doColor, const MColor &emitColor)`  
  Use this method to add density, heat, fuel, and/or color to a particular voxel of a fluid.
- `int index (int xi, int yi)`  
  This is a utility routine for finding the index of a cell in an array of fluid data.
- `int index (int xi, int yi, int zi)`  
  This is a utility routine for finding the index of a cell in an array of fluid data.
- `void index (int ai, int &xi, int &yi, int &zi)`  
  This is a utility routine for finding the coordinates of a cell in an array of fluid data given the index.
- `int index (int xi, int yi, int zi, int xres, int yres, int zres)`  
  This is a utility routine for finding the index of a cell given the X, Y and Z resolutions.
- `void index (int ai, int xres, int yres, int zres, int &xi, int &yi, int &zi)`  
  This is a utility routine for finding the coordinates of a cell given the index, and the X, Y and Z resolutions.
- `bool isAutoResize ()`  
  Is this an autoresize fluid?
- `bool isResizeToEmitter ()`  
  If this is an autoresize fluid, should it also resize to the emitter.
- `MStatus expandToInclude (const MPoint &min, const MPoint &max)`  
  Tells the fluid shape to autoresize to include these two points This would normally be used in a fluid emitter node if the fluid had resizeToEmitter turned on.
- `MFnFluid (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnGravityField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_gravity_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_gravity_field.html).

Function set for Gravity Fields. Function set for creation, edit, and query of Gravity Fields. A gravity field simulates the Earth's gravitational force. It pulls objects in a fixed direction entirely independent of their position or mass.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnGravityField ()`  
  Destructor.
- `MFnGravityField ()`  
  Default constructor.
- `MFnGravityField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnGravityField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction of the gravitational force.
- `MStatus setDirection (const MVector &gravityDirection)`  
  Sets the direction vector of the gravitational force.
- `MFnGravityField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnInstancer

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_instancer.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_instancer.html).

Particle Instancer object access class. Class for obtaining information about a particle instancer node.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnInstancer ()`  
  Destructor.
- `MFnInstancer ()`  
  Default constructor.
- `MFnInstancer (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnInstancer (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `int particleCount () const`  
  Returns the number of particles feeding the active instancer.
- `int instancesForParticle (int p, MDagPathArray &paths, MMatrix &instancerMatrix, MStatus *ReturnStatus=NULL) const`  
  Returns the DAG paths and instancer matrix for all instances generated by a specified particle.
- `MStatus allInstances (MDagPathArray &paths, MMatrixArray &matrices, MIntArray &particlePathStartIndices, MIntArray &pathIndices) const`  
  Returns information about all instances generated by a particular particle instancer node.
- `MFnInstancer (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnNIdData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_n_id_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_n_id_data.html).

function set for nId object data Class for transferring N id data between connections

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnNIdData ()`  
  Destructor.
- `MFnNIdData ()`  
  Default constructor.
- `MFnNIdData (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create () const`  
  Creates an underlying Maya nId data object and return it as an MObject.
- `MStatus getObjectPtr (MnObject *&ptr) const`  
  NO SCRIPT SUPPORT.
- `MnObject * getObjectPtr (MStatus *status=NULL) const`  
  Get the nObject pointer.
- `MFnNIdData (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnNObjectData

Module `OpenMayaFX`. Inherits `MFnData`. Source: [class_m_fn_n_object_data.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_n_object_data.html).

function set for nCloth object data Class for transferring N object data between connections

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnNObjectData ()`  
  Destructor.
- `MFnNObjectData ()`  
  Default constructor.
- `MFnNObjectData (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MObject create () const`  
  Creates an underlying Maya N data object and return it as an MObject.
- `MStatus getObjectPtr (MnCloth *&ptr) const`  
  NO SCRIPT SUPPORT.
- `MStatus getObjectPtr (MnRigid *&ptr) const`  
  NO SCRIPT SUPPORT.
- `MStatus getObjectPtr (MnParticle *&ptr) const`  
  NO SCRIPT SUPPORT.
- `MnCloth * getClothObjectPtr (MStatus *status=NULL) const`  
  Get the nObject pointer for an nCloth.
- `MnRigid * getRigidObjectPtr (MStatus *status=NULL) const`  
  Get the nObject pointer for an nRigid.
- `MnParticle * getParticleObjectPtr (MStatus *status=NULL) const`  
  Get the nObject pointer for an nParticle.
- `MStatus setObjectPtr (MnCloth *ptr)`  
  Set the nCloth pointer.
- `MStatus setObjectPtr (MnRigid *ptr)`  
  Set the nRigid pointer.
- `MStatus setObjectPtr (MnParticle *ptr)`  
  Set the nParticle pointer.
- `MStatus getCollide (bool &) const`  
  Get wether the nObject is allowed to participate in collisions.
- `MStatus getCached (bool &cached) const`  
  NO SCRIPT SUPPORT.
- `bool isCached (MStatus *status=NULL) const`  
  Get whether the nObject is cached.
- `MStatus setCached (bool cached)`  
  Set wether the nObject is cached.
- `MFnNObjectData (const MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnNewtonField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_newton_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_newton_field.html).

Function set for Newton Fields. Function set for creation, edit, and query of Newton Fields. A newton field pulls an object toward another with a force that is dependent on the mass of the object exerting the field. It follows Newton's Law of Universal Gravitation.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnNewtonField ()`  
  Destructor.
- `MFnNewtonField ()`  
  Default constructor.
- `MFnNewtonField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnNewtonField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `double minDistance (MStatus *ReturnStatus=NULL) const`  
  Returns the minimum distance from the newton field at which the force of the field is exerted.
- `MStatus setMinDistance (double distance)`  
  Sets the minimum distance from the newton field at which the force of the field is exerted.
- `MFnNewtonField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnParticleSystem

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_particle_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_particle_system.html).

Class for obtaining information about a particle system. Particle object access class. isValid(): data is valid (predicate) renderType(): object render type count(): particle count at current frame Use this chart to determine which methods to call based on the render type of the particle object: Valid Methods for render type kCloud: position(): particle position radius(): particle radius surfaceShading(): object surface shading value betterIllum(): invoke thick cloud sampling threshold(): object threshold disableCloudAxis(): for internal use; do not call flatShaded(): cloud is flat shaded in modeling view Valid Methods for render type kTube: position0(): particle start position position1(): particle end position radius0(): particle start radius radius1(): particle end radius tailSize(): length scale factor Valid Methods for render type kBlobby: position(): particle position radius(): particle radius threshold(): object threshold Valid Methods for render type kHardware: position(): particle position radius(): particle radius

**Public Types**

- `enum RenderType { kCloud, kTube, kBlobby, kMultiPoint, kMultiStreak, kNumeric, kPoints, kSpheres, kSprites, kStreak }`  
  Ways in which particles can be rendered.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnParticleSystem ()`  
  Destructor.
- `MFnParticleSystem ()`  
  Default constructor.
- `MFnParticleSystem (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnParticleSystem (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MObject create (MStatus *status=NULL)`  
  To create a new particleShape with a transform.
- `MObject create (MObject parent, MStatus *status=NULL)`  
  To create a new particleShape.
- `MStatus emit (const MPoint &position)`  
  To add a new particle at the given position.
- `MStatus emit (const MPointArray &positionArray)`  
  To add an array of new particles at the given positions.
- `MStatus emit (const MPoint &position, const MVector &velocity)`  
  To add a new particle at the position with its velocity.
- `MStatus emit (const MPointArray &positionArray, const MVectorArray &velocityArray)`  
  To add an array of new particles at the given positions with their velocity values.
- `MStatus saveInitialState () const`  
  To reset the particle's current state as the initial state.
- `void evaluateDynamics (MTime &to, bool runupFromStart)`  
  Run up the particle system to a certain frame, from either the current frame, or from the start of the simulation.
- `bool isValid () const`  
  Returns false if the particle array is nullptr.
- `MString particleName () const`  
  Returns the name of this particle system.
- `unsigned int count () const`  
  Returns the particle count at the current frame.
- `void setCount (unsigned int) const`  
  Set the particle count at the current frame.
- `RenderType renderType () const`  
  Return the render type used by the particle object.
- `void position (MVectorArray &) const`  
  Compute each particle's position at the start of the time step.
- `void velocity (MVectorArray &) const`  
  To return velocity array for all particles.
- `void acceleration (MVectorArray &) const`  
  To return acceleration array for all particles.
- `void position0 (MVectorArray &) const`  
  Only for use with an MFnParticleSystem of renderType kTube.
- `void position1 (MVectorArray &) const`  
  Only for use with an MFnParticleSystem of renderType kTube.
- `void radius (MDoubleArray &) const`  
  Calculates particle radii.
- `void radius0 (MDoubleArray &) const`  
  Only for use with an MFnParticleSystem of renderType kTube.
- `void radius1 (MDoubleArray &) const`  
  Only for use with an MFnParticleSystem of renderType kTube.
- `double surfaceShading () const`  
  Only for use with an MFnParticleSystem of renderType kCloud.
- `double threshold () const`  
  Only for use with an MFnParticleSystem of renderType kCloud or kBlobby.
- `bool betterIllum () const`  
  Only for use with an MFnParticleSystem of renderType kCloud.
- `bool flatShaded () const`  
  To check if a particle shape of cloud type is flat shaded.
- `bool disableCloudAxis () const`  
  Only for use with an MFnParticleSystem of renderType kCloud.
- `double tailSize () const`  
  Only for use with an MFnParticleSystem of renderType kTube.
- `void particleIds (MIntArray &) const`  
  Return an array of particle identifiers at the start of the time step.
- `void age (MDoubleArray &) const`  
  Populates the given array with the age values of the particles in this system.
- `void lifespan (MDoubleArray &) const`  
  Populates the given array with the lifespan values of the particles in this system, if the particles have this attribute.
- `void rgb (MVectorArray &) const`  
  Populates the given array with the color values of the particles in this system, if the particles have this attribute.
- `void opacity (MDoubleArray &) const`  
  Populates the given array with the opacity values of the particles in this system, if the particles have this attribute.
- `void mass (MDoubleArray &) const`  
  To return mass array for all particles.
- `void emission (MVectorArray &) const`  
  Populates the given array with the incandescence values of the particles in this system, if the particles have this attribute.
- `bool hasLifespan () const`  
  Returns whether or not the particles in this system have a lifespan attribute.
- `bool hasRgb () const`  
  Returns whether or not the particles in this system have an rgb attribute.
- `bool hasOpacity () const`  
  Returns whether or not the particles in this system have an opacity attribute.
- `bool hasEmission () const`  
  Returns whether or not the particles in this system have an emission attribute.
- `bool primaryVisibility () const`  
  Returns whether or not the rendering attribute for primary visibility in reflections has been enabled.
- `bool visibleInReflections () const`  
  Returns whether or not the rendering attribute for particle visibility in reflections has been enabled.
- `bool visibleInRefractions () const`  
  Returns whether or not the rendering attribute for particle visibility in refractions has been enabled.
- `bool castsShadows () const`  
  Returns whether or not the rendering attribute for particles casting shadows has been enabled.
- `bool receiveShadows () const`  
  Returns whether or not the rendering attribute for particles receiving shadows has been enabled.
- `unsigned int getPerParticleAttribute (const MString &attrName, MIntArray &, MStatus *status=NULL) const`  
  To get per particle integer attribute with its attribute name.
- `unsigned int getPerParticleAttribute (const MString &attrName, MVectorArray &, MStatus *status=NULL) const`  
  To get the per particle vector attribute of the given name.
- `unsigned int getPerParticleAttribute (const MString &attrName, MDoubleArray &, MStatus *status=NULL) const`  
  To get the per particle double attribute of the given name.
- `void setPerParticleAttribute (const MString &attrName, MVectorArray &, MStatus *status=NULL) const`  
  To set the per particle vector attribute of the given name with the given values.
- `void setPerParticleAttribute (const MString &attrName, MDoubleArray &, MStatus *status=NULL) const`  
  To set the per particle double attribute of the given name with the given values.
- `bool isPerParticleIntAttribute (const MString &attrName, MStatus *status=NULL) const`  
  To check if the input attribute is a per particle integer attribute.
- `bool isPerParticleDoubleAttribute (const MString &attrName, MStatus *status=NULL) const`  
  To check if the input attribute is a per particle double attribute.
- `bool isPerParticleVectorAttribute (const MString &attrName, MStatus *status=NULL) const`  
  To check if the input attribute is a per particle vector attribute.
- `bool isDeformedParticleShape (MStatus *status=NULL) const`  
  To return if this particle shape is deformed.
- `MObject deformedParticleShape (MStatus *status=NULL) const`  
  To get the deformed particleShape.
- `MObject originalParticleShape (MStatus *status=NULL) const`  
  To get the original particleShape.
- `MFnParticleSystem (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnPfxGeometry

Module `OpenMayaFX`. Inherits `MFnDagNode`. Source: [class_m_fn_pfx_geometry.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_pfx_geometry.html).

PfxGeometry node function set. This is the function set for paint effects objects. PfxGeometry is the parent class for the stroke and pfxHair node. The output geometry for pfxHair and stroke nodes may be accessed through this class.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnPfxGeometry ()`  
  Destructor.
- `MFnPfxGeometry ()`  
  Default constructor.
- `MFnPfxGeometry (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnPfxGeometry (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MStatus getLineData (MRenderLineArray &mainLines, MRenderLineArray &leafLines, MRenderLineArray &flowerLines, bool doLines, bool doTwist, bool doWidth, bool doFlatness, bool doParameter, bool doColor, bool doIncandescence, bool doTransparency, bool worldSpace)`  
  Get line data for the current output pfx tubes.
- `MStatus getBoundingBox (double *min, double *max)`  
  Gets the bounding box of the specified geometry.
- `MFnPfxGeometry (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnRadialField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_radial_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_radial_field.html).

Function set for Radial Fields. Function set for creation, edit, and query of Radial Fields. A radial field pushes objects directly away or pulls them directly toward itself, like a magnet.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnRadialField ()`  
  Destructor.
- `MFnRadialField ()`  
  Default constructor.
- `MFnRadialField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnRadialField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `double radialType (MStatus *ReturnStatus=NULL) const`  
  Returns a type that controls the way the radial field is attenuated.
- `MStatus setType (double value)`  
  Sets a radial field type that controls the way the field is attenuated.
- `MFnRadialField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnTurbulenceField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_turbulence_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_turbulence_field.html).

Function set for Turbulence Fields. Function set for creation, edit, and query of Turbulence Fields. A turbulence field causes irregularities in the motion of affected objects. These irregularities are also called noise or jitter.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnTurbulenceField ()`  
  Destructor.
- `MFnTurbulenceField ()`  
  Default constructor.
- `MFnTurbulenceField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnTurbulenceField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `double frequency (MStatus *ReturnStatus=NULL) const`  
  Returns the frequency parameter that generates irregularities in the fields motion.
- `MStatus setFrequency (double value)`  
  Sets the frequency parameter of the Perlin noise function used by the turbulence field.
- `double phase (MStatus *ReturnStatus=NULL) const`  
  Returns the phase shift parameter that influences the direction of the turbulence field disruption.
- `MStatus setPhase (double value)`  
  Sets the phase shift parameter of the Perlin noise function used by the turbulence field.
- `MFnTurbulenceField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnUniformField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_uniform_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_uniform_field.html).

Function set for Uniform Fields. Function set for creation, edit, and query of Uniform Fields. A uniform field pushes objects uniformly in a single direction.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnUniformField ()`  
  Destructor.
- `MFnUniformField ()`  
  Default constructor.
- `MFnUniformField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnUniformField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction the uniform field pushes objects.
- `MStatus setDirection (const MVector &uniformDirection)`  
  Sets the direction the uniform field pushes objects.
- `MFnUniformField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnVolumeAxisField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_volume_axis_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_volume_axis_field.html).

Function set for VolumeAxis Fields. Function set for creation, edit, and query of VolumeAxis Fields. A volume axis field provides in field form some of the speed controls of volume emitters.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnVolumeAxisField ()`  
  Destructor.
- `MFnVolumeAxisField ()`  
  Default constructor.
- `MFnVolumeAxisField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnVolumeAxisField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `bool invertAttenuation (MStatus *ReturnStatus=NULL) const`  
  Returns the value of the invertAttenuation attribute for the field force.
- `MVector direction (MStatus *ReturnStatus=NULL) const`  
  Returns the direction attribute for the field force.
- `double speedAlongAxis (MStatus *ReturnStatus=NULL) const`  
  Returns the alongAxis attribute of the field.
- `double speedAroundAxis (MStatus *ReturnStatus=NULL) const`  
  Returns the aroundAxis attribute of the field.
- `double speedAwayFromAxis (MStatus *ReturnStatus=NULL) const`  
  Returns the awayFromAxis attribute of the field.
- `double speedAwayFromCenter (MStatus *ReturnStatus=NULL) const`  
  Returns the awayFromCenter attribute of the field.
- `double directionalSpeed (MStatus *ReturnStatus=NULL) const`  
  Returns the directionalSpeed attribute of the field.
- `double turbulence (MStatus *ReturnStatus=NULL) const`  
  Returns the turbulence intensity of the field.
- `double turbulenceSpeed (MStatus *ReturnStatus=NULL) const`  
  Returns the rate of change of the turbulence over time.
- `MVector turbulenceFrequency (MStatus *ReturnStatus=NULL) const`  
  Returns the turbulenceFrequency attribute for the field force.
- `MVector turbulenceOffset (MStatus *ReturnStatus=NULL) const`  
  Returns the turbulenceOffset attribute for the field force.
- `double detailTurbulence (MStatus *ReturnStatus=NULL) const`  
  Returns the intensity of a second higher frequency turbulence.
- `MStatus setInvertAttenuation (bool enable)`  
  Enables the field will be stronger the closer to the edge of the volume a point is.
- `MStatus setDirection (const MVector &direction)`  
  Sets the direction attribute for the field force.
- `MStatus setSpeedAlongAxis (double value)`  
  Sets the speed along axis attribute for the field force.
- `MStatus setSpeedAroundAxis (double value)`  
  Sets the speed around axis attribute for the field force.
- `MStatus setSpeedAwayFromAxis (double value)`  
  Sets the speed away from axis attribute for the field force.
- `MStatus setSpeedAwayFromCenter (double value)`  
  Sets the speed away from center attribute for the field force.
- `MStatus setDirectionalSpeed (double value)`  
  Sets the directional speed attribute for the field force.
- `MStatus setTurbulence (double value)`  
  Sets the turbulence attribute for the field force.
- `MStatus setTurbulenceSpeed (double value)`  
  Sets the turbulence speed attribute for the field force.
- `MStatus setTurbulenceFrequency (const MVector &value)`  
  Sets the turbulenceFrequency attribute for the field force.
- `MStatus setTurbulenceOffset (const MVector &value)`  
  Sets the turbulenceOffset attribute for the field force.
- `MStatus detailTurbulence (double value)`  
  Sets the detailTurbulence attribute for the field force.
- `MFnVolumeAxisField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MFnVortexField

Module `OpenMayaFX`. Inherits `MFnField`. Source: [class_m_fn_vortex_field.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_vortex_field.html).

Function set for Vortex Fields. Function set for creation, edit, and query of Vortex Fields. A vortex field pulls objects in a circular direction, like a whirlpool or tornado.

**Public Member Functions**

- `virtual MFn::Type type () const`  
  Function set type.
- `virtual ~MFnVortexField ()`  
  Destructor.
- `MFnVortexField ()`  
  Default constructor.
- `MFnVortexField (MObject &object, MStatus *ReturnStatus=NULL)`  
  Constructor.
- `MFnVortexField (const MDagPath &object, MStatus *ret=NULL)`  
  Constructor.
- `MVector axis (MStatus *ReturnStatus=NULL) const`  
  Returns the axis around which the vortex field exerts it's force.
- `MStatus setAxis (const MVector &axisVector)`  
  Sets the axis around which the vortex field exerts it's force.
- `MFnVortexField (const MObject &object, MStatus *ret=NULL)`  
  Constructor.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MHairSystem

Module `OpenMayaFX`. Source: [class_m_hair_system.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_hair_system.html).

Interface with Maya's Dynamic Hair System. MHairSystem provides an interface with Maya's Dynamic Hair System. At present, this interface only supports the ability to override the geometry collision component of hair. There is currently no support for overriding the other hair collision components (i.e. implicit object collision and hair self collision). The rationale for providing a hair collision API is to allow the user to override Maya's internal algorithm which is a high-performance simulation. There is an example plug-in provided in the Developer's Toolkit called hairCollisionSolver.cpp.

**Static Public Member Functions**

- `static MStatus registerCollisionSolverCollide (MHairSystemCollisionSolverCollideFnPtr)`  
  Register the user-supplied collision solver `fnPtr' with Maya.
- `static MStatus registerCollisionSolverPreFrame (MHairSystemCollisionSolverPreFrameFnPtr)`  
  Register the user-supplied pre-frame method `fnPtr' with Maya.
- `static MStatus unregisterCollisionSolverCollide ()`  
  De-register any user-defined collision solver that might have been registered via registerCollisionSolverCollide().
- `static MStatus unregisterCollisionSolverPreFrame ()`  
  De-register any user-defined pre-frame callback that might have been registered via MHairSystem::registerCollisionSolverPreFrame.
- `static MObject getCollisionObject (const MObject hairSystem, const int index, MStatus *status)`  
  Returns the requested collision object from the hair system.
- `static MStatus getCollisionObject (const MObject hairSystem, MObjectArray &objects, MIntArray &logicalIndices)`  
  Returns all collision objects for the hair system.
- `static MObject getFollicle (const MObject hairSystem, const int index, MStatus *status)`  
  Returns the requested follicle from the hair system.
- `static MStatus getFollicle (const MObject hairSystem, MObjectArray &follicles, MIntArray &logicalIndices)`  
  Returns all follicles for the hair system.
- `static void setRegisteringCallableScript ()`  
  Mark this MHairSystem object as one that will be passed callbacks defined in script.
- `static bool registeringCallableScript ()`  
  Return true if this MHairSystem object has its callbacks defined in script.
- `static const char * className ()`  
  Returns the name of this class.


### MPxEmitterNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_emitter_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_emitter_node.html).

Base class for user defined particle emitters. MPxEmitterNode allows the creation and manipulation of dependency graph nodes representing particle emitters.

**Public Types**

- `enum MEmitterType { kDirectional = 0, kOmni, kSurface, kCurve, kVolume }`  
  Types of emitters.

**Public Member Functions**

- `MPxEmitterNode ()`  
  Class constructor.
- `~MPxEmitterNode () override`  
  The class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`  
  This method should be overridden in user defined nodes.
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`  
  Overriding this method allows the drawing of custom geometry using standard OpenGL calls.
- `MEmitterType getEmitterType (MDataBlock &block)`  
  Retrieves the type of the emitter, determined by the "emitterType" attribute value.
- `MObject getOwnerShape ()`  
  If the emitter is a emitting from an object, this method returns the shape node for the object.
- `MTime getCurrentTime (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method returns the time at which the emitter is currently being evaluated.
- `MTime getStartTime (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method returns the start times for each particle system or fluid into which the emitter is emitting.
- `MTime getDeltaTime (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method returns the width of the time interval represented by the current emitter evaluation.
- `int getRandomSeed (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method returns the random seed for a specified emission target (particle system or fluid shape into which the emitter is emitting).
- `void getRandomState (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method copies the emitter node attribute representing the current random state for a particular emitter target into a local variable on the emitter object, to facilitate efficient random number generation using the randgen() method.
- `void setRandomState (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method copies the emitter node class random state data member onto the emitter node's random state attribute.
- `void resetRandomState (int plugIndex, MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method resets the emitter's random state data member.
- `double randgen ()`  
  Intended to be called from within the emitter's compute() method, this method generates a double-precision random number in the rand [0,1].
- `double getRate (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "rate" attribute value common to all particle and fluid emitters.
- `double getMinDistance (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "minDistance" attribute value common to all particle and fluid emitters.
- `double getMaxDistance (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "maxDistance" attribute value common to all particle and fluid emitters.
- `MPoint getWorldPosition ()`  
  Returns the worldspace coordinates of the emitter.
- `MMatrix getWorldMatrix ()`  
  Returns the matrix that maps from the emitter's local space coordinates to worldspace.
- `bool volumePrimitiveBoundingBox (MBoundingBox &box)`  
  For volume emitters, this method returns the object-space bounding box of the volume primitive associated with the emitter.
- `bool volumePrimitivePointInside (const MPoint &worldPoint, const MMatrix &emitterWorldMatrix)`  
  For volume emitters, this method determines whether a particular point in space lies within the volume defined by the emitter's volume primitive.
- `double volumePrimitiveDistanceFromAxis (const MPoint &worldPoint, const MMatrix &emitterWorldMatrix)`  
  For volume emitters, this method determines the distance from a particular point to the major axis of the volumetric primitive associated with the emitter.
- `bool hasValidEmission2dTexture (const MObject &texAttr, MStatus *status=NULL)`  
  Certain aspects of Maya's particle and fluid emitters can be textured using 2d textures.
- `MStatus evalEmission2dTexture (const MObject &texAttr, MDoubleArray &uCoords, MDoubleArray &vCoords, MVectorArray *resultColors, MDoubleArray *resultAlphas)`  
  If a supported 2d texture (see hasValidEmission2dTexture() method documentation) is connected to the given emitter attribute, this method can be called to evaluate that texture at a number of (u,v) texture coordinate values.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject mEmitterType`  
  emitter type attribute
- `static MObject mRate`  
  rate attribute
- `static MObject mSpeed`  
  speed attribute
- `static MObject mDirection`  
  direction attribute
- `static MObject mDirectionX`  
  X component of mDirection.
- `static MObject mDirectionY`  
  Y component of mDirection.
- `static MObject mDirectionZ`  
  Z component of mDirection.
- `static MObject mOwnerPosData`  
  owner postion attribute, multi
- `static MObject mOwnerVelData`  
  owner velocity attribute, multi
- `static MObject mOwnerCentroid`  
  owner centroid attribute
- `static MObject mOwnerCentroidX`  
  X component of mOwnerCentroid.
- `static MObject mOwnerCentroidY`  
  Y component of mOwnerCentroid.
- `static MObject mOwnerCentroidZ`  
  Z component of mOwnerCentroid.
- `static MObject mSweptGeometry`  
  input swept geometry attribute
- `static MObject mWorldMatrix`  
  world matrix attribute
- `static MObject mStartTime`  
  start time attribute
- `static MObject mDeltaTime`  
  delta time attribute
- `static MObject mIsFull`  
  flag for maximum count attribute
- `static MObject mInheritFactor`  
  inherit factor attribute
- `static MObject mSeed`  
  random seed attribute
- `static MObject mRandState`  
  random state attribute
- `static MObject mCurrentTime`  
  current time attribute
- `static MObject mOutput`  
  output particle attribute, multi
- `static MObject mMinDistance`  
  min distance attribute
- `static MObject mMaxDistance`  
  max distance attribute


### MPxFieldNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_field_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_field_node.html).

Base class for user defined fields. MPxFieldNode allows the creation and manipulation of dependency graph nodes representing fields. This is the top level of a hierarchy of field node function sets. It permits manipulation of the attributes common to all types of fields.

**Public Member Functions**

- `MPxFieldNode ()`  
  Class constructor.
- `~MPxFieldNode () override`  
  The class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`  
  This method should be overridden in user defined nodes.
- `virtual MStatus getForceAtPoint (const MVectorArray &point, const MVectorArray &velocity, const MDoubleArray &mass, MVectorArray &force, double deltaTime)`  
  This method is not required to be overridden, it is only necessary for compatibility with the MFnField function set.
- `virtual MStatus iconSizeAndOrigin (GLuint &width, GLuint &height, GLuint &xbo, GLuint &ybo)`  
  Define the size and the origin of the field's icon.
- `virtual MStatus iconBitmap (GLubyte *bitmap)`  
  Define the bitmap for the field's icon.
- `virtual void draw (M3dView &view, const MDagPath &path, M3dView::DisplayStyle style, M3dView::DisplayStatus)`  
  Overriding this method allows the drawing of custom geometry using standard OpenGL calls.
- `virtual double falloffCurve (const double param, MStatus *ReturnStatus=NULL)`  
  Returns the falloff at the given parameter value.
- `virtual bool isFalloffCurveConstantOne (MStatus *ReturnStatus=NULL)`  
  Returns true if the falloffCurve is a constant one (default) or false if not.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject mMagnitude`  
  magnitude attribute
- `static MObject mAttenuation`  
  attenuation attribute
- `static MObject mMaxDistance`  
  maximum distance attribute
- `static MObject mUseMaxDistance`  
  flag for maximum distance attribute
- `static MObject mApplyPerVertex`  
  flag for per vertex attribute
- `static MObject mInputData`  
  input attribute compound, multi
- `static MObject mInputPositions`  
  child attribute, multi
- `static MObject mInputVelocities`  
  child attribute, multi
- `static MObject mInputMass`  
  child attribute, multi
- `static MObject mDeltaTime`  
  child attribute
- `static MObject mInputForce`  
  forces input attribute, multi
- `static MObject mOutputForce`  
  forces output attribute, multi
- `static MObject mOwnerCentroidX`  
  X component of mOwnerCentroid.
- `static MObject mOwnerCentroidY`  
  Y component of mOwnerCentroid.
- `static MObject mOwnerCentroidZ`  
  Z component of mOwnerCentroid.
- `static MObject mOwnerCentroid`  
  owner centroid attribute
- `static MObject mOwnerPosData`  
  owner position attribute, multi
- `static MObject mOwnerVelData`  
  owner velocity attribute, multi
- `static MObject mWorldMatrix`  
  world matrix attribute
- `static MObject mInputPPData`  
  Attribute for input pp data from particleShape.
- `static MObject mOwnerPPData`  
  Attribute for input pp data from owner particleShape.


### MPxFluidEmitterNode

Module `OpenMayaFX`. Inherits `MPxEmitterNode`. Source: [class_m_px_fluid_emitter_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_fluid_emitter_node.html).

Base class for user defined particle emitters. MPxFluidEmitterNode allows the creation and manipulation of dependency graph nodes representing fluid emitters.

**Public Member Functions**

- `MPxFluidEmitterNode ()`  
  Class constructor.
- `~MPxFluidEmitterNode () override`  
  The class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`  
  This method should be overridden in user defined nodes.
- `virtual MStatus fluidEmitter (const MObject &fluidObj, const MMatrix &worldMatrix, int plugIndex)`  
  This is the main method that plug-in fluid emitter nodes must override in order to emit into fluids.
- `double fluidDensityEmission (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidDensityEmission" attribute value common to all fluid emitters.
- `double fluidHeatEmission (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidHeatEmission" attribute value common to all fluid emitters.
- `double fluidFuelEmission (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidFuelEmission" attribute value common to all fluid emitters.
- `bool fluidEmitColor (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidEmitColor" attribute value common to all fluid emitters.
- `MColor fluidColor (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidColor" attribute value common to all fluid emitters.
- `double fluidDropoff (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidDropoff" attribute value common to all fluid emitters.
- `double turbulence (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "turbulence" attribute value common to all fluid emitters.
- `bool fluidJitter (MDataBlock &block)`  
  Intended to be called from within the emitter's compute() method, this method retrieves the "fluidJitter" attribute value common to all fluid emitters.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject mFluidDensityEmission`  
  density emission rate attribute
- `static MObject mFluidHeatEmission`  
  heat emission rate attribute
- `static MObject mFluidFuelEmission`  
  fuel emission rate attribute
- `static MObject mEmitFluidColor`  
  color emission toggle attribute
- `static MObject mFluidColor`  
  emission color attribute
- `static MObject mFluidColorR`  
  emission red color channel attribute
- `static MObject mFluidColorG`  
  emission green color channel attribute
- `static MObject mFluidColorB`  
  emission blue color channel attribute
- `static MObject mFluidDropoff`  
  emission dropoff attribute
- `static MObject mTurbulence`  
  emission turbulence attribute
- `static MObject mFluidJitter`  
  emission jitter attribute
- `static MObject mEmissionFunction`  
  emission function attribute


### MPxParticleAttributeMapperNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_particle_attribute_mapper_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_particle_attribute_mapper_node.html).

User defined per particle attribute mapping nodes. MPxParticleAttributeMapperNode is the parent class of all user defined per particle attribute mapping nodes. This class extends Maya's internal 'arrayMapper' node, inheriting its attributes and default behaviour.

**Public Member Functions**

- `MPxParticleAttributeMapperNode ()`  
  Class constructor.
- `~MPxParticleAttributeMapperNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  Returns the type of node that this is.
- `MStatus compute (const MPlug &plug, MDataBlock &dataBlock) override`  
  This method should be overridden in user defined nodes.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject computeNode`  
  Input node used to compute output values.
- `static MObject uCoordPP`  
  The U component of an array of sample positions.
- `static MObject vCoordPP`  
  The V component of an array of sample positions.
- `static MObject outMinValue`  
  The minimum limit on the output attribute outValuePP.
- `static MObject outMaxValue`  
  The maximum limit on the output attribute outValuePP.
- `static MObject computeNodeColor`  
  This attribute allows the propagation of any changes to the.
- `static MObject computeNodeColorR`  
  The R component of the computeNodeColor.
- `static MObject computeNodeColorG`  
  The G component of the computeNodeColor.
- `static MObject computeNodeColorB`  
  The B component of the computeNodeColor.
- `static MObject time`  
  This attribute holds the value for time (in seconds)
- `static MObject outColorPP`  
  The outputColor vector per-particle attribute.
- `static MObject outValuePP`  
  The outputValue vector per-particle attribute.


### MPxSpringNode

Module `OpenMayaFX`. Inherits `MPxNode`. Source: [class_m_px_spring_node.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_spring_node.html).

Base class for user defined spring law. MPxSpringNode allows the creation and manipulation of dependency graph nodes representing a spring law. The class is a DAG node and Maya manages the drawing, creation, and selection of springs. A user defined spring node is a DAG node that can have attributes and a applySpringLaw() method. To derive the full benefit of the MPxSpringNode class, it is suggested that you do not write your own compute() method. Instead, write the applySpringLaw() method. All of the parameters passed into this method will be supplied by Maya. To create a user defined spring node, derive from this class and override the applySpringLaw() method. The other methods of the parent class MPxNode may also be overridden to perform dependency node capabilities.

**Public Member Functions**

- `MPxSpringNode ()`  
  Class constructor.
- `~MPxSpringNode () override`  
  Class destructor.
- `MPxNode::Type type () const override`  
  This method returns the type of the node.
- `virtual MStatus applySpringLaw (double stiffness, double damping, double restLength, double endMass1, double endMass2, const MVector &endP1, const MVector &endP2, const MVector &endV1, const MVector &endV2, MVector &forceV1, MVector &forceV2)`  
  This method should be overridden in user defined nodes.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.

**Static Public Attributes**

- `static MObject mEnd1Weight`  
  from end weight attribute
- `static MObject mEnd2Weight`  
  to end weight attribute
- `static MObject mDeltaTime`  
  delta time attribute


### MRenderLine

Module `OpenMayaFX`. Source: [class_m_render_line.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_line.html).

Class for accessing paint effects output curve data. A MRenderLine provides access to paint effects curve rendering info. This class contains arrays for per vertex attributes along an individual curve: line: the vertices in worldspace twist: a twist vector at each vertice width: the tube widths flatness: the tube flatness along the twist vector direction parameter: the u parameterization value for each vertice color: color rgb value for each vertice incandescence: incandescence rgb value for each vertice transpareancy: transparency rgb value for each vertice

**Public Member Functions**

- `MRenderLine ()`  
  The class constructor.
- `~MRenderLine ()`  
  The class destructor.
- `MRenderLine & operator= (const MRenderLine &other)`  
  Assignment operator.
- `MVectorArray getLine () const`  
  Return the array of vertices along the curve.
- `MVectorArray getTwist () const`  
  Return the array of twist vectors along the curve.
- `MDoubleArray getWidth () const`  
  Return the array of tube widths along the curve.
- `MDoubleArray getFlatness () const`  
  Return the array of flatness along the curve.
- `MDoubleArray getParameter () const`  
  Return the array of parameter along the curve.
- `MVectorArray getColor () const`  
  Return the array of colors along the curve.
- `MVectorArray getIncandescence () const`  
  Return the array of incandescence along the curve.
- `MVectorArray getTransparency () const`  
  Return the array of transparency along the curve.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MRenderLineArray

Module `OpenMayaFX`. Source: [class_m_render_line_array.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_render_line_array.html).

Class for accessing paint effects output curve data. A MRenderLineArray provides access to paint effects curve rendering info. This class contains arrays for per vertex attributes along an individual curve: line: the vertices in worldspace twist: a twist vector at each vertex width: the tube widths flatness: the tube flatness along the twist vector direction parameter: the u parameterization value for each vertex color: color rgb value for each vertex incandescence: incandescence rgb value for each vertex transpareancy: transparency rgb value for each vertex

**Public Member Functions**

- `MRenderLineArray ()`  
  The class constructor.
- `~MRenderLineArray ()`  
  The class destructor.
- `MRenderLineArray & operator= (const MRenderLineArray &other)`  
  Assignment operator.
- `void deleteArray ()`  
  Free up the memory held in the render line array.
- `int length () const`  
  Return the number of entries in the array.
- `MRenderLine renderLine (unsigned int index, MStatus *status) const`  
  Return the render line at the defined index.

**Static Public Member Functions**

- `static const char * className ()`  
  Returns the name of this class.


### MnCloth

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_cloth.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_cloth.html).

Class for wrapping N cloth objects. This class wraps the internal Maya representation of N cloth objects suitable for use with the Nucleus solver.

**Public Member Functions**

- `MnCloth ()`  
  Default constructor.
- `~MnCloth () override`  
  The class destructor.
- `MStatus createNCloth ()`  
  Creates the underlying Maya TnCloth and sets this class to wrap it.
- `MStatus setTopology (const int numFaces, int *numVertsPerFace, int *faces, const int numEdges, int *edges)`  
  sets the topology of the underlying N Object.
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`  
  Sets the positions of the vertices of the underlying N cloth object.
- `MStatus setVelocities (const MFloatPointArray &velocities)`  
  Sets the velocities of the vertices of the underlying Ncloth object.
- `MStatus setBendRestAngleFromPositions (const MFloatPointArray &positions)`  
  Sets the the bend rest angle from the list of positions for the underlying N Object This sets the shape that bend resistance is trying to achieve.
- `MStatus setLinksRestLengthFromPositions (const MFloatPointArray &positions)`  
  Sets the the rest length from the list of positions for the underlying N Object.
- `MStatus setInputMeshAttractPositions (const MFloatPointArray &positions)`  
  Sets the positions for input mesh attract.
- `MStatus setInputMeshAttractDamping (float damping)`  
  Defines how springy the effect of Input Mesh Attract is.
- `MStatus setInputMeshAttractAndRigidStrength (float *inputAttractArray, float *rigidArray, float *deformArray)`  
  Sets on a per particle basis, the amount by which each particle is affected by the input mesh attract, rigidity, and deformability.
- `MStatus setComputeRestLength (const bool b)`  
  Sets whether rest lengths will be automatically computed, or overridden manually.
- `MStatus setComputeRestAngles (const bool b)`  
  Sets whether rest angles will be automatically computed, or overridden manually.
- `MStatus setThickness (float radius)`  
  sets a radius on each point of the mesh for collision purposes.
- `MStatus setThickness (float *radius)`  
  sets a radius (thickness) on a per point basis for the mesh for collision purposes.
- `MStatus setInverseMass (float invMass)`  
  sets the mass for every point in this mesh
- `MStatus setInverseMass (float *invMass)`  
  sets the mass on a per point basis in this mesh
- `MStatus setBounce (float bounce)`  
  sets the bounce for every point in this mesh
- `MStatus setBounce (float *bounce)`  
  sets the bounce on a per point basis in this mesh
- `MStatus setFriction (float friction)`  
  sets the friction for every point in this mesh
- `MStatus setFriction (float *friction)`  
  sets the friction on a per point basis for this mesh
- `MStatus setDamping (float damping)`  
  sets the damping for every point in this mesh
- `MStatus setDamping (float *damping)`  
  sets the on a per point basis in this mesh
- `MStatus setSelfCollisionFlags (const bool vertToVert, const bool vertToEdge=false, const bool vertToFace=false, const bool edgeToEdge=false, const bool edgeToFace=false)`  
  Sets how (or if) this object will collide with itself.
- `MStatus setCollisionFlags (const bool vertToVert, const bool edgeToEdge=false, const bool faceToFace=false)`  
  Sets how (or if) this object will collide with other objects.
- `MStatus setDisableGravity (const bool b)`  
  Sets whether gravity will affect this object.
- `MStatus setMaxIterations (const int it)`  
  Sets the number of iterations the solver will perform on various dynamic properties like drag, damping, stretch and bend.
- `MStatus setMaxSelfCollisionIterations (const int it)`  
  Sets the number of iterations the solver will perform for self collisions on this object.
- `MStatus setSelfCollisionSoftness (float softness)`  
  This allows one to lower the repulsive force of self collisions such that some interpenetration within the collide width is allowed.
- `MStatus setSelfTrappedCheck (bool on)`  
  This tracks self collision crossovers and attempts to push the crossed over points back.
- `MStatus setSelfCrossoverPush (float val)`  
  See node documentation.
- `MStatus setDragAndLift (float drag, float lift)`  
  Sets the drag and lift values for the cloth.
- `MStatus setTangentialDrag (float tangentialDrag)`  
  Sets the tangential drag values for the cloth.
- `MStatus setStartPressure (float startPressure)`  
  With the volume tracking pressure method this defines the relative air pressure inside the object at the startframe.
- `MStatus setSealHoles (bool seal)`  
  When the volume tracking pressure method is used this determines if physical holes in the cloth model are treated as being capped or not.
- `MStatus setPressure (float pressure)`  
  Sets the pressure within the cloth.
- `MStatus setTrackVolume (bool track)`
- `MStatus setIncompressibility (float v)`  
  When the volume tracking pressure model is used this defines how incompressible the internal volume of fluid is.
- `MStatus setPressureDamping (float damp)`  
  Sets the damping value for pressure.
- `MStatus setPumpRate (float pump)`  
  Defines the rate at which air pressure is added to the object.
- `MStatus setAirTightness (float airTightness)`  
  Defines the rate at which air can escape from the object, or how permiable the surface is.
- `MStatus setAddCrossLinks (bool addCrossLinks)`  
  For faces with more than 3 vertices this will create additional stretch and bends links such that each vertex is connected to each other vertex.
- `MStatus setBendAngleDropoff (float dropoff)`  
  Defines the way bend resistance changes with the angle.
- `MStatus setShearResistance (float resistance)`  
  Sets the shear resistance.
- `MStatus setStretchAndCompressionResistance (float stretchResist, float compressionResist)`  
  Stretch Resistance:
- `MStatus setStretchAndCompressionResistance (float *stretchResist, float *compressionResist)`  
  Sets the stretch and compression resistance for the mesh on a per vertex basis.
- `MStatus setBendAngleScale (float scale)`  
  Defines the amount by which the rest state of the bend angle is scaled.
- `MStatus setRestitutionAngle (float angle)`  
  Defines how far we can bend across an edge before it will fail to go back to the rest angle when there are no forces acting on the cloth.
- `MStatus setRestitutionTension (float tension)`  
  How far can the links be stretched before they fail to go back to their rest length when there are no forces acting on the cloth.
- `MStatus setSelfCollideWidth (float width)`  
  Sets the self collision width.
- `MStatus setBendResistance (float strength)`  
  Bend resistance measures the amount of attraction to the restAngle, which is defined between cvs on either side of an edge.
- `MStatus getNumVertices (unsigned int &numVerts) const`  
  Returns the number of vertices in the underlying n Cloth.
- `MStatus getPositions (MFloatPointArray &positions) const`  
  gets the positions of the points of the underlying N Object.
- `MStatus getVelocities (MFloatPointArray &velocities) const`  
  gets the velocities of the points of the underlying N cloth object.
- `MStatus getThickness (MFloatArray &radius) const`  
  gets the thickness at each point of the underlying N cloth object.
- `MStatus getInverseMass (MFloatArray &inverseMass) const`  
  gets the inverseMass at each point of the underlying N cloth object.
- `MStatus getBounce (MFloatArray &bounce) const`  
  gets the bounce at each point of the underlying N cloth object.
- `MStatus getFriction (MFloatArray &friction) const`  
  gets the friction at each point of the underlying N cloth object.

**Protected Member Functions**

- `void setNCloth (TnCloth *nObj, bool own)`  
  Sets the TnCloth pointer which this class wraps.


### MnObject

Module `OpenMayaFX`. Source: [class_mn_object.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_object.html).

Class for wrapping N cloth objects. This class wraps the internal Maya representation of N cloth objects suitable for use with the Nucleus solver.

**Public Member Functions**

- `MnObject ()`  
  Default constructor.
- `virtual ~MnObject ()`  
  The class destructor.

**Protected Member Functions**

- `void setNObject (TnObject *nObj, bool own)`  
  Sets the TnObject pointer which this class wraps.


### MnParticle

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_particle.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_particle.html).

Class for wrapping N cloth objects. This class wraps the internal Maya representation of N cloth objects suitable for use with the Nucleus solver.

**Public Member Functions**

- `MnParticle ()`  
  Default constructor.
- `~MnParticle () override`  
  The class destructor.
- `MStatus createNParticle ()`  
  Creates the underlying Maya TnParticle and sets this class to wrap it.
- `MStatus setTopology (const int numPoints)`  
  Sets the topology of the underlying N Object.
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`  
  Sets the positions of the vertices of the underlying nParticle object.
- `MStatus setVelocities (const MFloatPointArray &velocities)`  
  Sets the velocities of the vertices of the underlying nParticle object.
- `MStatus setThickness (float radius)`  
  sets a radius on each point collision purposes.
- `MStatus setThickness (float *radius)`  
  sets a radius (thickness) on a per point basis for collision purposes.
- `MStatus setInverseMass (float invMass)`  
  sets the mass for every particle
- `MStatus setInverseMass (float *invMass)`  
  sets the mass on a per point basis
- `MStatus setBounce (float bounce)`  
  sets the bounce for every particle
- `MStatus setBounce (float *bounce)`  
  sets the bounce on a per point basis for these particles
- `MStatus setFriction (float friction)`  
  sets the friction for every particle
- `MStatus setFriction (float *friction)`  
  sets the friction on a per point basis for these particles
- `MStatus setDamping (float damping)`  
  sets the damping for every particle
- `MStatus setDamping (float *damping)`  
  sets the damping on a per point basis for these particles
- `MStatus setDisableGravity (const bool b)`  
  Sets whether gravity will affect this object.
- `MStatus setLiquidSimulation (const bool b)`  
  Sets whether this object will solve as a liquid.
- `MStatus setIncompressibility (float incompressibility)`  
  sets the incompressibility
- `MStatus setRestDensity (float restDensity)`  
  sets the rest density for every particle
- `MStatus setLiquidRadiusScale (float liquidRadiusScale)`  
  sets the liquidRadiusScale for every particle
- `MStatus setViscosity (float viscosity)`  
  sets the viscosity for every particle
- `MStatus setViscosity (float *viscosity)`  
  sets the viscosity on a per point basis for these particles
- `MStatus setSurfaceTension (float surfaceTension)`  
  sets the surfaceTension for every particle
- `MStatus setSurfaceTension (float *surfaceTension)`  
  sets the surfaceTension on a per point basis for these particles
- `MStatus setMaxIterations (const int it)`  
  Sets the number of iterations the solver will perform on various dynamic properties like drag, damping, stretch and bend.
- `MStatus setMaxSelfCollisionIterations (const int it)`  
  Sets the number of iterations the solver will perform for self collisions on this object.
- `MStatus setSelfCollisionSoftness (float softness)`  
  This allows one to lower the repulsive force of self collisions such that some interpenetration within the collide width is allowed.
- `MStatus setDragAndLift (float drag, float lift)`  
  Sets the drag and lift values for the nParticle.
- `MStatus setCollide (const bool b)`  
  Sets whether collisions will affect this object.
- `MStatus setSelfCollide (const bool b)`  
  Sets whether self collisions will affect this object.
- `MStatus setSelfCollideWidth (float width)`  
  Sets the self collision width.
- `MStatus getNumVertices (unsigned int &numVerts) const`  
  Returns the number of vertices in the underlying nParticle.
- `MStatus getPositions (MFloatPointArray &positions) const`  
  gets the positions of the points of the underlying N Object.
- `MStatus getVelocities (MFloatPointArray &velocities) const`  
  gets the velocities of the points of the underlying nParticle object.
- `MStatus getThickness (MFloatArray &radius) const`  
  gets the radii of the points of the underlying N particle object.
- `MStatus getInverseMass (MFloatArray &inverseMass) const`  
  gets the inverseMass of the points of the underlying N particle object.
- `MStatus getBounce (MFloatArray &bounce) const`  
  gets the Bounce of the points of the underlying N particle object.
- `MStatus getFriction (MFloatArray &friction) const`  
  gets the friction of the points of the underlying N particle object.

**Protected Member Functions**

- `void setNParticle (TnParticle *nObj, bool own)`  
  Sets the TnParticle pointer which this class wraps.


### MnRigid

Module `OpenMayaFX`. Inherits `MnObject`. Source: [class_mn_rigid.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_rigid.html).

Class for wrapping N cloth objects. This class wraps the internal Maya representation of N cloth objects suitable for use with the Nucleus solver.

**Public Member Functions**

- `MnRigid ()`  
  Default constructor.
- `~MnRigid () override`  
  The class destructor.
- `MStatus setTopology (const int numFaces, int *numVertsPerFace, int *faces, const int numEdges, int *edges)`  
  sets the topology of the underlying N Object.
- `MStatus createNRigid ()`  
  Creates the underlying Maya TnRigid and sets this class to wrap it.
- `MStatus setPositions (const MFloatPointArray &positions, bool startFrame=true)`  
  Sets the positions of the vertices of the underlying nRigid object.
- `MStatus setVelocities (const MFloatPointArray &velocities)`  
  Sets the velocities of the vertices of the underlying nRigid object.
- `MStatus setThickness (float radius)`  
  sets a radius on each point of the mesh for collision purposes.
- `MStatus setThickness (float *radius)`  
  sets a radius (thickness) on a per point basis for the mesh for collision purposes.
- `MStatus setBounce (float bounce)`  
  sets the bounce for every point in this mesh
- `MStatus setBounce (float *bounce)`  
  sets the bounce on a per point basis in this mesh
- `MStatus setFriction (float friction)`  
  sets the friction for every point in this mesh
- `MStatus setFriction (float *friction)`  
  sets the friction on a per point basis for this mesh
- `MStatus setCollisionFlags (const bool vertToVert, const bool edgeToEdge=false, const bool faceToFace=false)`  
  Sets how (or if) this object will collide with other objects.
- `MStatus getNumVertices (unsigned int &numVerts) const`  
  Returns the number of vertices in the underlying nRigid.
- `MStatus getPositions (MFloatPointArray &positions) const`  
  gets the positions of the points of the underlying N Object.
- `MStatus getVelocities (MFloatPointArray &velocities) const`  
  gets the velocities of the points of the underlying nRigid object.
- `MStatus getThickness (MFloatArray &radius) const`  
  gets the thickness at each point of the underlying N rigid object.
- `MStatus getInverseMass (MFloatArray &inverseMass) const`  
  gets the inverseMass at each point of the underlying N rigid object.
- `MStatus getBounce (MFloatArray &bounce) const`  
  gets the bounce at each point of the underlying N rigid object.
- `MStatus getFriction (MFloatArray &friction) const`  
  gets the friction at each point of the underlying N rigid object.

**Protected Member Functions**

- `void setNRigid (TnRigid *nObj, bool own)`  
  Sets the TnRigid pointer which this class wraps.


### MnSolver

Module `OpenMayaFX`. Source: [class_mn_solver.html](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_mn_solver.html).

Class for wrapping N solver objects. This class wraps the internal Maya representation of a Nucleus solver suitable for use with the Nucleus objects such as nCloth or nParticles

**Public Member Functions**

- `MnSolver ()`  
  Default constructor.
- `virtual ~MnSolver ()`  
  The class destructor.
- `MStatus createNSolver ()`  
  Creates the underlying Maya solver object and sets this class to wrap it.
- `MStatus addNObject (MnObject *obj)`  
  Assign the nucleus object to be solved by this solver.
- `MStatus removeNObject (MnObject *obj)`  
  Remove the nucleus object from being solved by this solver.
- `MStatus setGravity (float grav)`  
  Sets the gravity magnitude for the objects being solved.
- `MStatus setGravityDir (float gravX, float gravY, float gravZ)`  
  Sets the gravity direction for the underlying Maya solver object.
- `MStatus setAirDensity (float dens)`  
  Sets the air Density for the solver.
- `MStatus setWindSpeed (float speed)`  
  Sets the wind magnitude for the underlying Maya solver object.
- `MStatus setWindDir (float windX, float windY, float windZ)`  
  Sets the wind direction for the underlying Maya solver object.
- `MStatus setWindNoiseIntensity (float noise)`  
  Sets the wind noise intensity for the underlying Maya solver object.
- `MStatus setDisabled (bool disabled)`  
  Disables the solver - or re-enables it again.
- `MStatus setSubsteps (int substeps)`  
  Set the number of substeps used by the solver.
- `MStatus setMaxIterations (int maxIter)`  
  Set the max number of collision iterations used by the solver.
- `MStatus setStartTime (float startTime)`  
  Sets the start Time in seconds for the solver.
- `MStatus makeAllCollide ()`  
  Allow all the objects assigned to the Maya solver object to collide.
- `MStatus removeAllCollisions ()`  
  Remove the collisions between all the objects assigned to the Maya solver object.
- `MStatus solve (float solveTime)`  
  Solve from the last eval time until the specified time (in seconds).

