from .boulmer import _Font
from .boulmer import _Glyph
from .boulmer import _Layer
from fontTools.pens.pointPen import PointToSegmentPen
from fontTools.pens.boundsPen import BoundsPen, ControlBoundsPen
from typing import NamedTuple


class BoundingBox(NamedTuple):
    """Represents a bounding box as a tuple of (xMin, yMin, xMax, yMax)."""

    xMin: float
    yMin: float
    xMax: float
    yMax: float


class Bounded:
    def getBounds(self, layer=None):
        pen = BoundsPen(layer)
        pen.skipMissingComponents = False
        self.draw(pen)
        return None if pen.bounds is None else BoundingBox(*pen.bounds)

    def getControlBounds(self, layer=None):
        pen = ControlBoundsPen(layer)
        # raise 'KeyError' when a referenced component is missing from glyph set
        pen.skipMissingComponents = False
        self.draw(pen)
        return None if pen.bounds is None else BoundingBox(*pen.bounds)

    @property
    def bounds(self):
        return self.getBounds()

    @property
    def controlPointBounds(self):
        return self.getControlBounds()


class Proxy(object):
    __slots__ = ["_obj", "__weakref__"]
    def __init__(self, obj):
        object.__setattr__(self, "_obj", obj)

    def __getattr__(self, item):
        real = object.__getattribute__(self, "_obj")
        if hasattr(real, item):
            return getattr(real, item)
        raise AttributeError(item)

    def __len__(self):
        return len(self._obj)

class Contour(Proxy, Bounded):
        pass

class Glyph(Proxy, Bounded):
    def draw(self, pen):
        pointPen = PointToSegmentPen(pen)
        self.drawPoints(pointPen)

    def __getitem__(self, i):
        return Contour(self._obj[i])


class LayerSet:
    def __init__(self, font):
        self.font = font

    @property
    def defaultLayer(self):
        return self.font.get_default_layer()

    def __len__(self):
        return self.font.layer_count()

    def __getitem__(self, layer):
        return self.font.find_layer_by_name(layer)

    def __contains__(self, layer):
        try:
            self.font.find_layer_by_name(layer)
        except KeyError as e:
            return False
        return True

class Font(Proxy):
    def __init__(self, path):
        object.__setattr__(self, "_obj", _Font.load(str(path)))

    def __getitem__(self, item):
        return Glyph(self._obj[item])

    @property
    def layers(self):
        return LayerSet(self)
