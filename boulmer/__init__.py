from .boulmer import _Font
from .boulmer import _Glyph
from .boulmer import _Layer
from .boulmer import _GlyphIterator
from fontTools.pens.pointPen import PointToSegmentPen
from fontTools.pens.boundsPen import BoundsPen, ControlBoundsPen
from typing import NamedTuple
from fontTools.misc.arrayTools import unionRect

class BoundingBox(NamedTuple):
    """Represents a bounding box as a tuple of (xMin, yMin, xMax, yMax)."""

    xMin: float
    yMin: float
    xMax: float
    yMax: float

def unionBounds(bounds1, bounds2):
    if bounds1 is None:
        return bounds2
    if bounds2 is None:
        return bounds1
    return BoundingBox(*unionRect(bounds1, bounds2))

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
    def draw(self, pen):
        pointPen = PointToSegmentPen(pen)
        self.drawPoints(pointPen)

class Glyph(Proxy, Bounded):
    def draw(self, pen):
        pointPen = PointToSegmentPen(pen)
        self.drawPoints(pointPen)

    def __getitem__(self, i):
        return Contour(self._obj[i])

class GlyphIterator(Proxy):
    def __iter__(self):
        return self._obj.__iter__()

    def __next__(self):
        return Glyph(next(self._obj))

class Layer(Proxy):
    def __contains__(self, glyph):
        return glyph in self._obj

    def __getitem__(self, i):
        return Glyph(self._obj[i])

    def __iter__(self):
        return GlyphIterator(self._obj.__iter__())

    @property
    def bounds(self):
        """Returns the (xMin, yMin, xMax, yMax) bounding box of the layer,
        taking the actual contours into account.

        |defcon_compat|
        """
        bounds = None
        for glyph in self:
            bounds = unionBounds(bounds, glyph.getBounds(self))
        return bounds

    @property
    def controlPointBounds(self):
        """Returns the (xMin, yMin, xMax, yMax) bounding box of the layer,
        taking only the control points into account.

        |defcon_compat|
        """
        bounds = None
        for glyph in self:
            bounds = unionBounds(bounds, glyph.getControlBounds(self))
        return bounds

    def test(self):
        return True

    @property
    def test2(self):
        return True

class LayerSet:
    def __init__(self, font):
        self.font = font

    @property
    def defaultLayer(self):
        return Layer(self.font.get_default_layer())

    def __len__(self):
        return self.font.layer_count()

    def __getitem__(self, layer):
        return Layer(self.font.find_layer_by_name(layer))

    def __contains__(self, layer):
        try:
            self.font.find_layer_by_name(layer)
        except KeyError as e:
            return False
        return True

class Features:
    def __init__(self, text, font):
        self._text = text
        self.font = font

    @property
    def text(self):
        return self._text

    @text.setter
    def text(self, value):
        self._text = value
        self.font.set_features(value)

class Font(Proxy):
    def __init__(self, path):
        object.__setattr__(self, "_obj", _Font.load(str(path)))

    def __getitem__(self, item):
        return Glyph(self._obj[item])

    def __contains__(self, glyph):
        return glyph in self._obj

    @classmethod
    def open(cls, filename):
        return Font(filename)

    @property
    def layers(self):
        return LayerSet(self)

    @property
    def features(self):
        return Features(self._features(), self)

    @property
    def glyphOrder(self):
        return self.lib.get("public.glyphOrder", [])

    @property
    def guidelines(self):
        raise NotImplementedError("norad does not support font.guidelines")

    @property
    def images(self):
        raise NotImplementedError("norad does not support font.images")

    def renameLayer(self, old, new, overwrite=False):
        self._renameLayer(old, new, overwrite)

    def renameGlyph(self, old, new, overwrite=False):
        self._renameGlyph(old, new, overwrite)
