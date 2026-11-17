// The UV Editor: the selected meshes' UVs in 0-1 space, picked and moved by UV, shell, edge or face, with the reference's
// projection, unfold, layout, cut and sew tools.
#pragma once
#include <QHash>
#include <QStringList>
class QComboBox;
#include <QImage>
#include <QJsonArray>
#include <array>
#include <QJsonObject>
#include <QPointF>
#include <QPolygonF>
#include <QSet>
#include <QVector>
#include <QWidget>

namespace az {

struct UvMesh {
    QString path;
    QVector<QPointF> maps;
    QVector<int> map_point;
    struct Face {
        int id = 0, shell = 0;
        QVector<int> maps, edges;
        // Its area on the surface, for UV Distortion.
        double area3d = 0;
    };
    QVector<Face> faces;
    // map a, map b, edge
    QVector<std::array<int, 3>> borders;
    QSet<int> sel_faces, sel_edges, sel_points;
};

class UvCanvas : public QWidget {
    Q_OBJECT
public:
    explicit UvCanvas(QWidget* parent = nullptr);
    void set_data(const QJsonObject& d);
    void set_mode(const QString& m);
    QString mode() const { return mode_; }
    void set_tool(const QString& t);
    QString tool() const { return tool_; }
    void frame(bool selected);
    // The UVs to act on, as polyEditUV arguments (pCube1.map[...]); falls back to the scene selection.
    QJsonArray targets() const;
    // The meshes shown, by path; the middle of what is selected (x below -1e8 when nothing is).
    QStringList mesh_paths() const;
    QPointF selection_center() const;
    // The selected UVs' bounds (empty when none).
    QRectF selection_bounds() const;
    // Edit Pivot: a click in the editor puts the pivot the Transform section turns and scales about.
    bool editing_pivot = false;
    bool has_pivot = false;
    QPointF pivot_uv;
    // Every UV of the meshes shown selected, or with invert those not selected now.
    void select_all_uvs(bool invert);
    bool show_checker = false;
    // The material's image behind the UVs; what Image > Display Image turns off.
    bool show_image = true;
    void set_image(const QString& path, QPointF repeat, QPointF offset);
    // The UVs picked here become the scene's UV selection, which the views share, as the reference's do.
    void share_uv_selection();
    // Takes the scene's selection type without setting it back.
    void follow_mode(const QString& m);
    bool shaded = true;
    // View and Image menu switches.
    bool show_distortion = false, show_shell_borders = false, show_texture_borders = true, show_grid = true;
    bool dim_image = false, image_unfiltered = false, image_ratio = false, pixel_snap = false;
    // 0 RGB, 1 RGBA, 2 Alpha.
    int image_channels = 1;
    // Image Range: the part of the image shown, 0 to 1 of its values.
    double image_lo = 0.0, image_hi = 1.0;
    // The image's exposure (stops), gamma and how strongly it shows (Dim's slider).
    double image_exposure = 0.0, image_gamma = 1.0, image_opacity = 1.0;
    QSize image_size() const { return image_.size(); }
    // Frame and zoom history: Undo View Change and Redo View Change.
    void undo_view(bool redo);
signals:
    void picked();
protected:
    void paintEvent(QPaintEvent*) override;
    void mousePressEvent(QMouseEvent*) override;
    void mouseMoveEvent(QMouseEvent*) override;
    void mouseReleaseEvent(QMouseEvent*) override;
    void wheelEvent(QWheelEvent*) override;
    void keyPressEvent(QKeyEvent*) override;
    void keyReleaseEvent(QKeyEvent*) override;
    void mouseDoubleClickEvent(QMouseEvent*) override;
    void contextMenuEvent(QContextMenuEvent*) override;
    void mode_menu(QPoint global);
private:
    QImage image_;
    struct ViewState {
        QPointF center;
        double scale;
    };
    QVector<ViewState> view_back_, view_ahead_;
    qint64 last_view_push_ = 0;
    void remember_view();
    QString image_path_;
    QPointF image_repeat_{1, 1}, image_offset_{0, 0};
    QPointF to_screen(QPointF uv) const;
    QPointF to_uv(QPointF p) const;
    // Selected maps per mesh, from the local UV selection or the scene's components.
    QHash<int, QSet<int>> active_maps() const;
    void pick(QPointF pos, QPointF to, bool rect, Qt::KeyboardModifiers mods);
    void tool_press(QMouseEvent* e);
    void pick_lasso(Qt::KeyboardModifiers mods);
    void pick_path(QPointF pos);
    void drag_to(QPointF pos, bool final);
    int handle_at(QPointF pos) const;

    QVector<UvMesh> meshes_;
    QString scene_mode_ = "object";
    // The scene's selection type it follows: object until the scene goes to components (#99).
    QString mode_ = "object";
    QString tool_ = "move";
    // Local UV selection: mesh index -> maps.
    QHash<QString, QSet<int>> uv_sel_;
    // The last two edges or faces clicked, for Shift+double-click's path: mesh, "edge" or "face", id.
    struct Pick {
        QString mesh, kind;
        int id = -1;
    };
    QList<Pick> pick_hist_;
    // View: uv at the canvas centre and pixels per uv unit.
    QPointF center_{0.5, 0.5};
    double scale_ = 400;
    bool framed_ = false;
    enum class Mode { None, Pan, Zoom, Marquee, Drag, Lasso, Brush, BrushSize, Lattice } drag_ = Mode::None;
    // The UV Lattice Tool: its box, its points (row by row from the bottom), the one dragged.
    QRectF lat_box_;
    QVector<QPointF> lat_pts_;
    int lat_n_ = 3, lat_drag_ = -1;
    void lattice_reset();
    void lattice_apply(bool final);
    // UV brushes (Tools > Brushes): the stroke so far as u v du dv per step, the brush's size in pixels, B held.
    QJsonArray stroke_;
    double brush_px_ = 40;
    bool b_held_ = false;
    QPointF hover_;
    bool is_brush() const { return tool_.startsWith("brush:"); }
    void brush_apply(bool final);
    QPolygonF lasso_;
    // The path tool's last pick: mesh and map.
    QString path_mesh_;
    int path_map_ = -1;
    QPointF press_, last_;
    QPointF pivot_;
    int handle_ = 0;
    bool applied_ = false;
    QJsonArray drag_targets_;
};

class UvEditor : public QWidget {
    Q_OBJECT
public:
    explicit UvEditor(QWidget* parent = nullptr);
    void refresh();
    // The tool, from the texture contexts; every UV Editor follows it.
    static void set_tool_everywhere(const QString& t);
    // What textureWindow's flags ask for.
    void apply_flags(const QJsonObject& props);
private:
    void run(const QString& cmd, const QJsonObject& call = QJsonObject(), bool on_selection = true);
    // The UV Toolkit side panel (uvtoolkit.cpp, #108).
    QWidget* build_toolkit();
    // The menu bar over it all, as the reference's UV Editor has (uvtoolkit.cpp).
    QWidget* build_menu_bar();
    // Snap Together on the two UVs picked: the second's shell onto the first, or the other way.
    void snap_together(bool b_to_a);
    void assign_checker();
    void camera_projection();
    void symmetrize();
    QWidget* toolkit_ = nullptr;
    UvCanvas* canvas_ = nullptr;
    QHash<QString, QAction*> mode_buttons_;
    QHash<QString, QAction*> tool_buttons_;
    void show_tool(const QString& t);
    ::QComboBox* uv_set_ = nullptr;
    QString uv_mesh_;
};

} // namespace az
