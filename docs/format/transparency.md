# Transparency

Implemented in `src/idml/transparency.rs`.

Page items keep their transparency settings in their attribute list
(chunk 0x6E03, see `attributes.md`), with attribute IDs in two ranges:
0x108xx and 0x1EBxx. Object styles keep a full list of the same
attributes in chunk 0x1B92C (u16 count, then records).

IDML writes them as child elements of the page item or object style: `TransparencySetting`
(the whole object), `StrokeTransparencySetting` and
`FillTransparencySetting`, each holding one element per effect
(`BlendingSetting`, `DropShadowSetting`, `InnerShadowSetting`,
`GradientFeatherSetting`, ...).

## How the mapping was checked

IDML writes a setting on a page item only when it differs from the item's
object style. So each INDD value was compared with the value in effect:

1. the value written on the IDML item, if there is one;
2. otherwise the value of its object style, following `BasedOn`;
3. otherwise the default in `Resources/Preferences.xml`
   (`TransparencyDefaultContainerObject`).

The tables count page items of the pairs whose IDML is from the same
InDesign version as the INDD. "On the item" means the IDML item states the
value itself, which is the evidence that tells the attribute apart. The
576 object styles of the pairs (matched by name) agree with their IDML
style on every attribute below.

## Values equal to the object style

The converter writes only the values that differ from the object
style's (its chunk 0x1B92C, then the styles it is based on), and an
effect element only when it has such a value or its opacity stops
differ from the style's. Per effect element the converter wrote before
this rule, over the 489 trustworthy pairs (D = the attributes whose value
differs from the style's effective value; style values followed through
`BasedOn`, then `TransparencyDefaultContainerObject`):

| Element | D empty, IDML writes nothing | D empty, IDML writes it | D not empty, IDML writes exactly D or more |
|---|---:|---:|---:|
| `GradientFeatherSetting` | 6,251 | 53 | 2,604 |
| `InnerShadowSetting` | 2,884 | 0 | 17 |
| `DropShadowSetting` | 44 | 8 | 289 |
| `BlendingSetting` | 1 | 0 | 1,616 |

Where IDML writes more than D (349 elements), the converter already
wrote those values. The 53 + 8 elements with D empty that IDML still
writes are not explained.

## Object transparency (`TransparencySetting`)

The drop shadow rows of 0x1081B, 0x1081C, 0x1081E, 0x1081F and 0x1084A
come from the items of the 1,251 trustworthy pairs of the corpus after
2026-10 that have a `DropShadowSetting` in IDML, groups included: each
ID is the only one whose value equals the IDML value in every item
where both exist (after resolving the value through the object style,
as below). In the item list 0x1081F is the drop shadow's blend mode; in
the document preference list the same number is `BlendingSpace`, a
separate list. With the write rule below, the converter reproduces over
the trustworthy pairs 580 of 580 `XOffset` and `YOffset` values, 397 of
397 `Opacity`, 62 of 62 `BlendMode` and 3 of 3 `Spread`, with no extra
value.

**Comparison with the style.** IDML writes an item's value when it
differs from the style's in any digit: one 15.0 document has 23 text
frames with `YOffset` 7.000000133081193 and a style value of 7, and
IDML writes all 23. Comparing the full text of the values changes
nothing else in the transparency settings of the trustworthy pairs.

