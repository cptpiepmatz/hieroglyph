// @generated

use std::sync::LazyLock;

use ab_glyph::FontRef;

macro_rules! lazy_font {
    ($path:literal) => {
        LazyLock::new(|| {
            // this is an internal macro and will only use predefined fonts
            FontRef::try_from_slice(include_bytes!($path)).unwrap()
        })
    };
}

static NOTO_SANS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSans/unhinted/otf/NotoSans-Regular.otf");
static NOTO_SANS_YI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansYi/unhinted/otf/NotoSansYi-Regular.otf");
static NOTO_SANS_LAO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLao/unhinted/otf/NotoSansLao-Regular.otf");
static NOTO_SANS_MRO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMro/unhinted/otf/NotoSansMro-Regular.otf");
static NOTO_SANS_N_KO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNKo/unhinted/otf/NotoSansNKo-Regular.otf");
static NOTO_SANS_VAI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansVai/unhinted/otf/NotoSansVai-Regular.otf");
static NOTO_SANS_CHAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCham/unhinted/otf/NotoSansCham-Regular.otf");
static NOTO_SANS_KAWI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKawi/unhinted/otf/NotoSansKawi-Regular.otf");
static NOTO_SANS_LISU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLisu/unhinted/otf/NotoSansLisu-Regular.otf");
static NOTO_SANS_MATH_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMath/unhinted/otf/NotoSansMath-Regular.otf");
static NOTO_SANS_MIAO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMiao/unhinted/otf/NotoSansMiao-Regular.otf");
static NOTO_SANS_MODI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansModi/unhinted/otf/NotoSansModi-Regular.otf");
static NOTO_SANS_MONO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMono/unhinted/otf/NotoSansMono-Regular.otf");
static NOTO_SANS_NEWA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNewa/unhinted/otf/NotoSansNewa-Regular.otf");
static NOTO_SANS_THAI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansThai/unhinted/otf/NotoSansThai-Regular.otf");
static NOTO_SANS_ADLAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansAdlam/unhinted/otf/NotoSansAdlam-Regular.otf");
static NOTO_SANS_BAMUM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBamum/unhinted/otf/NotoSansBamum-Regular.otf");
static NOTO_SANS_BATAK_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBatak/unhinted/otf/NotoSansBatak-Regular.otf");
static NOTO_SANS_BUHID_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBuhid/unhinted/otf/NotoSansBuhid-Regular.otf");
static NOTO_SANS_KHMER_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKhmer/unhinted/otf/NotoSansKhmer-Regular.otf");
static NOTO_SANS_LIMBU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLimbu/unhinted/otf/NotoSansLimbu-Regular.otf");
static NOTO_SANS_NUSHU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNushu/unhinted/otf/NotoSansNushu-Regular.otf");
static NOTO_SANS_OGHAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOgham/unhinted/otf/NotoSansOgham-Regular.otf");
static NOTO_SANS_ORIYA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOriya/unhinted/otf/NotoSansOriya-Regular.otf");
static NOTO_SANS_OSAGE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOsage/unhinted/otf/NotoSansOsage-Regular.otf");
static NOTO_SANS_RUNIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansRunic/unhinted/otf/NotoSansRunic-Regular.otf");
static NOTO_SANS_TAI_LE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTaiLe/unhinted/otf/NotoSansTaiLe-Regular.otf");
static NOTO_SANS_TAKRI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTakri/unhinted/otf/NotoSansTakri-Regular.otf");
static NOTO_SANS_TAMIL_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTamil/unhinted/otf/NotoSansTamil-Regular.otf");
static NOTO_SANS_ARABIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansArabic/unhinted/otf/NotoSansArabic-Regular.otf");
static NOTO_SANS_BRAHMI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBrahmi/unhinted/otf/NotoSansBrahmi-Regular.otf");
static NOTO_SANS_CARIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCarian/unhinted/otf/NotoSansCarian-Regular.otf");
static NOTO_SANS_CHAKMA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansChakma/unhinted/otf/NotoSansChakma-Regular.otf");
static NOTO_SANS_COPTIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCoptic/unhinted/otf/NotoSansCoptic-Regular.otf");
static NOTO_SANS_GOTHIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGothic/unhinted/otf/NotoSansGothic-Regular.otf");
static NOTO_SANS_HATRAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansHatran/unhinted/otf/NotoSansHatran-Regular.otf");
static NOTO_SANS_HEBREW_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansHebrew/unhinted/otf/NotoSansHebrew-Regular.otf");
static NOTO_SANS_KAITHI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKaithi/unhinted/otf/NotoSansKaithi-Regular.otf");
static NOTO_SANS_KHOJKI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKhojki/unhinted/otf/NotoSansKhojki-Regular.otf");
static NOTO_SANS_LEPCHA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLepcha/unhinted/otf/NotoSansLepcha-Regular.otf");
static NOTO_SANS_LYCIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLycian/unhinted/otf/NotoSansLycian-Regular.otf");
static NOTO_SANS_LYDIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLydian/unhinted/otf/NotoSansLydian-Regular.otf");
static NOTO_SANS_REJANG_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansRejang/unhinted/otf/NotoSansRejang-Regular.otf");
static NOTO_SANS_SYRIAC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSyriac/unhinted/otf/NotoSansSyriac-Regular.otf");
static NOTO_SANS_TANGSA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTangsa/unhinted/otf/NotoSansTangsa-Regular.otf");
static NOTO_SANS_TELUGU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTelugu/unhinted/otf/NotoSansTelugu-Regular.otf");
static NOTO_SANS_THAANA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansThaana/unhinted/otf/NotoSansThaana-Regular.otf");
static NOTO_SANS_WANCHO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansWancho/unhinted/otf/NotoSansWancho-Regular.otf");
static NOTO_SANS_AVESTAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansAvestan/unhinted/otf/NotoSansAvestan-Regular.otf");
static NOTO_SANS_BENGALI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBengali/unhinted/otf/NotoSansBengali-Regular.otf");
static NOTO_SANS_CYPRIOT_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCypriot/unhinted/otf/NotoSansCypriot-Regular.otf");
static NOTO_SANS_DESERET_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansDeseret/unhinted/otf/NotoSansDeseret-Regular.otf");
static NOTO_SANS_ELBASAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansElbasan/unhinted/otf/NotoSansElbasan-Regular.otf");
static NOTO_SANS_ELYMAIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansElymaic/unhinted/otf/NotoSansElymaic-Regular.otf");
static NOTO_SANS_GRANTHA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGrantha/unhinted/otf/NotoSansGrantha-Regular.otf");
static NOTO_SANS_HANUNOO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansHanunoo/unhinted/otf/NotoSansHanunoo-Regular.otf");
static NOTO_SANS_KANNADA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKannada/unhinted/otf/NotoSansKannada-Regular.otf");
static NOTO_SANS_KAYAH_LI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKayahLi/unhinted/otf/NotoSansKayahLi-Regular.otf");
static NOTO_SANS_LINEAR_A_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLinearA/unhinted/otf/NotoSansLinearA-Regular.otf");
static NOTO_SANS_LINEAR_B_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLinearB/unhinted/otf/NotoSansLinearB-Regular.otf");
static NOTO_SANS_MANDAIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMandaic/unhinted/otf/NotoSansMandaic-Regular.otf");
static NOTO_SANS_MARCHEN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMarchen/unhinted/otf/NotoSansMarchen-Regular.otf");
static NOTO_SANS_MULTANI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMultani/unhinted/otf/NotoSansMultani-Regular.otf");
static NOTO_SANS_MYANMAR_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMyanmar/unhinted/otf/NotoSansMyanmar-Regular.otf");
static NOTO_SANS_OL_CHIKI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOlChiki/unhinted/otf/NotoSansOlChiki-Regular.otf");
static NOTO_SANS_OSMANYA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOsmanya/unhinted/otf/NotoSansOsmanya-Regular.otf");
static NOTO_SANS_PHAGS_PA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPhagsPa/unhinted/otf/NotoSansPhagsPa-Regular.otf");
static NOTO_SANS_SHARADA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSharada/unhinted/otf/NotoSansSharada-Regular.otf");
static NOTO_SANS_SHAVIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansShavian/unhinted/otf/NotoSansShavian-Regular.otf");
static NOTO_SANS_SIDDHAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSiddham/unhinted/otf/NotoSansSiddham-Regular.otf");
static NOTO_SANS_SINHALA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSinhala/unhinted/otf/NotoSansSinhala-Regular.otf");
static NOTO_SANS_SOGDIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSogdian/unhinted/otf/NotoSansSogdian-Regular.otf");
static NOTO_SANS_SOYOMBO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSoyombo/unhinted/otf/NotoSansSoyombo-Regular.otf");
static NOTO_SANS_SYMBOLS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSymbols/unhinted/otf/NotoSansSymbols-Regular.otf");
static NOTO_SANS_TAGALOG_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTagalog/unhinted/otf/NotoSansTagalog-Regular.otf");
static NOTO_SANS_TAI_THAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTaiTham/unhinted/otf/NotoSansTaiTham-Regular.otf");
static NOTO_SANS_TAI_VIET_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTaiViet/unhinted/otf/NotoSansTaiViet-Regular.otf");
static NOTO_SANS_TIRHUTA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTirhuta/unhinted/otf/NotoSansTirhuta-Regular.otf");
static NOTO_SANS_ARABIC_UI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansArabicUI/unhinted/otf/NotoSansArabicUI-Regular.otf");
static NOTO_SANS_ARMENIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansArmenian/unhinted/otf/NotoSansArmenian-Regular.otf");
static NOTO_SANS_BALINESE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBalinese/unhinted/otf/NotoSansBalinese-Regular.otf");
static NOTO_SANS_BASSA_VAH_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBassaVah/unhinted/otf/NotoSansBassaVah-Regular.otf");
static NOTO_SANS_BUGINESE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBuginese/unhinted/otf/NotoSansBuginese-Regular.otf");
static NOTO_SANS_CHEROKEE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCherokee/unhinted/otf/NotoSansCherokee-Regular.otf");
static NOTO_SANS_DUPLOYAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansDuployan/unhinted/otf/NotoSansDuployan-Regular.otf");
static NOTO_SANS_ETHIOPIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansEthiopic/unhinted/otf/NotoSansEthiopic-Regular.otf");
static NOTO_SANS_GEORGIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGeorgian/unhinted/otf/NotoSansGeorgian-Regular.otf");
static NOTO_SANS_GUJARATI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGujarati/unhinted/otf/NotoSansGujarati-Regular.otf");
static NOTO_SANS_GURMUKHI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGurmukhi/unhinted/otf/NotoSansGurmukhi-Regular.otf");
static NOTO_SANS_JAVANESE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansJavanese/unhinted/otf/NotoSansJavanese-Regular.otf");
static NOTO_SANS_MAHAJANI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMahajani/unhinted/otf/NotoSansMahajani-Regular.otf");
static NOTO_SANS_MEROITIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMeroitic/unhinted/otf/NotoSansMeroitic-Regular.otf");
static NOTO_SANS_SYMBOLS2_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSymbols2/unhinted/otf/NotoSansSymbols2-Regular.otf");
static NOTO_SANS_TAGBANWA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTagbanwa/unhinted/otf/NotoSansTagbanwa-Regular.otf");
static NOTO_SANS_TIFINAGH_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTifinagh/unhinted/otf/NotoSansTifinagh-Regular.otf");
static NOTO_SANS_UGARITIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansUgaritic/unhinted/otf/NotoSansUgaritic-Regular.otf");
static NOTO_SANS_VITHKUQI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansVithkuqi/unhinted/otf/NotoSansVithkuqi-Regular.otf");
static NOTO_SANS_BHAIKSUKI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansBhaiksuki/unhinted/otf/NotoSansBhaiksuki-Regular.otf");
static NOTO_SANS_CUNEIFORM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCuneiform/unhinted/otf/NotoSansCuneiform-Regular.otf");
static NOTO_SANS_KHUDAWADI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKhudawadi/unhinted/otf/NotoSansKhudawadi-Regular.otf");
static NOTO_SANS_LAO_LOOPED_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansLaoLooped/unhinted/otf/NotoSansLaoLooped-Regular.otf");
static NOTO_SANS_MALAYALAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMalayalam/unhinted/otf/NotoSansMalayalam-Regular.otf");
static NOTO_SANS_MONGOLIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMongolian/unhinted/otf/NotoSansMongolian-Regular.otf");
static NOTO_SANS_NABATAEAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNabataean/unhinted/otf/NotoSansNabataean-Regular.otf");
static NOTO_SANS_NEW_TAI_LUE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNewTaiLue/unhinted/otf/NotoSansNewTaiLue-Regular.otf");
static NOTO_SANS_OLD_ITALIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldItalic/unhinted/otf/NotoSansOldItalic-Regular.otf");
static NOTO_SANS_OLD_PERMIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldPermic/unhinted/otf/NotoSansOldPermic-Regular.otf");
static NOTO_SANS_OLD_TURKIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldTurkic/unhinted/otf/NotoSansOldTurkic-Regular.otf");
static NOTO_SANS_PALMYRENE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPalmyrene/unhinted/otf/NotoSansPalmyrene-Regular.otf");
static NOTO_SANS_PAU_CIN_HAU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPauCinHau/unhinted/otf/NotoSansPauCinHau-Regular.otf");
static NOTO_SANS_SAMARITAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSamaritan/unhinted/otf/NotoSansSamaritan-Regular.otf");
static NOTO_SANS_SUNDANESE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSundanese/unhinted/otf/NotoSansSundanese-Regular.otf");
static NOTO_SANS_CHORASMIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansChorasmian/unhinted/otf/NotoSansChorasmian-Regular.otf");
static NOTO_SANS_DEVANAGARI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansDevanagari/unhinted/otf/NotoSansDevanagari-Regular.otf");
static NOTO_SANS_GLAGOLITIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGlagolitic/unhinted/otf/NotoSansGlagolitic-Regular.otf");
static NOTO_SANS_KHAROSHTHI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansKharoshthi/unhinted/otf/NotoSansKharoshthi-Regular.otf");
static NOTO_SANS_MANICHAEAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansManichaean/unhinted/otf/NotoSansManichaean-Regular.otf");
static NOTO_SANS_NAG_MUNDARI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNagMundari/unhinted/otf/NotoSansNagMundari-Regular.otf");
static NOTO_SANS_OLD_PERSIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldPersian/unhinted/otf/NotoSansOldPersian-Regular.otf");
static NOTO_SANS_OLD_SOGDIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldSogdian/unhinted/otf/NotoSansOldSogdian-Regular.otf");
static NOTO_SANS_PHOENICIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPhoenician/unhinted/otf/NotoSansPhoenician-Regular.otf");
static NOTO_SANS_SAURASHTRA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSaurashtra/unhinted/otf/NotoSansSaurashtra-Regular.otf");
static NOTO_SANS_THAI_LOOPED_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansThaiLooped/unhinted/otf/NotoSansThaiLooped-Regular.otf");
static NOTO_SANS_WARANG_CITI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansWarangCiti/unhinted/otf/NotoSansWarangCiti-Regular.otf");
static NOTO_SANS_CYPRO_MINOAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCyproMinoan/unhinted/otf/NotoSansCyproMinoan-Regular.otf");
static NOTO_SANS_MEDEFAIDRIN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMedefaidrin/unhinted/otf/NotoSansMedefaidrin-Regular.otf");
static NOTO_SANS_MEETEI_MAYEK_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMeeteiMayek/unhinted/otf/NotoSansMeeteiMayek-Regular.otf");
static NOTO_SANS_NANDINAGARI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNandinagari/unhinted/otf/NotoSansNandinagari-Regular.otf");
static NOTO_SANS_N_KO_UNJOINED_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansNKoUnjoined/unhinted/otf/NotoSansNKoUnjoined-Regular.otf");
static NOTO_SANS_PAHAWH_HMONG_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPahawhHmong/unhinted/otf/NotoSansPahawhHmong-Regular.otf");
static NOTO_SANS_SIGN_WRITING_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSignWriting/unhinted/otf/NotoSansSignWriting-Regular.otf");
static NOTO_SANS_SORA_SOMPENG_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSoraSompeng/unhinted/otf/NotoSansSoraSompeng-Regular.otf");
static NOTO_SANS_SYLOTI_NAGRI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSylotiNagri/unhinted/otf/NotoSansSylotiNagri-Regular.otf");
static NOTO_SANS_GUNJALA_GONDI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansGunjalaGondi/unhinted/otf/NotoSansGunjalaGondi-Regular.otf");
static NOTO_SANS_MASARAM_GONDI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMasaramGondi/unhinted/otf/NotoSansMasaramGondi-Regular.otf");
static NOTO_SANS_MENDE_KIKAKUI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMendeKikakui/unhinted/otf/NotoSansMendeKikakui-Regular.otf");
static NOTO_SANS_OLD_HUNGARIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldHungarian/unhinted/otf/NotoSansOldHungarian-Regular.otf");
static NOTO_SANS_ADLAM_UNJOINED_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansAdlamUnjoined/unhinted/otf/NotoSansAdlamUnjoined-Regular.otf");
static NOTO_SANS_MAYAN_NUMERALS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansMayanNumerals/unhinted/otf/NotoSansMayanNumerals-Regular.otf");
static NOTO_SANS_SYRIAC_EASTERN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSyriacEastern/unhinted/otf/NotoSansSyriacEastern-Regular.otf");
static NOTO_SANS_SYRIAC_WESTERN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansSyriacWestern/unhinted/otf/NotoSansSyriacWestern-Regular.otf");
static NOTO_SANS_HANIFI_ROHINGYA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansHanifiRohingya/unhinted/otf/NotoSansHanifiRohingya-Regular.otf");
static NOTO_SANS_PSALTER_PAHLAVI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansPsalterPahlavi/unhinted/otf/NotoSansPsalterPahlavi-Regular.otf");
static NOTO_SANS_IMPERIAL_ARAMAIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansImperialAramaic/unhinted/otf/NotoSansImperialAramaic-Regular.otf");
static NOTO_SANS_OLD_NORTH_ARABIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldNorthArabian/unhinted/otf/NotoSansOldNorthArabian-Regular.otf");
static NOTO_SANS_OLD_SOUTH_ARABIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansOldSouthArabian/unhinted/otf/NotoSansOldSouthArabian-Regular.otf");
static NOTO_SANS_TAMIL_SUPPLEMENT_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansTamilSupplement/unhinted/otf/NotoSansTamilSupplement-Regular.otf");
static NOTO_SANS_ZANABAZAR_SQUARE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansZanabazarSquare/unhinted/otf/NotoSansZanabazarSquare-Regular.otf");
static NOTO_SANS_CAUCASIAN_ALBANIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCaucasianAlbanian/unhinted/otf/NotoSansCaucasianAlbanian-Regular.otf");
static NOTO_SANS_INDIC_SIYAQ_NUMBERS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansIndicSiyaqNumbers/unhinted/otf/NotoSansIndicSiyaqNumbers-Regular.otf");
static NOTO_SANS_CANADIAN_ABORIGINAL_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansCanadianAboriginal/unhinted/otf/NotoSansCanadianAboriginal-Regular.otf");
static NOTO_SANS_EGYPTIAN_HIEROGLYPHS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansEgyptianHieroglyphs/unhinted/otf/NotoSansEgyptianHieroglyphs-Regular.otf");
static NOTO_SANS_ANATOLIAN_HIEROGLYPHS_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansAnatolianHieroglyphs/unhinted/otf/NotoSansAnatolianHieroglyphs-Regular.otf");
static NOTO_SANS_INSCRIPTIONAL_PAHLAVI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansInscriptionalPahlavi/unhinted/otf/NotoSansInscriptionalPahlavi-Regular.otf");
static NOTO_SANS_INSCRIPTIONAL_PARTHIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSansInscriptionalParthian/unhinted/otf/NotoSansInscriptionalParthian-Regular.otf");

