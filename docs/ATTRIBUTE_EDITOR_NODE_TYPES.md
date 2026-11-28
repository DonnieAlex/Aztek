# Attribute Editor node types

The node types Aztek exposes, and whether the Attribute Editor has a template of its own for each. Generated 25/11/2026 from `ls -nodeTypes` and the type names in `app/src/attreditor.cpp`.

## Counts

- 685 node types in all.
- 80 have a template of their own: the reference's sections and order for that family.
- 605 show the generic page: one section named after the type, holding the node's real attributes in one list. Every attribute still reads and writes; what is missing is the reference's grouping and order.

## With a template of their own

    aiAreaLight, aiMeshLight, aiPhotometricLight, aiSkyDomeLight, aiStandardSurface, aimConstraint, ambientLight, anisotropic, annotationShape, areaLight, baseLattice, blendShape, blinn, camera, cloth, cluster, clusterHandle, deformBend, deformFlare, deformSine, deformSquash, deformTwist, deformWave, directionalLight, distanceDimShape, emitter, file, follicle, geometryConstraint, ikEffector, ikHandle, imagePlane, implicitSphere, joint, lambert, lattice, leather, locator, max, mesh, min, motionTrailShape, nonLinear, normalConstraint, normalize, nurbsCurve, nurbsSurface, openPBRSurface, orientConstraint, parentConstraint, phong, phongE, place2dTexture, pointConstraint, pointLight, pointOnPolyConstraint, poleVectorConstraint, polyCone, polyCube, polyCylinder, polyPlane, polySphere, polyTorus, psdFileTex, rigidBody, scaleConstraint, sculpt, shadingEngine, shrinkWrap, skinCluster, softMod, softModHandle, spotLight, standardSurface, surfaceShader, tangentConstraint, transform, volumeLight, wire, wrap

## Generic page, by name family

Grouped by the names they share, not by the reference's own families. The grouping is for finding a name, not a claim about how the reference lays the family out.

### Animation and rigging (61)

    animBlendNodeAdditive, animBlendNodeAdditiveDA, animBlendNodeAdditiveDL, animBlendNodeAdditiveF, animBlendNodeAdditiveFA, animBlendNodeAdditiveFL, animBlendNodeAdditiveI16, animBlendNodeAdditiveI32, animBlendNodeAdditiveRotation, animBlendNodeAdditiveScale, animBlendNodeBoolean, animBlendNodeEnum, animBlendNodeTime, animCurve, animCurveTA, animCurveTL, animCurveTT, animCurveTU, animCurveUA, animCurveUL, animCurveUT, animCurveUU, animLayer, blendWeighted, character, characterMap, characterOffset, clipChannel, clipLibrary, clipScheduler, combinationShape, componentFalloff, componentMatch, componentTagBase, condition, controller, dagPose, deltaMush, expression, falloffEval, filter, filterClosestSample, filterEuler, filterResample, filterSimplify, holdMatrix, ikRPsolver, ikSCsolver, ikSolve, ikSplineSolver, ikSystem, jointCluster, jointLattice, pairBlend, poseInterpolator, time, timeFunction, timeSliderBookmark, timeToUnitConversion, timeWarp, wtAddMatrix