| ID | IDML | Encoding | On the item |
|---|---|---|---|
| 0x10816 | `BlendingSetting/Opacity` | f64, percent | 57 items in 4 files (40, 49, 50, 57, 69, 70) |
| 0x10817 | `BlendingSetting/BlendMode` | i32 code (below) | 56 items in 4 files |
| 0x1081A | `DropShadowSetting/Mode` | i32: 0 `None`, 1 `Drop` | 8 items with 1 in 2 files |
| 0x1081F | `DropShadowSetting/BlendMode` | i32 code (below): 0 `Normal`, 4 `SoftLight` seen | 89 of 89 items in 16 files |
| 0x1081E | `DropShadowSetting/Opacity` | f64, percent | 424 of 424 items in 78 files |
| 0x1081B | `DropShadowSetting/XOffset` | f64 | 649 of 649 items in 88 files |
| 0x1081C | `DropShadowSetting/YOffset` | f64 | 649 of 649; 256 of these items have X ≠ Y |
| 0x10820 | `DropShadowSetting/Size` | f64 | 6 items, two values; 406 of 406 rectangles and text frames over the 654 pairs of the later corpus |
| 0x1084A | `DropShadowSetting/Spread` | f64, percent | 28 of 28 items in 9 files |
| 0x10837 | `DropShadowSetting/EffectColor` | swatch UID | 11 of 11 items in 6 files (below) |
| 0x1EB6F | `DropShadowSetting/KnockedOut` | u16: 1 true, 0 false; `true` where not stored | 30,692 of 30,692 items (below) |
| 0x1084D | `InnerShadowSetting/Applied` | u16: 1 true, 0 false | 10 items with 1 |
| 0x1084E | `InnerShadowSetting/EffectColor` | swatch UID | 10 items (below) |
| 0x10852 | `InnerShadowSetting/Distance` | f64 | 10 items with 0 |
| 0x10855 | `InnerShadowSetting/Size` | f64 | 10 items with 2.83 |
| 0x1EB8A | `GradientFeatherSetting/Applied` | u16: 1 true, 0 false | 56 items in 2 files |
| 0x1EB8C | `GradientFeatherSetting` stops | list (below) | 59 items, 170 stops |
| 0x1EB8D | `GradientFeatherSetting/Angle` | f64 | 1 item (−90) |
| 0x1EB8E | `GradientFeatherSetting/Length` | f64 | 96 items in 3 files |
| 0x1EB8F | `GradientFeatherSetting/GradientStart` | two f64 | 96 items in 3 files |
| 0x1EB91 | `GradientFeatherSetting/HiliteAngle` | f64 | 1 item (−62.2) |
| 0x1EB92 | `GradientFeatherSetting/HiliteLength` | f64 | 1 item (1) |

More IDs, found over the 489 trustworthy pairs before 2026-10 by
matching the item's attribute list with the IDML value after resolving
the value through the object style, `BasedOn` and
`TransparencyDefaultContainerObject` (each ID is the only one whose
value maps one to one onto the IDML value in all rows):

| ID | IDML | Encoding | Items |
|---|---|---|---|
| 0x10819 | `BlendingSetting/IsolateBlending` | u16: 1 true | 46 of 46 |
| 0x10857 | `OuterGlowSetting/Applied` | u16: 1 true | 157 of 157 |
| 0x10858 | `OuterGlowSetting/BlendMode` | i32 code (below; 0, 1, 3, 9 seen) | 79 of 79 |
| 0x10859 | `OuterGlowSetting/Opacity` | f64 | 144 of 144 |
| 0x1085B | `OuterGlowSetting/EffectColor` | swatch UID | 73 of 73 |
| 0x1085D | `OuterGlowSetting/Spread` | f64 | 106 of 106 |
| 0x1085E | `OuterGlowSetting/Size` | f64 | 172 of 172 |
| 0x1085F | `InnerGlowSetting/Applied` | u16 | 6 of 6 |
| 0x10860 | `InnerGlowSetting/BlendMode` | i32 code (0, 1 seen) | 10 of 10 |
| 0x10866 | `InnerGlowSetting/Size` | f64 | 10 of 10 |
| 0x108D8 | `FillTransparencySetting/BlendingSetting/Opacity` | f64 | 6 of 6 (100, 40) |

IDML writes `OuterGlowSetting` and `InnerGlowSetting` after
`InnerShadowSetting`, as the schema lists them. Not identified: the
inner glow's `Opacity`, `Spread` and `EffectColor` and the outer glow's
`Noise`. Drop shadow `KnockedOut` is 0x1EB6F (below).

All items whose value is not on the IDML item (it comes from the style
or the default) match as well: for example 743 items for
`InnerShadowSetting` and the gradient feather attributes, with the
default values (`Applied="false"`, `Distance="7"`, `Size="7"`, `Length="0"`).

In pairs whose IDML is from an older version, 14 more items match on
opacity and blend mode. One item there has 50 where the IDML says 100;
that file was probably saved again after the export.

**Values outside the schema's range.** Four items in a file without an
IDML store a drop shadow size of −2.83; the schema allows 0 to 1000.
The converter leaves out any value of the attributes above that is
outside the schema's range, with a warning: sizes and distances 0–1000,
opacity 0–100, angles −180–180.