pub static NOTO_SANS: [&LazyLock<FontRef<'static>>; 155] = [
    &NOTO_SANS_FONT_REF,
    &NOTO_SANS_YI_FONT_REF,
    &NOTO_SANS_LAO_FONT_REF,
    &NOTO_SANS_MRO_FONT_REF,
    &NOTO_SANS_N_KO_FONT_REF,
    &NOTO_SANS_VAI_FONT_REF,
    &NOTO_SANS_CHAM_FONT_REF,
    &NOTO_SANS_KAWI_FONT_REF,
    &NOTO_SANS_LISU_FONT_REF,
    &NOTO_SANS_MATH_FONT_REF,
    &NOTO_SANS_MIAO_FONT_REF,
    &NOTO_SANS_MODI_FONT_REF,
    &NOTO_SANS_MONO_FONT_REF,
    &NOTO_SANS_NEWA_FONT_REF,
    &NOTO_SANS_THAI_FONT_REF,
    &NOTO_SANS_ADLAM_FONT_REF,
    &NOTO_SANS_BAMUM_FONT_REF,
    &NOTO_SANS_BATAK_FONT_REF,
    &NOTO_SANS_BUHID_FONT_REF,
    &NOTO_SANS_KHMER_FONT_REF,
    &NOTO_SANS_LIMBU_FONT_REF,
    &NOTO_SANS_NUSHU_FONT_REF,
    &NOTO_SANS_OGHAM_FONT_REF,
    &NOTO_SANS_ORIYA_FONT_REF,
    &NOTO_SANS_OSAGE_FONT_REF,
    &NOTO_SANS_RUNIC_FONT_REF,
    &NOTO_SANS_TAI_LE_FONT_REF,
    &NOTO_SANS_TAKRI_FONT_REF,
    &NOTO_SANS_TAMIL_FONT_REF,
    &NOTO_SANS_ARABIC_FONT_REF,
    &NOTO_SANS_BRAHMI_FONT_REF,
    &NOTO_SANS_CARIAN_FONT_REF,
    &NOTO_SANS_CHAKMA_FONT_REF,
    &NOTO_SANS_COPTIC_FONT_REF,
    &NOTO_SANS_GOTHIC_FONT_REF,
    &NOTO_SANS_HATRAN_FONT_REF,
    &NOTO_SANS_HEBREW_FONT_REF,
    &NOTO_SANS_KAITHI_FONT_REF,
    &NOTO_SANS_KHOJKI_FONT_REF,
    &NOTO_SANS_LEPCHA_FONT_REF,
    &NOTO_SANS_LYCIAN_FONT_REF,
    &NOTO_SANS_LYDIAN_FONT_REF,
    &NOTO_SANS_REJANG_FONT_REF,
    &NOTO_SANS_SYRIAC_FONT_REF,
    &NOTO_SANS_TANGSA_FONT_REF,
    &NOTO_SANS_TELUGU_FONT_REF,
    &NOTO_SANS_THAANA_FONT_REF,
    &NOTO_SANS_WANCHO_FONT_REF,
    &NOTO_SANS_AVESTAN_FONT_REF,
    &NOTO_SANS_BENGALI_FONT_REF,
    &NOTO_SANS_CYPRIOT_FONT_REF,
    &NOTO_SANS_DESERET_FONT_REF,
    &NOTO_SANS_ELBASAN_FONT_REF,
    &NOTO_SANS_ELYMAIC_FONT_REF,
    &NOTO_SANS_GRANTHA_FONT_REF,
    &NOTO_SANS_HANUNOO_FONT_REF,
    &NOTO_SANS_KANNADA_FONT_REF,
    &NOTO_SANS_KAYAH_LI_FONT_REF,
    &NOTO_SANS_LINEAR_A_FONT_REF,
    &NOTO_SANS_LINEAR_B_FONT_REF,
    &NOTO_SANS_MANDAIC_FONT_REF,
    &NOTO_SANS_MARCHEN_FONT_REF,
    &NOTO_SANS_MULTANI_FONT_REF,
    &NOTO_SANS_MYANMAR_FONT_REF,
    &NOTO_SANS_OL_CHIKI_FONT_REF,
    &NOTO_SANS_OSMANYA_FONT_REF,
    &NOTO_SANS_PHAGS_PA_FONT_REF,
    &NOTO_SANS_SHARADA_FONT_REF,
    &NOTO_SANS_SHAVIAN_FONT_REF,
    &NOTO_SANS_SIDDHAM_FONT_REF,
    &NOTO_SANS_SINHALA_FONT_REF,
    &NOTO_SANS_SOGDIAN_FONT_REF,
    &NOTO_SANS_SOYOMBO_FONT_REF,
    &NOTO_SANS_SYMBOLS_FONT_REF,
    &NOTO_SANS_TAGALOG_FONT_REF,
    &NOTO_SANS_TAI_THAM_FONT_REF,
    &NOTO_SANS_TAI_VIET_FONT_REF,
    &NOTO_SANS_TIRHUTA_FONT_REF,
    &NOTO_SANS_ARABIC_UI_FONT_REF,
    &NOTO_SANS_ARMENIAN_FONT_REF,
    &NOTO_SANS_BALINESE_FONT_REF,
    &NOTO_SANS_BASSA_VAH_FONT_REF,
    &NOTO_SANS_BUGINESE_FONT_REF,
    &NOTO_SANS_CHEROKEE_FONT_REF,
    &NOTO_SANS_DUPLOYAN_FONT_REF,
    &NOTO_SANS_ETHIOPIC_FONT_REF,
    &NOTO_SANS_GEORGIAN_FONT_REF,
    &NOTO_SANS_GUJARATI_FONT_REF,
    &NOTO_SANS_GURMUKHI_FONT_REF,
    &NOTO_SANS_JAVANESE_FONT_REF,
    &NOTO_SANS_MAHAJANI_FONT_REF,
    &NOTO_SANS_MEROITIC_FONT_REF,
    &NOTO_SANS_SYMBOLS2_FONT_REF,
    &NOTO_SANS_TAGBANWA_FONT_REF,
    &NOTO_SANS_TIFINAGH_FONT_REF,
    &NOTO_SANS_UGARITIC_FONT_REF,
    &NOTO_SANS_VITHKUQI_FONT_REF,
    &NOTO_SANS_BHAIKSUKI_FONT_REF,
    &NOTO_SANS_CUNEIFORM_FONT_REF,
    &NOTO_SANS_KHUDAWADI_FONT_REF,
    &NOTO_SANS_LAO_LOOPED_FONT_REF,
    &NOTO_SANS_MALAYALAM_FONT_REF,
    &NOTO_SANS_MONGOLIAN_FONT_REF,
    &NOTO_SANS_NABATAEAN_FONT_REF,
    &NOTO_SANS_NEW_TAI_LUE_FONT_REF,
    &NOTO_SANS_OLD_ITALIC_FONT_REF,
    &NOTO_SANS_OLD_PERMIC_FONT_REF,
    &NOTO_SANS_OLD_TURKIC_FONT_REF,
    &NOTO_SANS_PALMYRENE_FONT_REF,
    &NOTO_SANS_PAU_CIN_HAU_FONT_REF,
    &NOTO_SANS_SAMARITAN_FONT_REF,
    &NOTO_SANS_SUNDANESE_FONT_REF,
    &NOTO_SANS_CHORASMIAN_FONT_REF,
    &NOTO_SANS_DEVANAGARI_FONT_REF,
    &NOTO_SANS_GLAGOLITIC_FONT_REF,
    &NOTO_SANS_KHAROSHTHI_FONT_REF,
    &NOTO_SANS_MANICHAEAN_FONT_REF,
    &NOTO_SANS_NAG_MUNDARI_FONT_REF,
    &NOTO_SANS_OLD_PERSIAN_FONT_REF,
    &NOTO_SANS_OLD_SOGDIAN_FONT_REF,
    &NOTO_SANS_PHOENICIAN_FONT_REF,
    &NOTO_SANS_SAURASHTRA_FONT_REF,
    &NOTO_SANS_THAI_LOOPED_FONT_REF,
    &NOTO_SANS_WARANG_CITI_FONT_REF,
    &NOTO_SANS_CYPRO_MINOAN_FONT_REF,
    &NOTO_SANS_MEDEFAIDRIN_FONT_REF,
    &NOTO_SANS_MEETEI_MAYEK_FONT_REF,
    &NOTO_SANS_NANDINAGARI_FONT_REF,
    &NOTO_SANS_N_KO_UNJOINED_FONT_REF,
    &NOTO_SANS_PAHAWH_HMONG_FONT_REF,
    &NOTO_SANS_SIGN_WRITING_FONT_REF,
    &NOTO_SANS_SORA_SOMPENG_FONT_REF,
    &NOTO_SANS_SYLOTI_NAGRI_FONT_REF,
    &NOTO_SANS_GUNJALA_GONDI_FONT_REF,
    &NOTO_SANS_MASARAM_GONDI_FONT_REF,
    &NOTO_SANS_MENDE_KIKAKUI_FONT_REF,
    &NOTO_SANS_OLD_HUNGARIAN_FONT_REF,
    &NOTO_SANS_ADLAM_UNJOINED_FONT_REF,
    &NOTO_SANS_MAYAN_NUMERALS_FONT_REF,
    &NOTO_SANS_SYRIAC_EASTERN_FONT_REF,
    &NOTO_SANS_SYRIAC_WESTERN_FONT_REF,
    &NOTO_SANS_HANIFI_ROHINGYA_FONT_REF,
    &NOTO_SANS_PSALTER_PAHLAVI_FONT_REF,
    &NOTO_SANS_IMPERIAL_ARAMAIC_FONT_REF,
    &NOTO_SANS_OLD_NORTH_ARABIAN_FONT_REF,
    &NOTO_SANS_OLD_SOUTH_ARABIAN_FONT_REF,
    &NOTO_SANS_TAMIL_SUPPLEMENT_FONT_REF,
    &NOTO_SANS_ZANABAZAR_SQUARE_FONT_REF,
    &NOTO_SANS_CAUCASIAN_ALBANIAN_FONT_REF,
    &NOTO_SANS_INDIC_SIYAQ_NUMBERS_FONT_REF,
    &NOTO_SANS_CANADIAN_ABORIGINAL_FONT_REF,
    &NOTO_SANS_EGYPTIAN_HIEROGLYPHS_FONT_REF,
    &NOTO_SANS_ANATOLIAN_HIEROGLYPHS_FONT_REF,
    &NOTO_SANS_INSCRIPTIONAL_PAHLAVI_FONT_REF,
    &NOTO_SANS_INSCRIPTIONAL_PARTHIAN_FONT_REF,
];


