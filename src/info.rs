use norad::fontinfo::StyleMapStyle;
use norad::IntegerOrFloat;
use norad::NonNegativeIntegerOrFloat;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct _Info {
    pub fontinfo: norad::FontInfo,
}

impl From<norad::FontInfo> for _Info {
    fn from(fontinfo: norad::FontInfo) -> Self {
        Self { fontinfo }
    }
}

trait MyToString {
    fn to_string(&self) -> String;
}

impl MyToString for StyleMapStyle {
    fn to_string(&self) -> String {
        match self {
            StyleMapStyle::Regular => "regular".to_string(),
            StyleMapStyle::Italic => "italic".to_string(),
            StyleMapStyle::Bold => "bold".to_string(),
            StyleMapStyle::BoldItalic => "bold italic".to_string(),
        }
    }
}
trait MyToPyObject {
    fn to_object(&self, py: Python) -> PyObject;
}

impl MyToPyObject for Option<IntegerOrFloat> {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Some(s) => {
                if s.is_integer() {
                    (s.get() as i64).to_object(py)
                } else {
                    s.get().to_object(py)
                }
            }
            None => py.None(),
        }
    }
}

impl MyToPyObject for Option<NonNegativeIntegerOrFloat> {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Some(s) => {
                if s.is_integer() {
                    (s.get() as i64).to_object(py)
                } else {
                    s.get().to_object(py)
                }
            }
            None => py.None(),
        }
    }
}

