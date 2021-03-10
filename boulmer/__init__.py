from .boulmer import _Font
from .boulmer import _Glyph
from .boulmer import _Layer
from fontTools.pens.pointPen import PointToSegmentPen


class Glyph:
    def __init__(self, glyph):
        self.glyph = glyph

    def draw(self, pen):
        pointPen = PointToSegmentPen(pen)
        self.drawPoints(pointPen)

    def __len__(self):
        return len(self.glyph)

    def __getattr__(self, item):
        if hasattr(self.glyph, item):
            return getattr(self.glyph, item)
        raise AttributeError(item)


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

class Font:
    def __init__(self, path):
        self.font = _Font.load(str(path))

    def __len__(self):
        return len(self.font)

    def __getitem__(self, item):
        return Glyph(self.font[item])

    def __getattr__(self, item):
        if hasattr(self.font, item):
            return getattr(self.font, item)
        raise AttributeError(item)

    @property
    def layers(self):
        return LayerSet(self)
