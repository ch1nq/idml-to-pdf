use serde::Deserialize;

pub trait CommonTextPropertiesAttributes {
    fn applied_character_style(&self) -> Option<String> {
        None
    }
    fn applied_conditions(&self) -> Option<String> {
        None
    }
    fn applied_language(&self) -> Option<String> {
        None
    }
    fn applied_paragraph_style(&self) -> Option<String> {
        None
    }
    fn auto_leading(&self) -> Option<f64> {
        None
    }
    fn auto_tcy(&self) -> Option<i16> {
        None
    }
    fn auto_tcy_include_roman(&self) -> Option<bool> {
        None
    }
    fn baseline_shift(&self) -> Option<f64> {
        None
    }
    fn bullets_alignment(&self) -> Option<String> {
        None
    }
    fn bullets_and_numbering_list_type(&self) -> Option<String> {
        None
    }
    fn bullets_text_after(&self) -> Option<String> {
        None
    }
    fn bunri_kinshi(&self) -> Option<bool> {
        None
    }
    fn captilization(&self) -> Option<Capitalization> {
        None
    }
    fn character_alignment(&self) -> Option<CharacterAlignment> {
        None
    }
    fn character_direction(&self) -> Option<CharacterDirection> {
        None
    }
    fn character_rotation(&self) -> Option<f64> {
        None
    }
    fn cjk_grid_tracking(&self) -> Option<bool> {
        None
    }
    fn composer(&self) -> Option<String> {
        None
    }
    fn desired_word_spacing(&self) -> Option<f64> {
        None
    }
    fn diacritic_position(&self) -> Option<DiacriticPosition> {
        None
    }
    fn digits_type(&self) -> Option<DigitsType> {
        None
    }
    fn drop_cap_characters(&self) -> Option<i16> {
        None
    }
    fn drop_cap_lines(&self) -> Option<i16> {
        None
    }
    fn drop_cap_detail(&self) -> Option<i32> {
        None
    }
    fn end_join(&self) -> Option<OutlineJoin> {
        None
    }
    fn fill_color(&self) -> Option<String> {
        None
    }
    fn fill_tint(&self) -> Option<f64> {
        None
    }
    fn first_line_indent(&self) -> Option<f64> {
        None
    }
    fn font_style(&self) -> Option<String> {
        None
    }
    fn glyph_form(&self) -> Option<AlternateGlyphForms> {
        None
    }
    fn goto_next_x(&self) -> Option<GotoNextX> {
        None
    }
    fn gradient_fill_angle(&self) -> Option<f64> {
        None
    }
    fn gradient_fill_length(&self) -> Option<f64> {
        None
    }
    fn gradient_fill_start(&self) -> Option<Vec<f64>> {
        None
    }
    fn gradient_stroke_angle(&self) -> Option<f64> {
        None
    }
    fn gradient_stroke_length(&self) -> Option<f64> {
        None
    }
    fn gradient_stroke_start(&self) -> Option<Vec<f64>> {
        None
    }
    fn grid_align_first_line_only(&self) -> Option<bool> {
        None
    }
    fn grid_alignment(&self) -> Option<GridAlignment> {
        None
    }
    fn grid_gyoudori(&self) -> Option<i16> {
        None
    }
    fn horizontal_scale(&self) -> Option<f64> {
        None
    }
    fn hyphen_weight(&self) -> Option<i16> {
        None
    }
    fn hyphenate_across_columns(&self) -> Option<bool> {
        None
    }
    fn hyphenate_after_first(&self) -> Option<i16> {
        None
    }
    fn hyphenate_before_last(&self) -> Option<i16> {
        None
    }
    fn hyphenate_capitalized_words(&self) -> Option<bool> {
        None
    }
    fn hyphenate_ladder_limit(&self) -> Option<i16> {
        None
    }
    fn hyphenate_last_word(&self) -> Option<bool> {
        None
    }
    fn hyphenate_words_longer_than(&self) -> Option<i16> {
        None
    }
    fn hyphenation(&self) -> Option<bool> {
        None
    }
    fn hyphenation_zone(&self) -> Option<f64> {
        None
    }
    fn ignore_edge_alignment(&self) -> Option<bool> {
        None
    }
    fn jidori(&self) -> Option<i16> {
        None
    }
    fn justification(&self) -> Option<Justification> {
        None
    }
    fn kashidas(&self) -> Option<Kashidas> {
        None
    }
    fn keep_all_lines_together(&self) -> Option<bool> {
        None
    }
    fn keep_first_lines(&self) -> Option<i16> {
        None
    }
    fn keep_last_lines(&self) -> Option<i16> {
        None
    }
    fn keep_lines_together(&self) -> Option<bool> {
        None
    }
    fn keep_rule_above_in_frame(&self) -> Option<bool> {
        None
    }
    fn keep_with_next(&self) -> Option<bool> {
        None
    }
    fn keep_with_previous(&self) -> Option<bool> {
        None
    }
    fn kenten_alignment(&self) -> Option<KentenAlignment> {
        None
    }
    fn kenten_custom_character(&self) -> Option<String> {
        None
    }
    fn kenten_font_size(&self) -> Option<f64> {
        None
    }
    fn kenten_kind(&self) -> Option<KentenCharacter> {
        None
    }
    fn kenten_overprint_fill(&self) -> Option<AdornmentOverprint> {
        None
    }
    fn kenten_overprint_stroke(&self) -> Option<AdornmentOverprint> {
        None
    }
    fn kenten_placement(&self) -> Option<f64> {
        None
    }
    fn kenten_position(&self) -> Option<RubyKentenPosition> {
        None
    }
    fn kenten_stroke_tint(&self) -> Option<f64> {
        None
    }
    fn kenten_tint(&self) -> Option<f64> {
        None
    }
    fn kenten_weight(&self) -> Option<f64> {
        None
    }
    fn kenten_x_scale(&self) -> Option<f64> {
        None
    }
    fn kenten_y_scale(&self) -> Option<f64> {
        None
    }
    fn kerning_method(&self) -> Option<String> {
        None
    }
    fn kerning_value(&self) -> Option<f64> {
        None
    }
    fn keyboard_direction(&self) -> Option<CharacterDirection> {
        None
    }
    fn kinsoku_hang_type(&self) -> Option<KinsokuHangTypes> {
        None
    }
    fn kinsoku_type(&self) -> Option<KinsokuType> {
        None
    }
    fn last_line_indent(&self) -> Option<f64> {
        None
    }
    fn leading_aki(&self) -> Option<f64> {
        None
    }
    fn leading_model(&self) -> Option<LeadingModel> {
        None
    }
    fn left_indent(&self) -> Option<f64> {
        None
    }
    fn ligatures(&self) -> Option<bool> {
        None
    }
    fn link_resource_id(&self) -> Option<i32> {
        None
    }
    fn maximum_glyph_scaling(&self) -> Option<f64> {
        None
    }
    fn maximum_letter_spacing(&self) -> Option<f64> {
        None
    }
    fn maximum_word_spacing(&self) -> Option<f64> {
        None
    }
    fn minimum_glyph_scaling(&self) -> Option<f64> {
        None
    }
    fn minimum_letter_spacing(&self) -> Option<f64> {
        None
    }
    fn minimum_word_spacing(&self) -> Option<f64> {
        None
    }
    fn miter_limit(&self) -> Option<f64> {
        None
    }
    fn no_break(&self) -> Option<bool> {
        None
    }
    fn numbering_alignment(&self) -> Option<ListAlignment> {
        None
    }
    fn numbering_apply_restart_policy(&self) -> Option<bool> {
        None
    }
    fn numbering_continue(&self) -> Option<bool> {
        None
    }
    fn numbering_expression(&self) -> Option<String> {
        None
    }
    fn numbering_level(&self) -> Option<i32> {
        None
    }
    fn numbering_start_at(&self) -> Option<i32> {
        None
    }
    fn otf_contextual_alternate(&self) -> Option<bool> {
        None
    }
    fn otf_discretionary_ligature(&self) -> Option<bool> {
        None
    }
    fn otf_figure_style(&self) -> Option<OTFFigureStyle> {
        None
    }
    fn otf_fraction(&self) -> Option<bool> {
        None
    }
    fn otf_hv_kana(&self) -> Option<bool> {
        None
    }
    fn otf_historical(&self) -> Option<bool> {
        None
    }
    fn otf_justification_alternate(&self) -> Option<bool> {
        None
    }
    fn otf_locale(&self) -> Option<bool> {
        None
    }
    fn otf_mark(&self) -> Option<bool> {
        None
    }
    fn otf_ordinal(&self) -> Option<bool> {
        None
    }
    fn otf_overlap_swash(&self) -> Option<bool> {
        None
    }
    fn otf_proportional_metrics(&self) -> Option<bool> {
        None
    }
    fn otf_roman_italics(&self) -> Option<bool> {
        None
    }
    fn otf_slashed_zero(&self) -> Option<bool> {
        None
    }
    fn otf_stretched_alternate(&self) -> Option<bool> {
        None
    }
    fn otf_stylistic_alternate(&self) -> Option<bool> {
        None
    }
    fn otf_stylistic_sets(&self) -> Option<i32> {
        None
    }
    fn otf_swash(&self) -> Option<bool> {
        None
    }
    fn otf_titling(&self) -> Option<bool> {
        None
    }
    fn otf_overprint_fill(&self) -> Option<bool> {
        None
    }
    fn otf_overprint_stroke(&self) -> Option<bool> {
        None
    }
    fn page_number_type(&self) -> Option<PageNumberType> {
        None
    }
    fn paragraph_direction(&self) -> Option<ParagraphDirection> {
        None
    }
    fn paragraph_gyoudori(&self) -> Option<bool> {
        None
    }
    fn paragraph_justification(&self) -> Option<ParagraphJustification> {
        None
    }
    fn point_size(&self) -> Option<f64> {
        None
    }
    fn position(&self) -> Option<Position> {
        None
    }
    fn positional_form(&self) -> Option<PositionalForms> {
        None
    }
    fn rensuuji(&self) -> Option<bool> {
        None
    }
    fn right_indent(&self) -> Option<f64> {
        None
    }
    fn rotate_single_byte_character(&self) -> Option<bool> {
        None
    }
    fn ruby_alignment(&self) -> Option<RubyAlignments> {
        None
    }
    fn ruby_auto_align(&self) -> Option<bool> {
        None
    }
    fn ruby_auto_scaling(&self) -> Option<bool> {
        None
    }
    fn ruby_auto_tcy_auto_scale(&self) -> Option<bool> {
        None
    }
    fn ruby_auto_tcy_digits(&self) -> Option<i16> {
        None
    }
    fn ruby_auto_tcy_include_roman(&self) -> Option<bool> {
        None
    }
    fn ruby_flag(&self) -> Option<i32> {
        None
    }
    fn ruby_font_size(&self) -> Option<f64> {
        None
    }
    fn ruby_open_type(&self) -> Option<bool> {
        None
    }
    fn ruby_overhang(&self) -> Option<bool> {
        None
    }
    fn ruby_overprint_fill(&self) -> Option<AdornmentOverprint> {
        None
    }
    fn ruby_overprint_stroke(&self) -> Option<AdornmentOverprint> {
        None
    }
    fn ruby_parent_overhang_amount(&self) -> Option<RubyOverhang> {
        None
    }
    fn ruby_parent_scaling_percent(&self) -> Option<f64> {
        None
    }
    fn ruby_parent_spacing(&self) -> Option<RubyParentSpacing> {
        None
    }
    fn ruby_position(&self) -> Option<RubyKentenPosition> {
        None
    }
    fn ruby_string(&self) -> Option<String> {
        None
    }
    fn ruby_stroke_tint(&self) -> Option<f64> {
        None
    }
    fn ruby_tint(&self) -> Option<f64> {
        None
    }
    fn ruby_type(&self) -> Option<RubyTypes> {
        None
    }
    fn ruby_weight(&self) -> Option<f64> {
        None
    }
    fn ruby_x_offset(&self) -> Option<f64> {
        None
    }
    fn ruby_x_scale(&self) -> Option<f64> {
        None
    }
    fn ruby_y_offset(&self) -> Option<f64> {
        None
    }
    fn ruby_y_scale(&self) -> Option<f64> {
        None
    }
    fn rule_above(&self) -> Option<bool> {
        None
    }
    fn rule_above_gap_overprint(&self) -> Option<bool> {
        None
    }
    fn rule_above_gap_tint(&self) -> Option<f64> {
        None
    }
    fn rule_above_left_indent(&self) -> Option<f64> {
        None
    }
    fn rule_above_weight(&self) -> Option<f64> {
        None
    }
    fn rule_above_offset(&self) -> Option<f64> {
        None
    }
    fn rule_above_overprint(&self) -> Option<bool> {
        None
    }
    fn rule_above_right_indent(&self) -> Option<f64> {
        None
    }
    fn rule_above_tint(&self) -> Option<f64> {
        None
    }
    fn rule_above_width(&self) -> Option<RuleWidth> {
        None
    }
    fn rule_below_gap_overprint(&self) -> Option<bool> {
        None
    }
    fn rule_below_gap_tint(&self) -> Option<f64> {
        None
    }
    fn rule_below_left_indent(&self) -> Option<f64> {
        None
    }
    fn rule_below_weight(&self) -> Option<f64> {
        None
    }
    fn rule_below_offset(&self) -> Option<f64> {
        None
    }
    fn rule_below_overprint(&self) -> Option<bool> {
        None
    }
    fn rule_below_right_indent(&self) -> Option<f64> {
        None
    }
    fn rule_below_tint(&self) -> Option<f64> {
        None
    }
    fn rule_below_width(&self) -> Option<RuleWidth> {
        None
    }
    fn scale_affects_line_height(&self) -> Option<bool> {
        None
    }
    fn shatai_adjust_rotation(&self) -> Option<bool> {
        None
    }
    fn shatai_adjust_tsume(&self) -> Option<bool> {
        None
    }
    fn shatai_adjust_angle(&self) -> Option<f64> {
        None
    }
    fn shatai_magnification(&self) -> Option<f64> {
        None
    }
    fn single_word_justification(&self) -> Option<SingleWordJustification> {
        None
    }
    fn skew(&self) -> Option<f64> {
        None
    }
    fn space_after(&self) -> Option<f64> {
        None
    }
    fn space_before(&self) -> Option<f64> {
        None
    }
    fn span_column_inside_gutter(&self) -> Option<f64> {
        None
    }
    fn span_column_outside_gutter(&self) -> Option<f64> {
        None
    }
    fn span_column_type(&self) -> Option<SpanColumnTypeOptions> {
        None
    }
    fn span_split_column_count(&self) -> Option<i32> {
        None
    }
    fn start_paragraph(&self) -> Option<StartParagraph> {
        None
    }
    fn strike_through_gap_overprint(&self) -> Option<bool> {
        None
    }
    fn strike_through_gap_tint(&self) -> Option<f64> {
        None
    }
    fn strike_through_gap_offset(&self) -> Option<f64> {
        None
    }
    fn strike_through_overprint(&self) -> Option<bool> {
        None
    }
    fn strike_through_tint(&self) -> Option<f64> {
        None
    }
    fn strike_through_weight(&self) -> Option<f64> {
        None
    }
    fn strike_thru(&self) -> Option<bool> {
        None
    }
    fn stroke_alignment(&self) -> Option<TextStrokeAlign> {
        None
    }
    fn stroke_color(&self) -> Option<String> {
        None
    }
    fn stroke_tint(&self) -> Option<String> {
        None
    }
    fn stroke_weight(&self) -> Option<String> {
        None
    }
    fn tatechuyoko(&self) -> Option<bool> {
        None
    }
    fn tatechuyoko_x_offset(&self) -> Option<f64> {
        None
    }
    fn tatechuyoko_y_offset(&self) -> Option<f64> {
        None
    }
    fn tracking(&self) -> Option<f64> {
        None
    }
    fn trailing_aki(&self) -> Option<f64> {
        None
    }
    fn treat_ideographic_space_as_space(&self) -> Option<bool> {
        None
    }
    fn tsume(&self) -> Option<f64> {
        None
    }
    fn underline(&self) -> Option<bool> {
        None
    }
    fn underline_gap_overprint(&self) -> Option<bool> {
        None
    }
    fn underline_gap_tint(&self) -> Option<f64> {
        None
    }
    fn underline_gap_offset(&self) -> Option<bool> {
        None
    }
    fn underline_overprint(&self) -> Option<bool> {
        None
    }
    fn underline_tint(&self) -> Option<f64> {
        None
    }
    fn underline_weight(&self) -> Option<f64> {
        None
    }
    fn vertical_scale(&self) -> Option<f64> {
        None
    }
    fn warichu(&self) -> Option<bool> {
        None
    }
    fn warichu_alignment(&self) -> Option<WarichuAlignment> {
        None
    }
    fn warichu_chars_after_break(&self) -> Option<i16> {
        None
    }
    fn warichu_chars_before_break(&self) -> Option<i16> {
        None
    }
    fn warichu_line_spacing(&self) -> Option<f64> {
        None
    }
    fn warichu_lines(&self) -> Option<i16> {
        None
    }
    fn warichu_size(&self) -> Option<f64> {
        None
    }
    fn x_offset_diacritic(&self) -> Option<f64> {
        None
    }
    fn y_offset_diacritic(&self) -> Option<f64> {
        None
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum Capitalization {
    Normal,
    SmallCaps,
    AllCaps,
    CapToSmallCap,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum CharacterAlignment {
    AlignBaseline,
    AlignEmTop,
    AlignEmCenter,
    AlignEmBottom,
    AlignICFTop,
    AlignICFBottom,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum CharacterDirection {
    DefaultDirection,
    LeftToRightDirection,
    RightToLeftDirection,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum DiacriticPosition {
    DefaultPosition,
    LoosePosition,
    MediumPosition,
    TightPosition,
    OpenTypePosition,
    OpentypePositionFromBaseline,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum DigitsType {
    DefaultDigits,
    ArabicDigits,
    HindiDigits,
    FarsiDigits,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum OutlineJoin {
    MiterEndJoin,
    RoundEndJoin,
    BevelEndJoin,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum AlternateGlyphForms {
    None,
    TraditionalForm,
    ExpertForm,
    JIS78Form,
    JIS83Form,
    JIS04Form,
    JIS90Form,
    MonospaceHalfWidthForm,
    ThirdWidthForm,
    QuarterWidthForm,
    NLCWidthForm,
    ProportionalWidthForm,
    FullWidthForm,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum GotoNextX {}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum GridAlignment {
    None,
    AlignBaseLine,
    AlignEmTop,
    AlignEmCenter,
    AlignEmBottom,
    AlignICFTop,
    AlignICFBottom,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum Justification {
    LeftAlign,
    CenterAlign,
    RightAlign,
    LeftJustified,
    RightJustified,
    CenterJustified,
    FullyJustified,
    ToBindingSide,
    FromBindingSide,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum Kashidas {
    DefaultKashidas,
    KashidasOff,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum KentenAlignment {
    AlignKentenCenter,
    CharacterInput,
    ShiftJIS,
    JIS,
    Kuten,
    Unicode,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum KentenCharacter {
    None,
    KentenSesameDot,
    SesameDot,
    KentenBlackCircle,
    KentenWhiteCircle,
    KentenBlackTriangle,
    KentenWhiteTriangle,
    KentenBullseye,
    KentenFishseye,
    KentenSmallWhiteCircle,
    Custom,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum AdornmentOverprint {
    Auto,
    OverprintOn,
    OverprintOff,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RubyKentenPosition {
    AboveRight,
    BelowLeft,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum KinsokuHangTypes {
    None,
    KinsokuHangRegular,
    KinsokuHangForce,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum KinsokuType {
    KinsokuPushInFirst,
    KinsokuPushOutFirst,
    KinsokuPushOutOnly,
    KinsokuPrioritizeAdjustmentAmount,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum LeadingModel {
    LeadingModelRoman,
    LeadingModelAkiBelow,
    LeadingModelAkiAbove,
    LeadingModelCenter,
    LeadingModelCenterDown,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ListAlignment {
    LeftAlign,
    CenterAlign,
    RightAlign,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum OTFFigureStyle {
    TabularOldStyle,
    TablularLining,
    ProportionalOldStyle,
    ProportinalLining,
    Default,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum PageNumberType {
    AutoPageNumber,
    NextPageNumber,
    PreviousPageNumber,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ParagraphDirection {
    LeftToRightDirection,
    RightToLeftDirection,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ParagraphJustification {
    DefaultJustification,
    ArabicJustification,
    NaskhJustification,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum Position {
    Normal,
    Superscript,
    Subscript,
    OTSuperscript,
    OTSubscript,
    OTNumerator,
    OTDenominator,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum PositionalForms {
    None,
    Calculate,
    Initial,
    Medial,
    Final,
    Isolated,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RubyAlignments {
    RubyLeft,
    RubyCenter,
    RubyRight,
    FullJustify,
    RubyJIS,
    RubyEqualAki,
    Ruby1Aki,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RubyOverhang {
    None,
    RubyOverhangOneRuby,
    RubyOverhangHalfRuby,
    RubyOverhangOneChar,
    RubyOverhangHalfChar,
    RubyOverhangNoLimit,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RubyParentSpacing {
    RubyParentNoAdjustment,
    RubyParentBothSides,
    RubyParent121Aki,
    RubyParentEqualAki,
    RubyParentFullJustify,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RubyTypes {
    GroupRuby,
    PerCharacterRuby,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RuleWidth {
    TextWidth,
    ColumnWidth,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum SingleWordJustification {
    LeftAlign,
    CenterAlign,
    RightAlign,
    FullyJustified,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum SpanColumnTypeOptions {
    SingleColumn,
    SpanColumns,
    SplitColumns,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum StartParagraph {
    Anywhere,
    NextColumn,
    NextFrame,
    NextPage,
    NextOddPage,
    NextEvenPage,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum TextStrokeAlign {
    CenterAlignment,
    OutsideAlignment,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum WarichuAlignment {
    Auto,
    LeftAlign,
    CenterAlign,
    RightAlign,
    FullyJustified,
    LeftJustified,
    CenterJustified,
    RightJustified,
}

#[macro_export]
macro_rules! common_text_properties_struct {
    ($StructName:ident { $($manual_fields:tt)* }) => {
        #[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
        #[serde(rename_all = "PascalCase")]
        pub struct $StructName {
            $($manual_fields)*
            applied_character_style: Option<String>,
            applied_conditions: Option<String>,
            applied_language: Option<String>,
            applied_paragraph_style: Option<String>,
            auto_leading: Option<f64>,
            auto_tcy: Option<i16>,
            auto_tcy_include_roman: Option<bool>,
            baseline_shift: Option<f64>,
            bullets_alignment: Option<String>,
            bullets_and_numbering_list_type: Option<String>,
            bullets_text_after: Option<String>,
            bunri_kinshi: Option<bool>,
            captilization: Option<Capitalization>,
            character_alignment: Option<CharacterAlignment>,
            character_direction: Option<CharacterDirection>,
            character_rotation: Option<f64>,
            cjk_grid_tracking: Option<bool>,
            composer: Option<String>,
            desired_word_spacing: Option<f64>,
            diacritic_position: Option<DiacriticPosition>,
            digits_type: Option<DigitsType>,
            drop_cap_characters: Option<i16>,
            drop_cap_lines: Option<i16>,
            drop_cap_detail: Option<i32>,
            end_join: Option<OutlineJoin>,
            fill_color: Option<String>,
            fill_tint: Option<f64>,
            first_line_indent: Option<f64>,
            font_style: Option<String>,
            glyph_form: Option<AlternateGlyphForms>,
            goto_next_x: Option<GotoNextX>,
            gradient_fill_angle: Option<f64>,
            gradient_fill_length: Option<f64>,
            #[serde(default, deserialize_with="deserialize_space_seperated_opt_vec")]
            gradient_fill_start: Option<Vec<f64>>,
            gradient_stroke_angle: Option<f64>,
            gradient_stroke_length: Option<f64>,
            #[serde(default, deserialize_with="deserialize_space_seperated_opt_vec")]
            gradient_stroke_start: Option<Vec<f64>>,
            grid_align_first_line_only: Option<bool>,
            grid_alignment: Option<GridAlignment>,
            grid_gyoudori: Option<i16>,
            horizontal_scale: Option<f64>,
            hyphen_weight: Option<i16>,
            hyphenate_across_columns: Option<bool>,
            hyphenate_after_first: Option<i16>,
            hyphenate_before_last: Option<i16>,
            hyphenate_capitalized_words: Option<bool>,
            hyphenate_ladder_limit: Option<i16>,
            hyphenate_last_word: Option<bool>,
            hyphenate_words_longer_than: Option<i16>,
            hyphenation: Option<bool>,
            hyphenation_zone: Option<f64>,
            ignore_edge_alignment: Option<bool>,
            jidori: Option<i16>,
            justification: Option<Justification>,
            kashidas: Option<Kashidas>,
            keep_all_lines_together: Option<bool>,
            keep_first_lines: Option<i16>,
            keep_last_lines: Option<i16>,
            keep_lines_together: Option<bool>,
            keep_rule_above_in_frame: Option<bool>,
            keep_with_next: Option<bool>,
            keep_with_previous: Option<bool>,
            kenten_alignment: Option<KentenAlignment>,
            kenten_custom_character: Option<String>,
            kenten_font_size: Option<f64>,
            kenten_kind: Option<KentenCharacter>,
            kenten_overprint_fill: Option<AdornmentOverprint>,
            kenten_overprint_stroke: Option<AdornmentOverprint>,
            kenten_placement: Option<f64>,
            kenten_position: Option<RubyKentenPosition>,
            kenten_stroke_tint: Option<f64>,
            kenten_tint: Option<f64>,
            kenten_weight: Option<f64>,
            kenten_x_scale: Option<f64>,
            kenten_y_scale: Option<f64>,
            kerning_method: Option<String>,
            kerning_value: Option<f64>,
            keyboard_direction: Option<CharacterDirection>,
            kinsoku_hang_type: Option<KinsokuHangTypes>,
            kinsoku_type: Option<KinsokuType>,
            last_line_indent: Option<f64>,
            leading_aki: Option<f64>,
            leading_model: Option<LeadingModel>,
            left_indent: Option<f64>,
            ligatures: Option<bool>,
            link_resource_id: Option<i32>,
            maximum_glyph_scaling: Option<f64>,
            maximum_letter_spacing: Option<f64>,
            maximum_word_spacing: Option<f64>,
            minimum_glyph_scaling: Option<f64>,
            minimum_letter_spacing: Option<f64>,
            minimum_word_spacing: Option<f64>,
            miter_limit: Option<f64>,
            no_break: Option<bool>,
            numbering_alignment: Option<ListAlignment>,
            numbering_apply_restart_policy: Option<bool>,
            numbering_continue: Option<bool>,
            numbering_expression: Option<String>,
            numbering_level: Option<i32>,
            numbering_start_at: Option<i32>,
            otf_contextual_alternate: Option<bool>,
            otf_discretionary_ligature: Option<bool>,
            otf_figure_style: Option<OTFFigureStyle>,
            otf_fraction: Option<bool>,
            otf_hv_kana: Option<bool>,
            otf_historical: Option<bool>,
            otf_justification_alternate: Option<bool>,
            otf_locale: Option<bool>,
            otf_mark: Option<bool>,
            otf_ordinal: Option<bool>,
            otf_overlap_swash: Option<bool>,
            otf_proportional_metrics: Option<bool>,
            otf_roman_italics: Option<bool>,
            otf_slashed_zero: Option<bool>,
            otf_stretched_alternate: Option<bool>,
            otf_stylistic_alternate: Option<bool>,
            otf_stylistic_sets: Option<i32>,
            otf_swash: Option<bool>,
            otf_titling: Option<bool>,
            otf_overprint_fill: Option<bool>,
            otf_overprint_stroke: Option<bool>,
            page_number_type: Option<PageNumberType>,
            paragraph_direction: Option<ParagraphDirection>,
            paragraph_gyoudori: Option<bool>,
            paragraph_justification: Option<ParagraphJustification>,
            point_size: Option<f64>,
            position: Option<Position>,
            positional_form: Option<PositionalForms>,
            rensuuji: Option<bool>,
            right_indent: Option<f64>,
            rotate_single_byte_character: Option<bool>,
            ruby_alignment: Option<RubyAlignments>,
            ruby_auto_align: Option<bool>,
            ruby_auto_scaling: Option<bool>,
            ruby_auto_tcy_auto_scale: Option<bool>,
            ruby_auto_tcy_digits: Option<i16>,
            ruby_auto_tcy_include_roman: Option<bool>,
            ruby_flag: Option<i32>,
            ruby_font_size: Option<f64>,
            ruby_open_type: Option<bool>,
            ruby_overhang: Option<bool>,
            ruby_overprint_fill: Option<AdornmentOverprint>,
            ruby_overprint_stroke: Option<AdornmentOverprint>,
            ruby_parent_overhang_amount: Option<RubyOverhang>,
            ruby_parent_scaling_percent: Option<f64>,
            ruby_parent_spacing: Option<RubyParentSpacing>,
            ruby_position: Option<RubyKentenPosition>,
            ruby_string: Option<String>,
            ruby_stroke_tint: Option<f64>,
            ruby_tint: Option<f64>,
            ruby_type: Option<RubyTypes>,
            ruby_weight: Option<f64>,
            ruby_x_offset: Option<f64>,
            ruby_x_scale: Option<f64>,
            ruby_y_offset: Option<f64>,
            ruby_y_scale: Option<f64>,
            rule_above: Option<bool>,
            rule_above_gap_overprint: Option<bool>,
            rule_above_gap_tint: Option<f64>,
            rule_above_left_indent: Option<f64>,
            rule_above_weight: Option<f64>,
            rule_above_offset: Option<f64>,
            rule_above_overprint: Option<bool>,
            rule_above_right_indent: Option<f64>,
            rule_above_tint: Option<f64>,
            rule_above_width: Option<RuleWidth>,
            rule_below_gap_overprint: Option<bool>,
            rule_below_gap_tint: Option<f64>,
            rule_below_left_indent: Option<f64>,
            rule_below_weight: Option<f64>,
            rule_below_offset: Option<f64>,
            rule_below_overprint: Option<bool>,
            rule_below_right_indent: Option<f64>,
            rule_below_tint: Option<f64>,
            rule_below_width: Option<RuleWidth>,
            scale_affects_line_height: Option<bool>,
            shatai_adjust_rotation: Option<bool>,
            shatai_adjust_tsume: Option<bool>,
            shatai_adjust_angle: Option<f64>,
            shatai_magnification: Option<f64>,
            single_word_justification: Option<SingleWordJustification>,
            skew: Option<f64>,
            space_after: Option<f64>,
            space_before: Option<f64>,
            span_column_inside_gutter: Option<f64>,
            span_column_outside_gutter: Option<f64>,
            span_column_type: Option<SpanColumnTypeOptions>,
            span_split_column_count: Option<i32>,
            start_paragraph: Option<StartParagraph>,
            strike_through_gap_overprint: Option<bool>,
            strike_through_gap_tint: Option<f64>,
            strike_through_gap_offset: Option<f64>,
            strike_through_overprint: Option<bool>,
            strike_through_tint: Option<f64>,
            strike_through_weight: Option<f64>,
            strike_thru: Option<bool>,
            stroke_alignment: Option<TextStrokeAlign>,
            stroke_color: Option<String>,
            stroke_tint: Option<String>,
            stroke_weight: Option<String>,
            tatechuyoko: Option<bool>,
            tatechuyoko_x_offset: Option<f64>,
            tatechuyoko_y_offset: Option<f64>,
            tracking: Option<f64>,
            trailing_aki: Option<f64>,
            treat_ideographic_space_as_space: Option<bool>,
            tsume: Option<f64>,
            underline: Option<bool>,
            underline_gap_overprint: Option<bool>,
            underline_gap_tint: Option<f64>,
            underline_gap_offset: Option<bool>,
            underline_overprint: Option<bool>,
            underline_tint: Option<f64>,
            underline_weight: Option<f64>,
            vertical_scale: Option<f64>,
            warichu: Option<bool>,
            warichu_alignment: Option<WarichuAlignment>,
            warichu_chars_after_break: Option<i16>,
            warichu_chars_before_break: Option<i16>,
            warichu_line_spacing: Option<f64>,
            warichu_lines: Option<i16>,
            warichu_size: Option<f64>,
            x_offset_diacritic: Option<f64>,
            y_offset_diacritic: Option<f64>,
        }

        impl CommonTextPropertiesAttributes for $StructName {
            fn applied_character_style(&self) -> Option<String> {self.applied_character_style.clone()}
            fn applied_conditions(&self) -> Option<String> {self.applied_conditions.clone()}
            fn applied_language(&self) -> Option<String> {self.applied_language.clone()}
            fn applied_paragraph_style(&self) -> Option<String> {self.applied_paragraph_style.clone()}
            fn auto_leading(&self) -> Option<f64> {self.auto_leading.clone()}
            fn auto_tcy(&self) -> Option<i16> {self.auto_tcy.clone()}
            fn auto_tcy_include_roman(&self) -> Option<bool> {self.auto_tcy_include_roman.clone()}
            fn baseline_shift(&self) -> Option<f64> {self.baseline_shift.clone()}
            fn bullets_alignment(&self) -> Option<String> {self.bullets_alignment.clone()}
            fn bullets_and_numbering_list_type(&self) -> Option<String> {self.bullets_and_numbering_list_type.clone()}
            fn bullets_text_after(&self) -> Option<String> {self.bullets_text_after.clone()}
            fn bunri_kinshi(&self) -> Option<bool> {self.bunri_kinshi.clone()}
            fn captilization(&self) -> Option<Capitalization> {self.captilization.clone()}
            fn character_alignment(&self) -> Option<CharacterAlignment> {self.character_alignment.clone()}
            fn character_direction(&self) -> Option<CharacterDirection> {self.character_direction.clone()}
            fn character_rotation(&self) -> Option<f64> {self.character_rotation.clone()}
            fn cjk_grid_tracking(&self) -> Option<bool> {self.cjk_grid_tracking.clone()}
            fn composer(&self) -> Option<String> {self.composer.clone()}
            fn desired_word_spacing(&self) -> Option<f64> {self.desired_word_spacing.clone()}
            fn diacritic_position(&self) -> Option<DiacriticPosition> {self.diacritic_position.clone()}
            fn digits_type(&self) -> Option<DigitsType> {self.digits_type.clone()}
            fn drop_cap_characters(&self) -> Option<i16> {self.drop_cap_characters.clone()}
            fn drop_cap_lines(&self) -> Option<i16> {self.drop_cap_lines.clone()}
            fn drop_cap_detail(&self) -> Option<i32> {self.drop_cap_detail.clone()}
            fn end_join(&self) -> Option<OutlineJoin> {self.end_join.clone()}
            fn fill_color(&self) -> Option<String> {self.fill_color.clone()}
            fn fill_tint(&self) -> Option<f64> {self.fill_tint.clone()}
            fn first_line_indent(&self) -> Option<f64> {self.first_line_indent.clone()}
            fn font_style(&self) -> Option<String> {self.font_style.clone()}
            fn glyph_form(&self) -> Option<AlternateGlyphForms> {self.glyph_form.clone()}
            fn goto_next_x(&self) -> Option<GotoNextX> {self.goto_next_x.clone()}
            fn gradient_fill_angle(&self) -> Option<f64> {self.gradient_fill_angle.clone()}
            fn gradient_fill_length(&self) -> Option<f64> {self.gradient_fill_length.clone()}
            fn gradient_fill_start(&self) -> Option<Vec<f64>> {self.gradient_fill_start.clone()}
            fn gradient_stroke_angle(&self) -> Option<f64> {self.gradient_stroke_angle.clone()}
            fn gradient_stroke_length(&self) -> Option<f64> {self.gradient_stroke_length.clone()}
            fn gradient_stroke_start(&self) -> Option<Vec<f64>> {self.gradient_stroke_start.clone()}
            fn grid_align_first_line_only(&self) -> Option<bool> {self.grid_align_first_line_only.clone()}
            fn grid_alignment(&self) -> Option<GridAlignment> {self.grid_alignment.clone()}
            fn grid_gyoudori(&self) -> Option<i16> {self.grid_gyoudori.clone()}
            fn horizontal_scale(&self) -> Option<f64> {self.horizontal_scale.clone()}
            fn hyphen_weight(&self) -> Option<i16> {self.hyphen_weight.clone()}
            fn hyphenate_across_columns(&self) -> Option<bool> {self.hyphenate_across_columns.clone()}
            fn hyphenate_after_first(&self) -> Option<i16> {self.hyphenate_after_first.clone()}
            fn hyphenate_before_last(&self) -> Option<i16> {self.hyphenate_before_last.clone()}
            fn hyphenate_capitalized_words(&self) -> Option<bool> {self.hyphenate_capitalized_words.clone()}
            fn hyphenate_ladder_limit(&self) -> Option<i16> {self.hyphenate_ladder_limit.clone()}
            fn hyphenate_last_word(&self) -> Option<bool> {self.hyphenate_last_word.clone()}
            fn hyphenate_words_longer_than(&self) -> Option<i16> {self.hyphenate_words_longer_than.clone()}
            fn hyphenation(&self) -> Option<bool> {self.hyphenation.clone()}
            fn hyphenation_zone(&self) -> Option<f64> {self.hyphenation_zone.clone()}
            fn ignore_edge_alignment(&self) -> Option<bool> {self.ignore_edge_alignment.clone()}
            fn jidori(&self) -> Option<i16> {self.jidori.clone()}
            fn justification(&self) -> Option<Justification> {self.justification.clone()}
            fn kashidas(&self) -> Option<Kashidas> {self.kashidas.clone()}
            fn keep_all_lines_together(&self) -> Option<bool> {self.keep_all_lines_together.clone()}
            fn keep_first_lines(&self) -> Option<i16> {self.keep_first_lines.clone()}
            fn keep_last_lines(&self) -> Option<i16> {self.keep_last_lines.clone()}
            fn keep_lines_together(&self) -> Option<bool> {self.keep_lines_together.clone()}
            fn keep_rule_above_in_frame(&self) -> Option<bool> {self.keep_rule_above_in_frame.clone()}
            fn keep_with_next(&self) -> Option<bool> {self.keep_with_next.clone()}
            fn keep_with_previous(&self) -> Option<bool> {self.keep_with_previous.clone()}
            fn kenten_alignment(&self) -> Option<KentenAlignment> {self.kenten_alignment.clone()}
            fn kenten_custom_character(&self) -> Option<String> {self.kenten_custom_character.clone()}
            fn kenten_font_size(&self) -> Option<f64> {self.kenten_font_size.clone()}
            fn kenten_kind(&self) -> Option<KentenCharacter> {self.kenten_kind.clone()}
            fn kenten_overprint_fill(&self) -> Option<AdornmentOverprint> {self.kenten_overprint_fill.clone()}
            fn kenten_overprint_stroke(&self) -> Option<AdornmentOverprint> {self.kenten_overprint_stroke.clone()}
            fn kenten_placement(&self) -> Option<f64> {self.kenten_placement.clone()}
            fn kenten_position(&self) -> Option<RubyKentenPosition> {self.kenten_position.clone()}
            fn kenten_stroke_tint(&self) -> Option<f64> {self.kenten_stroke_tint.clone()}
            fn kenten_tint(&self) -> Option<f64> {self.kenten_tint.clone()}
            fn kenten_weight(&self) -> Option<f64> {self.kenten_weight.clone()}
            fn kenten_x_scale(&self) -> Option<f64> {self.kenten_x_scale.clone()}
            fn kenten_y_scale(&self) -> Option<f64> {self.kenten_y_scale.clone()}
            fn kerning_method(&self) -> Option<String> {self.kerning_method.clone()}
            fn kerning_value(&self) -> Option<f64> {self.kerning_value.clone()}
            fn keyboard_direction(&self) -> Option<CharacterDirection> {self.keyboard_direction.clone()}
            fn kinsoku_hang_type(&self) -> Option<KinsokuHangTypes> {self.kinsoku_hang_type.clone()}
            fn kinsoku_type(&self) -> Option<KinsokuType> {self.kinsoku_type.clone()}
            fn last_line_indent(&self) -> Option<f64> {self.last_line_indent.clone()}
            fn leading_aki(&self) -> Option<f64> {self.leading_aki.clone()}
            fn leading_model(&self) -> Option<LeadingModel> {self.leading_model.clone()}
            fn left_indent(&self) -> Option<f64> {self.left_indent.clone()}
            fn ligatures(&self) -> Option<bool> {self.ligatures.clone()}
            fn link_resource_id(&self) -> Option<i32> {self.link_resource_id.clone()}
            fn maximum_glyph_scaling(&self) -> Option<f64> {self.maximum_glyph_scaling.clone()}
            fn maximum_letter_spacing(&self) -> Option<f64> {self.maximum_letter_spacing.clone()}
            fn maximum_word_spacing(&self) -> Option<f64> {self.maximum_word_spacing.clone()}
            fn minimum_glyph_scaling(&self) -> Option<f64> {self.minimum_glyph_scaling.clone()}
            fn minimum_letter_spacing(&self) -> Option<f64> {self.minimum_letter_spacing.clone()}
            fn minimum_word_spacing(&self) -> Option<f64> {self.minimum_word_spacing.clone()}
            fn miter_limit(&self) -> Option<f64> {self.miter_limit.clone()}
            fn no_break(&self) -> Option<bool> {self.no_break.clone()}
            fn numbering_alignment(&self) -> Option<ListAlignment> {self.numbering_alignment.clone()}
            fn numbering_apply_restart_policy(&self) -> Option<bool> {self.numbering_apply_restart_policy.clone()}
            fn numbering_continue(&self) -> Option<bool> {self.numbering_continue.clone()}
            fn numbering_expression(&self) -> Option<String> {self.numbering_expression.clone()}
            fn numbering_level(&self) -> Option<i32> {self.numbering_level.clone()}
            fn numbering_start_at(&self) -> Option<i32> {self.numbering_start_at.clone()}
            fn otf_contextual_alternate(&self) -> Option<bool> {self.otf_contextual_alternate.clone()}
            fn otf_discretionary_ligature(&self) -> Option<bool> {self.otf_discretionary_ligature.clone()}
            fn otf_figure_style(&self) -> Option<OTFFigureStyle> {self.otf_figure_style.clone()}
            fn otf_fraction(&self) -> Option<bool> {self.otf_fraction.clone()}
            fn otf_hv_kana(&self) -> Option<bool> {self.otf_hv_kana.clone()}
            fn otf_historical(&self) -> Option<bool> {self.otf_historical.clone()}
            fn otf_justification_alternate(&self) -> Option<bool> {self.otf_justification_alternate.clone()}
            fn otf_locale(&self) -> Option<bool> {self.otf_locale.clone()}
            fn otf_mark(&self) -> Option<bool> {self.otf_mark.clone()}
            fn otf_ordinal(&self) -> Option<bool> {self.otf_ordinal.clone()}
            fn otf_overlap_swash(&self) -> Option<bool> {self.otf_overlap_swash.clone()}
            fn otf_proportional_metrics(&self) -> Option<bool> {self.otf_proportional_metrics.clone()}
            fn otf_roman_italics(&self) -> Option<bool> {self.otf_roman_italics.clone()}
            fn otf_slashed_zero(&self) -> Option<bool> {self.otf_slashed_zero.clone()}
            fn otf_stretched_alternate(&self) -> Option<bool> {self.otf_stretched_alternate.clone()}
            fn otf_stylistic_alternate(&self) -> Option<bool> {self.otf_stylistic_alternate.clone()}
            fn otf_stylistic_sets(&self) -> Option<i32> {self.otf_stylistic_sets.clone()}
            fn otf_swash(&self) -> Option<bool> {self.otf_swash.clone()}
            fn otf_titling(&self) -> Option<bool> {self.otf_titling.clone()}
            fn otf_overprint_fill(&self) -> Option<bool> {self.otf_overprint_fill.clone()}
            fn otf_overprint_stroke(&self) -> Option<bool> {self.otf_overprint_stroke.clone()}
            fn page_number_type(&self) -> Option<PageNumberType> {self.page_number_type.clone()}
            fn paragraph_direction(&self) -> Option<ParagraphDirection> {self.paragraph_direction.clone()}
            fn paragraph_gyoudori(&self) -> Option<bool> {self.paragraph_gyoudori.clone()}
            fn paragraph_justification(&self) -> Option<ParagraphJustification> {self.paragraph_justification.clone()}
            fn point_size(&self) -> Option<f64> {self.point_size.clone()}
            fn position(&self) -> Option<Position> {self.position.clone()}
            fn positional_form(&self) -> Option<PositionalForms> {self.positional_form.clone()}
            fn rensuuji(&self) -> Option<bool> {self.rensuuji.clone()}
            fn right_indent(&self) -> Option<f64> {self.right_indent.clone()}
            fn rotate_single_byte_character(&self) -> Option<bool> {self.rotate_single_byte_character.clone()}
            fn ruby_alignment(&self) -> Option<RubyAlignments> {self.ruby_alignment.clone()}
            fn ruby_auto_align(&self) -> Option<bool> {self.ruby_auto_align.clone()}
            fn ruby_auto_scaling(&self) -> Option<bool> {self.ruby_auto_scaling.clone()}
            fn ruby_auto_tcy_auto_scale(&self) -> Option<bool> {self.ruby_auto_tcy_auto_scale.clone()}
            fn ruby_auto_tcy_digits(&self) -> Option<i16> {self.ruby_auto_tcy_digits.clone()}
            fn ruby_auto_tcy_include_roman(&self) -> Option<bool> {self.ruby_auto_tcy_include_roman.clone()}
            fn ruby_flag(&self) -> Option<i32> {self.ruby_flag.clone()}
            fn ruby_font_size(&self) -> Option<f64> {self.ruby_font_size.clone()}
            fn ruby_open_type(&self) -> Option<bool> {self.ruby_open_type.clone()}
            fn ruby_overhang(&self) -> Option<bool> {self.ruby_overhang.clone()}
            fn ruby_overprint_fill(&self) -> Option<AdornmentOverprint> {self.ruby_overprint_fill.clone()}
            fn ruby_overprint_stroke(&self) -> Option<AdornmentOverprint> {self.ruby_overprint_stroke.clone()}
            fn ruby_parent_overhang_amount(&self) -> Option<RubyOverhang> {self.ruby_parent_overhang_amount.clone()}
            fn ruby_parent_scaling_percent(&self) -> Option<f64> {self.ruby_parent_scaling_percent.clone()}
            fn ruby_parent_spacing(&self) -> Option<RubyParentSpacing> {self.ruby_parent_spacing.clone()}
            fn ruby_position(&self) -> Option<RubyKentenPosition> {self.ruby_position.clone()}
            fn ruby_string(&self) -> Option<String> {self.ruby_string.clone()}
            fn ruby_stroke_tint(&self) -> Option<f64> {self.ruby_stroke_tint.clone()}
            fn ruby_tint(&self) -> Option<f64> {self.ruby_tint.clone()}
            fn ruby_type(&self) -> Option<RubyTypes> {self.ruby_type.clone()}
            fn ruby_weight(&self) -> Option<f64> {self.ruby_weight.clone()}
            fn ruby_x_offset(&self) -> Option<f64> {self.ruby_x_offset.clone()}
            fn ruby_x_scale(&self) -> Option<f64> {self.ruby_x_scale.clone()}
            fn ruby_y_offset(&self) -> Option<f64> {self.ruby_y_offset.clone()}
            fn ruby_y_scale(&self) -> Option<f64> {self.ruby_y_scale.clone()}
            fn rule_above(&self) -> Option<bool> {self.rule_above.clone()}
            fn rule_above_gap_overprint(&self) -> Option<bool> {self.rule_above_gap_overprint.clone()}
            fn rule_above_gap_tint(&self) -> Option<f64> {self.rule_above_gap_tint.clone()}
            fn rule_above_left_indent(&self) -> Option<f64> {self.rule_above_left_indent.clone()}
            fn rule_above_weight(&self) -> Option<f64> {self.rule_above_weight.clone()}
            fn rule_above_offset(&self) -> Option<f64> {self.rule_above_offset.clone()}
            fn rule_above_overprint(&self) -> Option<bool> {self.rule_above_overprint.clone()}
            fn rule_above_right_indent(&self) -> Option<f64> {self.rule_above_right_indent.clone()}
            fn rule_above_tint(&self) -> Option<f64> {self.rule_above_tint.clone()}
            fn rule_above_width(&self) -> Option<RuleWidth> {self.rule_above_width.clone()}
            fn rule_below_gap_overprint(&self) -> Option<bool> {self.rule_below_gap_overprint.clone()}
            fn rule_below_gap_tint(&self) -> Option<f64> {self.rule_below_gap_tint.clone()}
            fn rule_below_left_indent(&self) -> Option<f64> {self.rule_below_left_indent.clone()}
            fn rule_below_weight(&self) -> Option<f64> {self.rule_below_weight.clone()}
            fn rule_below_offset(&self) -> Option<f64> {self.rule_below_offset.clone()}
            fn rule_below_overprint(&self) -> Option<bool> {self.rule_below_overprint.clone()}
            fn rule_below_right_indent(&self) -> Option<f64> {self.rule_below_right_indent.clone()}
            fn rule_below_tint(&self) -> Option<f64> {self.rule_below_tint.clone()}
            fn rule_below_width(&self) -> Option<RuleWidth> {self.rule_below_width.clone()}
            fn scale_affects_line_height(&self) -> Option<bool> {self.scale_affects_line_height.clone()}
            fn shatai_adjust_rotation(&self) -> Option<bool> {self.shatai_adjust_rotation.clone()}
            fn shatai_adjust_tsume(&self) -> Option<bool> {self.shatai_adjust_tsume.clone()}
            fn shatai_adjust_angle(&self) -> Option<f64> {self.shatai_adjust_angle.clone()}
            fn shatai_magnification(&self) -> Option<f64> {self.shatai_magnification.clone()}
            fn single_word_justification(&self) -> Option<SingleWordJustification> {self.single_word_justification.clone()}
            fn skew(&self) -> Option<f64> {self.skew.clone()}
            fn space_after(&self) -> Option<f64> {self.space_after.clone()}
            fn space_before(&self) -> Option<f64> {self.space_before.clone()}
            fn span_column_inside_gutter(&self) -> Option<f64> {self.span_column_inside_gutter.clone()}
            fn span_column_outside_gutter(&self) -> Option<f64> {self.span_column_outside_gutter.clone()}
            fn span_column_type(&self) -> Option<SpanColumnTypeOptions> {self.span_column_type.clone()}
            fn span_split_column_count(&self) -> Option<i32> {self.span_split_column_count.clone()}
            fn start_paragraph(&self) -> Option<StartParagraph> {self.start_paragraph.clone()}
            fn strike_through_gap_overprint(&self) -> Option<bool> {self.strike_through_gap_overprint.clone()}
            fn strike_through_gap_tint(&self) -> Option<f64> {self.strike_through_gap_tint.clone()}
            fn strike_through_gap_offset(&self) -> Option<f64> {self.strike_through_gap_offset.clone()}
            fn strike_through_overprint(&self) -> Option<bool> {self.strike_through_overprint.clone()}
            fn strike_through_tint(&self) -> Option<f64> {self.strike_through_tint.clone()}
            fn strike_through_weight(&self) -> Option<f64> {self.strike_through_weight.clone()}
            fn strike_thru(&self) -> Option<bool> {self.strike_thru.clone()}
            fn stroke_alignment(&self) -> Option<TextStrokeAlign> {self.stroke_alignment.clone()}
            fn stroke_color(&self) -> Option<String> {self.stroke_color.clone()}
            fn stroke_tint(&self) -> Option<String> {self.stroke_tint.clone()}
            fn stroke_weight(&self) -> Option<String> {self.stroke_weight.clone()}
            fn tatechuyoko(&self) -> Option<bool> {self.tatechuyoko.clone()}
            fn tatechuyoko_x_offset(&self) -> Option<f64> {self.tatechuyoko_x_offset.clone()}
            fn tatechuyoko_y_offset(&self) -> Option<f64> {self.tatechuyoko_y_offset.clone()}
            fn tracking(&self) -> Option<f64> {self.tracking.clone()}
            fn trailing_aki(&self) -> Option<f64> {self.trailing_aki.clone()}
            fn treat_ideographic_space_as_space(&self) -> Option<bool> {self.treat_ideographic_space_as_space.clone()}
            fn tsume(&self) -> Option<f64> {self.tsume.clone()}
            fn underline(&self) -> Option<bool> {self.underline.clone()}
            fn underline_gap_overprint(&self) -> Option<bool> {self.underline_gap_overprint.clone()}
            fn underline_gap_tint(&self) -> Option<f64> {self.underline_gap_tint.clone()}
            fn underline_gap_offset(&self) -> Option<bool> {self.underline_gap_offset.clone()}
            fn underline_overprint(&self) -> Option<bool> {self.underline_overprint.clone()}
            fn underline_tint(&self) -> Option<f64> {self.underline_tint.clone()}
            fn underline_weight(&self) -> Option<f64> {self.underline_weight.clone()}
            fn vertical_scale(&self) -> Option<f64> {self.vertical_scale.clone()}
            fn warichu(&self) -> Option<bool> {self.warichu.clone()}
            fn warichu_alignment(&self) -> Option<WarichuAlignment> {self.warichu_alignment.clone()}
            fn warichu_chars_after_break(&self) -> Option<i16> {self.warichu_chars_after_break.clone()}
            fn warichu_chars_before_break(&self) -> Option<i16> {self.warichu_chars_before_break.clone()}
            fn warichu_line_spacing(&self) -> Option<f64> {self.warichu_line_spacing.clone()}
            fn warichu_lines(&self) -> Option<i16> {self.warichu_lines.clone()}
            fn warichu_size(&self) -> Option<f64> {self.warichu_size.clone()}
            fn x_offset_diacritic(&self) -> Option<f64> {self.x_offset_diacritic.clone()}
            fn y_offset_diacritic(&self) -> Option<f64> {self.y_offset_diacritic.clone()}
        }
    }
}
