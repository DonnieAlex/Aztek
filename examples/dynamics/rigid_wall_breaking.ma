//Maya ASCII 2024 scene
//Name: rigid_wall_breaking.ma
//Codeset: UTF-8
requires maya "2024";
requires "aztekDynamics" "1.0";
currentUnit -l centimeter -a degree -t film;
fileInfo "application" "aztek";
fileInfo "version" "1.0.0";
createNode transform -s -n "persp";
	setAttr ".v" no;
	setAttr ".t" -type "double3" 1600 900 -2200 ;
	setAttr ".r" -type "double3" 166.5614487428559 36.02737338510361 180 ;
createNode camera -s -n "perspShape" -p "persp";
	setAttr ".coi" 2796.873254189399;
	setAttr ".imn" -type "string" "persp";
	setAttr ".den" -type "string" "persp_depth";
	setAttr ".man" -type "string" "persp_mask";
	setAttr ".hc" -type "string" "viewSet -p %camera";
createNode transform -s -n "top";
	setAttr ".v" no;
	setAttr ".t" -type "double3" 0 1000.1 0 ;
	setAttr ".r" -type "double3" -90 0 0 ;
createNode camera -s -n "topShape" -p "top";
	setAttr ".coi" 1000.1;
	setAttr ".o" yes;
	setAttr ".rnd" no;
	setAttr ".imn" -type "string" "top";
	setAttr ".den" -type "string" "top_depth";
	setAttr ".man" -type "string" "top_mask";
	setAttr ".hc" -type "string" "viewSet -t %camera";
createNode transform -s -n "front";
	setAttr ".v" no;
	setAttr ".t" -type "double3" 0 0 1000.1 ;
createNode camera -s -n "frontShape" -p "front";
	setAttr ".coi" 1000.1;
	setAttr ".o" yes;
	setAttr ".rnd" no;
	setAttr ".imn" -type "string" "front";
	setAttr ".den" -type "string" "front_depth";
	setAttr ".man" -type "string" "front_mask";
	setAttr ".hc" -type "string" "viewSet -f %camera";
createNode transform -s -n "side";
	setAttr ".v" no;
	setAttr ".t" -type "double3" 1000.1 0 0 ;
	setAttr ".r" -type "double3" 0 90 0 ;
createNode camera -s -n "sideShape" -p "side";
	setAttr ".coi" 1000.1;
	setAttr ".o" yes;
	setAttr ".rnd" no;
	setAttr ".imn" -type "string" "side";
	setAttr ".den" -type "string" "side_depth";
	setAttr ".man" -type "string" "side_mask";
	setAttr ".hc" -type "string" "viewSet -s %camera";
createNode transform -n "floor";
createNode mesh -n "floorShape" -p "floor";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 4 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:3]" -type "float2" 0 0 1 0 1 1 0 1 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 4 ".vt";
	setAttr ".vt[0:3]"  -4000 0 4000 4000 0 4000 -4000 0 -4000 4000 0 -4000;
	setAttr -s 4 ".ed";
	setAttr ".ed[0:3]"  0 1 1 1 3 1 3 2 1 2 0 1;
	setAttr -s 1 -ch 4 ".fc[0]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 ;
createNode transform -n "solver1";
createNode solver -n "solverShape1" -p "solver1";
	setAttr -k off ".v";
createNode transform -n "brick_0_0";
	setAttr ".t" -type "double3" -780 32.25 0 ;
createNode mesh -n "brick_0_Shape0" -p "brick_0_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_1";
	setAttr ".t" -type "double3" -660 32.25 0 ;
createNode mesh -n "brick_0_Shape1" -p "brick_0_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_2";
	setAttr ".t" -type "double3" -540 32.25 0 ;
createNode mesh -n "brick_0_Shape2" -p "brick_0_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_3";
	setAttr ".t" -type "double3" -420 32.25 0 ;
createNode mesh -n "brick_0_Shape3" -p "brick_0_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_4";
	setAttr ".t" -type "double3" -300 32.25 0 ;
createNode mesh -n "brick_0_Shape4" -p "brick_0_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_5";
	setAttr ".t" -type "double3" -180 32.25 0 ;
createNode mesh -n "brick_0_Shape5" -p "brick_0_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_6";
	setAttr ".t" -type "double3" -60 32.25 0 ;
createNode mesh -n "brick_0_Shape6" -p "brick_0_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_7";
	setAttr ".t" -type "double3" 60 32.25 0 ;
createNode mesh -n "brick_0_Shape7" -p "brick_0_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_8";
	setAttr ".t" -type "double3" 180 32.25 0 ;
createNode mesh -n "brick_0_Shape8" -p "brick_0_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_9";
	setAttr ".t" -type "double3" 300 32.25 0 ;
createNode mesh -n "brick_0_Shape9" -p "brick_0_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_10";
	setAttr ".t" -type "double3" 420 32.25 0 ;
createNode mesh -n "brick_0_Shape10" -p "brick_0_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_11";
	setAttr ".t" -type "double3" 540 32.25 0 ;
createNode mesh -n "brick_0_Shape11" -p "brick_0_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_12";
	setAttr ".t" -type "double3" 660 32.25 0 ;
createNode mesh -n "brick_0_Shape12" -p "brick_0_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_0_13";
	setAttr ".t" -type "double3" 780 32.25 0 ;
createNode mesh -n "brick_0_Shape13" -p "brick_0_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_0";
	setAttr ".t" -type "double3" -810 94.5 0 ;
createNode mesh -n "brick_1_Shape0" -p "brick_1_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_1";
	setAttr ".t" -type "double3" -720 94.5 0 ;
createNode mesh -n "brick_1_Shape1" -p "brick_1_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_2";
	setAttr ".t" -type "double3" -600 94.5 0 ;
createNode mesh -n "brick_1_Shape2" -p "brick_1_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_3";
	setAttr ".t" -type "double3" -480 94.5 0 ;
createNode mesh -n "brick_1_Shape3" -p "brick_1_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_4";
	setAttr ".t" -type "double3" -360 94.5 0 ;
createNode mesh -n "brick_1_Shape4" -p "brick_1_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_5";
	setAttr ".t" -type "double3" -240 94.5 0 ;
createNode mesh -n "brick_1_Shape5" -p "brick_1_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_6";
	setAttr ".t" -type "double3" -120 94.5 0 ;
createNode mesh -n "brick_1_Shape6" -p "brick_1_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_7";
	setAttr ".t" -type "double3" 0 94.5 0 ;
createNode mesh -n "brick_1_Shape7" -p "brick_1_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_8";
	setAttr ".t" -type "double3" 120 94.5 0 ;
createNode mesh -n "brick_1_Shape8" -p "brick_1_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_9";
	setAttr ".t" -type "double3" 240 94.5 0 ;
createNode mesh -n "brick_1_Shape9" -p "brick_1_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_10";
	setAttr ".t" -type "double3" 360 94.5 0 ;
createNode mesh -n "brick_1_Shape10" -p "brick_1_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_11";
	setAttr ".t" -type "double3" 480 94.5 0 ;
createNode mesh -n "brick_1_Shape11" -p "brick_1_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_12";
	setAttr ".t" -type "double3" 600 94.5 0 ;
createNode mesh -n "brick_1_Shape12" -p "brick_1_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_13";
	setAttr ".t" -type "double3" 720 94.5 0 ;
createNode mesh -n "brick_1_Shape13" -p "brick_1_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_1_14";
	setAttr ".t" -type "double3" 810 94.5 0 ;
createNode mesh -n "brick_1_Shape14" -p "brick_1_14";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_0";
	setAttr ".t" -type "double3" -780 156.75 0 ;
createNode mesh -n "brick_2_Shape0" -p "brick_2_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_1";
	setAttr ".t" -type "double3" -660 156.75 0 ;
createNode mesh -n "brick_2_Shape1" -p "brick_2_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_2";
	setAttr ".t" -type "double3" -540 156.75 0 ;
createNode mesh -n "brick_2_Shape2" -p "brick_2_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_3";
	setAttr ".t" -type "double3" -420 156.75 0 ;
createNode mesh -n "brick_2_Shape3" -p "brick_2_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_4";
	setAttr ".t" -type "double3" -300 156.75 0 ;
createNode mesh -n "brick_2_Shape4" -p "brick_2_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_5";
	setAttr ".t" -type "double3" -180 156.75 0 ;
createNode mesh -n "brick_2_Shape5" -p "brick_2_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_6";
	setAttr ".t" -type "double3" -60 156.75 0 ;
createNode mesh -n "brick_2_Shape6" -p "brick_2_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_7";
	setAttr ".t" -type "double3" 60 156.75 0 ;
createNode mesh -n "brick_2_Shape7" -p "brick_2_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_8";
	setAttr ".t" -type "double3" 180 156.75 0 ;
createNode mesh -n "brick_2_Shape8" -p "brick_2_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_9";
	setAttr ".t" -type "double3" 300 156.75 0 ;
createNode mesh -n "brick_2_Shape9" -p "brick_2_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_10";
	setAttr ".t" -type "double3" 420 156.75 0 ;
createNode mesh -n "brick_2_Shape10" -p "brick_2_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_11";
	setAttr ".t" -type "double3" 540 156.75 0 ;
createNode mesh -n "brick_2_Shape11" -p "brick_2_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_12";
	setAttr ".t" -type "double3" 660 156.75 0 ;
createNode mesh -n "brick_2_Shape12" -p "brick_2_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_2_13";
	setAttr ".t" -type "double3" 780 156.75 0 ;
createNode mesh -n "brick_2_Shape13" -p "brick_2_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_0";
	setAttr ".t" -type "double3" -810 219 0 ;
createNode mesh -n "brick_3_Shape0" -p "brick_3_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_1";
	setAttr ".t" -type "double3" -720 219 0 ;
createNode mesh -n "brick_3_Shape1" -p "brick_3_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_2";
	setAttr ".t" -type "double3" -600 219 0 ;
createNode mesh -n "brick_3_Shape2" -p "brick_3_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_3";
	setAttr ".t" -type "double3" -480 219 0 ;
createNode mesh -n "brick_3_Shape3" -p "brick_3_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_4";
	setAttr ".t" -type "double3" -360 219 0 ;
createNode mesh -n "brick_3_Shape4" -p "brick_3_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_5";
	setAttr ".t" -type "double3" -240 219 0 ;
createNode mesh -n "brick_3_Shape5" -p "brick_3_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_6";
	setAttr ".t" -type "double3" -120 219 0 ;
createNode mesh -n "brick_3_Shape6" -p "brick_3_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_7";
	setAttr ".t" -type "double3" 0 219 0 ;
createNode mesh -n "brick_3_Shape7" -p "brick_3_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_8";
	setAttr ".t" -type "double3" 120 219 0 ;
createNode mesh -n "brick_3_Shape8" -p "brick_3_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_9";
	setAttr ".t" -type "double3" 240 219 0 ;
createNode mesh -n "brick_3_Shape9" -p "brick_3_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_10";
	setAttr ".t" -type "double3" 360 219 0 ;
createNode mesh -n "brick_3_Shape10" -p "brick_3_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_11";
	setAttr ".t" -type "double3" 480 219 0 ;
createNode mesh -n "brick_3_Shape11" -p "brick_3_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_12";
	setAttr ".t" -type "double3" 600 219 0 ;
createNode mesh -n "brick_3_Shape12" -p "brick_3_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_13";
	setAttr ".t" -type "double3" 720 219 0 ;
createNode mesh -n "brick_3_Shape13" -p "brick_3_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_3_14";
	setAttr ".t" -type "double3" 810 219 0 ;
createNode mesh -n "brick_3_Shape14" -p "brick_3_14";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_0";
	setAttr ".t" -type "double3" -780 281.25 0 ;
createNode mesh -n "brick_4_Shape0" -p "brick_4_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_1";
	setAttr ".t" -type "double3" -660 281.25 0 ;
createNode mesh -n "brick_4_Shape1" -p "brick_4_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_2";
	setAttr ".t" -type "double3" -540 281.25 0 ;
createNode mesh -n "brick_4_Shape2" -p "brick_4_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_3";
	setAttr ".t" -type "double3" -420 281.25 0 ;
createNode mesh -n "brick_4_Shape3" -p "brick_4_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_4";
	setAttr ".t" -type "double3" -300 281.25 0 ;
createNode mesh -n "brick_4_Shape4" -p "brick_4_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_5";
	setAttr ".t" -type "double3" -180 281.25 0 ;
createNode mesh -n "brick_4_Shape5" -p "brick_4_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_6";
	setAttr ".t" -type "double3" -60 281.25 0 ;
createNode mesh -n "brick_4_Shape6" -p "brick_4_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_7";
	setAttr ".t" -type "double3" 60 281.25 0 ;
createNode mesh -n "brick_4_Shape7" -p "brick_4_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_8";
	setAttr ".t" -type "double3" 180 281.25 0 ;
createNode mesh -n "brick_4_Shape8" -p "brick_4_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_9";
	setAttr ".t" -type "double3" 300 281.25 0 ;
createNode mesh -n "brick_4_Shape9" -p "brick_4_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_10";
	setAttr ".t" -type "double3" 420 281.25 0 ;
createNode mesh -n "brick_4_Shape10" -p "brick_4_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_11";
	setAttr ".t" -type "double3" 540 281.25 0 ;
createNode mesh -n "brick_4_Shape11" -p "brick_4_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_12";
	setAttr ".t" -type "double3" 660 281.25 0 ;
createNode mesh -n "brick_4_Shape12" -p "brick_4_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_4_13";
	setAttr ".t" -type "double3" 780 281.25 0 ;
createNode mesh -n "brick_4_Shape13" -p "brick_4_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_0";
	setAttr ".t" -type "double3" -810 343.5 0 ;
createNode mesh -n "brick_5_Shape0" -p "brick_5_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_1";
	setAttr ".t" -type "double3" -720 343.5 0 ;
createNode mesh -n "brick_5_Shape1" -p "brick_5_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_2";
	setAttr ".t" -type "double3" -600 343.5 0 ;
createNode mesh -n "brick_5_Shape2" -p "brick_5_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_3";
	setAttr ".t" -type "double3" -480 343.5 0 ;
createNode mesh -n "brick_5_Shape3" -p "brick_5_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_4";
	setAttr ".t" -type "double3" -360 343.5 0 ;
createNode mesh -n "brick_5_Shape4" -p "brick_5_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_5";
	setAttr ".t" -type "double3" -240 343.5 0 ;
createNode mesh -n "brick_5_Shape5" -p "brick_5_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_6";
	setAttr ".t" -type "double3" -120 343.5 0 ;
createNode mesh -n "brick_5_Shape6" -p "brick_5_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_7";
	setAttr ".t" -type "double3" 0 343.5 0 ;
createNode mesh -n "brick_5_Shape7" -p "brick_5_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_8";
	setAttr ".t" -type "double3" 120 343.5 0 ;
createNode mesh -n "brick_5_Shape8" -p "brick_5_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_9";
	setAttr ".t" -type "double3" 240 343.5 0 ;
createNode mesh -n "brick_5_Shape9" -p "brick_5_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_10";
	setAttr ".t" -type "double3" 360 343.5 0 ;
createNode mesh -n "brick_5_Shape10" -p "brick_5_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_11";
	setAttr ".t" -type "double3" 480 343.5 0 ;
createNode mesh -n "brick_5_Shape11" -p "brick_5_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_12";
	setAttr ".t" -type "double3" 600 343.5 0 ;
createNode mesh -n "brick_5_Shape12" -p "brick_5_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_13";
	setAttr ".t" -type "double3" 720 343.5 0 ;
createNode mesh -n "brick_5_Shape13" -p "brick_5_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_5_14";
	setAttr ".t" -type "double3" 810 343.5 0 ;
createNode mesh -n "brick_5_Shape14" -p "brick_5_14";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_0";
	setAttr ".t" -type "double3" -780 405.75 0 ;
createNode mesh -n "brick_6_Shape0" -p "brick_6_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_1";
	setAttr ".t" -type "double3" -660 405.75 0 ;
createNode mesh -n "brick_6_Shape1" -p "brick_6_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_2";
	setAttr ".t" -type "double3" -540 405.75 0 ;
createNode mesh -n "brick_6_Shape2" -p "brick_6_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_3";
	setAttr ".t" -type "double3" -420 405.75 0 ;
createNode mesh -n "brick_6_Shape3" -p "brick_6_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_4";
	setAttr ".t" -type "double3" -300 405.75 0 ;
createNode mesh -n "brick_6_Shape4" -p "brick_6_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_5";
	setAttr ".t" -type "double3" -180 405.75 0 ;
createNode mesh -n "brick_6_Shape5" -p "brick_6_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_6";
	setAttr ".t" -type "double3" -60 405.75 0 ;
createNode mesh -n "brick_6_Shape6" -p "brick_6_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_7";
	setAttr ".t" -type "double3" 60 405.75 0 ;
createNode mesh -n "brick_6_Shape7" -p "brick_6_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_8";
	setAttr ".t" -type "double3" 180 405.75 0 ;
createNode mesh -n "brick_6_Shape8" -p "brick_6_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_9";
	setAttr ".t" -type "double3" 300 405.75 0 ;
createNode mesh -n "brick_6_Shape9" -p "brick_6_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_10";
	setAttr ".t" -type "double3" 420 405.75 0 ;
createNode mesh -n "brick_6_Shape10" -p "brick_6_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_11";
	setAttr ".t" -type "double3" 540 405.75 0 ;
createNode mesh -n "brick_6_Shape11" -p "brick_6_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_12";
	setAttr ".t" -type "double3" 660 405.75 0 ;
createNode mesh -n "brick_6_Shape12" -p "brick_6_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_6_13";
	setAttr ".t" -type "double3" 780 405.75 0 ;
createNode mesh -n "brick_6_Shape13" -p "brick_6_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_0";
	setAttr ".t" -type "double3" -810 468 0 ;
createNode mesh -n "brick_7_Shape0" -p "brick_7_0";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_1";
	setAttr ".t" -type "double3" -720 468 0 ;
createNode mesh -n "brick_7_Shape1" -p "brick_7_1";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_2";
	setAttr ".t" -type "double3" -600 468 0 ;
createNode mesh -n "brick_7_Shape2" -p "brick_7_2";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_3";
	setAttr ".t" -type "double3" -480 468 0 ;
createNode mesh -n "brick_7_Shape3" -p "brick_7_3";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_4";
	setAttr ".t" -type "double3" -360 468 0 ;
createNode mesh -n "brick_7_Shape4" -p "brick_7_4";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_5";
	setAttr ".t" -type "double3" -240 468 0 ;
createNode mesh -n "brick_7_Shape5" -p "brick_7_5";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_6";
	setAttr ".t" -type "double3" -120 468 0 ;
createNode mesh -n "brick_7_Shape6" -p "brick_7_6";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_7";
	setAttr ".t" -type "double3" 0 468 0 ;
createNode mesh -n "brick_7_Shape7" -p "brick_7_7";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_8";
	setAttr ".t" -type "double3" 120 468 0 ;
createNode mesh -n "brick_7_Shape8" -p "brick_7_8";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_9";
	setAttr ".t" -type "double3" 240 468 0 ;
createNode mesh -n "brick_7_Shape9" -p "brick_7_9";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_10";
	setAttr ".t" -type "double3" 360 468 0 ;
createNode mesh -n "brick_7_Shape10" -p "brick_7_10";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_11";
	setAttr ".t" -type "double3" 480 468 0 ;
createNode mesh -n "brick_7_Shape11" -p "brick_7_11";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_12";
	setAttr ".t" -type "double3" 600 468 0 ;
createNode mesh -n "brick_7_Shape12" -p "brick_7_12";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_13";
	setAttr ".t" -type "double3" 720 468 0 ;
createNode mesh -n "brick_7_Shape13" -p "brick_7_13";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -57 -30 48 57 -30 48 57 30 48 -57 30 48 57 30 -48 -57 30 -48 57 -30 -48 -57 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "brick_7_14";
	setAttr ".t" -type "double3" 810 468 0 ;
createNode mesh -n "brick_7_Shape14" -p "brick_7_14";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 14 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:13]" -type "float2" 0.375 0 0.625 0 0.625 0.25 0.375 0.25 0.625 0.5 0.375 0.5 0.625 0.75 0.375 0.75 0.625 1 0.375 1 0.875 0 0.875 0.25 0.125 0 0.125 0.25 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 8 ".vt";
	setAttr ".vt[0:7]"  -27 -30 48 27 -30 48 27 30 48 -27 30 48 27 30 -48 -27 30 -48 27 -30 -48 -27 -30 -48;
	setAttr -s 12 ".ed";
	setAttr ".ed[0:11]"  0 1 0 1 2 0 2 3 0 3 0 0 2 4 0 4 5 0 5 3 0 4 6 0 6 7 0 7 5 0 6 1 0 0 7 0;
	setAttr -s 6 -ch 24 ".fc[0:5]" -type "polyFaces" 
		f 4 0 1 2 3 mu 0 4 0 1 2 3 
		f 4 -3 4 5 6 mu 0 4 3 2 4 5 
		f 4 -6 7 8 9 mu 0 4 5 4 6 7 
		f 4 -9 10 -1 11 mu 0 4 7 6 8 9 
		f 4 -11 -8 -5 -2 mu 0 4 1 10 11 2 
		f 4 -12 -4 -7 -10 mu 0 4 12 0 3 13 ;
createNode transform -n "wreckingBall";
	setAttr ".t" -type "double3" -150 150 -900 ;