### Geometry and deformers (212)

    alignCurve, alignSurface, arcLengthDimension, attachCurve, attachSurface, bevel, bevelModifier, bevelPlus, bezierCurveToNurbs, blendColors, blendFalloff, blendMatrix, blendTwoAttr, boneLattice, booleanModifier, boundary, closeCurve, closeSurface, closestPointOnMesh, closestPointOnSurface, createColorSet, createUVSet, curveFromSurfaceCoS, curveFromSurfaceIso, curveInfo, curveIntersect, curveWarp, decimateModifier, deformFunc, detachCurve, detachSurface, doubleProfileBirailSurface, dropoffLocator, extendCurve, extendSurface, extrude, extrudeModifier, ffBlendSrf, ffFilletSrf, ffd, filletCurve, flow, geomBind, geometryFilter, globalStitch, insertKnotCurve, insertKnotSurface, intersectSurface, loft, meshSource, multiProfileBirailSurface, multiply, multiplyDL, multiplyDivide, multiplyPointByMatrix, multiplyPointByMatrixDL, multiplyVectorByMatrix, multiplyVectorByMatrixDL, nurbsCurveToBezier, nurbsTessellate, nurbsToPoints, offsetCurve, offsetCurveOnSurface, offsetSurface, planarTrim, plane, polyAppend, polyAppendVertex, polyAutoProj, polyAverageColor, polyAverageNormal, polyAverageVertex, polyAxis, polyBevel3, polyBevelFace, polyBlendColor, polyBlindData, polyBoolOp, polyBridgeEdge, polyBridgeFace, polyCBoolOp, polyCapsule, polyChamfer, polyChipOff, polyCircularize, polyClean, polyCloseBorder, polyCollapseEdge, polyCollapseF, polyColorBlindData, polyColorDel, polyColorMod, polyColorPerVertex, polyConform, polyConnectComponents, polyConnectEdge, polyConnectVertex, polyContourProj, polyCopyUV, polyCreaseEdge, polyCreateFace, polyCut, polyCylProj, polyDelEdge, polyDelFacet, polyDelVertex, polyDisc, polyDissolveEdge, polyDissolveVertex, polyDuplicateEdge, polyDuplicateFaces, polyEdgeToCurve, polyEditEdgeFlow, polyExtrudeEdge, polyExtrudeFace, polyExtrudeVertex, polyFlipEdge, polyFlipUV, polyForceUV, polyGeoSampler, polyGridFillEdge, polyGridFillFace, polyGridFillVertex, polyHelix, polyHole, polyIcoSphere, polyInset, polyLayoutUV, polyMapCut, polyMapDel, polyMapSew, polyMapSewMove, polyMergeCenter, polyMergeEdge, polyMergeFace, polyMergeUV, polyMergeVert, polyMergeVertAll, polyMirror, polyModifier, polyMoveEdge, polyMoveFace, polyMoveUV, polyMoveVertex, polyNormal, polyNormalPerVertex, polyNormalizeUV, polyOffsetLoop, polyOptUvs, polyPipe, polyPlanarProj, polyPlatonicSolid, polyPoke, polyPrimitive, polyPrism, polyProjectCurve, polyPyramid, polyQuad, polyQuadSphere, polyReduce, polyRemesh, polyRetopo, polySeparate, polySetToFaceNormal, polySewEdge, polySlideEdge, polySmartExtrude, polySmoothFace, polySoccerBall, polySoftEdge, polySphProj, polySpinEdge, polySplit, polySplitEdge, polySplitRing, polySplitRingsEdge, polySplitRingsFace, polySplitVert, polyStraightenUVBorder, polySubdEdge, polySubdFace, polySuperShape, polyTransfer, polyTriangulate, polyTweak, polyTweakUV, polyUVRectangle, polyUnite, polyUnlockNormals, polyUnsmooth, polyWedgeFace, polyWeightedNormals, projectCurve, projectTangent, rebuildCurve, rebuildSurface, revolve, round, roundConstantRadius, roundDL, smoothCurve, smoothStep, smoothStepDL, smoothTangentSrf, squareSrf, stitchSrf, subdivisionModifier, surfaceInfo, sweepMeshCreator, transferAttributes, transferFalloff, trim

### Harold's ai nodes (16)

    aiAOV, aiAOVDriver, aiAOVFilter, aiAmbientOcclusion, aiAtmosphereVolume, aiBump2d, aiCurvature, aiFlat, aiImage, aiMixShader, aiNormalMap, aiOptions, aiShadowMatte, aiStandardHair, aiUtility, aimMatrix

### USD and plugin shapes (7)

    AL_usdmaya_ProxyShape, AlembicNode, CustomRigRetargeterNode, ProxyUsdGeomCamera, TdnAffect, Unfold3DUnfold, arubaTessellate

### Humanoid IK (13)

    HIKCharacterNode, HIKControlSetNode, HIKEffector2State, HIKFK2State, HIKPinning2State, HIKProperty2State, HIKRetargeterNode, HIKSK2State, HIKSolverNode, HIKState2Effector, HIKState2FK, HIKState2GlobalSK, HIKState2SK

### Dynamics, hair and cloth (8)

    collider, constraint, dynamicConstraint, field, hairSystem, particles, solver, uniformFalloff

### Textures (23)

    brownian, bulge, checker, cloud, crater, fractal, granite, grid, marble, mountain, noise, place3dTexture, ramp, rampShader, rock, snow, solidFractal, stucco, textureDeformer, textureDeformerHandle, volumeNoise, water, wood

### Shading (20)

    bump2d, bump3d, clamp, clampRange, clampRangeDL, contrast, displacementShader, dof, gammaCorrect, layeredShader, layeredTexture, light, lightLinker, lightList, luminance, materialInfo, remapColor, remapHsv, remapValue, shaderGlow

