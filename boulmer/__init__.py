from .boulmer import _Font
from .boulmer import _Glyph
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