createNode mesh -n "wreckingBallShape" -p "wreckingBall";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 439 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:249]" -type "float2" 0.025 0 0.05 0.05 0 0.05 0.075 0 0.1 0.05 0.125 0 0.15 0.05 0.17500001 0 0.2 0.05 0.225 0 0.25 0.05 0.275 0 0.3 0.05 0.325 0 0.35 0.05 0.375 0 0.4 0.05 0.425 0 0.45 0.05 0.475 0 0.5 0.05 0.525 0 0.55 0.05 0.57500005 0 0.6 0.05 0.625 0 0.65 0.05 0.67499995 0 0.7 0.05 0.725 0 0.75 0.05 0.775 0 0.8 0.05 0.82500005 0 0.85 0.05 0.875 0 0.9 0.05 0.92499995 0 0.95 0.05 0.975 0 1 0.05 0.05 0.1 0 0.1 0.1 0.1 0.15 0.1 0.2 0.1 0.25 0.1 0.3 0.1 0.35 0.1 0.4 0.1 0.45 0.1 0.5 0.1 0.55 0.1 0.6 0.1 0.65 0.1 0.7 0.1 0.75 0.1 0.8 0.1 0.85 0.1 0.9 0.1 0.95 0.1 1 0.1 0.05 0.15 0 0.15 0.1 0.15 0.15 0.15 0.2 0.15 0.25 0.15 0.3 0.15 0.35 0.15 0.4 0.15 0.45 0.15 0.5 0.15 0.55 0.15 0.6 0.15 0.65 0.15 0.7 0.15 0.75 0.15 0.8 0.15 0.85 0.15 0.9 0.15 0.95 0.15 1 0.15 0.05 0.2 0 0.2 0.1 0.2 0.15 0.2 0.2 0.2 0.25 0.2 0.3 0.2 0.35 0.2 0.4 0.2 0.45 0.2 0.5 0.2 0.55 0.2 0.6 0.2 0.65 0.2 0.7 0.2 0.75 0.2 0.8 0.2 0.85 0.2 0.9 0.2 0.95 0.2 1 0.2 0.05 0.25 0 0.25 0.1 0.25 0.15 0.25 0.2 0.25 0.25 0.25 0.3 0.25 0.35 0.25 0.4 0.25 0.45 0.25 0.5 0.25 0.55 0.25 0.6 0.25 0.65 0.25 0.7 0.25 0.75 0.25 0.8 0.25 0.85 0.25 0.9 0.25 0.95 0.25 1 0.25 0.05 0.3 0 0.3 0.1 0.3 0.15 0.3 0.2 0.3 0.25 0.3 0.3 0.3 0.35 0.3 0.4 0.3 0.45 0.3 0.5 0.3 0.55 0.3 0.6 0.3 0.65 0.3 0.7 0.3 0.75 0.3 0.8 0.3 0.85 0.3 0.9 0.3 0.95 0.3 1 0.3 0.05 0.35 0 0.35 0.1 0.35 0.15 0.35 0.2 0.35 0.25 0.35 0.3 0.35 0.35 0.35 0.4 0.35 0.45 0.35 0.5 0.35 0.55 0.35 0.6 0.35 0.65 0.35 0.7 0.35 0.75 0.35 0.8 0.35 0.85 0.35 0.9 0.35 0.95 0.35 1 0.35 0.05 0.4 0 0.4 0.1 0.4 0.15 0.4 0.2 0.4 0.25 0.4 0.3 0.4 0.35 0.4 0.4 0.4 0.45 0.4 0.5 0.4 0.55 0.4 0.6 0.4 0.65 0.4 0.7 0.4 0.75 0.4 0.8 0.4 0.85 0.4 0.9 0.4 0.95 0.4 1 0.4 0.05 0.45 0 0.45 0.1 0.45 0.15 0.45 0.2 0.45 0.25 0.45 0.3 0.45 0.35 0.45 0.4 0.45 0.45 0.45 0.5 0.45 0.55 0.45 0.6 0.45 0.65 0.45 0.7 0.45 0.75 0.45 0.8 0.45 0.85 0.45 0.9 0.45 0.95 0.45 1 0.45 0.05 0.5 0 0.5 0.1 0.5 0.15 0.5 0.2 0.5 0.25 0.5 0.3 0.5 0.35 0.5 0.4 0.5 0.45 0.5 0.5 0.5 0.55 0.5 0.6 0.5 0.65 0.5 0.7 0.5 0.75 0.5 0.8 0.5 0.85 0.5 0.9 0.5 0.95 0.5 1 0.5 0.05 0.55 0 0.55 0.1 0.55 0.15 0.55 0.2 0.55 0.25 0.55 0.3 0.55 0.35 0.55 0.4 0.55 0.45 0.55 0.5 0.55 0.55 0.55 0.6 0.55 0.65 0.55 0.7 0.55 0.75 0.55 0.8 0.55 0.85 0.55 0.9 0.55 0.95 0.55 ;
	setAttr ".uvst[0].uvsp[250:438]" -type "float2" 1 0.55 0.05 0.6 0 0.6 0.1 0.6 0.15 0.6 0.2 0.6 0.25 0.6 0.3 0.6 0.35 0.6 0.4 0.6 0.45 0.6 0.5 0.6 0.55 0.6 0.6 0.6 0.65 0.6 0.7 0.6 0.75 0.6 0.8 0.6 0.85 0.6 0.9 0.6 0.95 0.6 1 0.6 0.05 0.65 0 0.65 0.1 0.65 0.15 0.65 0.2 0.65 0.25 0.65 0.3 0.65 0.35 0.65 0.4 0.65 0.45 0.65 0.5 0.65 0.55 0.65 0.6 0.65 0.65 0.65 0.7 0.65 0.75 0.65 0.8 0.65 0.85 0.65 0.9 0.65 0.95 0.65 1 0.65 0.05 0.7 0 0.7 0.1 0.7 0.15 0.7 0.2 0.7 0.25 0.7 0.3 0.7 0.35 0.7 0.4 0.7 0.45 0.7 0.5 0.7 0.55 0.7 0.6 0.7 0.65 0.7 0.7 0.7 0.75 0.7 0.8 0.7 0.85 0.7 0.9 0.7 0.95 0.7 1 0.7 0.05 0.75 0 0.75 0.1 0.75 0.15 0.75 0.2 0.75 0.25 0.75 0.3 0.75 0.35 0.75 0.4 0.75 0.45 0.75 0.5 0.75 0.55 0.75 0.6 0.75 0.65 0.75 0.7 0.75 0.75 0.75 0.8 0.75 0.85 0.75 0.9 0.75 0.95 0.75 1 0.75 0.05 0.8 0 0.8 0.1 0.8 0.15 0.8 0.2 0.8 0.25 0.8 0.3 0.8 0.35 0.8 0.4 0.8 0.45 0.8 0.5 0.8 0.55 0.8 0.6 0.8 0.65 0.8 0.7 0.8 0.75 0.8 0.8 0.8 0.85 0.8 0.9 0.8 0.95 0.8 1 0.8 0.05 0.85 0 0.85 0.1 0.85 0.15 0.85 0.2 0.85 0.25 0.85 0.3 0.85 0.35 0.85 0.4 0.85 0.45 0.85 0.5 0.85 0.55 0.85 0.6 0.85 0.65 0.85 0.7 0.85 0.75 0.85 0.8 0.85 0.85 0.85 0.9 0.85 0.95 0.85 1 0.85 0.05 0.9 0 0.9 0.1 0.9 0.15 0.9 0.2 0.9 0.25 0.9 0.3 0.9 0.35 0.9 0.4 0.9 0.45 0.9 0.5 0.9 0.55 0.9 0.6 0.9 0.65 0.9 0.7 0.9 0.75 0.9 0.8 0.9 0.85 0.9 0.9 0.9 0.95 0.9 1 0.9 0.05 0.95 0 0.95 0.1 0.95 0.15 0.95 0.2 0.95 0.25 0.95 0.3 0.95 0.35 0.95 0.4 0.95 0.45 0.95 0.5 0.95 0.55 0.95 0.6 0.95 0.65 0.95 0.7 0.95 0.75 0.95 0.8 0.95 0.85 0.95 0.9 0.95 0.95 0.95 1 0.95 0.025 1 0.075 1 0.125 1 0.17500001 1 0.225 1 0.275 1 0.325 1 0.375 1 0.425 1 0.475 1 0.525 1 0.57500005 1 0.625 1 0.67499995 1 0.725 1 0.775 1 0.82500005 1 0.875 1 0.92499995 1 0.975 1 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 382 ".vt";
	setAttr ".vt[0:165]"  0 -90 0 14.079101853620779 -88.8919506535624 0 13.390021561469217 -88.8919506535624 -4.350681738304644 11.390232665115034 -88.8919506535624 -8.275488435081916 8.275488435081916 -88.8919506535624 -11.390232665115034 4.350681738304645 -88.8919506535624 -13.390021561469217 0.0000000000000008620963509953127 -88.8919506535624 -14.079101853620779 -4.350681738304643 -88.8919506535624 -13.390021561469219 -8.275488435081915 -88.8919506535624 -11.390232665115034 -11.390232665115033 -88.8919506535624 -8.275488435081918 -13.390021561469217 -88.8919506535624 -4.350681738304646 -14.079101853620779 -88.8919506535624 -0.0000000000000017241927019906254 -13.39002156146922 -88.8919506535624 4.350681738304637 -11.390232665115036 -88.8919506535624 8.275488435081915 -8.275488435081918 -88.8919506535624 11.390232665115033 -4.350681738304646 -88.8919506535624 13.390021561469217 -0.000000000000002586289052985938 -88.8919506535624 14.079101853620779 4.350681738304641 -88.8919506535624 13.390021561469219 8.275488435081913 -88.8919506535624 11.390232665115036 11.390232665115033 -88.8919506535624 8.275488435081918 13.390021561469217 -88.8919506535624 4.3506817383046466 27.811529493745265 -85.59508646656381 0 26.450336353161287 -85.59508646656381 -8.594235253127364 22.5 -85.59508646656381 -16.34720688012062 16.34720688012062 -85.59508646656381 -22.5 8.594235253127366 -85.59508646656381 -26.450336353161287 0.0000000000000017029650286953673 -85.59508646656381 -27.811529493745265 -8.594235253127362 -85.59508646656381 -26.45033635316129 -16.347206880120616 -85.59508646656381 -22.5 -22.499999999999996 -85.59508646656381 -16.34720688012062 -26.450336353161287 -85.59508646656381 -8.594235253127367 -27.811529493745265 -85.59508646656381 -0.0000000000000034059300573907346 -26.450336353161294 -85.59508646656381 8.59423525312735 -22.5 -85.59508646656381 16.347206880120616 -16.34720688012062 -85.59508646656381 22.499999999999996 -8.59423525312737 -85.59508646656381 26.450336353161287 -0.000000000000005108895086086102 -85.59508646656381 27.811529493745265 8.59423525312736 -85.59508646656381 26.45033635316129 16.347206880120613 -85.59508646656381 22.5 22.499999999999996 -85.59508646656381 16.347206880120623 26.450336353161287 -85.59508646656381 8.594235253127371 40.85914497655921 -80.19058717695312 0 38.859356080205025 -80.19058717695312 -12.626170173386557 33.055742661666166 -80.19058717695312 -24.01640283850159 24.01640283850159 -80.19058717695312 -33.055742661666166 12.626170173386559 -80.19058717695312 -38.859356080205025 0.0000000000000025019010555720446 -80.19058717695312 -40.85914497655921 -12.626170173386555 -80.19058717695312 -38.859356080205025 -24.016402838501584 -80.19058717695312 -33.055742661666166 -33.05574266166616 -80.19058717695312 -24.016402838501595 -38.859356080205025 -80.19058717695312 -12.626170173386562 -40.85914497655921 -80.19058717695312 -0.000000000000005003802111144089 -38.85935608020503 -80.19058717695312 12.626170173386535 -33.055742661666166 -80.19058717695312 24.016402838501584 -24.016402838501595 -80.19058717695312 33.05574266166616 -12.626170173386564 -80.19058717695312 38.859356080205025 -0.000000000000007505703166716133 -80.19058717695312 40.85914497655921 12.62617017338655 -80.19058717695312 38.859356080205025 24.01640283850158 -80.19058717695312 33.055742661666166 33.05574266166616 -80.19058717695312 24.016402838501598 38.859356080205025 -80.19058717695312 12.626170173386566 52.90067270632258 -72.81152949374527 0 50.31152949374527 -72.81152949374527 -16.34720688012062 42.797543233281914 -72.81152949374527 -31.094235253127366 31.094235253127366 -72.81152949374527 -42.797543233281914 16.34720688012062 -72.81152949374527 -50.31152949374527 0.000000000000003239231975126985 -72.81152949374527 -52.90067270632258 -16.347206880120616 -72.81152949374527 -50.31152949374527 -31.09423525312736 -72.81152949374527 -42.797543233281914 -42.79754323328191 -72.81152949374527 -31.094235253127373 -50.31152949374527 -72.81152949374527 -16.347206880120623 -52.90067270632258 -72.81152949374527 -0.00000000000000647846395025397 -50.311529493745276 -72.81152949374527 16.34720688012059 -42.79754323328192 -72.81152949374527 31.09423525312736 -31.094235253127373 -72.81152949374527 42.79754323328191 -16.347206880120627 -72.81152949374527 50.31152949374527 -0.000000000000009717695925380954 -72.81152949374527 52.90067270632258 16.34720688012061 -72.81152949374527 50.31152949374527 31.094235253127355 -72.81152949374527 42.79754323328192 42.79754323328191 -72.81152949374527 31.094235253127376 50.31152949374527 -72.81152949374527 16.34720688012063 63.63961030678927 -63.63961030678928 0 60.524866076756155 -63.63961030678928 -19.665721100196944 51.48552625359159 -63.63961030678928 -37.40642439997081 37.40642439997081 -63.63961030678928 -51.48552625359159 19.665721100196947 -63.63961030678928 -60.524866076756155 0.0000000000000038968022530597194 -63.63961030678928 -63.63961030678927 -19.66572110019694 -63.63961030678928 -60.52486607675616 -37.4064243999708 -63.63961030678928 -51.48552625359159 -51.48552625359158 -63.63961030678928 -37.40642439997082 -60.524866076756155 -63.63961030678928 -19.66572110019695 -63.63961030678927 -63.63961030678928 -0.000000000000007793604506119439 -60.52486607675617 -63.63961030678928 19.66572110019691 -51.485526253591594 -63.63961030678928 37.4064243999708 -37.40642439997082 -63.63961030678928 51.48552625359158 -19.665721100196954 -63.63961030678928 60.524866076756155 -0.000000000000011690406759179158 -63.63961030678928 63.63961030678927 19.665721100196933 -63.63961030678928 60.52486607675616 37.406424399970795 -63.63961030678928 51.485526253591594 51.48552625359158 -63.63961030678928 37.406424399970824 60.524866076756155 -63.63961030678928 19.665721100196958 72.81152949374527 -52.90067270632258 0 69.2478795864432 -52.90067270632258 -22.499999999999996 58.905764746872634 -52.90067270632258 -42.797543233281914 42.797543233281914 -52.90067270632258 -58.905764746872634 22.500000000000004 -52.90067270632258 -69.2478795864432 0.000000000000004458420326776913 -52.90067270632258 -72.81152949374527 -22.499999999999993 -52.90067270632258 -69.2478795864432 -42.79754323328191 -52.90067270632258 -58.905764746872634 -58.90576474687263 -52.90067270632258 -42.79754323328192 -69.2478795864432 -52.90067270632258 -22.500000000000007 -72.81152949374527 -52.90067270632258 -0.000000000000008916840653553825 -69.24787958644322 -52.90067270632258 22.49999999999996 -58.90576474687264 -52.90067270632258 42.79754323328191 -42.79754323328192 -52.90067270632258 58.90576474687263 -22.50000000000001 -52.90067270632258 69.2478795864432 -0.000000000000013375260980330736 -52.90067270632258 72.81152949374527 22.499999999999986 -52.90067270632258 69.2478795864432 42.79754323328189 -52.90067270632258 58.90576474687264 58.90576474687263 -52.90067270632258 42.79754323328193 69.2478795864432 -52.90067270632258 22.500000000000014 80.1905871769531 -40.859144976559215 0 76.26578048017583 -40.859144976559215 -24.780254226584244 64.8755478150608 -40.859144976559215 -47.13484451528694 47.13484451528694 -40.859144976559215 -64.8755478150608 24.78025422658425 -40.859144976559215 -76.26578048017583 0.0000000000000049102572954001204 -40.859144976559215 -80.1905871769531 -24.78025422658424 -40.859144976559215 -76.26578048017583 -47.13484451528693 -40.859144976559215 -64.8755478150608 -64.8755478150608 -40.859144976559215 -47.13484451528695 -76.26578048017583 -40.859144976559215 -24.780254226584255 -80.1905871769531 -40.859144976559215 -0.000000000000009820514590800241 -76.26578048017585 -40.859144976559215 24.780254226584205 -64.87554781506081 -40.859144976559215 47.13484451528693 -47.13484451528695 -40.859144976559215 64.8755478150608 -24.78025422658426 -40.859144976559215 76.26578048017583 -0.00000000000001473077188620036 -40.859144976559215 80.1905871769531 24.780254226584233 -40.859144976559215 76.26578048017583 47.13484451528692 -40.859144976559215 64.87554781506081 64.8755478150608 -40.859144976559215 47.13484451528696 76.26578048017583 -40.859144976559215 24.780254226584262 85.59508646656381 -27.81152949374527 0 81.40576474687262 -27.81152949374527 -26.450336353161287 69.2478795864432 -27.81152949374527 -50.31152949374526 50.31152949374526 -27.81152949374527 -69.2478795864432 26.45033635316129 -27.81152949374527 -81.40576474687262 0.000000000000005241187433200915 -27.81152949374527 -85.59508646656381 -26.45033635316128 -27.81152949374527 -81.40576474687263 -50.311529493745255 -27.81152949374527 -69.2478795864432 -69.24787958644319 -27.81152949374527 -50.311529493745276 -81.40576474687262 -27.81152949374527 -26.450336353161294 -85.59508646656381 -27.81152949374527 -0.00000000000001048237486640183 -81.40576474687265 -27.81152949374527 26.450336353161244 -69.2478795864432 -27.81152949374527 50.311529493745255 -50.311529493745276 -27.81152949374527 69.24787958644319 -26.4503363531613 -27.81152949374527 81.40576474687262 -0.000000000000015723562299602746 -27.81152949374527 85.59508646656381 26.450336353161273 -27.81152949374527 81.40576474687263 50.31152949374525 -27.81152949374527 69.2478795864432 69.24787958644319 -27.81152949374527 50.31152949374528 81.40576474687262 -27.81152949374527 26.450336353161305 88.8919506535624 -14.079101853620783 0 84.54126891525776 -14.079101853620783 -27.469123415089996 71.9150987418712 -14.079101853620783 -52.24937764167425 52.24937764167425 -14.079101853620783 -71.9150987418712 27.46912341509 -14.079101853620783 -84.54126891525776;
	setAttr ".vt[166:331]"  0.000000000000005443062141892484 -14.079101853620783 -88.8919506535624 -27.46912341508999 -14.079101853620783 -84.54126891525776 -52.24937764167424 -14.079101853620783 -71.9150987418712 -71.91509874187119 -14.079101853620783 -52.24937764167426 -84.54126891525776 -14.079101853620783 -27.469123415090003 -88.8919506535624 -14.079101853620783 -0.000000000000010886124283784968 -84.54126891525777 -14.079101853620783 27.46912341508995 -71.9150987418712 -14.079101853620783 52.24937764167424 -52.24937764167426 -14.079101853620783 71.91509874187119 -27.46912341509001 -14.079101853620783 84.54126891525776 -0.00000000000001632918642567745 -14.079101853620783 88.8919506535624 27.46912341508998 -14.079101853620783 84.54126891525776 52.24937764167423 -14.079101853620783 71.9150987418712 71.91509874187119 -14.079101853620783 52.24937764167427 84.54126891525776 -14.079101853620783 27.469123415090014 90 -0.0000000000000055109105961630896 0 85.59508646656381 -0.0000000000000055109105961630896 -27.811529493745265 72.81152949374527 -0.0000000000000055109105961630896 -52.90067270632258 52.90067270632258 -0.0000000000000055109105961630896 -72.81152949374527 27.81152949374527 -0.0000000000000055109105961630896 -85.59508646656381 0.0000000000000055109105961630896 -0.0000000000000055109105961630896 -90 -27.81152949374526 -0.0000000000000055109105961630896 -85.59508646656383 -52.900672706322574 -0.0000000000000055109105961630896 -72.81152949374527 -72.81152949374525 -0.0000000000000055109105961630896 -52.900672706322595 -85.59508646656381 -0.0000000000000055109105961630896 -27.811529493745276 -90 -0.0000000000000055109105961630896 -0.000000000000011021821192326179 -85.59508646656384 -0.0000000000000055109105961630896 27.81152949374522 -72.81152949374528 -0.0000000000000055109105961630896 52.900672706322574 -52.900672706322595 -0.0000000000000055109105961630896 72.81152949374525 -27.81152949374528 -0.0000000000000055109105961630896 85.59508646656381 -0.000000000000016532731788489267 -0.0000000000000055109105961630896 90 27.81152949374525 -0.0000000000000055109105961630896 85.59508646656383 52.90067270632256 -0.0000000000000055109105961630896 72.81152949374528 72.81152949374525 -0.0000000000000055109105961630896 52.9006727063226 85.59508646656381 -0.0000000000000055109105961630896 27.811529493745287 88.8919506535624 14.079101853620752 0 84.54126891525776 14.079101853620752 -27.469123415089996 71.9150987418712 14.079101853620752 -52.24937764167425 52.24937764167425 14.079101853620752 -71.9150987418712 27.46912341509 14.079101853620752 -84.54126891525776 0.000000000000005443062141892484 14.079101853620752 -88.8919506535624 -27.46912341508999 14.079101853620752 -84.54126891525776 -52.24937764167424 14.079101853620752 -71.9150987418712 -71.91509874187119 14.079101853620752 -52.24937764167426 -84.54126891525776 14.079101853620752 -27.469123415090003 -88.8919506535624 14.079101853620752 -0.000000000000010886124283784968 -84.54126891525777 14.079101853620752 27.46912341508995 -71.9150987418712 14.079101853620752 52.24937764167424 -52.24937764167426 14.079101853620752 71.91509874187119 -27.46912341509001 14.079101853620752 84.54126891525776 -0.00000000000001632918642567745 14.079101853620752 88.8919506535624 27.46912341508998 14.079101853620752 84.54126891525776 52.24937764167423 14.079101853620752 71.9150987418712 71.91509874187119 14.079101853620752 52.24937764167427 84.54126891525776 14.079101853620752 27.469123415090014 85.59508646656383 27.81152949374526 0 81.40576474687263 27.81152949374526 -26.45033635316129 69.2478795864432 27.81152949374526 -50.311529493745276 50.311529493745276 27.81152949374526 -69.2478795864432 26.450336353161294 27.81152949374526 -81.40576474687263 0.000000000000005241187433200916 27.81152949374526 -85.59508646656383 -26.450336353161287 27.81152949374526 -81.40576474687265 -50.31152949374526 27.81152949374526 -69.2478795864432 -69.2478795864432 27.81152949374526 -50.31152949374528 -81.40576474687263 27.81152949374526 -26.4503363531613 -85.59508646656383 27.81152949374526 -0.000000000000010482374866401832 -81.40576474687265 27.81152949374526 26.450336353161248 -69.24787958644322 27.81152949374526 50.31152949374526 -50.31152949374528 27.81152949374526 69.2478795864432 -26.450336353161305 27.81152949374526 81.40576474687263 -0.000000000000015723562299602746 27.81152949374526 85.59508646656383 26.450336353161276 27.81152949374526 81.40576474687265 50.311529493745255 27.81152949374526 69.24787958644322 69.2478795864432 27.81152949374526 50.31152949374529 81.40576474687263 27.81152949374526 26.45033635316131 80.19058717695312 40.8591449765592 0 76.26578048017583 40.8591449765592 -24.78025422658425 64.87554781506081 40.8591449765592 -47.13484451528695 47.13484451528695 40.8591449765592 -64.87554781506081 24.780254226584255 40.8591449765592 -76.26578048017583 0.000000000000004910257295400121 40.8591449765592 -80.19058717695312 -24.780254226584244 40.8591449765592 -76.26578048017585 -47.13484451528694 40.8591449765592 -64.87554781506081 -64.8755478150608 40.8591449765592 -47.13484451528696 -76.26578048017583 40.8591449765592 -24.78025422658426 -80.19058717695312 40.8591449765592 -0.000000000000009820514590800242 -76.26578048017586 40.8591449765592 24.78025422658421 -64.87554781506083 40.8591449765592 47.13484451528694 -47.13484451528696 40.8591449765592 64.8755478150608 -24.780254226584262 40.8591449765592 76.26578048017583 -0.000000000000014730771886200362 40.8591449765592 80.19058717695312 24.780254226584237 40.8591449765592 76.26578048017585 47.13484451528693 40.8591449765592 64.87554781506083 64.8755478150608 40.8591449765592 47.134844515286964 76.26578048017583 40.8591449765592 24.78025422658427 72.81152949374527 52.900672706322574 0 69.2478795864432 52.900672706322574 -22.499999999999996 58.905764746872634 52.900672706322574 -42.797543233281914 42.797543233281914 52.900672706322574 -58.905764746872634 22.500000000000004 52.900672706322574 -69.2478795864432 0.000000000000004458420326776913 52.900672706322574 -72.81152949374527 -22.499999999999993 52.900672706322574 -69.2478795864432 -42.79754323328191 52.900672706322574 -58.905764746872634 -58.90576474687263 52.900672706322574 -42.79754323328192 -69.2478795864432 52.900672706322574 -22.500000000000007 -72.81152949374527 52.900672706322574 -0.000000000000008916840653553825 -69.24787958644322 52.900672706322574 22.49999999999996 -58.90576474687264 52.900672706322574 42.79754323328191 -42.79754323328192 52.900672706322574 58.90576474687263 -22.50000000000001 52.900672706322574 69.2478795864432 -0.000000000000013375260980330736 52.900672706322574 72.81152949374527 22.499999999999986 52.900672706322574 69.2478795864432 42.79754323328189 52.900672706322574 58.90576474687264 58.90576474687263 52.900672706322574 42.79754323328193 69.2478795864432 52.900672706322574 22.500000000000014 63.63961030678928 63.63961030678927 0 60.52486607675616 63.63961030678927 -19.665721100196947 51.48552625359159 63.63961030678927 -37.40642439997081 37.40642439997081 63.63961030678927 -51.48552625359159 19.66572110019695 63.63961030678927 -60.52486607675616 0.00000000000000389680225305972 63.63961030678927 -63.63961030678928 -19.665721100196944 63.63961030678927 -60.52486607675617 -37.4064243999708 63.63961030678927 -51.48552625359159 -51.48552625359158 63.63961030678927 -37.40642439997082 -60.52486607675616 63.63961030678927 -19.665721100196954 -63.63961030678928 63.63961030678927 -0.00000000000000779360450611944 -60.524866076756176 63.63961030678927 19.665721100196915 -51.485526253591594 63.63961030678927 37.4064243999708 -37.40642439997082 63.63961030678927 51.48552625359158 -19.665721100196958 63.63961030678927 60.52486607675616 -0.00000000000001169040675917916 63.63961030678927 63.63961030678928 19.665721100196937 63.63961030678927 60.52486607675617 37.406424399970795 63.63961030678927 51.485526253591594 51.48552625359158 63.63961030678927 37.406424399970824 60.52486607675616 63.63961030678927 19.66572110019696 52.900672706322595 72.81152949374525 0 50.311529493745276 72.81152949374525 -16.347206880120623 42.79754323328192 72.81152949374525 -31.094235253127373 31.094235253127373 72.81152949374525 -42.79754323328192 16.347206880120627 72.81152949374525 -50.311529493745276 0.0000000000000032392319751269857 72.81152949374525 -52.900672706322595 -16.34720688012062 72.81152949374525 -50.31152949374528 -31.09423525312737 72.81152949374525 -42.79754323328192 -42.797543233281914 72.81152949374525 -31.09423525312738 -50.311529493745276 72.81152949374525 -16.347206880120627 -52.900672706322595 72.81152949374525 -0.0000000000000064784639502539714 -50.31152949374529 72.81152949374525 16.347206880120595 -42.79754323328193 72.81152949374525 31.09423525312737 -31.09423525312738 72.81152949374525 42.797543233281914 -16.34720688012063 72.81152949374525 50.311529493745276 -0.000000000000009717695925380958 72.81152949374525 52.900672706322595 16.347206880120613 72.81152949374525 50.31152949374528 31.094235253127362 72.81152949374525 42.79754323328193 42.797543233281914 72.81152949374525 31.094235253127387 50.311529493745276 72.81152949374525 16.347206880120634 40.859144976559215 80.1905871769531 0 38.85935608020503 80.1905871769531 -12.626170173386559 33.055742661666166 80.1905871769531 -24.016402838501595 24.016402838501595 80.1905871769531 -33.055742661666166 12.62617017338656 80.1905871769531 -38.85935608020503 0.000000000000002501901055572045 80.1905871769531 -40.859144976559215 -12.626170173386557 80.1905871769531 -38.85935608020503 -24.01640283850159 80.1905871769531 -33.055742661666166 -33.055742661666166 80.1905871769531 -24.016402838501598 -38.85935608020503 80.1905871769531 -12.626170173386564 -40.859144976559215 80.1905871769531 -0.00000000000000500380211114409;
	setAttr ".vt[332:381]"  -38.85935608020504 80.1905871769531 12.626170173386539 -33.05574266166617 80.1905871769531 24.01640283850159 -24.016402838501598 80.1905871769531 33.055742661666166 -12.626170173386566 80.1905871769531 38.85935608020503 -0.000000000000007505703166716134 80.1905871769531 40.859144976559215 12.626170173386551 80.1905871769531 38.85935608020503 24.016402838501584 80.1905871769531 33.05574266166617 33.055742661666166 80.1905871769531 24.0164028385016 38.85935608020503 80.1905871769531 12.626170173386567 27.811529493745276 85.59508646656381 0 26.450336353161298 85.59508646656381 -8.594235253127367 22.500000000000007 85.59508646656381 -16.347206880120623 16.347206880120623 85.59508646656381 -22.500000000000007 8.59423525312737 85.59508646656381 -26.450336353161298 0.000000000000001702965028695368 85.59508646656381 -27.811529493745276 -8.594235253127366 85.59508646656381 -26.4503363531613 -16.347206880120623 85.59508646656381 -22.500000000000007 -22.500000000000004 85.59508646656381 -16.347206880120627 -26.450336353161298 85.59508646656381 -8.594235253127371 -27.811529493745276 85.59508646656381 -0.000000000000003405930057390736 -26.450336353161305 85.59508646656381 8.594235253127353 -22.50000000000001 85.59508646656381 16.347206880120623 -16.347206880120627 85.59508646656381 22.500000000000004 -8.594235253127373 85.59508646656381 26.450336353161298 -0.000000000000005108895086086104 85.59508646656381 27.811529493745276 8.594235253127362 85.59508646656381 26.4503363531613 16.34720688012062 85.59508646656381 22.50000000000001 22.500000000000004 85.59508646656381 16.34720688012063 26.450336353161298 85.59508646656381 8.594235253127374 14.079101853620788 88.89195065356239 0 13.390021561469226 88.89195065356239 -4.3506817383046466 11.390232665115041 88.89195065356239 -8.275488435081922 8.275488435081922 88.89195065356239 -11.390232665115041 4.350681738304647 88.89195065356239 -13.390021561469226 0.0000000000000008620963509953133 88.89195065356239 -14.079101853620788 -4.350681738304646 88.89195065356239 -13.390021561469228 -8.27548843508192 88.89195065356239 -11.390232665115041 -11.39023266511504 88.89195065356239 -8.275488435081922 -13.390021561469226 88.89195065356239 -4.350681738304648 -14.079101853620788 88.89195065356239 -0.0000000000000017241927019906266 -13.390021561469228 88.89195065356239 4.3506817383046394 -11.390232665115043 88.89195065356239 8.27548843508192 -8.275488435081922 88.89195065356239 11.39023266511504 -4.350681738304649 88.89195065356239 13.390021561469226 -0.0000000000000025862890529859397 88.89195065356239 14.079101853620788 4.350681738304644 88.89195065356239 13.390021561469228 8.275488435081918 88.89195065356239 11.390232665115043 11.39023266511504 88.89195065356239 8.275488435081924 13.390021561469226 88.89195065356239 4.350681738304649 0 90 0;
	setAttr -s 780 ".ed";
	setAttr ".ed[0:165]"  0 2 1 2 1 1 1 0 1 0 3 1 3 2 1 0 4 1 4 3 1 0 5 1 5 4 1 0 6 1 6 5 1 0 7 1 7 6 1 0 8 1 8 7 1 0 9 1 9 8 1 0 10 1 10 9 1 0 11 1 11 10 1 0 12 1 12 11 1 0 13 1 13 12 1 0 14 1 14 13 1 0 15 1 15 14 1 0 16 1 16 15 1 0 17 1 17 16 1 0 18 1 18 17 1 0 19 1 19 18 1 0 20 1 20 19 1 1 20 1 2 22 1 22 21 1 21 1 1 3 23 1 23 22 1 4 24 1 24 23 1 5 25 1 25 24 1 6 26 1 26 25 1 7 27 1 27 26 1 8 28 1 28 27 1 9 29 1 29 28 1 10 30 1 30 29 1 11 31 1 31 30 1 12 32 1 32 31 1 13 33 1 33 32 1 14 34 1 34 33 1 15 35 1 35 34 1 16 36 1 36 35 1 17 37 1 37 36 1 18 38 1 38 37 1 19 39 1 39 38 1 20 40 1 40 39 1 21 40 1 22 42 1 42 41 1 41 21 1 23 43 1 43 42 1 24 44 1 44 43 1 25 45 1 45 44 1 26 46 1 46 45 1 27 47 1 47 46 1 28 48 1 48 47 1 29 49 1 49 48 1 30 50 1 50 49 1 31 51 1 51 50 1 32 52 1 52 51 1 33 53 1 53 52 1 34 54 1 54 53 1 35 55 1 55 54 1 36 56 1 56 55 1 37 57 1 57 56 1 38 58 1 58 57 1 39 59 1 59 58 1 40 60 1 60 59 1 41 60 1 42 62 1 62 61 1 61 41 1 43 63 1 63 62 1 44 64 1 64 63 1 45 65 1 65 64 1 46 66 1 66 65 1 47 67 1 67 66 1 48 68 1 68 67 1 49 69 1 69 68 1 50 70 1 70 69 1 51 71 1 71 70 1 52 72 1 72 71 1 53 73 1 73 72 1 54 74 1 74 73 1 55 75 1 75 74 1 56 76 1 76 75 1 57 77 1 77 76 1 58 78 1 78 77 1 59 79 1 79 78 1 60 80 1 80 79 1 61 80 1 62 82 1 82 81 1 81 61 1 63 83 1 83 82 1 64 84 1;
	setAttr ".ed[166:331]"  84 83 1 65 85 1 85 84 1 66 86 1 86 85 1 67 87 1 87 86 1 68 88 1 88 87 1 69 89 1 89 88 1 70 90 1 90 89 1 71 91 1 91 90 1 72 92 1 92 91 1 73 93 1 93 92 1 74 94 1 94 93 1 75 95 1 95 94 1 76 96 1 96 95 1 77 97 1 97 96 1 78 98 1 98 97 1 79 99 1 99 98 1 80 100 1 100 99 1 81 100 1 82 102 1 102 101 1 101 81 1 83 103 1 103 102 1 84 104 1 104 103 1 85 105 1 105 104 1 86 106 1 106 105 1 87 107 1 107 106 1 88 108 1 108 107 1 89 109 1 109 108 1 90 110 1 110 109 1 91 111 1 111 110 1 92 112 1 112 111 1 93 113 1 113 112 1 94 114 1 114 113 1 95 115 1 115 114 1 96 116 1 116 115 1 97 117 1 117 116 1 98 118 1 118 117 1 99 119 1 119 118 1 100 120 1 120 119 1 101 120 1 102 122 1 122 121 1 121 101 1 103 123 1 123 122 1 104 124 1 124 123 1 105 125 1 125 124 1 106 126 1 126 125 1 107 127 1 127 126 1 108 128 1 128 127 1 109 129 1 129 128 1 110 130 1 130 129 1 111 131 1 131 130 1 112 132 1 132 131 1 113 133 1 133 132 1 114 134 1 134 133 1 115 135 1 135 134 1 116 136 1 136 135 1 117 137 1 137 136 1 118 138 1 138 137 1 119 139 1 139 138 1 120 140 1 140 139 1 121 140 1 122 142 1 142 141 1 141 121 1 123 143 1 143 142 1 124 144 1 144 143 1 125 145 1 145 144 1 126 146 1 146 145 1 127 147 1 147 146 1 128 148 1 148 147 1 129 149 1 149 148 1 130 150 1 150 149 1 131 151 1 151 150 1 132 152 1 152 151 1 133 153 1 153 152 1 134 154 1 154 153 1 135 155 1 155 154 1 136 156 1 156 155 1 137 157 1 157 156 1 138 158 1 158 157 1 139 159 1 159 158 1 140 160 1 160 159 1 141 160 1 142 162 1 162 161 1 161 141 1 143 163 1 163 162 1 144 164 1 164 163 1 145 165 1 165 164 1 146 166 1 166 165 1 147 167 1;
	setAttr ".ed[332:497]"  167 166 1 148 168 1 168 167 1 149 169 1 169 168 1 150 170 1 170 169 1 151 171 1 171 170 1 152 172 1 172 171 1 153 173 1 173 172 1 154 174 1 174 173 1 155 175 1 175 174 1 156 176 1 176 175 1 157 177 1 177 176 1 158 178 1 178 177 1 159 179 1 179 178 1 160 180 1 180 179 1 161 180 1 162 182 1 182 181 1 181 161 1 163 183 1 183 182 1 164 184 1 184 183 1 165 185 1 185 184 1 166 186 1 186 185 1 167 187 1 187 186 1 168 188 1 188 187 1 169 189 1 189 188 1 170 190 1 190 189 1 171 191 1 191 190 1 172 192 1 192 191 1 173 193 1 193 192 1 174 194 1 194 193 1 175 195 1 195 194 1 176 196 1 196 195 1 177 197 1 197 196 1 178 198 1 198 197 1 179 199 1 199 198 1 180 200 1 200 199 1 181 200 1 182 202 1 202 201 1 201 181 1 183 203 1 203 202 1 184 204 1 204 203 1 185 205 1 205 204 1 186 206 1 206 205 1 187 207 1 207 206 1 188 208 1 208 207 1 189 209 1 209 208 1 190 210 1 210 209 1 191 211 1 211 210 1 192 212 1 212 211 1 193 213 1 213 212 1 194 214 1 214 213 1 195 215 1 215 214 1 196 216 1 216 215 1 197 217 1 217 216 1 198 218 1 218 217 1 199 219 1 219 218 1 200 220 1 220 219 1 201 220 1 202 222 1 222 221 1 221 201 1 203 223 1 223 222 1 204 224 1 224 223 1 205 225 1 225 224 1 206 226 1 226 225 1 207 227 1 227 226 1 208 228 1 228 227 1 209 229 1 229 228 1 210 230 1 230 229 1 211 231 1 231 230 1 212 232 1 232 231 1 213 233 1 233 232 1 214 234 1 234 233 1 215 235 1 235 234 1 216 236 1 236 235 1 217 237 1 237 236 1 218 238 1 238 237 1 219 239 1 239 238 1 220 240 1 240 239 1 221 240 1 222 242 1 242 241 1 241 221 1 223 243 1 243 242 1 224 244 1 244 243 1 225 245 1 245 244 1 226 246 1 246 245 1 227 247 1 247 246 1 228 248 1 248 247 1 229 249 1 249 248 1 230 250 1;
	setAttr ".ed[498:663]"  250 249 1 231 251 1 251 250 1 232 252 1 252 251 1 233 253 1 253 252 1 234 254 1 254 253 1 235 255 1 255 254 1 236 256 1 256 255 1 237 257 1 257 256 1 238 258 1 258 257 1 239 259 1 259 258 1 240 260 1 260 259 1 241 260 1 242 262 1 262 261 1 261 241 1 243 263 1 263 262 1 244 264 1 264 263 1 245 265 1 265 264 1 246 266 1 266 265 1 247 267 1 267 266 1 248 268 1 268 267 1 249 269 1 269 268 1 250 270 1 270 269 1 251 271 1 271 270 1 252 272 1 272 271 1 253 273 1 273 272 1 254 274 1 274 273 1 255 275 1 275 274 1 256 276 1 276 275 1 257 277 1 277 276 1 258 278 1 278 277 1 259 279 1 279 278 1 260 280 1 280 279 1 261 280 1 262 282 1 282 281 1 281 261 1 263 283 1 283 282 1 264 284 1 284 283 1 265 285 1 285 284 1 266 286 1 286 285 1 267 287 1 287 286 1 268 288 1 288 287 1 269 289 1 289 288 1 270 290 1 290 289 1 271 291 1 291 290 1 272 292 1 292 291 1 273 293 1 293 292 1 274 294 1 294 293 1 275 295 1 295 294 1 276 296 1 296 295 1 277 297 1 297 296 1 278 298 1 298 297 1 279 299 1 299 298 1 280 300 1 300 299 1 281 300 1 282 302 1 302 301 1 301 281 1 283 303 1 303 302 1 284 304 1 304 303 1 285 305 1 305 304 1 286 306 1 306 305 1 287 307 1 307 306 1 288 308 1 308 307 1 289 309 1 309 308 1 290 310 1 310 309 1 291 311 1 311 310 1 292 312 1 312 311 1 293 313 1 313 312 1 294 314 1 314 313 1 295 315 1 315 314 1 296 316 1 316 315 1 297 317 1 317 316 1 298 318 1 318 317 1 299 319 1 319 318 1 300 320 1 320 319 1 301 320 1 302 322 1 322 321 1 321 301 1 303 323 1 323 322 1 304 324 1 324 323 1 305 325 1 325 324 1 306 326 1 326 325 1 307 327 1 327 326 1 308 328 1 328 327 1 309 329 1 329 328 1 310 330 1 330 329 1 311 331 1 331 330 1 312 332 1 332 331 1 313 333 1;
	setAttr ".ed[664:779]"  333 332 1 314 334 1 334 333 1 315 335 1 335 334 1 316 336 1 336 335 1 317 337 1 337 336 1 318 338 1 338 337 1 319 339 1 339 338 1 320 340 1 340 339 1 321 340 1 322 342 1 342 341 1 341 321 1 323 343 1 343 342 1 324 344 1 344 343 1 325 345 1 345 344 1 326 346 1 346 345 1 327 347 1 347 346 1 328 348 1 348 347 1 329 349 1 349 348 1 330 350 1 350 349 1 331 351 1 351 350 1 332 352 1 352 351 1 333 353 1 353 352 1 334 354 1 354 353 1 335 355 1 355 354 1 336 356 1 356 355 1 337 357 1 357 356 1 338 358 1 358 357 1 339 359 1 359 358 1 340 360 1 360 359 1 341 360 1 342 362 1 362 361 1 361 341 1 343 363 1 363 362 1 344 364 1 364 363 1 345 365 1 365 364 1 346 366 1 366 365 1 347 367 1 367 366 1 348 368 1 368 367 1 349 369 1 369 368 1 350 370 1 370 369 1 351 371 1 371 370 1 352 372 1 372 371 1 353 373 1 373 372 1 354 374 1 374 373 1 355 375 1 375 374 1 356 376 1 376 375 1 357 377 1 377 376 1 358 378 1 378 377 1 359 379 1 379 378 1 360 380 1 380 379 1 361 380 1 362 381 1 381 361 1 363 381 1 364 381 1 365 381 1 366 381 1 367 381 1 368 381 1 369 381 1 370 381 1 371 381 1 372 381 1 373 381 1 374 381 1 375 381 1 376 381 1 377 381 1 378 381 1 379 381 1 380 381 1;
	setAttr -s 400 -ch 1560 ".fc[0:249]" -type "polyFaces" 
		f 3 0 1 2 mu 0 3 0 1 2 
		f 3 3 4 -1 mu 0 3 3 4 1 
		f 3 5 6 -4 mu 0 3 5 6 4 
		f 3 7 8 -6 mu 0 3 7 8 6 
		f 3 9 10 -8 mu 0 3 9 10 8 
		f 3 11 12 -10 mu 0 3 11 12 10 
		f 3 13 14 -12 mu 0 3 13 14 12 
		f 3 15 16 -14 mu 0 3 15 16 14 
		f 3 17 18 -16 mu 0 3 17 18 16 
		f 3 19 20 -18 mu 0 3 19 20 18 
		f 3 21 22 -20 mu 0 3 21 22 20 
		f 3 23 24 -22 mu 0 3 23 24 22 
		f 3 25 26 -24 mu 0 3 25 26 24 
		f 3 27 28 -26 mu 0 3 27 28 26 
		f 3 29 30 -28 mu 0 3 29 30 28 
		f 3 31 32 -30 mu 0 3 31 32 30 
		f 3 33 34 -32 mu 0 3 33 34 32 
		f 3 35 36 -34 mu 0 3 35 36 34 
		f 3 37 38 -36 mu 0 3 37 38 36 
		f 3 -3 39 -38 mu 0 3 39 40 38 
		f 4 -2 40 41 42 mu 0 4 2 1 41 42 
		f 4 -5 43 44 -41 mu 0 4 1 4 43 41 
		f 4 -7 45 46 -44 mu 0 4 4 6 44 43 
		f 4 -9 47 48 -46 mu 0 4 6 8 45 44 
		f 4 -11 49 50 -48 mu 0 4 8 10 46 45 
		f 4 -13 51 52 -50 mu 0 4 10 12 47 46 
		f 4 -15 53 54 -52 mu 0 4 12 14 48 47 
		f 4 -17 55 56 -54 mu 0 4 14 16 49 48 
		f 4 -19 57 58 -56 mu 0 4 16 18 50 49 
		f 4 -21 59 60 -58 mu 0 4 18 20 51 50 
		f 4 -23 61 62 -60 mu 0 4 20 22 52 51 
		f 4 -25 63 64 -62 mu 0 4 22 24 53 52 
		f 4 -27 65 66 -64 mu 0 4 24 26 54 53 
		f 4 -29 67 68 -66 mu 0 4 26 28 55 54 
		f 4 -31 69 70 -68 mu 0 4 28 30 56 55 
		f 4 -33 71 72 -70 mu 0 4 30 32 57 56 
		f 4 -35 73 74 -72 mu 0 4 32 34 58 57 
		f 4 -37 75 76 -74 mu 0 4 34 36 59 58 
		f 4 -39 77 78 -76 mu 0 4 36 38 60 59 
		f 4 -40 -43 79 -78 mu 0 4 38 40 61 60 
		f 4 -42 80 81 82 mu 0 4 42 41 62 63 
		f 4 -45 83 84 -81 mu 0 4 41 43 64 62 
		f 4 -47 85 86 -84 mu 0 4 43 44 65 64 
		f 4 -49 87 88 -86 mu 0 4 44 45 66 65 
		f 4 -51 89 90 -88 mu 0 4 45 46 67 66 
		f 4 -53 91 92 -90 mu 0 4 46 47 68 67 
		f 4 -55 93 94 -92 mu 0 4 47 48 69 68 
		f 4 -57 95 96 -94 mu 0 4 48 49 70 69 
		f 4 -59 97 98 -96 mu 0 4 49 50 71 70 
		f 4 -61 99 100 -98 mu 0 4 50 51 72 71 
		f 4 -63 101 102 -100 mu 0 4 51 52 73 72 
		f 4 -65 103 104 -102 mu 0 4 52 53 74 73 
		f 4 -67 105 106 -104 mu 0 4 53 54 75 74 
		f 4 -69 107 108 -106 mu 0 4 54 55 76 75 
		f 4 -71 109 110 -108 mu 0 4 55 56 77 76 
		f 4 -73 111 112 -110 mu 0 4 56 57 78 77 
		f 4 -75 113 114 -112 mu 0 4 57 58 79 78 
		f 4 -77 115 116 -114 mu 0 4 58 59 80 79 
		f 4 -79 117 118 -116 mu 0 4 59 60 81 80 
		f 4 -80 -83 119 -118 mu 0 4 60 61 82 81 
		f 4 -82 120 121 122 mu 0 4 63 62 83 84 
		f 4 -85 123 124 -121 mu 0 4 62 64 85 83 
		f 4 -87 125 126 -124 mu 0 4 64 65 86 85 
		f 4 -89 127 128 -126 mu 0 4 65 66 87 86 
		f 4 -91 129 130 -128 mu 0 4 66 67 88 87 
		f 4 -93 131 132 -130 mu 0 4 67 68 89 88 
		f 4 -95 133 134 -132 mu 0 4 68 69 90 89 
		f 4 -97 135 136 -134 mu 0 4 69 70 91 90 
		f 4 -99 137 138 -136 mu 0 4 70 71 92 91 
		f 4 -101 139 140 -138 mu 0 4 71 72 93 92 
		f 4 -103 141 142 -140 mu 0 4 72 73 94 93 
		f 4 -105 143 144 -142 mu 0 4 73 74 95 94 
		f 4 -107 145 146 -144 mu 0 4 74 75 96 95 
		f 4 -109 147 148 -146 mu 0 4 75 76 97 96 
		f 4 -111 149 150 -148 mu 0 4 76 77 98 97 
		f 4 -113 151 152 -150 mu 0 4 77 78 99 98 
		f 4 -115 153 154 -152 mu 0 4 78 79 100 99 
		f 4 -117 155 156 -154 mu 0 4 79 80 101 100 
		f 4 -119 157 158 -156 mu 0 4 80 81 102 101 
		f 4 -120 -123 159 -158 mu 0 4 81 82 103 102 
		f 4 -122 160 161 162 mu 0 4 84 83 104 105 
		f 4 -125 163 164 -161 mu 0 4 83 85 106 104 
		f 4 -127 165 166 -164 mu 0 4 85 86 107 106 
		f 4 -129 167 168 -166 mu 0 4 86 87 108 107 
		f 4 -131 169 170 -168 mu 0 4 87 88 109 108 
		f 4 -133 171 172 -170 mu 0 4 88 89 110 109 
		f 4 -135 173 174 -172 mu 0 4 89 90 111 110 
		f 4 -137 175 176 -174 mu 0 4 90 91 112 111 
		f 4 -139 177 178 -176 mu 0 4 91 92 113 112 
		f 4 -141 179 180 -178 mu 0 4 92 93 114 113 
		f 4 -143 181 182 -180 mu 0 4 93 94 115 114 
		f 4 -145 183 184 -182 mu 0 4 94 95 116 115 
		f 4 -147 185 186 -184 mu 0 4 95 96 117 116 
		f 4 -149 187 188 -186 mu 0 4 96 97 118 117 
		f 4 -151 189 190 -188 mu 0 4 97 98 119 118 
		f 4 -153 191 192 -190 mu 0 4 98 99 120 119 
		f 4 -155 193 194 -192 mu 0 4 99 100 121 120 
		f 4 -157 195 196 -194 mu 0 4 100 101 122 121 
		f 4 -159 197 198 -196 mu 0 4 101 102 123 122 
		f 4 -160 -163 199 -198 mu 0 4 102 103 124 123 
		f 4 -162 200 201 202 mu 0 4 105 104 125 126 
		f 4 -165 203 204 -201 mu 0 4 104 106 127 125 
		f 4 -167 205 206 -204 mu 0 4 106 107 128 127 
		f 4 -169 207 208 -206 mu 0 4 107 108 129 128 
		f 4 -171 209 210 -208 mu 0 4 108 109 130 129 
		f 4 -173 211 212 -210 mu 0 4 109 110 131 130 
		f 4 -175 213 214 -212 mu 0 4 110 111 132 131 
		f 4 -177 215 216 -214 mu 0 4 111 112 133 132 
		f 4 -179 217 218 -216 mu 0 4 112 113 134 133 
		f 4 -181 219 220 -218 mu 0 4 113 114 135 134 
		f 4 -183 221 222 -220 mu 0 4 114 115 136 135 
		f 4 -185 223 224 -222 mu 0 4 115 116 137 136 
		f 4 -187 225 226 -224 mu 0 4 116 117 138 137 
		f 4 -189 227 228 -226 mu 0 4 117 118 139 138 
		f 4 -191 229 230 -228 mu 0 4 118 119 140 139 
		f 4 -193 231 232 -230 mu 0 4 119 120 141 140 
		f 4 -195 233 234 -232 mu 0 4 120 121 142 141 
		f 4 -197 235 236 -234 mu 0 4 121 122 143 142 
		f 4 -199 237 238 -236 mu 0 4 122 123 144 143 
		f 4 -200 -203 239 -238 mu 0 4 123 124 145 144 
		f 4 -202 240 241 242 mu 0 4 126 125 146 147 
		f 4 -205 243 244 -241 mu 0 4 125 127 148 146 
		f 4 -207 245 246 -244 mu 0 4 127 128 149 148 
		f 4 -209 247 248 -246 mu 0 4 128 129 150 149 
		f 4 -211 249 250 -248 mu 0 4 129 130 151 150 
		f 4 -213 251 252 -250 mu 0 4 130 131 152 151 
		f 4 -215 253 254 -252 mu 0 4 131 132 153 152 
		f 4 -217 255 256 -254 mu 0 4 132 133 154 153 
		f 4 -219 257 258 -256 mu 0 4 133 134 155 154 
		f 4 -221 259 260 -258 mu 0 4 134 135 156 155 
		f 4 -223 261 262 -260 mu 0 4 135 136 157 156 
		f 4 -225 263 264 -262 mu 0 4 136 137 158 157 
		f 4 -227 265 266 -264 mu 0 4 137 138 159 158 
		f 4 -229 267 268 -266 mu 0 4 138 139 160 159 
		f 4 -231 269 270 -268 mu 0 4 139 140 161 160 
		f 4 -233 271 272 -270 mu 0 4 140 141 162 161 
		f 4 -235 273 274 -272 mu 0 4 141 142 163 162 
		f 4 -237 275 276 -274 mu 0 4 142 143 164 163 
		f 4 -239 277 278 -276 mu 0 4 143 144 165 164 
		f 4 -240 -243 279 -278 mu 0 4 144 145 166 165 
		f 4 -242 280 281 282 mu 0 4 147 146 167 168 
		f 4 -245 283 284 -281 mu 0 4 146 148 169 167 
		f 4 -247 285 286 -284 mu 0 4 148 149 170 169 
		f 4 -249 287 288 -286 mu 0 4 149 150 171 170 
		f 4 -251 289 290 -288 mu 0 4 150 151 172 171 
		f 4 -253 291 292 -290 mu 0 4 151 152 173 172 
		f 4 -255 293 294 -292 mu 0 4 152 153 174 173 
		f 4 -257 295 296 -294 mu 0 4 153 154 175 174 
		f 4 -259 297 298 -296 mu 0 4 154 155 176 175 
		f 4 -261 299 300 -298 mu 0 4 155 156 177 176 
		f 4 -263 301 302 -300 mu 0 4 156 157 178 177 
		f 4 -265 303 304 -302 mu 0 4 157 158 179 178 
		f 4 -267 305 306 -304 mu 0 4 158 159 180 179 
		f 4 -269 307 308 -306 mu 0 4 159 160 181 180 
		f 4 -271 309 310 -308 mu 0 4 160 161 182 181 
		f 4 -273 311 312 -310 mu 0 4 161 162 183 182 
		f 4 -275 313 314 -312 mu 0 4 162 163 184 183 
		f 4 -277 315 316 -314 mu 0 4 163 164 185 184 
		f 4 -279 317 318 -316 mu 0 4 164 165 186 185 
		f 4 -280 -283 319 -318 mu 0 4 165 166 187 186 
		f 4 -282 320 321 322 mu 0 4 168 167 188 189 
		f 4 -285 323 324 -321 mu 0 4 167 169 190 188 
		f 4 -287 325 326 -324 mu 0 4 169 170 191 190 
		f 4 -289 327 328 -326 mu 0 4 170 171 192 191 
		f 4 -291 329 330 -328 mu 0 4 171 172 193 192 
		f 4 -293 331 332 -330 mu 0 4 172 173 194 193 
		f 4 -295 333 334 -332 mu 0 4 173 174 195 194 
		f 4 -297 335 336 -334 mu 0 4 174 175 196 195 
		f 4 -299 337 338 -336 mu 0 4 175 176 197 196 
		f 4 -301 339 340 -338 mu 0 4 176 177 198 197 
		f 4 -303 341 342 -340 mu 0 4 177 178 199 198 
		f 4 -305 343 344 -342 mu 0 4 178 179 200 199 
		f 4 -307 345 346 -344 mu 0 4 179 180 201 200 
		f 4 -309 347 348 -346 mu 0 4 180 181 202 201 
		f 4 -311 349 350 -348 mu 0 4 181 182 203 202 
		f 4 -313 351 352 -350 mu 0 4 182 183 204 203 
		f 4 -315 353 354 -352 mu 0 4 183 184 205 204 
		f 4 -317 355 356 -354 mu 0 4 184 185 206 205 
		f 4 -319 357 358 -356 mu 0 4 185 186 207 206 
		f 4 -320 -323 359 -358 mu 0 4 186 187 208 207 
		f 4 -322 360 361 362 mu 0 4 189 188 209 210 
		f 4 -325 363 364 -361 mu 0 4 188 190 211 209 
		f 4 -327 365 366 -364 mu 0 4 190 191 212 211 
		f 4 -329 367 368 -366 mu 0 4 191 192 213 212 
		f 4 -331 369 370 -368 mu 0 4 192 193 214 213 
		f 4 -333 371 372 -370 mu 0 4 193 194 215 214 
		f 4 -335 373 374 -372 mu 0 4 194 195 216 215 
		f 4 -337 375 376 -374 mu 0 4 195 196 217 216 
		f 4 -339 377 378 -376 mu 0 4 196 197 218 217 
		f 4 -341 379 380 -378 mu 0 4 197 198 219 218 
		f 4 -343 381 382 -380 mu 0 4 198 199 220 219 
		f 4 -345 383 384 -382 mu 0 4 199 200 221 220 
		f 4 -347 385 386 -384 mu 0 4 200 201 222 221 
		f 4 -349 387 388 -386 mu 0 4 201 202 223 222 
		f 4 -351 389 390 -388 mu 0 4 202 203 224 223 
		f 4 -353 391 392 -390 mu 0 4 203 204 225 224 
		f 4 -355 393 394 -392 mu 0 4 204 205 226 225 
		f 4 -357 395 396 -394 mu 0 4 205 206 227 226 
		f 4 -359 397 398 -396 mu 0 4 206 207 228 227 
		f 4 -360 -363 399 -398 mu 0 4 207 208 229 228 
		f 4 -362 400 401 402 mu 0 4 210 209 230 231 
		f 4 -365 403 404 -401 mu 0 4 209 211 232 230 
		f 4 -367 405 406 -404 mu 0 4 211 212 233 232 
		f 4 -369 407 408 -406 mu 0 4 212 213 234 233 
		f 4 -371 409 410 -408 mu 0 4 213 214 235 234 
		f 4 -373 411 412 -410 mu 0 4 214 215 236 235 
		f 4 -375 413 414 -412 mu 0 4 215 216 237 236 
		f 4 -377 415 416 -414 mu 0 4 216 217 238 237 
		f 4 -379 417 418 -416 mu 0 4 217 218 239 238 
		f 4 -381 419 420 -418 mu 0 4 218 219 240 239 
		f 4 -383 421 422 -420 mu 0 4 219 220 241 240 
		f 4 -385 423 424 -422 mu 0 4 220 221 242 241 
		f 4 -387 425 426 -424 mu 0 4 221 222 243 242 
		f 4 -389 427 428 -426 mu 0 4 222 223 244 243 
		f 4 -391 429 430 -428 mu 0 4 223 224 245 244 
		f 4 -393 431 432 -430 mu 0 4 224 225 246 245 
		f 4 -395 433 434 -432 mu 0 4 225 226 247 246 
		f 4 -397 435 436 -434 mu 0 4 226 227 248 247 
		f 4 -399 437 438 -436 mu 0 4 227 228 249 248 
		f 4 -400 -403 439 -438 mu 0 4 228 229 250 249 
		f 4 -402 440 441 442 mu 0 4 231 230 251 252 
		f 4 -405 443 444 -441 mu 0 4 230 232 253 251 
		f 4 -407 445 446 -444 mu 0 4 232 233 254 253 
		f 4 -409 447 448 -446 mu 0 4 233 234 255 254 
		f 4 -411 449 450 -448 mu 0 4 234 235 256 255 
		f 4 -413 451 452 -450 mu 0 4 235 236 257 256 
		f 4 -415 453 454 -452 mu 0 4 236 237 258 257 
		f 4 -417 455 456 -454 mu 0 4 237 238 259 258 
		f 4 -419 457 458 -456 mu 0 4 238 239 260 259 
		f 4 -421 459 460 -458 mu 0 4 239 240 261 260 
		f 4 -423 461 462 -460 mu 0 4 240 241 262 261 
		f 4 -425 463 464 -462 mu 0 4 241 242 263 262 
		f 4 -427 465 466 -464 mu 0 4 242 243 264 263 
		f 4 -429 467 468 -466 mu 0 4 243 244 265 264 
		f 4 -431 469 470 -468 mu 0 4 244 245 266 265 
		f 4 -433 471 472 -470 mu 0 4 245 246 267 266 
		f 4 -435 473 474 -472 mu 0 4 246 247 268 267 
		f 4 -437 475 476 -474 mu 0 4 247 248 269 268 
		f 4 -439 477 478 -476 mu 0 4 248 249 270 269 
		f 4 -440 -443 479 -478 mu 0 4 249 250 271 270 
		f 4 -442 480 481 482 mu 0 4 252 251 272 273 
		f 4 -445 483 484 -481 mu 0 4 251 253 274 272 
		f 4 -447 485 486 -484 mu 0 4 253 254 275 274 
		f 4 -449 487 488 -486 mu 0 4 254 255 276 275 
		f 4 -451 489 490 -488 mu 0 4 255 256 277 276 
		f 4 -453 491 492 -490 mu 0 4 256 257 278 277 
		f 4 -455 493 494 -492 mu 0 4 257 258 279 278 
		f 4 -457 495 496 -494 mu 0 4 258 259 280 279 
		f 4 -459 497 498 -496 mu 0 4 259 260 281 280 
		f 4 -461 499 500 -498 mu 0 4 260 261 282 281 ;
	setAttr ".fc[250:399]" -type "polyFaces" 
		f 4 -463 501 502 -500 mu 0 4 261 262 283 282 
		f 4 -465 503 504 -502 mu 0 4 262 263 284 283 
		f 4 -467 505 506 -504 mu 0 4 263 264 285 284 
		f 4 -469 507 508 -506 mu 0 4 264 265 286 285 
		f 4 -471 509 510 -508 mu 0 4 265 266 287 286 
		f 4 -473 511 512 -510 mu 0 4 266 267 288 287 
		f 4 -475 513 514 -512 mu 0 4 267 268 289 288 
		f 4 -477 515 516 -514 mu 0 4 268 269 290 289 
		f 4 -479 517 518 -516 mu 0 4 269 270 291 290 
		f 4 -480 -483 519 -518 mu 0 4 270 271 292 291 
		f 4 -482 520 521 522 mu 0 4 273 272 293 294 
		f 4 -485 523 524 -521 mu 0 4 272 274 295 293 
		f 4 -487 525 526 -524 mu 0 4 274 275 296 295 
		f 4 -489 527 528 -526 mu 0 4 275 276 297 296 
		f 4 -491 529 530 -528 mu 0 4 276 277 298 297 
		f 4 -493 531 532 -530 mu 0 4 277 278 299 298 
		f 4 -495 533 534 -532 mu 0 4 278 279 300 299 
		f 4 -497 535 536 -534 mu 0 4 279 280 301 300 
		f 4 -499 537 538 -536 mu 0 4 280 281 302 301 
		f 4 -501 539 540 -538 mu 0 4 281 282 303 302 
		f 4 -503 541 542 -540 mu 0 4 282 283 304 303 
		f 4 -505 543 544 -542 mu 0 4 283 284 305 304 
		f 4 -507 545 546 -544 mu 0 4 284 285 306 305 
		f 4 -509 547 548 -546 mu 0 4 285 286 307 306 
		f 4 -511 549 550 -548 mu 0 4 286 287 308 307 
		f 4 -513 551 552 -550 mu 0 4 287 288 309 308 
		f 4 -515 553 554 -552 mu 0 4 288 289 310 309 
		f 4 -517 555 556 -554 mu 0 4 289 290 311 310 
		f 4 -519 557 558 -556 mu 0 4 290 291 312 311 
		f 4 -520 -523 559 -558 mu 0 4 291 292 313 312 
		f 4 -522 560 561 562 mu 0 4 294 293 314 315 
		f 4 -525 563 564 -561 mu 0 4 293 295 316 314 
		f 4 -527 565 566 -564 mu 0 4 295 296 317 316 
		f 4 -529 567 568 -566 mu 0 4 296 297 318 317 
		f 4 -531 569 570 -568 mu 0 4 297 298 319 318 
		f 4 -533 571 572 -570 mu 0 4 298 299 320 319 
		f 4 -535 573 574 -572 mu 0 4 299 300 321 320 
		f 4 -537 575 576 -574 mu 0 4 300 301 322 321 
		f 4 -539 577 578 -576 mu 0 4 301 302 323 322 
		f 4 -541 579 580 -578 mu 0 4 302 303 324 323 
		f 4 -543 581 582 -580 mu 0 4 303 304 325 324 
		f 4 -545 583 584 -582 mu 0 4 304 305 326 325 
		f 4 -547 585 586 -584 mu 0 4 305 306 327 326 
		f 4 -549 587 588 -586 mu 0 4 306 307 328 327 
		f 4 -551 589 590 -588 mu 0 4 307 308 329 328 
		f 4 -553 591 592 -590 mu 0 4 308 309 330 329 
		f 4 -555 593 594 -592 mu 0 4 309 310 331 330 
		f 4 -557 595 596 -594 mu 0 4 310 311 332 331 
		f 4 -559 597 598 -596 mu 0 4 311 312 333 332 
		f 4 -560 -563 599 -598 mu 0 4 312 313 334 333 
		f 4 -562 600 601 602 mu 0 4 315 314 335 336 
		f 4 -565 603 604 -601 mu 0 4 314 316 337 335 
		f 4 -567 605 606 -604 mu 0 4 316 317 338 337 
		f 4 -569 607 608 -606 mu 0 4 317 318 339 338 
		f 4 -571 609 610 -608 mu 0 4 318 319 340 339 
		f 4 -573 611 612 -610 mu 0 4 319 320 341 340 
		f 4 -575 613 614 -612 mu 0 4 320 321 342 341 
		f 4 -577 615 616 -614 mu 0 4 321 322 343 342 
		f 4 -579 617 618 -616 mu 0 4 322 323 344 343 
		f 4 -581 619 620 -618 mu 0 4 323 324 345 344 
		f 4 -583 621 622 -620 mu 0 4 324 325 346 345 
		f 4 -585 623 624 -622 mu 0 4 325 326 347 346 
		f 4 -587 625 626 -624 mu 0 4 326 327 348 347 
		f 4 -589 627 628 -626 mu 0 4 327 328 349 348 
		f 4 -591 629 630 -628 mu 0 4 328 329 350 349 
		f 4 -593 631 632 -630 mu 0 4 329 330 351 350 
		f 4 -595 633 634 -632 mu 0 4 330 331 352 351 
		f 4 -597 635 636 -634 mu 0 4 331 332 353 352 
		f 4 -599 637 638 -636 mu 0 4 332 333 354 353 
		f 4 -600 -603 639 -638 mu 0 4 333 334 355 354 
		f 4 -602 640 641 642 mu 0 4 336 335 356 357 
		f 4 -605 643 644 -641 mu 0 4 335 337 358 356 
		f 4 -607 645 646 -644 mu 0 4 337 338 359 358 
		f 4 -609 647 648 -646 mu 0 4 338 339 360 359 
		f 4 -611 649 650 -648 mu 0 4 339 340 361 360 
		f 4 -613 651 652 -650 mu 0 4 340 341 362 361 
		f 4 -615 653 654 -652 mu 0 4 341 342 363 362 
		f 4 -617 655 656 -654 mu 0 4 342 343 364 363 
		f 4 -619 657 658 -656 mu 0 4 343 344 365 364 
		f 4 -621 659 660 -658 mu 0 4 344 345 366 365 
		f 4 -623 661 662 -660 mu 0 4 345 346 367 366 
		f 4 -625 663 664 -662 mu 0 4 346 347 368 367 
		f 4 -627 665 666 -664 mu 0 4 347 348 369 368 
		f 4 -629 667 668 -666 mu 0 4 348 349 370 369 
		f 4 -631 669 670 -668 mu 0 4 349 350 371 370 
		f 4 -633 671 672 -670 mu 0 4 350 351 372 371 
		f 4 -635 673 674 -672 mu 0 4 351 352 373 372 
		f 4 -637 675 676 -674 mu 0 4 352 353 374 373 
		f 4 -639 677 678 -676 mu 0 4 353 354 375 374 
		f 4 -640 -643 679 -678 mu 0 4 354 355 376 375 
		f 4 -642 680 681 682 mu 0 4 357 356 377 378 
		f 4 -645 683 684 -681 mu 0 4 356 358 379 377 
		f 4 -647 685 686 -684 mu 0 4 358 359 380 379 
		f 4 -649 687 688 -686 mu 0 4 359 360 381 380 
		f 4 -651 689 690 -688 mu 0 4 360 361 382 381 
		f 4 -653 691 692 -690 mu 0 4 361 362 383 382 
		f 4 -655 693 694 -692 mu 0 4 362 363 384 383 
		f 4 -657 695 696 -694 mu 0 4 363 364 385 384 
		f 4 -659 697 698 -696 mu 0 4 364 365 386 385 
		f 4 -661 699 700 -698 mu 0 4 365 366 387 386 
		f 4 -663 701 702 -700 mu 0 4 366 367 388 387 
		f 4 -665 703 704 -702 mu 0 4 367 368 389 388 
		f 4 -667 705 706 -704 mu 0 4 368 369 390 389 
		f 4 -669 707 708 -706 mu 0 4 369 370 391 390 
		f 4 -671 709 710 -708 mu 0 4 370 371 392 391 
		f 4 -673 711 712 -710 mu 0 4 371 372 393 392 
		f 4 -675 713 714 -712 mu 0 4 372 373 394 393 
		f 4 -677 715 716 -714 mu 0 4 373 374 395 394 
		f 4 -679 717 718 -716 mu 0 4 374 375 396 395 
		f 4 -680 -683 719 -718 mu 0 4 375 376 397 396 
		f 4 -682 720 721 722 mu 0 4 378 377 398 399 
		f 4 -685 723 724 -721 mu 0 4 377 379 400 398 
		f 4 -687 725 726 -724 mu 0 4 379 380 401 400 
		f 4 -689 727 728 -726 mu 0 4 380 381 402 401 
		f 4 -691 729 730 -728 mu 0 4 381 382 403 402 
		f 4 -693 731 732 -730 mu 0 4 382 383 404 403 
		f 4 -695 733 734 -732 mu 0 4 383 384 405 404 
		f 4 -697 735 736 -734 mu 0 4 384 385 406 405 
		f 4 -699 737 738 -736 mu 0 4 385 386 407 406 
		f 4 -701 739 740 -738 mu 0 4 386 387 408 407 
		f 4 -703 741 742 -740 mu 0 4 387 388 409 408 
		f 4 -705 743 744 -742 mu 0 4 388 389 410 409 
		f 4 -707 745 746 -744 mu 0 4 389 390 411 410 
		f 4 -709 747 748 -746 mu 0 4 390 391 412 411 
		f 4 -711 749 750 -748 mu 0 4 391 392 413 412 
		f 4 -713 751 752 -750 mu 0 4 392 393 414 413 
		f 4 -715 753 754 -752 mu 0 4 393 394 415 414 
		f 4 -717 755 756 -754 mu 0 4 394 395 416 415 
		f 4 -719 757 758 -756 mu 0 4 395 396 417 416 
		f 4 -720 -723 759 -758 mu 0 4 396 397 418 417 
		f 3 -722 760 761 mu 0 3 399 398 419 
		f 3 -725 762 -761 mu 0 3 398 400 420 
		f 3 -727 763 -763 mu 0 3 400 401 421 
		f 3 -729 764 -764 mu 0 3 401 402 422 
		f 3 -731 765 -765 mu 0 3 402 403 423 
		f 3 -733 766 -766 mu 0 3 403 404 424 
		f 3 -735 767 -767 mu 0 3 404 405 425 
		f 3 -737 768 -768 mu 0 3 405 406 426 
		f 3 -739 769 -769 mu 0 3 406 407 427 
		f 3 -741 770 -770 mu 0 3 407 408 428 
		f 3 -743 771 -771 mu 0 3 408 409 429 
		f 3 -745 772 -772 mu 0 3 409 410 430 
		f 3 -747 773 -773 mu 0 3 410 411 431 
		f 3 -749 774 -774 mu 0 3 411 412 432 
		f 3 -751 775 -775 mu 0 3 412 413 433 
		f 3 -753 776 -776 mu 0 3 413 414 434 
		f 3 -755 777 -777 mu 0 3 414 415 435 
		f 3 -757 778 -778 mu 0 3 415 416 436 
		f 3 -759 779 -779 mu 0 3 416 417 437 
		f 3 -760 -762 -780 mu 0 3 417 418 438 ;