**Blend mode codes.** The code is the position of the value in the IDML
schema's `BlendMode` enumeration (`Normal`, `Multiply`, `Screen`,
`Overlay`, `SoftLight`, `HardLight`, `ColorDodge`, `ColorBurn`,
`Darken`, `Lighten`, `Difference`, `Exclusion`, `Hue`, `Saturation`,
`Color`, `Luminosity`, from 0). Seen: 1 `Multiply` (3 items), 3
`Overlay` (62), 9 `Lighten` (5), and 0 `Normal` in all 576 object
styles, whose IDML blend mode is `Normal`; over the 489 trustworthy
pairs before 2026-10 also 2
`Screen` (7 items), 4 `SoftLight` (10), 5 `HardLight` (12), 7
`ColorBurn` (3), 14 `Color` (3) and 15 `Luminosity` (7), 42 of 42. The
converter writes all sixteen positions.

**Inner shadow colour.** The 10 items with the effect applied name the
swatch that IDML writes as `EffectColor`. Items without the effect store
UID 0xB, where the IDML default is `n` (no colour). The converter writes
the colour only when the effect is applied.

**Drop shadow colour.** Item attribute 0x10837 is the swatch of the
drop shadow, IDML `EffectColor`. With the write rule above, the
converter reproduces all 11 `EffectColor` values of the trustworthy
pairs (8 polygons, 2 rectangles, 1 text frame; swatches such as
`Color/Paper` and named colours), with no extra value. In 7 of these
items the object style chain stores 0 for the attribute, which names no
swatch, so every item colour differs from it. The one other item whose
style stores 0 has the Black swatch, and its IDML has no
`EffectColor`. One item cannot show whether IDML compares with Black or
leaves the value out for another reason; the converter leaves out
Black when the style stores 0.

**Drop shadow knockout.** Page item attribute 0x1EB6F is
`DropShadowSetting/KnockedOut`: a u16, 1 `true`, 0 `false` (value type
0x6E65). Object styles hold it in their transparency list (chunk
0x1B92C); `[None]` stores 1, and the styles that store 0 are mostly
`[Normal Graphics Frame]`, `[Normal Text Frame]` and user styles of
older documents. The document's base list (class 0x6E02) does not have
it. An item or a style chain that does not store it has `true`, the
value of `TransparencyDefaultContainerObject` in every corpus IDML.
IDML writes `KnockedOut` on the item when this value differs from the
style's, by the rule above; an item without an object style gets no
value it does not store.

With this rule the converter reproduces, over the trustworthy pairs,
every `KnockedOut` of the items: 17,543 of 17,543 rectangles, 9,505 of
9,505 text frames, 1,911 of 1,911 polygons, 1,578 of 1,578 graphic
lines and 155 of 155 ovals. Before the rule, 25, 13, 14, 23 and 8
documents missed these `DropShadowSetting` elements. It also writes 380
elements that IDML does not have (760 extra values in 3 documents):
items that store nothing, whose style stores 0, and whose IDML has no
`KnockedOut`. Nothing stored in the items or their styles separates
them from the items that follow the rule; they stay open (`unknowns.md`).

Not covered: items whose drop shadow is off (`Mode="None"`) for which
IDML still writes a `DropShadowSetting` (the "D empty, IDML writes it"
column above).

## Stroke and fill transparency

Only the gradient feather has values that tell its attributes apart:

| ID | IDML | On the item |
|---|---|---|
| 0x1EB93 | `StrokeTransparencySetting/GradientFeatherSetting/Applied` | 2 items through their style, and the style |
| 0x1EB95 | stroke gradient feather stops | 2 items |
| 0x1EB96 | `StrokeTransparencySetting/GradientFeatherSetting/Angle` | 2 items through their style (90), and the style |
| 0x1EB97 | `StrokeTransparencySetting/GradientFeatherSetting/Length` | 4 items in 2 files |
| 0x1EB98 | `StrokeTransparencySetting/GradientFeatherSetting/GradientStart` | 4 items in 2 files |
| 0x1EB9C | `FillTransparencySetting/GradientFeatherSetting/Applied` | 10 items, 1 through its style |
| 0x1EB9E | fill gradient feather stops | 11 items |
| 0x1EB9F | `FillTransparencySetting/GradientFeatherSetting/Angle` | 10 items (−90) |
| 0x1EBA0 | `FillTransparencySetting/GradientFeatherSetting/Length` | 3 items in 2 files |
| 0x1EBA1 | `FillTransparencySetting/GradientFeatherSetting/GradientStart` | 3 items in 2 files |

