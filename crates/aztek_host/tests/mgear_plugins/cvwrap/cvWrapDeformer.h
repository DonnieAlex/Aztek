#ifndef CVWRAPDEFORMER_H
#define CVWRAPDEFORMER_H

#include <maya/MDGModifier.h>
#include <maya/MFloatArray.h>
#include <maya/MIntArray.h>
#include <maya/MMatrix.h> 
#include <maya/MMatrixArray.h> 
#include <maya/MMessage.h>
#include <maya/MPoint.h> 
#include <maya/MThreadPool.h>
#include <maya/MPxDeformerNode.h>


#include <map>
#include <vector>
#include "common.h"

struct TaskData {
  MMatrix drivenMatrix;
  MMatrix drivenInverseMatrix;
  float envelope;
  float scale;

  MIntArray membership;
  MFloatArray paintWeights;
  MPointArray points;

  MPointArray driverPoints;
  MFloatVectorArray driverNormals;
  MMatrixArray bindMatrices;
  std::vector<MIntArray> sampleIds;
  std::vector<MDoubleArray> sampleWeights;
  std::vector<MIntArray> triangleVerts;
  std::vector<BaryCoords> baryCoords;
};
 

class CVWrap : public MPxDeformerNode {
 public:
  CVWrap();
  virtual ~CVWrap(); 
  virtual void postConstructor();
  virtual MStatus deform(MDataBlock& data, MItGeometry& iter, const MMatrix& mat,
                         unsigned int mIndex);
  virtual MStatus setDependentsDirty(const MPlug& plugBeingDirtied, MPlugArray& affectedPlugs);

  static void* creator();
  static MStatus initialize();


  /**
    Distributes the ThreadData objects to the parallel threads.
    @param[in] data The user defined data.  In this case, the ThreadData array.
    @param[in] pRoot Maya's root task.
  */
  static void CreateTasks(void *data, MThreadRootTask *pRoot);
  static MThreadRetVal EvaluateWrap(void *pParam);
    
  const static char* kName;  /**< The name of the node. */
  static MObject aBindDriverGeo;
  static MObject aDriverGeo;
  static MObject aBindData;
  static MObject aSampleComponents;
  static MObject aSampleWeights;
    /** The vertex indices of the triangle containing the origin of each coordinate system. */
  static MObject aTriangleVerts;
  /** The indices of the tangents used to calculate the up vector. */
  static MObject aBarycentricWeights;

  static MObject aBindMatrix;
  static MObject aNumTasks;
  static MObject aScale;
  static MTypeId id;

private:
  static void aboutToDeleteCB(MObject &node, MDGModifier &modifier, void *clientData);

  std::map<unsigned int, bool> dirty_;
  std::vector<TaskData> taskData_;  /**< Per geometry evaluation data. */
  std::vector<ThreadData<TaskData>*> threadData_;
  MCallbackId onDeleteCallbackId;
};




#endif
