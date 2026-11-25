#include <algorithm>
#include <QJsonDocument>
#include <QSettings>
// The Attribute Editor: a tab per node in the chain, sections by node type, sliders on ranged attributes, a Notes box, and
// Select, Load Attributes and Copy Tab along the bottom.
#include "panels.h"
#include "colorchooser.h"
#include "host.h"
#include "icons.h"
#include "rendernodes.h"

#include <QApplication>
#include <QCheckBox>
#include <QTimer>
#include <QScrollBar>
#include <QColorDialog>
#include <QFileDialog>
#include <QFileInfo>
#include <QMenu>
#include <QComboBox>
#include <QInputDialog>
#include <QFormLayout>
#include <QHash>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QPointer>
#include <QPushButton>
#include <QRadioButton>
#include <QRegularExpression>
#include <QScrollArea>
#include <QMouseEvent>
#include <QPainter>
#include <QSlider>
#include <QStyle>
#include <QStyleOption>
#include <QWheelEvent>
#include <QTabBar>
#include <QTabWidget>
#include <QTableWidget>
#include <QHeaderView>
#include <QToolButton>
#include <QVBoxLayout>
#include <cmath>

namespace az {

namespace {

// A tab dragged out of the strip tears off into a window of its own, and tabs drag to reorder, as the reference's do.
class AeTabBar : public QTabBar {
public:
    explicit AeTabBar(QWidget* parent = nullptr) : QTabBar(parent) {
        setMovable(true);
        setExpanding(false);
    }
    std::function<void(int)> tearOff;

protected:
    void mouseMoveEvent(QMouseEvent* e) override {
        if ((e->buttons() & Qt::LeftButton) && !torn_ && (e->position().y() < -40 || e->position().y() > height() + 40)) {
            torn_ = true;
            if (tearOff) tearOff(currentIndex());
            e->accept();
            return;
        }
        QTabBar::mouseMoveEvent(e);
    }
    void mouseReleaseEvent(QMouseEvent* e) override {
        torn_ = false;
        QTabBar::mouseReleaseEvent(e);
    }

private:
    bool torn_ = false;
};

// The order the user has dragged the tabs into, kept while the editor runs.
QStringList& tab_order() {
    static QStringList order;
    return order;
}

class AeTabWidget : public QTabWidget {
public:
    explicit AeTabWidget(QWidget* parent = nullptr) : QTabWidget(parent) {}
    void use_bar(QTabBar* bar) { setTabBar(bar); }
};

QJsonObject args(const QJsonArray& a) {
    QJsonObject o;
    o["args"] = a;
    return o;
}

QString fmt3(double v) {
    QString s = QString::number(v, 'f', 3);
    return s == "-0.000" ? "0.000" : s;
}

// Whether a field shows a value it does not take: an output, a connection a key does not still drive, or a locked
// attribute (#212).
bool read_only(const QJsonObject& a) {
    return a["output"].toBool() || (a["connected"].toBool() && !a["keyed"].toBool()) || a["locked"].toBool();
}

void set_plug(const QString& plug, const QJsonValue& v, const QString& type) {
    QJsonObject call = args(QJsonArray{plug, v});
    if (type == "string") call["type"] = "string";
    exec("setAttr", call);
}

// Which section an attribute falls in, by node type, in the order sections come. Anything else goes to Extra Attributes.
struct Section { const char* title; const char* attrs; };

// In the order the Attribute Editor lists them under Display.
const char* const kDrawingOverrides = "overrideEnabled overrideDisplayType overrideLevelOfDetail overrideShading overrideTexturing overridePlayback overrideVisibility hideOnPlayback overrideRGBColors overrideColor overrideColorRGB";

QString override_label(const QString& attr, const QString& nice) {
    static const QHash<QString, QString> labels{{"overrideEnabled", "Enable Overrides"},  {"overrideDisplayType", "Display Type"}, {"overrideLevelOfDetail", "Level of Detail"},
                                                {"overrideShading", "Shading"},           {"overrideTexturing", "Texturing"},      {"overridePlayback", "Playback"},
                                                {"overrideVisibility", "Visible"},        {"hideOnPlayback", "Hide on Playback"},  {"overrideRGBColors", "Enable RGB Colors"},
                                                {"overrideColor", "Color"},               {"overrideColorRGB", "Color RGB"}};
    if (labels.contains(attr)) return labels.value(attr);
    // Harold's attributes keep the ai prefix of the files they come from; the frames they sit in say whose they are.
    static const QHash<QString, QString> harold{{"aiSubdivType", "Type"}, {"aiSubdivIterations", "Iterations"}, {"aiSubdivAdaptiveMetric", "Adaptive Metric"},
                                                {"aiSubdivPixelError", "Adaptive Error"}, {"aiSubdivUvSmoothing", "UV Smoothing"}, {"aiSubdivSmoothDerivs", "Smooth Tangents"},
                                                {"aiDispHeight", "Height"}, {"aiDispZeroValue", "Scalar Zero Value"}, {"aiDispPadding", "Bounds Padding"}, {"aiDispAutobump", "Auto Bump"},
                                                {"aiVisibleInDiffuseReflection", "Diffuse Reflection"}, {"aiVisibleInSpecularReflection", "Specular Reflection"},
                                                {"aiVisibleInDiffuseTransmission", "Diffuse Transmission"}, {"aiVisibleInSpecularTransmission", "Specular Transmission"},
                                                {"aiVisibleInVolume", "Volume"}, {"aiSss", "SSS"}, {"aiAov", "AOV Light Group"}, {"aiSssSetname", "SSS Set Name"},
                                                {"aiCastVolumetricShadows", "Volumetric Shadows"}, {"aiTranslator", "Light Shape"}, {"aiFilename", "Photometry File"}, {"fStop", "F Stop"}};
    if (harold.contains(attr)) return harold.value(attr);
    if (attr.startsWith("ai") && attr.size() > 2 && attr[2].isUpper() && nice.startsWith("Ai ")) return nice.mid(3);
    return nice;
}

// Harold's frames on lights, shapes and cameras.
const char* const kHaroldLight = "aiExposure aiSamples aiRadius aiAngle aiSpread aiRoundness aiSoftEdge aiResolution aiNormalize aiCastShadows aiShadowDensity aiCastVolumetricShadows aiDiffuse aiSpecular aiSss aiIndirect aiVolume aiTransmission aiMaxBounces aiVolumeSamples aiDecayType aiAov";
const char* const kRenderStats = "castsShadows receiveShadows primaryVisibility visibleInReflections visibleInRefractions doubleSided smoothShading motionBlur";
const char* const kHaroldShape = "aiOpaque aiMatte aiSelfShadows aiVisibleInDiffuseReflection aiVisibleInSpecularReflection aiVisibleInDiffuseTransmission aiVisibleInSpecularTransmission aiVisibleInVolume aiTraceSets aiSssSetname";
const char* const kHaroldSubdiv = "aiSubdivType aiSubdivIterations aiSubdivAdaptiveMetric aiSubdivPixelError aiSubdivUvSmoothing aiSubdivSmoothDerivs";
const char* const kHaroldDisp = "aiDispHeight aiDispZeroValue aiDispPadding aiDispAutobump";

// The standard surface's groups, as its specification orders them; snake case for the file name aiStandardSurface.
QList<Section> standard_sections(bool snake) {
    static QList<QByteArray> keep;
    auto s = [snake](const char* camel, const char* sn) {
        keep.append(QByteArray(snake ? sn : camel));
        return keep.last().constData();
    };
    return {
        {"Base", s("base baseColor diffuseRoughness metalness", "base base_color diffuse_roughness metalness")},
        {"Specular", s("specular specularColor specularRoughness specularIOR specularAnisotropy specularRotation", "specular specular_color specular_roughness specular_IOR specular_anisotropy specular_rotation")},
        {"Transmission", s("transmission transmissionColor transmissionDepth transmissionScatter transmissionScatterAnisotropy transmissionDispersion transmissionExtraRoughness",
                           "transmission transmission_color transmission_depth transmission_scatter transmission_scatter_anisotropy transmission_dispersion transmission_extra_roughness")},
        {"Subsurface", s("subsurface subsurfaceColor subsurfaceRadius subsurfaceScale subsurfaceAnisotropy subsurfaceType", "subsurface subsurface_color subsurface_radius subsurface_scale subsurface_anisotropy subsurface_type")},
        {"Coat", s("coat coatColor coatRoughness coatAnisotropy coatRotation coatIOR coatNormal coatAffectColor coatAffectRoughness",
                   "coat coat_color coat_roughness coat_anisotropy coat_rotation coat_IOR coat_normal coat_affect_color coat_affect_roughness")},
        {"Sheen", s("sheen sheenColor sheenRoughness", "sheen sheen_color sheen_roughness")},
        {"Emission", s("emission emissionColor", "emission emission_color")},
        {"Thin Film", s("thinFilmThickness thinFilmIOR", "thin_film_thickness thin_film_IOR")},
        {"Geometry", s("opacity thinWalled normalCamera tangentUCamera", "opacity thin_walled normalCamera tangent")},
        {"Advanced", s("caustics internalReflections exitToBackground indirectDiffuse indirectSpecular", "caustics internal_reflections exit_to_background indirect_diffuse indirect_specular")},
        {"Node Behaviour", "caching nodeState frozen"}};
}

const QList<Section>& sections_for(const QString& type) {
    static const QList<Section> xf{
        {"Transform Attributes", "translate rotate rotateOrder scale shear rotateAxis inheritsTransform"},
        {"Pivots", "rotatePivot scalePivot rotatePivotTranslate scalePivotTranslate displayRotatePivot displayScalePivot"},
        {"Limit Information", "minTransLimit maxTransLimit minRotLimit maxRotLimit minScaleLimit maxScaleLimit"},
        {"Display", "visibility template displayHandle displayLocalAxis intermediateObject useObjectColor objectColor wireColorRGB useOutlinerColor outlinerColor hiddenInOutliner"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> joint{
        {"Transform Attributes", "translate rotate rotateOrder scale shear inheritsTransform"},
        {"Joint", "jointOrient jointType segmentScaleCompensate preferredAngle stiffness radius drawStyle side type otherType"},
        {"Limit Information", "minRotLimit maxRotLimit minTransLimit maxTransLimit"},
        {"Display", "visibility template displayLocalAxis useObjectColor objectColor wireColorRGB useOutlinerColor outlinerColor hiddenInOutliner"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> mesh{
        {"Mesh Component Display", "displayVertices displayEdges displayBorders displayNormal normalSize backfaceCulling"},
        {"Smooth Mesh", "displaySmoothMesh smoothLevel smoothPreview"},
        {"Symmetry", "symmetry symmetryPlane"},
        {"Render Stats", kRenderStats},
        {"Harold", kHaroldShape},
        {"Harold Subdivision", kHaroldSubdiv},
        {"Harold Displacement", kHaroldDisp},
        {"Object Display", "visibility template intermediateObject hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> camera{
        {"Camera Attributes", "focalLength lensSqueezeRatio cameraScale nearClipPlane farClipPlane centerOfInterest"},
        {"Film Back", "horizontalFilmAperture verticalFilmAperture filmFit filmFitOffset horizontalFilmOffset verticalFilmOffset"},
        {"Orthographic Views", "orthographic orthographicWidth"},
        {"Harold", "renderable depthOfField fStop focusDistance focusRegionScale shutterAngle motionBlur"},
        {"Display Options", "displayFilmGate displayResolution displayGateMask overscan"},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"}};
    static const QList<Section> ambient{
        {"Ambient Light Attributes", "color intensity ambientShade"},
        {"Shadows", "shadowColor useRayTraceShadows shadowRays rayDepthLimit castSoftShadows shadowRadius receiveShadows"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> directional{
        {"Directional Light Attributes", "color intensity emitDiffuse emitSpecular"},
        {"Shadows", "shadowColor useDepthMapShadows useRayTraceShadows lightAngle shadowRays rayDepthLimit dmapResolution dmapFilterSize dmapBias"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> point{
        {"Point Light Attributes", "color intensity emitDiffuse emitSpecular decayRate"},
        {"Light Effects", "fogType fogRadius fogIntensity"},
        {"Shadows", "shadowColor useDepthMapShadows useRayTraceShadows lightRadius shadowRays rayDepthLimit dmapResolution dmapFilterSize dmapBias"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> spot{
        {"Spot Light Attributes", "color intensity emitDiffuse emitSpecular decayRate coneAngle penumbraAngle dropoff"},
        {"Light Effects", "fogSpread fogIntensity barnDoors leftBarnDoor rightBarnDoor topBarnDoor bottomBarnDoor useDecayRegions startDistance1 endDistance1 startDistance2 endDistance2 startDistance3 endDistance3"},
        {"Shadows", "shadowColor useDepthMapShadows useRayTraceShadows lightRadius shadowRays rayDepthLimit dmapResolution dmapFilterSize dmapBias"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> area{
        {"Area Light Attributes", "color intensity emitDiffuse emitSpecular decayRate normalize"},
        {"Shadows", "shadowColor useDepthMapShadows useRayTraceShadows shadowRays rayDepthLimit dmapResolution dmapFilterSize dmapBias"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> volume{
        {"Volume Light Attributes", "color intensity lightShape volumeLightDir arc coneEndRadius emitAmbient emitDiffuse emitSpecular"},
        {"Shadows", "shadowColor useRayTraceShadows lightRadius shadowRays rayDepthLimit"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> curve{
        {"NURBS Curve History", "degree form spans"},
        {"Component Display", "dispCV dispEP dispHull lineWidth"},
        {"Object Display", "visibility template intermediateObject hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> skin{
        {"Skin Cluster Attributes", "envelope skinningMethod useComponents maxInfluences maintainMaxInfluences normalizeWeights weightDistribution deformUserNormals"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> material{
        {"Common Material Attributes", "color outColor transparency outTransparency ambientColor incandescence normalCamera diffuse translucence translucenceDepth translucenceFocus baseColor base metalness opacity"},
        {"Specular Shading", "cosinePower eccentricity specularRollOff angle spreadX spreadY roughness highlightSize whiteness specularColor specular specularRoughness reflectivity reflectedColor reflectionLimit fresnelRefractiveIndex"},
        {"Special Effects", "hideSource glowIntensity emission emissionColor outGlowColor"},
        {"Matte Opacity", "matteOpacityMode matteOpacity outMatteOpacity"},
        {"Raytrace Options", "refractions refractiveIndex refractionLimit lightAbsorbance surfaceThickness shadowAttenuation chromaticAberration reflectionSpecularity"}};
    static const QList<Section> file{
        {"File Attributes", "fileTextureName colorSpace ignoreColorSpaceFileRules filterType filter filterOffset useFrameExtension frameExtension frameOffset uvTilingMode"},
        {"Color Balance", "defaultColor colorGain colorOffset alphaGain alphaOffset alphaIsLuminance invert exposure"},
        {"Effects", "fileHasAlpha"},
        {"2d Texture Placement", "coverage translateFrame rotateFrame mirrorU mirrorV wrapU wrapV stagger repeatUV offset rotateUV noiseUV"}};
    static const QList<Section> psd{
        {"PSD File Attributes", "fileTextureName layerSetName alpha colorSpace ignoreColorSpaceFileRules filterType filter filterOffset useFrameExtension frameExtension frameOffset uvTilingMode"},
        {"Color Balance", "defaultColor colorGain colorOffset alphaGain alphaOffset alphaIsLuminance invert exposure"},
        {"Effects", "fileHasAlpha"},
        {"2d Texture Placement", "coverage translateFrame rotateFrame mirrorU mirrorV wrapU wrapV stagger repeatUV offset rotateUV noiseUV"}};
    static const QList<Section> place{{"2d Texture Placement Attributes", "coverage translateFrame rotateFrame mirrorU mirrorV wrapU wrapV stagger repeatUV offset rotateUV noiseUV"}};
    static const QList<Section> group{{"Shading Group Attributes", "surfaceShader volumeShader displacementShader imageShader renderableOnlySet annotation"}};
    static const QList<Section> other{{"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> shape{
        {"Object Display", "visibility template intermediateObject hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> surface{
        {"Render Stats", kRenderStats},
        {"Harold", kHaroldShape},
        {"Harold Subdivision", kHaroldSubdiv},
        {"Harold Displacement", kHaroldDisp},
        {"Object Display", "visibility template intermediateObject hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Drawing Overrides", kDrawingOverrides},
        {"Node Behaviour", "caching nodeState frozen"}};
    if (type == "nurbsSurface") return surface;
    static const QList<Section> standard = standard_sections(false);
    static const QList<Section> ai_standard = standard_sections(true);
    if (type == "standardSurface") return standard;
    if (type == "aiStandardSurface") return ai_standard;
    static const QList<Section> sky{
        {"Sky Dome Light Attributes", "color intensity aiExposure shadowColor resolution aiResolution format camera aiCamera skyRadius"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> ai_area{
        {"Area Light Attributes", "color intensity aiExposure shadowColor aiTranslator aiSpread aiRoundness aiSoftEdge"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> mesh_light{
        {"Mesh Light Attributes", "color intensity aiExposure shadowColor lightVisible"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    static const QList<Section> photometric{
        {"Photometric Light Attributes", "color intensity aiExposure shadowColor aiFilename"},
        {"Harold", kHaroldLight},
        {"Object Display", "visibility template hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor"},
        {"Node Behaviour", "caching nodeState frozen"}};
    if (type == "aiSkyDomeLight") return sky;
    if (type == "aiAreaLight") return ai_area;
    if (type == "aiMeshLight") return mesh_light;
    if (type == "aiPhotometricLight") return photometric;
    static const QSet<QString> shapes{"nurbsSurface", "lattice", "baseLattice", "deformBend", "deformTwist", "deformSquash", "deformFlare", "deformSine", "deformWave", "clusterHandle", "implicitSphere", "softModHandle", "follicle", "distanceDimShape", "annotationShape", "motionTrailShape", "ikEffector"};
    if (shapes.contains(type)) return shape;
    static const QSet<QString> materials{"lambert", "blinn", "phong", "phongE", "anisotropic", "surfaceShader"};
    if (materials.contains(type)) return material;
    if (type == "file") return file;
    if (type == "psdFileTex") return psd;
    if (type == "place2dTexture") return place;
    if (type == "shadingEngine") return group;
    if (type == "transform" || type == "ikHandle" || type == "locator") return xf;
    // Constraints are transforms (#121): their own settings first, then the transform's sections. Weights are added
    // attributes, so they come under Extra Attributes, as in the reference.
    static const QStringList constraints{"parentConstraint", "pointConstraint", "orientConstraint", "scaleConstraint", "aimConstraint", "poleVectorConstraint",
                                         "geometryConstraint", "normalConstraint", "pointOnPolyConstraint", "tangentConstraint"};
    if (constraints.contains(type)) {
        static QHash<QString, QList<Section>> made;
        static QList<QByteArray> titles;
        if (!made.contains(type)) {
            QString nice;
            for (QChar c : type) {
                if (c.isUpper()) nice += ' ';
                nice += nice.isEmpty() ? c.toUpper() : c;
            }
            // The reference's layout: the transform's sections, then the constraint's own in its order; a parent
            // constraint's are Constraint Attributes, after its Target Offsets.
            titles.append((type == "parentConstraint" ? QString("Constraint Attributes") : nice + " Attributes").toUtf8());
            QList<Section> l = xf;
            l.append(Section{titles.last().constData(), "lockOutput offset offsetPolarity interpType aimVector upVector worldUpType worldUpVector constraintTranslate constraintRotate constraintScale constraintVector enableRestPosition restTranslate restRotate restScale constraintRotateOrder"});
            made.insert(type, l);
        }
        return made[type];
    }
    if (type == "joint") return joint;
    if (type == "mesh") return mesh;
    if (type == "camera") return camera;
    if (type == "ambientLight") return ambient;
    if (type == "directionalLight") return directional;
    if (type == "pointLight") return point;
    if (type == "spotLight") return spot;
    if (type == "areaLight") return area;
    if (type == "volumeLight") return volume;
    if (type == "nurbsCurve") return curve;
    if (type == "skinCluster") return skin;
    return other;
}

// A slider as the reference's: a press anywhere on the track puts the handle there and the drag goes on from it.
class AttrSlider : public QSlider {
public:
    using QSlider::QSlider;

protected:
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() == Qt::LeftButton) {
            jump(e);
            setSliderDown(true);
            emit sliderPressed();
            e->accept();
            return;
        }
        QSlider::mousePressEvent(e);
    }
    void mouseMoveEvent(QMouseEvent* e) override {
        if (isSliderDown() && (e->buttons() & Qt::LeftButton)) {
            jump(e);
            e->accept();
            return;
        }
        QSlider::mouseMoveEvent(e);
    }
    void mouseReleaseEvent(QMouseEvent* e) override {
        if (e->button() == Qt::LeftButton && isSliderDown()) {
            setSliderDown(false);
            emit sliderReleased();
            e->accept();
            return;
        }
        QSlider::mouseReleaseEvent(e);
    }
    void wheelEvent(QWheelEvent* e) override { e->ignore(); }

private:
    void jump(QMouseEvent* e) {
        const int handle = 10;
        int v = QStyle::sliderValueFromPosition(minimum(), maximum(), int(e->position().x()) - handle / 2, qMax(1, width() - handle));
        if (v != sliderPosition()) {
            setSliderPosition(v);
            emit sliderMoved(v);
        }
    }
};

// The map button's two faces: a checker where a texture can go, an arrow into a box where something is connected.
QIcon map_icon(bool connected) {
    QPixmap px(14, 14);
    px.fill(Qt::transparent);
    QPainter p(&px);
    if (!connected) {
        p.fillRect(1, 1, 12, 12, QColor(20, 20, 20));
        p.fillRect(1, 1, 6, 6, QColor(235, 235, 235));
        p.fillRect(7, 7, 6, 6, QColor(235, 235, 235));
    } else {
        p.setPen(QPen(QColor(225, 225, 225), 1.2));
        p.drawRect(5, 2, 8, 10);
        p.drawLine(0, 7, 8, 7);
        p.drawLine(8, 7, 5, 4);
        p.drawLine(8, 7, 5, 10);
    }
    return QIcon(px);
}

void break_connection(const QString& plug) {
    QJsonArray from = ask("listConnections", QJsonObject{{"args", QJsonArray{plug}}, {"plugs", true}, {"destination", false}}).toArray();
    if (!from.isEmpty()) exec("disconnectAttr", args(QJsonArray{from.at(0), plug}));
}

// The button at the end of a row that a texture can drive: it opens Create Render Node, or goes to what is connected.
QToolButton* map_button(const QJsonObject& a, QWidget* parent) {
    QString plug = a["plug"].toString(), source = a["source"].toString();
    auto* b = new QToolButton(parent);
    b->setObjectName("mapButton");
    b->setAutoRaise(true);
    b->setFixedSize(18, 18);
    b->setIcon(map_icon(!source.isEmpty()));
    b->setToolTip(source.isEmpty() ? "Create a render node and connect it to this attribute" : "Go to input connection: " + source);
    b->setContextMenuPolicy(Qt::CustomContextMenu);
    QObject::connect(b, &QToolButton::clicked, b, [b, plug, source] {
        if (!source.isEmpty()) {
            exec("select", args(QJsonArray{source}));
            return;
        }
        create_render_node(b->window(), plug);
    });
    QObject::connect(b, &QToolButton::customContextMenuRequested, b, [b, plug, source](const QPoint& at) {
        if (source.isEmpty()) return;
        QMenu m;
        m.addAction("Break Connection", [plug] { break_connection(plug); });
        m.exec(b->mapToGlobal(at));
    });
    return b;
}

// Slider ranges widened by a value typed past them, kept while the program runs, as the reference's sliders keep theirs.
QHash<QString, QPair<double, double>>& widened() {
    static QHash<QString, QPair<double, double>> w;
    return w;
}

// A number with a slider beside it, when the attribute has a range. The slider runs over the soft range; a value typed
// past it, within the attribute's limits, widens the slider so the value sits half way.
QWidget* ranged_field(const QJsonObject& a, double lo, double hi, QWidget* parent) {
    QString plug = a["plug"].toString(), type = a["type"].toString();
    const double value = a["value"].toDouble();
    if (widened().contains(plug)) {
        lo = qMin(lo, widened()[plug].first);
        hi = qMax(hi, widened()[plug].second);
    }
    if (value > hi) hi = value * 2.0;
    if (value < lo) lo = value < 0 ? value * 2.0 : value;
    if (a["hardMax"].isDouble()) hi = qMin(hi, a["hardMax"].toDouble());
    if (a["hardMin"].isDouble()) lo = qMax(lo, a["hardMin"].toDouble());
    widened()[plug] = {lo, hi};
    auto* row = new QWidget(parent);
    auto* h = new QHBoxLayout(row);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(4);
    auto* e = new QLineEdit(type == "int" ? QString::number(a["value"].toInteger()) : fmt3(value), row);
    e->setProperty("plugKey", plug);
    e->setFixedWidth(68);
    auto* s = new AttrSlider(Qt::Horizontal, row);
    s->setMinimumWidth(8);
    s->setRange(0, 1000);
    s->setProperty("plugKey", plug);
    auto clamp = [a](double v) {
        if (a["hardMin"].isDouble()) v = qMax(v, a["hardMin"].toDouble());
        if (a["hardMax"].isDouble()) v = qMin(v, a["hardMax"].toDouble());
        return v;
    };
    auto to_pos = [lo, hi](double v) { return int(qBound(0.0, (v - lo) / qMax(1e-9, hi - lo), 1.0) * 1000 + 0.5); };
    s->setValue(to_pos(value));
    bool ro = read_only(a);
    e->setReadOnly(ro);
    if (a["keyed"].toBool()) e->setObjectName("keyedField");
    else if (a["connected"].toBool()) e->setObjectName("connectedField");
    if (a["locked"].toBool()) e->setObjectName("lockedField");
    s->setEnabled(!ro);
    bool is_int = type == "int";
    auto value_of = [lo, hi, is_int, clamp](int p) {
        double v = clamp(lo + (hi - lo) * p / 1000.0);
        return is_int ? QJsonValue(qint64(std::round(v))) : QJsonValue(v);
    };
    QObject::connect(s, &QSlider::sliderPressed, s, [plug] { exec("undoInfo", QJsonObject{{"openChunk", true}}); });
    QObject::connect(s, &QSlider::sliderMoved, e, [e, value_of, plug, is_int](int p) {
        QJsonValue v = value_of(p);
        e->setText(is_int ? QString::number(v.toInteger()) : fmt3(v.toDouble()));
        exec_quiet("setAttr", args(QJsonArray{plug, v}));
        Host::get()->poke();
    });
    QObject::connect(s, &QSlider::sliderReleased, e, [s, value_of, plug] {
        exec("setAttr", args(QJsonArray{plug, value_of(s->value())}));
        exec("undoInfo", QJsonObject{{"closeChunk", true}});
    });
    QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
    QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug, is_int, clamp] {
        if (!e->property("editing").toBool()) return;
        e->setProperty("editing", false);
        double v = clamp(e->text().toDouble());
        e->setText(is_int ? QString::number(qint64(std::round(v))) : fmt3(v));
        exec("setAttr", args(QJsonArray{plug, is_int ? QJsonValue(qint64(std::round(v))) : QJsonValue(v)}));
    });
    h->addWidget(e);
    h->addWidget(s, 1);
    if (a["mappable"].toBool()) h->addWidget(map_button(a, row));
    return row;
}

// A linear colour as the swatch shows it.
QColor swatch_color(const QJsonArray& v) {
    auto enc = [](double x) { return qBound(0, int(std::pow(qBound(0.0, x, 1.0), 1.0 / 2.2) * 255.0 + 0.5), 255); };
    return QColor(enc(v.at(0).toDouble()), enc(v.at(1).toDouble()), enc(v.at(2).toDouble()));
}

// The usual network behind a slot: a file node, its placement, the image, and its connection.
void make_file_texture(const QString& plug, const QString& path) {
    QString file = exec("shadingNode", QJsonObject{{"args", QJsonArray{"file"}}, {"asTexture", true}}).toString();
    QString place = exec("shadingNode", QJsonObject{{"args", QJsonArray{"place2dTexture"}}, {"asUtility", true}}).toString();
    if (file.isEmpty() || place.isEmpty()) return;
    exec("defaultNavigation", QJsonObject{{"connectToExisting", true}, {"source", place}, {"destination", file}});
    set_plug(file + ".fileTextureName", path, "string");
    exec("defaultNavigation", QJsonObject{{"connectToExisting", true}, {"source", file}, {"destination", plug}});
}

const char* kImageFilter = "Images (*.png *.jpg *.jpeg *.tga *.bmp *.tif *.tiff *.psd);;All Files (*)";

// A colour: its swatch, and the slot a texture goes in.
QWidget* color_field(const QJsonObject& a, QWidget* parent) {
    QString plug = a["plug"].toString();
    bool ro = read_only(a);
    auto* w = new QWidget(parent);
    auto* h = new QHBoxLayout(w);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(3);
    auto* swatch = new QPushButton(w);
    swatch->setFixedSize(40, 18);
    bool stored = a["displayColor"].toBool();
    QJsonArray value = a["value"].toArray();
    QColor shown = stored ? QColor::fromRgbF(value.at(0).toDouble(), value.at(1).toDouble(), value.at(2).toDouble()) : swatch_color(value);
    swatch->setStyleSheet(QString("background-color: %1; border: 1px solid palette(shadow);").arg(shown.name()));
    swatch->setEnabled(!ro);
    swatch->setObjectName("colorSwatch");
    QObject::connect(swatch, &QPushButton::clicked, swatch, [swatch, plug, shown, stored] {
        // Owned by the window, not the swatch: the swatch's colour is a style sheet, which its children would take.
        QColor c = QColorDialog::getColor(shown, swatch->window(), "Color");
        if (!c.isValid()) return;
        auto dec = [stored](int x) { return stored ? x / 255.0 : std::pow(x / 255.0, 2.2); };
        exec("setAttr", QJsonObject{{"args", QJsonArray{plug, dec(c.red()), dec(c.green()), dec(c.blue())}}, {"type", "double3"}});
    });
    h->addWidget(swatch);
    // The slider runs the colour's value from black to full, keeping its hue and saturation.
    auto* s = new AttrSlider(Qt::Horizontal, w);
    s->setMinimumWidth(8);
    s->setRange(0, 1000);
    s->setValue(int(shown.valueF() * 1000 + 0.5));
    s->setProperty("plugKey", plug);
    s->setEnabled(!ro);
    auto colour_at = [shown, stored](int p) {
        QColor c = QColor::fromHsvF(qMax(0.0f, shown.hsvHueF()), shown.hsvSaturationF(), p / 1000.0);
        auto dec = [stored](double x) { return stored ? x : std::pow(x, 2.2); };
        return QJsonArray{dec(c.redF()), dec(c.greenF()), dec(c.blueF())};
    };
    QObject::connect(s, &QSlider::sliderPressed, swatch, [plug] { exec("undoInfo", QJsonObject{{"openChunk", true}}); });
    QObject::connect(s, &QSlider::sliderMoved, swatch, [swatch, colour_at, plug, shown](int p) {
        QJsonArray c = colour_at(p);
        swatch->setStyleSheet(QString("background-color: %1; border: 1px solid palette(shadow);").arg(QColor::fromHsvF(qMax(0.0f, shown.hsvHueF()), shown.hsvSaturationF(), p / 1000.0).name()));
        exec_quiet("setAttr", QJsonObject{{"args", QJsonArray{plug, c.at(0), c.at(1), c.at(2)}}, {"type", "double3"}});
        Host::get()->poke();
    });
    QObject::connect(s, &QSlider::sliderReleased, swatch, [s, colour_at, plug] {
        QJsonArray c = colour_at(s->value());
        exec("setAttr", QJsonObject{{"args", QJsonArray{plug, c.at(0), c.at(1), c.at(2)}}, {"type", "double3"}});
        exec("undoInfo", QJsonObject{{"closeChunk", true}});
    });
    h->addWidget(s, 1);
    if (a["mappable"].toBool()) h->addWidget(map_button(a, w));
    return w;
}

// A file node's path: what is written, a browse button, and where it was found.
QWidget* file_field(const QJsonObject& a, QWidget* parent) {
    QString plug = a["plug"].toString();
    auto* w = new QWidget(parent);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    v->setSpacing(2);
    auto* row = new QHBoxLayout();
    row->setSpacing(3);
    auto* e = new QLineEdit(a["value"].toString(), w);
    e->setProperty("plugKey", plug);
    QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
    QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
        if (!e->property("editing").toBool()) return;
        e->setProperty("editing", false);
        set_plug(plug, e->text(), "string");
    });
    auto* browse = new QToolButton(w);
    browse->setIcon(icon("open", 16));
    browse->setToolTip("Browse for an image");
    QObject::connect(browse, &QToolButton::clicked, browse, [browse, plug] {
        QString path = QFileDialog::getOpenFileName(browse, "Choose an image", QString(), kImageFilter);
        if (!path.isEmpty()) set_plug(plug, path, "string");
    });
    row->addWidget(e, 1);
    row->addWidget(browse);
    v->addLayout(row);
    QJsonObject where = query("texturePath", plug.section('.', 0, 0)).toObject();
    QString found = where["resolved"].toString();
    QString written = where["written"].toString();
    if (!written.isEmpty()) {
        auto* note = new QLabel(found.isEmpty() ? QString("Not found") : found == written ? QString() : "Found: " + found, w);
        note->setWordWrap(true);
        if (!note->text().isEmpty()) v->addWidget(note);
    }
    return w;
}

// The override colour's index: its swatch, which opens the palette, and a slider through the 32 entries.
QWidget* color_index_field(const QJsonObject& a, QWidget* parent) {
    QString plug = a["plug"].toString();
    bool ro = read_only(a);
    int index = a["value"].toInt();
    auto* w = new QWidget(parent);
    auto* h = new QHBoxLayout(w);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(4);
    auto* swatch = new QPushButton(w);
    swatch->setObjectName("colorSwatch");
    swatch->setFixedSize(36, 18);
    swatch->setStyleSheet(QString("background-color: %1; border: 1px solid palette(shadow);").arg(palette_color(index).name()));
    swatch->setEnabled(!ro);
    QObject::connect(swatch, &QPushButton::clicked, swatch, [swatch, plug, index] {
        ColorChoice c = choose_display_color(swatch, "Color", index, false, Qt::red);
        if (!c.accepted) return;
        if (c.use_rgb) {
            QString node = plug.left(plug.lastIndexOf('.'));
            set_plug(node + ".overrideRGBColors", true, "bool");
            exec("setAttr", QJsonObject{{"args", QJsonArray{node + ".overrideColorRGB", c.rgb.redF(), c.rgb.greenF(), c.rgb.blueF()}}, {"type", "double3"}});
        } else {
            set_plug(plug, c.index, "int");
        }
    });
    auto* s = new QSlider(Qt::Horizontal, w);
    s->setRange(0, 31);
    s->setValue(index);
    s->setEnabled(!ro);
    auto* number = new QLabel(QString::number(index), w);
    number->setFixedWidth(20);
    QObject::connect(s, &QSlider::sliderMoved, number, [number, swatch](int v) {
        number->setText(QString::number(v));
        swatch->setStyleSheet(QString("background-color: %1; border: 1px solid palette(shadow);").arg(palette_color(v).name()));
    });
    QObject::connect(s, &QSlider::sliderReleased, s, [s, plug] { set_plug(plug, s->value(), "int"); });
    h->addWidget(swatch);
    h->addWidget(number);
    h->addWidget(s, 1);
    return w;
}

QWidget* attr_field(const QJsonObject& a, QWidget* parent) {
    QString type = a["type"].toString(), plug = a["plug"].toString();
    bool ro = read_only(a);
    if (a["attr"].toString() == "overrideColor") return color_index_field(a, parent);
    if (type == "vec3" && a["color"].toBool()) return color_field(a, parent);
    if (type == "string" && a["file"].toBool() && !ro) return file_field(a, parent);
    if (type == "string" && a["attr"].toString() == "layerSetName" && !ro) {
        // A psdFileTex's Link to Layer Set: the document's layer sets, or all of it (#108).
        const QString node = plug.left(plug.lastIndexOf('.'));
        const QString psd = ask("getAttr", QJsonObject{{"args", QJsonArray{node + ".fileTextureName"}}}).toString();
        auto* box = new QComboBox(parent);
        box->addItem("Composite (all layers)", QString());
        if (!psd.isEmpty())
            for (const QJsonValue& v : ask("psdExport", QJsonObject{{"psdFileName", psd}}).toArray()) box->addItem(v.toString(), v.toString());
        const QString cur = a["value"].toString();
        int at = box->findData(cur);
        if (at < 0 && !cur.isEmpty()) {
            box->addItem(cur + " (not in the file)", cur);
            at = box->count() - 1;
        }
        box->setCurrentIndex(qMax(0, at));
        QObject::connect(box, &QComboBox::activated, box, [box, plug](int i) { set_plug(plug, box->itemData(i).toString(), "string"); });
        return box;
    }
    if (type == "bool") {
        auto* cb = new QCheckBox(parent);
        cb->setChecked(a["value"].toBool());
        cb->setEnabled(!ro);
        QObject::connect(cb, &QCheckBox::toggled, cb, [plug](bool on) { set_plug(plug, on, "bool"); });
        return cb;
    }
    if (type == "enum") {
        auto* box = new QComboBox(parent);
        for (const QJsonValue& n : a["enum"].toArray()) box->addItem(n.toString());
        box->setCurrentIndex(a["value"].toInt());
        box->setEnabled(!ro);
        QObject::connect(box, &QComboBox::activated, box, [plug](int i) { set_plug(plug, i, "int"); });
        return box;
    }
    if (type == "vec3") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(3);
        QJsonArray v = a["value"].toArray();
        QJsonArray kids = a["children"].toArray();
        QJsonArray childLocked = a["childLocked"].toArray();
        QJsonArray childConnected = a["childConnected"].toArray();
        for (int k = 0; k < (a["pair"].toBool() ? 2 : 3); ++k) {
            auto* e = new QLineEdit(fmt3(v.at(k).toDouble()), w);
            // Three to a row, thin padding from the stylesheet so a value shows whole at the panel's usual width.
            e->setMinimumWidth(34);
            // A locked part takes the locked shade. A driven or output-only part is read-only but is not locked, so it
            // keeps its own look instead of reading as a lock (#212 review).
            const bool locked = a["locked"].toBool() || childLocked.at(k).toBool();
            e->setReadOnly(ro || locked);
            if (a["childKeyed"].toArray().at(k).toBool()) e->setObjectName("keyedField");
            else if (childConnected.at(k).toBool()) e->setObjectName("connectedField");
            if (locked) e->setObjectName("lockedField");
            QString child = kids.at(k).toString();
            if (child.isEmpty()) child = a["attr"].toString() + QString("XYZ")[k];
            QString cp = plug.left(plug.lastIndexOf('.')) + "." + child;
            e->setProperty("plugKey", cp);
            QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
            QObject::connect(e, &QLineEdit::editingFinished, e, [e, cp] {
                if (!e->property("editing").toBool()) return;
                e->setProperty("editing", false);
                set_plug(cp, e->text().toDouble(), "float");
            });
            h->addWidget(e);
        }
        return w;
    }
    if (type == "float" || type == "int") {
        if (a["min"].isDouble() && a["max"].isDouble() && a["max"].toDouble() > a["min"].toDouble()) return ranged_field(a, a["min"].toDouble(), a["max"].toDouble(), parent);
        auto* e = new QLineEdit(type == "int" ? QString::number(a["value"].toInteger()) : fmt3(a["value"].toDouble()), parent);
        e->setProperty("plugKey", plug);
        e->setReadOnly(ro);
        bool is_int = type == "int";
        QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
        QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug, is_int] {
            if (!e->property("editing").toBool()) return;
            e->setProperty("editing", false);
            set_plug(plug, is_int ? QJsonValue(qint64(e->text().toDouble())) : QJsonValue(e->text().toDouble()), "float");
        });
        if (a["keyed"].toBool()) e->setObjectName("keyedField");
        else if (a["connected"].toBool()) e->setObjectName("connectedField");
        if (a["locked"].toBool()) e->setObjectName("lockedField");
        if (!a["mappable"].toBool()) return e;
        // A number a texture can drive: the field, then the map button at the row's end where the sliders' are.
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        e->setParent(w);
        e->setFixedWidth(68);
        h->addWidget(e);
        h->addStretch(1);
        h->addWidget(map_button(a, w));
        return w;
    }
    if (type == "string") {
        auto* e = new QLineEdit(a["value"].toString(), parent);
        e->setProperty("plugKey", plug);
        e->setReadOnly(ro);
        if (a["locked"].toBool()) e->setObjectName("lockedField");
        QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
        QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
            if (!e->property("editing").toBool()) return;
            e->setProperty("editing", false);
            set_plug(plug, e->text(), "string");
        });
        return e;
    }
    return new QLabel(type == "matrix" ? "(matrix)" : "", parent);
}

// A row's name in a column of one width down the whole tab, so the fields line up from frame to frame. The column is
// as wide as the tab's longest name needs, within limits (see fit_labels); a name longer still loses its start, and the
// whole of it is in the tooltip.
class RowLabel : public QLabel {
public:
    RowLabel(const QString& text, QWidget* parent) : QLabel(text, parent) {
        setFixedWidth(160);
        setToolTip(text);
        setProperty("rowLabel", true);
    }

    // A locked row is drawn dimmed, as the reference dims it (#212), and still takes the unlock menu.
    void setLocked(bool on) {
        locked_ = on;
        update();
    }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        const bool dim = locked_ || !isEnabled();
        p.setPen(palette().color(dim ? QPalette::Disabled : QPalette::Active, QPalette::WindowText));
        p.drawText(rect(), Qt::AlignRight | Qt::AlignVCenter, fontMetrics().elidedText(text(), Qt::ElideLeft, width()));
    }

private:
    bool locked_ = false;
};

// Widens a tab's name column to its longest name, between the default and a cap, so names read whole.
void fit_labels(QWidget* body) {
    // By a property: findChildren needs Q_OBJECT on the class it looks for, and newer Qt refuses to build without it.
    QList<QLabel*> labels = body->findChildren<QLabel*>();
    labels.removeIf([](QLabel* l) { return !l->property("rowLabel").toBool(); });
    int need = 160;
    for (QLabel* l : labels) need = std::max(need, l->fontMetrics().horizontalAdvance(l->text()) + 8);
    need = std::min(need, 240);
    for (QLabel* l : labels) l->setFixedWidth(need);
}

void style_ae_menu(QMenu* m);

// What a right click on an attribute gives, for what the attribute can take: its connection, its key, its texture.
void attach_attr_menu(QWidget* target, const QJsonObject& a, QWidget* field) {
    QPointer<QWidget> fieldp = field;
    const QString plug = a["plug"].toString(), source = a["source"].toString();
    const bool mappable = a["mappable"].toBool(), connected = a["connected"].toBool(), output = a["output"].toBool(), keyed = a["keyed"].toBool();
    const bool colour = a["type"].toString() == "vec3" && a["color"].toBool();
    target->setContextMenuPolicy(Qt::CustomContextMenu);
    QObject::connect(target, &QWidget::customContextMenuRequested, target, [=](const QPoint& at) {
        QMenu m;
        style_ae_menu(&m);
        if (!source.isEmpty()) {
            m.addAction(source + "...", [source] { exec("select", args(QJsonArray{source})); });
            m.addSeparator();
        }
        if (!output) m.addAction("Set Key", [plug] { exec("setKeyframe", args(QJsonArray{plug})); })->setEnabled(!connected || keyed);
        if (connected) m.addAction("Break Connection", [plug] { break_connection(plug); });
        if (mappable && !connected) m.addAction("Create New Texture...", [target, plug] { create_render_node(target->window(), plug); });
        if (colour && !connected && !output) {
            m.addAction("Color Chooser...", [fieldp] {
                if (!fieldp) return;
                if (auto* b = fieldp->findChild<QPushButton*>("colorSwatch")) b->click();
            });
        }
        if (!output) {
            const bool locked = ask("getAttr", QJsonObject{{"args", QJsonArray{plug}}, {"lock", true}}).toBool();
            m.addAction(locked ? "Unlock Attribute" : "Lock Attribute", [plug, locked] { exec("setAttr", QJsonObject{{"args", QJsonArray{plug}}, {"lock", !locked}}); });
        }
        if (!m.isEmpty()) m.exec(target->mapToGlobal(at));
    });
}

// A row's name. A right click on it gives what the reference's gives there, for what this attribute can take.
QLabel* attr_label(const QString& text, const QJsonObject& a, QWidget* parent, QWidget* field_widget) {
    auto* l = new RowLabel(text, parent);
    l->setLocked(a["locked"].toBool());
    attach_attr_menu(l, a, field_widget);
    return l;
}

// A standard surface's rows as the reference names them: inside the Specular frame, specularRoughness is Roughness and
// the frame's own weight is Weight.
QString material_label(const QString& section, const QString& attr, const QString& nice) {
    static const QHash<QString, QString> named{{"transmissionDispersion", "Dispersion Abbe"}, {"transmission_dispersion", "Dispersion Abbe"}, {"normalCamera", "Bump Mapping"},
                                               {"tangentUCamera", "Anisotropy Tangent"}, {"tangent", "Anisotropy Tangent"}, {"coatNormal", "Normal"}, {"coat_normal", "Normal"}};
    if (named.contains(attr)) return named.value(attr);
    QString camel = section;
    camel.remove(' ');
    if (camel.isEmpty()) return nice;
    camel[0] = camel[0].toLower();
    QString snake = section.toLower().replace(' ', '_');
    QString rest;
    if (attr == camel || attr == snake) return "Weight";
    if (attr.startsWith(camel) && attr.size() > camel.size() && attr[camel.size()].isUpper()) rest = attr.mid(camel.size());
    else if (attr.startsWith(snake + "_")) rest = attr.mid(snake.size() + 1);
    else return nice;
    if (rest == "IOR") return rest;
    QString out;
    for (int i = 0; i < rest.size(); ++i) {
        if (rest[i] == '_') {
            out += ' ';
        } else if (i == 0 || rest[i - 1] == '_') {
            out += rest[i].toUpper();
        } else {
            if (rest[i].isUpper() && !rest[i - 1].isUpper()) out += ' ';
            out += rest[i];
        }
    }
    return out;
}

// A section's open state, per node (by its UUID, so a rename keeps it) and section path, and each node's scroll
// position. The template only says how a section starts; once the user opens or closes it, that is what rebuilds keep.
QHash<QString, QHash<QString, bool>>& section_states() {
    static QHash<QString, QHash<QString, bool>> s;
    return s;
}

bool section_open(const QString& owner, const QString& key, bool by_default) {
    if (owner.isEmpty()) return by_default;
    const auto it = section_states().constFind(owner);
    if (it == section_states().constEnd() || !it->contains(key)) return by_default;
    return it->value(key);
}

void remember_section(const QString& owner, const QString& key, bool open) {
    if (!owner.isEmpty()) section_states()[owner][key] = open;
}

QHash<QString, int>& scroll_states() {
    static QHash<QString, int> s;
    return s;
}

bool default_closed_section(const QString& title) {
    static const QSet<QString> s{"Node Behaviour", "Limit Information", "Extra Attributes", "Special Effects", "Matte Opacity",
                                 "Raytrace Options", "Light Effects", "Shadows", "Harold", "Harold Subdivision", "Harold Displacement",
                                 "Drawing Overrides", "Display", "Object Display", "Transmission", "Subsurface", "Coat", "Sheen",
                                 "Emission", "Thin Film", "Geometry", "Advanced", "File Attributes", "Color Balance", "Effects",
                                 "2d Texture Placement"};
    return s.contains(title);
}

// The Attribute Editor's layout follows the reference's templates: sections that nest, and open or closed by default.
struct AeRow {
    QString attr;
    QString label;
    // The control a row draws when the node has no such attribute: still in place, greyed, as the reference's feature
    // being absent leaves it. Empty when the row needs the real attribute.
    QString fallback;
    QString kind; // "", "check", "float", "int", "vec3", "combo", "text", "color", "matrix", "check2", "separator"
    QString extra; // the second attribute of a check2 row, or the combo's entries separated by "|"
};

struct AeBox {
    QString title;
    QList<AeRow> rows;
    QList<AeBox> subs;
    int open = 0;
    QString limit; // "Trans", "Rot" or "Scale": the group draws the reference's Min, Current and Max rows.
};

AeRow row(const char* attr) {
    AeRow r;
    r.attr = attr;
    return r;
}

AeRow shown(const char* attr, const char* label, const char* kind, const char* fallback = "") {
    AeRow r;
    r.attr = attr;
    r.label = label;
    r.kind = kind;
    r.fallback = fallback;
    return r;
}

AeBox box(const char* title, std::initializer_list<AeRow> rows, int open = 0) {
    AeBox b;
    b.title = title;
    b.rows = QList<AeRow>(rows);
    b.open = open;
    return b;
}

// The reference's labels for the parts every shape shares.
const char* const kDisplayRows = "visibility template lodVisibility hideOnPlayback hiddenInOutliner useOutlinerColor outlinerColor";

QList<AeBox> transform_boxes() {
    QList<AeBox> l;
    l << box("Transform Attributes",
             {row("translate"), row("rotate"), row("scale"), row("shear"), row("rotateOrder"), row("rotateAxis"), row("inheritsTransform"),
              shown("offsetParentMatrix", "Offset Parent Matrix", "matrix")},
             1);
    AeBox pivots = box("Pivots", {row("displayRotatePivot"), row("displayScalePivot")});
    pivots.subs << box("Local Space", {shown("rotatePivot", "Local Rotate Pivot", "vec3"), shown("scalePivot", "Local Scale Pivot", "vec3")});
    pivots.subs << box("World Space",
                       {shown("", "World Rotate Pivot", "vec3"), shown("", "World Scale Pivot", "vec3")}, 1);
    l << pivots;
    AeBox limits = box("Limit Information", {});
    AeBox trans = box("Translate", {});
    trans.limit = "Trans";
    AeBox rot = box("Rotate", {});
    rot.limit = "Rot";
    rot.open = 1;
    AeBox scale = box("Scale", {});
    scale.limit = "Scale";
    limits.subs << trans << rot << scale;
    l << limits;
    AeBox display = box("Display",
                        {row("displayHandle"), row("displayLocalAxis"), shown("selectHandle", "Selection Handle", "vec3"),
                         shown("showManipDefault", "Show Manip Default", "combo", "None"), shown("manipHistoryNode", "Manip History Node", "text"),
                         shown("visibility", "Visibility", "check2"), row("lodVisibility"), row("hideOnPlayback"), row("hiddenInOutliner"),
                         row("useOutlinerColor"), row("outlinerColor")});
    display.rows[5].extra = "template";
    display.subs << box("Ghosting Information", {row("ghosting"), row("ghostPreSteps"), row("ghostPostSteps"), row("ghostStepSize")});
    display.subs << box("Bounding Box Information", {shown("boundingBoxMin", "Bounding Box Min", "vec3"), shown("boundingBoxMax", "Bounding Box Max", "vec3")});
    AeBox overrides = box("Drawing Overrides",
                          {shown("overrideEnabled", "Enable Overrides", "check"), row("overrideDisplayType"), row("overrideLevelOfDetail"), row("overrideShading"),
                           row("overrideTexturing"), row("overridePlayback"), row("overrideVisibility"), row("overrideRGBColors"), row("overrideColor"),
                           shown("overrideColorA", "Color Alpha", "float")});
    overrides.open = -1;
    display.subs << overrides;
    display.subs << box("Selection", {row("selectionChildHighlighting")});
    l << display;
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

QList<AeBox> joint_boxes() {
    QList<AeBox> l;
    l << transform_boxes().first();
    AeBox joint = box("Joint", {row("drawStyle"), row("radius"), shown("", "Degrees of Freedom", "freedom"), row("stiffness"),
                                row("preferredAngle"), row("jointOrient"), row("segmentScaleCompensate")},
                      1);
    joint.subs << box("Joint Labelling", {row("side"), row("type"), shown("otherType", "Other Type", "text"), row("drawLabel")}, 1);
    l << joint;
    QList<AeBox> rest = transform_boxes();
    rest.removeFirst();
    rest.removeFirst(); // no Pivots on a joint
    l << rest;
    return l;
}

QList<AeBox> mesh_boxes() {
    QList<AeBox> l;
    l << box("Tessellation Attributes",
             {shown("maxTriangles", "Max Triangles", "int"), shown("useMaxSubdivisions", "Use Max Subdivisions", "check"),
              shown("maxSubd", "Max Subd", "int"), shown("useMinScreen", "Use Min Screen", "check"),
              shown("minScreen", "Min Screen", "float"), shown("useMaxUV", "Use Max UV", "check"), shown("maxUV", "Max UV", "float"),
              shown("useMinEdgeLength", "Use Min Edge Length", "check"), shown("minEdgeLength", "Min Edge Length", "float"),
              shown("useMaxEdgeLength", "Use Max Edge Length", "check"), shown("maxEdgeLength", "Max Edge Length", "float")});
    l << box("Mesh Component Display",
             {shown("displayImmediate", "Display Immediate", "check"), row("displayVertices"),
              shown("vertexSize", "Vertex Size", "float"), shown("vertexBackfaceCulling", "Vertex Backface Culling", "check"),
              shown("backfaceCulling", "Backface Culling", "combo", "off"), shown("", "", "separator"),
              shown("displayEdges", "Display Edges", "combo", "Standard"), row("displayBorders"), shown("borderWidth", "Border Width", "int"),
              shown("", "", "separator"), row("displayCenter"), row("displayTriangles"), shown("displayUVs", "Display UVs", "check"),
              shown("uvSize", "UV Size", "float"), shown("displayNonPlanar", "Display Non Planar", "check"),
              shown("displayInvisibleFaces", "Display Invisible Faces", "check"), shown("", "", "separator"), shown("displayColors", "Display Colors", "check"),
              shown("displayColorChannel", "Display Color Channel", "combo", "Ambient+Diffuse"),
              shown("materialBlend", "Material Blend", "combo", "overwrite"), shown("", "Channels", "channels"),
              shown("displayColorAsGreyScale", "Display color as greyscale", "check"),
              shown("displayAlphaAsGreyScale", "Display alpha as greyscale", "check"), shown("", "", "separator"),
              shown("displayNormal", "Display Normal", "check"), shown("displayTangent", "Display Tangent", "check"),
              shown("normalSize", "Normal Size", "float"), shown("normalType", "Normal Type", "combo", "face")});
    l << box("Mesh Controls",
             {shown("currentUVSet", "Current UVSet", "combo", "map1"), shown("currentColorSet", "Current Color Set", "combo", ""),
              shown("motionVectorColorSet", "Motion Vector Color Set", "combo", ""), shown("vertexColorSource", "Vertex Color Source", "combo", "None"),
              shown("weightsVisualization", "Weights Visualization", "combo", "off"), shown("allowTopologyMod", "Allow Topology Mod", "check"),
              shown("quadSplit", "Quad Split", "combo", "Best Shape"),
              shown("vertexNormalMethod", "Vertex Normal Method", "combo", "Angle And Area Weighted")});
    l << box("Tangent Space",
             {shown("", "Coordinate System:", "radio2", "0"), shown("", "UV Winding Order:", "radio2", "0"),
              shown("tangentSmoothingAngle", "Tangent Smoothing Angle", "float"), shown("tangentNormalThreshold", "Tangent Normal Threshold", "float")});
    AeBox smooth = box("Smooth Mesh", {shown("displaySmoothMesh", "Smooth Mesh Preview", "check")});
    smooth.rows << shown("", "Display", "radio2", "0");
    smooth.rows.last().extra = "Cage + Smooth Mesh|Smooth Mesh";
    smooth.rows << shown("useGlobalSmoothDrawType", "Subdivision Method", "checktext", "Use Global (OpenSubdiv Catmull-Clark)");
    smooth.rows << shown("smoothDrawType", "", "combo", "OpenSubdiv Catmull-Clark");
    smooth.subs << box("Subdivision Levels", {shown("displaySubdComps", "Display Subdivisions", "check"), row("smoothLevel"),
                                              shown("useSmoothPreviewForRender", "Use Preview Level for Rendering", "check"), row("renderSmoothLevel")},
                       1);
    smooth.subs << box("OpenSubdiv Controls",
                       {shown("osdVertBoundary", "Vertex Boundary", "combo"), shown("osdFvarBoundary", "UV Boundary Smoothing", "combo"),
                        shown("osdPropagateCorners", "Propagate UV Corners", "check"), shown("osdSmoothTriangles", "Smooth Triangles", "check"),
                        shown("osdCreaseMethod", "Crease Method", "combo"), shown("enableOpenCL", "OpenCL Acceleration", "check")});
    smooth.subs << box("Catmull-Clark Controls",
                       {shown("boundaryRule", "Boundary Rules", "combo"), row("continuity"), shown("smoothUVs", "Smooth UVs", "check"),
                        shown("propagateEdgeHardness", "Propagate Edge Hardness", "check")});
    l << smooth;
    l << box("Displacement Map",
             {shown("featureDisplacement", "Feature Displacement", "check"), shown("initialSampleRate", "Initial Sample Rate", "int"),
              shown("extraSampleRate", "Extra Sample Rate", "int"), shown("textureThreshold", "Texture Threshold", "float"),
              shown("normalThreshold", "Normal Threshold", "float"), shown("", "", "separator"),
              shown("boundingBoxScale", "Bounding Box Scale", "vec3"), shown("", "Calculate Bounding Box Scale", "button")});
    AeBox render = box("Render Stats",
                       {row("castsShadows"), row("receiveShadows"), shown("holdOut", "Hold-Out", "check"), row("motionBlur"),
                        row("primaryVisibility"), row("smoothShading"), row("visibleInReflections"), row("visibleInRefractions"),
                        row("doubleSided"), shown("opposite", "Opposite", "check"), shown("", "", "separator"),
                        shown("geometryAntialiasingOverride", "Geometry Anti-aliasing Override", "check"),
                        shown("antialiasingLevel", "Anti-aliasing Level", "int"), shown("shadingSamplesOverride", "Shading Samples Override", "check"),
                        shown("shadingSamples", "Shading Samples", "int"), shown("maxShadingSamples", "Max Shading Samples", "int")});
    render.subs << box("3D Motion Blur", {shown("", "Max Visib. Samples", "int"), row("motionBlur")});
    l << render;
    AeBox object = box("Object Display", {});
    object.rows << shown("visibility", "Visibility", "check2");
    object.rows.last().extra = "template";
    object.rows << shown("lodVisibility", "LOD Visibility", "check2");
    object.rows << shown("ignoreHwShader", "Ignore Hardware Shader", "check2");
    object.rows << shown("hideOnPlayback", "Hide on playback", "check2");
    object.rows.last().extra = "intermediateObject";
    object.rows << shown("hiddenInOutliner", "Hidden In Outliner", "check2");
    object.rows << shown("useOutlinerColor", "Use Outliner Color", "check2");
    object.rows << shown("outlinerColor", "Outliner Color", "color");
    AeBox ghost = box("Ghosting Information", {row("ghosting"), row("ghostPreSteps"), row("ghostPostSteps"), row("ghostStepSize")});
    object.subs << ghost;
    object.subs << box("Bounding Box Information", {shown("boundingBoxMin", "Bounding Box Min", "vec3"), shown("boundingBoxMax", "Bounding Box Max", "vec3")});
    AeBox meshOverrides;
    meshOverrides.title = "Drawing Overrides";
    meshOverrides.rows = transform_boxes().at(3).subs.at(2).rows;
    meshOverrides.open = -1;
    object.subs << meshOverrides;
    object.subs << box("Selection", {row("selectionChildHighlighting")});
    l << object;
    l << box("Component Tags", {});
    l << box("Deformation", {});
    l << box("Harold", {});
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

QList<AeBox> blend_boxes() {
    QList<AeBox> l;
    l << box("Blend Shape Attributes", {row("origin")}, 1);
    AeBox weights = box("Weight", {shown("supportNegativeWeights", "Support Negative Weights", "check")}, 1);
    l << weights;
    AeBox deformer = box("Deformer Attributes", {row("envelope"), shown("", "", "manageGeometry"), shown("", "", "inputTable")});
    deformer.subs << box("GPU", {shown("blockGPU", "Block GPU", "check")});
    l << deformer;
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

QList<AeBox> skin_boxes() {
    QList<AeBox> l;
    AeBox skin = box("Smooth Skin Attributes",
                     {shown("skinningMethod", "Skinning Method", "combo", "Classic Linear|Dual Quaternion|Weight Blended"),
                      shown("useComponents", "Use Components", "check"), shown("deformUserNormals", "Deform User Normals", "check", "1"),
                      shown("relativeSpaceMode", "Relative Space Mode", "combo", "World|Object|Custom"),
                      shown("relativeSpaceMatrix", "Relative Space Matrix", "matrix"),
                      shown("dqsSupportNonRigid", "Support Non-Rigid Transformations", "check"), shown("dqsScale", "Scale", "vec3"),
                      shown("", "", "separator"), shown("normalizeWeights", "Normalize Weights", "combo", "None|Interactive"),
                      shown("weightDistribution", "Weight Distribution", "combo", "Neighbors|Distance"), shown("maxInfluences", "Max Influences", "int"),
                      shown("maintainMaxInfluences", "Maintain Max Influences", "check")},
                     1);
    l << skin;
    AeBox deformer = box("Deformer Attributes", {row("envelope"), shown("", "", "manageGeometry"), shown("", "", "inputTable")});
    deformer.subs << box("GPU", {shown("blockGPU", "Block GPU", "check")});
    l << deformer;
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// A deformer's tab, from the cluster, lattice, nonlinear, sculpt, soft modification and wrap templates.
QList<AeBox> deformer_boxes(const QString& type) {
    QList<AeBox> l;
    const bool nonlinear = type == "nonLinear" || type == "deformBend" || type == "deformTwist" || type == "deformSquash" ||
                           type == "deformFlare" || type == "deformSine" || type == "deformWave";
    const bool handle = type == "clusterHandle" || type == "softModHandle";
    if (nonlinear) {
        QList<AeRow> rows;
        if (type == "nonLinear" || type == "deformTwist" || type == "deformSquash" || type == "deformFlare")
            rows << shown("handleWidth", "Handle Width", "float");
        l << box("Handle Display", {}, 0);
        l.last().rows = rows;
    } else if (handle) {
        l << box("Cluster Attributes", {shown("origin", "Origin", "float")}, 1);
        if (type == "softModHandle") l.last().title = "Soft Modification Attributes";
        l << box("Weighted Node", {shown("", "Use Weighted Node", "check"), shown("", "Weighted Node", "text")}, 0);
    } else if (type == "cluster") {
        l << box("Cluster Attributes", {row("relative"), shown("angleInterpolation", "Angle Interpolation", "combo", "None|Quaternion")}, 1);
        l << box("Deformer Attributes", {row("envelope"), shown("", "", "manageGeometry"), shown("", "", "inputTable")});
    } else if (type == "lattice" || type == "baseLattice") {
        l << box("Lattice History", {row("sDivisions"), row("tDivisions"), row("uDivisions")}, 1);
    } else if (type == "sculpt") {
        l << box("Sculpt History",
                 {shown("mode", "Mode", "combo", "Flip|Project|Stretch"), shown("insideMode", "Inside Mode", "check"),
                  shown("maximumDisplacement", "Maximum Displacement", "slider", "0,10"), shown("dropoffType", "Dropoff Type", "combo", "Linear|Cubic"),
                  shown("dropoffDistance", "Dropoff Distance", "slider", "0,10")},
                 1);
        l << box("Deformer Attributes", {row("envelope"), shown("", "", "manageGeometry"), shown("", "", "inputTable")});
    } else {
        l << box("Deformer Attributes", {row("envelope"), shown("", "", "manageGeometry"), shown("", "", "inputTable")});
    }
    AeBox display = box("Object Display",
                        {shown("visibility", "Visibility", "check2"), shown("lodVisibility", "LOD Visibility", "check2"),
                         shown("hideOnPlayback", "Hide on playback", "check2"), shown("hiddenInOutliner", "Hidden In Outliner", "check2"),
                         shown("useOutlinerColor", "Use Outliner Color", "check2"), shown("outlinerColor", "Outliner Color", "color")});
    display.rows[0].extra = "template";
    l << display;
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// A NURBS curve's tab, from AEnurbsCurveTemplate.
QList<AeBox> curve_boxes() {
    QList<AeBox> l;
    l << box("NURBS Curve History",
             {shown("", "Lock Length", "check"), row("minValue"), row("maxValue"), row("spans"),
              shown("form", "Form", "combo", "Open|Closed|Periodic"), row("degree")},
             1);
    l << box("Component Display", {row("dispCV"), row("dispEP"), row("dispHull"), shown("dispGeometry", "Display Geometry", "check")});
    AeBox display = box("Object Display",
                        {shown("visibility", "Visibility", "check2"), shown("lodVisibility", "LOD Visibility", "check2"),
                         shown("hideOnPlayback", "Hide on playback", "check2"), shown("hiddenInOutliner", "Hidden In Outliner", "check2"),
                         shown("useOutlinerColor", "Use Outliner Color", "check2"), shown("outlinerColor", "Outliner Color", "color"),
                         row("intermediateObject")});
    display.rows[0].extra = "template";
    display.subs << box("Bounding Box Information", {shown("boundingBoxMin", "Bounding Box Min", "vec3"), shown("boundingBoxMax", "Bounding Box Max", "vec3")});
    AeBox overrides;
    overrides.title = "Drawing Overrides";
    overrides.rows = transform_boxes().at(3).subs.at(2).rows;
    overrides.open = -1;
    display.subs << overrides;
    display.subs << box("Selection", {row("selectionChildHighlighting")});
    l << display;
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// A NURBS surface's tab, from AEnurbsSurfaceTemplate.
QList<AeBox> surface_boxes() {
    QList<AeBox> l;
    l << box("NURBS Surface History",
             {shown("minValueU", "Min/Max Range U", "pair", "maxValueU"), shown("minValueV", "Min/Max Range V", "pair", "maxValueV"),
              shown("spansU", "Spans", "pair", "spansV"), shown("degreeU", "Degree", "pair", "degreeV"),
              shown("formU", "Form U", "combo", "Open|Closed|Periodic"), shown("formV", "Form V", "combo", "Open|Closed|Periodic")},
             1);
    l << box("NURBS Surface Display",
             {shown("curvePrecision", "Curve Precision", "int"), shown("curvePrecisionShaded", "Curve Precision Shaded", "int"),
              shown("simplifyMode", "Simplify Mode", "combo", "None|Simplify"), shown("simplifyU", "Simplify U", "int"),
              shown("simplifyV", "Simplify V", "int"), row("divisionsU"), row("divisionsV"),
              shown("", "Normals Display Scale", "float")});
    AeBox tess = box("Tessellation",
                     {shown("", "Enable Advanced Tessellation", "check"), shown("displayRenderTessellation", "Display Render Tessellation", "check")});
    tess.subs << box("Simple Tessellation Options",
                     {shown("curvatureTolerance", "Curvature Tolerance", "float"), shown("uDivisionsFactor", "U Divisions Factor", "float"),
                      shown("vDivisionsFactor", "V Divisions Factor", "float")});
    AeBox advanced;
    advanced.title = "Advanced Tessellation";
    advanced.rows = {shown("", "Enable Advanced Tessellation", "check")};
    advanced.subs << box("Primary Tessellation Attributes",
                         {shown("modeU", "U Type", "combo", "Per Span # of Iso Params|Per Surface # of Iso Params"),
                          shown("numberU", "Number U", "int"), shown("modeV", "V Type", "combo", "Per Span # of Iso Params|Per Surface # of Iso Params"),
                          shown("numberV", "Number V", "int")},
                         1);
    advanced.subs << box("Secondary Tessellation Attributes",
                         {shown("useChordHeight", "Use Chord Height", "check"), shown("chordHeight", "Chord Height", "float"),
                          shown("useChordHeightRatio", "Use Chord Height Ratio", "check"), shown("chordHeightRatio", "Chord Height Ratio", "float"),
                          shown("useMinScreen", "Use Min Screen", "check"), shown("minScreen", "Min Screen", "float")},
                         1);
    advanced.subs << box("Common Tessellation Options",
                         {shown("smoothEdge", "Smooth Edge", "check"), shown("smoothEdgeRatio", "Smooth Edge Ratio", "float"),
                          shown("edgeSwap", "Edge Swap", "check")});
    tess.subs << advanced;
    l << tess;
    l << box("Render Stats", {row("castsShadows"), row("receiveShadows"), row("primaryVisibility"), row("visibleInReflections"),
                              row("visibleInRefractions"), row("doubleSided"), row("smoothShading"), row("motionBlur")});
    AeBox display = box("Object Display",
                        {shown("visibility", "Visibility", "check2"), shown("lodVisibility", "LOD Visibility", "check2"),
                         shown("hideOnPlayback", "Hide on playback", "check2"), shown("hiddenInOutliner", "Hidden In Outliner", "check2"),
                         shown("useOutlinerColor", "Use Outliner Color", "check2"), shown("outlinerColor", "Outliner Color", "color"),
                         row("intermediateObject")});
    display.rows[0].extra = "template";
    AeBox overrides;
    overrides.title = "Drawing Overrides";
    overrides.rows = transform_boxes().at(3).subs.at(2).rows;
    overrides.open = -1;
    display.subs << overrides;
    display.subs << box("Selection", {row("selectionChildHighlighting")});
    l << display;
    l << box("Harold", {});
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// The OpenPBR surface, from AEopenPBRSurfaceTemplate.
QList<AeBox> openpbr_boxes() {
    QList<AeBox> l;
    l << box("Base",
             {shown("baseWeight", "Weight", "slider", "0,1"), shown("baseColor", "Color", "color"),
              shown("baseDiffuseRoughness", "Diffuse Roughness", "slider", "0,1"), shown("baseMetalness", "Metalness", "slider", "0,1")},
             1);
    l << box("Specular",
             {shown("specularWeight", "Weight", "slider", "0,1"), shown("specularColor", "Color", "color"),
              shown("specularIOR", "IOR", "slider", "1,3"), shown("specularRoughness", "Roughness", "slider", "0,1"),
              shown("specularRoughnessAnisotropy", "Anisotropy", "slider", "0,1"),
              shown("", "Anisotropy Tangent", "text")},
             1);
    l << box("Transmission",
             {shown("transmissionWeight", "Weight", "slider", "0,1"), shown("transmissionColor", "Color", "color"),
              shown("transmissionDepth", "Depth", "slider", "0,10"), shown("transmissionScatter", "Scatter", "color"),
              shown("transmissionScatterAnisotropy", "Scatter Anisotropy", "slider", "-1,1"),
              shown("transmissionDispersionScale", "Dispersion Scale", "slider", "0,1"),
              shown("transmissionDispersionAbbeNumber", "Abbe Number", "slider", "0,100")});
    l << box("Subsurface",
             {shown("subsurfaceWeight", "Weight", "slider", "0,1"), shown("subsurfaceColor", "Color", "color"),
              shown("subsurfaceRadius", "Radius", "slider", "0,10"), shown("subsurfaceRadiusScale", "Radius Scale", "vec3"),
              shown("subsurfaceScatterAnisotropy", "Scatter Anisotropy", "slider", "-1,1")});
    l << box("Fuzz",
             {shown("fuzzWeight", "Weight", "slider", "0,1"), shown("fuzzColor", "Color", "color"),
              shown("fuzzRoughness", "Roughness", "slider", "0,1")});
    l << box("Coat",
             {shown("coatWeight", "Weight", "slider", "0,1"), shown("coatColor", "Color", "color"),
              shown("coatDarkening", "Darkening", "slider", "0,1"), shown("coatIOR", "IOR", "slider", "1,3"),
              shown("coatRoughness", "Roughness", "slider", "0,1"), shown("coatRoughnessAnisotropy", "Anisotropy", "slider", "0,1"),
              shown("", "Anisotropy Tangent", "text"), shown("", "Bump Mapping", "text")});
    l << box("Thin Film", {shown("thinFilmWeight", "Weight", "slider", "0,1"), shown("thinFilmThickness", "Thickness", "slider", "0,10"),
                           shown("thinFilmIOR", "IOR", "slider", "1,3")});
    l << box("Emission", {shown("emissionLuminance", "Luminance", "slider", "0,100"), shown("emissionColor", "Color", "color")});
    l << box("Geometry",
             {shown("geometryThinWalled", "Thin Walled", "check"), shown("geometryOpacity", "Opacity", "slider", "0,1"),
              shown("", "Bump Mapping", "text")});
    l << box("Hardware Shading", {shown("", "Hardware Shader", "combo", "None")});
    l << box("Harold", {});
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// The camera's tab, from AEcameraTemplateCommon.
QList<AeBox> camera_boxes() {
    QList<AeBox> l;
    l << box("Camera Attributes",
             {shown("", "Controls", "combo", "Camera|Camera and Aim|Camera, Aim and Up"),
              shown("", "Angle of View", "float"), shown("focalLength", "Focal Length", "slider", "1,200"),
              shown("cameraScale", "Camera Scale", "float"), shown("bestFitClippingPlanes", "Auto Render Clip Plane", "check"),
              shown("nearClipPlane", "Near Clip Plane", "float"), shown("farClipPlane", "Far Clip Plane", "float")},
             1);
    l << box("Frustum Display Controls",
             {shown("displayCameraNearClip", "Display Camera Near Clip", "check"), shown("displayCameraFarClip", "Display Camera Far Clip", "check"),
              shown("displayCameraFrustum", "Display Camera Frustum", "check")});
    l << box("Film Back",
             {shown("", "Film Gate", "combo", "User|16mm|35mm Academy|35mm Full Aperture"),
              shown("horizontalFilmAperture", "Camera Aperture (inch)", "pair", "verticalFilmAperture"),
              shown("", "Camera Aperture (mm)", "pair"), shown("", "Film Aspect Ratio", "float"),
              shown("lensSqueezeRatio", "Lens Squeeze Ratio", "float"), shown("filmFit", "Fit Resolution Gate", "combo", "Fill|Horizontal|Vertical|Overscan"),
              shown("filmFitOffset", "Film Fit Offset", "float"), shown("horizontalFilmOffset", "Film Offset", "pair", "verticalFilmOffset"),
              shown("", "", "separator"), shown("shakeEnabled", "Shake Enabled", "check"),
              shown("horizontalShake", "Shake", "pair", "verticalShake"), shown("", "Shake Overscan Enabled", "check"),
              shown("", "Shake Overscan", "pair"), shown("", "", "separator"), shown("preScale", "Pre Scale", "float"),
              shown("", "Film Translate", "pair"), shown("", "Film Roll Pivot", "pair"), shown("", "Film Roll Value", "float"),
              shown("", "Film Roll Order", "combo", "Rotate-Translate|Translate-Rotate"), shown("postScale", "Post Scale", "float")},
             1);
    l << box("Depth of Field", {shown("depthOfField", "Depth Of Field", "check"), shown("focusDistance", "Focus Distance", "slider", "0,10000"),
                                shown("fStop", "F Stop", "slider", "1,64"), shown("focusRegionScale", "Focus Region Scale", "slider", "0,10")});
    l << box("Output Settings", {shown("renderable", "Renderable", "check"), shown("image", "Image", "check"), shown("mask", "Mask", "check"),
                                 shown("depth", "Depth", "check"), shown("threshold", "Threshold", "float")});
    l << box("Environment", {shown("imagePlane", "Image Plane", "check")});
    l << box("Special Effects", {shown("shutterAngle", "Shutter Angle", "slider", "0,360")});
    l << box("Display Options",
             {shown("displayFilmGate", "Display Film Gate", "check"), shown("displayResolution", "Display Resolution", "check"),
              shown("displayGateMask", "Display Gate Mask", "check"), shown("displaySafeAction", "Display Safe Action", "check"),
              shown("displaySafeTitle", "Display Safe Title", "check"), shown("displayFieldChart", "Display Field Chart", "check"),
              shown("displayFilmOrigin", "Display Film Origin", "check"), shown("displayFilmPivot", "Display Film Pivot", "check"),
              shown("overscan", "Overscan", "slider", "0,10")});
    l << box("Movement Options",
             {shown("journalCommand", "Undoable Movements", "check"), row("centerOfInterest"),
              shown("tumblePivot", "Tumble Pivot", "vec3"), shown("usePivotAsLocalSpace", "Use Pivot As Local Space", "check"),
              shown("bookmarks", "Bookmarks", "text")});
    l << box("Orthographic Views", {shown("orthographic", "Orthographic", "check"), shown("orthographicWidth", "Orthographic Width", "float")});
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// The poly primitives' history, from their AETemplates: the shape's own settings, Axis, Component Tags.
QList<AeBox> poly_boxes(const QString& type) {
    QList<AeBox> l;
    AeBox history;
    if (type == "polyCube") {
        history = box("Poly Cube History",
                      {shown("width", "", "slider", "0,10"), shown("height", "", "slider", "0,10"), shown("depth", "", "slider", "0,10"),
                       shown("", "", "separator"), shown("heightBaseline", "Height Baseline", "slider", "0,10"),
                       shown("", "", "separator"), shown("subdivisionsWidth", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"),
                       shown("subdivisionsDepth", "", "slider", "0,20"),
                       shown("", "", "separator"), shown("createUVs", "Create UVs", "widecombo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    } else if (type == "polySphere") {
        history = box("Poly Sphere History",
                      {shown("radius", "", "slider", "0,10"), shown("", "", "separator"), shown("heightBaseline", "Height Baseline", "slider", "0,10"), shown("", "", "separator"),
                       shown("subdivisionsAxis", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"), shown("", "", "separator"),
                       shown("createUVs", "Create UVs", "combo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    } else if (type == "polyCylinder") {
        history = box("Poly Cylinder History",
                      {shown("radius", "", "slider", "0,10"), shown("height", "", "slider", "0,10"), shown("", "", "separator"),
                       shown("heightBaseline", "Height Baseline", "slider", "0,10"), shown("", "", "separator"),
                       shown("subdivisionsAxis", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"), shown("subdivisionsCaps", "", "slider", "0,20"),
                       shown("", "", "separator"), shown("roundCap", "Round Cap", "check"),
                       shown("roundCapHeightCompensation", "Round Cap Height Compensation", "float"), shown("", "", "separator"),
                       shown("createUVs", "Create UVs", "combo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    } else if (type == "polyCone") {
        history = box("Poly Cone History",
                      {shown("radius", "", "slider", "0,10"), shown("height", "", "slider", "0,10"), shown("", "", "separator"),
                       shown("subdivisionsAxis", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"),
                       shown("subdivisionsCaps", "", "slider", "0,20"), shown("", "", "separator"), shown("createUVs", "Create UVs", "combo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    } else if (type == "polyTorus") {
        history = box("Poly Torus History",
                      {shown("radius", "", "slider", "0,10"), shown("sectionRadius", "", "slider", "0,10"), shown("twist", "", "slider", "0,360"),
                       shown("", "", "separator"), shown("heightBaseline", "Height Baseline", "slider", "0,10"), shown("", "", "separator"),
                       shown("subdivisionsAxis", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"), shown("", "", "separator"),
                       shown("createUVs", "Create UVs", "combo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    } else {
        history = box("Poly Plane History",
                      {shown("width", "", "slider", "0,10"), shown("height", "", "slider", "0,10"), shown("", "", "separator"),
                       shown("heightBaseline", "Height Baseline", "slider", "0,10"), shown("", "", "separator"),
                       shown("subdivisionsWidth", "", "slider", "0,20"), shown("subdivisionsHeight", "", "slider", "0,20"), shown("", "", "separator"),
                       shown("createUVs", "Create UVs", "combo", "Normalize Collectively and Preserve Aspect Ratio")},
                      1);
    }
    l << history;
    l << box("Axis", {row("axis")}, 1);
    l << box("Component Tags", {shown("componentTagCreate", "Component Tag Create", "check"),
                                shown("componentTagPrefix", "Component Tag Prefix", "text"),
                                shown("componentTagSuffix", "Component Tag Suffix", "text")},
             1);
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// The trees whose builders come later.
QList<AeBox> light_boxes(const QString& type);
QWidget* light_header(const QString& type, QWidget* parent);

// The tree for a node type, when its reference template is laid out here. Other types keep the generic sections.
QList<AeBox> ae_tree(const QString& type) {
    if (type == "transform" || type == "locator" || type == "ikHandle") return transform_boxes();
    if (type == "joint") return joint_boxes();
    if (type == "mesh") return mesh_boxes();
    if (type == "camera") return camera_boxes();
    if (type == "openPBRSurface") return openpbr_boxes();
    static const QSet<QString> lights{"ambientLight", "directionalLight", "pointLight", "spotLight", "areaLight", "volumeLight"};
    if (lights.contains(type)) return light_boxes(type);
    if (type == "blendShape") return blend_boxes();
    if (type == "skinCluster") return skin_boxes();
    static const QSet<QString> deformers{"nonLinear", "clusterHandle", "cluster", "lattice", "baseLattice", "deformBend", "deformTwist",
                                         "deformSquash", "deformFlare", "deformSine", "deformWave", "sculpt", "softModHandle",
                                         "wrap", "shrinkWrap"};
    if (deformers.contains(type)) return deformer_boxes(type);
    if (type == "nurbsCurve") return curve_boxes();
    if (type == "nurbsSurface") return surface_boxes();
    static const QSet<QString> polys{"polyCube", "polySphere", "polyCylinder", "polyCone", "polyTorus", "polyPlane"};
    if (polys.contains(type)) return poly_boxes(type);
    return {};
}

// A section's bar: the reference's raised grey band with its triangle at the left.
class AeHead : public QWidget {
public:
    AeHead(const QString& title, bool open, int depth, QWidget* parent) : QWidget(parent), title_(title), open_(open), depth_(depth) {
        setObjectName("aeSectionHead");
        setProperty("title", title);
        setProperty("open", open);
        setFixedHeight(22);
        setCursor(Qt::PointingHandCursor);
        setToolTip(title);
    }
    std::function<void(bool)> toggled;

    void set_open(bool on) {
        open_ = on;
        setProperty("open", on);
        update();
    }
    bool open() const { return open_; }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        QStyleOption option;
        option.initFrom(this);
        style()->drawPrimitive(QStyle::PE_Widget, &option, &p, this);
        const QColor ink = option.palette.color(QPalette::WindowText);
        const int left = 8 + depth_ * 12;
        p.setPen(Qt::NoPen);
        p.setBrush(ink);
        QPolygon tri;
        if (open_)
            tri << QPoint(left, height() / 2 - 2) << QPoint(left + 8, height() / 2 - 2) << QPoint(left + 4, height() / 2 + 3);
        else
            tri << QPoint(left + 1, height() / 2 - 4) << QPoint(left + 1, height() / 2 + 4) << QPoint(left + 6, height() / 2);
        p.drawPolygon(tri);
        QFont f = font();
        f.setPixelSize(11);
        f.setWeight(QFont::DemiBold);
        p.setFont(f);
        p.setPen(ink);
        p.drawText(QRect(left + 16, 0, width() - left - 18, height()), Qt::AlignLeft | Qt::AlignVCenter, title_);
    }
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() != Qt::LeftButton) return;
        open_ = !open_;
        update();
        if (toggled) toggled(open_);
        // Shift takes every section on the tab with it, as the reference's expand arrows do.
        if (e->modifiers() & Qt::ShiftModifier) {
            QWidget* page = this;
            while (page->parentWidget() && !qobject_cast<QScrollArea*>(page->parentWidget()->parentWidget())) page = page->parentWidget();
            if (page->parentWidget() && page->parentWidget()->parentWidget()) {
                const bool state = open_;
                for (QWidget* w : page->parentWidget()->parentWidget()->findChildren<QWidget*>()) {
                    auto* other = dynamic_cast<AeHead*>(w);
                    if (!other || other == this) continue;
                    other->open_ = state;
                    other->update();
                    if (other->toggled) other->toggled(state);
                }
            }
        }
        e->accept();
    }

private:
    QString title_;
    bool open_;
    int depth_;
};

QString ae_type_label(const QString& type) {
    return type;
}

// Keep the small drawn icons legible when the theme changes while the editor is open.
class AeIconButton : public QToolButton {
public:
    AeIconButton(std::function<QIcon()> draw, QWidget* parent) : QToolButton(parent), draw_(std::move(draw)) {
        setIcon(draw_());
    }

protected:
    void changeEvent(QEvent* event) override {
        QToolButton::changeEvent(event);
        if (event->type() == QEvent::PaletteChange) setIcon(draw_());
    }

private:
    std::function<QIcon()> draw_;
};

// The two small buttons beside the name field: where the node's input and output connections are.
QIcon connect_icon(bool in) {
    QPixmap px(14, 14);
    px.fill(Qt::transparent);
    QPainter p(&px);
    p.setPen(QPen(qApp->palette().color(QPalette::ButtonText), 1));
    p.drawRect(in ? 7 : 2, 3, 6, 8);
    if (in) {
        p.drawLine(1, 7, 7, 7);
        p.drawLine(1, 7, 4, 4);
        p.drawLine(1, 7, 4, 10);
    } else {
        p.drawLine(8, 7, 13, 7);
        p.drawLine(13, 7, 10, 4);
        p.drawLine(13, 7, 10, 10);
    }
    return QIcon(px);
}

// The reference's small solid triangle on the limit buttons.
QIcon triangle_icon(bool left) {
    QPixmap px(9, 9);
    px.fill(Qt::transparent);
    QPainter p(&px);
    p.setRenderHint(QPainter::Antialiasing, false);
    p.setPen(Qt::NoPen);
    p.setBrush(qApp->palette().color(QPalette::ButtonText));
    QPolygon tri;
    if (left) tri << QPoint(6, 1) << QPoint(6, 8) << QPoint(1, 4);
    else tri << QPoint(2, 1) << QPoint(2, 8) << QPoint(7, 4);
    p.drawPolygon(tri);
    return QIcon(px);
}
QIcon pin_icon() {
    QPixmap px(14, 14);
    px.fill(Qt::transparent);
    QPainter p(&px);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.translate(7, 7);
    p.rotate(45);
    p.setPen(QPen(QColor(225, 225, 225), 1.2));
    p.drawLine(0, -5, 0, 5);
    p.drawLine(-4, -5, 4, -5);
    p.drawLine(-2, 3, 2, 3);
    return QIcon(px);
}

// The Attribute Editor's compact menus use the current theme.
void style_ae_menu(QMenu* m) {
    m->setObjectName("aeMenu");
}

// The reference's gear on the Manage Deformed Geometry button.
QIcon gear_icon() {
    QPixmap px(14, 14);
    px.fill(Qt::transparent);
    QPainter p(&px);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.setPen(QPen(qApp->palette().color(QPalette::ButtonText), 1.2));
    p.setBrush(Qt::NoBrush);
    p.drawEllipse(QRectF(4.5, 4.5, 5, 5));
    for (int i = 0; i < 8; ++i) {
        p.save();
        p.translate(7, 7);
        p.rotate(i * 45);
        p.drawLine(0, -6, 0, -4);
        p.restore();
    }
    return QIcon(px);
}

QWidget* manage_geometry_button(QWidget* parent) {
    auto* b = new AeIconButton(gear_icon, parent);
    b->setObjectName("aeManageGeometry");
    b->setText("Manage Deformed Geometry");
    b->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
    b->setPopupMode(QToolButton::InstantPopup);
    b->setFixedHeight(22);
    auto* menu = new QMenu(b);
    style_ae_menu(menu);
    menu->addAction("Add Selected Geometry")->setEnabled(false);
    menu->addAction("Remove Selected Geometry")->setEnabled(false);
    b->setMenu(menu);
    return b;
}

// The Deformer Attributes input table: one row per deformed shape, as the reference shows it.
QWidget* input_table(const QString& geometry, QWidget* parent) {
    auto* table = new QTableWidget(4, 4, parent);
    table->setObjectName("aeInputTable");
    table->setHorizontalHeaderLabels({"Index", "Component Tag", "Falloff", "Geometry"});
    table->verticalHeader()->setVisible(false);
    table->horizontalHeader()->setFixedHeight(20);
    table->horizontalHeader()->setSectionResizeMode(0, QHeaderView::Fixed);
    table->horizontalHeader()->setSectionResizeMode(1, QHeaderView::Stretch);
    table->horizontalHeader()->setSectionResizeMode(2, QHeaderView::Stretch);
    table->horizontalHeader()->setSectionResizeMode(3, QHeaderView::Stretch);
    table->setColumnWidth(0, 42);
    table->setFixedHeight(110);
    table->setEditTriggers(QAbstractItemView::NoEditTriggers);
    table->setSelectionMode(QAbstractItemView::NoSelection);
    table->setFocusPolicy(Qt::NoFocus);
    table->setShowGrid(true);
    table->setContextMenuPolicy(Qt::CustomContextMenu);
    for (int r = 0; r < table->rowCount(); ++r) table->setRowHeight(r, 18);
    if (!geometry.isEmpty()) {
        table->setItem(0, 0, new QTableWidgetItem("0"));
        table->setItem(0, 1, new QTableWidgetItem("* (All Components)"));
        auto* falloff = new QTableWidgetItem("Right-click to Add");
        QFont italic = falloff->font();
        italic.setItalic(true);
        falloff->setFont(italic);
        table->setItem(0, 2, falloff);
        table->setItem(0, 3, new QTableWidgetItem(geometry.section('|', -1)));
    }
    QObject::connect(table, &QTableWidget::customContextMenuRequested, table, [table](const QPoint& at) {
        QMenu menu;
        style_ae_menu(&menu);
        menu.addAction("Select Geometry")->setEnabled(false);
        menu.addAction("Select Components")->setEnabled(false);
        menu.addSeparator();
        menu.addAction("Remove Input")->setEnabled(false);
        menu.exec(table->mapToGlobal(at));
    });
    return table;
}

// A light's sample box, as the reference draws it above the light's own frames.
QWidget* light_header(const QString& type, QWidget* parent) {
    static const QHash<QString, QString> icons{{"ambientLight", "ambientLight"}, {"directionalLight", "directionalLight"},
                                               {"pointLight", "pointLight"}, {"spotLight", "spotLight"},
                                               {"areaLight", "areaLight"}, {"volumeLight", "volumeLight"}};
    auto* w = new QWidget(parent);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(2, 6, 12, 6);
    v->setSpacing(2);
    auto* row = new QWidget(w);
    auto* h = new QHBoxLayout(row);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(4);
    auto* label = new QLabel("Sample", row);
    label->setObjectName("aeRowLabel");
    label->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    label->setFixedWidth(160);
    h->addWidget(label);
    QPixmap px(56, 56);
    px.fill(QColor(20, 20, 20));
    QPainter p(&px);
    QIcon ic = icon(icons.value(type, "pointLight"), 40);
    ic.paint(&p, 8, 8, 40, 40);
    auto* sample = new QLabel(row);
    sample->setPixmap(px);
    sample->setFixedSize(56, 56);
    h->addWidget(sample);
    h->addStretch(1);
    v->addWidget(row);
    return w;
}

// A light's tab: Type, Color, Intensity, Illuminates by Default, Emit Diffuse and Specular, then its own frames.
QList<AeBox> light_boxes(const QString& type) {
    QList<AeBox> l;
    QList<AeRow> rows;
    rows << shown("__lightType", "Type", "lightType") << shown("color", "Color", "color") << shown("intensity", "Intensity", "slider", "0,10")
         << shown("__illuminates", "Illuminates by Default", "illuminates");
    if (type == "ambientLight") rows << shown("ambientShade", "Ambient Shade", "slider", "0,1");
    else {
        rows << shown("emitDiffuse", "Emit Diffuse", "check2");
        rows.last().extra = "emitSpecular";
    }
    if (type == "spotLight") rows << shown("coneAngle", "Cone Angle", "slider", "0,180") << shown("penumbraAngle", "Penumbra Angle", "slider", "-10,10")
                                  << shown("dropoff", "Dropoff", "slider", "0,255") << shown("decayRate", "Decay Rate", "combo", "No Decay|Linear|Quadratic|Cubic");
    else if (type == "areaLight") rows << shown("decayRate", "Decay Rate", "combo", "No Decay|Linear|Quadratic|Cubic") << shown("normalize", "Normalize", "check");
    else if (type == "pointLight") rows << shown("decayRate", "Decay Rate", "combo", "No Decay|Linear|Quadratic|Cubic");
    else if (type == "volumeLight")
        rows << shown("lightShape", "Light Shape", "combo", "Sphere|Cylinder|Cone") << shown("volumeLightDir", "Volume Light Dir", "combo", "Outward|Inward|Down Axis")
             << shown("arc", "Arc", "slider", "0,360") << shown("coneEndRadius", "Cone End Radius", "slider", "0,10")
             << shown("emitAmbient", "Emit Ambient", "check");
    const char* title = type == "pointLight" ? "Point Light Attributes"
                      : type == "directionalLight" ? "Directional Light Attributes"
                      : type == "spotLight" ? "Spot Light Attributes"
                      : type == "ambientLight" ? "Ambient Light Attributes"
                      : type == "areaLight" ? "Area Light Attributes"
                                             : "Volume Light Attributes";
    l << box(title, {}, 1);
    l.last().rows = rows;
    if (type == "pointLight" || type == "spotLight") {
        AeBox effects;
        effects.title = "Light Effects";
        effects.open = 0;
        if (type == "pointLight")
            effects.rows = {shown("", "Light Fog", "text"), row("fogType"), row("fogRadius"), row("fogIntensity"),
                            shown("", "", "separator"), shown("", "Light Glow", "text")};
        else effects.rows = {row("fogSpread"), row("fogIntensity"), row("barnDoors"), row("leftBarnDoor"), row("rightBarnDoor"),
                             row("topBarnDoor"), row("bottomBarnDoor"), row("useDecayRegions"), row("startDistance1"), row("endDistance1"),
                             row("startDistance2"), row("endDistance2"), row("startDistance3"), row("endDistance3")};
        l << effects;
    }
    AeBox shadows;
    shadows.title = "Shadows";
    shadows.rows = {row("shadowColor")};
    AeBox depth;
    depth.title = "Depth Map Shadow Attributes";
    depth.open = 1;
    depth.rows = {row("useDepthMapShadows"), shown("dmapResolution", "Resolution", "int"), shown("useMidDistDmap", "Use Mid Dist", "check"),
                  shown("useDmapAutoFocus", "Use Auto Focus", "check"), shown("dmapFocus", "Focus", "float"),
                  shown("dmapFilterSize", "Filter Size", "int"), shown("dmapBias", "Bias", "float"),
                  shown("fogShadowIntensity", "Fog Shadow Intensity", "float"), shown("volumeShadowSamples", "Fog Shadow Samples", "int"),
                  shown("diskBasedDmaps", "Disk Based Dmaps", "combo", "Off|Overwrite Existing Dmap(s)|Reuse Existing Dmap(s)"),
                  shown("dmapName", "Shadow Map File Name", "text"), shown("dmapSceneName", "Add Scene Name", "check"),
                  shown("dmapLightName", "Add Light Name", "check"), shown("dmapFrameExt", "Add Frame Ext", "check")};
    if (type == "ambientLight" || type == "volumeLight") depth.rows = {row("useDepthMapShadows")};
    AeBox raytrace;
    raytrace.title = "Ray Trace Shadow Attributes";
    raytrace.open = 1;
    raytrace.rows = {row("useRayTraceShadows")};
    if (type == "directionalLight") raytrace.rows << shown("lightAngle", "Light Angle", "float");
    else raytrace.rows << shown("lightRadius", "Light Radius", "float");
    raytrace.rows << row("shadowRays") << row("rayDepthLimit");
    if (type == "ambientLight") raytrace.rows << row("castSoftShadows") << shown("shadowRadius", "Shadow Radius", "float");
    shadows.subs << depth << raytrace;
    l << shadows;
    l << box("Object Display",
             {shown("visibility", "Visibility", "check2"), shown("lodVisibility", "LOD Visibility", "check2"),
              shown("hideOnPlayback", "Hide on playback", "check2"), shown("hiddenInOutliner", "Hidden In Outliner", "check2"),
              shown("useOutlinerColor", "Use Outliner Color", "check2"), shown("outlinerColor", "Outliner Color", "color")});
    l.last().rows[0].extra = "template";
    l << box("Harold", {});
    l << box("Node Behavior", {row("caching"), row("nodeState")});
    l << box("UUID", {shown("__uuid", "", "uuidValue")});
    l << box("Extra Attributes", {});
    return l;
}

// A material's sample sphere and its shader type, above the frames as the reference draws them.
QWidget* material_header(const QString& type, const QHash<QString, QJsonObject>& by, QWidget* parent) {
    static const QHash<QString, QString> names{{"lambert", "Lambert"}, {"blinn", "Blinn"}, {"phong", "Phong"}, {"phongE", "Phong E"},
                                                {"anisotropic", "Anisotropic"}, {"standardSurface", "Standard Surface"},
                                                {"aiStandardSurface", "aiStandardSurface"}, {"openPBRSurface", "OpenPBR Surface"},
                                                {"surfaceShader", "Surface Shader"}};
    QJsonObject colourAttr = by.value("baseColor");
    if (colourAttr.isEmpty()) colourAttr = by.value("color");
    QColor tint(128, 128, 128);
    if (!colourAttr.isEmpty()) {
        QJsonArray v = colourAttr["value"].toArray();
        if (v.size() >= 3) tint = swatch_color(v);
    }
    auto* w = new QWidget(parent);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(2, 6, 12, 6);
    v->setSpacing(2);
    auto* sampleRow = new QWidget(w);
    auto* sh = new QHBoxLayout(sampleRow);
    sh->setContentsMargins(0, 0, 0, 0);
    sh->setSpacing(4);
    auto* sampleLabel = new QLabel("Sample", sampleRow);
    sampleLabel->setObjectName("aeRowLabel");
    sampleLabel->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    sampleLabel->setFixedWidth(160);
    sh->addWidget(sampleLabel);
    QPixmap px(56, 56);
    px.fill(QColor(70, 70, 70));
    QPainter p(&px);
    p.setRenderHint(QPainter::Antialiasing, true);
    QRadialGradient g(QPointF(20, 18), 46);
    g.setColorAt(0, QColor(245, 245, 245));
    g.setColorAt(0.35, tint.lighter(140));
    g.setColorAt(0.75, tint.darker(170));
    g.setColorAt(1, QColor(15, 15, 15));
    p.setPen(Qt::NoPen);
    p.setBrush(g);
    p.drawEllipse(QRectF(4, 4, 48, 48));
    auto* sample = new QLabel(sampleRow);
    sample->setPixmap(px);
    sample->setFixedSize(56, 56);
    sh->addWidget(sample);
    sh->addStretch(1);
    v->addWidget(sampleRow);
    auto* typeRow = new QWidget(w);
    auto* th = new QHBoxLayout(typeRow);
    th->setContentsMargins(0, 0, 0, 0);
    th->setSpacing(4);
    auto* typeLabel = new QLabel("Type", typeRow);
    typeLabel->setObjectName("aeRowLabel");
    typeLabel->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    typeLabel->setFixedWidth(160);
    th->addWidget(typeLabel);
    auto* combo = new QComboBox(typeRow);
    combo->addItem(names.value(type, type));
    combo->setFixedWidth(148);
    combo->setEnabled(false);
    th->addWidget(combo);
    th->addStretch(1);
    v->addWidget(typeRow);
    return w;
}

QFormLayout* add_section(QVBoxLayout* v, QWidget* parent, const QString& title, const QString& owner, const QString& key = QString()) {
    const QString stateKey = key.isEmpty() ? title : key;
    const bool open = section_open(owner, stateKey, !default_closed_section(title));
    auto* headWrap = new QWidget(parent);
    auto* hw = new QHBoxLayout(headWrap);
    hw->setContentsMargins(18, 0, 18, 0);
    hw->setSpacing(0);
    auto* head = new AeHead(title, open, 0, headWrap);
    hw->addWidget(head);
    v->addWidget(headWrap);
    auto* box = new QWidget(parent);
    box->setObjectName("aeSectionBody");
    auto* f = new QFormLayout(box);
    f->setLabelAlignment(Qt::AlignRight);
    f->setContentsMargins(2, 3, 64, 6);
    f->setHorizontalSpacing(4);
    f->setVerticalSpacing(3);
    box->setVisible(open);
    v->addWidget(box);
    head->toggled = [box, head, owner, stateKey](bool on) {
        box->setVisible(on);
        head->set_open(on);
        remember_section(owner, stateKey, on);
    };
    return f;
}

// The reference's greyed control for a feature Aztek does not have yet: present, and taking no typing.
QLabel* ae_label(const QString& text, QWidget* parent, int width = 160) {
    auto* l = new QLabel(parent);
    l->setObjectName("aeRowLabel");
    l->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    l->setFixedWidth(width);
    l->setText(l->fontMetrics().elidedText(text, Qt::ElideLeft, width - 2));
    l->setToolTip(text);
    return l;
}

QWidget* ae_placeholder(const AeRow& r, const QJsonObject& a, QWidget* parent) {
    const QString kind = r.kind.isEmpty() ? "float" : r.kind;
    if (kind == "check") {
        auto* cb = new QCheckBox(parent);
        cb->setChecked(r.fallback == "1");
        cb->setEnabled(false);
        return cb;
    }
    if (kind == "checktext") {
        auto* cb = new QCheckBox(r.fallback, parent);
        cb->setEnabled(false);
        return cb;
    }
    if (kind == "check2") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        for (int k = 0; k < 2; ++k) {
            auto* cb = new QCheckBox(parent);
            cb->setEnabled(false);
            h->addWidget(cb);
            auto* l = new QLabel(k == 0 ? (r.label.isEmpty() ? a["nice"].toString() : r.label) : a["nice"].toString(), w);
            l->setObjectName("aeDimLabel");
            h->addWidget(l);
            if (k == 0) h->addSpacing(24);
        }
        h->addStretch(1);
        return w;
    }
    if (kind == "radio2") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        QStringList items = r.extra.isEmpty() ? QStringList{"Right Handed", "Left Handed"} : r.extra.split('|');
        if (r.label == "UV Winding Order:") items = QStringList{"Detect", "Ignore"};
        for (int k = 0; k < items.size(); ++k) {
            auto* rb = new QRadioButton(items.at(k), w);
            rb->setChecked(k == (r.fallback.isEmpty() ? 0 : r.fallback.toInt()));
            rb->setEnabled(false);
            h->addWidget(rb);
            if (k == 0) h->addSpacing(10);
        }
        h->addStretch(1);
        return w;
    }
    if (kind == "channels") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        auto* lab = new QLabel(r.label.isEmpty() ? "Channels" : r.label, w);
        h->addWidget(lab);
        for (const char* c : {"R", "G", "B"}) {
            auto* cb = new QCheckBox(w);
            cb->setFixedSize(14, 14);
            cb->setEnabled(false);
            h->addWidget(cb);
            h->addWidget(new QLabel(c, w));
            h->addSpacing(12);
        }
        h->addStretch(1);
        return w;
    }
    if (kind == "button") {
        auto* b = new QPushButton(r.label, parent);
        b->setObjectName("aeSmallButton");
        b->setFixedSize(240, 22);
        b->setEnabled(false);
        return b;
    }
    if (kind == "combo" || kind == "widecombo") {
        auto* box = new QComboBox(parent);
        QString source = !r.extra.isEmpty() ? r.extra : r.fallback;
        QStringList items = source.isEmpty() ? QStringList{"None"} : source.split('|');
        for (const QString& s : items) box->addItem(s);
        box->setFixedWidth(kind == "widecombo" ? 300 : 148);
        box->setEnabled(false);
        return box;
    }
    if (kind == "vec3" || kind == "color") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        if (kind == "color") {
            auto* sw = new QPushButton(w);
            sw->setObjectName("colorSwatch");
            sw->setFixedSize(40, 18);
            sw->setEnabled(false);
            h->addWidget(sw);
        }
        int count = kind == "color" ? 1 : 3;
        for (int k = 0; k < count; ++k) {
            auto* e = new QLineEdit(w);
            e->setFixedSize(kind == "color" ? 148 : 68, 20);
            e->setEnabled(false);
            h->addWidget(e);
        }
        h->addStretch(1);
        return w;
    }
    if (kind == "matrix" || kind == "text") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        auto* e = new QLineEdit(w);
        e->setFixedHeight(20);
        e->setMinimumWidth(kind == "matrix" ? 250 : 225);
        e->setEnabled(false);
        h->addWidget(e, 1);
        if (kind == "text") {
            auto* map = new QToolButton(w);
            map->setObjectName("mapButton");
            map->setAutoRaise(true);
            map->setFixedSize(18, 18);
            map->setIcon(map_icon(false));
            map->setEnabled(false);
            h->addWidget(map);
        }
        return w;
    }
    if (kind == "uuid") {
        auto* e = new QLineEdit(parent);
        e->setFixedHeight(20);
        e->setReadOnly(true);
        e->setEnabled(false);
        return e;
    }
    if (kind == "slider") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        auto* e = new QLineEdit(w);
        e->setFixedSize(68, 20);
        e->setEnabled(false);
        auto* s = new QSlider(Qt::Horizontal, w);
        s->setRange(0, 1000);
        s->setEnabled(false);
        h->addWidget(e);
        h->addWidget(s, 1);
        return w;
    }
    auto* e = new QLineEdit(parent);
    e->setFixedSize(68, 20);
    e->setEnabled(false);
    return e;
}

// One attribute row's field, sized as the reference sizes it.
QWidget* ae_field(const AeRow& r, const QJsonObject& a, const QString& style, QWidget* parent) {
    QString type = a["type"].toString();
    bool ro = read_only(a);
    if ((r.kind == "combo" || r.kind == "widecombo") && type != "enum") {
        auto* box = new QComboBox(parent);
        QString source = !r.extra.isEmpty() ? r.extra : r.fallback;
        QStringList items;
        for (const QJsonValue& e : a["enum"].toArray()) items << e.toString();
        if (items.isEmpty()) items = source.isEmpty() ? QStringList{"None"} : source.split('|');
        for (const QString& item : items) box->addItem(item);
        box->setCurrentIndex(qBound(0, a["value"].toInt(), box->count() - 1));
        box->setFixedWidth(r.kind == "widecombo" ? 300 : 148);
        box->setEnabled(!ro);
        QString plug = a["plug"].toString();
        box->setProperty("plugKey", plug);
        QObject::connect(box, QOverload<int>::of(&QComboBox::activated), box, [plug](int i) { set_plug(plug, i, "int"); });
        return box;
    }
    if (r.kind == "check") {
        auto* cb = new QCheckBox(parent);
        cb->setChecked(a["value"].toBool() || (a["value"].isDouble() && a["value"].toDouble() != 0));
        cb->setEnabled(!ro);
        QString plug = a["plug"].toString();
        cb->setProperty("plugKey", plug);
        QObject::connect(cb, &QCheckBox::toggled, cb, [plug](bool on) { set_plug(plug, on, "bool"); });
        return cb;
    }
    QWidget* field = nullptr;
    if (r.kind == "text") {
        auto* w = new QWidget(parent);
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(4);
        auto* e = new QLineEdit(w);
        e->setFixedHeight(20);
        e->setMinimumWidth(225);
        if (type == "string") {
            e->setText(a["value"].toString());
            e->setReadOnly(ro);
            QString plug = a["plug"].toString();
            e->setProperty("plugKey", plug);
            QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
            QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
                if (!e->property("editing").toBool()) return;
                e->setProperty("editing", false);
                set_plug(plug, e->text(), "string");
            });
            if (a["locked"].toBool()) e->setObjectName("lockedField");
        } else {
            e->setEnabled(false);
        }
        h->addWidget(e, 1);
        if (a["mappable"].toBool()) h->addWidget(map_button(a, w));
        return w;
    }
    field = attr_field(a, parent);
    int width = 0;
    if (r.kind == "matrix" || type == "matrix") width = 295;
    else if (r.kind == "combo" || type == "enum") width = r.attr == "rotateOrder" ? 72 : 148;
    else if (type == "vec3") width = 212;
    else if (r.kind == "text") width = 0;
    else if (type == "float" || type == "int") {
        // Ranged numbers keep their slider, and the slider runs to the end of the value column.
        width = (a["min"].isDouble() && a["max"].isDouble() && a["max"].toDouble() > a["min"].toDouble()) ? 210 : 68;
        if (type == "int" && !(a["min"].isDouble() && a["max"].isDouble())) width = 68;
    }
    if (width > 0) {
        if (type == "vec3") {
            field->setFixedWidth(width);
            // Three valued parts a row: each 72 wide, as the reference's fields.
            for (QLineEdit* e : field->findChildren<QLineEdit*>()) e->setFixedSize(68, 20);
        } else if (type == "enum") {
            field->setFixedWidth(width);
        } else {
            field->setFixedWidth(width);
        }
    }
    if (type == "float" || type == "int" || type == "string" || type == "vec3") {
        for (QLineEdit* e : field->findChildren<QLineEdit*>()) e->setFixedHeight(20);
    }
    return field;
}

// A vector's three parts, the reference's three fields of 72 pixels.
QWidget* ae_vec3(const QJsonObject& a, QWidget* parent) {
    auto* w = new QWidget(parent);
    auto* h = new QHBoxLayout(w);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(4);
    QJsonArray v = a["value"].toArray();
    QJsonArray kids = a["children"].toArray();
    QJsonArray childLocked = a["childLocked"].toArray();
    QJsonArray childConnected = a["childConnected"].toArray();
    QString plug = a["plug"].toString();
    int count = a["pair"].toBool() ? 2 : 3;
    for (int k = 0; k < count; ++k) {
        auto* e = new QLineEdit(fmt3(v.at(k).toDouble()), w);
        e->setFixedSize(68, 20);
        const bool locked = a["locked"].toBool() || childLocked.at(k).toBool();
        e->setReadOnly(read_only(a) || locked);
        if (a["childKeyed"].toArray().at(k).toBool()) e->setObjectName("keyedField");
        else if (childConnected.at(k).toBool()) e->setObjectName("connectedField");
        if (locked) e->setObjectName("lockedField");
        QString child = kids.at(k).toString();
        if (child.isEmpty()) child = a["attr"].toString() + QString("XYZ")[k];
        QString cp = plug.left(plug.lastIndexOf('.')) + "." + child;
        e->setProperty("plugKey", cp);
        QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
        QObject::connect(e, &QLineEdit::editingFinished, e, [e, cp] {
            if (!e->property("editing").toBool()) return;
            e->setProperty("editing", false);
            set_plug(cp, e->text().toDouble(), "float");
        });
        h->addWidget(e);
    }
    h->addStretch(1);
    return w;
}

// A matrix as the reference's long matrix field: its sixteen values, live, and written back as sixteen numbers.
QWidget* ae_matrix(const QJsonObject& a, QWidget* parent) {
    auto* e = new QLineEdit(parent);
    e->setFixedHeight(20);
    e->setMinimumWidth(225);
    QJsonArray v = a["value"].toArray();
    QStringList parts;
    for (const QJsonValue& x : v) parts << QString::number(x.toDouble(), 'g', 6);
    e->setText(parts.join(' '));
    e->setToolTip(parts.join(' '));
    const QString plug = a["plug"].toString();
    bool ro = read_only(a);
    e->setReadOnly(ro);
    e->setProperty("plugKey", plug);
    QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
    QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug, v] {
        if (!e->property("editing").toBool()) return;
        e->setProperty("editing", false);
        QStringList parts = e->text().split(QRegularExpression("[ ,]+"), Qt::SkipEmptyParts);
        QJsonArray values;
        if (parts.size() == 16) {
            for (const QString& part : parts) values.append(part.toDouble());
            exec("setAttr", args(QJsonArray{plug, values}));
        } else {
            QStringList current;
            for (const QJsonValue& x : v) current << QString::number(x.toDouble(), 'g', 6);
            e->setText(current.join(' '));
        }
    });
    return e;
}

// The reference's three column limit row: the minimum and maximum greyed until enabled, and buttons that take them.
QWidget* ae_limit_row(const AeBox& group, const QString& axis, const QHash<QString, QJsonObject>& by, QWidget* parent, const QString& label) {
    auto* w = new QWidget(parent);
    auto* h = new QHBoxLayout(w);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(2);
    QString type = group.limit;
    QString prefix = type == "Trans" ? "Trans" : type;
    QString curName = (type == "Trans" ? "translate" : type == "Rot" ? "rotate" : "scale") + axis;
    QString minName = "min" + prefix + axis + "Limit";
    QString maxName = "max" + prefix + axis + "Limit";
    auto value_field = [&](const QString& name, int width) {
        QJsonObject a = by.value(name);
        int part = -1;
        QString childName = name;
        auto component_of = [&](const QString& base, int index) {
            QJsonObject parentAttr = by.value(base);
            if (parentAttr.isEmpty() || parentAttr["type"].toString() != "vec3") return false;
            QJsonArray kids = parentAttr["children"].toArray();
            childName = index < kids.size() ? kids.at(index).toString() : base + QString("XYZ")[index];
            a = parentAttr;
            part = index;
            return true;
        };
        if (a.isEmpty()) {
            // minRotXLimit is a part of minRotLimit.
            for (int partIndex = 0; partIndex < 3 && a.isEmpty(); ++partIndex) {
                QString suffix = QString("XYZ").mid(partIndex, 1) + "Limit";
                if (name.endsWith(suffix)) component_of(name.left(name.size() - suffix.size()) + "Limit", partIndex);
            }
            if (a.isEmpty() && name.size() > 1) {
                QChar last = name.back();
                if (last == 'X' || last == 'Y' || last == 'Z') component_of(name.left(name.size() - 1), last == 'X' ? 0 : last == 'Y' ? 1 : 2);
            }
        }
        auto* e = new QLineEdit(w);
        e->setFixedSize(width, 20);
        if (!a.isEmpty()) {
            if (part >= 0) {
                QJsonArray values = a["value"].toArray();
                QString shownValue = (name.startsWith("min") || name.startsWith("max"))
                                         ? QString::number(values.at(part).toDouble(), 'f', 2)
                                         : fmt3(values.at(part).toDouble());
                e->setText(shownValue);
                QString base = a["plug"].toString();
                QString plug = base.left(base.lastIndexOf('.')) + "." + childName;
                e->setProperty("plugKey", plug);
                e->setReadOnly(read_only(a));
                QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
                QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
                    if (!e->property("editing").toBool()) return;
                    e->setProperty("editing", false);
                    set_plug(plug, e->text().toDouble(), "float");
                });
            } else {
            e->setText(a["type"].toString() == "int" ? QString::number(a["value"].toInteger()) : fmt3(a["value"].toDouble()));
            e->setReadOnly(read_only(a));
            QString plug = a["plug"].toString();
            QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
            QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
                if (!e->property("editing").toBool()) return;
                e->setProperty("editing", false);
                set_plug(plug, e->text().toDouble(), "float");
            });
            }
        } else {
            e->setEnabled(false);
        }
        return e;
    };
    auto enable_box = [&](const QString& name) {
        auto* cb = new QCheckBox(w);
        cb->setFixedSize(14, 14);
        QJsonObject a = by.value(name);
        if (!a.isEmpty()) {
            cb->setChecked(a["value"].toBool());
            QString plug = a["plug"].toString();
            cb->setProperty("plugKey", plug);
            QObject::connect(cb, &QCheckBox::toggled, cb, [plug](bool on) { set_plug(plug, on, "bool"); });
        } else {
            cb->setEnabled(false);
        }
        return cb;
    };
    QLineEdit* minField = value_field(minName, 44);
    QLineEdit* maxField = value_field(maxName, 44);
    QCheckBox* minOn = enable_box("min" + prefix + axis + "LimitEnable");
    QCheckBox* maxOn = enable_box("max" + prefix + axis + "LimitEnable");
    QLineEdit* curField = value_field(curName, 42);
    auto* lo = new AeIconButton([] { return triangle_icon(true); }, w);
    lo->setObjectName("aeArrowButton");
    lo->setIconSize(QSize(9, 9));
    lo->setFixedSize(16, 20);
    auto* hi = new AeIconButton([] { return triangle_icon(false); }, w);
    hi->setObjectName("aeArrowButton");
    hi->setIconSize(QSize(9, 9));
    hi->setFixedSize(16, 20);
    QJsonObject cur = by.value(curName);
    QString curPlug = cur.value("plug").toString();
    QObject::connect(lo, &QToolButton::clicked, lo, [=] {
        if (curPlug.isEmpty()) return;
        exec("setAttr", args(QJsonArray{curPlug, minField->text().toDouble()}));
    });
    QObject::connect(hi, &QToolButton::clicked, hi, [=] {
        if (curPlug.isEmpty()) return;
        exec("setAttr", args(QJsonArray{curPlug, maxField->text().toDouble()}));
    });
    minField->setEnabled(minOn->isChecked() && minOn->isEnabled());
    maxField->setEnabled(maxOn->isChecked() && maxOn->isEnabled());
    QObject::connect(minOn, &QCheckBox::toggled, minField, [minField](bool on) { minField->setEnabled(on); });
    QObject::connect(maxOn, &QCheckBox::toggled, maxField, [maxField](bool on) { maxField->setEnabled(on); });
    h->addWidget(minField);
    h->addWidget(minOn);
    h->addWidget(lo);
    h->addWidget(curField);
    h->addWidget(hi);
    h->addWidget(maxField);
    h->addWidget(maxOn);
    h->addStretch(1);
    Q_UNUSED(label);
    return w;
}

QWidget* ae_limit_header(QWidget* parent) {
    auto* w = new QWidget(parent);
    auto* h = new QHBoxLayout(w);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(2);
    auto add = [&](const char* text, int width) {
        auto* l = new QLabel(text, w);
        l->setObjectName("aeDimLabel");
        l->setAlignment(Qt::AlignCenter);
        l->setFixedWidth(width);
        h->addWidget(l);
    };
    add("Min", 58);
    add("Current", 76);
    add("Max", 58);
    h->addStretch(1);
    return w;
}

class AeRowLine : public QFrame {
public:
    AeRowLine(QWidget* parent) : QFrame(parent) {
        setFixedHeight(1);
        setObjectName("aeRowLine");
    }
};

// Which of the node's attributes the fixed layout has placed.
void ae_render_rows(QVBoxLayout* v, QWidget* parent, const QList<AeRow>& rows, const QHash<QString, QJsonObject>& by, QSet<QString>& used,
                    const QString& style, int depth);

// An extra attribute the layout does not name: its own row, under Extra Attributes.
void ae_extra_rows(QVBoxLayout* v, QWidget* parent, const QHash<QString, QJsonObject>& by, QSet<QString>& used, const QString& style) {
    QList<QString> names = by.keys();
    names.sort();
    for (const QString& n : names) {
        if (used.contains(n)) continue;
        const QJsonObject a = by.value(n);
        if (a["type"].toString() == "matrix" || n == "notes" || n == "message") continue;
        QWidget* field = attr_field(a, parent);
        field->setToolTip(a["doc"].toString());
        if (a["mapped"].toBool()) field->setEnabled(false);
        QWidget* rowWidget = new QWidget(parent);
        auto* h = new QHBoxLayout(rowWidget);
        h->setContentsMargins(2, 0, 12, 0);
        h->setSpacing(4);
        auto* lab = ae_label(a["nice"].toString(), rowWidget);
        attach_attr_menu(lab, a, field);
        h->addWidget(lab);
        h->addWidget(field);
        h->addStretch(1);
        v->addWidget(rowWidget);
        used.insert(n);
    }
}

void ae_render_box(QVBoxLayout* v, QWidget* parent, const AeBox& b, const QHash<QString, QJsonObject>& by, QSet<QString>& used,
                   const QString& style, int depth, const QString& owner, const QString& prefix) {
    const QString key = prefix.isEmpty() ? b.title : prefix + "/" + b.title;
    bool open = section_open(owner, key, b.open == 1);
    auto* headWrap = new QWidget(parent);
    auto* hw = new QHBoxLayout(headWrap);
    hw->setContentsMargins(18, 0, 18, 0);
    hw->setSpacing(0);
    auto* head = new AeHead(b.title, open, depth, headWrap);
    hw->addWidget(head);
    v->addWidget(headWrap);
    auto* content = new QWidget(parent);
    content->setObjectName("aeSectionBody");
    auto* cv = new QVBoxLayout(content);
    cv->setContentsMargins(0, 3, 0, 5);
    cv->setSpacing(2);
    content->setVisible(open);
    v->addWidget(content);
    head->toggled = [content, head, owner, key](bool on) {
        content->setVisible(on);
        head->set_open(on);
        remember_section(owner, key, on);
    };
    // The group's three limit rows, with the reference's Min, Current and Max above them.
    if (!b.limit.isEmpty()) {
        const QString prefix = b.limit;
        used.insert("min" + prefix + "Limit");
        used.insert("max" + prefix + "Limit");
        for (const char* axis : {"X", "Y", "Z"}) {
            used.insert("min" + prefix + axis + "Limit");
            used.insert("max" + prefix + axis + "Limit");
            used.insert("min" + prefix + axis + "LimitEnable");
            used.insert("max" + prefix + axis + "LimitEnable");
        }
        auto* hw = new QWidget(content);
        auto* hh = new QHBoxLayout(hw);
        hh->setContentsMargins(2, 0, 12, 0);
        auto* pad = new QWidget(hw);
        pad->setFixedWidth(160);
        hh->addWidget(pad);
        hh->addWidget(ae_limit_header(hw));
        cv->addWidget(hw);
        for (const char* axis : {"X", "Y", "Z"}) {
            auto* rw = new QWidget(content);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* lab = ae_label(b.title == "Translate" ? QString("Trans Limit %1").arg(axis)
                                  : b.title == "Rotate" ? QString("Rot Limit %1").arg(axis)
                                                        : QString("Scale Limit %1").arg(axis),
                                  rw);
            h->addWidget(lab);
            h->addWidget(ae_limit_row(b, axis, by, rw, lab->text()));
            h->addStretch(1);
            cv->addWidget(rw);
        }
        return;
    }
    ae_render_rows(cv, content, b.rows, by, used, style, depth);
    for (const AeBox& sub : b.subs) ae_render_box(cv, content, sub, by, used, style, depth + 1, owner, key);
}

void ae_render_rows(QVBoxLayout* v, QWidget* parent, const QList<AeRow>& rows, const QHash<QString, QJsonObject>& by, QSet<QString>& used,
                    const QString& style, int depth) {
    const bool extra = false;
    for (const AeRow& r : rows) {
        if (r.kind == "separator") {
            auto* line = new AeRowLine(parent);
            auto* wrap = new QWidget(parent);
            auto* wh = new QHBoxLayout(wrap);
            wh->setContentsMargins(18, depth * 0, 12, 0);
            wh->addWidget(line);
            v->addWidget(wrap);
            continue;
        }
        QJsonObject a = by.value(r.attr);
        QString label = r.label;
        if (style == "Long" && !a.isEmpty()) label = a["attr"].toString();
        if (label.isEmpty() && !a.isEmpty()) label = a["nice"].toString();
        if (label.isEmpty() && !r.attr.isEmpty()) label = r.attr;
        if (r.kind == "check2") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* pad = new QWidget(rw);
            pad->setFixedWidth(160);
            h->addWidget(pad);
            auto add_check = [&](const QString& name, const QString& text, bool enabled) {
                QJsonObject at = by.value(name);
                auto* cb = new QCheckBox(rw);
                cb->setFixedSize(14, 14);
                if (!at.isEmpty()) {
                    cb->setChecked(at["value"].toBool());
                    QString plug = at["plug"].toString();
                    cb->setProperty("plugKey", plug);
                    QObject::connect(cb, &QCheckBox::toggled, cb, [plug](bool on) { set_plug(plug, on, "bool"); });
                    used.insert(name);
                } else {
                    cb->setEnabled(false);
                }
                Q_UNUSED(enabled);
                auto* l = new QLabel(text, rw);
                h->addWidget(cb);
                h->addWidget(l);
            };
            add_check(r.attr, label, true);
            if (!r.extra.isEmpty()) {
                const int used = 14 + 4 + rw->fontMetrics().horizontalAdvance(label);
                h->addSpacing(qMax(8, 148 - used));
                QString second = by.value(r.extra).isEmpty() ? QString() : by.value(r.extra)["nice"].toString();
                if (second.isEmpty()) second = r.extra == "template" ? "Template" : r.extra == "intermediateObject" ? "Intermediate Object" : r.extra;
                add_check(r.extra, second, true);
            }
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        if (r.kind == "slider" && !a.isEmpty() && !(a["min"].isDouble() && a["max"].isDouble())) {
            QStringList bounds = (r.extra.isEmpty() ? r.fallback : r.extra).split(',');
            if (bounds.size() == 2) {
                a["min"] = bounds.at(0).toDouble();
                a["max"] = bounds.at(1).toDouble();
            }
        }
        if (r.kind == "manageGeometry" || r.kind == "inputTable") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 2, 12, 2);
            h->setSpacing(4);
            if (r.kind == "manageGeometry") {
                h->addStretch(1);
                h->addWidget(manage_geometry_button(rw));
                h->addStretch(1);
            } else {
                h->addWidget(input_table(by.value("__geometry")["value"].toString(), rw));
            }
            v->addWidget(rw);
            continue;
        }
        if (r.kind == "uuidValue") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* lab = ae_label(label.isEmpty() ? QString() : label, rw);
            h->addWidget(lab);
            auto* e = new QLineEdit(by.value("__uuid")["value"].toString(), rw);
            e->setReadOnly(true);
            e->setFixedHeight(20);
            e->setMinimumWidth(225);
            e->setObjectName("aeField");
            e->setProperty("uuidField", true);
            h->addWidget(e);
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        if (r.kind == "lightType") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            h->addWidget(ae_label(label, rw));
            auto* box = new QComboBox(rw);
            for (const char* n : {"Ambient Light", "Directional Light", "Point Light", "Spot Light", "Area Light", "Volume Light"}) box->addItem(n);
            box->setCurrentIndex(qBound(0, a["value"].toInt(), box->count() - 1));
            box->setFixedWidth(148);
            const QString node = a["plug"].toString();
            const QStringList types{"ambientLight", "directionalLight", "pointLight", "spotLight", "areaLight", "volumeLight"};
            QObject::connect(box, QOverload<int>::of(&QComboBox::activated), box, [node, types](int i) {
                if (i >= 0 && i < types.size()) exec("aztekChangeLightType", args(QJsonArray{node, types.at(i)}));
            });
            h->addWidget(box);
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        if (r.kind == "illuminates") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* pad = new QWidget(rw);
            pad->setFixedWidth(160);
            h->addWidget(pad);
            auto* cb = new QCheckBox(label, rw);
            cb->setChecked(a["value"].toBool());
            const QString node = a["plug"].toString();
            QObject::connect(cb, &QCheckBox::toggled, cb, [node](bool on) {
                exec("aztekIlluminatesByDefault", QJsonObject{{"args", QJsonArray{node}}, {"set", on}});
            });
            h->addWidget(cb);
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        if (r.kind == "pair") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* lab = ae_label(label, rw);
            h->addWidget(lab);
            const QString secondName = r.extra.isEmpty() ? r.fallback : r.extra;
            for (const QString& name : {r.attr, secondName}) {
                QJsonObject at = name.isEmpty() ? QJsonObject() : by.value(name);
                auto* e = new QLineEdit(rw);
                e->setFixedSize(68, 20);
                if (!at.isEmpty()) {
                    used.insert(name);
                    if (at["type"].toString() == "vec3") {
                        QJsonArray values = at["value"].toArray();
                        e->setText(fmt3(values.at(0).toDouble()));
                    } else {
                        e->setText(fmt3(at["value"].toDouble()));
                    }
                    e->setReadOnly(read_only(at));
                    QString plug = at["plug"].toString();
                    QObject::connect(e, &QLineEdit::textEdited, e, [e] { e->setProperty("editing", true); });
                    QObject::connect(e, &QLineEdit::editingFinished, e, [e, plug] {
                        if (!e->property("editing").toBool()) return;
                        e->setProperty("editing", false);
                        set_plug(plug, e->text().toDouble(), "float");
                    });
                } else {
                    e->setEnabled(false);
                }
                h->addWidget(e);
            }
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        const bool checklike = r.kind == "check" || r.kind == "channels" || (r.kind.isEmpty() && a["type"].toString() == "bool");
        if (r.kind == "freedom") {
            auto* rw = new QWidget(parent);
            auto* h = new QHBoxLayout(rw);
            h->setContentsMargins(2, 0, 12, 0);
            h->setSpacing(4);
            auto* lab = ae_label(label, rw);
            h->addWidget(lab);
            for (const char* axis : {"X", "Y", "Z"}) {
                QString name = QString("jointType") + axis;
                QJsonObject at = by.value(name);
                auto* cb = new QCheckBox(rw);
                cb->setFixedSize(14, 14);
                if (!at.isEmpty()) {
                    cb->setChecked(at["value"].toBool());
                    QString plug = at["plug"].toString();
                    cb->setProperty("plugKey", plug);
                    QObject::connect(cb, &QCheckBox::toggled, cb, [plug](bool on) { set_plug(plug, on, "bool"); });
                    used.insert(name);
                } else {
                    cb->setEnabled(false);
                }
                h->addWidget(cb);
                h->addWidget(new QLabel(axis, rw));
                h->addSpacing(18);
            }
            h->addStretch(1);
            v->addWidget(rw);
            continue;
        }
        QWidget* field = nullptr;
        if (!a.isEmpty()) {
            used.insert(r.attr);
            if (a["type"].toString() == "matrix") field = ae_matrix(a, parent);
            else if (a["type"].toString() == "vec3" && !a["color"].toBool() && r.kind != "color") field = ae_vec3(a, parent);
            else field = ae_field(r, a, style, parent);
            if (auto* cb = qobject_cast<QCheckBox*>(field)) cb->setProperty("plugKey", a["plug"].toString());
            if (auto* box = qobject_cast<QComboBox*>(field)) box->setProperty("plugKey", a["plug"].toString());
            if (checklike) {
                if (auto* cb = qobject_cast<QCheckBox*>(field)) cb->setText(label);
                else for (QCheckBox* cb : field->findChildren<QCheckBox*>()) cb->setText(label);
                attach_attr_menu(field, a, field);
            }
            if (a["type"].toString() == "string" && r.kind != "text" && r.kind != "combo") {
                field->setMinimumWidth(295);
            }
        } else {
            field = ae_placeholder(r, a, parent);
            if (checklike) {
                if (auto* cb = qobject_cast<QCheckBox*>(field)) cb->setText(label);
                else for (QCheckBox* cb : field->findChildren<QCheckBox*>()) cb->setText(label);
                attach_attr_menu(field, a, field);
            }
        }
        field->setToolTip(a.isEmpty() ? QString() : a["doc"].toString());
        auto* rowWidget = new QWidget(parent);
        auto* h = new QHBoxLayout(rowWidget);
        h->setContentsMargins(2, 0, 12, 0);
        h->setSpacing(4);
        if (checklike) {
            auto* pad = new QWidget(rowWidget);
            pad->setFixedWidth(160);
            h->addWidget(pad);
        } else {
            auto* lab = ae_label(label, rowWidget);
            if (!a.isEmpty()) {
                const QString doc = a["doc"].toString();
                lab->setToolTip(doc.isEmpty() ? a["attr"].toString() : doc);
                if (a["locked"].toBool()) {
                    lab->setProperty("locked", true);
                    lab->setEnabled(false);
                }
                attach_attr_menu(lab, a, field);
            } else {
                lab->setObjectName("aeDimLabel");
            }
            h->addWidget(lab);
        }
        h->addWidget(field);
        h->addStretch(1);
        v->addWidget(rowWidget);
    }
    Q_UNUSED(extra);
}

void ae_render_tree(QVBoxLayout* v, QWidget* parent, const QList<AeBox>& boxes, const QHash<QString, QJsonObject>& by, QSet<QString>& used,
                    const QString& style, const QString& owner) {
    for (const AeBox& b : boxes) {
        if (b.title == "Harold") {
            const QString key = b.title;
            const bool open = section_open(owner, key, b.open == 1);
            auto* headWrap = new QWidget(parent);
            auto* hw = new QHBoxLayout(headWrap);
            hw->setContentsMargins(18, 0, 18, 0);
            hw->setSpacing(0);
            auto* head = new AeHead(b.title, open, 0, headWrap);
            hw->addWidget(head);
            v->addWidget(headWrap);
            auto* content = new QWidget(parent);
            content->setObjectName("aeSectionBody");
            auto* cv = new QVBoxLayout(content);
            cv->setContentsMargins(0, 3, 0, 5);
            cv->setSpacing(2);
            content->setVisible(open);
            v->addWidget(content);
            head->toggled = [content, head, owner, key](bool on) {
                content->setVisible(on);
                head->set_open(on);
                remember_section(owner, key, on);
            };
            QList<QString> names = by.keys();
            names.sort();
            for (const QString& n : names) {
                if (!n.startsWith("ai") || used.contains(n)) continue;
                QJsonObject a = by.value(n);
                if (a["type"].toString() == "matrix") continue;
                used.insert(n);
                QWidget* field = attr_field(a, content);
                auto* rw = new QWidget(content);
                auto* h = new QHBoxLayout(rw);
                h->setContentsMargins(2, 0, 12, 0);
                h->setSpacing(4);
                auto* lab = ae_label(override_label(n, a["nice"].toString()), rw);
                h->addWidget(lab);
                h->addWidget(field);
                h->addStretch(1);
                cv->addWidget(rw);
            }
            continue;
        }
        if (b.title == "Extra Attributes") {
            const QString key = b.title;
            const bool open = section_open(owner, key, b.open == 1);
            auto* headWrap = new QWidget(parent);
            auto* hw = new QHBoxLayout(headWrap);
            hw->setContentsMargins(18, 0, 18, 0);
            hw->setSpacing(0);
            auto* head = new AeHead(b.title, open, 0, headWrap);
            hw->addWidget(head);
            v->addWidget(headWrap);
            auto* content = new QWidget(parent);
            content->setObjectName("aeSectionBody");
            auto* cv = new QVBoxLayout(content);
            cv->setContentsMargins(0, 3, 0, 5);
            cv->setSpacing(2);
            content->setVisible(open);
            v->addWidget(content);
            head->toggled = [content, head, owner, key](bool on) {
                content->setVisible(on);
                head->set_open(on);
                remember_section(owner, key, on);
            };
            ae_extra_rows(cv, content, by, used, style);
            continue;
        }
        ae_render_box(v, parent, b, by, used, style, 0, owner, QString());
    }
}

} // namespace

AttributeEditor::AttributeEditor(QWidget* parent, const QString& pinned) : Panel(parent), pinned_(pinned) {
    QFont editorFont;
    editorFont.setPixelSize(11);
    setFont(editorFont);
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(0, 0, 0, 0);
    lay->setSpacing(0);
    name_style_ = QSettings().value("ae/nameStyle", "Nice").toString();
    show_notes_ = QSettings().value("ae/showNotes", true).toBool();

    // The reference's menu bar above the tabs, and the pin at its right.
    auto* bar = new QWidget(this);
    bar->setObjectName("aeMenuBar");
    auto* barLay = new QHBoxLayout(bar);
    barLay->setContentsMargins(14, 0, 8, 0);
    barLay->setSpacing(0);
    auto menu_button = [&](const QString& text, QMenu* menu) {
        auto* b = new QToolButton(bar);
        b->setObjectName("aeMenuItem");
        b->setText(text);
        b->setPopupMode(QToolButton::InstantPopup);
        b->setMenu(menu);
        style_ae_menu(menu);
        b->setToolTip(text);
        barLay->addWidget(b);
        return b;
    };
    auto* listMenu = new QMenu(this);
    connect(listMenu, &QMenu::aboutToShow, this, [this, listMenu] {
        listMenu->clear();
        auto* pinAction = listMenu->addAction("Pin Tab");
        pinAction->setCheckable(true);
        pinAction->setChecked(!pinned_.isEmpty());
        connect(pinAction, &QAction::toggled, this, [this](bool on) {
            QWidget* w = tabs_->currentWidget();
            pinned_ = on && w ? w->property("nodePath").toString() : QString();
            refresh();
        });
        const QJsonArray selectedAttrs = ask("channelBox", QJsonObject{{"query", true}, {"selectedMainAttributes", true}}).toArray();
        auto* loadSel = listMenu->addAction("Load Selected Attributes");
        loadSel->setEnabled(!selectedAttrs.isEmpty());
        auto* selItems = listMenu->addAction("Selected Items");
        selItems->setCheckable(true);
        selItems->setChecked(false);
        selItems->setEnabled(false);
        listMenu->addSeparator();
        auto names_of = [](const QStringList& types) {
            QStringList names;
            for (const QString& t : types)
                for (const QJsonValue& v : ask("ls", QJsonObject{{"type", t}}).toArray()) names << v.toString();
            names.removeDuplicates();
            names.sort();
            return names;
        };
        auto add_category = [this, listMenu](const QString& title, const QStringList& names) {
            QMenu* sub = listMenu->addMenu(title);
            style_ae_menu(sub);
            if (names.isEmpty()) {
                sub->addAction("None")->setEnabled(false);
                sub->setEnabled(false);
                return;
            }
            for (const QString& n : names) {
                const QString leaf = n.section('|', -1);
                sub->addAction(leaf, this, [n] { exec("select", args(QJsonArray{n})); });
            }
        };
        add_category("Dynamics", names_of({"particle", "nParticle", "fluidShape", "nCloth", "nRigid", "rigidBody", "emitter"}));
        add_category("Kinematics", names_of({"joint", "ikHandle", "ikEffector"}));
        add_category("Deformers", names_of({"skinCluster", "blendShape", "cluster", "lattice", "baseLattice", "deformBend", "deformTwist",
                                            "deformSquash", "deformFlare", "deformSine", "deformWave", "softMod", "wire", "wrap", "shrinkWrap"}));
        QStringList lights;
        for (const QJsonValue& v : ask("ls", QJsonObject{{"lights", true}}).toArray()) lights << v.toString();
        lights.sort();
        add_category("Lights", lights);
        QStringList shading = names_of({"lambert", "blinn", "phong", "phongE", "anisotropic", "standardSurface", "aiStandardSurface"});
        for (const QJsonValue& v : ask("ls", QJsonObject{{"textures", true}}).toArray()) shading << v.toString();
        shading.removeDuplicates();
        shading.sort();
        add_category("Shading", shading);
    });
    menu_button("List", listMenu);

    auto* selectedMenu = new QMenu(this);
    connect(selectedMenu, &QMenu::aboutToShow, this, [this, selectedMenu] {
        selectedMenu->clear();
        QStringList names;
        for (const QJsonValue& v : ask("ls", QJsonObject{{"args", QJsonArray{}}, {"selection", true}}).toArray()) names << v.toString().section('|', -1);
        if (names.isEmpty()) {
            selectedMenu->addAction("No Selection")->setEnabled(false);
            return;
        }
        const QString focused = tabs_->currentWidget() ? tabs_->currentWidget()->property("nodePath").toString().section('|', -1) : QString();
        for (const QString& n : names) {
            auto* a = selectedMenu->addAction(n);
            a->setCheckable(true);
            a->setChecked(n == focused);
            connect(a, &QAction::triggered, this, [this, n] {
                for (int i = 0; i < tabs_->count(); ++i)
                    if (tabs_->tabText(i) == n) {
                        QWidget* w = tabs_->widget(i);
                        QString node = w ? w->property("nodePath").toString() : QString();
                        if (!node.isEmpty()) exec("select", args(QJsonArray{node}));
                        tabs_->setCurrentIndex(i);
                        return;
                    }
            });
        }
    });
    menu_button("Selected", selectedMenu);

    auto* focusMenu = new QMenu(this);
    connect(focusMenu, &QMenu::aboutToShow, this, [this, focusMenu] {
        focusMenu->clear();
        for (int i = 0; i < tabs_->count(); ++i) {
            QWidget* w = tabs_->widget(i);
            QString path = w ? w->property("nodePath").toString() : QString();
            auto* a = focusMenu->addAction(path);
            if (i == tabs_->currentIndex()) a->setCheckable(true), a->setChecked(true);
            connect(a, &QAction::triggered, this, [path] { exec("select", args(QJsonArray{path})); });
        }
    });
    menu_button("Focus", focusMenu);

    auto* attrsMenu = new QMenu(this);
    attrsMenu->addAction("Add Attributes...", this, [] { exec("AddAttribute"); });
    attrsMenu->addAction("Delete Attributes...", this, [this] { exec("DeleteAttribute"); });
    attrsMenu->addAction("Edit Attributes...", this, [this] { exec("EditAttribute"); });
    menu_button("Attributes", attrsMenu);

    auto* displayMenu = new QMenu(this);
    auto* namesMenu = displayMenu->addMenu("Attribute Names");
    auto* nice = namesMenu->addAction("Nice");
    auto* lng = namesMenu->addAction("Long");
    auto* shortNames = namesMenu->addAction("Short");
    for (auto* a : {nice, lng, shortNames}) a->setCheckable(true);
    nice->setChecked(name_style_ == "Nice");
    lng->setChecked(name_style_ == "Long");
    shortNames->setChecked(name_style_ == "Short");
    auto set_style = [this, nice, lng, shortNames](const QString& s) {
        name_style_ = s;
        QSettings().setValue("ae/nameStyle", s);
        nice->setChecked(s == "Nice");
        lng->setChecked(s == "Long");
        shortNames->setChecked(s == "Short");
        last_.clear();
        refresh();
    };
    connect(nice, &QAction::triggered, this, [set_style] { set_style("Nice"); });
    connect(lng, &QAction::triggered, this, [set_style] { set_style("Long"); });
    connect(shortNames, &QAction::triggered, this, [set_style] { set_style("Short"); });
    menu_button("Display", displayMenu);

    auto* showMenu = new QMenu(this);
    auto* byType = showMenu->addMenu("By Node Type");
    byType->addAction("None")->setEnabled(false);
    showMenu->addAction("Add New Filter Type...")->setEnabled(false);
    showMenu->addAction("Add Filter Types for Selected")->setEnabled(false);
    showMenu->addAction("Add Filter Types for Related")->setEnabled(false);
    showMenu->addAction("Remove Filter Type")->setEnabled(false);
    showMenu->addAction("Save Show Preset")->setEnabled(false);
    showMenu->addAction("Load Show Preset")->setEnabled(false);
    showMenu->addAction("Set Current View")->setEnabled(false);
    showMenu->addAction("Set Global View")->setEnabled(false);
    showMenu->addSeparator();
    auto* noGlobal = showMenu->addAction("Show default if no global view");
    noGlobal->setCheckable(true);
    noGlobal->setChecked(true);
    noGlobal->setEnabled(false);
    auto* viewLabels = showMenu->addAction("Show View Labels");
    viewLabels->setCheckable(true);
    viewLabels->setChecked(true);
    viewLabels->setEnabled(false);
    auto* showNotes = showMenu->addAction("Show Notes");
    showNotes->setCheckable(true);
    showNotes->setChecked(show_notes_);
    connect(showNotes, &QAction::toggled, this, [this](bool on) {
        show_notes_ = on;
        QSettings().setValue("ae/showNotes", on);
        last_.clear();
        refresh();
    });
    auto* colorPots = showMenu->addAction("Show Color Managed Pots");
    colorPots->setCheckable(true);
    colorPots->setChecked(true);
    colorPots->setEnabled(false);
    menu_button("Show", showMenu);

    auto* helpMenu = new QMenu(this);
    helpMenu->addAction("Help on Attribute Editor")->setEnabled(false);
    helpMenu->addSeparator();
    helpMenu->addAction("Help on \"mesh\"")->setEnabled(false);
    helpMenu->addAction("Help on \"openPBRSurface\"")->setEnabled(false);
    helpMenu->addAction("Help on \"polyCube\"")->setEnabled(false);
    helpMenu->addAction("Help on \"transform\"")->setEnabled(false);
    helpMenu->addSeparator();
    helpMenu->addAction("Node and Attribute Reference")->setEnabled(false);
    menu_button("Help", helpMenu);

    barLay->addStretch(1);
    auto* pin = new QToolButton(bar);
    pin->setObjectName("aePin");
    pin->setText(QString::fromUtf8("\u2934"));
    pin->setToolTip("Pin Tab");
    pin->setCheckable(true);
    pin->setChecked(!pinned_.isEmpty());
    connect(pin, &QToolButton::toggled, this, [this, pin](bool on) {
        QWidget* w = tabs_->currentWidget();
        pinned_ = on && w ? w->property("nodePath").toString() : QString();
        refresh();
    });
    barLay->addWidget(pin);
    lay->addWidget(bar);

    auto* tabWidget = new AeTabWidget(this);
    tabs_ = tabWidget;
    tabs_->setObjectName("aeTabs");
    tabs_->setDocumentMode(false);
    tabs_->setUsesScrollButtons(true);
    auto* tabBar = new AeTabBar(tabs_);
    tabWidget->use_bar(tabBar);
    tabs_->setMovable(true);
    connect(tabBar, &QTabBar::tabMoved, this, [this](int, int) {
        QStringList order;
        for (int i = 0; i < tabs_->count(); ++i) order << tabs_->tabText(i);
        tab_order() = order;
    });
    tabBar->tearOff = [this](int index) {
        QWidget* w = index >= 0 ? tabs_->widget(index) : nullptr;
        if (!w) return;
        auto* win = new AttributeEditor(main_window_widget(), w->property("nodePath").toString());
        win->setAttribute(Qt::WA_DeleteOnClose);
        win->setWindowFlags(editor_window_flags());
        win->setWindowTitle(tabs_->tabText(index));
        win->resize(480, 700);
        win->move(QCursor::pos() - QPoint(240, 40));
        win->show();
    };
    lay->addWidget(tabs_, 1);
    empty_ = new QLabel("Select an object in the scene to view and edit its attributes", this);
    empty_->setObjectName("aeEmpty");
    empty_->setAlignment(Qt::AlignCenter);
    empty_->hide();
    lay->addWidget(empty_, 1);
    connect(tabs_, &QTabWidget::currentChanged, this, [this](int i) {
        QWidget* w = i >= 0 ? tabs_->widget(i) : nullptr;
        current_tab_ = w ? w->property("stateKey").toString() : QString();
    });

    auto* bottom = new QHBoxLayout();
    bottom->setContentsMargins(4, 4, 4, 4);
    bottom->setSpacing(4);
    auto* sel = new QPushButton("Select", this);
    auto* load = new QPushButton("Load Attributes", this);
    auto* copy = new QPushButton("Copy Tab", this);
    sel->setToolTip("Select the node shown");
    load->setToolTip("Load the attributes selected in the Channel Box");
    copy->setToolTip("Open the current tab in a window of its own");
    for (auto* b : {sel, load, copy}) {
        b->setObjectName("aeBottomButton");
        b->setMinimumHeight(23);
        bottom->addWidget(b, 1);
    }
    lay->addLayout(bottom);
    connect(sel, &QPushButton::clicked, this, [this] {
        QWidget* w = tabs_->currentWidget();
        if (w) exec("select", args(QJsonArray{w->property("nodePath").toString()}));
    });
    connect(load, &QPushButton::clicked, this, [this] { refresh(); });
    connect(copy, &QPushButton::clicked, this, [this] {
        QWidget* w = tabs_->currentWidget();
        if (!w) return;
        // A window of its own that stays on this node.
        auto* win = new AttributeEditor(main_window_widget(), w->property("nodePath").toString());
        win->setAttribute(Qt::WA_DeleteOnClose);
        win->setWindowFlags(editor_window_flags());
        win->setWindowTitle(tabs_->tabText(tabs_->currentIndex()));
        win->resize(480, 700);
        win->show();
    });
}

void AttributeEditor::refresh() {
    QJsonObject ae = query("attributeEditor", pinned_).toObject();
    const QByteArray now = QJsonDocument(ae).toJson(QJsonDocument::Compact);
    if (now == last_ && tabs_->count()) return;
    // A slider being dragged changes the value this is built from: rebuilding would take the slider from under the
    // mouse. The release comes back here.
    for (QSlider* sl : findChildren<QSlider*>())
        if (sl->isSliderDown()) return;
    last_ = now;
    // The tab on show, and the node the selection gave the panel, both by their key (the node's UUID, or its path
    // when it has none): a rename keeps them, and only a new focus is a selection change that moves to another tab.
    QString keep = current_tab_;
    if (auto* page = tabs_->currentWidget()) {
        const QString key = page->property("stateKey").toString();
        if (!key.isEmpty()) keep = key;
    }
    const QString focus_path = ae["focus"].toString();
    // The field being typed in, and where the tab was scrolled to, come back after the rebuild: a value set and Tab
    // to the next field rebuilds the editor, and the cursor goes on to that next field, as in the reference.
    QString focus_key;
    QString focus_text;
    bool focus_edited = false;
    int caret = -1;
    int sel_start = -1;
    int sel_end = -1;
    if (auto* fe = qobject_cast<QLineEdit*>(QApplication::focusWidget()); fe && isAncestorOf(fe)) {
        focus_key = fe->property("plugKey").toString();
        if (focus_key.isEmpty() && fe->property("nameField").toBool()) focus_key = "__name:" + fe->property("nodeName").toString();
        focus_edited = fe->property("editing").toBool();
        focus_text = fe->text();
        caret = fe->cursorPosition();
        sel_start = fe->selectionStart();
        sel_end = fe->selectionEnd();
    }
    // Where this node was scrolled to, kept per node so a rebuild and a tab switch come back to it.
    if (auto* page = tabs_->currentWidget()) {
        if (auto* sa = page->findChild<QScrollArea*>()) {
            const QString key = page->property("stateKey").toString();
            if (!key.isEmpty()) scroll_states()[key] = sa->verticalScrollBar()->value();
        }
    }
    tabs_->blockSignals(true);
    while (tabs_->count()) {
        QWidget* w = tabs_->widget(0);
        tabs_->removeTab(0);
        w->deleteLater();
    }
    for (const QJsonValue& tv : ae["tabs"].toArray()) {
        QJsonObject t = tv.toObject();
        QString node_type = t["type"].toString();
        auto* scroll = new QScrollArea();
        scroll->setWidgetResizable(true);
        scroll->setFrameShape(QFrame::NoFrame);
        // Rows fit the panel's width: the sliders give, so the map buttons at the rows' ends stay in sight.
        scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        auto* body = new QWidget();
        body->setObjectName("aePage");
        body->setMinimumWidth(40);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 8, 0, 8);
        v->setSpacing(2);

        QString path = t["path"].toString();
        scroll->setProperty("nodePath", path);
        QString state_key = t["uuid"].toString();
        if (state_key.isEmpty()) state_key = path;

        QJsonArray attrs = t["attrs"].toArray();
        QHash<QString, QJsonObject> by;
        QJsonValue notes;
        for (const QJsonValue& av : attrs) {
            QJsonObject a = av.toObject();
            by.insert(a["attr"].toString(), a);
            if (a["attr"].toString() == "notes") notes = a["value"];
        }
        QString notes_plug = path + ".notes";
        if (t.contains("uuid")) {
            QJsonObject u;
            u["attr"] = "__uuid";
            u["value"] = t["uuid"];
            by.insert("__uuid", u);
        }
        if (t.contains("geometry")) {
            QJsonObject g;
            g["attr"] = "__geometry";
            g["value"] = t["geometry"];
            by.insert("__geometry", g);
        }
        static const QSet<QString> light_node_types{"ambientLight", "directionalLight", "pointLight", "spotLight", "areaLight", "volumeLight"};
        if (light_node_types.contains(node_type)) {
            static const QStringList order{"ambientLight", "directionalLight", "pointLight", "spotLight", "areaLight", "volumeLight"};
            QJsonObject kind;
            kind["attr"] = "__lightType";
            kind["plug"] = path;
            kind["type"] = "int";
            kind["value"] = order.indexOf(node_type);
            by.insert("__lightType", kind);
            QJsonObject lit;
            lit["attr"] = "__illuminates";
            lit["plug"] = path;
            lit["type"] = "bool";
            lit["value"] = ask("aztekIlluminatesByDefault", args(QJsonArray{path})).toBool();
            by.insert("__illuminates", lit);
        }

        // The node's type above its name field, with the reference's connection buttons, Focus, Presets and Show/Hide.
        auto* nameRow = new QWidget(body);
        auto* nh = new QHBoxLayout(nameRow);
        nh->setContentsMargins(2, 2, 18, 10);
        nh->setSpacing(4);
        auto* typeLabel = new QLabel(ae_type_label(node_type) + ":", nameRow);
        typeLabel->setObjectName("aeTypeLabel");
        typeLabel->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
        typeLabel->setFixedWidth(160);
        typeLabel->setToolTip(path);
        nh->addWidget(typeLabel);
        auto* name = new QLineEdit(nameRow);
        name->setObjectName("aeField");
        name->setProperty("nameField", true);
        name->setProperty("nodeName", t["name"].toString());
        name->setFixedSize(152, 20);
        name->setText(t["name"].toString());
        name->setToolTip(path);
        connect(name, &QLineEdit::editingFinished, name, [name, path] {
            const QString typed = name->text().trimmed();
            const QString current = name->property("nodeName").toString();
            if (typed.isEmpty()) {
                name->setText(current);
                return;
            }
            if (typed != current) {
                // A rename that the scene refuses (a default node, a name in use) leaves the field on the real name.
                if (exec("rename", args(QJsonArray{path, typed})).isUndefined()) name->setText(current);
                else name->setProperty("nodeName", typed);
            }
        });
        nh->addWidget(name);
        auto* links = new QWidget(nameRow);
        auto* lv = new QVBoxLayout(links);
        lv->setContentsMargins(0, 0, 0, 0);
        lv->setSpacing(1);
        auto* in = new AeIconButton([] { return connect_icon(true); }, links);
        in->setObjectName("aeLinkButton");
        in->setIconSize(QSize(14, 14));
        in->setFixedSize(18, 9);
        const QString inputNode = t["inputNode"].toString();
        in->setToolTip(inputNode.isEmpty() ? "Go to input connection" : "Go to input connection: " + inputNode);
        in->setEnabled(!inputNode.isEmpty());
        if (!inputNode.isEmpty())
            connect(in, &QToolButton::clicked, in, [inputNode] { exec("select", args(QJsonArray{inputNode})); });
        auto* out = new AeIconButton([] { return connect_icon(false); }, links);
        out->setObjectName("aeLinkButton");
        out->setIconSize(QSize(14, 14));
        out->setFixedSize(18, 9);
        const QString outputNode = t["outputNode"].toString();
        out->setToolTip(outputNode.isEmpty() ? "Go to output connection" : "Go to output connection: " + outputNode);
        out->setEnabled(!outputNode.isEmpty());
        if (!outputNode.isEmpty())
            connect(out, &QToolButton::clicked, out, [outputNode] { exec("select", args(QJsonArray{outputNode})); });
        lv->addWidget(in);
        lv->addWidget(out);
        nh->addWidget(links);
        nh->addSpacing(8);
        auto* right = new QWidget(nameRow);
        right->setFixedWidth(62);
        auto* rv = new QVBoxLayout(right);
        rv->setContentsMargins(0, 0, 0, 0);
        rv->setSpacing(1);
        auto* focus = new QPushButton("Focus", right);
        focus->setObjectName("aeSmallButton");
        focus->setFixedHeight(20);
        focus->setToolTip("Select this node");
        connect(focus, &QPushButton::clicked, focus, [path] { exec("select", args(QJsonArray{path})); });
        rv->addWidget(focus);
        auto* presets = new QToolButton(right);
        presets->setObjectName("aeSmallButton");
        presets->setText("Presets");
        presets->setFixedHeight(20);
        presets->setPopupMode(QToolButton::InstantPopup);
        auto* pmenu = new QMenu(presets);
        style_ae_menu(pmenu);
        presets->setMenu(pmenu);
        rv->addWidget(presets);
        connect(pmenu, &QMenu::aboutToShow, this, [this, pmenu] {
            pmenu->clear();
            QWidget* w = tabs_->currentWidget();
            const QString node = w ? w->property("nodePath").toString() : QString();
            if (node.isEmpty()) {
                pmenu->addAction("No node")->setEnabled(false);
                return;
            }
            const QString type = exec_quiet("nodeType", QJsonObject{{"args", QJsonArray{node}}}).toString();
            pmenu->addAction(QString("Save %1 Preset...").arg(type), this, [this, node] {
                bool ok = false;
                const QString name = QInputDialog::getText(this, "Save Attribute Preset", "Preset name:", QLineEdit::Normal, QString(), &ok).trimmed();
                if (ok && !name.isEmpty()) exec("nodePreset", QJsonObject{{"save", QJsonArray{node, name}}});
            });
            pmenu->addAction("Import Reference Presets...", this, [this] {
                const QStringList files = QFileDialog::getOpenFileNames(this, "Import Reference Presets", QString(), "MEL presets (*.mel)");
                if (files.isEmpty()) return;
                exec("importAttrPresets", QJsonObject{{"args", QJsonArray::fromStringList(files)}});
            });
            const QJsonArray list = exec_quiet("nodePreset", QJsonObject{{"list", node}}).toArray();
            if (!list.isEmpty()) pmenu->addSeparator();
            for (const QJsonValue& pv : list) {
                const QString preset = pv.toString();
                QMenu* sub = pmenu->addMenu(preset);
                sub->addAction("Replace", this, [node, preset] { exec("applyAttrPreset", QJsonObject{{"args", QJsonArray{node, preset, 100}}}); });
                for (int pct : {90, 75, 50, 25, 10})
                    sub->addAction(QString("Blend %1%").arg(pct), this, [node, preset, pct] { exec("applyAttrPreset", QJsonObject{{"args", QJsonArray{node, preset, pct}}}); });
                sub->addSeparator();
                sub->addAction("Delete", this, [node, preset] { exec("nodePreset", QJsonObject{{"delete", QJsonArray{node, preset}}}); });
            }
        });
        auto* showHide = new QWidget(right);
        auto* sh = new QHBoxLayout(showHide);
        sh->setContentsMargins(0, 0, 0, 0);
        sh->setSpacing(1);
        auto* show = new QPushButton("Show", showHide);
        show->setObjectName("aeTinyButton");
        show->setFixedHeight(20);
        show->setToolTip("Show the node");
        show->setEnabled(by.contains("visibility"));
        auto* hide = new QPushButton("Hide", showHide);
        hide->setObjectName("aeTinyButton");
        hide->setFixedHeight(20);
        hide->setToolTip("Hide the node");
        hide->setEnabled(by.contains("visibility"));
        sh->addWidget(show);
        sh->addWidget(hide);
        connect(show, &QPushButton::clicked, show, [path] { exec("setAttr", QJsonObject{{"args", QJsonArray{path + ".visibility", true}}}); });
        connect(hide, &QPushButton::clicked, hide, [path] { exec("setAttr", QJsonObject{{"args", QJsonArray{path + ".visibility", false}}}); });
        rv->addWidget(showHide);
        nh->addWidget(right);
        nh->addStretch(1);
        v->addWidget(nameRow);
        static const QSet<QString> material_types{"lambert", "blinn", "phong", "phongE", "anisotropic", "standardSurface",
                                                   "aiStandardSurface", "openPBRSurface", "surfaceShader"};
        if (material_types.contains(node_type)) v->addWidget(material_header(node_type, by, body));
        if (light_node_types.contains(node_type)) v->addWidget(light_header(node_type, body));

        QList<AeBox> boxes = ae_tree(node_type);
        if (node_type == "nonLinear") {
            for (AeBox& b : boxes)
                if (b.title == "Handle Display") b.rows.removeIf([&by](const AeRow& r) { return r.attr == "handleWidth" && !by.contains("handleWidth"); });
        }
        // A blend shape's target weights, named after their targets as the reference names them.
        if (node_type == "blendShape") {
            QList<AeRow> weightRows;
            for (const QJsonValue& wv : t["weights"].toArray()) {
                QJsonObject w = wv.toObject();
                QString key = QString("weight[%1]").arg(w["index"].toInt());
                QJsonObject a;
                a["attr"] = key;
                a["plug"] = w["plug"];
                a["nice"] = w["name"];
                a["type"] = "float";
                a["value"] = w["value"];
                a["min"] = 0.0;
                a["max"] = 1.0;
                by.insert(key, a);
                weightRows << row(key.toUtf8().constData());
            }
            for (AeBox& b : boxes) {
                if (b.title != "Weight") continue;
                QList<AeRow> all = weightRows;
                all.append(b.rows);
                b.rows = all;
            }
        }
        const bool tree_layout = !boxes.isEmpty();
        QSet<QString> used;
        if (tree_layout) {
            // Attributes the reference's template suppresses never reach Extra Attributes.
            static const QStringList xfSuppressed{"rotationInterpolation", "pivotOrient", "intermediateObject", "specifiedManipLocation",
                                                  "message", "useObjectColor", "objectColor", "objectColorRGB", "wireColorRGB"};
            if (node_type == "transform" || node_type == "joint" || node_type == "locator" || node_type == "ikHandle")
                for (const QString& n : xfSuppressed) used.insert(n);
            if (node_type == "joint")
                for (const QString& n : {"inverseScale", "bindPose", "bindRotation", "bindJointOrient", "bindScale", "bindInverseScale",
                                         "bindSegmentScaleCompensate", "jointOrientType", "jointType", "dofMask", "fkRotate", "ikRotate"})
                    used.insert(n);
            if (node_type == "mesh")
                for (const QString& n : {"symmetry", "smoothPreview", "symmetryPlane", "symmetryOffset", "radialCount", "radialAxis",
                                         "radialDirection", "radialDirectionX", "radialDirectionY", "radialDirectionZ", "radialMirror",
                                         "radialPhase", "radialCenter", "radialCenterX", "radialCenterY", "radialCenterZ", "displayMapBorders",
                                         "message", "componentTags", "instObjGroups"})
                    used.insert(n);
            ae_render_tree(v, body, boxes, by, used, name_style_, state_key);
        } else {
        // A parent constraint's offset from each target, as the reference's Target Offsets: Update takes the object's
        // place now as the offset, Key keys the offsets (#121).
        if (node_type == "parentConstraint") {
            auto* form = add_section(v, body, "Parent Constraint Target Offsets", state_key);
            const QJsonArray targets = ask("parentConstraint", QJsonObject{{"args", QJsonArray{path}}, {"query", true}, {"targetList", true}, {"queried", QJsonArray{"targetList"}}}).toArray();
            const QJsonArray driven = ask("listRelatives", QJsonObject{{"args", QJsonArray{path}}, {"parent", true}}).toArray();
            for (int i = 0; i < targets.size(); ++i) {
                const QString target = targets.at(i).toString();
                form->addRow(new QLabel("<b>" + target.section('|', -1) + "</b>", body));
                for (auto [label, attr] : {std::pair{"Translate", "targetOffsetTranslate"}, std::pair{"Rotate", "targetOffsetRotate"}}) {
                    const QString plug = QString("%1.target[%2].%3").arg(path).arg(i).arg(attr);
                    QJsonArray val = ask("getAttr", QJsonObject{{"args", QJsonArray{plug}}}).toArray();
                    if (val.size() == 1 && val.at(0).isArray()) val = val.at(0).toArray();
                    auto* row = new QWidget(body);
                    auto* h = new QHBoxLayout(row);
                    h->setContentsMargins(0, 0, 0, 0);
                    QList<QLineEdit*> fields;
                    for (int k = 0; k < 3; ++k) {
                        auto* f = new QLineEdit(fmt3(val.at(k).toDouble()), row);
                        f->setProperty("plugKey", plug + QString::number(k));
                        fields << f;
                        h->addWidget(f);
                    }
                    for (QLineEdit* f : fields)
                        connect(f, &QLineEdit::editingFinished, f, [plug, fields] {
                            exec("setAttr", QJsonObject{{"args", QJsonArray{plug, fields[0]->text().toDouble(), fields[1]->text().toDouble(), fields[2]->text().toDouble()}}});
                        });
                    form->addRow(label, row);
                }
                auto* buttons = new QWidget(body);
                auto* bh = new QHBoxLayout(buttons);
                bh->setContentsMargins(0, 0, 0, 0);
                auto* update = new QPushButton("Update", buttons);
                auto* key = new QPushButton("Key", buttons);
                const QString obj = driven.isEmpty() ? QString() : driven.at(0).toString();
                connect(update, &QPushButton::clicked, update, [target, obj] {
                    if (!obj.isEmpty()) exec("parentConstraint", QJsonObject{{"args", QJsonArray{target, obj}}, {"edit", true}, {"maintainOffset", true}});
                });
                connect(key, &QPushButton::clicked, key, [path, i] {
                    for (const char* a : {"targetOffsetTranslate", "targetOffsetRotate"})
                        exec("setKeyframe", QJsonObject{{"args", QJsonArray{QString("%1.target[%2].%3").arg(path).arg(i).arg(a)}}});
                });
                bh->addWidget(update);
                bh->addWidget(key);
                form->addRow(QString(), buttons);
            }
        }

        // Cloth: material presets.
        if (node_type == "cloth") {
            auto* form = add_section(v, body, "Presets", state_key);
            auto* preset = new QComboBox(body);
            preset->addItem("Choose a material...");
            for (const char* p : {"Silk", "Cotton", "Leather", "Heavy Denim", "Rubber"}) preset->addItem(p);
            connect(preset, QOverload<int>::of(&QComboBox::activated), preset, [path, preset](int i) {
                static const QStringList names{"", "silk", "cotton", "leather", "heavyDenim", "rubber"};
                if (i > 0) exec("clothPreset", QJsonObject{{"args", QJsonArray{path}}, {"preset", names.at(i)}});
                preset->setCurrentIndex(0);
            });
            form->addRow("Material", preset);
        }

        QJsonArray attrs = t["attrs"].toArray();
        // A script's AE<type>Template lays the tab out: what it does not name shows only under -addExtraControls.
        QJsonArray layout = query("aeTemplate", node_type + " " + path).toArray();
        bool extra = layout.isEmpty();
        QFormLayout* current = nullptr;
        for (const QJsonValue& iv : layout) {
            QJsonObject item = iv.toObject();
            QString kind = item["kind"].toString();
            if (kind == "beginLayout") {
                current = add_section(v, body, item["title"].toString(), state_key);
            } else if (kind == "endLayout") {
                current = nullptr;
            } else if (kind == "separator") {
                auto* line = new QFrame(body);
                line->setFrameShape(QFrame::HLine);
                if (current) current->addRow(line);
                else v->addWidget(line);
            } else if (kind == "suppress") {
                used.insert(item["attr"].toString());
            } else if (kind == "extraControls") {
                extra = true;
            } else if (kind == "control") {
                const QString n = item["attr"].toString();
                for (const QJsonValue& av : attrs) {
                    QJsonObject a = av.toObject();
                    if (a["attr"].toString() != n) continue;
                    if (!current) current = add_section(v, body, node_type, state_key);
                    used.insert(n);
                    QWidget* field = attr_field(a, body);
                    field->setToolTip(item.contains("annotation") ? item["annotation"].toString() : a["doc"].toString());
                    QWidget* lab = attr_label(item.contains("label") ? item["label"].toString() : a["nice"].toString(), a, body, field);
                    // An annotation covers the whole row unless -annotateFieldOnly keeps it to the field.
                    if (lab && item.contains("annotation") && !item["annotateFieldOnly"].toBool()) lab->setToolTip(item["annotation"].toString());
                    current->addRow(lab, field);
                }
            }
        }
        // Drawing overrides do nothing until they are enabled, and show the index or RGB colour that is in use.
        bool overrides_on = false, overrides_rgb = false;
        for (const QJsonValue& av : attrs) {
            QJsonObject a = av.toObject();
            if (a["attr"].toString() == "overrideEnabled") overrides_on = a["value"].toBool();
            if (a["attr"].toString() == "overrideRGBColors") overrides_rgb = a["value"].toBool();
        }
        for (const Section& sec : layout.isEmpty() ? sections_for(node_type) : QList<Section>()) {
            QStringList names = QString(sec.attrs).split(' ');
            QFormLayout* f = nullptr;
            bool drawing = QString(sec.title) == "Drawing Overrides";
            for (const QString& n : names)
                for (const QJsonValue& av : attrs) {
                    QJsonObject a = av.toObject();
                    if (a["attr"].toString() != n || a["type"].toString() == "matrix") continue;
                    if (!f) f = add_section(v, body, sec.title, state_key);
                    used.insert(n);
                    if (drawing && n == (overrides_rgb ? "overrideColor" : "overrideColorRGB")) continue;
                    QWidget* field = attr_field(a, body);
                    field->setToolTip(a["doc"].toString());
                    if (drawing && n != "overrideEnabled") field->setEnabled(overrides_on);
                    const bool standard = node_type == "standardSurface" || node_type == "aiStandardSurface";
                    f->addRow(attr_label(standard ? material_label(sec.title, n, a["nice"].toString()) : override_label(n, a["nice"].toString()), a, body, field), field);
                }
        }
        // The rest grouped by their own category, then Extra Attributes.
        QMap<QString, QFormLayout*> forms;
        for (const QJsonValue& av : attrs) {
            QJsonObject a = av.toObject();
            QString n = a["attr"].toString();
            if (n == "notes") {
                notes = a["value"];
                continue;
            }
            if (used.contains(n) || a["type"].toString() == "matrix" || !extra) continue;
            QString cat = a["category"].toString();
            if (cat.isEmpty()) cat = sections_for(node_type).size() > 1 ? QString("Extra Attributes") : QString("%1 Attributes").arg(node_type.left(1).toUpper() + node_type.mid(1));
            if (!forms.contains(cat)) forms[cat] = add_section(v, body, cat, state_key);
            QWidget* field = attr_field(a, body);
            field->setToolTip(a["doc"].toString());
            forms[cat]->addRow(attr_label(a["nice"].toString(), a, body, field), field);
        }
        }
        // Notes: a string attribute added the first time it's written, in the reference's fixed area under the sections.
        auto* page = new QWidget();
        page->setProperty("nodePath", path);
        page->setProperty("stateKey", state_key);
        auto* pl = new QVBoxLayout(page);
        pl->setContentsMargins(0, 0, 0, 0);
        pl->setSpacing(0);
        pl->addWidget(scroll, 1);
        if (show_notes_) {
            auto* notesArea = new QWidget(page);
            notesArea->setObjectName("aeNotesArea");
            auto* nv = new QVBoxLayout(notesArea);
            nv->setContentsMargins(10, 2, 12, 2);
            nv->setSpacing(1);
            auto* dots = new QWidget(notesArea);
            dots->setFixedHeight(6);
            auto* dl = new QHBoxLayout(dots);
            dl->setContentsMargins(0, 4, 0, 0);
            dl->addStretch(1);
            auto* line = new QFrame(dots);
            line->setObjectName("aeDottedLine");
            line->setFixedSize(146, 1);
            dl->addWidget(line);
            dl->addStretch(1);
            nv->addWidget(dots);
            auto* note_head = new QLabel("Notes: " + t["name"].toString(), notesArea);
            note_head->setObjectName("aeNotesLabel");
            nv->addWidget(note_head);
            auto* note = new QPlainTextEdit(notes.toString(), notesArea);
            note->setObjectName("aeNotes");
            note->setFixedHeight(96);
            note->setProperty("editing", false);
            bool had = !notes.isUndefined();
            connect(note, &QPlainTextEdit::textChanged, note, [note] { note->setProperty("editing", true); });
            note->installEventFilter(this);
            note->setProperty("notesPlug", notes_plug);
            note->setProperty("notesNode", path);
            note->setProperty("notesHad", had);
            nv->addWidget(note);
            pl->addWidget(notesArea);
        }
        v->addStretch(1);
        fit_labels(body);
        scroll->setWidget(body);
        auto* vbar = scroll->verticalScrollBar();
        // The page's own scroll position, restored after the layout has given the page its height and again once the
        // fields have settled. Until then, a focus scrolling the view does not overwrite what the user left.
        const int restore_to = scroll_states().value(state_key);
        vbar->setProperty("restoreTarget", restore_to);
        vbar->setProperty("settling", true);
        QObject::connect(vbar, &QScrollBar::valueChanged, scroll, [state_key, vbar](int v) {
            if (!vbar->property("settling").toBool()) scroll_states()[state_key] = v;
        });
        auto restore_scroll = [vbar] { vbar->setValue(qMin(vbar->property("restoreTarget").toInt(), vbar->maximum())); };
        QTimer::singleShot(0, vbar, restore_scroll);
        QTimer::singleShot(50, vbar, restore_scroll);
        // A hidden page's bar is settled on its own; the current page's is settled after its focus comes back.
        QTimer::singleShot(250, vbar, [vbar] { vbar->setProperty("settling", false); });
        for (QLineEdit* e : page->findChildren<QLineEdit*>()) {
            const QString old = e->objectName();
            if (old == "keyedField") e->setProperty("keyed", true);
            else if (old == "connectedField") e->setProperty("connected", true);
            else if (old == "lockedField") e->setProperty("locked", true);
            e->setObjectName("aeField");
            e->installEventFilter(this);
        }
        tabs_->addTab(page, t["name"].toString());
    }
    // The order the tabs were dragged into, for the tabs still here. Signals stay off until the tab is chosen, or
    // the removals and moves would write a tab into current_tab_ that is not on show.
    if (!tab_order().isEmpty()) {
        tabs_->tabBar()->blockSignals(true);
        int at = 0;
        for (const QString& name : tab_order()) {
            for (int i = at; i < tabs_->count(); ++i)
                if (tabs_->tabText(i) == name) {
                    tabs_->tabBar()->moveTab(i, at++);
                    break;
                }
        }
        tabs_->tabBar()->blockSignals(false);
    }
    // Find the tab to show now that the order is settled. A new focus (another node selected, or the panel first
    // built) shows that node's tab; any other refresh keeps the tab the user has on show.
    int select = -1;
    int focus_index = -1;
    QString focus_node_key;
    for (int i = 0; i < tabs_->count(); ++i) {
        QWidget* w = tabs_->widget(i);
        const QString key = w ? w->property("stateKey").toString() : QString();
        if (key == keep) select = i;
        if (w && w->property("nodePath").toString() == focus_path) {
            focus_index = i;
            focus_node_key = key;
        }
    }
    const bool selection_changed = focus_node_key != last_focus_;
    last_focus_ = focus_node_key;
    if (selection_changed && focus_index >= 0) select = focus_index;
    else if (select < 0) select = focus_index;
    if (select < 0) select = 0;
    tabs_->blockSignals(false);
    tabs_->setCurrentIndex(select);
    if (QWidget* shown = tabs_->currentWidget()) current_tab_ = shown->property("stateKey").toString();
    else current_tab_.clear();
    // Nothing selected: the reference's own message in the panel.
    if (tabs_->count() == 0) {
        tabs_->hide();
        empty_->show();
    } else {
        empty_->hide();
        tabs_->show();
    }
    // The focused field must not scroll the page away from where the user left it: put the saved position back
    // after the focus has done its own ensure-visible, and settle the bar a moment later.
    if (auto* page = tabs_->currentWidget()) {
        if (auto* sa = page->findChild<QScrollArea*>()) {
            auto* bar = sa->verticalScrollBar();
            const QString key = page->property("stateKey").toString();
            auto restore = [bar, key] { bar->setValue(qMin(scroll_states().value(key), bar->maximum())); };
            bar->setProperty("settling", true);
            restore();
            QTimer::singleShot(30, bar, restore);
            QTimer::singleShot(80, bar, [bar, restore] { restore(); bar->setProperty("settling", false); });
        }
    }
    if (!focus_key.isEmpty())
        for (QLineEdit* e : tabs_->currentWidget() ? tabs_->currentWidget()->findChildren<QLineEdit*>() : QList<QLineEdit*>())
            if (e->property("plugKey").toString() == focus_key ||
                (e->property("nameField").toBool() && focus_key == "__name:" + e->property("nodeName").toString())) {
                // A value half typed stays as typed, and the caret and selection stay where they were.
                if (focus_edited) {
                    e->setText(focus_text);
                    e->setProperty("editing", true);
                }
                e->setFocus(Qt::TabFocusReason);
                if (caret >= 0) {
                    e->setCursorPosition(qBound(0, caret, e->text().size()));
                    if (sel_start >= 0 && sel_end >= 0) {
                        const int a = qBound(0, qMin(sel_start, sel_end), e->text().size());
                        const int b = qBound(0, qMax(sel_start, sel_end), e->text().size());
                        e->setSelection(a, b - a);
                    }
                } else {
                    e->selectAll();
                }
                break;
            }
}

bool AttributeEditor::eventFilter(QObject* o, QEvent* e) {
    // A middle drag on a number scrubs it, as the reference's fields do.
    if (auto* le = qobject_cast<QLineEdit*>(o)) {
        if (e->type() == QEvent::MouseButtonPress && le->property("nameField").toBool()) {
            auto* me = static_cast<QMouseEvent*>(e);
            if (me->button() == Qt::LeftButton) QTimer::singleShot(0, le, &QLineEdit::selectAll);
        }
        if (e->type() == QEvent::KeyPress) {
            auto* ke = static_cast<QKeyEvent*>(e);
            if (ke->key() == Qt::Key_Escape && le->property("nameField").toBool()) {
                le->setText(le->property("nodeName").toString());
                le->setProperty("editing", false);
                le->selectAll();
                return true;
            }
        }
        const QString plug = le->property("plugKey").toString();
        if (!plug.isEmpty()) {
            if (e->type() == QEvent::MouseButtonPress) {
                auto* me = static_cast<QMouseEvent*>(e);
                if (me->button() == Qt::MiddleButton) {
                    le->setProperty("aeDragX", int(me->globalPosition().x()));
                    le->setProperty("aeDragStart", le->text().toDouble());
                    le->setProperty("aeDragging", true);
                    return true;
                }
            } else if (e->type() == QEvent::MouseMove && le->property("aeDragging").toBool()) {
                auto* me = static_cast<QMouseEvent*>(e);
                const int dx = int(me->globalPosition().x()) - le->property("aeDragX").toInt();
                const double start = le->property("aeDragStart").toDouble();
                const double step = qMax(0.01, qAbs(start) * 0.01);
                const double value = start + dx * step;
                le->setText(QString::number(value, 'f', 3));
                exec_quiet("setAttr", args(QJsonArray{plug, value}));
                Host::get()->poke();
                return true;
            } else if (e->type() == QEvent::MouseButtonRelease && le->property("aeDragging").toBool()) {
                le->setProperty("aeDragging", false);
                exec("setAttr", args(QJsonArray{plug, le->text().toDouble()}));
                return true;
            }
        }
    }
    // A field takes its whole value on entering it, as the Channel Box's and the reference's fields do.
    if (e->type() == QEvent::FocusIn) {
        if (auto* le = qobject_cast<QLineEdit*>(o))
            if (!le->property("plugKey").toString().isEmpty() || le->property("nameField").toBool())
                QTimer::singleShot(0, le, &QLineEdit::selectAll);
    }
    if (e->type() == QEvent::FocusOut) {
        auto* note = qobject_cast<QPlainTextEdit*>(o);
        if (note && note->property("editing").toBool()) {
            note->setProperty("editing", false);
            if (!note->property("notesHad").toBool()) {
                exec("addAttr", QJsonObject{{"args", QJsonArray{note->property("notesNode").toString()}}, {"longName", "notes"}, {"dataType", "string"}});
                note->setProperty("notesHad", true);
            }
            set_plug(note->property("notesPlug").toString(), note->toPlainText(), "string");
        }
    }
    return Panel::eventFilter(o, e);
}

} // namespace az