static NOTO_SERIF_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerif/unhinted/otf/NotoSerif-Regular.otf");
static NOTO_SERIF_LAO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifLao/unhinted/otf/NotoSerifLao-Regular.otf");
static NOTO_SERIF_AHOM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifAhom/unhinted/otf/NotoSerifAhom-Regular.otf");
static NOTO_SERIF_THAI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifThai/unhinted/otf/NotoSerifThai-Regular.otf");
static NOTO_SERIF_TOTO_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifToto/unhinted/otf/NotoSerifToto-Regular.otf");
static NOTO_SERIF_DOGRA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifDogra/unhinted/otf/NotoSerifDogra-Regular.otf");
static NOTO_SERIF_KHMER_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifKhmer/unhinted/otf/NotoSerifKhmer-Regular.otf");
static NOTO_SERIF_ORIYA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifOriya/unhinted/otf/NotoSerifOriya-Regular.otf");
static NOTO_SERIF_TAMIL_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifTamil/unhinted/otf/NotoSerifTamil-Regular.otf");
static NOTO_SERIF_HEBREW_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifHebrew/unhinted/otf/NotoSerifHebrew-Regular.otf");
static NOTO_SERIF_KHOJKI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifKhojki/unhinted/otf/NotoSerifKhojki-Regular.otf");
static NOTO_SERIF_TANGUT_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifTangut/unhinted/otf/NotoSerifTangut-Regular.otf");
static NOTO_SERIF_TELUGU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifTelugu/unhinted/otf/NotoSerifTelugu-Regular.otf");
static NOTO_SERIF_YEZIDI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifYezidi/unhinted/otf/NotoSerifYezidi-Regular.otf");
static NOTO_SERIF_BENGALI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifBengali/unhinted/otf/NotoSerifBengali-Regular.otf");
static NOTO_SERIF_DISPLAY_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifDisplay/unhinted/otf/NotoSerifDisplay-Regular.otf");
static NOTO_SERIF_GRANTHA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifGrantha/unhinted/otf/NotoSerifGrantha-Regular.otf");
static NOTO_SERIF_KANNADA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifKannada/unhinted/otf/NotoSerifKannada-Regular.otf");
static NOTO_SERIF_MAKASAR_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifMakasar/unhinted/otf/NotoSerifMakasar-Regular.otf");
static NOTO_SERIF_MYANMAR_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifMyanmar/unhinted/otf/NotoSerifMyanmar-Regular.otf");
static NOTO_SERIF_NP_HMONG_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifNPHmong/unhinted/otf/NotoSerifNPHmong-Regular.otf");
static NOTO_SERIF_SINHALA_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifSinhala/unhinted/otf/NotoSerifSinhala-Regular.otf");
static NOTO_SERIF_TIBETAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifTibetan/unhinted/otf/NotoSerifTibetan-Regular.otf");
static NOTO_SERIF_ARMENIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifArmenian/unhinted/otf/NotoSerifArmenian-Regular.otf");
static NOTO_SERIF_BALINESE_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifBalinese/unhinted/otf/NotoSerifBalinese-Regular.otf");
static NOTO_SERIF_ETHIOPIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifEthiopic/unhinted/otf/NotoSerifEthiopic-Regular.otf");
static NOTO_SERIF_GEORGIAN_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifGeorgian/unhinted/otf/NotoSerifGeorgian-Regular.otf");
static NOTO_SERIF_GUJARATI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifGujarati/unhinted/otf/NotoSerifGujarati-Regular.otf");
static NOTO_SERIF_GURMUKHI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifGurmukhi/unhinted/otf/NotoSerifGurmukhi-Regular.otf");
static NOTO_SERIF_VITHKUQI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifVithkuqi/unhinted/otf/NotoSerifVithkuqi-Regular.otf");
static NOTO_SERIF_MALAYALAM_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifMalayalam/unhinted/otf/NotoSerifMalayalam-Regular.otf");
static NOTO_SERIF_OLD_UYGHUR_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifOldUyghur/unhinted/otf/NotoSerifOldUyghur-Regular.otf");
static NOTO_SERIF_DEVANAGARI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifDevanagari/unhinted/otf/NotoSerifDevanagari-Regular.otf");
static NOTO_SERIF_DIVES_AKURU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifDivesAkuru/unhinted/otf/NotoSerifDivesAkuru-Regular.otf");
static NOTO_SERIF_OTTOMAN_SIYAQ_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifOttomanSiyaq/unhinted/otf/NotoSerifOttomanSiyaq-Regular.otf");
static NOTO_SERIF_KHITAN_SMALL_SCRIPT_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoSerifKhitanSmallScript/unhinted/otf/NotoSerifKhitanSmallScript-Regular.otf");