createNode transform -n "secondBall";
	setAttr ".t" -type "double3" 900 380 -2600 ;
createNode mesh -n "secondBallShape" -p "secondBall";
	setAttr -k off ".v";
	setAttr ".vir" yes;
	setAttr ".vif" yes;
	setAttr ".uvst[0].uvsn" -type "string" "map1";
	setAttr -s 439 ".uvst[0].uvsp";
	setAttr ".uvst[0].uvsp[0:249]" -type "float2" 0.025 0 0.05 0.05 0 0.05 0.075 0 0.1 0.05 0.125 0 0.15 0.05 0.17500001 0 0.2 0.05 0.225 0 0.25 0.05 0.275 0 0.3 0.05 0.325 0 0.35 0.05 0.375 0 0.4 0.05 0.425 0 0.45 0.05 0.475 0 0.5 0.05 0.525 0 0.55 0.05 0.57500005 0 0.6 0.05 0.625 0 0.65 0.05 0.67499995 0 0.7 0.05 0.725 0 0.75 0.05 0.775 0 0.8 0.05 0.82500005 0 0.85 0.05 0.875 0 0.9 0.05 0.92499995 0 0.95 0.05 0.975 0 1 0.05 0.05 0.1 0 0.1 0.1 0.1 0.15 0.1 0.2 0.1 0.25 0.1 0.3 0.1 0.35 0.1 0.4 0.1 0.45 0.1 0.5 0.1 0.55 0.1 0.6 0.1 0.65 0.1 0.7 0.1 0.75 0.1 0.8 0.1 0.85 0.1 0.9 0.1 0.95 0.1 1 0.1 0.05 0.15 0 0.15 0.1 0.15 0.15 0.15 0.2 0.15 0.25 0.15 0.3 0.15 0.35 0.15 0.4 0.15 0.45 0.15 0.5 0.15 0.55 0.15 0.6 0.15 0.65 0.15 0.7 0.15 0.75 0.15 0.8 0.15 0.85 0.15 0.9 0.15 0.95 0.15 1 0.15 0.05 0.2 0 0.2 0.1 0.2 0.15 0.2 0.2 0.2 0.25 0.2 0.3 0.2 0.35 0.2 0.4 0.2 0.45 0.2 0.5 0.2 0.55 0.2 0.6 0.2 0.65 0.2 0.7 0.2 0.75 0.2 0.8 0.2 0.85 0.2 0.9 0.2 0.95 0.2 1 0.2 0.05 0.25 0 0.25 0.1 0.25 0.15 0.25 0.2 0.25 0.25 0.25 0.3 0.25 0.35 0.25 0.4 0.25 0.45 0.25 0.5 0.25 0.55 0.25 0.6 0.25 0.65 0.25 0.7 0.25 0.75 0.25 0.8 0.25 0.85 0.25 0.9 0.25 0.95 0.25 1 0.25 0.05 0.3 0 0.3 0.1 0.3 0.15 0.3 0.2 0.3 0.25 0.3 0.3 0.3 0.35 0.3 0.4 0.3 0.45 0.3 0.5 0.3 0.55 0.3 0.6 0.3 0.65 0.3 0.7 0.3 0.75 0.3 0.8 0.3 0.85 0.3 0.9 0.3 0.95 0.3 1 0.3 0.05 0.35 0 0.35 0.1 0.35 0.15 0.35 0.2 0.35 0.25 0.35 0.3 0.35 0.35 0.35 0.4 0.35 0.45 0.35 0.5 0.35 0.55 0.35 0.6 0.35 0.65 0.35 0.7 0.35 0.75 0.35 0.8 0.35 0.85 0.35 0.9 0.35 0.95 0.35 1 0.35 0.05 0.4 0 0.4 0.1 0.4 0.15 0.4 0.2 0.4 0.25 0.4 0.3 0.4 0.35 0.4 0.4 0.4 0.45 0.4 0.5 0.4 0.55 0.4 0.6 0.4 0.65 0.4 0.7 0.4 0.75 0.4 0.8 0.4 0.85 0.4 0.9 0.4 0.95 0.4 1 0.4 0.05 0.45 0 0.45 0.1 0.45 0.15 0.45 0.2 0.45 0.25 0.45 0.3 0.45 0.35 0.45 0.4 0.45 0.45 0.45 0.5 0.45 0.55 0.45 0.6 0.45 0.65 0.45 0.7 0.45 0.75 0.45 0.8 0.45 0.85 0.45 0.9 0.45 0.95 0.45 1 0.45 0.05 0.5 0 0.5 0.1 0.5 0.15 0.5 0.2 0.5 0.25 0.5 0.3 0.5 0.35 0.5 0.4 0.5 0.45 0.5 0.5 0.5 0.55 0.5 0.6 0.5 0.65 0.5 0.7 0.5 0.75 0.5 0.8 0.5 0.85 0.5 0.9 0.5 0.95 0.5 1 0.5 0.05 0.55 0 0.55 0.1 0.55 0.15 0.55 0.2 0.55 0.25 0.55 0.3 0.55 0.35 0.55 0.4 0.55 0.45 0.55 0.5 0.55 0.55 0.55 0.6 0.55 0.65 0.55 0.7 0.55 0.75 0.55 0.8 0.55 0.85 0.55 0.9 0.55 0.95 0.55 ;
	setAttr ".uvst[0].uvsp[250:438]" -type "float2" 1 0.55 0.05 0.6 0 0.6 0.1 0.6 0.15 0.6 0.2 0.6 0.25 0.6 0.3 0.6 0.35 0.6 0.4 0.6 0.45 0.6 0.5 0.6 0.55 0.6 0.6 0.6 0.65 0.6 0.7 0.6 0.75 0.6 0.8 0.6 0.85 0.6 0.9 0.6 0.95 0.6 1 0.6 0.05 0.65 0 0.65 0.1 0.65 0.15 0.65 0.2 0.65 0.25 0.65 0.3 0.65 0.35 0.65 0.4 0.65 0.45 0.65 0.5 0.65 0.55 0.65 0.6 0.65 0.65 0.65 0.7 0.65 0.75 0.65 0.8 0.65 0.85 0.65 0.9 0.65 0.95 0.65 1 0.65 0.05 0.7 0 0.7 0.1 0.7 0.15 0.7 0.2 0.7 0.25 0.7 0.3 0.7 0.35 0.7 0.4 0.7 0.45 0.7 0.5 0.7 0.55 0.7 0.6 0.7 0.65 0.7 0.7 0.7 0.75 0.7 0.8 0.7 0.85 0.7 0.9 0.7 0.95 0.7 1 0.7 0.05 0.75 0 0.75 0.1 0.75 0.15 0.75 0.2 0.75 0.25 0.75 0.3 0.75 0.35 0.75 0.4 0.75 0.45 0.75 0.5 0.75 0.55 0.75 0.6 0.75 0.65 0.75 0.7 0.75 0.75 0.75 0.8 0.75 0.85 0.75 0.9 0.75 0.95 0.75 1 0.75 0.05 0.8 0 0.8 0.1 0.8 0.15 0.8 0.2 0.8 0.25 0.8 0.3 0.8 0.35 0.8 0.4 0.8 0.45 0.8 0.5 0.8 0.55 0.8 0.6 0.8 0.65 0.8 0.7 0.8 0.75 0.8 0.8 0.8 0.85 0.8 0.9 0.8 0.95 0.8 1 0.8 0.05 0.85 0 0.85 0.1 0.85 0.15 0.85 0.2 0.85 0.25 0.85 0.3 0.85 0.35 0.85 0.4 0.85 0.45 0.85 0.5 0.85 0.55 0.85 0.6 0.85 0.65 0.85 0.7 0.85 0.75 0.85 0.8 0.85 0.85 0.85 0.9 0.85 0.95 0.85 1 0.85 0.05 0.9 0 0.9 0.1 0.9 0.15 0.9 0.2 0.9 0.25 0.9 0.3 0.9 0.35 0.9 0.4 0.9 0.45 0.9 0.5 0.9 0.55 0.9 0.6 0.9 0.65 0.9 0.7 0.9 0.75 0.9 0.8 0.9 0.85 0.9 0.9 0.9 0.95 0.9 1 0.9 0.05 0.95 0 0.95 0.1 0.95 0.15 0.95 0.2 0.95 0.25 0.95 0.3 0.95 0.35 0.95 0.4 0.95 0.45 0.95 0.5 0.95 0.55 0.95 0.6 0.95 0.65 0.95 0.7 0.95 0.75 0.95 0.8 0.95 0.85 0.95 0.9 0.95 0.95 0.95 1 0.95 0.025 1 0.075 1 0.125 1 0.17500001 1 0.225 1 0.275 1 0.325 1 0.375 1 0.425 1 0.475 1 0.525 1 0.57500005 1 0.625 1 0.67499995 1 0.725 1 0.775 1 0.82500005 1 0.875 1 0.92499995 1 0.975 1 ;
	setAttr ".cuvs" -type "string" "map1";
	setAttr ".dcc" -type "string" "Ambient+Diffuse";
	setAttr ".covm[0]"  0 1 1;
	setAttr ".cdvm[0]"  0 1 1;
	setAttr -s 382 ".vt";
	setAttr ".vt[0:165]"  0 -70 0 10.95041255281616 -69.13818384165964 0 10.414461214476058 -69.13818384165964 -3.383863574236945 8.859069850645026 -69.13818384165964 -6.436491005063712 6.436491005063712 -69.13818384165964 -8.859069850645026 3.3838635742369454 -69.13818384165964 -10.414461214476058 0.0000000000000006705193841074654 -69.13818384165964 -10.95041255281616 -3.3838635742369445 -69.13818384165964 -10.414461214476058 -6.4364910050637105 -69.13818384165964 -8.859069850645026 -8.859069850645025 -69.13818384165964 -6.436491005063713 -10.414461214476058 -69.13818384165964 -3.3838635742369463 -10.95041255281616 -69.13818384165964 -0.0000000000000013410387682149309 -10.41446121447606 -69.13818384165964 3.3838635742369396 -8.859069850645028 -69.13818384165964 6.4364910050637105 -6.436491005063713 -69.13818384165964 8.859069850645025 -3.3838635742369467 -69.13818384165964 10.414461214476058 -0.000000000000002011558152322396 -69.13818384165964 10.95041255281616 3.383863574236943 -69.13818384165964 10.414461214476058 6.43649100506371 -69.13818384165964 8.859069850645028 8.859069850645025 -69.13818384165964 6.436491005063714 10.414461214476058 -69.13818384165964 3.383863574236947 21.631189606246316 -66.57395614066074 0 20.572483830236557 -66.57395614066074 -6.684405196876838 17.499999999999996 -66.57395614066074 -12.714494240093813 12.714494240093813 -66.57395614066074 -17.499999999999996 6.68440519687684 -66.57395614066074 -20.572483830236557 0.0000000000000013245283556519523 -66.57395614066074 -21.631189606246316 -6.684405196876837 -66.57395614066074 -20.572483830236557 -12.714494240093812 -66.57395614066074 -17.499999999999996 -17.499999999999996 -66.57395614066074 -12.714494240093815 -20.572483830236557 -66.57395614066074 -6.684405196876841 -21.631189606246316 -66.57395614066074 -0.0000000000000026490567113039045 -20.57248383023656 -66.57395614066074 6.684405196876828 -17.5 -66.57395614066074 12.714494240093812 -12.714494240093815 -66.57395614066074 17.499999999999996 -6.684405196876842 -66.57395614066074 20.572483830236557 -0.000000000000003973585066955857 -66.57395614066074 21.631189606246316 6.684405196876835 -66.57395614066074 20.572483830236557 12.714494240093808 -66.57395614066074 17.5 17.499999999999996 -66.57395614066074 12.714494240093819 20.572483830236557 -66.57395614066074 6.684405196876844 31.779334981768272 -62.37045669318575 0 30.22394361793724 -62.37045669318575 -9.820354579300655 25.71002207018479 -62.37045669318575 -18.67942442994568 18.67942442994568 -62.37045669318575 -25.71002207018479 9.820354579300657 -62.37045669318575 -30.22394361793724 0.0000000000000019459230432227012 -62.37045669318575 -31.779334981768272 -9.820354579300654 -62.37045669318575 -30.223943617937245 -18.679424429945676 -62.37045669318575 -25.71002207018479 -25.710022070184788 -62.37045669318575 -18.679424429945684 -30.22394361793724 -62.37045669318575 -9.820354579300659 -31.779334981768272 -62.37045669318575 -0.0000000000000038918460864454025 -30.22394361793725 -62.37045669318575 9.82035457930064 -25.710022070184795 -62.37045669318575 18.679424429945676 -18.679424429945684 -62.37045669318575 25.710022070184788 -9.82035457930066 -62.37045669318575 30.22394361793724 -0.000000000000005837769129668104 -62.37045669318575 31.779334981768272 9.82035457930065 -62.37045669318575 30.223943617937245 18.679424429945673 -62.37045669318575 25.710022070184795 25.710022070184788 -62.37045669318575 18.679424429945687 30.22394361793724 -62.37045669318575 9.820354579300663 41.14496766047312 -56.63118960624632 0 39.131189606246316 -56.63118960624632 -12.714494240093815 33.28697807033038 -56.63118960624632 -24.184405196876842 24.184405196876842 -56.63118960624632 -33.28697807033038 12.714494240093817 -56.63118960624632 -39.131189606246316 0.0000000000000025194026473209883 -56.63118960624632 -41.14496766047312 -12.714494240093813 -56.63118960624632 -39.13118960624632 -24.18440519687684 -56.63118960624632 -33.28697807033038 -33.28697807033037 -56.63118960624632 -24.184405196876845 -39.131189606246316 -56.63118960624632 -12.714494240093819 -41.14496766047312 -56.63118960624632 -0.000000000000005038805294641977 -39.13118960624633 -56.63118960624632 12.714494240093794 -33.286978070330385 -56.63118960624632 24.18440519687684 -24.184405196876845 -56.63118960624632 33.28697807033037 -12.714494240093822 -56.63118960624632 39.131189606246316 -0.000000000000007558207941962964 -56.63118960624632 41.14496766047312 12.714494240093808 -56.63118960624632 39.13118960624632 24.18440519687683 -56.63118960624632 33.286978070330385 33.28697807033037 -56.63118960624632 24.18440519687685 39.131189606246316 -56.63118960624632 12.714494240093824 49.49747468305832 -49.49747468305833 0 47.07489583747701 -49.49747468305833 -15.295560855708734 40.0442981972379 -49.49747468305833 -29.093885644421736 29.093885644421736 -49.49747468305833 -40.0442981972379 15.295560855708738 -49.49747468305833 -47.07489583747701 0.000000000000003030846196824226 -49.49747468305833 -49.49747468305832 -15.295560855708732 -49.49747468305833 -47.07489583747701 -29.093885644421732 -49.49747468305833 -40.0442981972379 -40.04429819723789 -49.49747468305833 -29.093885644421743 -47.07489583747701 -49.49747468305833 -15.29556085570874 -49.49747468305832 -49.49747468305833 -0.000000000000006061692393648452 -47.074895837477015 -49.49747468305833 15.29556085570871 -40.0442981972379 -49.49747468305833 29.093885644421732 -29.093885644421743 -49.49747468305833 40.04429819723789 -15.295560855708743 -49.49747468305833 47.07489583747701 -0.000000000000009092538590472679 -49.49747468305833 49.49747468305832 15.295560855708727 -49.49747468305833 47.07489583747701 29.093885644421725 -49.49747468305833 40.0442981972379 40.04429819723789 -49.49747468305833 29.09388564442175 47.07489583747701 -49.49747468305833 15.295560855708745 56.63118960624632 -41.14496766047312 0 53.859461900566934 -41.14496766047312 -17.5 45.81559480312316 -41.14496766047312 -33.28697807033038 33.28697807033038 -41.14496766047312 -45.81559480312316 17.500000000000004 -41.14496766047312 -53.859461900566934 0.000000000000003467660254159821 -41.14496766047312 -56.63118960624632 -17.499999999999996 -41.14496766047312 -53.85946190056694 -33.28697807033037 -41.14496766047312 -45.81559480312316 -45.815594803123155 -41.14496766047312 -33.286978070330385 -53.859461900566934 -41.14496766047312 -17.500000000000007 -56.63118960624632 -41.14496766047312 -0.000000000000006935320508319642 -53.85946190056695 -41.14496766047312 17.49999999999997 -45.81559480312317 -41.14496766047312 33.28697807033037 -33.286978070330385 -41.14496766047312 45.815594803123155 -17.50000000000001 -41.14496766047312 53.859461900566934 -0.000000000000010402980762479462 -41.14496766047312 56.63118960624632 17.49999999999999 -41.14496766047312 53.85946190056694 33.28697807033036 -41.14496766047312 45.81559480312317 45.815594803123155 -41.14496766047312 33.28697807033039 53.859461900566934 -41.14496766047312 17.50000000000001 62.370456693185744 -31.779334981768276 0 59.31782926235898 -31.779334981768276 -19.27353106512108 50.458759411713956 -31.779334981768276 -36.66043462300095 36.66043462300095 -31.779334981768276 -50.458759411713956 19.273531065121084 -31.779334981768276 -59.31782926235898 0.0000000000000038190890075334265 -31.779334981768276 -62.370456693185744 -19.273531065121077 -31.779334981768276 -59.317829262358984 -36.660434623000945 -31.779334981768276 -50.458759411713956 -50.45875941171395 -31.779334981768276 -36.66043462300096 -59.31782926235898 -31.779334981768276 -19.273531065121087 -62.370456693185744 -31.779334981768276 -0.000000000000007638178015066853 -59.31782926235899 -31.779334981768276 19.27353106512105 -50.45875941171396 -31.779334981768276 36.660434623000945 -36.66043462300096 -31.779334981768276 50.45875941171395 -19.27353106512109 -31.779334981768276 59.31782926235898 -0.000000000000011457267022600279 -31.779334981768276 62.370456693185744 19.27353106512107 -31.779334981768276 59.317829262358984 36.66043462300094 -31.779334981768276 50.45875941171396 50.45875941171395 -31.779334981768276 36.66043462300097 59.31782926235898 -31.779334981768276 19.273531065121094 66.57395614066074 -21.631189606246323 0 63.31559480312315 -21.631189606246323 -20.572483830236553 53.85946190056693 -21.631189606246323 -39.131189606246316 39.131189606246316 -21.631189606246323 -53.85946190056693 20.572483830236557 -21.631189606246323 -63.31559480312315 0.0000000000000040764791147118225 -21.631189606246323 -66.57395614066074 -20.57248383023655 -21.631189606246323 -63.315594803123155 -39.13118960624631 -21.631189606246323 -53.85946190056693 -53.85946190056692 -21.631189606246323 -39.13118960624632 -63.31559480312315 -21.631189606246323 -20.57248383023656 -66.57395614066074 -21.631189606246323 -0.000000000000008152958229423645 -63.31559480312316 -21.631189606246323 20.57248383023652 -53.859461900566934 -21.631189606246323 39.13118960624631 -39.13118960624632 -21.631189606246323 53.85946190056692 -20.572483830236568 -21.631189606246323 63.31559480312315 -0.000000000000012229437344135468 -21.631189606246323 66.57395614066074 20.572483830236543 -21.631189606246323 63.315594803123155 39.1311896062463 -21.631189606246323 53.859461900566934 53.85946190056692 -21.631189606246323 39.13118960624633 63.31559480312315 -21.631189606246323 20.57248383023657 69.13818384165964 -10.950412552816164 0 65.75432026742268 -10.950412552816164 -21.364873767292217 55.933965688122036 -10.950412552816164 -40.6384048324133 40.6384048324133 -10.950412552816164 -55.933965688122036 21.36487376729222 -10.950412552816164 -65.75432026742268;
	setAttr ".vt[166:331]"  0.0000000000000042334927770274865 -10.950412552816164 -69.13818384165964 -21.364873767292213 -10.950412552816164 -65.7543202674227 -40.63840483241329 -10.950412552816164 -55.933965688122036 -55.93396568812203 -10.950412552816164 -40.63840483241331 -65.75432026742268 -10.950412552816164 -21.364873767292224 -69.13818384165964 -10.950412552816164 -0.000000000000008466985554054973 -65.75432026742271 -10.950412552816164 21.36487376729218 -55.93396568812204 -10.950412552816164 40.63840483241329 -40.63840483241331 -10.950412552816164 55.93396568812203 -21.364873767292227 -10.950412552816164 65.75432026742268 -0.000000000000012700478331082459 -10.950412552816164 69.13818384165964 21.364873767292206 -10.950412552816164 65.7543202674227 40.638404832413286 -10.950412552816164 55.93396568812204 55.93396568812203 -10.950412552816164 40.638404832413315 65.75432026742268 -10.950412552816164 21.36487376729223 70 -0.0000000000000042862637970157365 0 66.57395614066074 -0.0000000000000042862637970157365 -21.631189606246316 56.63118960624632 -0.0000000000000042862637970157365 -41.14496766047312 41.14496766047312 -0.0000000000000042862637970157365 -56.63118960624632 21.631189606246323 -0.0000000000000042862637970157365 -66.57395614066074 0.0000000000000042862637970157365 -0.0000000000000042862637970157365 -70 -21.631189606246313 -0.0000000000000042862637970157365 -66.57395614066075 -41.144967660473114 -0.0000000000000042862637970157365 -56.63118960624632 -56.631189606246316 -0.0000000000000042862637970157365 -41.14496766047313 -66.57395614066074 -0.0000000000000042862637970157365 -21.631189606246327 -70 -0.0000000000000042862637970157365 -0.000000000000008572527594031473 -66.57395614066077 -0.0000000000000042862637970157365 21.631189606246284 -56.63118960624633 -0.0000000000000042862637970157365 41.144967660473114 -41.14496766047313 -0.0000000000000042862637970157365 56.631189606246316 -21.63118960624633 -0.0000000000000042862637970157365 66.57395614066074 -0.000000000000012858791391047208 -0.0000000000000042862637970157365 70 21.631189606246306 -0.0000000000000042862637970157365 66.57395614066075 41.14496766047311 -0.0000000000000042862637970157365 56.63118960624633 56.631189606246316 -0.0000000000000042862637970157365 41.144967660473135 66.57395614066074 -0.0000000000000042862637970157365 21.631189606246334 69.13818384165964 10.950412552816141 0 65.75432026742268 10.950412552816141 -21.364873767292217 55.933965688122036 10.950412552816141 -40.6384048324133 40.6384048324133 10.950412552816141 -55.933965688122036 21.36487376729222 10.950412552816141 -65.75432026742268 0.0000000000000042334927770274865 10.950412552816141 -69.13818384165964 -21.364873767292213 10.950412552816141 -65.7543202674227 -40.63840483241329 10.950412552816141 -55.933965688122036 -55.93396568812203 10.950412552816141 -40.63840483241331 -65.75432026742268 10.950412552816141 -21.364873767292224 -69.13818384165964 10.950412552816141 -0.000000000000008466985554054973 -65.75432026742271 10.950412552816141 21.36487376729218 -55.93396568812204 10.950412552816141 40.63840483241329 -40.63840483241331 10.950412552816141 55.93396568812203 -21.364873767292227 10.950412552816141 65.75432026742268 -0.000000000000012700478331082459 10.950412552816141 69.13818384165964 21.364873767292206 10.950412552816141 65.7543202674227 40.638404832413286 10.950412552816141 55.93396568812204 55.93396568812203 10.950412552816141 40.638404832413315 65.75432026742268 10.950412552816141 21.36487376729223 66.57395614066075 21.631189606246313 0 63.31559480312316 21.631189606246313 -20.57248383023656 53.85946190056694 21.631189606246313 -39.13118960624632 39.13118960624632 21.631189606246313 -53.85946190056694 20.572483830236564 21.631189606246313 -63.31559480312316 0.000000000000004076479114711823 21.631189606246313 -66.57395614066075 -20.572483830236557 21.631189606246313 -63.31559480312317 -39.131189606246316 21.631189606246313 -53.85946190056694 -53.859461900566934 21.631189606246313 -39.13118960624633 -63.31559480312316 21.631189606246313 -20.572483830236568 -66.57395614066075 21.631189606246313 -0.000000000000008152958229423647 -63.315594803123176 21.631189606246313 20.572483830236525 -53.85946190056695 21.631189606246313 39.131189606246316 -39.13118960624633 21.631189606246313 53.859461900566934 -20.57248383023657 21.631189606246313 63.31559480312316 -0.00000000000001222943734413547 21.631189606246313 66.57395614066075 20.57248383023655 21.631189606246313 63.31559480312317 39.13118960624631 21.631189606246313 53.85946190056695 53.859461900566934 21.631189606246313 39.13118960624634 63.31559480312316 21.631189606246313 20.572483830236575 62.37045669318575 31.77933498176827 0 59.317829262358984 31.77933498176827 -19.27353106512108 50.45875941171396 31.77933498176827 -36.66043462300096 36.66043462300096 31.77933498176827 -50.45875941171396 19.273531065121084 31.77933498176827 -59.317829262358984 0.000000000000003819089007533427 31.77933498176827 -62.37045669318575 -19.273531065121077 31.77933498176827 -59.31782926235899 -36.66043462300095 31.77933498176827 -50.45875941171396 -50.458759411713956 31.77933498176827 -36.66043462300097 -59.317829262358984 31.77933498176827 -19.273531065121087 -62.37045669318575 31.77933498176827 -0.000000000000007638178015066855 -59.317829262359 31.77933498176827 19.273531065121052 -50.45875941171397 31.77933498176827 36.66043462300095 -36.66043462300097 31.77933498176827 50.458759411713956 -19.27353106512109 31.77933498176827 59.317829262358984 -0.00000000000001145726702260028 31.77933498176827 62.37045669318575 19.27353106512107 31.77933498176827 59.31782926235899 36.660434623000945 31.77933498176827 50.45875941171397 50.458759411713956 31.77933498176827 36.660434623000974 59.317829262358984 31.77933498176827 19.273531065121094 56.63118960624632 41.144967660473114 0 53.859461900566934 41.144967660473114 -17.5 45.81559480312316 41.144967660473114 -33.28697807033038 33.28697807033038 41.144967660473114 -45.81559480312316 17.500000000000004 41.144967660473114 -53.859461900566934 0.000000000000003467660254159821 41.144967660473114 -56.63118960624632 -17.499999999999996 41.144967660473114 -53.85946190056694 -33.28697807033037 41.144967660473114 -45.81559480312316 -45.815594803123155 41.144967660473114 -33.286978070330385 -53.859461900566934 41.144967660473114 -17.500000000000007 -56.63118960624632 41.144967660473114 -0.000000000000006935320508319642 -53.85946190056695 41.144967660473114 17.49999999999997 -45.81559480312317 41.144967660473114 33.28697807033037 -33.286978070330385 41.144967660473114 45.815594803123155 -17.50000000000001 41.144967660473114 53.859461900566934 -0.000000000000010402980762479462 41.144967660473114 56.63118960624632 17.49999999999999 41.144967660473114 53.85946190056694 33.28697807033036 41.144967660473114 45.81559480312317 45.815594803123155 41.144967660473114 33.28697807033039 53.859461900566934 41.144967660473114 17.50000000000001 49.49747468305833 49.49747468305832 0 47.074895837477015 49.49747468305832 -15.295560855708736 40.0442981972379 49.49747468305832 -29.093885644421743 29.093885644421743 49.49747468305832 -40.0442981972379 15.29556085570874 49.49747468305832 -47.074895837477015 0.0000000000000030308461968242265 49.49747468305832 -49.49747468305833 -15.295560855708734 49.49747468305832 -47.074895837477015 -29.093885644421736 49.49747468305832 -40.0442981972379 -40.0442981972379 49.49747468305832 -29.093885644421746 -47.074895837477015 49.49747468305832 -15.295560855708743 -49.49747468305833 49.49747468305832 -0.000000000000006061692393648453 -47.07489583747702 49.49747468305832 15.295560855708711 -40.04429819723791 49.49747468305832 29.093885644421736 -29.093885644421746 49.49747468305832 40.0442981972379 -15.295560855708745 49.49747468305832 47.074895837477015 -0.000000000000009092538590472679 49.49747468305832 49.49747468305833 15.295560855708729 49.49747468305832 47.074895837477015 29.093885644421732 49.49747468305832 40.04429819723791 40.0442981972379 49.49747468305832 29.093885644421754 47.074895837477015 49.49747468305832 15.295560855708748 41.14496766047313 56.631189606246316 0 39.13118960624632 56.631189606246316 -12.714494240093817 33.286978070330385 56.631189606246316 -24.184405196876845 24.184405196876845 56.631189606246316 -33.286978070330385 12.714494240093819 56.631189606246316 -39.13118960624632 0.0000000000000025194026473209887 56.631189606246316 -41.14496766047313 -12.714494240093815 56.631189606246316 -39.13118960624633 -24.184405196876842 56.631189606246316 -33.286978070330385 -33.28697807033038 56.631189606246316 -24.18440519687685 -39.13118960624632 56.631189606246316 -12.714494240093822 -41.14496766047313 56.631189606246316 -0.0000000000000050388052946419774 -39.13118960624634 56.631189606246316 12.714494240093796 -33.286978070330385 56.631189606246316 24.184405196876842 -24.18440519687685 56.631189606246316 33.28697807033038 -12.714494240093824 56.631189606246316 39.13118960624632 -0.000000000000007558207941962966 56.631189606246316 41.14496766047313 12.71449424009381 56.631189606246316 39.13118960624633 24.18440519687684 56.631189606246316 33.286978070330385 33.28697807033038 56.631189606246316 24.184405196876856 39.13118960624632 56.631189606246316 12.714494240093826 31.77933498176828 62.370456693185744 0 30.223943617937245 62.370456693185744 -9.820354579300657 25.7100220701848 62.370456693185744 -18.679424429945687 18.679424429945687 62.370456693185744 -25.7100220701848 9.820354579300659 62.370456693185744 -30.223943617937245 0.0000000000000019459230432227016 62.370456693185744 -31.77933498176828 -9.820354579300655 62.370456693185744 -30.22394361793725 -18.679424429945684 62.370456693185744 -25.7100220701848 -25.710022070184795 62.370456693185744 -18.67942442994569 -30.223943617937245 62.370456693185744 -9.82035457930066 -31.77933498176828 62.370456693185744 -0.000000000000003891846086445403;
	setAttr ".vt[332:381]"  -30.223943617937252 62.370456693185744 9.820354579300641 -25.710022070184802 62.370456693185744 18.679424429945684 -18.67942442994569 62.370456693185744 25.710022070184795 -9.820354579300663 62.370456693185744 30.223943617937245 -0.0000000000000058377691296681045 62.370456693185744 31.77933498176828 9.820354579300652 62.370456693185744 30.22394361793725 18.67942442994568 62.370456693185744 25.710022070184802 25.710022070184795 62.370456693185744 18.679424429945694 30.223943617937245 62.370456693185744 9.820354579300664 21.631189606246327 66.57395614066074 0 20.572483830236564 66.57395614066074 -6.684405196876842 17.500000000000007 66.57395614066074 -12.71449424009382 12.71449424009382 66.57395614066074 -17.500000000000007 6.684405196876843 66.57395614066074 -20.572483830236564 0.000000000000001324528355651953 66.57395614066074 -21.631189606246327 -6.684405196876841 66.57395614066074 -20.572483830236568 -12.714494240093817 66.57395614066074 -17.500000000000007 -17.500000000000004 66.57395614066074 -12.714494240093822 -20.572483830236564 66.57395614066074 -6.6844051968768445 -21.631189606246327 66.57395614066074 -0.000000000000002649056711303906 -20.57248383023657 66.57395614066074 6.684405196876831 -17.500000000000007 66.57395614066074 12.714494240093817 -12.714494240093822 66.57395614066074 17.500000000000004 -6.684405196876845 66.57395614066074 20.572483830236564 -0.000000000000003973585066955858 66.57395614066074 21.631189606246327 6.684405196876838 66.57395614066074 20.572483830236568 12.714494240093815 66.57395614066074 17.500000000000007 17.500000000000004 66.57395614066074 12.714494240093824 20.572483830236564 66.57395614066074 6.684405196876846 10.950412552816168 69.13818384165964 0 10.414461214476063 69.13818384165964 -3.383863574236947 8.859069850645032 69.13818384165964 -6.436491005063716 6.436491005063716 69.13818384165964 -8.859069850645032 3.3838635742369476 69.13818384165964 -10.414461214476063 0.0000000000000006705193841074658 69.13818384165964 -10.950412552816168 -3.3838635742369467 69.13818384165964 -10.414461214476065 -6.436491005063715 69.13818384165964 -8.859069850645032 -8.85906985064503 69.13818384165964 -6.436491005063718 -10.414461214476063 69.13818384165964 -3.3838635742369485 -10.950412552816168 69.13818384165964 -0.0000000000000013410387682149316 -10.414461214476066 69.13818384165964 3.383863574236942 -8.859069850645033 69.13818384165964 6.436491005063715 -6.436491005063718 69.13818384165964 8.85906985064503 -3.383863574236949 69.13818384165964 10.414461214476063 -0.0000000000000020115581523223973 69.13818384165964 10.950412552816168 3.3838635742369454 69.13818384165964 10.414461214476065 6.436491005063714 69.13818384165964 8.859069850645033 8.85906985064503 69.13818384165964 6.4364910050637185 10.414461214476063 69.13818384165964 3.3838635742369494 0 70 0;
	setAttr -s 780 ".ed";
	setAttr ".ed[0:165]"  0 2 1 2 1 1 1 0 1 0 3 1 3 2 1 0 4 1 4 3 1 0 5 1 5 4 1 0 6 1 6 5 1 0 7 1 7 6 1 0 8 1 8 7 1 0 9 1 9 8 1 0 10 1 10 9 1 0 11 1 11 10 1 0 12 1 12 11 1 0 13 1 13 12 1 0 14 1 14 13 1 0 15 1 15 14 1 0 16 1 16 15 1 0 17 1 17 16 1 0 18 1 18 17 1 0 19 1 19 18 1 0 20 1 20 19 1 1 20 1 2 22 1 22 21 1 21 1 1 3 23 1 23 22 1 4 24 1 24 23 1 5 25 1 25 24 1 6 26 1 26 25 1 7 27 1 27 26 1 8 28 1 28 27 1 9 29 1 29 28 1 10 30 1 30 29 1 11 31 1 31 30 1 12 32 1 32 31 1 13 33 1 33 32 1 14 34 1 34 33 1 15 35 1 35 34 1 16 36 1 36 35 1 17 37 1 37 36 1 18 38 1 38 37 1 19 39 1 39 38 1 20 40 1 40 39 1 21 40 1 22 42 1 42 41 1 41 21 1 23 43 1 43 42 1 24 44 1 44 43 1 25 45 1 45 44 1 26 46 1 46 45 1 27 47 1 47 46 1 28 48 1 48 47 1 29 49 1 49 48 1 30 50 1 50 49 1 31 51 1 51 50 1 32 52 1 52 51 1 33 53 1 53 52 1 34 54 1 54 53 1 35 55 1 55 54 1 36 56 1 56 55 1 37 57 1 57 56 1 38 58 1 58 57 1 39 59 1 59 58 1 40 60 1 60 59 1 41 60 1 42 62 1 62 61 1 61 41 1 43 63 1 63 62 1 44 64 1 64 63 1 45 65 1 65 64 1 46 66 1 66 65 1 47 67 1 67 66 1 48 68 1 68 67 1 49 69 1 69 68 1 50 70 1 70 69 1 51 71 1 71 70 1 52 72 1 72 71 1 53 73 1 73 72 1 54 74 1 74 73 1 55 75 1 75 74 1 56 76 1 76 75 1 57 77 1 77 76 1 58 78 1 78 77 1 59 79 1 79 78 1 60 80 1 80 79 1 61 80 1 62 82 1 82 81 1 81 61 1 63 83 1 83 82 1 64 84 1;
	setAttr ".ed[166:331]"  84 83 1 65 85 1 85 84 1 66 86 1 86 85 1 67 87 1 87 86 1 68 88 1 88 87 1 69 89 1 89 88 1 70 90 1 90 89 1 71 91 1 91 90 1 72 92 1 92 91 1 73 93 1 93 92 1 74 94 1 94 93 1 75 95 1 95 94 1 76 96 1 96 95 1 77 97 1 97 96 1 78 98 1 98 97 1 79 99 1 99 98 1 80 100 1 100 99 1 81 100 1 82 102 1 102 101 1 101 81 1 83 103 1 103 102 1 84 104 1 104 103 1 85 105 1 105 104 1 86 106 1 106 105 1 87 107 1 107 106 1 88 108 1 108 107 1 89 109 1 109 108 1 90 110 1 110 109 1 91 111 1 111 110 1 92 112 1 112 111 1 93 113 1 113 112 1 94 114 1 114 113 1 95 115 1 115 114 1 96 116 1 116 115 1 97 117 1 117 116 1 98 118 1 118 117 1 99 119 1 119 118 1 100 120 1 120 119 1 101 120 1 102 122 1 122 121 1 121 101 1 103 123 1 123 122 1 104 124 1 124 123 1 105 125 1 125 124 1 106 126 1 126 125 1 107 127 1 127 126 1 108 128 1 128 127 1 109 129 1 129 128 1 110 130 1 130 129 1 111 131 1 131 130 1 112 132 1 132 131 1 113 133 1 133 132 1 114 134 1 134 133 1 115 135 1 135 134 1 116 136 1 136 135 1 117 137 1 137 136 1 118 138 1 138 137 1 119 139 1 139 138 1 120 140 1 140 139 1 121 140 1 122 142 1 142 141 1 141 121 1 123 143 1 143 142 1 124 144 1 144 143 1 125 145 1 145 144 1 126 146 1 146 145 1 127 147 1 147 146 1 128 148 1 148 147 1 129 149 1 149 148 1 130 150 1 150 149 1 131 151 1 151 150 1 132 152 1 152 151 1 133 153 1 153 152 1 134 154 1 154 153 1 135 155 1 155 154 1 136 156 1 156 155 1 137 157 1 157 156 1 138 158 1 158 157 1 139 159 1 159 158 1 140 160 1 160 159 1 141 160 1 142 162 1 162 161 1 161 141 1 143 163 1 163 162 1 144 164 1 164 163 1 145 165 1 165 164 1 146 166 1 166 165 1 147 167 1;
	setAttr ".ed[332:497]"  167 166 1 148 168 1 168 167 1 149 169 1 169 168 1 150 170 1 170 169 1 151 171 1 171 170 1 152 172 1 172 171 1 153 173 1 173 172 1 154 174 1 174 173 1 155 175 1 175 174 1 156 176 1 176 175 1 157 177 1 177 176 1 158 178 1 178 177 1 159 179 1 179 178 1 160 180 1 180 179 1 161 180 1 162 182 1 182 181 1 181 161 1 163 183 1 183 182 1 164 184 1 184 183 1 165 185 1 185 184 1 166 186 1 186 185 1 167 187 1 187 186 1 168 188 1 188 187 1 169 189 1 189 188 1 170 190 1 190 189 1 171 191 1 191 190 1 172 192 1 192 191 1 173 193 1 193 192 1 174 194 1 194 193 1 175 195 1 195 194 1 176 196 1 196 195 1 177 197 1 197 196 1 178 198 1 198 197 1 179 199 1 199 198 1 180 200 1 200 199 1 181 200 1 182 202 1 202 201 1 201 181 1 183 203 1 203 202 1 184 204 1 204 203 1 185 205 1 205 204 1 186 206 1 206 205 1 187 207 1 207 206 1 188 208 1 208 207 1 189 209 1 209 208 1 190 210 1 210 209 1 191 211 1 211 210 1 192 212 1 212 211 1 193 213 1 213 212 1 194 214 1 214 213 1 195 215 1 215 214 1 196 216 1 216 215 1 197 217 1 217 216 1 198 218 1 218 217 1 199 219 1 219 218 1 200 220 1 220 219 1 201 220 1 202 222 1 222 221 1 221 201 1 203 223 1 223 222 1 204 224 1 224 223 1 205 225 1 225 224 1 206 226 1 226 225 1 207 227 1 227 226 1 208 228 1 228 227 1 209 229 1 229 228 1 210 230 1 230 229 1 211 231 1 231 230 1 212 232 1 232 231 1 213 233 1 233 232 1 214 234 1 234 233 1 215 235 1 235 234 1 216 236 1 236 235 1 217 237 1 237 236 1 218 238 1 238 237 1 219 239 1 239 238 1 220 240 1 240 239 1 221 240 1 222 242 1 242 241 1 241 221 1 223 243 1 243 242 1 224 244 1 244 243 1 225 245 1 245 244 1 226 246 1 246 245 1 227 247 1 247 246 1 228 248 1 248 247 1 229 249 1 249 248 1 230 250 1;
	setAttr ".ed[498:663]"  250 249 1 231 251 1 251 250 1 232 252 1 252 251 1 233 253 1 253 252 1 234 254 1 254 253 1 235 255 1 255 254 1 236 256 1 256 255 1 237 257 1 257 256 1 238 258 1 258 257 1 239 259 1 259 258 1 240 260 1 260 259 1 241 260 1 242 262 1 262 261 1 261 241 1 243 263 1 263 262 1 244 264 1 264 263 1 245 265 1 265 264 1 246 266 1 266 265 1 247 267 1 267 266 1 248 268 1 268 267 1 249 269 1 269 268 1 250 270 1 270 269 1 251 271 1 271 270 1 252 272 1 272 271 1 253 273 1 273 272 1 254 274 1 274 273 1 255 275 1 275 274 1 256 276 1 276 275 1 257 277 1 277 276 1 258 278 1 278 277 1 259 279 1 279 278 1 260 280 1 280 279 1 261 280 1 262 282 1 282 281 1 281 261 1 263 283 1 283 282 1 264 284 1 284 283 1 265 285 1 285 284 1 266 286 1 286 285 1 267 287 1 287 286 1 268 288 1 288 287 1 269 289 1 289 288 1 270 290 1 290 289 1 271 291 1 291 290 1 272 292 1 292 291 1 273 293 1 293 292 1 274 294 1 294 293 1 275 295 1 295 294 1 276 296 1 296 295 1 277 297 1 297 296 1 278 298 1 298 297 1 279 299 1 299 298 1 280 300 1 300 299 1 281 300 1 282 302 1 302 301 1 301 281 1 283 303 1 303 302 1 284 304 1 304 303 1 285 305 1 305 304 1 286 306 1 306 305 1 287 307 1 307 306 1 288 308 1 308 307 1 289 309 1 309 308 1 290 310 1 310 309 1 291 311 1 311 310 1 292 312 1 312 311 1 293 313 1 313 312 1 294 314 1 314 313 1 295 315 1 315 314 1 296 316 1 316 315 1 297 317 1 317 316 1 298 318 1 318 317 1 299 319 1 319 318 1 300 320 1 320 319 1 301 320 1 302 322 1 322 321 1 321 301 1 303 323 1 323 322 1 304 324 1 324 323 1 305 325 1 325 324 1 306 326 1 326 325 1 307 327 1 327 326 1 308 328 1 328 327 1 309 329 1 329 328 1 310 330 1 330 329 1 311 331 1 331 330 1 312 332 1 332 331 1 313 333 1;
	setAttr ".ed[664:779]"  333 332 1 314 334 1 334 333 1 315 335 1 335 334 1 316 336 1 336 335 1 317 337 1 337 336 1 318 338 1 338 337 1 319 339 1 339 338 1 320 340 1 340 339 1 321 340 1 322 342 1 342 341 1 341 321 1 323 343 1 343 342 1 324 344 1 344 343 1 325 345 1 345 344 1 326 346 1 346 345 1 327 347 1 347 346 1 328 348 1 348 347 1 329 349 1 349 348 1 330 350 1 350 349 1 331 351 1 351 350 1 332 352 1 352 351 1 333 353 1 353 352 1 334 354 1 354 353 1 335 355 1 355 354 1 336 356 1 356 355 1 337 357 1 357 356 1 338 358 1 358 357 1 339 359 1 359 358 1 340 360 1 360 359 1 341 360 1 342 362 1 362 361 1 361 341 1 343 363 1 363 362 1 344 364 1 364 363 1 345 365 1 365 364 1 346 366 1 366 365 1 347 367 1 367 366 1 348 368 1 368 367 1 349 369 1 369 368 1 350 370 1 370 369 1 351 371 1 371 370 1 352 372 1 372 371 1 353 373 1 373 372 1 354 374 1 374 373 1 355 375 1 375 374 1 356 376 1 376 375 1 357 377 1 377 376 1 358 378 1 378 377 1 359 379 1 379 378 1 360 380 1 380 379 1 361 380 1 362 381 1 381 361 1 363 381 1 364 381 1 365 381 1 366 381 1 367 381 1 368 381 1 369 381 1 370 381 1 371 381 1 372 381 1 373 381 1 374 381 1 375 381 1 376 381 1 377 381 1 378 381 1 379 381 1 380 381 1;
	setAttr -s 400 -ch 1560 ".fc[0:249]" -type "polyFaces" 
		f 3 0 1 2 mu 0 3 0 1 2 
		f 3 3 4 -1 mu 0 3 3 4 1 
		f 3 5 6 -4 mu 0 3 5 6 4 
		f 3 7 8 -6 mu 0 3 7 8 6 
		f 3 9 10 -8 mu 0 3 9 10 8 
		f 3 11 12 -10 mu 0 3 11 12 10 
		f 3 13 14 -12 mu 0 3 13 14 12 
		f 3 15 16 -14 mu 0 3 15 16 14 
		f 3 17 18 -16 mu 0 3 17 18 16 
		f 3 19 20 -18 mu 0 3 19 20 18 
		f 3 21 22 -20 mu 0 3 21 22 20 
		f 3 23 24 -22 mu 0 3 23 24 22 
		f 3 25 26 -24 mu 0 3 25 26 24 
		f 3 27 28 -26 mu 0 3 27 28 26 
		f 3 29 30 -28 mu 0 3 29 30 28 
		f 3 31 32 -30 mu 0 3 31 32 30 
		f 3 33 34 -32 mu 0 3 33 34 32 
		f 3 35 36 -34 mu 0 3 35 36 34 
		f 3 37 38 -36 mu 0 3 37 38 36 
		f 3 -3 39 -38 mu 0 3 39 40 38 
		f 4 -2 40 41 42 mu 0 4 2 1 41 42 
		f 4 -5 43 44 -41 mu 0 4 1 4 43 41 
		f 4 -7 45 46 -44 mu 0 4 4 6 44 43 
		f 4 -9 47 48 -46 mu 0 4 6 8 45 44 
		f 4 -11 49 50 -48 mu 0 4 8 10 46 45 
		f 4 -13 51 52 -50 mu 0 4 10 12 47 46 
		f 4 -15 53 54 -52 mu 0 4 12 14 48 47 
		f 4 -17 55 56 -54 mu 0 4 14 16 49 48 
		f 4 -19 57 58 -56 mu 0 4 16 18 50 49 
		f 4 -21 59 60 -58 mu 0 4 18 20 51 50 
		f 4 -23 61 62 -60 mu 0 4 20 22 52 51 
		f 4 -25 63 64 -62 mu 0 4 22 24 53 52 
		f 4 -27 65 66 -64 mu 0 4 24 26 54 53 
		f 4 -29 67 68 -66 mu 0 4 26 28 55 54 
		f 4 -31 69 70 -68 mu 0 4 28 30 56 55 
		f 4 -33 71 72 -70 mu 0 4 30 32 57 56 
		f 4 -35 73 74 -72 mu 0 4 32 34 58 57 
		f 4 -37 75 76 -74 mu 0 4 34 36 59 58 
		f 4 -39 77 78 -76 mu 0 4 36 38 60 59 
		f 4 -40 -43 79 -78 mu 0 4 38 40 61 60 
		f 4 -42 80 81 82 mu 0 4 42 41 62 63 
		f 4 -45 83 84 -81 mu 0 4 41 43 64 62 
		f 4 -47 85 86 -84 mu 0 4 43 44 65 64 
		f 4 -49 87 88 -86 mu 0 4 44 45 66 65 
		f 4 -51 89 90 -88 mu 0 4 45 46 67 66 
		f 4 -53 91 92 -90 mu 0 4 46 47 68 67 
		f 4 -55 93 94 -92 mu 0 4 47 48 69 68 
		f 4 -57 95 96 -94 mu 0 4 48 49 70 69 
		f 4 -59 97 98 -96 mu 0 4 49 50 71 70 
		f 4 -61 99 100 -98 mu 0 4 50 51 72 71 
		f 4 -63 101 102 -100 mu 0 4 51 52 73 72 
		f 4 -65 103 104 -102 mu 0 4 52 53 74 73 
		f 4 -67 105 106 -104 mu 0 4 53 54 75 74 
		f 4 -69 107 108 -106 mu 0 4 54 55 76 75 
		f 4 -71 109 110 -108 mu 0 4 55 56 77 76 
		f 4 -73 111 112 -110 mu 0 4 56 57 78 77 
		f 4 -75 113 114 -112 mu 0 4 57 58 79 78 
		f 4 -77 115 116 -114 mu 0 4 58 59 80 79 
		f 4 -79 117 118 -116 mu 0 4 59 60 81 80 
		f 4 -80 -83 119 -118 mu 0 4 60 61 82 81 
		f 4 -82 120 121 122 mu 0 4 63 62 83 84 
		f 4 -85 123 124 -121 mu 0 4 62 64 85 83 
		f 4 -87 125 126 -124 mu 0 4 64 65 86 85 
		f 4 -89 127 128 -126 mu 0 4 65 66 87 86 
		f 4 -91 129 130 -128 mu 0 4 66 67 88 87 
		f 4 -93 131 132 -130 mu 0 4 67 68 89 88 
		f 4 -95 133 134 -132 mu 0 4 68 69 90 89 
		f 4 -97 135 136 -134 mu 0 4 69 70 91 90 
		f 4 -99 137 138 -136 mu 0 4 70 71 92 91 
		f 4 -101 139 140 -138 mu 0 4 71 72 93 92 
		f 4 -103 141 142 -140 mu 0 4 72 73 94 93 
		f 4 -105 143 144 -142 mu 0 4 73 74 95 94 
		f 4 -107 145 146 -144 mu 0 4 74 75 96 95 
		f 4 -109 147 148 -146 mu 0 4 75 76 97 96 
		f 4 -111 149 150 -148 mu 0 4 76 77 98 97 
		f 4 -113 151 152 -150 mu 0 4 77 78 99 98 
		f 4 -115 153 154 -152 mu 0 4 78 79 100 99 
		f 4 -117 155 156 -154 mu 0 4 79 80 101 100 
		f 4 -119 157 158 -156 mu 0 4 80 81 102 101 
		f 4 -120 -123 159 -158 mu 0 4 81 82 103 102 
		f 4 -122 160 161 162 mu 0 4 84 83 104 105 
		f 4 -125 163 164 -161 mu 0 4 83 85 106 104 
		f 4 -127 165 166 -164 mu 0 4 85 86 107 106 
		f 4 -129 167 168 -166 mu 0 4 86 87 108 107 
		f 4 -131 169 170 -168 mu 0 4 87 88 109 108 
		f 4 -133 171 172 -170 mu 0 4 88 89 110 109 
		f 4 -135 173 174 -172 mu 0 4 89 90 111 110 
		f 4 -137 175 176 -174 mu 0 4 90 91 112 111 
		f 4 -139 177 178 -176 mu 0 4 91 92 113 112 
		f 4 -141 179 180 -178 mu 0 4 92 93 114 113 
		f 4 -143 181 182 -180 mu 0 4 93 94 115 114 
		f 4 -145 183 184 -182 mu 0 4 94 95 116 115 
		f 4 -147 185 186 -184 mu 0 4 95 96 117 116 
		f 4 -149 187 188 -186 mu 0 4 96 97 118 117 
		f 4 -151 189 190 -188 mu 0 4 97 98 119 118 
		f 4 -153 191 192 -190 mu 0 4 98 99 120 119 
		f 4 -155 193 194 -192 mu 0 4 99 100 121 120 
		f 4 -157 195 196 -194 mu 0 4 100 101 122 121 
		f 4 -159 197 198 -196 mu 0 4 101 102 123 122 
		f 4 -160 -163 199 -198 mu 0 4 102 103 124 123 
		f 4 -162 200 201 202 mu 0 4 105 104 125 126 
		f 4 -165 203 204 -201 mu 0 4 104 106 127 125 
		f 4 -167 205 206 -204 mu 0 4 106 107 128 127 
		f 4 -169 207 208 -206 mu 0 4 107 108 129 128 
		f 4 -171 209 210 -208 mu 0 4 108 109 130 129 
		f 4 -173 211 212 -210 mu 0 4 109 110 131 130 
		f 4 -175 213 214 -212 mu 0 4 110 111 132 131 
		f 4 -177 215 216 -214 mu 0 4 111 112 133 132 
		f 4 -179 217 218 -216 mu 0 4 112 113 134 133 
		f 4 -181 219 220 -218 mu 0 4 113 114 135 134 
		f 4 -183 221 222 -220 mu 0 4 114 115 136 135 
		f 4 -185 223 224 -222 mu 0 4 115 116 137 136 
		f 4 -187 225 226 -224 mu 0 4 116 117 138 137 
		f 4 -189 227 228 -226 mu 0 4 117 118 139 138 
		f 4 -191 229 230 -228 mu 0 4 118 119 140 139 
		f 4 -193 231 232 -230 mu 0 4 119 120 141 140 
		f 4 -195 233 234 -232 mu 0 4 120 121 142 141 
		f 4 -197 235 236 -234 mu 0 4 121 122 143 142 
		f 4 -199 237 238 -236 mu 0 4 122 123 144 143 
		f 4 -200 -203 239 -238 mu 0 4 123 124 145 144 
		f 4 -202 240 241 242 mu 0 4 126 125 146 147 
		f 4 -205 243 244 -241 mu 0 4 125 127 148 146 
		f 4 -207 245 246 -244 mu 0 4 127 128 149 148 
		f 4 -209 247 248 -246 mu 0 4 128 129 150 149 
		f 4 -211 249 250 -248 mu 0 4 129 130 151 150 
		f 4 -213 251 252 -250 mu 0 4 130 131 152 151 
		f 4 -215 253 254 -252 mu 0 4 131 132 153 152 
		f 4 -217 255 256 -254 mu 0 4 132 133 154 153 
		f 4 -219 257 258 -256 mu 0 4 133 134 155 154 
		f 4 -221 259 260 -258 mu 0 4 134 135 156 155 
		f 4 -223 261 262 -260 mu 0 4 135 136 157 156 
		f 4 -225 263 264 -262 mu 0 4 136 137 158 157 
		f 4 -227 265 266 -264 mu 0 4 137 138 159 158 
		f 4 -229 267 268 -266 mu 0 4 138 139 160 159 
		f 4 -231 269 270 -268 mu 0 4 139 140 161 160 
		f 4 -233 271 272 -270 mu 0 4 140 141 162 161 
		f 4 -235 273 274 -272 mu 0 4 141 142 163 162 
		f 4 -237 275 276 -274 mu 0 4 142 143 164 163 
		f 4 -239 277 278 -276 mu 0 4 143 144 165 164 
		f 4 -240 -243 279 -278 mu 0 4 144 145 166 165 
		f 4 -242 280 281 282 mu 0 4 147 146 167 168 
		f 4 -245 283 284 -281 mu 0 4 146 148 169 167 
		f 4 -247 285 286 -284 mu 0 4 148 149 170 169 
		f 4 -249 287 288 -286 mu 0 4 149 150 171 170 
		f 4 -251 289 290 -288 mu 0 4 150 151 172 171 
		f 4 -253 291 292 -290 mu 0 4 151 152 173 172 
		f 4 -255 293 294 -292 mu 0 4 152 153 174 173 
		f 4 -257 295 296 -294 mu 0 4 153 154 175 174 
		f 4 -259 297 298 -296 mu 0 4 154 155 176 175 
		f 4 -261 299 300 -298 mu 0 4 155 156 177 176 
		f 4 -263 301 302 -300 mu 0 4 156 157 178 177 
		f 4 -265 303 304 -302 mu 0 4 157 158 179 178 
		f 4 -267 305 306 -304 mu 0 4 158 159 180 179 
		f 4 -269 307 308 -306 mu 0 4 159 160 181 180 
		f 4 -271 309 310 -308 mu 0 4 160 161 182 181 
		f 4 -273 311 312 -310 mu 0 4 161 162 183 182 
		f 4 -275 313 314 -312 mu 0 4 162 163 184 183 
		f 4 -277 315 316 -314 mu 0 4 163 164 185 184 
		f 4 -279 317 318 -316 mu 0 4 164 165 186 185 
		f 4 -280 -283 319 -318 mu 0 4 165 166 187 186 
		f 4 -282 320 321 322 mu 0 4 168 167 188 189 
		f 4 -285 323 324 -321 mu 0 4 167 169 190 188 
		f 4 -287 325 326 -324 mu 0 4 169 170 191 190 
		f 4 -289 327 328 -326 mu 0 4 170 171 192 191 
		f 4 -291 329 330 -328 mu 0 4 171 172 193 192 
		f 4 -293 331 332 -330 mu 0 4 172 173 194 193 
		f 4 -295 333 334 -332 mu 0 4 173 174 195 194 
		f 4 -297 335 336 -334 mu 0 4 174 175 196 195 
		f 4 -299 337 338 -336 mu 0 4 175 176 197 196 
		f 4 -301 339 340 -338 mu 0 4 176 177 198 197 
		f 4 -303 341 342 -340 mu 0 4 177 178 199 198 
		f 4 -305 343 344 -342 mu 0 4 178 179 200 199 
		f 4 -307 345 346 -344 mu 0 4 179 180 201 200 
		f 4 -309 347 348 -346 mu 0 4 180 181 202 201 
		f 4 -311 349 350 -348 mu 0 4 181 182 203 202 
		f 4 -313 351 352 -350 mu 0 4 182 183 204 203 
		f 4 -315 353 354 -352 mu 0 4 183 184 205 204 
		f 4 -317 355 356 -354 mu 0 4 184 185 206 205 
		f 4 -319 357 358 -356 mu 0 4 185 186 207 206 
		f 4 -320 -323 359 -358 mu 0 4 186 187 208 207 
		f 4 -322 360 361 362 mu 0 4 189 188 209 210 
		f 4 -325 363 364 -361 mu 0 4 188 190 211 209 
		f 4 -327 365 366 -364 mu 0 4 190 191 212 211 
		f 4 -329 367 368 -366 mu 0 4 191 192 213 212 
		f 4 -331 369 370 -368 mu 0 4 192 193 214 213 
		f 4 -333 371 372 -370 mu 0 4 193 194 215 214 
		f 4 -335 373 374 -372 mu 0 4 194 195 216 215 
		f 4 -337 375 376 -374 mu 0 4 195 196 217 216 
		f 4 -339 377 378 -376 mu 0 4 196 197 218 217 
		f 4 -341 379 380 -378 mu 0 4 197 198 219 218 
		f 4 -343 381 382 -380 mu 0 4 198 199 220 219 
		f 4 -345 383 384 -382 mu 0 4 199 200 221 220 
		f 4 -347 385 386 -384 mu 0 4 200 201 222 221 
		f 4 -349 387 388 -386 mu 0 4 201 202 223 222 
		f 4 -351 389 390 -388 mu 0 4 202 203 224 223 
		f 4 -353 391 392 -390 mu 0 4 203 204 225 224 
		f 4 -355 393 394 -392 mu 0 4 204 205 226 225 
		f 4 -357 395 396 -394 mu 0 4 205 206 227 226 
		f 4 -359 397 398 -396 mu 0 4 206 207 228 227 
		f 4 -360 -363 399 -398 mu 0 4 207 208 229 228 
		f 4 -362 400 401 402 mu 0 4 210 209 230 231 
		f 4 -365 403 404 -401 mu 0 4 209 211 232 230 
		f 4 -367 405 406 -404 mu 0 4 211 212 233 232 
		f 4 -369 407 408 -406 mu 0 4 212 213 234 233 
		f 4 -371 409 410 -408 mu 0 4 213 214 235 234 
		f 4 -373 411 412 -410 mu 0 4 214 215 236 235 
		f 4 -375 413 414 -412 mu 0 4 215 216 237 236 
		f 4 -377 415 416 -414 mu 0 4 216 217 238 237 
		f 4 -379 417 418 -416 mu 0 4 217 218 239 238 
		f 4 -381 419 420 -418 mu 0 4 218 219 240 239 
		f 4 -383 421 422 -420 mu 0 4 219 220 241 240 
		f 4 -385 423 424 -422 mu 0 4 220 221 242 241 
		f 4 -387 425 426 -424 mu 0 4 221 222 243 242 
		f 4 -389 427 428 -426 mu 0 4 222 223 244 243 
		f 4 -391 429 430 -428 mu 0 4 223 224 245 244 
		f 4 -393 431 432 -430 mu 0 4 224 225 246 245 
		f 4 -395 433 434 -432 mu 0 4 225 226 247 246 
		f 4 -397 435 436 -434 mu 0 4 226 227 248 247 
		f 4 -399 437 438 -436 mu 0 4 227 228 249 248 
		f 4 -400 -403 439 -438 mu 0 4 228 229 250 249 
		f 4 -402 440 441 442 mu 0 4 231 230 251 252 
		f 4 -405 443 444 -441 mu 0 4 230 232 253 251 
		f 4 -407 445 446 -444 mu 0 4 232 233 254 253 
		f 4 -409 447 448 -446 mu 0 4 233 234 255 254 
		f 4 -411 449 450 -448 mu 0 4 234 235 256 255 
		f 4 -413 451 452 -450 mu 0 4 235 236 257 256 
		f 4 -415 453 454 -452 mu 0 4 236 237 258 257 
		f 4 -417 455 456 -454 mu 0 4 237 238 259 258 
		f 4 -419 457 458 -456 mu 0 4 238 239 260 259 
		f 4 -421 459 460 -458 mu 0 4 239 240 261 260 
		f 4 -423 461 462 -460 mu 0 4 240 241 262 261 
		f 4 -425 463 464 -462 mu 0 4 241 242 263 262 
		f 4 -427 465 466 -464 mu 0 4 242 243 264 263 
		f 4 -429 467 468 -466 mu 0 4 243 244 265 264 
		f 4 -431 469 470 -468 mu 0 4 244 245 266 265 
		f 4 -433 471 472 -470 mu 0 4 245 246 267 266 
		f 4 -435 473 474 -472 mu 0 4 246 247 268 267 
		f 4 -437 475 476 -474 mu 0 4 247 248 269 268 
		f 4 -439 477 478 -476 mu 0 4 248 249 270 269 
		f 4 -440 -443 479 -478 mu 0 4 249 250 271 270 
		f 4 -442 480 481 482 mu 0 4 252 251 272 273 
		f 4 -445 483 484 -481 mu 0 4 251 253 274 272 
		f 4 -447 485 486 -484 mu 0 4 253 254 275 274 
		f 4 -449 487 488 -486 mu 0 4 254 255 276 275 
		f 4 -451 489 490 -488 mu 0 4 255 256 277 276 
		f 4 -453 491 492 -490 mu 0 4 256 257 278 277 
		f 4 -455 493 494 -492 mu 0 4 257 258 279 278 
		f 4 -457 495 496 -494 mu 0 4 258 259 280 279 
		f 4 -459 497 498 -496 mu 0 4 259 260 281 280 
		f 4 -461 499 500 -498 mu 0 4 260 261 282 281 ;
	setAttr ".fc[250:399]" -type "polyFaces" 
		f 4 -463 501 502 -500 mu 0 4 261 262 283 282 
		f 4 -465 503 504 -502 mu 0 4 262 263 284 283 
		f 4 -467 505 506 -504 mu 0 4 263 264 285 284 
		f 4 -469 507 508 -506 mu 0 4 264 265 286 285 
		f 4 -471 509 510 -508 mu 0 4 265 266 287 286 
		f 4 -473 511 512 -510 mu 0 4 266 267 288 287 
		f 4 -475 513 514 -512 mu 0 4 267 268 289 288 
		f 4 -477 515 516 -514 mu 0 4 268 269 290 289 
		f 4 -479 517 518 -516 mu 0 4 269 270 291 290 
		f 4 -480 -483 519 -518 mu 0 4 270 271 292 291 
		f 4 -482 520 521 522 mu 0 4 273 272 293 294 
		f 4 -485 523 524 -521 mu 0 4 272 274 295 293 
		f 4 -487 525 526 -524 mu 0 4 274 275 296 295 
		f 4 -489 527 528 -526 mu 0 4 275 276 297 296 
		f 4 -491 529 530 -528 mu 0 4 276 277 298 297 
		f 4 -493 531 532 -530 mu 0 4 277 278 299 298 
		f 4 -495 533 534 -532 mu 0 4 278 279 300 299 
		f 4 -497 535 536 -534 mu 0 4 279 280 301 300 
		f 4 -499 537 538 -536 mu 0 4 280 281 302 301 
		f 4 -501 539 540 -538 mu 0 4 281 282 303 302 
		f 4 -503 541 542 -540 mu 0 4 282 283 304 303 
		f 4 -505 543 544 -542 mu 0 4 283 284 305 304 
		f 4 -507 545 546 -544 mu 0 4 284 285 306 305 
		f 4 -509 547 548 -546 mu 0 4 285 286 307 306 
		f 4 -511 549 550 -548 mu 0 4 286 287 308 307 
		f 4 -513 551 552 -550 mu 0 4 287 288 309 308 
		f 4 -515 553 554 -552 mu 0 4 288 289 310 309 
		f 4 -517 555 556 -554 mu 0 4 289 290 311 310 
		f 4 -519 557 558 -556 mu 0 4 290 291 312 311 
		f 4 -520 -523 559 -558 mu 0 4 291 292 313 312 
		f 4 -522 560 561 562 mu 0 4 294 293 314 315 
		f 4 -525 563 564 -561 mu 0 4 293 295 316 314 
		f 4 -527 565 566 -564 mu 0 4 295 296 317 316 
		f 4 -529 567 568 -566 mu 0 4 296 297 318 317 
		f 4 -531 569 570 -568 mu 0 4 297 298 319 318 
		f 4 -533 571 572 -570 mu 0 4 298 299 320 319 
		f 4 -535 573 574 -572 mu 0 4 299 300 321 320 
		f 4 -537 575 576 -574 mu 0 4 300 301 322 321 
		f 4 -539 577 578 -576 mu 0 4 301 302 323 322 
		f 4 -541 579 580 -578 mu 0 4 302 303 324 323 
		f 4 -543 581 582 -580 mu 0 4 303 304 325 324 
		f 4 -545 583 584 -582 mu 0 4 304 305 326 325 
		f 4 -547 585 586 -584 mu 0 4 305 306 327 326 
		f 4 -549 587 588 -586 mu 0 4 306 307 328 327 
		f 4 -551 589 590 -588 mu 0 4 307 308 329 328 
		f 4 -553 591 592 -590 mu 0 4 308 309 330 329 
		f 4 -555 593 594 -592 mu 0 4 309 310 331 330 
		f 4 -557 595 596 -594 mu 0 4 310 311 332 331 
		f 4 -559 597 598 -596 mu 0 4 311 312 333 332 
		f 4 -560 -563 599 -598 mu 0 4 312 313 334 333 
		f 4 -562 600 601 602 mu 0 4 315 314 335 336 
		f 4 -565 603 604 -601 mu 0 4 314 316 337 335 
		f 4 -567 605 606 -604 mu 0 4 316 317 338 337 
		f 4 -569 607 608 -606 mu 0 4 317 318 339 338 
		f 4 -571 609 610 -608 mu 0 4 318 319 340 339 
		f 4 -573 611 612 -610 mu 0 4 319 320 341 340 
		f 4 -575 613 614 -612 mu 0 4 320 321 342 341 
		f 4 -577 615 616 -614 mu 0 4 321 322 343 342 
		f 4 -579 617 618 -616 mu 0 4 322 323 344 343 
		f 4 -581 619 620 -618 mu 0 4 323 324 345 344 
		f 4 -583 621 622 -620 mu 0 4 324 325 346 345 
		f 4 -585 623 624 -622 mu 0 4 325 326 347 346 
		f 4 -587 625 626 -624 mu 0 4 326 327 348 347 
		f 4 -589 627 628 -626 mu 0 4 327 328 349 348 
		f 4 -591 629 630 -628 mu 0 4 328 329 350 349 
		f 4 -593 631 632 -630 mu 0 4 329 330 351 350 
		f 4 -595 633 634 -632 mu 0 4 330 331 352 351 
		f 4 -597 635 636 -634 mu 0 4 331 332 353 352 
		f 4 -599 637 638 -636 mu 0 4 332 333 354 353 
		f 4 -600 -603 639 -638 mu 0 4 333 334 355 354 
		f 4 -602 640 641 642 mu 0 4 336 335 356 357 
		f 4 -605 643 644 -641 mu 0 4 335 337 358 356 
		f 4 -607 645 646 -644 mu 0 4 337 338 359 358 
		f 4 -609 647 648 -646 mu 0 4 338 339 360 359 
		f 4 -611 649 650 -648 mu 0 4 339 340 361 360 
		f 4 -613 651 652 -650 mu 0 4 340 341 362 361 
		f 4 -615 653 654 -652 mu 0 4 341 342 363 362 
		f 4 -617 655 656 -654 mu 0 4 342 343 364 363 
		f 4 -619 657 658 -656 mu 0 4 343 344 365 364 
		f 4 -621 659 660 -658 mu 0 4 344 345 366 365 
		f 4 -623 661 662 -660 mu 0 4 345 346 367 366 
		f 4 -625 663 664 -662 mu 0 4 346 347 368 367 
		f 4 -627 665 666 -664 mu 0 4 347 348 369 368 
		f 4 -629 667 668 -666 mu 0 4 348 349 370 369 
		f 4 -631 669 670 -668 mu 0 4 349 350 371 370 
		f 4 -633 671 672 -670 mu 0 4 350 351 372 371 
		f 4 -635 673 674 -672 mu 0 4 351 352 373 372 
		f 4 -637 675 676 -674 mu 0 4 352 353 374 373 
		f 4 -639 677 678 -676 mu 0 4 353 354 375 374 
		f 4 -640 -643 679 -678 mu 0 4 354 355 376 375 
		f 4 -642 680 681 682 mu 0 4 357 356 377 378 
		f 4 -645 683 684 -681 mu 0 4 356 358 379 377 
		f 4 -647 685 686 -684 mu 0 4 358 359 380 379 
		f 4 -649 687 688 -686 mu 0 4 359 360 381 380 
		f 4 -651 689 690 -688 mu 0 4 360 361 382 381 
		f 4 -653 691 692 -690 mu 0 4 361 362 383 382 
		f 4 -655 693 694 -692 mu 0 4 362 363 384 383 
		f 4 -657 695 696 -694 mu 0 4 363 364 385 384 
		f 4 -659 697 698 -696 mu 0 4 364 365 386 385 
		f 4 -661 699 700 -698 mu 0 4 365 366 387 386 
		f 4 -663 701 702 -700 mu 0 4 366 367 388 387 
		f 4 -665 703 704 -702 mu 0 4 367 368 389 388 
		f 4 -667 705 706 -704 mu 0 4 368 369 390 389 
		f 4 -669 707 708 -706 mu 0 4 369 370 391 390 
		f 4 -671 709 710 -708 mu 0 4 370 371 392 391 
		f 4 -673 711 712 -710 mu 0 4 371 372 393 392 
		f 4 -675 713 714 -712 mu 0 4 372 373 394 393 
		f 4 -677 715 716 -714 mu 0 4 373 374 395 394 
		f 4 -679 717 718 -716 mu 0 4 374 375 396 395 
		f 4 -680 -683 719 -718 mu 0 4 375 376 397 396 
		f 4 -682 720 721 722 mu 0 4 378 377 398 399 
		f 4 -685 723 724 -721 mu 0 4 377 379 400 398 
		f 4 -687 725 726 -724 mu 0 4 379 380 401 400 
		f 4 -689 727 728 -726 mu 0 4 380 381 402 401 
		f 4 -691 729 730 -728 mu 0 4 381 382 403 402 
		f 4 -693 731 732 -730 mu 0 4 382 383 404 403 
		f 4 -695 733 734 -732 mu 0 4 383 384 405 404 
		f 4 -697 735 736 -734 mu 0 4 384 385 406 405 
		f 4 -699 737 738 -736 mu 0 4 385 386 407 406 
		f 4 -701 739 740 -738 mu 0 4 386 387 408 407 
		f 4 -703 741 742 -740 mu 0 4 387 388 409 408 
		f 4 -705 743 744 -742 mu 0 4 388 389 410 409 
		f 4 -707 745 746 -744 mu 0 4 389 390 411 410 
		f 4 -709 747 748 -746 mu 0 4 390 391 412 411 
		f 4 -711 749 750 -748 mu 0 4 391 392 413 412 
		f 4 -713 751 752 -750 mu 0 4 392 393 414 413 
		f 4 -715 753 754 -752 mu 0 4 393 394 415 414 
		f 4 -717 755 756 -754 mu 0 4 394 395 416 415 
		f 4 -719 757 758 -756 mu 0 4 395 396 417 416 
		f 4 -720 -723 759 -758 mu 0 4 396 397 418 417 
		f 3 -722 760 761 mu 0 3 399 398 419 
		f 3 -725 762 -761 mu 0 3 398 400 420 
		f 3 -727 763 -763 mu 0 3 400 401 421 
		f 3 -729 764 -764 mu 0 3 401 402 422 
		f 3 -731 765 -765 mu 0 3 402 403 423 
		f 3 -733 766 -766 mu 0 3 403 404 424 
		f 3 -735 767 -767 mu 0 3 404 405 425 
		f 3 -737 768 -768 mu 0 3 405 406 426 
		f 3 -739 769 -769 mu 0 3 406 407 427 
		f 3 -741 770 -770 mu 0 3 407 408 428 
		f 3 -743 771 -771 mu 0 3 408 409 429 
		f 3 -745 772 -772 mu 0 3 409 410 430 
		f 3 -747 773 -773 mu 0 3 410 411 431 
		f 3 -749 774 -774 mu 0 3 411 412 432 
		f 3 -751 775 -775 mu 0 3 412 413 433 
		f 3 -753 776 -776 mu 0 3 413 414 434 
		f 3 -755 777 -777 mu 0 3 414 415 435 
		f 3 -757 778 -778 mu 0 3 415 416 436 
		f 3 -759 779 -779 mu 0 3 416 417 437 
		f 3 -760 -762 -780 mu 0 3 417 418 438 ;
