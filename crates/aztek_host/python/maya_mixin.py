# maya.app.general.mayaMixin: the classes tools mix into their Qt widgets so the windows belong to the application.
# It needs a Qt binding built against the Qt Aztek runs on (PySide6, or PySide2).
import uuid

try:
    from PySide6 import QtCore, QtWidgets
except ImportError:
    from PySide2 import QtCore, QtWidgets

import aztek.OpenAztekUI as _omui


def _shiboken():
    """shiboken for the binding in use, or None: only wanted when the main window can't be found through Qt itself."""
    for name in ("shiboken6", "shiboken2"):
        try:
            return __import__(name)
        except ImportError:
            continue
    return None


def _main_window():
    """The application's main window: found among the top-level widgets by the name $gMainWindow holds, which needs
    nothing but Qt, then through MQtUtil.mainWindow() and shiboken (#142)."""
    app = QtWidgets.QApplication.instance()
    if app is not None:
        for w in app.topLevelWidgets():
            if isinstance(w, QtWidgets.QMainWindow) and w.objectName() == "MayaWindow":
                return w
    p = _omui.MQtUtil.mainWindow()
    shiboken = _shiboken() if p else None
    if shiboken is None:
        return None
    try:
        return shiboken.wrapInstance(int(p), QtWidgets.QMainWindow)
    except TypeError:
        return None


class MayaQWidgetBaseMixin(object):
    """A widget with no parent given goes under the main window, as a window of its own, with a name of its own."""

    def __init__(self, parent=None, *args, **kwargs):
        super(MayaQWidgetBaseMixin, self).__init__(parent=parent, *args, **kwargs)
        self._initForMaya(parent=parent)

    def _initForMaya(self, parent=None, *args, **kwargs):
        if parent is None:
            main = _main_window()
            if main is not None:
                flags = self.windowFlags()
                self.setParent(main)
                self.setWindowFlags(flags | QtCore.Qt.Window)
        if not self.objectName():
            self.setObjectName(self.__class__.__name__ + "_" + str(uuid.uuid4()))

    def setVisible(self, visible):
        super(MayaQWidgetBaseMixin, self).setVisible(visible)


class MayaQDockWidget(MayaQWidgetBaseMixin, QtWidgets.QDockWidget):
    """A dock widget under the main window."""

    windowStateChanged = QtCore.Signal()

    def __init__(self, parent=None, *args, **kwargs):
        super(MayaQDockWidget, self).__init__(parent=parent, *args, **kwargs)

    def setArea(self, area):
        main = _main_window()
        if main is not None and area not in (None, "none"):
            areas = {"left": QtCore.Qt.LeftDockWidgetArea, "right": QtCore.Qt.RightDockWidgetArea, "top": QtCore.Qt.TopDockWidgetArea, "bottom": QtCore.Qt.BottomDockWidgetArea}
            main.addDockWidget(areas.get(area, area), self)

    def resizeEvent(self, event):
        super(MayaQDockWidget, self).resizeEvent(event)
        self.windowStateChanged.emit()


class MayaQWidgetDockableMixin(MayaQWidgetBaseMixin):
    """show(dockable=True) puts the widget in a dock of the main window; without it the widget is a window."""

    windowStateChanged = QtCore.Signal()

    def __init__(self, parent=None, *args, **kwargs):
        super(MayaQWidgetDockableMixin, self).__init__(parent=parent, *args, **kwargs)
        self._dock = None
        self._dockable = False

    def setDockableParameters(self, dockable=None, floating=None, area=None, allowedArea=None, width=None, height=None, x=None, y=None, retain=True, plugins=None, controls=None, uiScript=None, closeCallback=None, *args, **kwargs):
        if dockable is not None and dockable != self._dockable:
            self._dockable = bool(dockable)
            if dockable:
                dock = MayaQDockWidget()
                dock.setObjectName(self.objectName() + "WorkspaceControl")
                dock.setWindowTitle(self.windowTitle())
                dock.setWidget(self)
                self._dock = dock
            elif self._dock is not None:
                self._dock.setWidget(None)
                self.setParent(_main_window())
                self.setWindowFlags(self.windowFlags() | QtCore.Qt.Window)
                self._dock.deleteLater()
                self._dock = None
        target = self._dock if self._dock is not None else self
        if self._dock is not None:
            if area not in (None, "none") and not floating:
                self._dock.setArea(area)
            if floating is not None:
                self._dock.setFloating(bool(floating))
        if width is not None or height is not None:
            target.resize(width if width is not None else target.width(), height if height is not None else target.height())
        if x is not None or y is not None:
            target.move(x if x is not None else target.x(), y if y is not None else target.y())

    def show(self, *args, **kwargs):
        if args or kwargs:
            self.setDockableParameters(*args, **kwargs)
        if self._dock is not None:
            self._dock.show()
        super(MayaQWidgetDockableMixin, self).show()

    def hide(self, *args, **kwargs):
        if self._dock is not None:
            self._dock.hide()
        super(MayaQWidgetDockableMixin, self).hide()

    def close(self, *args, **kwargs):
        if self._dock is not None:
            self._dock.close()
        return super(MayaQWidgetDockableMixin, self).close()

    def isVisible(self):
        return self._dock.isVisible() if self._dock is not None else super(MayaQWidgetDockableMixin, self).isVisible()

    def raise_(self):
        (self._dock if self._dock is not None else super(MayaQWidgetDockableMixin, self)).raise_()

    def setWindowTitle(self, title):
        super(MayaQWidgetDockableMixin, self).setWindowTitle(title)
        if getattr(self, "_dock", None) is not None:
            self._dock.setWindowTitle(title)

    def isDockable(self):
        return self._dock is not None

    def isFloating(self):
        return self._dock.isFloating() if self._dock is not None else True

    def floatingChanged(self, isFloating):
        pass

    def dockCloseEventTriggered(self):
        pass

    def dockArea(self):
        main = _main_window()
        return main.dockWidgetArea(self._dock) if main is not None and self._dock is not None else None

    def parentContainer(self):
        return self._dock

    def showRepr(self):
        return "show(dockable=%s)" % (self._dock is not None)
