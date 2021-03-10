from boulmer import Font
import boulmer
from fontTools.pens.recordingPen import RecordingPen


def test_constructor_from_path(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    assert len(font) == 2
    assert len(font.layers) == 2
    assert "public.background" in font.layers
    assert "public.backgroundx" not in font.layers
    assert isinstance(font.layers.defaultLayer,boulmer._Layer)
    assert len(font["a"]) == 2
    assert font["a"].bounds == (43,-11,448,533)

def test_draw(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    rpen = RecordingPen()
    font["a"].draw(rpen)

    assert rpen.value == [
      ('moveTo', ((256.0, 67.0),)),
      ('qCurveTo', ((289.0, 67.0), (340.0, 70.0), (357.0, 74.0))),
      ('lineTo', ((357.0, 229.0),)),
      ('qCurveTo', ((347.0, 234.0), (302.0, 241.0), (270.0, 241.0))),
      ('qCurveTo', ((249.0, 241.0), (202.0, 235.0), (163.0, 216.0), (137.0, 183.0), (137.0, 156.0))), ('qCurveTo', ((137.0, 106.0), (201.0, 67.0), (256.0, 67.0))), ('closePath', ()),
      ('moveTo', ((248.0, 533.0),)),
      ('qCurveTo', ((304.0, 533.0), (381.0, 504.0), (428.0, 451.0), (448.0, 378.0), (448.0, 334.0))),
      ('lineTo', ((448.0, 9.0),)),
      ('qCurveTo', ((436.0, 7.0), (393.0, 0.0), (339.0, -6.0), (276.0, -11.0), (245.0, -11.0))),
      ('qCurveTo', ((201.0, -11.0), (127.0, 7.0), (73.0, 46.0), (43.0, 110.0), (43.0, 155.0))),
      ('qCurveTo', ((43.0, 198.0), (78.0, 260.0), (138.0, 298.0), (218.0, 316.0), (262.0, 316.0))),
      ('qCurveTo', ((276.0, 316.0), (306.0, 313.0), (333.0, 308.0), (353.0, 304.0), (357.0, 303.0))),
      ('lineTo', ((357.0, 329.0),)),
      ('qCurveTo', ((357.0, 352.0), (347.0, 397.0), (321.0, 432.0), (276.0, 453.0), (240.0, 453.0))),
      ('qCurveTo', ((194.0, 453.0), (125.0, 440.0), (108.0, 433.0))),
      ('lineTo', ((97.0, 510.0),)),
      ('qCurveTo', ((115.0, 518.0), (199.0, 533.0), (248.0, 533.0))),
      ('closePath', ())
    ]

def test_lib(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    assert font.lib == {'public.glyphOrder': ['A', 'a']}
