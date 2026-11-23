# Plug-ins the plug-in kit is tested with (crates/aztek_host/tests/shipped_plugins.rs, mgear_support.rs), built from
# their own source with Aztek's plug-in kit. They don't ship with Aztek: Help > Install mGear installs mGear's, built
# for Aztek, from github.com/srlegrand/mgear. Each folder holds the source as its project publishes it, with its
# licence; THIRD_PARTY.md says where each came from.
#
#   mgear_solvers     mGear's solver nodes (mgear_curveCns, mgear_ikfk2Bone, mgear_matrixConstraint ...), from
#                     mGear 5's src/ (github.com/mgear-dev/mgear), unchanged
#   mGearWeightDriver mGear's RBF and vector angle node (github.com/mgear-dev/mgear), unchanged
#   cvwrap            the cvWrap deformer and command, mGear's copy (github.com/mgear-dev/cvwrap), unchanged
#
# They are written to plug-ins/ in the build folder.

find_package(AztekDevkit REQUIRED CONFIG PATHS "${CMAKE_CURRENT_SOURCE_DIR}/../devkit/cmake" NO_DEFAULT_PATH)

set(AZTEK_SHIPPED_PLUGIN_DIR "${CMAKE_BINARY_DIR}/plug-ins")

function(aztek_shipped_plugin name)
  aztek_add_plugin(${name} ${ARGN})
  set_target_properties(${name} PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY "${AZTEK_SHIPPED_PLUGIN_DIR}"
    LIBRARY_OUTPUT_DIRECTORY_DEBUG "${AZTEK_SHIPPED_PLUGIN_DIR}"
    LIBRARY_OUTPUT_DIRECTORY_RELEASE "${AZTEK_SHIPPED_PLUGIN_DIR}"
    LIBRARY_OUTPUT_DIRECTORY_RELWITHDEBINFO "${AZTEK_SHIPPED_PLUGIN_DIR}")
endfunction()

file(GLOB MGEAR_SOLVERS_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/mgear_solvers/*.cpp")
aztek_shipped_plugin(mgear_solvers SOURCES ${MGEAR_SOLVERS_SOURCES})


file(GLOB MGEAR_WEIGHT_DRIVER_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/mGearWeightDriver/*.cpp")
aztek_shipped_plugin(mGearWeightDriver SOURCES ${MGEAR_WEIGHT_DRIVER_SOURCES})

file(GLOB CVWRAP_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/cvwrap/*.cpp")
aztek_shipped_plugin(cvwrap SOURCES ${CVWRAP_SOURCES})