createNode transform -n "keyLight";
	setAttr ".r" -type "double3" -45 35 0 ;
createNode directionalLight -n "keyLightShape" -p "keyLight";
	setAttr -k off ".v";
createNode transform -n "fillLight";
createNode ambientLight -n "fillLightShape" -p "fillLight";
	setAttr -k off ".v";
	setAttr ".in" 0.25;
createNode lightLinker -s -n "lightLinker1";
createNode displayLayerManager -n "layerManager";
createNode displayLayer -n "defaultLayer";
	setAttr ".ufem" -type "stringArray" 0  ;
createNode renderLayerManager -n "renderLayerManager";
createNode renderLayer -n "defaultRenderLayer";
	setAttr ".g" yes;
createNode lambert -n "floorMat";
	setAttr ".c" -type "float3" 0.45 0.45 0.45 ;
createNode shadingEngine -n "floorMatSG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody1";
	setAttr ".object" -type "string" "floor";
	setAttr ".active" no;
createNode rigidBody -n "rigidBody2";
	setAttr ".object" -type "string" "brick_0_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -780 32.25 0 1;
createNode lambert -n "brickMat_0_0";
	setAttr ".c" -type "float3" 0.591813242373306 0.285906621186653 0.22 ;
createNode shadingEngine -n "brickMat_0_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody3";
	setAttr ".object" -type "string" "brick_0_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -660 32.25 0 1;
