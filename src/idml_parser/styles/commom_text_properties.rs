use super::CombineWithParent;
use derive_getters::Getters;
use serde::Deserialize;
// use std::collections::HashMap;
use std::str::FromStr;

use serde::de::{self, Deserializer, Visitor};
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
pub struct CTPMap {
    #[serde(flatten)]
    ctp_fields: Vec<CTPEnum>,
}

impl CTPMap {
    fn add(&mut self, item: CTPEnum) {
        // TODO: add overwrite logic here
        self.ctp_fields.push(item);
    }

    pub fn iter(&self) -> Iter<'_, CTPEnum> {
        self.ctp_fields.iter()
    }
}

// impl Into<Iter<'_, CTPEnum>> for CTPMap {
//     fn into(self) -> Iter<'static, CTPEnum> {
//         self.ctp_fields.iter()
//     }
// }

// impl Into<IterMut<'_, CTPEnum>> for CTPMap {
//     fn into(self) -> IterMut<'static, CTPEnum> {
//         self.ctp_fields.iter_mut()
//     }
// }

pub trait CommonTextProperties {
    // fn ctp_fields(&self) -> &HashMap<CTPKey, CTPValue>;
    // fn mut_ctp_fields(&mut self) -> &mut HashMap<CTPKey, CTPValue>;
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

#[derive(Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
pub enum CTPEnum {
    AppliedCharacterStyle(String),
    AppliedConditions(String),
    AppliedLanguage(String),
    AppliedParagraphStyle(String),
    AutoLeading(String),
    AutoTcy(String),
    AutoTcyIncludeRoman(String),
    BaselineShift(String),
    BulletsAlignment(String),
    BulletsAndNumberingListType(String),
    BulletsTextAfter(String),
    BunriKinshi(String),
    Captilization(String),
    CharacterAlignment(String),
    CharacterDirection(String),
    CharacterRotation(String),
    CjkGridTracking(String),
    Composer(String),
    DesiredWordSpacing(String),
    DiacriticPosition(String),
    DigitsType(String),
    DropCapCharacters(String),
    DropCapLines(String),
    DropCapDetail(String),
    EndJoin(String),
    FillColor(String),
    FillTint(String),
    FirstLineIndent(String),
    FontStyle(String),
    GlyphForm(String),
    GotoNextX(String),
    GradientFillAngle(String),
    GradientFillLength(String),
    GradientFillStart(String),
    GradientStrokeAngle(String),
    GradientStrokeLength(String),
    GradientStrokeStart(String),
    GridAlignFirstLineOnly(String),
    GridAlignment(String),
    GridGyoudori(String),
    HorizontalScale(String),
    HyphenWeight(String),
    HyphenateAcrossColumns(String),
    HyphenateAfterFirst(String),
    HyphenateBeforeLast(String),
    HyphenateCapitalizedWords(String),
    HyphenateLadderLimit(String),
    HyphenateLastWord(String),
    HyphenateWordsLongerThan(String),
    Hyphenation(String),
    HyphenationZone(String),
    IgnoreEdgeAlignment(String),
    Jidori(String),
    Justification(String),
    Kashidas(String),
    KeepAllLinesTogether(String),
    KeepFirstLines(String),
    KeepLastLines(String),
    KeepLinesTogether(String),
    KeepRuleAboveInFrame(String),
    KeepWithNext(String),
    KeepWithPrevious(String),
    KentenAlignment(String),
    KentenCustomCharacter(String),
    KentenFontSize(String),
    KentenKind(String),
    KentenOverprintFill(String),
    KentenOverprintStroke(String),
    KentenPlacement(String),
    KentenPosition(String),
    KentenStrokeTint(String),
    KentenTint(String),
    KentenWeight(String),
    KentenXScale(String),
    KentenYScale(String),
    KerningMethod(String),
    KerningValue(String),
    KeyboardDirection(String),
    KinsokuHangType(String),
    KinsokuType(String),
    LastLineIndent(String),
    LeadingAki(String),
    LeadingModel(String),
    LeftIndent(String),
    Ligatures(String),
    LinkResourceId(String),
    MaximumGlyphScaling(String),
    MaximumLetterSpacing(String),
    MaximumWordSpacing(String),
    MinimumGlyphScaling(String),
    MinimumLetterSpacing(String),
    MinimumWordSpacing(String),
    MiterLimit(String),
    NoBreak(String),
    NumberingAlignment(String),
    NumberingApplyRestartPolicy(String),
    NumberingContinue(String),
    NumberingExpression(String),
    NumberingLevel(String),
    NumberingStartAt(String),
    OtfContextualAlternate(String),
    OtfDiscretionaryLigature(String),
    OtfFigureStyle(String),
    OtfFraction(String),
    OtfHvKana(String),
    OtfHistorical(String),
    OtfJustificationAlternate(String),
    OtfLocale(String),
    OtfMark(String),
    OtfOrdinal(String),
    OtfOverlapSwash(String),
    OtfProportionalMetrics(String),
    OtfRomanItalics(String),
    OtfSlashedZero(String),
    OtfStretchedAlternate(String),
    OtfStylisticAlternate(String),
    OtfStylisticSets(String),
    OtfSwash(String),
    OtfTitling(String),
    OtfOverprintFill(String),
    OtfOverprintStroke(String),
    PageNumberType(String),
    ParagraphDirection(String),
    ParagraphGyoudori(String),
    ParagraphJustification(String),
    PointSize(String),
    Position(String),
    PositionalForm(String),
    Rensuuji(String),
    RightIndent(String),
    RotateSingleByteCharacter(String),
    RubyAlignment(String),
    RubyAutoAlign(String),
    RubyAutoScaling(String),
    RubyAutoTcyAutoScale(String),
    RubyAutoTcyDigits(String),
    RubyAutoTcyIncludeRoman(String),
    RubyFlag(String),
    RubyFontSize(String),
    RubyOpenType(String),
    RubyOverhang(String),
    RubyOverprintFill(String),
    RubyOverprintStroke(String),
    RubyParentOverhangAmount(String),
    RubyParentScalingPercent(String),
    RubyParentSpacing(String),
    RubyPosition(String),
    RubyString(String),
    RubyStrokeTint(String),
    RubyTint(String),
    RubyType(String),
    RubyWeight(String),
    RubyXOffset(String),
    RubyXScale(String),
    RubyYOffset(String),
    RubyYScale(String),
    RuleAbove(String),
    RuleAboveGapOverprint(String),
    RuleAboveGapTint(String),
    RuleAboveLeftIndent(String),
    RuleAboveWeight(String),
    RuleAboveOffset(String),
    RuleAboveOverprint(String),
    RuleAboveRightIndent(String),
    RuleAboveTint(String),
    RuleAboveWidth(String),
    RuleBelowGapOverprint(String),
    RuleBelowGapTint(String),
    RuleBelowLeftIndent(String),
    RuleBelowWeight(String),
    RuleBelowOffset(String),
    RuleBelowOverprint(String),
    RuleBelowRightIndent(String),
    RuleBelowTint(String),
    RuleBelowWidth(String),
    ScaleAffectsLineHeight(String),
    ShataiAdjustRotation(String),
    ShataiAdjustTsume(String),
    ShataiAdjustAngle(String),
    ShataiMagnification(String),
    SingleWordJustification(String),
    Skew(String),
    SpaceAfter(String),
    SpaceBefore(String),
    SpanColumnInsideGutter(String),
    SpanColumnOutsideGutter(String),
    SpanColumnType(String),
    SpanSplitColumnCount(String),
    StartParagraph(String),
    StrikeThroughGapOverprint(String),
    StrikeThroughGapTint(String),
    StrikeThroughGapOffset(String),
    StrikeThroughOverprint(String),
    StrikeThroughTint(String),
    StrikeThroughWeight(String),
    StrikeThru(String),
    StrokeAlignment(String),
    StrokeColor(String),
    StrokeTint(String),
    StrokeWeight(String),
    Tatechuyoko(String),
    TatechuyokoXOffset(String),
    TatechuyokoYOffset(String),
    Tracking(String),
    TrailingAki(String),
    TreatIdeographicSpaceAsSpace(String),
    Tsume(String),
    Underline(String),
    UnderlineGapOverprint(String),
    UnderlineGapTint(String),
    UnderlineGapOffset(String),
    UnderlineOverprint(String),
    UnderlineTint(String),
    UnderlineWeight(String),
    VerticalScale(String),
    Warichu(String),
    WarichuAlignment(String),
    WarichuCharsAfterBreak(String),
    WarichuCharsBeforeBreak(String),
    WarichuLineSpacing(String),
    WarichuLines(String),
    WarichuSize(String),
    XOffsetDiacritic(String),
    YOffsetDiacritic(String),
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

#[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Properties {
    based_on: Option<String>,
    applied_font: Option<String>,
}

impl<T: CommonTextProperties + Clone> CombineWithParent for T {
    fn combine_with_parent(&self, parent: &Self) -> Self {
        let mut combined = self.clone();

        for field in parent.ctp_fields().iter() {
            combined.mut_ctp_fields().add(field.clone());
        }
        // for (key, value) in parent.ctp_fields() {
        //     combined.mut_ctp_fields().insert(*key, value.clone());
        // }
        combined
    }
}

// #[macro_export]
// macro_rules! impl_common_text_properties {
//     ($StructName:ident) => {
//         impl CommonTextProperties for $StructName {
//             fn ctp_fields(&self) -> &HashMap<CTPKey, CTPValue> {
//                 &self.ctp_fields
//             }
//             fn mut_ctp_fields(&mut self) -> &mut HashMap<CTPKey, CTPValue> {
//                 &mut self.ctp_fields
//             }
//             fn properties(&self) -> &Option<Properties> {
//                 &self.properties
//             }
//         }
//     };
// }

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

// TODO: DELETE ONCE WE KNOW CTP FIELDS ARE DESERIALIZED CORRECTLY:

//     fn applied_character_style(&self) -> Option<String>;
//     fn applied_conditions(&self) -> Option<String> ;
//     fn applied_language(&self) -> Option<String> ;
//     fn applied_paragraph_style(&self) -> Option<String> ;
//     fn auto_leading(&self) -> Option<f64> ;
//     fn auto_tcy(&self) -> Option<i16> ;
//     fn auto_tcy_include_roman(&self) -> Option<bool> ;
//     fn baseline_shift(&self) -> Option<f64> ;
//     fn bullets_alignment(&self) -> Option<String> ;
//     fn bullets_and_numbering_list_type(&self) -> Option<String> ;
//     fn bullets_text_after(&self) -> Option<String> ;
//     fn bunri_kinshi(&self) -> Option<bool> ;
//     fn captilization(&self) -> Option<Capitalization> ;
//     fn character_alignment(&self) -> Option<CharacterAlignment> ;
//     fn character_direction(&self) -> Option<CharacterDirection> ;
//     fn character_rotation(&self) -> Option<f64> ;
//     fn cjk_grid_tracking(&self) -> Option<bool> ;
//     fn composer(&self) -> Option<String> ;
//     fn desired_word_spacing(&self) -> Option<f64> ;
//     fn diacritic_position(&self) -> Option<DiacriticPosition> ;
//     fn digits_type(&self) -> Option<DigitsType> ;
//     fn drop_cap_characters(&self) -> Option<i16> ;
//     fn drop_cap_lines(&self) -> Option<i16> ;
//     fn drop_cap_detail(&self) -> Option<i32> ;
//     fn end_join(&self) -> Option<OutlineJoin> ;
//     fn fill_color(&self) -> Option<String> ;
//     fn fill_tint(&self) -> Option<f64> ;
//     fn first_line_indent(&self) -> Option<f64> ;
//     fn font_style(&self) -> Option<String> ;
//     fn glyph_form(&self) -> Option<AlternateGlyphForms> ;
//     fn goto_next_x(&self) -> Option<GotoNextX>;
//     fn gradient_fill_angle(&self) -> Option<f64> ;
//     fn gradient_fill_length(&self) -> Option<f64> ;
//     fn gradient_fill_start(&self) -> Option<Vec<f64>> ;
//     fn gradient_stroke_angle(&self) -> Option<f64> ;
//     fn gradient_stroke_length(&self) -> Option<f64> ;
//     fn gradient_stroke_start(&self) -> Option<Vec<f64>> ;
//     fn grid_align_first_line_only(&self) -> Option<bool>;
//     fn grid_alignment(&self) -> Option<GridAlignment>;
//     fn grid_gyoudori(&self) -> Option<i16>;
//     fn horizontal_scale(&self) -> Option<f64>;
//     fn hyphen_weight(&self) -> Option<i16>;
//     fn hyphenate_across_columns(&self) -> Option<bool>;
//     fn hyphenate_after_first(&self) -> Option<i16>;
//     fn hyphenate_before_last(&self) -> Option<i16>;
//     fn hyphenate_capitalized_words(&self) -> Option<bool>;
//     fn hyphenate_ladder_limit(&self) -> Option<i16>;
//     fn hyphenate_last_word(&self) -> Option<bool>;
//     fn hyphenate_words_longer_than(&self) -> Option<i16>;
//     fn hyphenation(&self) -> Option<bool>;
//     fn hyphenation_zone(&self) -> Option<f64>;
//     fn ignore_edge_alignment(&self) -> Option<bool>;
//     fn jidori(&self) -> Option<i16>;
//     fn justification(&self) -> Option<Justification>;
//     fn kashidas(&self) -> Option<Kashidas>;
//     fn keep_all_lines_together(&self) -> Option<bool>;
//     fn keep_first_lines(&self) -> Option<i16>;
//     fn keep_last_lines(&self) -> Option<i16>;
//     fn keep_lines_together(&self) -> Option<bool>;
//     fn keep_rule_above_in_frame(&self) -> Option<bool>;
//     fn keep_with_next(&self) -> Option<bool>;
//     fn keep_with_previous(&self) -> Option<bool>;
//     fn kenten_alignment(&self) -> Option<KentenAlignment>;
//     fn kenten_custom_character(&self) -> Option<String>;
//     fn kenten_font_size(&self) -> Option<f64>;
//     fn kenten_kind(&self) -> Option<KentenCharacter>;
//     fn kenten_overprint_fill(&self) -> Option<AdornmentOverprint>;
//     fn kenten_overprint_stroke(&self) -> Option<AdornmentOverprint>;
//     fn kenten_placement(&self) -> Option<f64>;
//     fn kenten_position(&self) -> Option<RubyKentenPosition>;
//     fn kenten_stroke_tint(&self) -> Option<f64>;
//     fn kenten_tint(&self) -> Option<f64>;
//     fn kenten_weight(&self) -> Option<f64>;
//     fn kenten_x_scale(&self) -> Option<f64>;
//     fn kenten_y_scale(&self) -> Option<f64>;
//     fn kerning_method(&self) -> Option<String>;
//     fn kerning_value(&self) -> Option<f64>;
//     fn keyboard_direction(&self) -> Option<CharacterDirection>;
//     fn kinsoku_hang_type(&self) -> Option<KinsokuHangTypes>;
//     fn kinsoku_type(&self) -> Option<KinsokuType>;
//     fn last_line_indent(&self) -> Option<f64>;
//     fn leading_aki(&self) -> Option<f64>;
//     fn leading_model(&self) -> Option<LeadingModel>;
//     fn left_indent(&self) -> Option<f64>;
//     fn ligatures(&self) -> Option<bool>;
//     fn link_resource_id(&self) -> Option<i32>;
//     fn maximum_glyph_scaling(&self) -> Option<f64>;
//     fn maximum_letter_spacing(&self) -> Option<f64>;
//     fn maximum_word_spacing(&self) -> Option<f64>;
//     fn minimum_glyph_scaling(&self) -> Option<f64>;
//     fn minimum_letter_spacing(&self) -> Option<f64>;
//     fn minimum_word_spacing(&self) -> Option<f64>;
//     fn miter_limit(&self) -> Option<f64>;
//     fn no_break(&self) -> Option<bool>;
//     fn numbering_alignment(&self) -> Option<ListAlignment>;
//     fn numbering_apply_restart_policy(&self) -> Option<bool>;
//     fn numbering_continue(&self) -> Option<bool>;
//     fn numbering_expression(&self) -> Option<String>;
//     fn numbering_level(&self) -> Option<i32>;
//     fn numbering_start_at(&self) -> Option<i32>;
//     fn otf_contextual_alternate(&self) -> Option<bool>;
//     fn otf_discretionary_ligature(&self) -> Option<bool>;
//     fn otf_figure_style(&self) -> Option<OTFFigureStyle>;
//     fn otf_fraction(&self) -> Option<bool>;
//     fn otf_hv_kana(&self) -> Option<bool>;
//     fn otf_historical(&self) -> Option<bool>;
//     fn otf_justification_alternate(&self) -> Option<bool>;
//     fn otf_locale(&self) -> Option<bool>;
//     fn otf_mark(&self) -> Option<bool>;
//     fn otf_ordinal(&self) -> Option<bool>;
//     fn otf_overlap_swash(&self) -> Option<bool>;
//     fn otf_proportional_metrics(&self) -> Option<bool>;
//     fn otf_roman_italics(&self) -> Option<bool>;
//     fn otf_slashed_zero(&self) -> Option<bool>;
//     fn otf_stretched_alternate(&self) -> Option<bool>;
//     fn otf_stylistic_alternate(&self) -> Option<bool>;
//     fn otf_stylistic_sets(&self) -> Option<i32>;
//     fn otf_swash(&self) -> Option<bool>;
//     fn otf_titling(&self) -> Option<bool>;
//     fn otf_overprint_fill(&self) -> Option<bool>;
//     fn otf_overprint_stroke(&self) -> Option<bool>;
//     fn page_number_type(&self) -> Option<PageNumberType>;
//     fn paragraph_direction(&self) -> Option<ParagraphDirection>;
//     fn paragraph_gyoudori(&self) -> Option<bool>;
//     fn paragraph_justification(&self) -> Option<ParagraphJustification>;
//     fn point_size(&self) -> Option<f64>;
//     fn position(&self) -> Option<Position>;
//     fn positional_form(&self) -> Option<PositionalForms>;
//     fn rensuuji(&self) -> Option<bool>;
//     fn right_indent(&self) -> Option<f64>;
//     fn rotate_single_byte_character(&self) -> Option<bool>;
//     fn ruby_alignment(&self) -> Option<RubyAlignments>;
//     fn ruby_auto_align(&self) -> Option<bool>;
//     fn ruby_auto_scaling(&self) -> Option<bool>;
//     fn ruby_auto_tcy_auto_scale(&self) -> Option<bool>;
//     fn ruby_auto_tcy_digits(&self) -> Option<i16>;
//     fn ruby_auto_tcy_include_roman(&self) -> Option<bool>;
//     fn ruby_flag(&self) -> Option<i32>;
//     fn ruby_font_size(&self) -> Option<f64>;
//     fn ruby_open_type(&self) -> Option<bool>;
//     fn ruby_overhang(&self) -> Option<bool>;
//     fn ruby_overprint_fill(&self) -> Option<AdornmentOverprint>;
//     fn ruby_overprint_stroke(&self) -> Option<AdornmentOverprint>;
//     fn ruby_parent_overhang_amount(&self) -> Option<RubyOverhang>;
//     fn ruby_parent_scaling_percent(&self) -> Option<f64>;
//     fn ruby_parent_spacing(&self) -> Option<RubyParentSpacing>;
//     fn ruby_position(&self) -> Option<RubyKentenPosition>;
//     fn ruby_string(&self) -> Option<String>;
//     fn ruby_stroke_tint(&self) -> Option<f64>;
//     fn ruby_tint(&self) -> Option<f64>;
//     fn ruby_type(&self) -> Option<RubyTypes>;
//     fn ruby_weight(&self) -> Option<f64>;
//     fn ruby_x_offset(&self) -> Option<f64>;
//     fn ruby_x_scale(&self) -> Option<f64>;
//     fn ruby_y_offset(&self) -> Option<f64>;
//     fn ruby_y_scale(&self) -> Option<f64>;
//     fn rule_above(&self) -> Option<bool>;
//     fn rule_above_gap_overprint(&self) -> Option<bool>;
//     fn rule_above_gap_tint(&self) -> Option<f64>;
//     fn rule_above_left_indent(&self) -> Option<f64>;
//     fn rule_above_weight(&self) -> Option<f64>;
//     fn rule_above_offset(&self) -> Option<f64>;
//     fn rule_above_overprint(&self) -> Option<bool>;
//     fn rule_above_right_indent(&self) -> Option<f64>;
//     fn rule_above_tint(&self) -> Option<f64>;
//     fn rule_above_width(&self) -> Option<RuleWidth>;
//     fn rule_below_gap_overprint(&self) -> Option<bool>;
//     fn rule_below_gap_tint(&self) -> Option<f64>;
//     fn rule_below_left_indent(&self) -> Option<f64>;
//     fn rule_below_weight(&self) -> Option<f64>;
//     fn rule_below_offset(&self) -> Option<f64>;
//     fn rule_below_overprint(&self) -> Option<bool>;
//     fn rule_below_right_indent(&self) -> Option<f64>;
//     fn rule_below_tint(&self) -> Option<f64>;
//     fn rule_below_width(&self) -> Option<RuleWidth>;
//     fn scale_affects_line_height(&self) -> Option<bool>;
//     fn shatai_adjust_rotation(&self) -> Option<bool>;
//     fn shatai_adjust_tsume(&self) -> Option<bool>;
//     fn shatai_adjust_angle(&self) -> Option<f64>;
//     fn shatai_magnification(&self) -> Option<f64>;
//     fn single_word_justification(&self) -> Option<SingleWordJustification>;
//     fn skew(&self) -> Option<f64>;
//     fn space_after(&self) -> Option<f64>;
//     fn space_before(&self) -> Option<f64>;
//     fn span_column_inside_gutter(&self) -> Option<f64>;
//     fn span_column_outside_gutter(&self) -> Option<f64>;
//     fn span_column_type(&self) -> Option<SpanColumnTypeOptions>;
//     fn span_split_column_count(&self) -> Option<i32>;
//     fn start_paragraph(&self) -> Option<StartParagraph>;
//     fn strike_through_gap_overprint(&self) -> Option<bool>;
//     fn strike_through_gap_tint(&self) -> Option<f64>;
//     fn strike_through_gap_offset(&self) -> Option<f64>;
//     fn strike_through_overprint(&self) -> Option<bool>;
//     fn strike_through_tint(&self) -> Option<f64>;
//     fn strike_through_weight(&self) -> Option<f64>;
//     fn strike_thru(&self) -> Option<bool>;
//     fn stroke_alignment(&self) -> Option<TextStrokeAlign>;
//     fn stroke_color(&self) -> Option<String>;
//     fn stroke_tint(&self) -> Option<String>;
//     fn stroke_weight(&self) -> Option<String>;
//     fn tatechuyoko(&self) -> Option<bool>;
//     fn tatechuyoko_x_offset(&self) -> Option<f64>;
//     fn tatechuyoko_y_offset(&self) -> Option<f64>;
//     fn tracking(&self) -> Option<f64>;
//     fn trailing_aki(&self) -> Option<f64>;
//     fn treat_ideographic_space_as_space(&self) -> Option<bool>;
//     fn tsume(&self) -> Option<f64>;
//     fn underline(&self) -> Option<bool>;
//     fn underline_gap_overprint(&self) -> Option<bool>;
//     fn underline_gap_tint(&self) -> Option<f64>;
//     fn underline_gap_offset(&self) -> Option<bool>;
//     fn underline_overprint(&self) -> Option<bool>;
//     fn underline_tint(&self) -> Option<f64>;
//     fn underline_weight(&self) -> Option<f64>;
//     fn vertical_scale(&self) -> Option<f64>;
//     fn warichu(&self) -> Option<bool>;
//     fn warichu_alignment(&self) -> Option<WarichuAlignment>;
//     fn warichu_chars_after_break(&self) -> Option<i16>;
//     fn warichu_chars_before_break(&self) -> Option<i16>;
//     fn warichu_line_spacing(&self) -> Option<f64>;
//     fn warichu_lines(&self) -> Option<i16>;
//     fn warichu_size(&self) -> Option<f64>;
//     fn x_offset_diacritic(&self) -> Option<f64>;
//     fn y_offset_diacritic(&self) -> Option<f64>;
//     fn properties(&self) -> Option<Properties>;
