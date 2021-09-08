use super::CombineWithParent;
use derive_getters::Getters;
use serde::Deserialize;
// use std::collections::HashMap;
use std::str::FromStr;

use serde::de::{self, Deserializer, Visitor};
use std::slice::Iter;

#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
pub struct CTPMap {
    #[serde(flatten)]
    ctp_fields: Vec<CTPEnum>,
}

impl CTPMap {
    fn add_field(&mut self, field: CTPEnum) {
        // TODO: Add overwrite logic here
        self.ctp_fields.push(field);
    }

    pub fn iter(&self) -> Iter<'_, CTPEnum> {
        self.ctp_fields.iter()
    }
}

pub trait CommonTextProperties {
    fn ctp_fields(&self) -> &CTPMap;
    fn mut_ctp_fields(&mut self) -> &mut CTPMap;
    fn properties(&self) -> &Option<Properties>;
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum CTPKey {
    AppliedCharacterStyle,
    AppliedConditions,
    AppliedLanguage,
    AppliedParagraphStyle,
    AutoLeading,
    AutoTcy,
    AutoTcyIncludeRoman,
    BaselineShift,
    BulletsAlignment,
    BulletsAndNumberingListType,
    BulletsTextAfter,
    BunriKinshi,
    Captilization,
    CharacterAlignment,
    CharacterDirection,
    CharacterRotation,
    CjkGridTracking,
    Composer,
    DesiredWordSpacing,
    DiacriticPosition,
    DigitsType,
    DropCapCharacters,
    DropCapLines,
    DropCapDetail,
    EndJoin,
    FillColor,
    FillTint,
    FirstLineIndent,
    FontStyle,
    GlyphForm,
    GotoNextX,
    GradientFillAngle,
    GradientFillLength,
    GradientFillStart,
    GradientStrokeAngle,
    GradientStrokeLength,
    GradientStrokeStart,
    GridAlignFirstLineOnly,
    GridAlignment,
    GridGyoudori,
    HorizontalScale,
    HyphenWeight,
    HyphenateAcrossColumns,
    HyphenateAfterFirst,
    HyphenateBeforeLast,
    HyphenateCapitalizedWords,
    HyphenateLadderLimit,
    HyphenateLastWord,
    HyphenateWordsLongerThan,
    Hyphenation,
    HyphenationZone,
    IgnoreEdgeAlignment,
    Jidori,
    Justification,
    Kashidas,
    KeepAllLinesTogether,
    KeepFirstLines,
    KeepLastLines,
    KeepLinesTogether,
    KeepRuleAboveInFrame,
    KeepWithNext,
    KeepWithPrevious,
    KentenAlignment,
    KentenCustomCharacter,
    KentenFontSize,
    KentenKind,
    KentenOverprintFill,
    KentenOverprintStroke,
    KentenPlacement,
    KentenPosition,
    KentenStrokeTint,
    KentenTint,
    KentenWeight,
    KentenXScale,
    KentenYScale,
    KerningMethod,
    KerningValue,
    KeyboardDirection,
    KinsokuHangType,
    KinsokuType,
    LastLineIndent,
    LeadingAki,
    LeadingModel,
    LeftIndent,
    Ligatures,
    LinkResourceId,
    MaximumGlyphScaling,
    MaximumLetterSpacing,
    MaximumWordSpacing,
    MinimumGlyphScaling,
    MinimumLetterSpacing,
    MinimumWordSpacing,
    MiterLimit,
    NoBreak,
    NumberingAlignment,
    NumberingApplyRestartPolicy,
    NumberingContinue,
    NumberingExpression,
    NumberingLevel,
    NumberingStartAt,
    OtfContextualAlternate,
    OtfDiscretionaryLigature,
    OtfFigureStyle,
    OtfFraction,
    OtfHvKana,
    OtfHistorical,
    OtfJustificationAlternate,
    OtfLocale,
    OtfMark,
    OtfOrdinal,
    OtfOverlapSwash,
    OtfProportionalMetrics,
    OtfRomanItalics,
    OtfSlashedZero,
    OtfStretchedAlternate,
    OtfStylisticAlternate,
    OtfStylisticSets,
    OtfSwash,
    OtfTitling,
    OtfOverprintFill,
    OtfOverprintStroke,
    PageNumberType,
    ParagraphDirection,
    ParagraphGyoudori,
    ParagraphJustification,
    PointSize,
    Position,
    PositionalForm,
    Rensuuji,
    RightIndent,
    RotateSingleByteCharacter,
    RubyAlignment,
    RubyAutoAlign,
    RubyAutoScaling,
    RubyAutoTcyAutoScale,
    RubyAutoTcyDigits,
    RubyAutoTcyIncludeRoman,
    RubyFlag,
    RubyFontSize,
    RubyOpenType,
    RubyOverhang,
    RubyOverprintFill,
    RubyOverprintStroke,
    RubyParentOverhangAmount,
    RubyParentScalingPercent,
    RubyParentSpacing,
    RubyPosition,
    RubyString,
    RubyStrokeTint,
    RubyTint,
    RubyType,
    RubyWeight,
    RubyXOffset,
    RubyXScale,
    RubyYOffset,
    RubyYScale,
    RuleAbove,
    RuleAboveGapOverprint,
    RuleAboveGapTint,
    RuleAboveLeftIndent,
    RuleAboveWeight,
    RuleAboveOffset,
    RuleAboveOverprint,
    RuleAboveRightIndent,
    RuleAboveTint,
    RuleAboveWidth,
    RuleBelowGapOverprint,
    RuleBelowGapTint,
    RuleBelowLeftIndent,
    RuleBelowWeight,
    RuleBelowOffset,
    RuleBelowOverprint,
    RuleBelowRightIndent,
    RuleBelowTint,
    RuleBelowWidth,
    ScaleAffectsLineHeight,
    ShataiAdjustRotation,
    ShataiAdjustTsume,
    ShataiAdjustAngle,
    ShataiMagnification,
    SingleWordJustification,
    Skew,
    SpaceAfter,
    SpaceBefore,
    SpanColumnInsideGutter,
    SpanColumnOutsideGutter,
    SpanColumnType,
    SpanSplitColumnCount,
    StartParagraph,
    StrikeThroughGapOverprint,
    StrikeThroughGapTint,
    StrikeThroughGapOffset,
    StrikeThroughOverprint,
    StrikeThroughTint,
    StrikeThroughWeight,
    StrikeThru,
    StrokeAlignment,
    StrokeColor,
    StrokeTint,
    StrokeWeight,
    Tatechuyoko,
    TatechuyokoXOffset,
    TatechuyokoYOffset,
    Tracking,
    TrailingAki,
    TreatIdeographicSpaceAsSpace,
    Tsume,
    Underline,
    UnderlineGapOverprint,
    UnderlineGapTint,
    UnderlineGapOffset,
    UnderlineOverprint,
    UnderlineTint,
    UnderlineWeight,
    VerticalScale,
    Warichu,
    WarichuAlignment,
    WarichuCharsAfterBreak,
    WarichuCharsBeforeBreak,
    WarichuLineSpacing,
    WarichuLines,
    WarichuSize,
    XOffsetDiacritic,
    YOffsetDiacritic,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CTPValue {
    // Enums
    Capitalization(Capitalization),
    CharacterAlignment(CharacterAlignment),
    CharacterDirection(CharacterDirection),
    DiacriticPosition(DiacriticPosition),
    DigitsType(DigitsType),
    OutlineJoin(OutlineJoin),
    AlternateGlyphForms(AlternateGlyphForms),
    GotoNextX(GotoNextX),
    GridAlignment(GridAlignment),
    Justification(Justification),
    Kashidas(Kashidas),
    KentenAlignment(KentenAlignment),
    KentenCharacter(KentenCharacter),
    AdornmentOverprint(AdornmentOverprint),
    RubyKentenPosition(RubyKentenPosition),
    KinsokuHangTypes(KinsokuHangTypes),
    KinsokuType(KinsokuType),
    LeadingModel(LeadingModel),
    ListAlignment(ListAlignment),
    OTFFigureStyle(OTFFigureStyle),
    PageNumberType(PageNumberType),
    ParagraphDirection(ParagraphDirection),
    ParagraphJustification(ParagraphJustification),
    Position(Position),
    PositionalForms(PositionalForms),
    RubyAlignments(RubyAlignments),
    RubyOverhang(RubyOverhang),
    RubyParentSpacing(RubyParentSpacing),
    RubyTypes(RubyTypes),
    RuleWidth(RuleWidth),
    SingleWordJustification(SingleWordJustification),
    SpanColumnTypeOptions(SpanColumnTypeOptions),
    StartParagraph(StartParagraph),
    TextStrokeAlign(TextStrokeAlign),
    WarichuAlignment(WarichuAlignment),

    // Atomic types
    I32(i32),
    I64(i64),
    F64(f64),
    F32(f32),
    Bool(bool),
    String(String),
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum CTPEnum {
    AppliedCharacterStyle(String),
    AppliedConditions(String),
    AppliedLanguage(String),
    AppliedParagraphStyle(String),
    AutoLeading(f64),
    AutoTcy(i16),
    AutoTcyIncludeRoman(bool),
    BaselineShift(f64),
    BulletsAlignment(String),
    BulletsAndNumberingListType(String),
    BulletsTextAfter(String),
    BunriKinshi(bool),
    Captilization(Capitalization),
    CharacterAlignment(CharacterAlignment),
    CharacterDirection(CharacterDirection),
    CharacterRotation(f64),
    CjkGridTracking(bool),
    Composer(String),
    DesiredWordSpacing(f64),
    DiacriticPosition(DiacriticPosition),
    DigitsType(DigitsType),
    DropCapCharacters(i16),
    DropCapLines(i16),
    DropCapDetail(i32),
    EndJoin(OutlineJoin),
    FillColor(String),
    FillTint(f64),
    FirstLineIndent(f64),
    FontStyle(String),
    GlyphForm(AlternateGlyphForms),
    GotoNextX(GotoNextX),
    GradientFillAngle(f64),
    GradientFillLength(f64),
    GradientFillStart(Vec<f64>),
    GradientStrokeAngle(f64),
    GradientStrokeLength(f64),
    GradientStrokeStart(Vec<f64>),
    GridAlignFirstLineOnly(bool),
    GridAlignment(GridAlignment),
    GridGyoudori(i16),
    HorizontalScale(f64),
    HyphenWeight(i16),
    HyphenateAcrossColumns(bool),
    HyphenateAfterFirst(i16),
    HyphenateBeforeLast(i16),
    HyphenateCapitalizedWords(bool),
    HyphenateLadderLimit(i16),
    HyphenateLastWord(bool),
    HyphenateWordsLongerThan(i16),
    Hyphenation(bool),
    HyphenationZone(f64),
    IgnoreEdgeAlignment(bool),
    Jidori(i16),
    Justification(Justification),
    Kashidas(Kashidas),
    KeepAllLinesTogether(bool),
    KeepFirstLines(i16),
    KeepLastLines(i16),
    KeepLinesTogether(bool),
    KeepRuleAboveInFrame(bool),
    KeepWithNext(bool),
    KeepWithPrevious(bool),
    KentenAlignment(KentenAlignment),
    KentenCustomCharacter(String),
    KentenFontSize(f64),
    KentenKind(KentenCharacter),
    KentenOverprintFill(AdornmentOverprint),
    KentenOverprintStroke(AdornmentOverprint),
    KentenPlacement(f64),
    KentenPosition(RubyKentenPosition),
    KentenStrokeTint(f64),
    KentenTint(f64),
    KentenWeight(f64),
    KentenXScale(f64),
    KentenYScale(f64),
    KerningMethod(String),
    KerningValue(f64),
    KeyboardDirection(CharacterDirection),
    KinsokuHangType(KinsokuHangTypes),
    KinsokuType(KinsokuType),
    LastLineIndent(f64),
    LeadingAki(f64),
    LeadingModel(LeadingModel),
    LeftIndent(f64),
    Ligatures(bool),
    LinkResourceId(i32),
    MaximumGlyphScaling(f64),
    MaximumLetterSpacing(f64),
    MaximumWordSpacing(f64),
    MinimumGlyphScaling(f64),
    MinimumLetterSpacing(f64),
    MinimumWordSpacing(f64),
    MiterLimit(f64),
    NoBreak(bool),
    NumberingAlignment(ListAlignment),
    NumberingApplyRestartPolicy(bool),
    NumberingContinue(bool),
    NumberingExpression(String),
    NumberingLevel(i32),
    NumberingStartAt(i32),
    OtfContextualAlternate(bool),
    OtfDiscretionaryLigature(bool),
    OtfFigureStyle(OTFFigureStyle),
    OtfFraction(bool),
    OtfHvKana(bool),
    OtfHistorical(bool),
    OtfJustificationAlternate(bool),
    OtfLocale(bool),
    OtfMark(bool),
    OtfOrdinal(bool),
    OtfOverlapSwash(bool),
    OtfProportionalMetrics(bool),
    OtfRomanItalics(bool),
    OtfSlashedZero(bool),
    OtfStretchedAlternate(bool),
    OtfStylisticAlternate(bool),
    OtfStylisticSets(i32),
    OtfSwash(bool),
    OtfTitling(bool),
    OtfOverprintFill(bool),
    OtfOverprintStroke(bool),
    PageNumberType(PageNumberType),
    ParagraphDirection(ParagraphDirection),
    ParagraphGyoudori(bool),
    ParagraphJustification(ParagraphJustification),
    PointSize(f64),
    Position(Position),
    PositionalForm(PositionalForms),
    Rensuuji(bool),
    RightIndent(f64),
    RotateSingleByteCharacter(bool),
    RubyAlignment(RubyAlignments),
    RubyAutoAlign(bool),
    RubyAutoScaling(bool),
    RubyAutoTcyAutoScale(bool),
    RubyAutoTcyDigits(i16),
    RubyAutoTcyIncludeRoman(bool),
    RubyFlag(i32),
    RubyFontSize(f64),
    RubyOpenType(bool),
    RubyOverhang(bool),
    RubyOverprintFill(AdornmentOverprint),
    RubyOverprintStroke(AdornmentOverprint),
    RubyParentOverhangAmount(RubyOverhang),
    RubyParentScalingPercent(f64),
    RubyParentSpacing(RubyParentSpacing),
    RubyPosition(RubyKentenPosition),
    RubyString(String),
    RubyStrokeTint(f64),
    RubyTint(f64),
    RubyType(RubyTypes),
    RubyWeight(f64),
    RubyXOffset(f64),
    RubyXScale(f64),
    RubyYOffset(f64),
    RubyYScale(f64),
    RuleAbove(bool),
    RuleAboveGapOverprint(bool),
    RuleAboveGapTint(f64),
    RuleAboveLeftIndent(f64),
    RuleAboveWeight(f64),
    RuleAboveOffset(f64),
    RuleAboveOverprint(bool),
    RuleAboveRightIndent(f64),
    RuleAboveTint(f64),
    RuleAboveWidth(RuleWidth),
    RuleBelowGapOverprint(bool),
    RuleBelowGapTint(f64),
    RuleBelowLeftIndent(f64),
    RuleBelowWeight(f64),
    RuleBelowOffset(f64),
    RuleBelowOverprint(bool),
    RuleBelowRightIndent(f64),
    RuleBelowTint(f64),
    RuleBelowWidth(RuleWidth),
    ScaleAffectsLineHeight(bool),
    ShataiAdjustRotation(bool),
    ShataiAdjustTsume(bool),
    ShataiAdjustAngle(f64),
    ShataiMagnification(f64),
    SingleWordJustification(SingleWordJustification),
    Skew(f64),
    SpaceAfter(f64),
    SpaceBefore(f64),
    SpanColumnInsideGutter(f64),
    SpanColumnOutsideGutter(f64),
    SpanColumnType(SpanColumnTypeOptions),
    SpanSplitColumnCount(i32),
    StartParagraph(StartParagraph),
    StrikeThroughGapOverprint(bool),
    StrikeThroughGapTint(f64),
    StrikeThroughGapOffset(f64),
    StrikeThroughOverprint(bool),
    StrikeThroughTint(f64),
    StrikeThroughWeight(f64),
    StrikeThru(bool),
    StrokeAlignment(TextStrokeAlign),
    StrokeColor(String),
    StrokeTint(String),
    StrokeWeight(String),
    Tatechuyoko(bool),
    TatechuyokoXOffset(f64),
    TatechuyokoYOffset(f64),
    Tracking(f64),
    TrailingAki(f64),
    TreatIdeographicSpaceAsSpace(bool),
    Tsume(f64),
    Underline(bool),
    UnderlineGapOverprint(bool),
    UnderlineGapTint(f64),
    UnderlineGapOffset(bool),
    UnderlineOverprint(bool),
    UnderlineTint(f64),
    UnderlineWeight(f64),
    VerticalScale(f64),
    Warichu(bool),
    WarichuAlignment(WarichuAlignment),
    WarichuCharsAfterBreak(i16),
    WarichuCharsBeforeBreak(i16),
    WarichuLineSpacing(f64),
    WarichuLines(i16),
    WarichuSize(f64),
    XOffsetDiacritic(f64),
    YOffsetDiacritic(f64),
    // #[serde(other)]
    // Other
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Capitalization {
    Normal,
    SmallCaps,
    AllCaps,
    CapToSmallCap,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum CharacterAlignment {
    AlignBaseline,
    AlignEmTop,
    AlignEmCenter,
    AlignEmBottom,
    AlignICFTop,
    AlignICFBottom,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum CharacterDirection {
    DefaultDirection,
    LeftToRightDirection,
    RightToLeftDirection,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum DiacriticPosition {
    DefaultPosition,
    LoosePosition,
    MediumPosition,
    TightPosition,
    OpenTypePosition,
    OpentypePositionFromBaseline,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum DigitsType {
    DefaultDigits,
    ArabicDigits,
    HindiDigits,
    FarsiDigits,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum OutlineJoin {
    MiterEndJoin,
    RoundEndJoin,
    BevelEndJoin,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GotoNextX {}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GridAlignment {
    None,
    AlignBaseLine,
    AlignEmTop,
    AlignEmCenter,
    AlignEmBottom,
    AlignICFTop,
    AlignICFBottom,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Kashidas {
    DefaultKashidas,
    KashidasOff,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum KentenAlignment {
    AlignKentenCenter,
    CharacterInput,
    ShiftJIS,
    JIS,
    Kuten,
    Unicode,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum AdornmentOverprint {
    Auto,
    OverprintOn,
    OverprintOff,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RubyKentenPosition {
    AboveRight,
    BelowLeft,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum KinsokuHangTypes {
    None,
    KinsokuHangRegular,
    KinsokuHangForce,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum KinsokuType {
    KinsokuPushInFirst,
    KinsokuPushOutFirst,
    KinsokuPushOutOnly,
    KinsokuPrioritizeAdjustmentAmount,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum LeadingModel {
    LeadingModelRoman,
    LeadingModelAkiBelow,
    LeadingModelAkiAbove,
    LeadingModelCenter,
    LeadingModelCenterDown,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ListAlignment {
    LeftAlign,
    CenterAlign,
    RightAlign,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum OTFFigureStyle {
    TabularOldStyle,
    TablularLining,
    ProportionalOldStyle,
    ProportinalLining,
    Default,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum PageNumberType {
    AutoPageNumber,
    NextPageNumber,
    PreviousPageNumber,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ParagraphDirection {
    LeftToRightDirection,
    RightToLeftDirection,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ParagraphJustification {
    DefaultJustification,
    ArabicJustification,
    NaskhJustification,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Position {
    Normal,
    Superscript,
    Subscript,
    OTSuperscript,
    OTSubscript,
    OTNumerator,
    OTDenominator,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum PositionalForms {
    None,
    Calculate,
    Initial,
    Medial,
    Final,
    Isolated,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RubyAlignments {
    RubyLeft,
    RubyCenter,
    RubyRight,
    FullJustify,
    RubyJIS,
    RubyEqualAki,
    Ruby1Aki,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RubyOverhang {
    None,
    RubyOverhangOneRuby,
    RubyOverhangHalfRuby,
    RubyOverhangOneChar,
    RubyOverhangHalfChar,
    RubyOverhangNoLimit,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RubyParentSpacing {
    RubyParentNoAdjustment,
    RubyParentBothSides,
    RubyParent121Aki,
    RubyParentEqualAki,
    RubyParentFullJustify,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RubyTypes {
    GroupRuby,
    PerCharacterRuby,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum RuleWidth {
    TextWidth,
    ColumnWidth,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum SingleWordJustification {
    LeftAlign,
    CenterAlign,
    RightAlign,
    FullyJustified,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum SpanColumnTypeOptions {
    SingleColumn,
    SpanColumns,
    SplitColumns,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum StartParagraph {
    Anywhere,
    NextColumn,
    NextFrame,
    NextPage,
    NextOddPage,
    NextEvenPage,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum TextStrokeAlign {
    CenterAlignment,
    OutsideAlignment,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
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

#[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Properties {
    based_on: Option<String>,
    applied_font: Option<String>,
}

impl<T: CommonTextProperties + Clone> CombineWithParent for T {
    /// Combines all the common text properties fields of the parent with the child.
    ///
    fn combine_with_parent(&self, parent: &Self) -> Self {
        let mut combined = parent.clone();
        // Override
        for p_field in self.ctp_fields().iter() {
            combined.mut_ctp_fields().add_field(p_field.clone());
        }
        combined
    }
}

#[macro_export]
macro_rules! impl_common_text_properties {
    ($StructName:ident) => {
        impl CommonTextProperties for $StructName {
            fn ctp_fields(&self) -> &CTPMap {
                &self.ctp_fields
            }
            fn mut_ctp_fields(&mut self) -> &mut CTPMap {
                &mut self.ctp_fields
            }
            fn properties(&self) -> &Option<Properties> {
                &self.properties
            }
        }
    };
}