createNode lambert -n "brickMat_0_1";
	setAttr ".c" -type "float3" 0.5641358678279202 0.2720679339139601 0.22 ;
createNode shadingEngine -n "brickMat_0_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody4";
	setAttr ".object" -type "string" "brick_0_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -540 32.25 0 1;
createNode lambert -n "brickMat_0_2";
	setAttr ".c" -type "float3" 0.6441495156863766 0.3120747578431883 0.22 ;
createNode shadingEngine -n "brickMat_0_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody5";
	setAttr ".object" -type "string" "brick_0_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -420 32.25 0 1;
createNode lambert -n "brickMat_0_3";
	setAttr ".c" -type "float3" 0.5515898058668068 0.2657949029334034 0.22 ;
createNode shadingEngine -n "brickMat_0_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody6";
	setAttr ".object" -type "string" "brick_0_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -300 32.25 0 1;
createNode lambert -n "brickMat_0_4";
	setAttr ".c" -type "float3" 0.6257411206890703 0.30287056034453513 0.22 ;
createNode shadingEngine -n "brickMat_0_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody7";
	setAttr ".object" -type "string" "brick_0_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -180 32.25 0 1;
createNode lambert -n "brickMat_0_5";
	setAttr ".c" -type "float3" 0.5985102267060137 0.28925511335300685 0.22 ;
createNode shadingEngine -n "brickMat_0_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody8";
	setAttr ".object" -type "string" "brick_0_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -60 32.25 0 1;
createNode lambert -n "brickMat_0_6";
	setAttr ".c" -type "float3" 0.549279827963953 0.2646399139819765 0.22 ;
createNode shadingEngine -n "brickMat_0_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody9";
	setAttr ".object" -type "string" "brick_0_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 60 32.25 0 1;
createNode lambert -n "brickMat_0_7";
	setAttr ".c" -type "float3" 0.6211897173103073 0.3005948586551536 0.22 ;
createNode shadingEngine -n "brickMat_0_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody10";
	setAttr ".object" -type "string" "brick_0_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 180 32.25 0 1;
createNode lambert -n "brickMat_0_8";
	setAttr ".c" -type "float3" 0.5459993053507176 0.2629996526753588 0.22 ;
createNode shadingEngine -n "brickMat_0_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody11";
	setAttr ".object" -type "string" "brick_0_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 300 32.25 0 1;
createNode lambert -n "brickMat_0_9";
	setAttr ".c" -type "float3" 0.6093833093859817 0.29469165469299086 0.22 ;
createNode shadingEngine -n "brickMat_0_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody12";
	setAttr ".object" -type "string" "brick_0_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 420 32.25 0 1;
createNode lambert -n "brickMat_0_10";
	setAttr ".c" -type "float3" 0.5511768677719391 0.26558843388596953 0.22 ;
createNode shadingEngine -n "brickMat_0_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody13";
	setAttr ".object" -type "string" "brick_0_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 540 32.25 0 1;
createNode lambert -n "brickMat_0_11";
	setAttr ".c" -type "float3" 0.5545140821350184 0.2672570410675092 0.22 ;
createNode shadingEngine -n "brickMat_0_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody14";
	setAttr ".object" -type "string" "brick_0_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 660 32.25 0 1;
createNode lambert -n "brickMat_0_12";
	setAttr ".c" -type "float3" 0.6079230702628022 0.2939615351314011 0.22 ;
createNode shadingEngine -n "brickMat_0_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody15";
	setAttr ".object" -type "string" "brick_0_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 780 32.25 0 1;
createNode lambert -n "brickMat_0_13";
	setAttr ".c" -type "float3" 0.6722963399475261 0.32614816997376306 0.22 ;
createNode shadingEngine -n "brickMat_0_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody16";
	setAttr ".object" -type "string" "brick_1_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -810 94.5 0 1;
createNode lambert -n "brickMat_1_0";
	setAttr ".c" -type "float3" 0.5598083137839432 0.2699041568919716 0.22 ;
createNode shadingEngine -n "brickMat_1_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody17";
	setAttr ".object" -type "string" "brick_1_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -720 94.5 0 1;
createNode lambert -n "brickMat_1_1";
	setAttr ".c" -type "float3" 0.5757182343371223 0.27785911716856115 0.22 ;
createNode shadingEngine -n "brickMat_1_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody18";
	setAttr ".object" -type "string" "brick_1_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -600 94.5 0 1;
createNode lambert -n "brickMat_1_2";
	setAttr ".c" -type "float3" 0.6403893155848943 0.31019465779244715 0.22 ;
createNode shadingEngine -n "brickMat_1_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody19";
	setAttr ".object" -type "string" "brick_1_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -480 94.5 0 1;
createNode lambert -n "brickMat_1_3";
	setAttr ".c" -type "float3" 0.6916334307931209 0.33581671539656044 0.22 ;
createNode shadingEngine -n "brickMat_1_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody20";
	setAttr ".object" -type "string" "brick_1_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -360 94.5 0 1;
createNode lambert -n "brickMat_1_4";
	setAttr ".c" -type "float3" 0.6323364717787998 0.3061682358893999 0.22 ;
createNode shadingEngine -n "brickMat_1_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody21";
	setAttr ".object" -type "string" "brick_1_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -240 94.5 0 1;
createNode lambert -n "brickMat_1_5";
	setAttr ".c" -type "float3" 0.6034688759441248 0.2917344379720624 0.22 ;
createNode shadingEngine -n "brickMat_1_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody22";
	setAttr ".object" -type "string" "brick_1_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -120 94.5 0 1;
createNode lambert -n "brickMat_1_6";
	setAttr ".c" -type "float3" 0.6962008168948672 0.3381004084474336 0.22 ;
createNode shadingEngine -n "brickMat_1_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody23";
	setAttr ".object" -type "string" "brick_1_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 0 94.5 0 1;
createNode lambert -n "brickMat_1_7";
	setAttr ".c" -type "float3" 0.547453228898841 0.26372661444942047 0.22 ;
createNode shadingEngine -n "brickMat_1_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody24";
	setAttr ".object" -type "string" "brick_1_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 120 94.5 0 1;
createNode lambert -n "brickMat_1_8";
	setAttr ".c" -type "float3" 0.6773549534477887 0.32867747672389436 0.22 ;
createNode shadingEngine -n "brickMat_1_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody25";
	setAttr ".object" -type "string" "brick_1_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 240 94.5 0 1;
createNode lambert -n "brickMat_1_9";
	setAttr ".c" -type "float3" 0.5863374858130682 0.2831687429065341 0.22 ;
createNode shadingEngine -n "brickMat_1_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody26";
	setAttr ".object" -type "string" "brick_1_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 360 94.5 0 1;
createNode lambert -n "brickMat_1_10";
	setAttr ".c" -type "float3" 0.56308081333719 0.27154040666859497 0.22 ;
createNode shadingEngine -n "brickMat_1_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody27";
	setAttr ".object" -type "string" "brick_1_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 480 94.5 0 1;
createNode lambert -n "brickMat_1_11";
	setAttr ".c" -type "float3" 0.5588467580925389 0.26942337904626945 0.22 ;
createNode shadingEngine -n "brickMat_1_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody28";
	setAttr ".object" -type "string" "brick_1_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 600 94.5 0 1;
createNode lambert -n "brickMat_1_12";
	setAttr ".c" -type "float3" 0.5893570918563095 0.2846785459281547 0.22 ;
createNode shadingEngine -n "brickMat_1_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody29";
	setAttr ".object" -type "string" "brick_1_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 720 94.5 0 1;
createNode lambert -n "brickMat_1_13";
	setAttr ".c" -type "float3" 0.670580217459205 0.3252901087296025 0.22 ;
createNode shadingEngine -n "brickMat_1_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody30";
	setAttr ".object" -type "string" "brick_1_14";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 810 94.5 0 1;
createNode lambert -n "brickMat_1_14";
	setAttr ".c" -type "float3" 0.56891622078783 0.274458110393915 0.22 ;
createNode shadingEngine -n "brickMat_1_14SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody31";
	setAttr ".object" -type "string" "brick_2_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -780 156.75 0 1;
createNode lambert -n "brickMat_2_0";
	setAttr ".c" -type "float3" 0.6330560261859945 0.30652801309299726 0.22 ;
createNode shadingEngine -n "brickMat_2_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody32";
	setAttr ".object" -type "string" "brick_2_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -660 156.75 0 1;
createNode lambert -n "brickMat_2_1";
	setAttr ".c" -type "float3" 0.6422261550281895 0.3111130775140947 0.22 ;
createNode shadingEngine -n "brickMat_2_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody33";
	setAttr ".object" -type "string" "brick_2_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -540 156.75 0 1;
createNode lambert -n "brickMat_2_2";
	setAttr ".c" -type "float3" 0.599583606836117 0.2897918034180585 0.22 ;
createNode shadingEngine -n "brickMat_2_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody34";
	setAttr ".object" -type "string" "brick_2_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -420 156.75 0 1;
createNode lambert -n "brickMat_2_3";
	setAttr ".c" -type "float3" 0.6276391145135293 0.3038195572567646 0.22 ;
createNode shadingEngine -n "brickMat_2_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody35";
	setAttr ".object" -type "string" "brick_2_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -300 156.75 0 1;
createNode lambert -n "brickMat_2_4";
	setAttr ".c" -type "float3" 0.5500462359957317 0.2650231179978658 0.22 ;
createNode shadingEngine -n "brickMat_2_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody36";
	setAttr ".object" -type "string" "brick_2_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -180 156.75 0 1;
createNode lambert -n "brickMat_2_5";
	setAttr ".c" -type "float3" 0.5495361871945972 0.2647680935972986 0.22 ;
createNode shadingEngine -n "brickMat_2_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody37";
	setAttr ".object" -type "string" "brick_2_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -60 156.75 0 1;
createNode lambert -n "brickMat_2_6";
	setAttr ".c" -type "float3" 0.5729533940510922 0.2764766970255461 0.22 ;
createNode shadingEngine -n "brickMat_2_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody38";
	setAttr ".object" -type "string" "brick_2_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 60 156.75 0 1;
createNode lambert -n "brickMat_2_7";
	setAttr ".c" -type "float3" 0.6488639957090857 0.31443199785454284 0.22 ;
createNode shadingEngine -n "brickMat_2_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody39";
	setAttr ".object" -type "string" "brick_2_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 180 156.75 0 1;
createNode lambert -n "brickMat_2_8";
	setAttr ".c" -type "float3" 0.6084147689071044 0.2942073844535522 0.22 ;
createNode shadingEngine -n "brickMat_2_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody40";
	setAttr ".object" -type "string" "brick_2_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 300 156.75 0 1;
createNode lambert -n "brickMat_2_9";
	setAttr ".c" -type "float3" 0.5902635472602866 0.2851317736301433 0.22 ;
createNode shadingEngine -n "brickMat_2_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody41";
	setAttr ".object" -type "string" "brick_2_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 420 156.75 0 1;
createNode lambert -n "brickMat_2_10";
	setAttr ".c" -type "float3" 0.6336898981612222 0.3068449490806111 0.22 ;
createNode shadingEngine -n "brickMat_2_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody42";
	setAttr ".object" -type "string" "brick_2_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 540 156.75 0 1;
createNode lambert -n "brickMat_2_11";
	setAttr ".c" -type "float3" 0.612509500219324 0.296254750109662 0.22 ;
createNode shadingEngine -n "brickMat_2_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody43";
	setAttr ".object" -type "string" "brick_2_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 660 156.75 0 1;
createNode lambert -n "brickMat_2_12";
	setAttr ".c" -type "float3" 0.5879627194981891 0.28398135974909455 0.22 ;
createNode shadingEngine -n "brickMat_2_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody44";
	setAttr ".object" -type "string" "brick_2_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 780 156.75 0 1;
createNode lambert -n "brickMat_2_13";
	setAttr ".c" -type "float3" 0.6671007170435986 0.3235503585217993 0.22 ;
createNode shadingEngine -n "brickMat_2_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody45";
	setAttr ".object" -type "string" "brick_3_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -810 219 0 1;
createNode lambert -n "brickMat_3_0";
	setAttr ".c" -type "float3" 0.6518391093967314 0.3159195546983657 0.22 ;
createNode shadingEngine -n "brickMat_3_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody46";
	setAttr ".object" -type "string" "brick_3_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -720 219 0 1;
createNode lambert -n "brickMat_3_1";
	setAttr ".c" -type "float3" 0.5790554417155445 0.2795277208577722 0.22 ;
createNode shadingEngine -n "brickMat_3_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody47";
	setAttr ".object" -type "string" "brick_3_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -600 219 0 1;
createNode lambert -n "brickMat_3_2";
	setAttr ".c" -type "float3" 0.6319077936413874 0.3059538968206937 0.22 ;
createNode shadingEngine -n "brickMat_3_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody48";
	setAttr ".object" -type "string" "brick_3_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -480 219 0 1;
createNode lambert -n "brickMat_3_3";
	setAttr ".c" -type "float3" 0.6240314406098322 0.3020157203049161 0.22 ;
createNode shadingEngine -n "brickMat_3_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody49";
	setAttr ".object" -type "string" "brick_3_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -360 219 0 1;
createNode lambert -n "brickMat_3_4";
	setAttr ".c" -type "float3" 0.6800219992917487 0.3300109996458743 0.22 ;
createNode shadingEngine -n "brickMat_3_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody50";
	setAttr ".object" -type "string" "brick_3_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -240 219 0 1;
createNode lambert -n "brickMat_3_5";
	setAttr ".c" -type "float3" 0.6567112463102748 0.3183556231551374 0.22 ;
createNode shadingEngine -n "brickMat_3_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody51";
	setAttr ".object" -type "string" "brick_3_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -120 219 0 1;
createNode lambert -n "brickMat_3_6";
	setAttr ".c" -type "float3" 0.5860700423824299 0.28303502119121493 0.22 ;
createNode shadingEngine -n "brickMat_3_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody52";
	setAttr ".object" -type "string" "brick_3_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 0 219 0 1;
createNode lambert -n "brickMat_3_7";
	setAttr ".c" -type "float3" 0.6968279755988132 0.33841398779940657 0.22 ;
createNode shadingEngine -n "brickMat_3_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody53";
	setAttr ".object" -type "string" "brick_3_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 120 219 0 1;
createNode lambert -n "brickMat_3_8";
	setAttr ".c" -type "float3" 0.5588905245207939 0.26944526226039694 0.22 ;
createNode shadingEngine -n "brickMat_3_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody54";
	setAttr ".object" -type "string" "brick_3_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 240 219 0 1;
createNode lambert -n "brickMat_3_9";
	setAttr ".c" -type "float3" 0.6068996514856364 0.2934498257428182 0.22 ;
createNode shadingEngine -n "brickMat_3_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody55";
	setAttr ".object" -type "string" "brick_3_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 360 219 0 1;
createNode lambert -n "brickMat_3_10";
	setAttr ".c" -type "float3" 0.6611425487304399 0.3205712743652199 0.22 ;
createNode shadingEngine -n "brickMat_3_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody56";
	setAttr ".object" -type "string" "brick_3_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 480 219 0 1;
createNode lambert -n "brickMat_3_11";
	setAttr ".c" -type "float3" 0.5643175255456807 0.27215876277284035 0.22 ;
createNode shadingEngine -n "brickMat_3_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody57";
	setAttr ".object" -type "string" "brick_3_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 600 219 0 1;
createNode lambert -n "brickMat_3_12";
	setAttr ".c" -type "float3" 0.6182340960761289 0.29911704803806444 0.22 ;
createNode shadingEngine -n "brickMat_3_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody58";
	setAttr ".object" -type "string" "brick_3_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 720 219 0 1;
createNode lambert -n "brickMat_3_13";
	setAttr ".c" -type "float3" 0.5462731611275901 0.263136580563795 0.22 ;
createNode shadingEngine -n "brickMat_3_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody59";
	setAttr ".object" -type "string" "brick_3_14";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 810 219 0 1;
createNode lambert -n "brickMat_3_14";
	setAttr ".c" -type "float3" 0.6469145370455032 0.3134572685227516 0.22 ;
createNode shadingEngine -n "brickMat_3_14SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody60";
	setAttr ".object" -type "string" "brick_4_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -780 281.25 0 1;
createNode lambert -n "brickMat_4_0";
	setAttr ".c" -type "float3" 0.6623313385940501 0.32116566929702506 0.22 ;
createNode shadingEngine -n "brickMat_4_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody61";
	setAttr ".object" -type "string" "brick_4_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -660 281.25 0 1;
createNode lambert -n "brickMat_4_1";
	setAttr ".c" -type "float3" 0.6316841504443814 0.3058420752221907 0.22 ;
createNode shadingEngine -n "brickMat_4_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody62";
	setAttr ".object" -type "string" "brick_4_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -540 281.25 0 1;
createNode lambert -n "brickMat_4_2";
	setAttr ".c" -type "float3" 0.6800764498929421 0.33003822494647106 0.22 ;
createNode shadingEngine -n "brickMat_4_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody63";
	setAttr ".object" -type "string" "brick_4_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -420 281.25 0 1;
createNode lambert -n "brickMat_4_3";
	setAttr ".c" -type "float3" 0.5901996020556954 0.2850998010278477 0.22 ;
createNode shadingEngine -n "brickMat_4_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody64";
	setAttr ".object" -type "string" "brick_4_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -300 281.25 0 1;
createNode lambert -n "brickMat_4_4";
	setAttr ".c" -type "float3" 0.6512472586037855 0.31562362930189275 0.22 ;
createNode shadingEngine -n "brickMat_4_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody65";
	setAttr ".object" -type "string" "brick_4_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -180 281.25 0 1;
createNode lambert -n "brickMat_4_5";
	setAttr ".c" -type "float3" 0.6350991803368029 0.30754959016840144 0.22 ;
createNode shadingEngine -n "brickMat_4_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody66";
	setAttr ".object" -type "string" "brick_4_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -60 281.25 0 1;
createNode lambert -n "brickMat_4_6";
	setAttr ".c" -type "float3" 0.6327832326851988 0.30639161634259937 0.22 ;
createNode shadingEngine -n "brickMat_4_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody67";
	setAttr ".object" -type "string" "brick_4_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 60 281.25 0 1;
createNode lambert -n "brickMat_4_7";
	setAttr ".c" -type "float3" 0.612992853008226 0.296496426504113 0.22 ;
createNode shadingEngine -n "brickMat_4_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody68";
	setAttr ".object" -type "string" "brick_4_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 180 281.25 0 1;
createNode lambert -n "brickMat_4_8";
	setAttr ".c" -type "float3" 0.6743948448820066 0.3271974224410033 0.22 ;
createNode shadingEngine -n "brickMat_4_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody69";
	setAttr ".object" -type "string" "brick_4_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 300 281.25 0 1;
createNode lambert -n "brickMat_4_9";
	setAttr ".c" -type "float3" 0.69114897521727 0.335574487608635 0.22 ;
createNode shadingEngine -n "brickMat_4_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody70";
	setAttr ".object" -type "string" "brick_4_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 420 281.25 0 1;
createNode lambert -n "brickMat_4_10";
	setAttr ".c" -type "float3" 0.6158557339871431 0.29792786699357154 0.22 ;
createNode shadingEngine -n "brickMat_4_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody71";
	setAttr ".object" -type "string" "brick_4_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 540 281.25 0 1;
createNode lambert -n "brickMat_4_11";
	setAttr ".c" -type "float3" 0.646264352875948 0.313132176437974 0.22 ;
createNode shadingEngine -n "brickMat_4_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody72";
	setAttr ".object" -type "string" "brick_4_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 660 281.25 0 1;
createNode lambert -n "brickMat_4_12";
	setAttr ".c" -type "float3" 0.5497071084155551 0.26485355420777756 0.22 ;
createNode shadingEngine -n "brickMat_4_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody73";
	setAttr ".object" -type "string" "brick_4_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 780 281.25 0 1;
createNode lambert -n "brickMat_4_13";
	setAttr ".c" -type "float3" 0.6522387234087078 0.3161193617043539 0.22 ;
createNode shadingEngine -n "brickMat_4_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody74";
	setAttr ".object" -type "string" "brick_5_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -810 343.5 0 1;
createNode lambert -n "brickMat_5_0";
	setAttr ".c" -type "float3" 0.643540616724427 0.3117703083622135 0.22 ;
createNode shadingEngine -n "brickMat_5_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody75";
	setAttr ".object" -type "string" "brick_5_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -720 343.5 0 1;
createNode lambert -n "brickMat_5_1";
	setAttr ".c" -type "float3" 0.6988953503146614 0.3394476751573307 0.22 ;
createNode shadingEngine -n "brickMat_5_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody76";
	setAttr ".object" -type "string" "brick_5_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -600 343.5 0 1;
createNode lambert -n "brickMat_5_2";
	setAttr ".c" -type "float3" 0.6715079658575543 0.32575398292877716 0.22 ;
createNode shadingEngine -n "brickMat_5_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody77";
	setAttr ".object" -type "string" "brick_5_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -480 343.5 0 1;
createNode lambert -n "brickMat_5_3";
	setAttr ".c" -type "float3" 0.5855352851350639 0.28276764256753195 0.22 ;
createNode shadingEngine -n "brickMat_5_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody78";
	setAttr ".object" -type "string" "brick_5_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -360 343.5 0 1;
createNode lambert -n "brickMat_5_4";
	setAttr ".c" -type "float3" 0.6017266307914737 0.29086331539573684 0.22 ;
createNode shadingEngine -n "brickMat_5_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody79";
	setAttr ".object" -type "string" "brick_5_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -240 343.5 0 1;
createNode lambert -n "brickMat_5_5";
	setAttr ".c" -type "float3" 0.6469844345414701 0.31349221727073506 0.22 ;
createNode shadingEngine -n "brickMat_5_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody80";
	setAttr ".object" -type "string" "brick_5_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -120 343.5 0 1;
createNode lambert -n "brickMat_5_6";
	setAttr ".c" -type "float3" 0.5436100684888941 0.26180503424444707 0.22 ;
createNode shadingEngine -n "brickMat_5_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody81";
	setAttr ".object" -type "string" "brick_5_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 0 343.5 0 1;
createNode lambert -n "brickMat_5_7";
	setAttr ".c" -type "float3" 0.6138712458079625 0.29693562290398123 0.22 ;
createNode shadingEngine -n "brickMat_5_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody82";
	setAttr ".object" -type "string" "brick_5_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 120 343.5 0 1;
createNode lambert -n "brickMat_5_8";
	setAttr ".c" -type "float3" 0.5668877406250471 0.27344387031252354 0.22 ;
createNode shadingEngine -n "brickMat_5_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody83";
	setAttr ".object" -type "string" "brick_5_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 240 343.5 0 1;
createNode lambert -n "brickMat_5_9";
	setAttr ".c" -type "float3" 0.5587353271170771 0.26936766355853853 0.22 ;
createNode shadingEngine -n "brickMat_5_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody84";
	setAttr ".object" -type "string" "brick_5_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 360 343.5 0 1;
createNode lambert -n "brickMat_5_10";
	setAttr ".c" -type "float3" 0.5494327070930096 0.2647163535465048 0.22 ;
createNode shadingEngine -n "brickMat_5_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody85";
	setAttr ".object" -type "string" "brick_5_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 480 343.5 0 1;
createNode lambert -n "brickMat_5_11";
	setAttr ".c" -type "float3" 0.6629172781556033 0.32145863907780164 0.22 ;
createNode shadingEngine -n "brickMat_5_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody86";
	setAttr ".object" -type "string" "brick_5_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 600 343.5 0 1;
createNode lambert -n "brickMat_5_12";
	setAttr ".c" -type "float3" 0.5606944355229895 0.27034721776149473 0.22 ;
createNode shadingEngine -n "brickMat_5_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody87";
	setAttr ".object" -type "string" "brick_5_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 720 343.5 0 1;
createNode lambert -n "brickMat_5_13";
	setAttr ".c" -type "float3" 0.5796183733915062 0.2798091866957531 0.22 ;
createNode shadingEngine -n "brickMat_5_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody88";
	setAttr ".object" -type "string" "brick_5_14";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 810 343.5 0 1;
createNode lambert -n "brickMat_5_14";
	setAttr ".c" -type "float3" 0.6025519525013163 0.29127597625065815 0.22 ;
createNode shadingEngine -n "brickMat_5_14SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody89";
	setAttr ".object" -type "string" "brick_6_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -780 405.75 0 1;
createNode lambert -n "brickMat_6_0";
	setAttr ".c" -type "float3" 0.6794275158602079 0.32971375793010393 0.22 ;
createNode shadingEngine -n "brickMat_6_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody90";
	setAttr ".object" -type "string" "brick_6_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -660 405.75 0 1;
createNode lambert -n "brickMat_6_1";
	setAttr ".c" -type "float3" 0.5528930081920221 0.26644650409601106 0.22 ;
createNode shadingEngine -n "brickMat_6_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody91";
	setAttr ".object" -type "string" "brick_6_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -540 405.75 0 1;
createNode lambert -n "brickMat_6_2";
	setAttr ".c" -type "float3" 0.611869984151893 0.2959349920759465 0.22 ;
