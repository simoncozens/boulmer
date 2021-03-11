from boulmer import Font
import boulmer
from fontTools.pens.recordingPen import RecordingPen
import pytest


def test_constructor_from_path(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    assert len(font) == 2
    assert len(font.layers) == 2
    assert "public.background" in font.layers
    assert "public.backgroundx" not in font.layers
    assert isinstance(font.layers.defaultLayer, boulmer._Layer)
    assert len(font["a"]) == 2
    assert font["a"].bounds == (43, -11, 448, 533)
    assert font.get("Nonesuch", -123) == -123


def test_draw(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    rpen = RecordingPen()
    font["a"].draw(rpen)

    assert rpen.value == [
        ("moveTo", ((256.0, 67.0),)),
        ("qCurveTo", ((289.0, 67.0), (340.0, 70.0), (357.0, 74.0))),
        ("lineTo", ((357.0, 229.0),)),
        ("qCurveTo", ((347.0, 234.0), (302.0, 241.0), (270.0, 241.0))),
        ("qCurveTo", ((249.0, 241.0), (202.0, 235.0), (163.0, 216.0), (137.0, 183.0), (137.0, 156.0), ), ),
        ("qCurveTo", ((137.0, 106.0), (201.0, 67.0), (256.0, 67.0))),
        ("closePath", ()),
        ("moveTo", ((248.0, 533.0),)),
        ("qCurveTo", ((304.0, 533.0), (381.0, 504.0), (428.0, 451.0), (448.0, 378.0), (448.0, 334.0), ), ),
        ("lineTo", ((448.0, 9.0),)),
        ("qCurveTo", ((436.0, 7.0), (393.0, 0.0), (339.0, -6.0), (276.0, -11.0), (245.0, -11.0)), ),
        ("qCurveTo", ((201.0, -11.0), (127.0, 7.0), (73.0, 46.0), (43.0, 110.0), (43.0, 155.0)), ),
        ("qCurveTo", ((43.0, 198.0), (78.0, 260.0), (138.0, 298.0), (218.0, 316.0), (262.0, 316.0), ), ),
        ("qCurveTo", ((276.0, 316.0), (306.0, 313.0), (333.0, 308.0), (353.0, 304.0), (357.0, 303.0), ), ),
        ("lineTo", ((357.0, 329.0),)),
        ("qCurveTo", ((357.0, 352.0), (347.0, 397.0), (321.0, 432.0), (276.0, 453.0), (240.0, 453.0), ), ),
        ("qCurveTo", ((194.0, 453.0), (125.0, 440.0), (108.0, 433.0))),
        ("lineTo", ((97.0, 510.0),)),
        ("qCurveTo", ((115.0, 518.0), (199.0, 533.0), (248.0, 533.0))),
        ("closePath", ()),
    ]


def test_lib(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    assert font.lib == {"public.glyphOrder": ["A", "a"]}


def test_features(datadir):
    path = datadir / "MutatorSansBoldCondensed.ufo"
    font = Font(path)
    feat = font.features
    assert isinstance(feat, boulmer.Features)
    assert feat.text == "# this is the feature from boldcondensed.\n"


def test_glyphOrder(datadir):
    path = datadir / "MutatorSansBoldCondensed.ufo"
    font = Font(path)
    order = font.glyphOrder
    assert order[0:4] == ["A", "Aacute", "Adieresis", "B"]


def test_groups(datadir):
    path = datadir / "MutatorSansBoldCondensed.ufo"
    font = Font(path)
    assert font.groups["testGroup"] == ["E", "F", "H"]

def test_info(datadir):
    path = datadir / "MutatorSansBoldCondensed.ufo"
    font = Font(path)
    assert font.info.ascender == 800
    assert font.info.capHeight == 800
    assert font.info.copyright == "License same as MutatorMath. BSD 3-clause. [test-token: A]"
    assert font.info.descender == -200
    assert font.info.familyName == "MutatorMathTest"
    assert font.info.italicAngle == 0
    assert font.info.openTypeNameLicense == "License same as MutatorMath. BSD 3-clause. [test-token: A]"
    assert font.info.openTypeOS2VendorID == "LTTR"
    assert font.info.postscriptBlueValues == [-10, 0, 800, 810, ]
    assert font.info.postscriptDefaultWidthX == 500
    assert font.info.postscriptFontName == "MutatorMathTest-BoldCondensed"
    assert font.info.postscriptFullName == "MutatorMathTest BoldCondensed"
    assert font.info.postscriptOtherBlues == [ 500, 520 ]
    assert font.info.postscriptSlantAngle == 0
    assert font.info.postscriptStemSnapH == []
    assert font.info.postscriptStemSnapV == []
    assert font.info.postscriptWindowsCharacterSet == 1
    assert font.info.styleMapFamilyName == ""
    assert font.info.styleMapStyleName == "regular"
    assert font.info.styleName == "BoldCondensed"
    assert font.info.unitsPerEm == 1000
    assert font.info.versionMajor == 1
    assert font.info.versionMinor == 2
    assert font.info.xHeight == 500
    assert font.info.year == 2004

def test_kerning(datadir):
    path = datadir / "MutatorSansBoldCondensed.ufo"
    font = Font(path)
    assert font.kerning["A"] == {'J': -20.0, 'O': -30.0, 'T': -70.0, 'U': -30.0, 'V': -50.0}

@pytest.mark.skip("Not currently working")
def test_rename(datadir):
    path = datadir / "UbuTestData.ufo"
    font = Font(path)
    assert "badger" not in font
    assert "A" in font
    assert font.keys() == ['A', 'a']
    with pytest.raises(ValueError):
        font.renameGlyph("A", "a")
    font.renameGlyph("A", "badger")
    assert font.keys() == ['badger', 'a']
    assert "badger" in font
    assert "A" not in font