pub static NOTO_SERIF: [&LazyLock<FontRef<'static>>; 36] = [
    &NOTO_SERIF_FONT_REF,
    &NOTO_SERIF_LAO_FONT_REF,
    &NOTO_SERIF_AHOM_FONT_REF,
    &NOTO_SERIF_THAI_FONT_REF,
    &NOTO_SERIF_TOTO_FONT_REF,
    &NOTO_SERIF_DOGRA_FONT_REF,
    &NOTO_SERIF_KHMER_FONT_REF,
    &NOTO_SERIF_ORIYA_FONT_REF,
    &NOTO_SERIF_TAMIL_FONT_REF,
    &NOTO_SERIF_HEBREW_FONT_REF,
    &NOTO_SERIF_KHOJKI_FONT_REF,
    &NOTO_SERIF_TANGUT_FONT_REF,
    &NOTO_SERIF_TELUGU_FONT_REF,
    &NOTO_SERIF_YEZIDI_FONT_REF,
    &NOTO_SERIF_BENGALI_FONT_REF,
    &NOTO_SERIF_DISPLAY_FONT_REF,
    &NOTO_SERIF_GRANTHA_FONT_REF,
    &NOTO_SERIF_KANNADA_FONT_REF,
    &NOTO_SERIF_MAKASAR_FONT_REF,
    &NOTO_SERIF_MYANMAR_FONT_REF,
    &NOTO_SERIF_NP_HMONG_FONT_REF,
    &NOTO_SERIF_SINHALA_FONT_REF,
    &NOTO_SERIF_TIBETAN_FONT_REF,
    &NOTO_SERIF_ARMENIAN_FONT_REF,
    &NOTO_SERIF_BALINESE_FONT_REF,
    &NOTO_SERIF_ETHIOPIC_FONT_REF,
    &NOTO_SERIF_GEORGIAN_FONT_REF,
    &NOTO_SERIF_GUJARATI_FONT_REF,
    &NOTO_SERIF_GURMUKHI_FONT_REF,
    &NOTO_SERIF_VITHKUQI_FONT_REF,
    &NOTO_SERIF_MALAYALAM_FONT_REF,
    &NOTO_SERIF_OLD_UYGHUR_FONT_REF,
    &NOTO_SERIF_DEVANAGARI_FONT_REF,
    &NOTO_SERIF_DIVES_AKURU_FONT_REF,
    &NOTO_SERIF_OTTOMAN_SIYAQ_FONT_REF,
    &NOTO_SERIF_KHITAN_SMALL_SCRIPT_FONT_REF,
];