The stroke, fill and object gradient feathers of the same item can hold
the same length and start, so each ID was taken from items where only one
of the three is set (for example 0x1EB97 matches the stroke value in 4
items and the fill value in only 2).

## Opacity gradient stops

Value type 0x1085F. A u32 stop count, then three f64 per stop:

| Field | Contents |
|---|---|
| f64 | Location, 0–1 (IDML `Location` in percent) |
| f64 | Position of the midpoint between this stop and the next, 0–1, measured from the start of the gradient; 1 for the last stop |
| f64 | Opacity in percent (IDML `Opacity`) |

IDML writes a `Midpoint` on every stop but the first: the midpoint
between the previous stop and this one, in percent of the distance
between them. So for stop *i*, `Midpoint` = (midpoint of stop *i*−1 −
location of stop *i*−1) / (location of stop *i* − location of stop *i*−1)
× 100. A list with count 0 is stored as a 4-byte value.

All 170 stops of the items above match on `Opacity`, `Location` and
`Midpoint`; so do the 5 stops of two object styles. IDML names an item's
stops `<item>TransparencySetting1GradientFeatherSetting1OpacityGradientStop<i>`
(with `StrokeTransparencySetting` or `FillTransparencySetting` for those
settings), 170 of 170.

## Object styles

Chunk 0x1B92C of an object style holds the transparency attributes with
the IDs and value types of page items (u16 count, then records, see
`attributes.md`). Evidence: the 1,252 trustworthy pairs of the corpus
after 2026-10, 5,560 INDD object styles, 5,466 of them matched by name
to an IDML `ObjectStyle`. Every style of DOM 7.0 to 21.5 has the same
360 IDs as its root style. InDesign 4.0 documents have 17 IDs in another
layout; the converter writes no transparency for their styles.

**Write rule.** IDML writes an attribute of a style when the style's
value differs from the value of the root style `[None]`. Counted over
the matched styles, per style, setting and attribute (the drop shadow
colour aside, below):

| Category flag (chunk 0x1B92E) | Value differs from the root | Cells | In IDML |
|---|---|---:|---|
| on | no | 1,663,254 | none |
| on | yes | 371 | all |
| off | no | 3,480 | none |
| off | yes | 25 | all |

So the transparency categories (`EnableTransparency="false"`) do not
change what is written, and the comparison is with the root, not the
style it is based on: 36 written values equal the parent's value, and 8
values equal to the root whose parent differs are not written. The root
has the same list in all 1,252 documents and gets no setting element;
neither does a style whose values all equal the root's. Opacity stops
follow the same rule: the root has an empty list, and the 11 styles
with stops have them in IDML (8 with the category on, 3 off), all
matching the formula below. Their `Self` is
`u<style UID in hex><setting>1GradientFeatherSetting1OpacityGradientStop<i>`
(23 of 23 stops).

IDML writes the settings after `FrameFittingOption` and before
`ObjectStyleObjectEffectsCategorySettings` in all 184 styles that have
them. No style has `ContentTransparencySetting`.

**Drop shadow colour.** The root stores 0 (no swatch) for the object
drop shadow's `EffectColor` (0x10837). 14 styles store Black (0xB), 10
of them with the drop shadow on and 4 off, and none has an `EffectColor`
in IDML; 9 styles store another swatch, and IDML writes it in all 9.
The converter leaves out Black where the root stores 0, as for page
items. In the stroke and fill blocks the root stores Black, so the rule
above already leaves it out. Other colours follow the rule: outer glow
`Color/Black` where the root has Paper (2 values), satin `Color/Paper`
where the root has Black (2).

**Blocks.** The list holds four blocks of 69 attributes, for
`TransparencySetting`, `StrokeTransparencySetting`,
`FillTransparencySetting` and `ContentTransparencySetting`, plus the
gradient feather and drop shadow `KnockedOut` IDs listed above for page
items. The attributes are in the same order in every block, and the
root's values agree with `TransparencyDefaultContainerObject` of the
IDML at all 205 numeric, boolean and point positions:

| Block | IDs |
|---|---|
| Stroke | 0x10891 to 0x108A0, 0x108A3 to 0x108D7 |
| Fill | 0x108D8 to 0x108E7, 0x108EA to 0x108FF, 0x1EB00 to 0x1EB1E |
| Content | 0x1EB28 to 0x1EB37, 0x1EB3A to 0x1EB6E |
| Object | 0x10816 to 0x1081C, 0x1081E to 0x10822, 0x10825, 0x10837, 0x1084A, 0x1084D to 0x1087F |