### General and math utilities (120)

    absolute, absoluteDL, acos, acosDL, addDL, addDoubleLinear, addMatrix, and, angleBetween, angleBetweenDL, arrayModifier, asin, asinDL, atan, atan2, atan2DL, atanDL, average, averageDL, axisFromMatrix, axisFromMatrixDL, blindDataTemplate, ceil, ceilDL, choice, colorConstant, colorManagementGlobals, columnFromMatrix, columnFromMatrixDL, composeMatrix, cos, cosDL, crossProduct, crossProductDL, decomposeMatrix, determinant, determinantDL, distanceBetween, distanceBetweenDL, divide, divideDL, dotProduct, dotProductDL, equal, equalDL, eulerToQuat, floatCondition, floatConstant, floatLogic, floatMath, floor, floorDL, fourByFourMatrix, greaterThan, greaterThanDL, hsvToRgb, inverseLerp, inverseLerpDL, inverseMatrix, length, lengthDL, lessThan, lessThanDL, log, logDL, maxDL, minDL, multDL, multDoubleLinear, multMatrix, nearestPointOnCurve, normalizeDL, not, or, orientationMarker, plusMinusAverage, pointCurveConstraint, pointMatrixMult, pointMatrixMultDL, pointOnCurve, pointOnCurveInfo, pointOnSurfaceInfo, pointsToNurbs, power, powerDL, quadShadingSwitch, quatAdd, quatConjugate, quatInvert, quatMultiply, quatNegate, quatNormalize, quatProd, quatSlerp, quatSub, quatToEuler, reverse, reverseCurve, reverseSurface, rgbToHsv, rowFromMatrix, rowFromMatrixDL, sin, sinDL, singleProfileBirailSurface, singleShadingSwitch, subSurface, subsetFalloff, subtract, subtractDL, sum, sumDL, tan, tanDL, transposeMatrix, tripleShadingSwitch, unitConversion, unitToTimeConversion, vectorProduct, weightedNormalsModifier

### Scene, layers and defaults (37)

    audio, container, dagNode, defaultLightList, defaultRenderUtilityList, defaultRenderingList, defaultShaderList, defaultTextureList, displayLayer, displayLayerManager, dynController, globalCacheControl, groupId, groupParts, hardwareRenderingGlobals, hwRenderGlobals, hyperGraphInfo, hyperLayout, objectSet, partition, postProcessList, reference, renderGlobals, renderGlobalsList, renderLayer, renderLayerManager, renderLight, renderQuality, script, sequenceManager, setRange, strokeGlobals, unknown, unknownDag, unknownTransform, uvPin, viewColorManager

### Other (88)

    autoBevelModifier, doubleShadingSwitch, fitBspline, hardenPointCurve, heldNode, illustratorCurves, jiggle, keyingGroup, lerp, lerpDL, makeNurbCircle, makeNurbCone, makeNurbCube, makeNurbCylinder, makeNurbPlane, makeNurbSphere, makeNurbTorus, makeNurbsSquare, makeTextCurves, makeThreePointCircularArc, makeTwoPointCircularArc, mayaUsdProxyShape, mayaUsdProxyShapeBase, mia_material, mia_material_x, mia_material_x_passes, mirrorModifier, mlDeformer, modulo, moduloDL, morph, motionPath, motionTrail, mute, negate, negateDL, network, nonAmbientLightShapeNode, nonExtendedLightShapeNode, paramDimension, parentMatrix, particleCloud, pi, pickMatrix, positionMarker, primitiveFalloff, proximityFalloff, proximityPin, proximityWrap, rbfSrf, reflect, resolution, rotateVector, rotateVectorDL, rotationFromMatrix, sampler, samplerInfo, scaleFromMatrix, scaleFromMatrixDL, shape, snapshot, solidify, solidifyModifier, splitRingsModifier, symmetryConstraint, tension, transformGeometry, translationFromMatrix, translationFromMatrixDL, triangulateModifier, truncate, truncateDL, tweak, untrim, wrinkle, xgmCurveToSpline, xgmModifierClump, xgmModifierCollide, xgmModifierCut, xgmModifierDisplace, xgmModifierGuide, xgmModifierLinearWire, xgmModifierNoise, xgmModifierScale, xgmModifierSculpt, xgmModifierSplineCache, xgmSplineBase, xgmSplineDescription