static NOTO_MUSIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoMusic/unhinted/otf/NotoMusic-Regular.otf");
static NOTO_KUFI_ARABIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoKufiArabic/unhinted/otf/NotoKufiArabic-Regular.otf");
static NOTO_NASKH_ARABIC_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoNaskhArabic/unhinted/otf/NotoNaskhArabic-Regular.otf");
static NOTO_RASHI_HEBREW_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoRashiHebrew/unhinted/otf/NotoRashiHebrew-Regular.otf");
static NOTO_NASTALIQ_URDU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoNastaliqUrdu/unhinted/otf/NotoNastaliqUrdu-Regular.otf");
static NOTO_NASKH_ARABIC_UI_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoNaskhArabicUI/unhinted/otf/NotoNaskhArabicUI-Regular.otf");
static NOTO_TRADITIONAL_NUSHU_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoTraditionalNushu/unhinted/otf/NotoTraditionalNushu-Regular.otf");
static NOTO_FANGSONG_KSS_ROTATED_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoFangsongKSSRotated/unhinted/otf/NotoFangsongKSSRotated-Regular.otf");
static NOTO_FANGSONG_KSS_VERTICAL_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoFangsongKSSVertical/unhinted/otf/NotoFangsongKSSVertical-Regular.otf");
static NOTO_ZNAMENNY_MUSICAL_NOTATION_FONT_REF: LazyLock<FontRef<'static>> = lazy_font!("../fonts/Noto/fonts/NotoZnamennyMusicalNotation/unhinted/otf/NotoZnamennyMusicalNotation-Regular.otf");

pub static NOTO_REST: [&LazyLock<FontRef<'static>>; 10] = [
    &NOTO_MUSIC_FONT_REF,
    &NOTO_KUFI_ARABIC_FONT_REF,
    &NOTO_NASKH_ARABIC_FONT_REF,
    &NOTO_RASHI_HEBREW_FONT_REF,
    &NOTO_NASTALIQ_URDU_FONT_REF,
    &NOTO_NASKH_ARABIC_UI_FONT_REF,
    &NOTO_TRADITIONAL_NUSHU_FONT_REF,
    &NOTO_FANGSONG_KSS_ROTATED_FONT_REF,
    &NOTO_FANGSONG_KSS_VERTICAL_FONT_REF,
    &NOTO_ZNAMENNY_MUSICAL_NOTATION_FONT_REF,
];


#[cfg(test)]
#[test]
#[rustfmt::skip]
fn fonts_are_valid() {
    let _ = NOTO_SANS.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
    let _ = NOTO_SERIF.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
    let _ = NOTO_REST.iter().map(|this| LazyLock::force(this)).collect::<Vec<_>>();
}