createNode shadingEngine -n "brickMat_6_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody92";
	setAttr ".object" -type "string" "brick_6_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -420 405.75 0 1;
createNode lambert -n "brickMat_6_3";
	setAttr ".c" -type "float3" 0.627910385463046 0.303955192731523 0.22 ;
createNode shadingEngine -n "brickMat_6_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody93";
	setAttr ".object" -type "string" "brick_6_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -300 405.75 0 1;
createNode lambert -n "brickMat_6_4";
	setAttr ".c" -type "float3" 0.681341412230642 0.330670706115321 0.22 ;
createNode shadingEngine -n "brickMat_6_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody94";
	setAttr ".object" -type "string" "brick_6_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -180 405.75 0 1;
createNode lambert -n "brickMat_6_5";
	setAttr ".c" -type "float3" 0.6710847740537186 0.3255423870268593 0.22 ;
createNode shadingEngine -n "brickMat_6_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody95";
	setAttr ".object" -type "string" "brick_6_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -60 405.75 0 1;
createNode lambert -n "brickMat_6_6";
	setAttr ".c" -type "float3" 0.6782375151517624 0.3291187575758812 0.22 ;
createNode shadingEngine -n "brickMat_6_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody96";
	setAttr ".object" -type "string" "brick_6_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 60 405.75 0 1;
createNode lambert -n "brickMat_6_7";
	setAttr ".c" -type "float3" 0.5845473703222235 0.28227368516111173 0.22 ;
createNode shadingEngine -n "brickMat_6_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody97";
	setAttr ".object" -type "string" "brick_6_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 180 405.75 0 1;
createNode lambert -n "brickMat_6_8";
	setAttr ".c" -type "float3" 0.6064474427538717 0.29322372137693586 0.22 ;
createNode shadingEngine -n "brickMat_6_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody98";
	setAttr ".object" -type "string" "brick_6_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 300 405.75 0 1;
createNode lambert -n "brickMat_6_9";
	setAttr ".c" -type "float3" 0.5974033864530599 0.28870169322652994 0.22 ;
createNode shadingEngine -n "brickMat_6_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody99";
	setAttr ".object" -type "string" "brick_6_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 420 405.75 0 1;
createNode lambert -n "brickMat_6_10";
	setAttr ".c" -type "float3" 0.6814708523517147 0.3307354261758573 0.22 ;
createNode shadingEngine -n "brickMat_6_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody100";
	setAttr ".object" -type "string" "brick_6_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 540 405.75 0 1;
createNode lambert -n "brickMat_6_11";
	setAttr ".c" -type "float3" 0.6932369926342385 0.33661849631711926 0.22 ;
createNode shadingEngine -n "brickMat_6_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody101";
	setAttr ".object" -type "string" "brick_6_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 660 405.75 0 1;
createNode lambert -n "brickMat_6_12";
	setAttr ".c" -type "float3" 0.5641473449265775 0.27207367246328873 0.22 ;
createNode shadingEngine -n "brickMat_6_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody102";
	setAttr ".object" -type "string" "brick_6_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 780 405.75 0 1;
createNode lambert -n "brickMat_6_13";
	setAttr ".c" -type "float3" 0.5681948365584593 0.2740974182792296 0.22 ;
createNode shadingEngine -n "brickMat_6_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody103";
	setAttr ".object" -type "string" "brick_7_0";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -810 468 0 1;
createNode lambert -n "brickMat_7_0";
	setAttr ".c" -type "float3" 0.5771130986911257 0.27855654934556284 0.22 ;
createNode shadingEngine -n "brickMat_7_0SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody104";
	setAttr ".object" -type "string" "brick_7_1";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -720 468 0 1;
createNode lambert -n "brickMat_7_1";
	setAttr ".c" -type "float3" 0.5773337733889378 0.2786668866944689 0.22 ;
createNode shadingEngine -n "brickMat_7_1SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody105";
	setAttr ".object" -type "string" "brick_7_2";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -600 468 0 1;
createNode lambert -n "brickMat_7_2";
	setAttr ".c" -type "float3" 0.6175940368546171 0.29879701842730855 0.22 ;
createNode shadingEngine -n "brickMat_7_2SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody106";
	setAttr ".object" -type "string" "brick_7_3";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -480 468 0 1;
createNode lambert -n "brickMat_7_3";
	setAttr ".c" -type "float3" 0.6342597605971609 0.30712988029858046 0.22 ;
createNode shadingEngine -n "brickMat_7_3SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody107";
	setAttr ".object" -type "string" "brick_7_4";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -360 468 0 1;
createNode lambert -n "brickMat_7_4";
	setAttr ".c" -type "float3" 0.5820394590877661 0.28101972954388305 0.22 ;
createNode shadingEngine -n "brickMat_7_4SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody108";
	setAttr ".object" -type "string" "brick_7_5";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -240 468 0 1;
createNode lambert -n "brickMat_7_5";
	setAttr ".c" -type "float3" 0.5406549765416102 0.2603274882708051 0.22 ;
createNode shadingEngine -n "brickMat_7_5SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody109";
	setAttr ".object" -type "string" "brick_7_6";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -120 468 0 1;
createNode lambert -n "brickMat_7_6";
	setAttr ".c" -type "float3" 0.6070314401800525 0.29351572009002624 0.22 ;
createNode shadingEngine -n "brickMat_7_6SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody110";
	setAttr ".object" -type "string" "brick_7_7";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 0 468 0 1;
createNode lambert -n "brickMat_7_7";
	setAttr ".c" -type "float3" 0.5990805716631561 0.28954028583157804 0.22 ;
createNode shadingEngine -n "brickMat_7_7SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody111";
	setAttr ".object" -type "string" "brick_7_8";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 120 468 0 1;
createNode lambert -n "brickMat_7_8";
	setAttr ".c" -type "float3" 0.6306145957930227 0.30530729789651134 0.22 ;
createNode shadingEngine -n "brickMat_7_8SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody112";
	setAttr ".object" -type "string" "brick_7_9";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 240 468 0 1;
createNode lambert -n "brickMat_7_9";
	setAttr ".c" -type "float3" 0.6924956680840152 0.3362478340420076 0.22 ;
createNode shadingEngine -n "brickMat_7_9SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody113";
	setAttr ".object" -type "string" "brick_7_10";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 360 468 0 1;
createNode lambert -n "brickMat_7_10";
	setAttr ".c" -type "float3" 0.6504789851417565 0.31523949257087824 0.22 ;
createNode shadingEngine -n "brickMat_7_10SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody114";
	setAttr ".object" -type "string" "brick_7_11";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 480 468 0 1;
createNode lambert -n "brickMat_7_11";
	setAttr ".c" -type "float3" 0.6224786292913246 0.3012393146456623 0.22 ;
createNode shadingEngine -n "brickMat_7_11SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody115";
	setAttr ".object" -type "string" "brick_7_12";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 600 468 0 1;
createNode lambert -n "brickMat_7_12";
	setAttr ".c" -type "float3" 0.6388148399054604 0.3094074199527302 0.22 ;
createNode shadingEngine -n "brickMat_7_12SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody116";
	setAttr ".object" -type "string" "brick_7_13";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 720 468 0 1;
createNode lambert -n "brickMat_7_13";
	setAttr ".c" -type "float3" 0.6481920131919202 0.3140960065959601 0.22 ;
createNode shadingEngine -n "brickMat_7_13SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode rigidBody -n "rigidBody117";
	setAttr ".object" -type "string" "brick_7_14";
	setAttr ".mass" 8;
	setAttr ".bounciness" 0.05;
	setAttr ".staticFriction" 0.5;
	setAttr ".dynamicFriction" 0.5;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 810 468 0 1;
createNode lambert -n "brickMat_7_14";
	setAttr ".c" -type "float3" 0.5486388629158064 0.2643194314579032 0.22 ;
createNode shadingEngine -n "brickMat_7_14SG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode dynamicConstraint -n "glue_0_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_0";
	setAttr ".targets" -type "string" "brick_0_1";
createNode dynamicConstraint -n "glue_0_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_0";
	setAttr ".targets" -type "string" "brick_1_0";
createNode dynamicConstraint -n "glue_0_0_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_0";
	setAttr ".targets" -type "string" "brick_1_1";
createNode dynamicConstraint -n "glue_0_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_1";
	setAttr ".targets" -type "string" "brick_0_2";
createNode dynamicConstraint -n "glue_0_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_1";
	setAttr ".targets" -type "string" "brick_1_1";
createNode dynamicConstraint -n "glue_0_1_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_1";
	setAttr ".targets" -type "string" "brick_1_2";
createNode dynamicConstraint -n "glue_0_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_2";
	setAttr ".targets" -type "string" "brick_0_3";
createNode dynamicConstraint -n "glue_0_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_2";
	setAttr ".targets" -type "string" "brick_1_2";
createNode dynamicConstraint -n "glue_0_2_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_2";
	setAttr ".targets" -type "string" "brick_1_3";
createNode dynamicConstraint -n "glue_0_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_3";
	setAttr ".targets" -type "string" "brick_0_4";
createNode dynamicConstraint -n "glue_0_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_3";
	setAttr ".targets" -type "string" "brick_1_3";
createNode dynamicConstraint -n "glue_0_3_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_3";
	setAttr ".targets" -type "string" "brick_1_4";
createNode dynamicConstraint -n "glue_0_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_4";
	setAttr ".targets" -type "string" "brick_0_5";
createNode dynamicConstraint -n "glue_0_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_4";
	setAttr ".targets" -type "string" "brick_1_4";
createNode dynamicConstraint -n "glue_0_4_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_4";
	setAttr ".targets" -type "string" "brick_1_5";
createNode dynamicConstraint -n "glue_0_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_5";
	setAttr ".targets" -type "string" "brick_0_6";
createNode dynamicConstraint -n "glue_0_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_5";
	setAttr ".targets" -type "string" "brick_1_5";
createNode dynamicConstraint -n "glue_0_5_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_5";
	setAttr ".targets" -type "string" "brick_1_6";
createNode dynamicConstraint -n "glue_0_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_6";
	setAttr ".targets" -type "string" "brick_0_7";
createNode dynamicConstraint -n "glue_0_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_6";
	setAttr ".targets" -type "string" "brick_1_6";
createNode dynamicConstraint -n "glue_0_6_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_6";
	setAttr ".targets" -type "string" "brick_1_7";
createNode dynamicConstraint -n "glue_0_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_7";
	setAttr ".targets" -type "string" "brick_0_8";
createNode dynamicConstraint -n "glue_0_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_7";
	setAttr ".targets" -type "string" "brick_1_7";
createNode dynamicConstraint -n "glue_0_7_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_7";
	setAttr ".targets" -type "string" "brick_1_8";
createNode dynamicConstraint -n "glue_0_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_8";
	setAttr ".targets" -type "string" "brick_0_9";
createNode dynamicConstraint -n "glue_0_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_8";
	setAttr ".targets" -type "string" "brick_1_8";
createNode dynamicConstraint -n "glue_0_8_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_8";
	setAttr ".targets" -type "string" "brick_1_9";
createNode dynamicConstraint -n "glue_0_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_9";
	setAttr ".targets" -type "string" "brick_0_10";
createNode dynamicConstraint -n "glue_0_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_9";
	setAttr ".targets" -type "string" "brick_1_9";
createNode dynamicConstraint -n "glue_0_9_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_9";
	setAttr ".targets" -type "string" "brick_1_10";
createNode dynamicConstraint -n "glue_0_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_10";
	setAttr ".targets" -type "string" "brick_0_11";
createNode dynamicConstraint -n "glue_0_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_10";
	setAttr ".targets" -type "string" "brick_1_10";
createNode dynamicConstraint -n "glue_0_10_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_10";
	setAttr ".targets" -type "string" "brick_1_11";
createNode dynamicConstraint -n "glue_0_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_11";
	setAttr ".targets" -type "string" "brick_0_12";
createNode dynamicConstraint -n "glue_0_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_11";
	setAttr ".targets" -type "string" "brick_1_11";
createNode dynamicConstraint -n "glue_0_11_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_11";
	setAttr ".targets" -type "string" "brick_1_12";
createNode dynamicConstraint -n "glue_0_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_12";
	setAttr ".targets" -type "string" "brick_0_13";
createNode dynamicConstraint -n "glue_0_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_12";
	setAttr ".targets" -type "string" "brick_1_12";
createNode dynamicConstraint -n "glue_0_12_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_12";
	setAttr ".targets" -type "string" "brick_1_13";
createNode dynamicConstraint -n "glue_0_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_13";
	setAttr ".targets" -type "string" "brick_1_13";
createNode dynamicConstraint -n "glue_0_13_up14";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_0_13";
	setAttr ".targets" -type "string" "brick_1_14";
createNode dynamicConstraint -n "glue_1_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_0";
	setAttr ".targets" -type "string" "brick_1_1";
createNode dynamicConstraint -n "glue_1_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_0";
	setAttr ".targets" -type "string" "brick_2_0";
createNode dynamicConstraint -n "glue_1_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_1";
	setAttr ".targets" -type "string" "brick_1_2";
createNode dynamicConstraint -n "glue_1_1_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_1";
	setAttr ".targets" -type "string" "brick_2_0";
createNode dynamicConstraint -n "glue_1_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_1";
	setAttr ".targets" -type "string" "brick_2_1";
createNode dynamicConstraint -n "glue_1_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_2";
	setAttr ".targets" -type "string" "brick_1_3";
createNode dynamicConstraint -n "glue_1_2_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_2";
	setAttr ".targets" -type "string" "brick_2_1";
createNode dynamicConstraint -n "glue_1_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_2";
	setAttr ".targets" -type "string" "brick_2_2";
createNode dynamicConstraint -n "glue_1_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_3";
	setAttr ".targets" -type "string" "brick_1_4";
createNode dynamicConstraint -n "glue_1_3_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_3";
	setAttr ".targets" -type "string" "brick_2_2";
createNode dynamicConstraint -n "glue_1_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_3";
	setAttr ".targets" -type "string" "brick_2_3";
createNode dynamicConstraint -n "glue_1_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_4";
	setAttr ".targets" -type "string" "brick_1_5";
createNode dynamicConstraint -n "glue_1_4_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_4";
	setAttr ".targets" -type "string" "brick_2_3";
createNode dynamicConstraint -n "glue_1_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_4";
	setAttr ".targets" -type "string" "brick_2_4";
createNode dynamicConstraint -n "glue_1_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_5";
	setAttr ".targets" -type "string" "brick_1_6";
createNode dynamicConstraint -n "glue_1_5_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_5";
	setAttr ".targets" -type "string" "brick_2_4";
createNode dynamicConstraint -n "glue_1_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_5";
	setAttr ".targets" -type "string" "brick_2_5";
createNode dynamicConstraint -n "glue_1_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_6";
	setAttr ".targets" -type "string" "brick_1_7";
createNode dynamicConstraint -n "glue_1_6_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_6";
	setAttr ".targets" -type "string" "brick_2_5";
createNode dynamicConstraint -n "glue_1_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_6";
	setAttr ".targets" -type "string" "brick_2_6";
createNode dynamicConstraint -n "glue_1_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_7";
	setAttr ".targets" -type "string" "brick_1_8";
createNode dynamicConstraint -n "glue_1_7_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_7";
	setAttr ".targets" -type "string" "brick_2_6";
createNode dynamicConstraint -n "glue_1_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_7";
	setAttr ".targets" -type "string" "brick_2_7";
createNode dynamicConstraint -n "glue_1_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_8";
	setAttr ".targets" -type "string" "brick_1_9";
createNode dynamicConstraint -n "glue_1_8_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_8";
	setAttr ".targets" -type "string" "brick_2_7";
createNode dynamicConstraint -n "glue_1_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_8";
	setAttr ".targets" -type "string" "brick_2_8";
createNode dynamicConstraint -n "glue_1_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_9";
	setAttr ".targets" -type "string" "brick_1_10";
createNode dynamicConstraint -n "glue_1_9_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_9";
	setAttr ".targets" -type "string" "brick_2_8";
createNode dynamicConstraint -n "glue_1_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_9";
	setAttr ".targets" -type "string" "brick_2_9";
createNode dynamicConstraint -n "glue_1_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_10";
	setAttr ".targets" -type "string" "brick_1_11";
createNode dynamicConstraint -n "glue_1_10_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_10";
	setAttr ".targets" -type "string" "brick_2_9";
createNode dynamicConstraint -n "glue_1_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_10";
	setAttr ".targets" -type "string" "brick_2_10";
createNode dynamicConstraint -n "glue_1_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_11";
	setAttr ".targets" -type "string" "brick_1_12";
createNode dynamicConstraint -n "glue_1_11_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_11";
	setAttr ".targets" -type "string" "brick_2_10";
createNode dynamicConstraint -n "glue_1_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_11";
	setAttr ".targets" -type "string" "brick_2_11";
createNode dynamicConstraint -n "glue_1_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_12";
	setAttr ".targets" -type "string" "brick_1_13";
createNode dynamicConstraint -n "glue_1_12_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_12";
	setAttr ".targets" -type "string" "brick_2_11";
createNode dynamicConstraint -n "glue_1_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_12";
	setAttr ".targets" -type "string" "brick_2_12";
createNode dynamicConstraint -n "glue_1_13_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_13";
	setAttr ".targets" -type "string" "brick_1_14";
createNode dynamicConstraint -n "glue_1_13_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_13";
	setAttr ".targets" -type "string" "brick_2_12";
createNode dynamicConstraint -n "glue_1_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_13";
	setAttr ".targets" -type "string" "brick_2_13";
createNode dynamicConstraint -n "glue_1_14_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_1_14";
	setAttr ".targets" -type "string" "brick_2_13";
createNode dynamicConstraint -n "glue_2_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_0";
	setAttr ".targets" -type "string" "brick_2_1";
createNode dynamicConstraint -n "glue_2_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_0";
	setAttr ".targets" -type "string" "brick_3_0";
createNode dynamicConstraint -n "glue_2_0_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_0";
	setAttr ".targets" -type "string" "brick_3_1";
createNode dynamicConstraint -n "glue_2_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_1";
	setAttr ".targets" -type "string" "brick_2_2";
createNode dynamicConstraint -n "glue_2_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_1";
	setAttr ".targets" -type "string" "brick_3_1";
createNode dynamicConstraint -n "glue_2_1_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_1";
	setAttr ".targets" -type "string" "brick_3_2";
createNode dynamicConstraint -n "glue_2_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_2";
	setAttr ".targets" -type "string" "brick_2_3";
createNode dynamicConstraint -n "glue_2_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_2";
	setAttr ".targets" -type "string" "brick_3_2";
createNode dynamicConstraint -n "glue_2_2_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_2";
	setAttr ".targets" -type "string" "brick_3_3";
createNode dynamicConstraint -n "glue_2_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_3";
	setAttr ".targets" -type "string" "brick_2_4";
createNode dynamicConstraint -n "glue_2_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_3";
	setAttr ".targets" -type "string" "brick_3_3";
createNode dynamicConstraint -n "glue_2_3_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_3";
	setAttr ".targets" -type "string" "brick_3_4";
createNode dynamicConstraint -n "glue_2_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_4";
	setAttr ".targets" -type "string" "brick_2_5";
createNode dynamicConstraint -n "glue_2_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_4";
	setAttr ".targets" -type "string" "brick_3_4";
createNode dynamicConstraint -n "glue_2_4_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_4";
	setAttr ".targets" -type "string" "brick_3_5";
createNode dynamicConstraint -n "glue_2_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_5";
	setAttr ".targets" -type "string" "brick_2_6";
createNode dynamicConstraint -n "glue_2_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_5";
	setAttr ".targets" -type "string" "brick_3_5";
createNode dynamicConstraint -n "glue_2_5_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_5";
	setAttr ".targets" -type "string" "brick_3_6";
createNode dynamicConstraint -n "glue_2_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_6";
	setAttr ".targets" -type "string" "brick_2_7";
createNode dynamicConstraint -n "glue_2_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_6";
	setAttr ".targets" -type "string" "brick_3_6";
createNode dynamicConstraint -n "glue_2_6_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_6";
	setAttr ".targets" -type "string" "brick_3_7";
createNode dynamicConstraint -n "glue_2_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_7";
	setAttr ".targets" -type "string" "brick_2_8";
createNode dynamicConstraint -n "glue_2_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_7";
	setAttr ".targets" -type "string" "brick_3_7";
createNode dynamicConstraint -n "glue_2_7_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_7";
	setAttr ".targets" -type "string" "brick_3_8";
createNode dynamicConstraint -n "glue_2_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_8";
	setAttr ".targets" -type "string" "brick_2_9";
createNode dynamicConstraint -n "glue_2_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_8";
	setAttr ".targets" -type "string" "brick_3_8";
createNode dynamicConstraint -n "glue_2_8_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_8";
	setAttr ".targets" -type "string" "brick_3_9";
createNode dynamicConstraint -n "glue_2_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_9";
	setAttr ".targets" -type "string" "brick_2_10";
createNode dynamicConstraint -n "glue_2_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_9";
	setAttr ".targets" -type "string" "brick_3_9";
createNode dynamicConstraint -n "glue_2_9_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_9";
	setAttr ".targets" -type "string" "brick_3_10";
createNode dynamicConstraint -n "glue_2_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_10";
	setAttr ".targets" -type "string" "brick_2_11";
createNode dynamicConstraint -n "glue_2_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_10";
	setAttr ".targets" -type "string" "brick_3_10";
createNode dynamicConstraint -n "glue_2_10_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_10";
	setAttr ".targets" -type "string" "brick_3_11";
createNode dynamicConstraint -n "glue_2_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_11";
	setAttr ".targets" -type "string" "brick_2_12";
createNode dynamicConstraint -n "glue_2_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_11";
	setAttr ".targets" -type "string" "brick_3_11";
createNode dynamicConstraint -n "glue_2_11_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_11";
	setAttr ".targets" -type "string" "brick_3_12";
createNode dynamicConstraint -n "glue_2_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_12";
	setAttr ".targets" -type "string" "brick_2_13";
createNode dynamicConstraint -n "glue_2_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_12";
	setAttr ".targets" -type "string" "brick_3_12";
createNode dynamicConstraint -n "glue_2_12_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_12";
	setAttr ".targets" -type "string" "brick_3_13";
createNode dynamicConstraint -n "glue_2_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_13";
	setAttr ".targets" -type "string" "brick_3_13";
createNode dynamicConstraint -n "glue_2_13_up14";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_2_13";
	setAttr ".targets" -type "string" "brick_3_14";
createNode dynamicConstraint -n "glue_3_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_0";
	setAttr ".targets" -type "string" "brick_3_1";
createNode dynamicConstraint -n "glue_3_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_0";
	setAttr ".targets" -type "string" "brick_4_0";
createNode dynamicConstraint -n "glue_3_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_1";
	setAttr ".targets" -type "string" "brick_3_2";
createNode dynamicConstraint -n "glue_3_1_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_1";
	setAttr ".targets" -type "string" "brick_4_0";
createNode dynamicConstraint -n "glue_3_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_1";
	setAttr ".targets" -type "string" "brick_4_1";
createNode dynamicConstraint -n "glue_3_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_2";
	setAttr ".targets" -type "string" "brick_3_3";
createNode dynamicConstraint -n "glue_3_2_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_2";
	setAttr ".targets" -type "string" "brick_4_1";
createNode dynamicConstraint -n "glue_3_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_2";
	setAttr ".targets" -type "string" "brick_4_2";
createNode dynamicConstraint -n "glue_3_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_3";
	setAttr ".targets" -type "string" "brick_3_4";
createNode dynamicConstraint -n "glue_3_3_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_3";
	setAttr ".targets" -type "string" "brick_4_2";
createNode dynamicConstraint -n "glue_3_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_3";
	setAttr ".targets" -type "string" "brick_4_3";
createNode dynamicConstraint -n "glue_3_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_4";
	setAttr ".targets" -type "string" "brick_3_5";
createNode dynamicConstraint -n "glue_3_4_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_4";
	setAttr ".targets" -type "string" "brick_4_3";
createNode dynamicConstraint -n "glue_3_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_4";
	setAttr ".targets" -type "string" "brick_4_4";
createNode dynamicConstraint -n "glue_3_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_5";
	setAttr ".targets" -type "string" "brick_3_6";
createNode dynamicConstraint -n "glue_3_5_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_5";
	setAttr ".targets" -type "string" "brick_4_4";
createNode dynamicConstraint -n "glue_3_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_5";
	setAttr ".targets" -type "string" "brick_4_5";
createNode dynamicConstraint -n "glue_3_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_6";
	setAttr ".targets" -type "string" "brick_3_7";
createNode dynamicConstraint -n "glue_3_6_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_6";
	setAttr ".targets" -type "string" "brick_4_5";
createNode dynamicConstraint -n "glue_3_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_6";
	setAttr ".targets" -type "string" "brick_4_6";
createNode dynamicConstraint -n "glue_3_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_7";
	setAttr ".targets" -type "string" "brick_3_8";
createNode dynamicConstraint -n "glue_3_7_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_7";
	setAttr ".targets" -type "string" "brick_4_6";
createNode dynamicConstraint -n "glue_3_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_7";
	setAttr ".targets" -type "string" "brick_4_7";
createNode dynamicConstraint -n "glue_3_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_8";
	setAttr ".targets" -type "string" "brick_3_9";
createNode dynamicConstraint -n "glue_3_8_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_8";
	setAttr ".targets" -type "string" "brick_4_7";
createNode dynamicConstraint -n "glue_3_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_8";
	setAttr ".targets" -type "string" "brick_4_8";
createNode dynamicConstraint -n "glue_3_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_9";
	setAttr ".targets" -type "string" "brick_3_10";
createNode dynamicConstraint -n "glue_3_9_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_9";
	setAttr ".targets" -type "string" "brick_4_8";
createNode dynamicConstraint -n "glue_3_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_9";
	setAttr ".targets" -type "string" "brick_4_9";
createNode dynamicConstraint -n "glue_3_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_10";
	setAttr ".targets" -type "string" "brick_3_11";
createNode dynamicConstraint -n "glue_3_10_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_10";
	setAttr ".targets" -type "string" "brick_4_9";
createNode dynamicConstraint -n "glue_3_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_10";
	setAttr ".targets" -type "string" "brick_4_10";
createNode dynamicConstraint -n "glue_3_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_11";
	setAttr ".targets" -type "string" "brick_3_12";
createNode dynamicConstraint -n "glue_3_11_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_11";
	setAttr ".targets" -type "string" "brick_4_10";
createNode dynamicConstraint -n "glue_3_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_11";
	setAttr ".targets" -type "string" "brick_4_11";
createNode dynamicConstraint -n "glue_3_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_12";
	setAttr ".targets" -type "string" "brick_3_13";
createNode dynamicConstraint -n "glue_3_12_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_12";
	setAttr ".targets" -type "string" "brick_4_11";
createNode dynamicConstraint -n "glue_3_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_12";
	setAttr ".targets" -type "string" "brick_4_12";
createNode dynamicConstraint -n "glue_3_13_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_13";
	setAttr ".targets" -type "string" "brick_3_14";
createNode dynamicConstraint -n "glue_3_13_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_13";
	setAttr ".targets" -type "string" "brick_4_12";
createNode dynamicConstraint -n "glue_3_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_13";
	setAttr ".targets" -type "string" "brick_4_13";
createNode dynamicConstraint -n "glue_3_14_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_3_14";
	setAttr ".targets" -type "string" "brick_4_13";
createNode dynamicConstraint -n "glue_4_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_0";
	setAttr ".targets" -type "string" "brick_4_1";
createNode dynamicConstraint -n "glue_4_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_0";
	setAttr ".targets" -type "string" "brick_5_0";
createNode dynamicConstraint -n "glue_4_0_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_0";
	setAttr ".targets" -type "string" "brick_5_1";
createNode dynamicConstraint -n "glue_4_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_1";
	setAttr ".targets" -type "string" "brick_4_2";
createNode dynamicConstraint -n "glue_4_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_1";
	setAttr ".targets" -type "string" "brick_5_1";
createNode dynamicConstraint -n "glue_4_1_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_1";
	setAttr ".targets" -type "string" "brick_5_2";
createNode dynamicConstraint -n "glue_4_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_2";
	setAttr ".targets" -type "string" "brick_4_3";
createNode dynamicConstraint -n "glue_4_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_2";
	setAttr ".targets" -type "string" "brick_5_2";
createNode dynamicConstraint -n "glue_4_2_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_2";
	setAttr ".targets" -type "string" "brick_5_3";
createNode dynamicConstraint -n "glue_4_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_3";
	setAttr ".targets" -type "string" "brick_4_4";
createNode dynamicConstraint -n "glue_4_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_3";
	setAttr ".targets" -type "string" "brick_5_3";
createNode dynamicConstraint -n "glue_4_3_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_3";
	setAttr ".targets" -type "string" "brick_5_4";
createNode dynamicConstraint -n "glue_4_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_4";
	setAttr ".targets" -type "string" "brick_4_5";
createNode dynamicConstraint -n "glue_4_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_4";
	setAttr ".targets" -type "string" "brick_5_4";
createNode dynamicConstraint -n "glue_4_4_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_4";
	setAttr ".targets" -type "string" "brick_5_5";
createNode dynamicConstraint -n "glue_4_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_5";
	setAttr ".targets" -type "string" "brick_4_6";
createNode dynamicConstraint -n "glue_4_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_5";
	setAttr ".targets" -type "string" "brick_5_5";
createNode dynamicConstraint -n "glue_4_5_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_5";
	setAttr ".targets" -type "string" "brick_5_6";
createNode dynamicConstraint -n "glue_4_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_6";
	setAttr ".targets" -type "string" "brick_4_7";
createNode dynamicConstraint -n "glue_4_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_6";
	setAttr ".targets" -type "string" "brick_5_6";
createNode dynamicConstraint -n "glue_4_6_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_6";
	setAttr ".targets" -type "string" "brick_5_7";
createNode dynamicConstraint -n "glue_4_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_7";
	setAttr ".targets" -type "string" "brick_4_8";
createNode dynamicConstraint -n "glue_4_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_7";
	setAttr ".targets" -type "string" "brick_5_7";
createNode dynamicConstraint -n "glue_4_7_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_7";
	setAttr ".targets" -type "string" "brick_5_8";
createNode dynamicConstraint -n "glue_4_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_8";
	setAttr ".targets" -type "string" "brick_4_9";
createNode dynamicConstraint -n "glue_4_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_8";
	setAttr ".targets" -type "string" "brick_5_8";
createNode dynamicConstraint -n "glue_4_8_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_8";
	setAttr ".targets" -type "string" "brick_5_9";
createNode dynamicConstraint -n "glue_4_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_9";
	setAttr ".targets" -type "string" "brick_4_10";
createNode dynamicConstraint -n "glue_4_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_9";
	setAttr ".targets" -type "string" "brick_5_9";
createNode dynamicConstraint -n "glue_4_9_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_9";
	setAttr ".targets" -type "string" "brick_5_10";
createNode dynamicConstraint -n "glue_4_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_10";
	setAttr ".targets" -type "string" "brick_4_11";
createNode dynamicConstraint -n "glue_4_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_10";
	setAttr ".targets" -type "string" "brick_5_10";
createNode dynamicConstraint -n "glue_4_10_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_10";
	setAttr ".targets" -type "string" "brick_5_11";
createNode dynamicConstraint -n "glue_4_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_11";
	setAttr ".targets" -type "string" "brick_4_12";
createNode dynamicConstraint -n "glue_4_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_11";
	setAttr ".targets" -type "string" "brick_5_11";
createNode dynamicConstraint -n "glue_4_11_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_11";
	setAttr ".targets" -type "string" "brick_5_12";
createNode dynamicConstraint -n "glue_4_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_12";
	setAttr ".targets" -type "string" "brick_4_13";
createNode dynamicConstraint -n "glue_4_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_12";
	setAttr ".targets" -type "string" "brick_5_12";
createNode dynamicConstraint -n "glue_4_12_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_12";
	setAttr ".targets" -type "string" "brick_5_13";
createNode dynamicConstraint -n "glue_4_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_13";
	setAttr ".targets" -type "string" "brick_5_13";
createNode dynamicConstraint -n "glue_4_13_up14";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_4_13";
	setAttr ".targets" -type "string" "brick_5_14";
createNode dynamicConstraint -n "glue_5_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_0";
	setAttr ".targets" -type "string" "brick_5_1";
createNode dynamicConstraint -n "glue_5_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_0";
	setAttr ".targets" -type "string" "brick_6_0";
createNode dynamicConstraint -n "glue_5_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_1";
	setAttr ".targets" -type "string" "brick_5_2";
createNode dynamicConstraint -n "glue_5_1_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_1";
	setAttr ".targets" -type "string" "brick_6_0";
createNode dynamicConstraint -n "glue_5_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_1";
	setAttr ".targets" -type "string" "brick_6_1";
createNode dynamicConstraint -n "glue_5_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_2";
	setAttr ".targets" -type "string" "brick_5_3";
createNode dynamicConstraint -n "glue_5_2_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_2";
	setAttr ".targets" -type "string" "brick_6_1";
createNode dynamicConstraint -n "glue_5_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_2";
	setAttr ".targets" -type "string" "brick_6_2";
createNode dynamicConstraint -n "glue_5_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_3";
	setAttr ".targets" -type "string" "brick_5_4";
createNode dynamicConstraint -n "glue_5_3_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_3";
	setAttr ".targets" -type "string" "brick_6_2";
createNode dynamicConstraint -n "glue_5_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_3";
	setAttr ".targets" -type "string" "brick_6_3";
createNode dynamicConstraint -n "glue_5_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_4";
	setAttr ".targets" -type "string" "brick_5_5";
createNode dynamicConstraint -n "glue_5_4_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_4";
	setAttr ".targets" -type "string" "brick_6_3";
createNode dynamicConstraint -n "glue_5_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_4";
	setAttr ".targets" -type "string" "brick_6_4";
createNode dynamicConstraint -n "glue_5_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_5";
	setAttr ".targets" -type "string" "brick_5_6";
createNode dynamicConstraint -n "glue_5_5_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_5";
	setAttr ".targets" -type "string" "brick_6_4";
createNode dynamicConstraint -n "glue_5_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_5";
	setAttr ".targets" -type "string" "brick_6_5";
createNode dynamicConstraint -n "glue_5_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_6";
	setAttr ".targets" -type "string" "brick_5_7";
createNode dynamicConstraint -n "glue_5_6_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_6";
	setAttr ".targets" -type "string" "brick_6_5";
createNode dynamicConstraint -n "glue_5_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_6";
	setAttr ".targets" -type "string" "brick_6_6";
createNode dynamicConstraint -n "glue_5_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_7";
	setAttr ".targets" -type "string" "brick_5_8";
createNode dynamicConstraint -n "glue_5_7_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_7";
	setAttr ".targets" -type "string" "brick_6_6";
createNode dynamicConstraint -n "glue_5_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_7";
	setAttr ".targets" -type "string" "brick_6_7";
createNode dynamicConstraint -n "glue_5_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_8";
	setAttr ".targets" -type "string" "brick_5_9";
createNode dynamicConstraint -n "glue_5_8_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_8";
	setAttr ".targets" -type "string" "brick_6_7";
createNode dynamicConstraint -n "glue_5_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_8";
	setAttr ".targets" -type "string" "brick_6_8";
createNode dynamicConstraint -n "glue_5_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_9";
	setAttr ".targets" -type "string" "brick_5_10";
createNode dynamicConstraint -n "glue_5_9_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_9";
	setAttr ".targets" -type "string" "brick_6_8";
createNode dynamicConstraint -n "glue_5_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_9";
	setAttr ".targets" -type "string" "brick_6_9";
createNode dynamicConstraint -n "glue_5_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_10";
	setAttr ".targets" -type "string" "brick_5_11";
createNode dynamicConstraint -n "glue_5_10_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_10";
	setAttr ".targets" -type "string" "brick_6_9";
createNode dynamicConstraint -n "glue_5_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_10";
	setAttr ".targets" -type "string" "brick_6_10";
createNode dynamicConstraint -n "glue_5_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_11";
	setAttr ".targets" -type "string" "brick_5_12";
createNode dynamicConstraint -n "glue_5_11_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_11";
	setAttr ".targets" -type "string" "brick_6_10";
createNode dynamicConstraint -n "glue_5_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_11";
	setAttr ".targets" -type "string" "brick_6_11";
createNode dynamicConstraint -n "glue_5_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_12";
	setAttr ".targets" -type "string" "brick_5_13";
createNode dynamicConstraint -n "glue_5_12_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_12";
	setAttr ".targets" -type "string" "brick_6_11";
createNode dynamicConstraint -n "glue_5_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_12";
	setAttr ".targets" -type "string" "brick_6_12";
createNode dynamicConstraint -n "glue_5_13_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_13";
	setAttr ".targets" -type "string" "brick_5_14";
createNode dynamicConstraint -n "glue_5_13_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_13";
	setAttr ".targets" -type "string" "brick_6_12";
createNode dynamicConstraint -n "glue_5_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_13";
	setAttr ".targets" -type "string" "brick_6_13";
createNode dynamicConstraint -n "glue_5_14_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_5_14";
	setAttr ".targets" -type "string" "brick_6_13";
createNode dynamicConstraint -n "glue_6_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_0";
	setAttr ".targets" -type "string" "brick_6_1";
createNode dynamicConstraint -n "glue_6_0_up0";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_0";
	setAttr ".targets" -type "string" "brick_7_0";
createNode dynamicConstraint -n "glue_6_0_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_0";
	setAttr ".targets" -type "string" "brick_7_1";
createNode dynamicConstraint -n "glue_6_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_1";
	setAttr ".targets" -type "string" "brick_6_2";
createNode dynamicConstraint -n "glue_6_1_up1";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_1";
	setAttr ".targets" -type "string" "brick_7_1";
createNode dynamicConstraint -n "glue_6_1_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_1";
	setAttr ".targets" -type "string" "brick_7_2";
createNode dynamicConstraint -n "glue_6_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_2";
	setAttr ".targets" -type "string" "brick_6_3";
createNode dynamicConstraint -n "glue_6_2_up2";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_2";
	setAttr ".targets" -type "string" "brick_7_2";
createNode dynamicConstraint -n "glue_6_2_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_2";
	setAttr ".targets" -type "string" "brick_7_3";
createNode dynamicConstraint -n "glue_6_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_3";
	setAttr ".targets" -type "string" "brick_6_4";
createNode dynamicConstraint -n "glue_6_3_up3";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_3";
	setAttr ".targets" -type "string" "brick_7_3";
createNode dynamicConstraint -n "glue_6_3_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_3";
	setAttr ".targets" -type "string" "brick_7_4";
createNode dynamicConstraint -n "glue_6_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_4";
	setAttr ".targets" -type "string" "brick_6_5";
createNode dynamicConstraint -n "glue_6_4_up4";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_4";
	setAttr ".targets" -type "string" "brick_7_4";
createNode dynamicConstraint -n "glue_6_4_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_4";
	setAttr ".targets" -type "string" "brick_7_5";
createNode dynamicConstraint -n "glue_6_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_5";
	setAttr ".targets" -type "string" "brick_6_6";
createNode dynamicConstraint -n "glue_6_5_up5";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_5";
	setAttr ".targets" -type "string" "brick_7_5";