The object block's drop shadow `Noise` and `UseGlobalLight` and its
feather `Noise` have no identified ID.

The converter writes on styles the page item IDs above and the IDs
below, each shown by a value: in every style where IDML writes the
attribute, the ID holds the IDML value (0 mismatches), at the position
the block layout gives it. Values are as for page items: f64 with all digits,
percent stored as percent, u16 1 `true`, blend modes by the code table
below, colours as swatch UIDs.

| Setting | Effect | Attribute: ID (values written) |
|---|---|---|
| Object | `FeatherSetting` | `Mode`: 0x10821, i32: 0 `None`, 3 `Standard` (2) |
| Object | `InnerShadowSetting` | `Opacity`: 0x10850 (1) |
| Object | `BevelAndEmbossSetting` | `ShadowColor`: 0x10875 (3) |
| Stroke | `BlendingSetting` | `BlendMode`: 0x10892 (2) |
| Stroke | `OuterGlowSetting` | `Applied` 0x108AF, `Opacity` 0x108B1, `EffectColor` 0x108B3, `Spread` 0x108B5 (2 each) |
| Fill | `BlendingSetting` | `BlendMode`: 0x108D9 (1) |
| Fill | `DropShadowSetting` | `Mode` 0x108DC (0 `None`, 1 `Drop`), `XOffset` 0x108DD, `YOffset` 0x108DE, `UseGlobalLight` 0x108E5 (2 each) |
| Fill | `InnerShadowSetting` | `Applied` 0x108EC, `EffectColor` 0x108ED, `Opacity` 0x108EF, `Angle` 0x108F0, `Distance` 0x108F1, `ChokeAmount` 0x108F3, `Size` 0x108F4 (2 each) |
| Fill | `OuterGlowSetting` | `Applied` 0x108F6, `BlendMode` 0x108F7, `EffectColor` 0x108FA, `Size` 0x108FD (2 each) |
| Fill | `InnerGlowSetting` | `Applied` 0x108FE (8), `BlendMode` 0x108FF (2), `Opacity` 0x1EB00 (8), `EffectColor` 0x1EB02 (2), `Size` 0x1EB05 (8) |
| Fill | `BevelAndEmbossSetting` | `Applied` 0x1EB07, `Size` 0x1EB0C, `Soften` 0x1EB0D, `Angle` 0x1EB0E, `Altitude` 0x1EB0F, `HighlightOpacity` 0x1EB13, `ShadowOpacity` 0x1EB16 (2 each) |
| Fill | `SatinSetting` | `Applied` 0x1EB17, `EffectColor` 0x1EB18, `BlendMode` 0x1EB19, `Opacity` 0x1EB1A, `Angle` 0x1EB1B, `Distance` 0x1EB1C, `Size` 0x1EB1D, `InvertEffect` 0x1EB1E (2 each) |

Styles confirm page item rows as well, for example drop shadow `Mode`
(32 values), `XOffset` and `YOffset` (29), `Opacity` and `Size` (25),
`EffectColor` (9) and `KnockedOut` (121 values, all `false`, in 31
documents; the root stores 1), and the fill and stroke gradient
feathers (`Applied` 7 and 1, `Angle` 6 and 1, stops 7 and 1).

With these rows the converter reproduces, over the 1,252 trustworthy
pairs, 491 of the 493 transparency attribute values of object styles
(stop attributes counted one by one), with no value that IDML does not
have. The 2 left are `HonorOtherEffects` of a fill drop shadow, which
has no identified ID. The IDs found on styles are written on styles
only; no page item sample was checked against them.

## Not converted

- Drop shadow `Noise`, and `UseGlobalLight` other than that of the fill
  drop shadow of object styles.
- On page items, the stroke, fill and content effects other than the
  gradient feather and fill opacity. One text frame has
  `ContentTransparencySetting` with an inner glow and a satin effect
  applied, and two attributes set to 1, so it cannot show which is
  which.
- On object styles, the attributes that no sample shows by a value
  (`unknowns.md`), and `ContentTransparencySetting`.
- `GradientFeatherSetting/Type`: `Linear` in every sample.
- Transparency of placed graphics.