impl MyToPyObject for Option<Vec<IntegerOrFloat>> {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Some(s) => PyList::new(
                py,
                s.iter()
                    .map(|val| val.to_object(py))
                    .collect::<Vec<PyObject>>(),
            )
            .into(),
            None => py.None(),
        }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl _Info {
    #[getter]
    fn ascender(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.ascender.to_object(py)
    }
    #[getter]
    fn capHeight(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.cap_height.to_object(py)
    }
    #[getter]
    fn copyright(&self) -> Option<&String> {
        self.fontinfo.copyright.as_ref()
    }
    #[getter]
    fn descender(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.descender.to_object(py)
    }
    #[getter]
    fn familyName(&self) -> Option<&String> {
        self.fontinfo.family_name.as_ref()
    }
    // #[getter]
    // fn guidelines(&self) -> Option<Vec<Guideline>> {
    //
    // }
    #[getter]
    fn italicAngle(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.italic_angle.to_object(py)
    }

    #[getter]
    fn macintoshFondFamilyID(&self) -> Option<i32> {
        self.fontinfo.macintosh_fond_family_id
    }
    #[getter]
    fn macintoshFondName(&self) -> Option<&String> {
        self.fontinfo.macintosh_fond_name.as_ref()
    }
    #[getter]
    fn note(&self) -> Option<&String> {
        self.fontinfo.note.as_ref()
    }
    // #[getter]
    // fn openTypeGaspRangeRecords(&self) -> Option<Vec<GaspRangeRecord>> {
    //     self.fontinfo.open_type_gasp_range_records
    // }
    #[getter]
    fn openTypeHeadCreated(&self) -> Option<&String> {
        self.fontinfo.open_type_head_created.as_ref()
    }
    #[getter]
    fn openTypeHeadFlags(&self) -> Option<Vec<u8>> {
        self.fontinfo.open_type_head_flags.clone()
    }
    #[getter]
    fn openTypeHeadLowestRecPpem(&self) -> Option<u32> {
        self.fontinfo.open_type_head_lowest_rec_ppem
    }
    #[getter]
    fn openTypeHheaAscender(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_ascender
    }
    #[getter]
    fn openTypeHheaCaretOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_caret_offset
    }
    #[getter]
    fn openTypeHheaCaretSlopeRise(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_caret_slope_rise
    }
    #[getter]
    fn openTypeHheaCaretSlopeRun(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_caret_slope_run
    }
    #[getter]
    fn openTypeHheaDescender(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_descender
    }
    #[getter]
    fn openTypeHheaLineGap(&self) -> Option<i32> {
        self.fontinfo.open_type_hhea_line_gap
    }
    #[getter]
    fn openTypeNameCompatibleFullName(&self) -> Option<&String> {
        self.fontinfo.open_type_name_compatible_full_name.as_ref()
    }
    #[getter]
    fn openTypeNameDescription(&self) -> Option<&String> {
        self.fontinfo.open_type_name_description.as_ref()
    }
    #[getter]
    fn openTypeNameDesignerURL(&self) -> Option<&String> {
        self.fontinfo.open_type_name_designer_url.as_ref()
    }
    #[getter]
    fn openTypeNameDesigner(&self) -> Option<&String> {
        self.fontinfo.open_type_name_designer.as_ref()
    }
    #[getter]
    fn openTypeNameLicense(&self) -> Option<&String> {
        self.fontinfo.open_type_name_license.as_ref()
    }
    #[getter]
    fn openTypeNameLicenseURL(&self) -> Option<&String> {
        self.fontinfo.open_type_name_license_url.as_ref()
    }
    #[getter]
    fn openTypeNameManufacturer(&self) -> Option<&String> {
        self.fontinfo.open_type_name_manufacturer.as_ref()
    }
    #[getter]
    fn openTypeNameManufacturerURL(&self) -> Option<&String> {
        self.fontinfo.open_type_name_manufacturer_url.as_ref()
    }
    #[getter]
    fn openTypeNamePreferredFamilyName(&self) -> Option<&String> {
        self.fontinfo.open_type_name_preferred_family_name.as_ref()
    }
    #[getter]
    fn openTypeNamePreferredSubfamilyName(&self) -> Option<&String> {
        self.fontinfo
            .open_type_name_preferred_subfamily_name
            .as_ref()
    }
    // #[getter]
    // fn openTypeNameRecords(&self) -> Option<Vec<NameRecord>> {
    //     self.fontinfo.open_type_name_records
    // }
    #[getter]
    fn openTypeNameSampleText(&self) -> Option<&String> {
        self.fontinfo.open_type_name_sample_text.as_ref()
    }
    #[getter]
    fn openTypeNameUniqueID(&self) -> Option<&String> {
        self.fontinfo.open_type_name_unique_id.as_ref()
    }
    #[getter]
    fn openTypeNameVersion(&self) -> Option<&String> {
        self.fontinfo.open_type_name_version.as_ref()
    }
    #[getter]
    fn openTypeNameWwsFamilyName(&self) -> Option<&String> {
        self.fontinfo.open_type_name_wws_family_name.as_ref()
    }
    #[getter]
    fn openTypeNameWwsSubfamilyName(&self) -> Option<&String> {
        self.fontinfo.open_type_name_wws_subfamily_name.as_ref()
    }
    #[getter]
    fn openTypeOS2CodePageRanges(&self) -> Option<Vec<u8>> {
        self.fontinfo.open_type_os2_code_page_ranges.clone()
    }
    // #[getter]
    // fn openTypeOS2FamilyClass(&self) -> Option<OS2FamilyClass> {
    //     self.fontinfo.open_type_os2_family_class
    // }
    // #[getter]
    // fn openTypeOS2Panose(&self) -> Option<OS2Panose> {
    //     self.fontinfo.open_type_os2_panose
    // }
    #[getter]
    fn openTypeOS2Selection(&self) -> Option<Vec<u8>> {
        self.fontinfo.open_type_os2_selection.clone()
    }
    #[getter]
    fn openTypeOS2StrikeoutPosition(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_strikeout_position
    }
    #[getter]
    fn openTypeOS2StrikeoutSize(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_strikeout_size
    }
    #[getter]
    fn openTypeOS2SubscriptXOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_subscript_x_offset
    }
    #[getter]
    fn openTypeOS2SubscriptXSize(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_subscript_x_size
    }
    #[getter]
    fn openTypeOS2SubscriptYOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_subscript_y_offset
    }
    #[getter]
    fn openTypeOS2SubscriptYSize(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_subscript_y_size
    }
    #[getter]
    fn openTypeOS2SuperscriptXOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_superscript_x_offset
    }
    #[getter]
    fn openTypeOS2SuperscriptXSize(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_superscript_x_size
    }
    #[getter]
    fn openTypeOS2SuperscriptYOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_superscript_y_offset
    }
    #[getter]
    fn openTypeOS2SuperscriptYSize(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_superscript_y_size
    }
    #[getter]
    fn openTypeOS2Type(&self) -> Option<Vec<u8>> {
        self.fontinfo.open_type_os2_type.clone()
    }
    #[getter]
    fn openTypeOS2TypoAscender(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_typo_ascender
    }
    #[getter]
    fn openTypeOS2TypoDescender(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_typo_descender
    }
    #[getter]
    fn openTypeOS2TypoLineGap(&self) -> Option<i32> {
        self.fontinfo.open_type_os2_typo_line_gap
    }
    #[getter]
    fn openTypeOS2UnicodeRanges(&self) -> Option<Vec<u8>> {
        self.fontinfo.open_type_os2_unicode_ranges.clone()
    }
    #[getter]
    fn openTypeOS2VendorID(&self) -> Option<&String> {
        self.fontinfo.open_type_os2_vendor_id.as_ref()
    }
    #[getter]
    fn openTypeOS2WeightClass(&self) -> Option<u32> {
        self.fontinfo.open_type_os2_weight_class
    }
    // #[getter]
    // fn openTypeOS2WidthClass(&self) -> Option<OS2WidthClass> {
    //     self.fontinfo.open_type_os2_width_class
    // }
    #[getter]
    fn openTypeOS2WinAscent(&self) -> Option<u32> {
        self.fontinfo.open_type_os2_win_ascent
    }
    #[getter]
    fn openTypeOS2WinDescent(&self) -> Option<u32> {
        self.fontinfo.open_type_os2_win_descent
    }
    #[getter]
    fn openTypeVheaCaretOffset(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_caret_offset
    }
    #[getter]
    fn openTypeVheaCaretSlopeRise(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_caret_slope_rise
    }
    #[getter]
    fn openTypeVheaCaretSlopeRun(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_caret_slope_run
    }
    #[getter]
    fn openTypeVheaVertTypoAscender(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_vert_typo_ascender
    }
    #[getter]
    fn openTypeVheaVertTypoDescender(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_vert_typo_descender
    }
    #[getter]
    fn openTypeVheaVertTypoLineGap(&self) -> Option<i32> {
        self.fontinfo.open_type_vhea_vert_typo_line_gap
    }
    #[getter]
    fn postscriptBlueFuzz(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_blue_fuzz.to_object(py)
    }
    #[getter]
    fn postscriptBlueScale(&self) -> Option<f64> {
        self.fontinfo.postscript_blue_scale
    }
    #[getter]
    fn postscriptBlueShift(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_blue_shift.to_object(py)
    }
    #[getter]
    fn postscriptBlueValues(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_blue_values.to_object(py)
    }
    #[getter]
    fn postscriptDefaultCharacter(&self) -> Option<&String> {
        self.fontinfo.postscript_default_character.as_ref()
    }
    #[getter]
    fn postscriptDefaultWidthX(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_default_width_x.to_object(py)
    }
    #[getter]
    fn postscriptFamilyBlues(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_family_blues.to_object(py)
    }
    #[getter]
    fn postscriptFamilyOtherBlues(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_family_other_blues.to_object(py)
    }
    #[getter]
    fn postscriptFontName(&self) -> Option<&String> {
        self.fontinfo.postscript_font_name.as_ref()
    }
    #[getter]
    fn postscriptForceBold(&self) -> Option<bool> {
        self.fontinfo.postscript_force_bold
    }
    #[getter]
    fn postscriptFullName(&self) -> Option<&String> {
        self.fontinfo.postscript_full_name.as_ref()
    }
    #[getter]
    fn postscriptIsFixedPitch(&self) -> Option<bool> {
        self.fontinfo.postscript_is_fixed_pitch
    }
    #[getter]
    fn postscriptNominalWidthX(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_nominal_width_x.to_object(py)
    }
    #[getter]
    fn postscriptOtherBlues(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_other_blues.to_object(py)
    }
    #[getter]
    fn postscriptSlantAngle(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_slant_angle.to_object(py)
    }
    #[getter]
    fn postscriptStemSnapH(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_stem_snap_h.to_object(py)
    }
    #[getter]
    fn postscriptStemSnapV(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_stem_snap_v.to_object(py)
    }
    #[getter]
    fn postscriptUnderlinePosition(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_underline_position.to_object(py)
    }
    #[getter]
    fn postscriptUnderlineThickness(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.postscript_underline_thickness.to_object(py)
    }
    #[getter]
    fn postscriptUniqueID(&self) -> Option<i32> {
        self.fontinfo.postscript_unique_id
    }
    #[getter]
    fn postscriptWeightName(&self) -> Option<&String> {
        self.fontinfo.postscript_weight_name.as_ref()
    }
    #[getter]
    fn postscriptWindowsCharacterSet(&self) -> Option<u8> {
        match self.fontinfo.postscript_windows_character_set {
            Some(v) => Some(v as u8),
            None => None,
        }
    }
    #[getter]
    fn styleMapFamilyName(&self) -> Option<&String> {
        self.fontinfo.style_map_family_name.as_ref()
    }
    #[getter]
    fn styleMapStyleName(&self) -> Option<String> {
        match &self.fontinfo.style_map_style_name {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
    #[getter]
    fn styleName(&self) -> Option<&String> {
        self.fontinfo.style_name.as_ref()
    }
    #[getter]
    fn trademark(&self) -> Option<&String> {
        self.fontinfo.trademark.as_ref()
    }
    #[getter]
    fn unitsPerEm(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.units_per_em.to_object(py)
    }
    #[getter]
    fn versionMajor(&self) -> Option<i32> {
        self.fontinfo.version_major
    }
    #[getter]
    fn versionMinor(&self) -> Option<u32> {
        self.fontinfo.version_minor
    }
    #[getter]
    fn woffMajorVersion(&self) -> Option<u32> {
        self.fontinfo.woff_major_version
    }
    // #[getter]
    // fn woffMetadataCopyright(&self) -> Option<WoffMetadataCopyright> {
    //     self.fontinfo.woff_metadata_copyright
    // }
    // #[getter]
    // fn woffMetadataCredits(&self) -> Option<WoffMetadataCredits> {
    //     self.fontinfo.woff_metadata_credits
    // }
    // #[getter]
    // fn woffMetadataDescription(&self) -> Option<WoffMetadataDescription> {
    //     self.fontinfo.woff_metadata_description
    // }
    // #[getter]
    // fn woffMetadataExtensions(&self) -> Option<Vec<WoffMetadataExtensionRecord>> {
    //     self.fontinfo.woff_metadata_extensions
    // }
    // #[getter]
    // fn woffMetadataLicense(&self) -> Option<WoffMetadataLicense> {
    //     self.fontinfo.woff_metadata_license
    // }
    // #[getter]
    // fn woffMetadataLicensee(&self) -> Option<WoffMetadataLicensee> {
    //     self.fontinfo.woff_metadata_licensee
    // }
    // #[getter]
    // fn woffMetadataTrademark(&self) -> Option<WoffMetadataTrademark> {
    //     self.fontinfo.woff_metadata_trademark
    // }
    // #[getter]
    // fn woffMetadataUniqueID(&self) -> Option<WoffMetadataUniqueID> {
    //     self.fontinfo.woff_metadata_unique_id
    // }
    // #[getter]
    // fn woffMetadataVendor(&self) -> Option<WoffMetadataVendor> {
    //     self.fontinfo.woff_metadata_vendor
    // }
    #[getter]
    fn woffMinorVersion(&self) -> Option<u32> {
        self.fontinfo.woff_minor_version
    }
    #[getter]
    fn xHeight(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.fontinfo.x_height.to_object(py)
    }
    #[getter]
    fn year(&self) -> Option<i32> {
        self.fontinfo.year
    }
}