createNode dynamicConstraint -n "glue_6_5_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_5";
	setAttr ".targets" -type "string" "brick_7_6";
createNode dynamicConstraint -n "glue_6_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_6";
	setAttr ".targets" -type "string" "brick_6_7";
createNode dynamicConstraint -n "glue_6_6_up6";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_6";
	setAttr ".targets" -type "string" "brick_7_6";
createNode dynamicConstraint -n "glue_6_6_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_6";
	setAttr ".targets" -type "string" "brick_7_7";
createNode dynamicConstraint -n "glue_6_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_7";
	setAttr ".targets" -type "string" "brick_6_8";
createNode dynamicConstraint -n "glue_6_7_up7";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_7";
	setAttr ".targets" -type "string" "brick_7_7";
createNode dynamicConstraint -n "glue_6_7_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_7";
	setAttr ".targets" -type "string" "brick_7_8";
createNode dynamicConstraint -n "glue_6_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_8";
	setAttr ".targets" -type "string" "brick_6_9";
createNode dynamicConstraint -n "glue_6_8_up8";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_8";
	setAttr ".targets" -type "string" "brick_7_8";
createNode dynamicConstraint -n "glue_6_8_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_8";
	setAttr ".targets" -type "string" "brick_7_9";
createNode dynamicConstraint -n "glue_6_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_9";
	setAttr ".targets" -type "string" "brick_6_10";
createNode dynamicConstraint -n "glue_6_9_up9";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_9";
	setAttr ".targets" -type "string" "brick_7_9";
createNode dynamicConstraint -n "glue_6_9_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_9";
	setAttr ".targets" -type "string" "brick_7_10";
createNode dynamicConstraint -n "glue_6_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_10";
	setAttr ".targets" -type "string" "brick_6_11";
createNode dynamicConstraint -n "glue_6_10_up10";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_10";
	setAttr ".targets" -type "string" "brick_7_10";
createNode dynamicConstraint -n "glue_6_10_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_10";
	setAttr ".targets" -type "string" "brick_7_11";
createNode dynamicConstraint -n "glue_6_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_11";
	setAttr ".targets" -type "string" "brick_6_12";
createNode dynamicConstraint -n "glue_6_11_up11";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_11";
	setAttr ".targets" -type "string" "brick_7_11";
createNode dynamicConstraint -n "glue_6_11_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_11";
	setAttr ".targets" -type "string" "brick_7_12";
createNode dynamicConstraint -n "glue_6_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_12";
	setAttr ".targets" -type "string" "brick_6_13";
createNode dynamicConstraint -n "glue_6_12_up12";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_12";
	setAttr ".targets" -type "string" "brick_7_12";
createNode dynamicConstraint -n "glue_6_12_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_12";
	setAttr ".targets" -type "string" "brick_7_13";
createNode dynamicConstraint -n "glue_6_13_up13";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_13";
	setAttr ".targets" -type "string" "brick_7_13";
createNode dynamicConstraint -n "glue_6_13_up14";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_6_13";
	setAttr ".targets" -type "string" "brick_7_14";
createNode dynamicConstraint -n "glue_7_0_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_0";
	setAttr ".targets" -type "string" "brick_7_1";
createNode dynamicConstraint -n "glue_7_1_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_1";
	setAttr ".targets" -type "string" "brick_7_2";
createNode dynamicConstraint -n "glue_7_2_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_2";
	setAttr ".targets" -type "string" "brick_7_3";
createNode dynamicConstraint -n "glue_7_3_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_3";
	setAttr ".targets" -type "string" "brick_7_4";
createNode dynamicConstraint -n "glue_7_4_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_4";
	setAttr ".targets" -type "string" "brick_7_5";
createNode dynamicConstraint -n "glue_7_5_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_5";
	setAttr ".targets" -type "string" "brick_7_6";
createNode dynamicConstraint -n "glue_7_6_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_6";
	setAttr ".targets" -type "string" "brick_7_7";
createNode dynamicConstraint -n "glue_7_7_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_7";
	setAttr ".targets" -type "string" "brick_7_8";
createNode dynamicConstraint -n "glue_7_8_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_8";
	setAttr ".targets" -type "string" "brick_7_9";
createNode dynamicConstraint -n "glue_7_9_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_9";
	setAttr ".targets" -type "string" "brick_7_10";
createNode dynamicConstraint -n "glue_7_10_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_10";
	setAttr ".targets" -type "string" "brick_7_11";
createNode dynamicConstraint -n "glue_7_11_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_11";
	setAttr ".targets" -type "string" "brick_7_12";
createNode dynamicConstraint -n "glue_7_12_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_12";
	setAttr ".targets" -type "string" "brick_7_13";
createNode dynamicConstraint -n "glue_7_13_side";
	setAttr ".constraintType" 4;
	setAttr ".glueStrength" 0.05;
	setAttr ".members" -type "string" "brick_7_13";
	setAttr ".targets" -type "string" "brick_7_14";
createNode rigidBody -n "rigidBody118";
	setAttr ".object" -type "string" "wreckingBall";
	setAttr ".mass" 3000;
	setAttr ".initialVelocity" -type "double3" 0 0 1500 ;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 -150 150 -900 1;
createNode rigidBody -n "rigidBody119";
	setAttr ".object" -type "string" "secondBall";
	setAttr ".mass" 2000;
	setAttr ".initialVelocity" -type "double3" 0 0 1800 ;
	setAttr ".initialMatrix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 900 380 -2600 1;
createNode blinn -n "wreckingBallMat";
	setAttr ".c" -type "float3" 0.15 0.15 0.18 ;
createNode shadingEngine -n "wreckingBallMatSG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode blinn -n "secondBallMat";
	setAttr ".c" -type "float3" 0.15 0.15 0.18 ;
createNode shadingEngine -n "secondBallMatSG";
	setAttr ".ihi" 0;
	setAttr ".ro" yes;
createNode aiOptions -s -n "defaultArnoldRenderOptions";
createNode aiAOVFilter -s -n "defaultArnoldFilter";
createNode aiAOVDriver -s -n "defaultArnoldDriver";
createNode aiAOVDriver -s -n "defaultArnoldDisplayDriver";
select -ne :lightList1;
	setAttr -s 2 ".l";
select -ne :defaultLightSet;
	setAttr -s 2 ".dsm";
createNode script -n "sceneConfigurationScriptNode";
	setAttr ".b" -type "string" "playbackOptions -min 1 -max 200 -ast 1 -aet 200 ";
	setAttr ".st" 6;
select -ne :time1;
	setAttr ".o" 1;
	setAttr ".unw" 1;
select -ne :hardwareRenderingGlobals;
	setAttr ".fprt" yes;
	setAttr ".rtfm" 1;
	setAttr ".otfna" -type "stringArray" 22 "NURBS Curves" "NURBS Surfaces" "Polygons" "Subdiv Surface" "Particles" "Particle Instance" "Fluids" "Strokes" "Image Planes" "UI" "Lights" "Cameras" "Locators" "Joints" "IK Handles" "Deformers" "Motion Trails" "Components" "Hair Systems" "Follicles" "Misc. UI" "Ornaments";
	setAttr ".otfva" -type "Int32Array" 22 0 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0;
select -ne :renderPartition;
	setAttr -s 121 ".st";
select -ne :renderGlobalsList1;
select -ne :defaultShaderList1;
	setAttr -s 125 ".s";
select -ne :postProcessList1;
	setAttr -s 2 ".p";
select -ne :defaultRenderingList1;
select -ne :standardSurface1;
	setAttr ".bc" -type "float3" 0.4 0.4 0.4 ;
	setAttr ".sr" 0.5;
select -ne :openPBR_shader1;
	setAttr ".bc" -type "float3" 0.4 0.4 0.4 ;
	setAttr ".sr" 0.5;
select -ne :initialShadingGroup;
	setAttr ".ro" yes;
select -ne :initialParticleSE;
	setAttr ".ro" yes;
select -ne :defaultRenderGlobals;
select -ne :defaultResolution;
select -ne :defaultColorMgtGlobals;
	setAttr ".cfe" yes;
	setAttr ".cfp" -type "string" "<MAYA_RESOURCES>/OCIO-configs/Maya2022-default/config.ocio";
	setAttr ".dn" -type "string" "sRGB";
	setAttr ".otn" -type "string" "ACES 1.0 SDR-video (sRGB)";
	setAttr ".potn" -type "string" "ACES 1.0 SDR-video (sRGB)";
	setAttr ".vn" -type "string" "ACES 1.0 SDR-video";
	setAttr ".vtn" -type "string" "ACES 1.0 SDR-video (sRGB)";
	setAttr ".wsn" -type "string" "ACEScg";
select -ne :hardwareRenderGlobals;
	setAttr ".ctrs" 256;
	setAttr ".btrs" 512;
relationship "link" ":lightLinker1" ":initialShadingGroup.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" ":initialParticleSE.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "floorMatSG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_0_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_1_14SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_2_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_3_14SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_4_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_5_14SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_6_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_0SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_1SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_2SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_3SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_4SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_5SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_6SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_7SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_8SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_9SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_10SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_11SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_12SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_13SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "brickMat_7_14SG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "wreckingBallMatSG.message" ":defaultLightSet.message";
relationship "link" ":lightLinker1" "secondBallMatSG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" ":initialShadingGroup.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" ":initialParticleSE.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "floorMatSG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_0_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_1_14SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_2_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_3_14SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_4_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_5_14SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_6_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_0SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_1SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_2SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_3SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_4SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_5SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_6SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_7SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_8SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_9SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_10SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_11SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_12SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_13SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "brickMat_7_14SG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "wreckingBallMatSG.message" ":defaultLightSet.message";
relationship "shadowLink" ":lightLinker1" "secondBallMatSG.message" ":defaultLightSet.message";
connectAttr "brickMat_0_0.oc" "brickMat_0_0SG.ss";
connectAttr "brickMat_0_1.oc" "brickMat_0_1SG.ss";
connectAttr "brickMat_0_10.oc" "brickMat_0_10SG.ss";
connectAttr "brickMat_0_11.oc" "brickMat_0_11SG.ss";
connectAttr "brickMat_0_12.oc" "brickMat_0_12SG.ss";
connectAttr "brickMat_0_13.oc" "brickMat_0_13SG.ss";
connectAttr "brickMat_0_2.oc" "brickMat_0_2SG.ss";
connectAttr "brickMat_0_3.oc" "brickMat_0_3SG.ss";
connectAttr "brickMat_0_4.oc" "brickMat_0_4SG.ss";
connectAttr "brickMat_0_5.oc" "brickMat_0_5SG.ss";
connectAttr "brickMat_0_6.oc" "brickMat_0_6SG.ss";
connectAttr "brickMat_0_7.oc" "brickMat_0_7SG.ss";
connectAttr "brickMat_0_8.oc" "brickMat_0_8SG.ss";
connectAttr "brickMat_0_9.oc" "brickMat_0_9SG.ss";
connectAttr "brickMat_1_0.oc" "brickMat_1_0SG.ss";
connectAttr "brickMat_1_1.oc" "brickMat_1_1SG.ss";
connectAttr "brickMat_1_10.oc" "brickMat_1_10SG.ss";
connectAttr "brickMat_1_11.oc" "brickMat_1_11SG.ss";
connectAttr "brickMat_1_12.oc" "brickMat_1_12SG.ss";
connectAttr "brickMat_1_13.oc" "brickMat_1_13SG.ss";
connectAttr "brickMat_1_14.oc" "brickMat_1_14SG.ss";
connectAttr "brickMat_1_2.oc" "brickMat_1_2SG.ss";
connectAttr "brickMat_1_3.oc" "brickMat_1_3SG.ss";
connectAttr "brickMat_1_4.oc" "brickMat_1_4SG.ss";
connectAttr "brickMat_1_5.oc" "brickMat_1_5SG.ss";
connectAttr "brickMat_1_6.oc" "brickMat_1_6SG.ss";
connectAttr "brickMat_1_7.oc" "brickMat_1_7SG.ss";
connectAttr "brickMat_1_8.oc" "brickMat_1_8SG.ss";
connectAttr "brickMat_1_9.oc" "brickMat_1_9SG.ss";
connectAttr "brickMat_2_0.oc" "brickMat_2_0SG.ss";
connectAttr "brickMat_2_1.oc" "brickMat_2_1SG.ss";
connectAttr "brickMat_2_10.oc" "brickMat_2_10SG.ss";
connectAttr "brickMat_2_11.oc" "brickMat_2_11SG.ss";
connectAttr "brickMat_2_12.oc" "brickMat_2_12SG.ss";
connectAttr "brickMat_2_13.oc" "brickMat_2_13SG.ss";
connectAttr "brickMat_2_2.oc" "brickMat_2_2SG.ss";
connectAttr "brickMat_2_3.oc" "brickMat_2_3SG.ss";
connectAttr "brickMat_2_4.oc" "brickMat_2_4SG.ss";
connectAttr "brickMat_2_5.oc" "brickMat_2_5SG.ss";
connectAttr "brickMat_2_6.oc" "brickMat_2_6SG.ss";
connectAttr "brickMat_2_7.oc" "brickMat_2_7SG.ss";
connectAttr "brickMat_2_8.oc" "brickMat_2_8SG.ss";
connectAttr "brickMat_2_9.oc" "brickMat_2_9SG.ss";
connectAttr "brickMat_3_0.oc" "brickMat_3_0SG.ss";
connectAttr "brickMat_3_1.oc" "brickMat_3_1SG.ss";
connectAttr "brickMat_3_10.oc" "brickMat_3_10SG.ss";
connectAttr "brickMat_3_11.oc" "brickMat_3_11SG.ss";
connectAttr "brickMat_3_12.oc" "brickMat_3_12SG.ss";
connectAttr "brickMat_3_13.oc" "brickMat_3_13SG.ss";
connectAttr "brickMat_3_14.oc" "brickMat_3_14SG.ss";
connectAttr "brickMat_3_2.oc" "brickMat_3_2SG.ss";
connectAttr "brickMat_3_3.oc" "brickMat_3_3SG.ss";
connectAttr "brickMat_3_4.oc" "brickMat_3_4SG.ss";
connectAttr "brickMat_3_5.oc" "brickMat_3_5SG.ss";
connectAttr "brickMat_3_6.oc" "brickMat_3_6SG.ss";
connectAttr "brickMat_3_7.oc" "brickMat_3_7SG.ss";
connectAttr "brickMat_3_8.oc" "brickMat_3_8SG.ss";
connectAttr "brickMat_3_9.oc" "brickMat_3_9SG.ss";
connectAttr "brickMat_4_0.oc" "brickMat_4_0SG.ss";
connectAttr "brickMat_4_1.oc" "brickMat_4_1SG.ss";
connectAttr "brickMat_4_10.oc" "brickMat_4_10SG.ss";
connectAttr "brickMat_4_11.oc" "brickMat_4_11SG.ss";
connectAttr "brickMat_4_12.oc" "brickMat_4_12SG.ss";
connectAttr "brickMat_4_13.oc" "brickMat_4_13SG.ss";
connectAttr "brickMat_4_2.oc" "brickMat_4_2SG.ss";
connectAttr "brickMat_4_3.oc" "brickMat_4_3SG.ss";
connectAttr "brickMat_4_4.oc" "brickMat_4_4SG.ss";
connectAttr "brickMat_4_5.oc" "brickMat_4_5SG.ss";
connectAttr "brickMat_4_6.oc" "brickMat_4_6SG.ss";
connectAttr "brickMat_4_7.oc" "brickMat_4_7SG.ss";
connectAttr "brickMat_4_8.oc" "brickMat_4_8SG.ss";
connectAttr "brickMat_4_9.oc" "brickMat_4_9SG.ss";
connectAttr "brickMat_5_0.oc" "brickMat_5_0SG.ss";
connectAttr "brickMat_5_1.oc" "brickMat_5_1SG.ss";
connectAttr "brickMat_5_10.oc" "brickMat_5_10SG.ss";
connectAttr "brickMat_5_11.oc" "brickMat_5_11SG.ss";
connectAttr "brickMat_5_12.oc" "brickMat_5_12SG.ss";
connectAttr "brickMat_5_13.oc" "brickMat_5_13SG.ss";
connectAttr "brickMat_5_14.oc" "brickMat_5_14SG.ss";
connectAttr "brickMat_5_2.oc" "brickMat_5_2SG.ss";
connectAttr "brickMat_5_3.oc" "brickMat_5_3SG.ss";
connectAttr "brickMat_5_4.oc" "brickMat_5_4SG.ss";
connectAttr "brickMat_5_5.oc" "brickMat_5_5SG.ss";
connectAttr "brickMat_5_6.oc" "brickMat_5_6SG.ss";
connectAttr "brickMat_5_7.oc" "brickMat_5_7SG.ss";
connectAttr "brickMat_5_8.oc" "brickMat_5_8SG.ss";
connectAttr "brickMat_5_9.oc" "brickMat_5_9SG.ss";
connectAttr "brickMat_6_0.oc" "brickMat_6_0SG.ss";
connectAttr "brickMat_6_1.oc" "brickMat_6_1SG.ss";
connectAttr "brickMat_6_10.oc" "brickMat_6_10SG.ss";
connectAttr "brickMat_6_11.oc" "brickMat_6_11SG.ss";
connectAttr "brickMat_6_12.oc" "brickMat_6_12SG.ss";
connectAttr "brickMat_6_13.oc" "brickMat_6_13SG.ss";
connectAttr "brickMat_6_2.oc" "brickMat_6_2SG.ss";
connectAttr "brickMat_6_3.oc" "brickMat_6_3SG.ss";
connectAttr "brickMat_6_4.oc" "brickMat_6_4SG.ss";
connectAttr "brickMat_6_5.oc" "brickMat_6_5SG.ss";
connectAttr "brickMat_6_6.oc" "brickMat_6_6SG.ss";
connectAttr "brickMat_6_7.oc" "brickMat_6_7SG.ss";
connectAttr "brickMat_6_8.oc" "brickMat_6_8SG.ss";
connectAttr "brickMat_6_9.oc" "brickMat_6_9SG.ss";
connectAttr "brickMat_7_0.oc" "brickMat_7_0SG.ss";
connectAttr "brickMat_7_1.oc" "brickMat_7_1SG.ss";
connectAttr "brickMat_7_10.oc" "brickMat_7_10SG.ss";
connectAttr "brickMat_7_11.oc" "brickMat_7_11SG.ss";
connectAttr "brickMat_7_12.oc" "brickMat_7_12SG.ss";
connectAttr "brickMat_7_13.oc" "brickMat_7_13SG.ss";
connectAttr "brickMat_7_14.oc" "brickMat_7_14SG.ss";
connectAttr "brickMat_7_2.oc" "brickMat_7_2SG.ss";
connectAttr "brickMat_7_3.oc" "brickMat_7_3SG.ss";
connectAttr "brickMat_7_4.oc" "brickMat_7_4SG.ss";
connectAttr "brickMat_7_5.oc" "brickMat_7_5SG.ss";
connectAttr "brickMat_7_6.oc" "brickMat_7_6SG.ss";
connectAttr "brickMat_7_7.oc" "brickMat_7_7SG.ss";
connectAttr "brickMat_7_8.oc" "brickMat_7_8SG.ss";
connectAttr "brickMat_7_9.oc" "brickMat_7_9SG.ss";
connectAttr "floorMat.oc" "floorMatSG.ss";
connectAttr "layerManager.dli[0]" "defaultLayer.id";
connectAttr "renderLayerManager.rlmi[0]" "defaultRenderLayer.rlid";
connectAttr "rigidBody10.outRotate" "brick_0_8.rotate";
connectAttr "rigidBody10.outTranslate" "brick_0_8.translate";
connectAttr "rigidBody100.outRotate" "brick_6_11.rotate";
connectAttr "rigidBody100.outTranslate" "brick_6_11.translate";
connectAttr "rigidBody101.outRotate" "brick_6_12.rotate";
connectAttr "rigidBody101.outTranslate" "brick_6_12.translate";
connectAttr "rigidBody102.outRotate" "brick_6_13.rotate";
connectAttr "rigidBody102.outTranslate" "brick_6_13.translate";
connectAttr "rigidBody103.outRotate" "brick_7_0.rotate";
connectAttr "rigidBody103.outTranslate" "brick_7_0.translate";
connectAttr "rigidBody104.outRotate" "brick_7_1.rotate";
connectAttr "rigidBody104.outTranslate" "brick_7_1.translate";
connectAttr "rigidBody105.outRotate" "brick_7_2.rotate";
connectAttr "rigidBody105.outTranslate" "brick_7_2.translate";
connectAttr "rigidBody106.outRotate" "brick_7_3.rotate";
connectAttr "rigidBody106.outTranslate" "brick_7_3.translate";
connectAttr "rigidBody107.outRotate" "brick_7_4.rotate";
connectAttr "rigidBody107.outTranslate" "brick_7_4.translate";
connectAttr "rigidBody108.outRotate" "brick_7_5.rotate";
connectAttr "rigidBody108.outTranslate" "brick_7_5.translate";
connectAttr "rigidBody109.outRotate" "brick_7_6.rotate";
connectAttr "rigidBody109.outTranslate" "brick_7_6.translate";
connectAttr "rigidBody11.outRotate" "brick_0_9.rotate";
connectAttr "rigidBody11.outTranslate" "brick_0_9.translate";
connectAttr "rigidBody110.outRotate" "brick_7_7.rotate";
connectAttr "rigidBody110.outTranslate" "brick_7_7.translate";
connectAttr "rigidBody111.outRotate" "brick_7_8.rotate";
connectAttr "rigidBody111.outTranslate" "brick_7_8.translate";
connectAttr "rigidBody112.outRotate" "brick_7_9.rotate";
connectAttr "rigidBody112.outTranslate" "brick_7_9.translate";
connectAttr "rigidBody113.outRotate" "brick_7_10.rotate";
connectAttr "rigidBody113.outTranslate" "brick_7_10.translate";
connectAttr "rigidBody114.outRotate" "brick_7_11.rotate";
connectAttr "rigidBody114.outTranslate" "brick_7_11.translate";
connectAttr "rigidBody115.outRotate" "brick_7_12.rotate";
connectAttr "rigidBody115.outTranslate" "brick_7_12.translate";
connectAttr "rigidBody116.outRotate" "brick_7_13.rotate";
connectAttr "rigidBody116.outTranslate" "brick_7_13.translate";
connectAttr "rigidBody117.outRotate" "brick_7_14.rotate";
connectAttr "rigidBody117.outTranslate" "brick_7_14.translate";
connectAttr "rigidBody118.outRotate" "wreckingBall.rotate";
connectAttr "rigidBody118.outTranslate" "wreckingBall.translate";
connectAttr "rigidBody119.outRotate" "secondBall.rotate";
connectAttr "rigidBody119.outTranslate" "secondBall.translate";
connectAttr "rigidBody12.outRotate" "brick_0_10.rotate";
connectAttr "rigidBody12.outTranslate" "brick_0_10.translate";
connectAttr "rigidBody13.outRotate" "brick_0_11.rotate";
connectAttr "rigidBody13.outTranslate" "brick_0_11.translate";
connectAttr "rigidBody14.outRotate" "brick_0_12.rotate";
connectAttr "rigidBody14.outTranslate" "brick_0_12.translate";
connectAttr "rigidBody15.outRotate" "brick_0_13.rotate";
connectAttr "rigidBody15.outTranslate" "brick_0_13.translate";
connectAttr "rigidBody16.outRotate" "brick_1_0.rotate";
connectAttr "rigidBody16.outTranslate" "brick_1_0.translate";
connectAttr "rigidBody17.outRotate" "brick_1_1.rotate";
connectAttr "rigidBody17.outTranslate" "brick_1_1.translate";
connectAttr "rigidBody18.outRotate" "brick_1_2.rotate";
connectAttr "rigidBody18.outTranslate" "brick_1_2.translate";
connectAttr "rigidBody19.outRotate" "brick_1_3.rotate";
connectAttr "rigidBody19.outTranslate" "brick_1_3.translate";
connectAttr "rigidBody2.outRotate" "brick_0_0.rotate";
connectAttr "rigidBody2.outTranslate" "brick_0_0.translate";
connectAttr "rigidBody20.outRotate" "brick_1_4.rotate";
connectAttr "rigidBody20.outTranslate" "brick_1_4.translate";
connectAttr "rigidBody21.outRotate" "brick_1_5.rotate";
connectAttr "rigidBody21.outTranslate" "brick_1_5.translate";
connectAttr "rigidBody22.outRotate" "brick_1_6.rotate";
connectAttr "rigidBody22.outTranslate" "brick_1_6.translate";
connectAttr "rigidBody23.outRotate" "brick_1_7.rotate";
connectAttr "rigidBody23.outTranslate" "brick_1_7.translate";
connectAttr "rigidBody24.outRotate" "brick_1_8.rotate";
connectAttr "rigidBody24.outTranslate" "brick_1_8.translate";
connectAttr "rigidBody25.outRotate" "brick_1_9.rotate";
connectAttr "rigidBody25.outTranslate" "brick_1_9.translate";
connectAttr "rigidBody26.outRotate" "brick_1_10.rotate";
connectAttr "rigidBody26.outTranslate" "brick_1_10.translate";
connectAttr "rigidBody27.outRotate" "brick_1_11.rotate";
connectAttr "rigidBody27.outTranslate" "brick_1_11.translate";
connectAttr "rigidBody28.outRotate" "brick_1_12.rotate";
connectAttr "rigidBody28.outTranslate" "brick_1_12.translate";
connectAttr "rigidBody29.outRotate" "brick_1_13.rotate";
connectAttr "rigidBody29.outTranslate" "brick_1_13.translate";
connectAttr "rigidBody3.outRotate" "brick_0_1.rotate";
connectAttr "rigidBody3.outTranslate" "brick_0_1.translate";
connectAttr "rigidBody30.outRotate" "brick_1_14.rotate";
connectAttr "rigidBody30.outTranslate" "brick_1_14.translate";
connectAttr "rigidBody31.outRotate" "brick_2_0.rotate";
connectAttr "rigidBody31.outTranslate" "brick_2_0.translate";
connectAttr "rigidBody32.outRotate" "brick_2_1.rotate";
connectAttr "rigidBody32.outTranslate" "brick_2_1.translate";
connectAttr "rigidBody33.outRotate" "brick_2_2.rotate";
connectAttr "rigidBody33.outTranslate" "brick_2_2.translate";
connectAttr "rigidBody34.outRotate" "brick_2_3.rotate";
connectAttr "rigidBody34.outTranslate" "brick_2_3.translate";
connectAttr "rigidBody35.outRotate" "brick_2_4.rotate";
connectAttr "rigidBody35.outTranslate" "brick_2_4.translate";
connectAttr "rigidBody36.outRotate" "brick_2_5.rotate";
connectAttr "rigidBody36.outTranslate" "brick_2_5.translate";
connectAttr "rigidBody37.outRotate" "brick_2_6.rotate";
connectAttr "rigidBody37.outTranslate" "brick_2_6.translate";
connectAttr "rigidBody38.outRotate" "brick_2_7.rotate";
connectAttr "rigidBody38.outTranslate" "brick_2_7.translate";
connectAttr "rigidBody39.outRotate" "brick_2_8.rotate";
connectAttr "rigidBody39.outTranslate" "brick_2_8.translate";
connectAttr "rigidBody4.outRotate" "brick_0_2.rotate";
connectAttr "rigidBody4.outTranslate" "brick_0_2.translate";
connectAttr "rigidBody40.outRotate" "brick_2_9.rotate";
connectAttr "rigidBody40.outTranslate" "brick_2_9.translate";
connectAttr "rigidBody41.outRotate" "brick_2_10.rotate";
connectAttr "rigidBody41.outTranslate" "brick_2_10.translate";
connectAttr "rigidBody42.outRotate" "brick_2_11.rotate";
connectAttr "rigidBody42.outTranslate" "brick_2_11.translate";
connectAttr "rigidBody43.outRotate" "brick_2_12.rotate";
connectAttr "rigidBody43.outTranslate" "brick_2_12.translate";
connectAttr "rigidBody44.outRotate" "brick_2_13.rotate";
connectAttr "rigidBody44.outTranslate" "brick_2_13.translate";
connectAttr "rigidBody45.outRotate" "brick_3_0.rotate";
connectAttr "rigidBody45.outTranslate" "brick_3_0.translate";
connectAttr "rigidBody46.outRotate" "brick_3_1.rotate";
connectAttr "rigidBody46.outTranslate" "brick_3_1.translate";
connectAttr "rigidBody47.outRotate" "brick_3_2.rotate";
connectAttr "rigidBody47.outTranslate" "brick_3_2.translate";
connectAttr "rigidBody48.outRotate" "brick_3_3.rotate";
connectAttr "rigidBody48.outTranslate" "brick_3_3.translate";
connectAttr "rigidBody49.outRotate" "brick_3_4.rotate";
connectAttr "rigidBody49.outTranslate" "brick_3_4.translate";
connectAttr "rigidBody5.outRotate" "brick_0_3.rotate";
connectAttr "rigidBody5.outTranslate" "brick_0_3.translate";
connectAttr "rigidBody50.outRotate" "brick_3_5.rotate";
connectAttr "rigidBody50.outTranslate" "brick_3_5.translate";
connectAttr "rigidBody51.outRotate" "brick_3_6.rotate";
connectAttr "rigidBody51.outTranslate" "brick_3_6.translate";
connectAttr "rigidBody52.outRotate" "brick_3_7.rotate";
connectAttr "rigidBody52.outTranslate" "brick_3_7.translate";
connectAttr "rigidBody53.outRotate" "brick_3_8.rotate";
connectAttr "rigidBody53.outTranslate" "brick_3_8.translate";
connectAttr "rigidBody54.outRotate" "brick_3_9.rotate";
connectAttr "rigidBody54.outTranslate" "brick_3_9.translate";
connectAttr "rigidBody55.outRotate" "brick_3_10.rotate";
connectAttr "rigidBody55.outTranslate" "brick_3_10.translate";
connectAttr "rigidBody56.outRotate" "brick_3_11.rotate";
connectAttr "rigidBody56.outTranslate" "brick_3_11.translate";
connectAttr "rigidBody57.outRotate" "brick_3_12.rotate";
connectAttr "rigidBody57.outTranslate" "brick_3_12.translate";
connectAttr "rigidBody58.outRotate" "brick_3_13.rotate";
connectAttr "rigidBody58.outTranslate" "brick_3_13.translate";
connectAttr "rigidBody59.outRotate" "brick_3_14.rotate";
connectAttr "rigidBody59.outTranslate" "brick_3_14.translate";
connectAttr "rigidBody6.outRotate" "brick_0_4.rotate";
connectAttr "rigidBody6.outTranslate" "brick_0_4.translate";
connectAttr "rigidBody60.outRotate" "brick_4_0.rotate";
connectAttr "rigidBody60.outTranslate" "brick_4_0.translate";
connectAttr "rigidBody61.outRotate" "brick_4_1.rotate";
connectAttr "rigidBody61.outTranslate" "brick_4_1.translate";
connectAttr "rigidBody62.outRotate" "brick_4_2.rotate";
connectAttr "rigidBody62.outTranslate" "brick_4_2.translate";
connectAttr "rigidBody63.outRotate" "brick_4_3.rotate";
connectAttr "rigidBody63.outTranslate" "brick_4_3.translate";
connectAttr "rigidBody64.outRotate" "brick_4_4.rotate";
connectAttr "rigidBody64.outTranslate" "brick_4_4.translate";
connectAttr "rigidBody65.outRotate" "brick_4_5.rotate";
connectAttr "rigidBody65.outTranslate" "brick_4_5.translate";
connectAttr "rigidBody66.outRotate" "brick_4_6.rotate";
connectAttr "rigidBody66.outTranslate" "brick_4_6.translate";
connectAttr "rigidBody67.outRotate" "brick_4_7.rotate";
connectAttr "rigidBody67.outTranslate" "brick_4_7.translate";
connectAttr "rigidBody68.outRotate" "brick_4_8.rotate";
connectAttr "rigidBody68.outTranslate" "brick_4_8.translate";
connectAttr "rigidBody69.outRotate" "brick_4_9.rotate";
connectAttr "rigidBody69.outTranslate" "brick_4_9.translate";
connectAttr "rigidBody7.outRotate" "brick_0_5.rotate";
connectAttr "rigidBody7.outTranslate" "brick_0_5.translate";
connectAttr "rigidBody70.outRotate" "brick_4_10.rotate";
connectAttr "rigidBody70.outTranslate" "brick_4_10.translate";
connectAttr "rigidBody71.outRotate" "brick_4_11.rotate";
connectAttr "rigidBody71.outTranslate" "brick_4_11.translate";
connectAttr "rigidBody72.outRotate" "brick_4_12.rotate";
connectAttr "rigidBody72.outTranslate" "brick_4_12.translate";
connectAttr "rigidBody73.outRotate" "brick_4_13.rotate";
connectAttr "rigidBody73.outTranslate" "brick_4_13.translate";
connectAttr "rigidBody74.outRotate" "brick_5_0.rotate";
connectAttr "rigidBody74.outTranslate" "brick_5_0.translate";
connectAttr "rigidBody75.outRotate" "brick_5_1.rotate";
connectAttr "rigidBody75.outTranslate" "brick_5_1.translate";
connectAttr "rigidBody76.outRotate" "brick_5_2.rotate";
connectAttr "rigidBody76.outTranslate" "brick_5_2.translate";
connectAttr "rigidBody77.outRotate" "brick_5_3.rotate";
connectAttr "rigidBody77.outTranslate" "brick_5_3.translate";
connectAttr "rigidBody78.outRotate" "brick_5_4.rotate";
connectAttr "rigidBody78.outTranslate" "brick_5_4.translate";
connectAttr "rigidBody79.outRotate" "brick_5_5.rotate";
connectAttr "rigidBody79.outTranslate" "brick_5_5.translate";
connectAttr "rigidBody8.outRotate" "brick_0_6.rotate";
connectAttr "rigidBody8.outTranslate" "brick_0_6.translate";
connectAttr "rigidBody80.outRotate" "brick_5_6.rotate";
connectAttr "rigidBody80.outTranslate" "brick_5_6.translate";
connectAttr "rigidBody81.outRotate" "brick_5_7.rotate";
connectAttr "rigidBody81.outTranslate" "brick_5_7.translate";
connectAttr "rigidBody82.outRotate" "brick_5_8.rotate";
connectAttr "rigidBody82.outTranslate" "brick_5_8.translate";
connectAttr "rigidBody83.outRotate" "brick_5_9.rotate";
connectAttr "rigidBody83.outTranslate" "brick_5_9.translate";
connectAttr "rigidBody84.outRotate" "brick_5_10.rotate";
connectAttr "rigidBody84.outTranslate" "brick_5_10.translate";
connectAttr "rigidBody85.outRotate" "brick_5_11.rotate";
connectAttr "rigidBody85.outTranslate" "brick_5_11.translate";
connectAttr "rigidBody86.outRotate" "brick_5_12.rotate";
connectAttr "rigidBody86.outTranslate" "brick_5_12.translate";
connectAttr "rigidBody87.outRotate" "brick_5_13.rotate";
connectAttr "rigidBody87.outTranslate" "brick_5_13.translate";
connectAttr "rigidBody88.outRotate" "brick_5_14.rotate";
connectAttr "rigidBody88.outTranslate" "brick_5_14.translate";
connectAttr "rigidBody89.outRotate" "brick_6_0.rotate";
connectAttr "rigidBody89.outTranslate" "brick_6_0.translate";
connectAttr "rigidBody9.outRotate" "brick_0_7.rotate";
connectAttr "rigidBody9.outTranslate" "brick_0_7.translate";
connectAttr "rigidBody90.outRotate" "brick_6_1.rotate";
connectAttr "rigidBody90.outTranslate" "brick_6_1.translate";
connectAttr "rigidBody91.outRotate" "brick_6_2.rotate";
connectAttr "rigidBody91.outTranslate" "brick_6_2.translate";
connectAttr "rigidBody92.outRotate" "brick_6_3.rotate";
connectAttr "rigidBody92.outTranslate" "brick_6_3.translate";
connectAttr "rigidBody93.outRotate" "brick_6_4.rotate";
connectAttr "rigidBody93.outTranslate" "brick_6_4.translate";
connectAttr "rigidBody94.outRotate" "brick_6_5.rotate";
connectAttr "rigidBody94.outTranslate" "brick_6_5.translate";
connectAttr "rigidBody95.outRotate" "brick_6_6.rotate";
connectAttr "rigidBody95.outTranslate" "brick_6_6.translate";
connectAttr "rigidBody96.outRotate" "brick_6_7.rotate";
connectAttr "rigidBody96.outTranslate" "brick_6_7.translate";
connectAttr "rigidBody97.outRotate" "brick_6_8.rotate";
connectAttr "rigidBody97.outTranslate" "brick_6_8.translate";
connectAttr "rigidBody98.outRotate" "brick_6_9.rotate";
connectAttr "rigidBody98.outTranslate" "brick_6_9.translate";
connectAttr "rigidBody99.outRotate" "brick_6_10.rotate";
connectAttr "rigidBody99.outTranslate" "brick_6_10.translate";
connectAttr "secondBallMat.oc" "secondBallMatSG.ss";
connectAttr "solverShape1.message" "glue_0_0_side.solver";
connectAttr "solverShape1.message" "glue_0_0_up0.solver";
connectAttr "solverShape1.message" "glue_0_0_up1.solver";
connectAttr "solverShape1.message" "glue_0_10_side.solver";
connectAttr "solverShape1.message" "glue_0_10_up10.solver";
connectAttr "solverShape1.message" "glue_0_10_up11.solver";
connectAttr "solverShape1.message" "glue_0_11_side.solver";
connectAttr "solverShape1.message" "glue_0_11_up11.solver";
connectAttr "solverShape1.message" "glue_0_11_up12.solver";
connectAttr "solverShape1.message" "glue_0_12_side.solver";
connectAttr "solverShape1.message" "glue_0_12_up12.solver";
connectAttr "solverShape1.message" "glue_0_12_up13.solver";
connectAttr "solverShape1.message" "glue_0_13_up13.solver";
connectAttr "solverShape1.message" "glue_0_13_up14.solver";
connectAttr "solverShape1.message" "glue_0_1_side.solver";
connectAttr "solverShape1.message" "glue_0_1_up1.solver";
connectAttr "solverShape1.message" "glue_0_1_up2.solver";
connectAttr "solverShape1.message" "glue_0_2_side.solver";
connectAttr "solverShape1.message" "glue_0_2_up2.solver";
connectAttr "solverShape1.message" "glue_0_2_up3.solver";
connectAttr "solverShape1.message" "glue_0_3_side.solver";
connectAttr "solverShape1.message" "glue_0_3_up3.solver";
connectAttr "solverShape1.message" "glue_0_3_up4.solver";
connectAttr "solverShape1.message" "glue_0_4_side.solver";
connectAttr "solverShape1.message" "glue_0_4_up4.solver";
connectAttr "solverShape1.message" "glue_0_4_up5.solver";
connectAttr "solverShape1.message" "glue_0_5_side.solver";
connectAttr "solverShape1.message" "glue_0_5_up5.solver";
connectAttr "solverShape1.message" "glue_0_5_up6.solver";
connectAttr "solverShape1.message" "glue_0_6_side.solver";
connectAttr "solverShape1.message" "glue_0_6_up6.solver";
connectAttr "solverShape1.message" "glue_0_6_up7.solver";
connectAttr "solverShape1.message" "glue_0_7_side.solver";
connectAttr "solverShape1.message" "glue_0_7_up7.solver";
connectAttr "solverShape1.message" "glue_0_7_up8.solver";
connectAttr "solverShape1.message" "glue_0_8_side.solver";
connectAttr "solverShape1.message" "glue_0_8_up8.solver";
connectAttr "solverShape1.message" "glue_0_8_up9.solver";
connectAttr "solverShape1.message" "glue_0_9_side.solver";
connectAttr "solverShape1.message" "glue_0_9_up10.solver";
connectAttr "solverShape1.message" "glue_0_9_up9.solver";
connectAttr "solverShape1.message" "glue_1_0_side.solver";
connectAttr "solverShape1.message" "glue_1_0_up0.solver";
connectAttr "solverShape1.message" "glue_1_10_side.solver";
connectAttr "solverShape1.message" "glue_1_10_up10.solver";
connectAttr "solverShape1.message" "glue_1_10_up9.solver";
connectAttr "solverShape1.message" "glue_1_11_side.solver";
connectAttr "solverShape1.message" "glue_1_11_up10.solver";
connectAttr "solverShape1.message" "glue_1_11_up11.solver";
connectAttr "solverShape1.message" "glue_1_12_side.solver";
connectAttr "solverShape1.message" "glue_1_12_up11.solver";
connectAttr "solverShape1.message" "glue_1_12_up12.solver";
connectAttr "solverShape1.message" "glue_1_13_side.solver";
connectAttr "solverShape1.message" "glue_1_13_up12.solver";
connectAttr "solverShape1.message" "glue_1_13_up13.solver";
connectAttr "solverShape1.message" "glue_1_14_up13.solver";
connectAttr "solverShape1.message" "glue_1_1_side.solver";
connectAttr "solverShape1.message" "glue_1_1_up0.solver";
connectAttr "solverShape1.message" "glue_1_1_up1.solver";
connectAttr "solverShape1.message" "glue_1_2_side.solver";
connectAttr "solverShape1.message" "glue_1_2_up1.solver";
connectAttr "solverShape1.message" "glue_1_2_up2.solver";
connectAttr "solverShape1.message" "glue_1_3_side.solver";
connectAttr "solverShape1.message" "glue_1_3_up2.solver";
connectAttr "solverShape1.message" "glue_1_3_up3.solver";
connectAttr "solverShape1.message" "glue_1_4_side.solver";
connectAttr "solverShape1.message" "glue_1_4_up3.solver";
connectAttr "solverShape1.message" "glue_1_4_up4.solver";
connectAttr "solverShape1.message" "glue_1_5_side.solver";
connectAttr "solverShape1.message" "glue_1_5_up4.solver";
connectAttr "solverShape1.message" "glue_1_5_up5.solver";
connectAttr "solverShape1.message" "glue_1_6_side.solver";
connectAttr "solverShape1.message" "glue_1_6_up5.solver";
connectAttr "solverShape1.message" "glue_1_6_up6.solver";
connectAttr "solverShape1.message" "glue_1_7_side.solver";
connectAttr "solverShape1.message" "glue_1_7_up6.solver";
connectAttr "solverShape1.message" "glue_1_7_up7.solver";
connectAttr "solverShape1.message" "glue_1_8_side.solver";
connectAttr "solverShape1.message" "glue_1_8_up7.solver";
connectAttr "solverShape1.message" "glue_1_8_up8.solver";
connectAttr "solverShape1.message" "glue_1_9_side.solver";
connectAttr "solverShape1.message" "glue_1_9_up8.solver";
connectAttr "solverShape1.message" "glue_1_9_up9.solver";
connectAttr "solverShape1.message" "glue_2_0_side.solver";
connectAttr "solverShape1.message" "glue_2_0_up0.solver";
connectAttr "solverShape1.message" "glue_2_0_up1.solver";
connectAttr "solverShape1.message" "glue_2_10_side.solver";
connectAttr "solverShape1.message" "glue_2_10_up10.solver";
connectAttr "solverShape1.message" "glue_2_10_up11.solver";
connectAttr "solverShape1.message" "glue_2_11_side.solver";
connectAttr "solverShape1.message" "glue_2_11_up11.solver";
connectAttr "solverShape1.message" "glue_2_11_up12.solver";
connectAttr "solverShape1.message" "glue_2_12_side.solver";
connectAttr "solverShape1.message" "glue_2_12_up12.solver";
connectAttr "solverShape1.message" "glue_2_12_up13.solver";
connectAttr "solverShape1.message" "glue_2_13_up13.solver";
connectAttr "solverShape1.message" "glue_2_13_up14.solver";
connectAttr "solverShape1.message" "glue_2_1_side.solver";
connectAttr "solverShape1.message" "glue_2_1_up1.solver";
connectAttr "solverShape1.message" "glue_2_1_up2.solver";
connectAttr "solverShape1.message" "glue_2_2_side.solver";
connectAttr "solverShape1.message" "glue_2_2_up2.solver";
connectAttr "solverShape1.message" "glue_2_2_up3.solver";
connectAttr "solverShape1.message" "glue_2_3_side.solver";
connectAttr "solverShape1.message" "glue_2_3_up3.solver";
connectAttr "solverShape1.message" "glue_2_3_up4.solver";
connectAttr "solverShape1.message" "glue_2_4_side.solver";
connectAttr "solverShape1.message" "glue_2_4_up4.solver";
connectAttr "solverShape1.message" "glue_2_4_up5.solver";
connectAttr "solverShape1.message" "glue_2_5_side.solver";
connectAttr "solverShape1.message" "glue_2_5_up5.solver";
connectAttr "solverShape1.message" "glue_2_5_up6.solver";
connectAttr "solverShape1.message" "glue_2_6_side.solver";
connectAttr "solverShape1.message" "glue_2_6_up6.solver";
connectAttr "solverShape1.message" "glue_2_6_up7.solver";
connectAttr "solverShape1.message" "glue_2_7_side.solver";
connectAttr "solverShape1.message" "glue_2_7_up7.solver";
connectAttr "solverShape1.message" "glue_2_7_up8.solver";
connectAttr "solverShape1.message" "glue_2_8_side.solver";
connectAttr "solverShape1.message" "glue_2_8_up8.solver";
connectAttr "solverShape1.message" "glue_2_8_up9.solver";
connectAttr "solverShape1.message" "glue_2_9_side.solver";
connectAttr "solverShape1.message" "glue_2_9_up10.solver";
connectAttr "solverShape1.message" "glue_2_9_up9.solver";
connectAttr "solverShape1.message" "glue_3_0_side.solver";
connectAttr "solverShape1.message" "glue_3_0_up0.solver";
connectAttr "solverShape1.message" "glue_3_10_side.solver";
connectAttr "solverShape1.message" "glue_3_10_up10.solver";
connectAttr "solverShape1.message" "glue_3_10_up9.solver";
connectAttr "solverShape1.message" "glue_3_11_side.solver";
connectAttr "solverShape1.message" "glue_3_11_up10.solver";
connectAttr "solverShape1.message" "glue_3_11_up11.solver";
connectAttr "solverShape1.message" "glue_3_12_side.solver";
connectAttr "solverShape1.message" "glue_3_12_up11.solver";
connectAttr "solverShape1.message" "glue_3_12_up12.solver";
connectAttr "solverShape1.message" "glue_3_13_side.solver";
connectAttr "solverShape1.message" "glue_3_13_up12.solver";
connectAttr "solverShape1.message" "glue_3_13_up13.solver";
connectAttr "solverShape1.message" "glue_3_14_up13.solver";
connectAttr "solverShape1.message" "glue_3_1_side.solver";
connectAttr "solverShape1.message" "glue_3_1_up0.solver";
connectAttr "solverShape1.message" "glue_3_1_up1.solver";
connectAttr "solverShape1.message" "glue_3_2_side.solver";
connectAttr "solverShape1.message" "glue_3_2_up1.solver";
connectAttr "solverShape1.message" "glue_3_2_up2.solver";
connectAttr "solverShape1.message" "glue_3_3_side.solver";
connectAttr "solverShape1.message" "glue_3_3_up2.solver";
connectAttr "solverShape1.message" "glue_3_3_up3.solver";
connectAttr "solverShape1.message" "glue_3_4_side.solver";
connectAttr "solverShape1.message" "glue_3_4_up3.solver";
connectAttr "solverShape1.message" "glue_3_4_up4.solver";
connectAttr "solverShape1.message" "glue_3_5_side.solver";
connectAttr "solverShape1.message" "glue_3_5_up4.solver";
connectAttr "solverShape1.message" "glue_3_5_up5.solver";
connectAttr "solverShape1.message" "glue_3_6_side.solver";
connectAttr "solverShape1.message" "glue_3_6_up5.solver";
connectAttr "solverShape1.message" "glue_3_6_up6.solver";
connectAttr "solverShape1.message" "glue_3_7_side.solver";
connectAttr "solverShape1.message" "glue_3_7_up6.solver";
connectAttr "solverShape1.message" "glue_3_7_up7.solver";
connectAttr "solverShape1.message" "glue_3_8_side.solver";
connectAttr "solverShape1.message" "glue_3_8_up7.solver";
connectAttr "solverShape1.message" "glue_3_8_up8.solver";
connectAttr "solverShape1.message" "glue_3_9_side.solver";
connectAttr "solverShape1.message" "glue_3_9_up8.solver";
connectAttr "solverShape1.message" "glue_3_9_up9.solver";
connectAttr "solverShape1.message" "glue_4_0_side.solver";
connectAttr "solverShape1.message" "glue_4_0_up0.solver";
connectAttr "solverShape1.message" "glue_4_0_up1.solver";
connectAttr "solverShape1.message" "glue_4_10_side.solver";
connectAttr "solverShape1.message" "glue_4_10_up10.solver";
connectAttr "solverShape1.message" "glue_4_10_up11.solver";
connectAttr "solverShape1.message" "glue_4_11_side.solver";
connectAttr "solverShape1.message" "glue_4_11_up11.solver";
connectAttr "solverShape1.message" "glue_4_11_up12.solver";
connectAttr "solverShape1.message" "glue_4_12_side.solver";
connectAttr "solverShape1.message" "glue_4_12_up12.solver";
connectAttr "solverShape1.message" "glue_4_12_up13.solver";
connectAttr "solverShape1.message" "glue_4_13_up13.solver";
connectAttr "solverShape1.message" "glue_4_13_up14.solver";
connectAttr "solverShape1.message" "glue_4_1_side.solver";
connectAttr "solverShape1.message" "glue_4_1_up1.solver";
connectAttr "solverShape1.message" "glue_4_1_up2.solver";
connectAttr "solverShape1.message" "glue_4_2_side.solver";
connectAttr "solverShape1.message" "glue_4_2_up2.solver";
connectAttr "solverShape1.message" "glue_4_2_up3.solver";
connectAttr "solverShape1.message" "glue_4_3_side.solver";
connectAttr "solverShape1.message" "glue_4_3_up3.solver";
connectAttr "solverShape1.message" "glue_4_3_up4.solver";
connectAttr "solverShape1.message" "glue_4_4_side.solver";
connectAttr "solverShape1.message" "glue_4_4_up4.solver";
connectAttr "solverShape1.message" "glue_4_4_up5.solver";
connectAttr "solverShape1.message" "glue_4_5_side.solver";
connectAttr "solverShape1.message" "glue_4_5_up5.solver";
connectAttr "solverShape1.message" "glue_4_5_up6.solver";
connectAttr "solverShape1.message" "glue_4_6_side.solver";
connectAttr "solverShape1.message" "glue_4_6_up6.solver";
connectAttr "solverShape1.message" "glue_4_6_up7.solver";
connectAttr "solverShape1.message" "glue_4_7_side.solver";
connectAttr "solverShape1.message" "glue_4_7_up7.solver";
connectAttr "solverShape1.message" "glue_4_7_up8.solver";
connectAttr "solverShape1.message" "glue_4_8_side.solver";
connectAttr "solverShape1.message" "glue_4_8_up8.solver";
connectAttr "solverShape1.message" "glue_4_8_up9.solver";
connectAttr "solverShape1.message" "glue_4_9_side.solver";
connectAttr "solverShape1.message" "glue_4_9_up10.solver";
connectAttr "solverShape1.message" "glue_4_9_up9.solver";
connectAttr "solverShape1.message" "glue_5_0_side.solver";
connectAttr "solverShape1.message" "glue_5_0_up0.solver";
connectAttr "solverShape1.message" "glue_5_10_side.solver";
connectAttr "solverShape1.message" "glue_5_10_up10.solver";
connectAttr "solverShape1.message" "glue_5_10_up9.solver";
connectAttr "solverShape1.message" "glue_5_11_side.solver";
connectAttr "solverShape1.message" "glue_5_11_up10.solver";
connectAttr "solverShape1.message" "glue_5_11_up11.solver";
connectAttr "solverShape1.message" "glue_5_12_side.solver";
connectAttr "solverShape1.message" "glue_5_12_up11.solver";
connectAttr "solverShape1.message" "glue_5_12_up12.solver";
connectAttr "solverShape1.message" "glue_5_13_side.solver";
connectAttr "solverShape1.message" "glue_5_13_up12.solver";
connectAttr "solverShape1.message" "glue_5_13_up13.solver";
connectAttr "solverShape1.message" "glue_5_14_up13.solver";
connectAttr "solverShape1.message" "glue_5_1_side.solver";
connectAttr "solverShape1.message" "glue_5_1_up0.solver";
connectAttr "solverShape1.message" "glue_5_1_up1.solver";
connectAttr "solverShape1.message" "glue_5_2_side.solver";
connectAttr "solverShape1.message" "glue_5_2_up1.solver";
connectAttr "solverShape1.message" "glue_5_2_up2.solver";
connectAttr "solverShape1.message" "glue_5_3_side.solver";
connectAttr "solverShape1.message" "glue_5_3_up2.solver";
connectAttr "solverShape1.message" "glue_5_3_up3.solver";
connectAttr "solverShape1.message" "glue_5_4_side.solver";
connectAttr "solverShape1.message" "glue_5_4_up3.solver";
connectAttr "solverShape1.message" "glue_5_4_up4.solver";
connectAttr "solverShape1.message" "glue_5_5_side.solver";
connectAttr "solverShape1.message" "glue_5_5_up4.solver";
connectAttr "solverShape1.message" "glue_5_5_up5.solver";
connectAttr "solverShape1.message" "glue_5_6_side.solver";
connectAttr "solverShape1.message" "glue_5_6_up5.solver";
connectAttr "solverShape1.message" "glue_5_6_up6.solver";
connectAttr "solverShape1.message" "glue_5_7_side.solver";
connectAttr "solverShape1.message" "glue_5_7_up6.solver";
connectAttr "solverShape1.message" "glue_5_7_up7.solver";
connectAttr "solverShape1.message" "glue_5_8_side.solver";
connectAttr "solverShape1.message" "glue_5_8_up7.solver";
connectAttr "solverShape1.message" "glue_5_8_up8.solver";
connectAttr "solverShape1.message" "glue_5_9_side.solver";
connectAttr "solverShape1.message" "glue_5_9_up8.solver";
connectAttr "solverShape1.message" "glue_5_9_up9.solver";
connectAttr "solverShape1.message" "glue_6_0_side.solver";
connectAttr "solverShape1.message" "glue_6_0_up0.solver";
connectAttr "solverShape1.message" "glue_6_0_up1.solver";
connectAttr "solverShape1.message" "glue_6_10_side.solver";
connectAttr "solverShape1.message" "glue_6_10_up10.solver";
connectAttr "solverShape1.message" "glue_6_10_up11.solver";
connectAttr "solverShape1.message" "glue_6_11_side.solver";
connectAttr "solverShape1.message" "glue_6_11_up11.solver";
connectAttr "solverShape1.message" "glue_6_11_up12.solver";
connectAttr "solverShape1.message" "glue_6_12_side.solver";
connectAttr "solverShape1.message" "glue_6_12_up12.solver";
connectAttr "solverShape1.message" "glue_6_12_up13.solver";
connectAttr "solverShape1.message" "glue_6_13_up13.solver";
connectAttr "solverShape1.message" "glue_6_13_up14.solver";
connectAttr "solverShape1.message" "glue_6_1_side.solver";
connectAttr "solverShape1.message" "glue_6_1_up1.solver";
connectAttr "solverShape1.message" "glue_6_1_up2.solver";
connectAttr "solverShape1.message" "glue_6_2_side.solver";
connectAttr "solverShape1.message" "glue_6_2_up2.solver";
connectAttr "solverShape1.message" "glue_6_2_up3.solver";
connectAttr "solverShape1.message" "glue_6_3_side.solver";
connectAttr "solverShape1.message" "glue_6_3_up3.solver";
connectAttr "solverShape1.message" "glue_6_3_up4.solver";
connectAttr "solverShape1.message" "glue_6_4_side.solver";
connectAttr "solverShape1.message" "glue_6_4_up4.solver";
connectAttr "solverShape1.message" "glue_6_4_up5.solver";
connectAttr "solverShape1.message" "glue_6_5_side.solver";
connectAttr "solverShape1.message" "glue_6_5_up5.solver";
connectAttr "solverShape1.message" "glue_6_5_up6.solver";
connectAttr "solverShape1.message" "glue_6_6_side.solver";
connectAttr "solverShape1.message" "glue_6_6_up6.solver";
connectAttr "solverShape1.message" "glue_6_6_up7.solver";
connectAttr "solverShape1.message" "glue_6_7_side.solver";
connectAttr "solverShape1.message" "glue_6_7_up7.solver";
connectAttr "solverShape1.message" "glue_6_7_up8.solver";
connectAttr "solverShape1.message" "glue_6_8_side.solver";
connectAttr "solverShape1.message" "glue_6_8_up8.solver";
connectAttr "solverShape1.message" "glue_6_8_up9.solver";
connectAttr "solverShape1.message" "glue_6_9_side.solver";
connectAttr "solverShape1.message" "glue_6_9_up10.solver";
connectAttr "solverShape1.message" "glue_6_9_up9.solver";
connectAttr "solverShape1.message" "glue_7_0_side.solver";
connectAttr "solverShape1.message" "glue_7_10_side.solver";
connectAttr "solverShape1.message" "glue_7_11_side.solver";
connectAttr "solverShape1.message" "glue_7_12_side.solver";
connectAttr "solverShape1.message" "glue_7_13_side.solver";
connectAttr "solverShape1.message" "glue_7_1_side.solver";
connectAttr "solverShape1.message" "glue_7_2_side.solver";
connectAttr "solverShape1.message" "glue_7_3_side.solver";
connectAttr "solverShape1.message" "glue_7_4_side.solver";
connectAttr "solverShape1.message" "glue_7_5_side.solver";
connectAttr "solverShape1.message" "glue_7_6_side.solver";
connectAttr "solverShape1.message" "glue_7_7_side.solver";
connectAttr "solverShape1.message" "glue_7_8_side.solver";
connectAttr "solverShape1.message" "glue_7_9_side.solver";
connectAttr "solverShape1.message" "rigidBody1.solver";
connectAttr "solverShape1.message" "rigidBody10.solver";
connectAttr "solverShape1.message" "rigidBody100.solver";
connectAttr "solverShape1.message" "rigidBody101.solver";
connectAttr "solverShape1.message" "rigidBody102.solver";
connectAttr "solverShape1.message" "rigidBody103.solver";
connectAttr "solverShape1.message" "rigidBody104.solver";
connectAttr "solverShape1.message" "rigidBody105.solver";
connectAttr "solverShape1.message" "rigidBody106.solver";
connectAttr "solverShape1.message" "rigidBody107.solver";
connectAttr "solverShape1.message" "rigidBody108.solver";
connectAttr "solverShape1.message" "rigidBody109.solver";
connectAttr "solverShape1.message" "rigidBody11.solver";
connectAttr "solverShape1.message" "rigidBody110.solver";
connectAttr "solverShape1.message" "rigidBody111.solver";
connectAttr "solverShape1.message" "rigidBody112.solver";
connectAttr "solverShape1.message" "rigidBody113.solver";
connectAttr "solverShape1.message" "rigidBody114.solver";
connectAttr "solverShape1.message" "rigidBody115.solver";
connectAttr "solverShape1.message" "rigidBody116.solver";
connectAttr "solverShape1.message" "rigidBody117.solver";
connectAttr "solverShape1.message" "rigidBody118.solver";
connectAttr "solverShape1.message" "rigidBody119.solver";
connectAttr "solverShape1.message" "rigidBody12.solver";
connectAttr "solverShape1.message" "rigidBody13.solver";
connectAttr "solverShape1.message" "rigidBody14.solver";
connectAttr "solverShape1.message" "rigidBody15.solver";
connectAttr "solverShape1.message" "rigidBody16.solver";
connectAttr "solverShape1.message" "rigidBody17.solver";
connectAttr "solverShape1.message" "rigidBody18.solver";
connectAttr "solverShape1.message" "rigidBody19.solver";
connectAttr "solverShape1.message" "rigidBody2.solver";
connectAttr "solverShape1.message" "rigidBody20.solver";
connectAttr "solverShape1.message" "rigidBody21.solver";
connectAttr "solverShape1.message" "rigidBody22.solver";
connectAttr "solverShape1.message" "rigidBody23.solver";
connectAttr "solverShape1.message" "rigidBody24.solver";
connectAttr "solverShape1.message" "rigidBody25.solver";
connectAttr "solverShape1.message" "rigidBody26.solver";
connectAttr "solverShape1.message" "rigidBody27.solver";
connectAttr "solverShape1.message" "rigidBody28.solver";
connectAttr "solverShape1.message" "rigidBody29.solver";
connectAttr "solverShape1.message" "rigidBody3.solver";
connectAttr "solverShape1.message" "rigidBody30.solver";
connectAttr "solverShape1.message" "rigidBody31.solver";
connectAttr "solverShape1.message" "rigidBody32.solver";
connectAttr "solverShape1.message" "rigidBody33.solver";
connectAttr "solverShape1.message" "rigidBody34.solver";
connectAttr "solverShape1.message" "rigidBody35.solver";
connectAttr "solverShape1.message" "rigidBody36.solver";
connectAttr "solverShape1.message" "rigidBody37.solver";
connectAttr "solverShape1.message" "rigidBody38.solver";
connectAttr "solverShape1.message" "rigidBody39.solver";
connectAttr "solverShape1.message" "rigidBody4.solver";
connectAttr "solverShape1.message" "rigidBody40.solver";
connectAttr "solverShape1.message" "rigidBody41.solver";
connectAttr "solverShape1.message" "rigidBody42.solver";
connectAttr "solverShape1.message" "rigidBody43.solver";
connectAttr "solverShape1.message" "rigidBody44.solver";
connectAttr "solverShape1.message" "rigidBody45.solver";
connectAttr "solverShape1.message" "rigidBody46.solver";
connectAttr "solverShape1.message" "rigidBody47.solver";
connectAttr "solverShape1.message" "rigidBody48.solver";
connectAttr "solverShape1.message" "rigidBody49.solver";
connectAttr "solverShape1.message" "rigidBody5.solver";
connectAttr "solverShape1.message" "rigidBody50.solver";
connectAttr "solverShape1.message" "rigidBody51.solver";
connectAttr "solverShape1.message" "rigidBody52.solver";
connectAttr "solverShape1.message" "rigidBody53.solver";
connectAttr "solverShape1.message" "rigidBody54.solver";
connectAttr "solverShape1.message" "rigidBody55.solver";
connectAttr "solverShape1.message" "rigidBody56.solver";
connectAttr "solverShape1.message" "rigidBody57.solver";
connectAttr "solverShape1.message" "rigidBody58.solver";
connectAttr "solverShape1.message" "rigidBody59.solver";
connectAttr "solverShape1.message" "rigidBody6.solver";
connectAttr "solverShape1.message" "rigidBody60.solver";
connectAttr "solverShape1.message" "rigidBody61.solver";
connectAttr "solverShape1.message" "rigidBody62.solver";
connectAttr "solverShape1.message" "rigidBody63.solver";
connectAttr "solverShape1.message" "rigidBody64.solver";
connectAttr "solverShape1.message" "rigidBody65.solver";
connectAttr "solverShape1.message" "rigidBody66.solver";
connectAttr "solverShape1.message" "rigidBody67.solver";
connectAttr "solverShape1.message" "rigidBody68.solver";
connectAttr "solverShape1.message" "rigidBody69.solver";
connectAttr "solverShape1.message" "rigidBody7.solver";
connectAttr "solverShape1.message" "rigidBody70.solver";
connectAttr "solverShape1.message" "rigidBody71.solver";
connectAttr "solverShape1.message" "rigidBody72.solver";
connectAttr "solverShape1.message" "rigidBody73.solver";
connectAttr "solverShape1.message" "rigidBody74.solver";
connectAttr "solverShape1.message" "rigidBody75.solver";
connectAttr "solverShape1.message" "rigidBody76.solver";
connectAttr "solverShape1.message" "rigidBody77.solver";
connectAttr "solverShape1.message" "rigidBody78.solver";
connectAttr "solverShape1.message" "rigidBody79.solver";
connectAttr "solverShape1.message" "rigidBody8.solver";
connectAttr "solverShape1.message" "rigidBody80.solver";
connectAttr "solverShape1.message" "rigidBody81.solver";
connectAttr "solverShape1.message" "rigidBody82.solver";
connectAttr "solverShape1.message" "rigidBody83.solver";
connectAttr "solverShape1.message" "rigidBody84.solver";
connectAttr "solverShape1.message" "rigidBody85.solver";
connectAttr "solverShape1.message" "rigidBody86.solver";
connectAttr "solverShape1.message" "rigidBody87.solver";
connectAttr "solverShape1.message" "rigidBody88.solver";
connectAttr "solverShape1.message" "rigidBody89.solver";
connectAttr "solverShape1.message" "rigidBody9.solver";
connectAttr "solverShape1.message" "rigidBody90.solver";
connectAttr "solverShape1.message" "rigidBody91.solver";
connectAttr "solverShape1.message" "rigidBody92.solver";
connectAttr "solverShape1.message" "rigidBody93.solver";
connectAttr "solverShape1.message" "rigidBody94.solver";
connectAttr "solverShape1.message" "rigidBody95.solver";
connectAttr "solverShape1.message" "rigidBody96.solver";
connectAttr "solverShape1.message" "rigidBody97.solver";
connectAttr "solverShape1.message" "rigidBody98.solver";
connectAttr "solverShape1.message" "rigidBody99.solver";
connectAttr "wreckingBallMat.oc" "wreckingBallMatSG.ss";
connectAttr "brickMat_0_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_0_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_0_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_14.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_14SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_1_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_1_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_2_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_2_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_14.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_14SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_3_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_3_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_4_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_4_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_14.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_14SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_5_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_5_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_6_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_6_9SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_0.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_0SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_1.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_10.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_10SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_11.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_11SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_12.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_12SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_13.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_13SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_14.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_14SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_1SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_2.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_2SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_3.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_3SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_4.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_4SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_5.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_5SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_6.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_6SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_7.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_7SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_8.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_8SG.pa" ":renderPartition.st" -na;
connectAttr "brickMat_7_9.msg" ":defaultShaderList1.s" -na;
connectAttr "brickMat_7_9SG.pa" ":renderPartition.st" -na;
connectAttr "brick_0_Shape0.iog" "brickMat_0_0SG.dsm" -na;
connectAttr "brick_0_Shape1.iog" "brickMat_0_1SG.dsm" -na;
connectAttr "brick_0_Shape10.iog" "brickMat_0_10SG.dsm" -na;
connectAttr "brick_0_Shape11.iog" "brickMat_0_11SG.dsm" -na;
connectAttr "brick_0_Shape12.iog" "brickMat_0_12SG.dsm" -na;
connectAttr "brick_0_Shape13.iog" "brickMat_0_13SG.dsm" -na;
connectAttr "brick_0_Shape2.iog" "brickMat_0_2SG.dsm" -na;
connectAttr "brick_0_Shape3.iog" "brickMat_0_3SG.dsm" -na;
connectAttr "brick_0_Shape4.iog" "brickMat_0_4SG.dsm" -na;
connectAttr "brick_0_Shape5.iog" "brickMat_0_5SG.dsm" -na;
connectAttr "brick_0_Shape6.iog" "brickMat_0_6SG.dsm" -na;
connectAttr "brick_0_Shape7.iog" "brickMat_0_7SG.dsm" -na;
connectAttr "brick_0_Shape8.iog" "brickMat_0_8SG.dsm" -na;
connectAttr "brick_0_Shape9.iog" "brickMat_0_9SG.dsm" -na;
connectAttr "brick_1_Shape0.iog" "brickMat_1_0SG.dsm" -na;
connectAttr "brick_1_Shape1.iog" "brickMat_1_1SG.dsm" -na;
connectAttr "brick_1_Shape10.iog" "brickMat_1_10SG.dsm" -na;
connectAttr "brick_1_Shape11.iog" "brickMat_1_11SG.dsm" -na;
connectAttr "brick_1_Shape12.iog" "brickMat_1_12SG.dsm" -na;
connectAttr "brick_1_Shape13.iog" "brickMat_1_13SG.dsm" -na;
connectAttr "brick_1_Shape14.iog" "brickMat_1_14SG.dsm" -na;
connectAttr "brick_1_Shape2.iog" "brickMat_1_2SG.dsm" -na;
connectAttr "brick_1_Shape3.iog" "brickMat_1_3SG.dsm" -na;
connectAttr "brick_1_Shape4.iog" "brickMat_1_4SG.dsm" -na;
connectAttr "brick_1_Shape5.iog" "brickMat_1_5SG.dsm" -na;
connectAttr "brick_1_Shape6.iog" "brickMat_1_6SG.dsm" -na;
connectAttr "brick_1_Shape7.iog" "brickMat_1_7SG.dsm" -na;
connectAttr "brick_1_Shape8.iog" "brickMat_1_8SG.dsm" -na;
connectAttr "brick_1_Shape9.iog" "brickMat_1_9SG.dsm" -na;
connectAttr "brick_2_Shape0.iog" "brickMat_2_0SG.dsm" -na;
connectAttr "brick_2_Shape1.iog" "brickMat_2_1SG.dsm" -na;
connectAttr "brick_2_Shape10.iog" "brickMat_2_10SG.dsm" -na;
connectAttr "brick_2_Shape11.iog" "brickMat_2_11SG.dsm" -na;
connectAttr "brick_2_Shape12.iog" "brickMat_2_12SG.dsm" -na;
connectAttr "brick_2_Shape13.iog" "brickMat_2_13SG.dsm" -na;
connectAttr "brick_2_Shape2.iog" "brickMat_2_2SG.dsm" -na;
connectAttr "brick_2_Shape3.iog" "brickMat_2_3SG.dsm" -na;
connectAttr "brick_2_Shape4.iog" "brickMat_2_4SG.dsm" -na;
connectAttr "brick_2_Shape5.iog" "brickMat_2_5SG.dsm" -na;
connectAttr "brick_2_Shape6.iog" "brickMat_2_6SG.dsm" -na;
connectAttr "brick_2_Shape7.iog" "brickMat_2_7SG.dsm" -na;
connectAttr "brick_2_Shape8.iog" "brickMat_2_8SG.dsm" -na;
connectAttr "brick_2_Shape9.iog" "brickMat_2_9SG.dsm" -na;
connectAttr "brick_3_Shape0.iog" "brickMat_3_0SG.dsm" -na;
connectAttr "brick_3_Shape1.iog" "brickMat_3_1SG.dsm" -na;
connectAttr "brick_3_Shape10.iog" "brickMat_3_10SG.dsm" -na;
connectAttr "brick_3_Shape11.iog" "brickMat_3_11SG.dsm" -na;
connectAttr "brick_3_Shape12.iog" "brickMat_3_12SG.dsm" -na;
connectAttr "brick_3_Shape13.iog" "brickMat_3_13SG.dsm" -na;
connectAttr "brick_3_Shape14.iog" "brickMat_3_14SG.dsm" -na;
connectAttr "brick_3_Shape2.iog" "brickMat_3_2SG.dsm" -na;
connectAttr "brick_3_Shape3.iog" "brickMat_3_3SG.dsm" -na;
connectAttr "brick_3_Shape4.iog" "brickMat_3_4SG.dsm" -na;
connectAttr "brick_3_Shape5.iog" "brickMat_3_5SG.dsm" -na;
connectAttr "brick_3_Shape6.iog" "brickMat_3_6SG.dsm" -na;
connectAttr "brick_3_Shape7.iog" "brickMat_3_7SG.dsm" -na;
connectAttr "brick_3_Shape8.iog" "brickMat_3_8SG.dsm" -na;
connectAttr "brick_3_Shape9.iog" "brickMat_3_9SG.dsm" -na;
connectAttr "brick_4_Shape0.iog" "brickMat_4_0SG.dsm" -na;
connectAttr "brick_4_Shape1.iog" "brickMat_4_1SG.dsm" -na;
connectAttr "brick_4_Shape10.iog" "brickMat_4_10SG.dsm" -na;
connectAttr "brick_4_Shape11.iog" "brickMat_4_11SG.dsm" -na;
connectAttr "brick_4_Shape12.iog" "brickMat_4_12SG.dsm" -na;
connectAttr "brick_4_Shape13.iog" "brickMat_4_13SG.dsm" -na;
connectAttr "brick_4_Shape2.iog" "brickMat_4_2SG.dsm" -na;
connectAttr "brick_4_Shape3.iog" "brickMat_4_3SG.dsm" -na;
connectAttr "brick_4_Shape4.iog" "brickMat_4_4SG.dsm" -na;
connectAttr "brick_4_Shape5.iog" "brickMat_4_5SG.dsm" -na;
connectAttr "brick_4_Shape6.iog" "brickMat_4_6SG.dsm" -na;
connectAttr "brick_4_Shape7.iog" "brickMat_4_7SG.dsm" -na;
connectAttr "brick_4_Shape8.iog" "brickMat_4_8SG.dsm" -na;
connectAttr "brick_4_Shape9.iog" "brickMat_4_9SG.dsm" -na;
connectAttr "brick_5_Shape0.iog" "brickMat_5_0SG.dsm" -na;
connectAttr "brick_5_Shape1.iog" "brickMat_5_1SG.dsm" -na;
connectAttr "brick_5_Shape10.iog" "brickMat_5_10SG.dsm" -na;
connectAttr "brick_5_Shape11.iog" "brickMat_5_11SG.dsm" -na;
connectAttr "brick_5_Shape12.iog" "brickMat_5_12SG.dsm" -na;
connectAttr "brick_5_Shape13.iog" "brickMat_5_13SG.dsm" -na;
connectAttr "brick_5_Shape14.iog" "brickMat_5_14SG.dsm" -na;
connectAttr "brick_5_Shape2.iog" "brickMat_5_2SG.dsm" -na;
connectAttr "brick_5_Shape3.iog" "brickMat_5_3SG.dsm" -na;
connectAttr "brick_5_Shape4.iog" "brickMat_5_4SG.dsm" -na;
connectAttr "brick_5_Shape5.iog" "brickMat_5_5SG.dsm" -na;
connectAttr "brick_5_Shape6.iog" "brickMat_5_6SG.dsm" -na;
connectAttr "brick_5_Shape7.iog" "brickMat_5_7SG.dsm" -na;
connectAttr "brick_5_Shape8.iog" "brickMat_5_8SG.dsm" -na;
connectAttr "brick_5_Shape9.iog" "brickMat_5_9SG.dsm" -na;
connectAttr "brick_6_Shape0.iog" "brickMat_6_0SG.dsm" -na;
connectAttr "brick_6_Shape1.iog" "brickMat_6_1SG.dsm" -na;
connectAttr "brick_6_Shape10.iog" "brickMat_6_10SG.dsm" -na;
connectAttr "brick_6_Shape11.iog" "brickMat_6_11SG.dsm" -na;
connectAttr "brick_6_Shape12.iog" "brickMat_6_12SG.dsm" -na;
connectAttr "brick_6_Shape13.iog" "brickMat_6_13SG.dsm" -na;
connectAttr "brick_6_Shape2.iog" "brickMat_6_2SG.dsm" -na;
connectAttr "brick_6_Shape3.iog" "brickMat_6_3SG.dsm" -na;
connectAttr "brick_6_Shape4.iog" "brickMat_6_4SG.dsm" -na;
connectAttr "brick_6_Shape5.iog" "brickMat_6_5SG.dsm" -na;
connectAttr "brick_6_Shape6.iog" "brickMat_6_6SG.dsm" -na;
connectAttr "brick_6_Shape7.iog" "brickMat_6_7SG.dsm" -na;
connectAttr "brick_6_Shape8.iog" "brickMat_6_8SG.dsm" -na;
connectAttr "brick_6_Shape9.iog" "brickMat_6_9SG.dsm" -na;
connectAttr "brick_7_Shape0.iog" "brickMat_7_0SG.dsm" -na;
connectAttr "brick_7_Shape1.iog" "brickMat_7_1SG.dsm" -na;
connectAttr "brick_7_Shape10.iog" "brickMat_7_10SG.dsm" -na;
connectAttr "brick_7_Shape11.iog" "brickMat_7_11SG.dsm" -na;
connectAttr "brick_7_Shape12.iog" "brickMat_7_12SG.dsm" -na;
connectAttr "brick_7_Shape13.iog" "brickMat_7_13SG.dsm" -na;
connectAttr "brick_7_Shape14.iog" "brickMat_7_14SG.dsm" -na;
connectAttr "brick_7_Shape2.iog" "brickMat_7_2SG.dsm" -na;
connectAttr "brick_7_Shape3.iog" "brickMat_7_3SG.dsm" -na;
connectAttr "brick_7_Shape4.iog" "brickMat_7_4SG.dsm" -na;
connectAttr "brick_7_Shape5.iog" "brickMat_7_5SG.dsm" -na;
connectAttr "brick_7_Shape6.iog" "brickMat_7_6SG.dsm" -na;
connectAttr "brick_7_Shape7.iog" "brickMat_7_7SG.dsm" -na;
connectAttr "brick_7_Shape8.iog" "brickMat_7_8SG.dsm" -na;
connectAttr "brick_7_Shape9.iog" "brickMat_7_9SG.dsm" -na;
connectAttr "defaultRenderLayer.msg" ":defaultRenderingList1.r" -na;
connectAttr "fillLight.iog" ":defaultLightSet.dsm" -na;
connectAttr "fillLightShape.ltd" ":lightList1.l" -na;
connectAttr "floorMat.msg" ":defaultShaderList1.s" -na;
connectAttr "floorMatSG.pa" ":renderPartition.st" -na;
connectAttr "floorShape.iog" "floorMatSG.dsm" -na;
connectAttr "keyLight.iog" ":defaultLightSet.dsm" -na;
connectAttr "keyLightShape.ltd" ":lightList1.l" -na;
connectAttr "secondBallMat.msg" ":defaultShaderList1.s" -na;
connectAttr "secondBallMatSG.pa" ":renderPartition.st" -na;
connectAttr "secondBallShape.iog" "secondBallMatSG.dsm" -na;
connectAttr "wreckingBallMat.msg" ":defaultShaderList1.s" -na;
connectAttr "wreckingBallMatSG.pa" ":renderPartition.st" -na;
connectAttr "wreckingBallShape.iog" "wreckingBallMatSG.dsm" -na;
// End of rigid_wall_breaking.ma
