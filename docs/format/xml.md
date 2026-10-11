# XML structure

How the document's XML structure (IDML `XmlStory` and `XMLElement`) is
stored. XML tags are in `objects.md`. Implemented in `src/model/xml.rs`
and the story writer in `src/idml/story.rs`.

**Evidence.** All 251 distinct little-endian corpus files have an XML
structure; in 250 it is only the root element. The corpus pairs have 78
IDML `XmlStory` elements and 86 `XMLElement` elements: 78 root elements
and 8 more elements in one document. Unless stated otherwise, counts are
over the 251 files.

## Nodes

The structure is a tree of nodes. The document node is the root
element's parent and has no IDML element. Each node belongs to a story,
the one whose text holds its markers (below), and is named by that
story's UID and an ID that is unique within the story: a *node key*.

**Node store.** A story's chunk 0x16126 starts with the UID of the first
page of its node store. Store pages are objects of class 0x1610A with the
same chunk 0x16127 layout as hyperlink range tree pages
(`hyperlinks.md`): u32 next page, u32 previous page, u32 story, u32, u32
*n*, *n* node IDs, u32 node count, then the nodes, each u32 type, u32
size and the node data. Types: 0xBF2F the document node (251, one per
file), 0xBF34 an element (259).

**Node data:** a u16 count, then that many parts, each u32 ID, u32
length and data. Two parts are used:

| Part | Contents |
|---|---|
| 0xBF0D | u32 tag UID (0 for the document node), u32 content UID (0 = none), 22 bytes, parent node key (u32 story, u32 ID; 0, 0 for the document node), the node's own key, u32 *n*, *n* child node keys |
| 0x1032 | u32 start marker ID, u32 end marker ID; 0xFFFFFFFF = none |

All 510 nodes have part 0xBF0D of exactly this size, and the 22 bytes are
the same in all of them.

**Attributes.** The 22 bytes end with a u32 attribute count, 0 in the
nodes above. A node with attributes has them after the count, before
the parent key:

```
u32 name length (UTF-16 units), name as text segments
u16 (not identified)
u32 value length, value as text segments
```

IDML writes each as `<XMLAttribute Self="<element Self>XMLAttributen<name>"
Name="…" Value="…"/>`, the first children of its `XMLElement`. Over the
pairs of the corpus of 2026-10, the nodes of 30 pairs have attributes,
every such node parses to the end of its part with this layout, and in
every pair the converter's `XMLAttribute` elements have the same
multisets of `Self` and of (`Name`, `Value`) as the IDML's (28 trustworthy pairs, 71
attributes; 12,247 attributes in the 2 stale pairs). Without the layout
the converter left the whole XML structure of these stories out
("XML node of unknown layout", 108 stories). The own key names the story whose store holds
the node (510 of 510). In the InDesign 3.0 and 4.0 files the 22 bytes
are 4 and 20 bytes (`big-endian.md`).

**References to nodes.** Chunk 0xBF14 is a node key:

- In the document (class 0xE01): the backing story and the document node
  (251 of 251).
- In a story: the element whose content is that story (4 stories, all in
  one document; each element's content UID is the story).
- In an object of class 0x2C65: the element whose content it is (1
  object). This object is a child of a page item: its chunk 0x15B names
  the page item as parent.

## Markers

Each node has a start marker and, unless it is an element that stands
for content held elsewhere, an end marker. A marker is a U+FEFF
character in the text of the node's story. Its position is in a tree,
chunk 0xCA57 of the story's strand of class 0x2B8:

- u16: 0 for an empty tree.
- The tree's nodes in preorder, each u16 (0 or 1, not identified), u32
  value, u32 marker ID, u32 length (1 in every sample), u16 flags: 1 =
  has a left child, 2 = has a right child.
- The rest is not used (u32, u32 *n*, then *n* pairs of u32).

The root's value is its position, a left child is *value* before its
parent and a right child *value* after it, as in hyperlink range trees.
Positions count characters (`objects.md`).

Checks: the tree parses in all 17,911 stories (252 trees are not empty).
Of their 1,015 markers, all are on a U+FEFF character, all are named by
a node's part 0x1032, and every marker that a node names is in the tree.
The texts hold another 2,690 U+FEFF characters that are not markers.

The positions in the marker tree are markers even when the story's node
store cannot be read (a node layout the converter does not know: 10
trustworthy pairs, one story each, and 79 stories of one stale pair). The
converter then leaves the structure out, with a warning, and does not
write the marker characters; dropping them gives the IDML text in every
such story of the trustworthy pairs (18 markers).

## The backing story

The document's chunk 0xBF14 names the backing story, which IDML writes
as `XmlStory` with `Self="u<UID>"` in `XML/BackingStory.xml` (76 of 78
pairs; the other 2 IDML files were exported from another save). Its text
is markers and a paragraph return. The document's chunk 0x222 holds two
UID lists: the stories and then the backing story. IDML `StoryList` is
the two lists in that order (74 of 78; the other 4 are the pairs from
another save, or whose IDML has stories that the INDD no longer has).

## IDML elements

**`Self`** is `d`, then `i` and the hexadecimal node ID of each element
on the path from the root element down to the element: the root element
with ID 2 is `di2`, its child with ID 0xB `di2ib`. The path follows the
parent keys, so it can pass through nodes of several stories (an element
with ID 4 in another story's store, whose parent is `di2ibifi1f`, is
`di2ibifi1fi4`). Matches 84 of 86 IDML elements; the other 2 are the
root elements of the 2 pairs from another save. Root element IDs are 2
(63 pairs) and 3 (15).

**`MarkupTag`** is `XMLTag/` and the name of the tag that the node names
(84 of 84).

**`XMLContent`** (5 elements in the pairs): a story's `Self` when the
content UID is a story (4 of 4); for an object of class 0x2C65, the
`Self` of its parent page item (1 of 1).

## Where IDML writes the elements

IDML writes elements in story text at their markers, and drops the
marker characters:

- An element with a start and an end marker holds the text between them.
- An element with only a start marker is written empty, at the marker.
- The start marker of the document node is not written. Its end marker
  and the U+FEFF characters that are not markers are written as text.
- An element whose content is a story holds all of that story's text, in
  the story's own file: the story's chunk 0xBF14 names it, and IDML
  writes it around the story's paragraph ranges (4 stories).

### Nesting with ranges

An element with a start and an end marker is written at one of three
levels. Its *last character* is the one before its end marker.

| Level | When | Written as |
|---|---|---|
| Around paragraph ranges | its start marker and last character are in different paragraph ranges | a child of the story or of its parent element; the paragraph ranges split after the start marker and at the end marker |
| Around character ranges | otherwise, when its start marker and last character are in different character ranges | a child of the paragraph range or of its parent element, holding character ranges |
| Inside a character range | otherwise | a child of the character range |

The marker characters are not written, but they decide where ranges
start and end:

- The start marker of an element written around ranges is in a
  character range of the parent: the one that is open, or an empty one
  written for it. This gives the empty `CharacterStyleRange` before such
  an element, and between two of them.
- An end marker is in no range. A character range that would start with
  an end marker opens at the next character that needs one, and a
  paragraph range that would hold only the end marker of an element
  written around paragraph ranges is not written.
- An element whose last character is a paragraph return is written in
  that paragraph, even when its end marker is in the next one.

Elements with a story as content (*story elements*) depend on the
version:

- From version 11, a story element is written like any other empty
  element: what follows it stays in the same character range. Samples:
  65 story elements in 5 trustworthy pairs of InDesign 11.4 to 17.4, all
  directly in a paragraph range, 35 of them followed by more in the same
  character range; and 79 in the stale pair of InDesign 11.4 below, all
  inside an element written in a character range, 72 of them followed by
  more in the same character range.
- Before version 11, a story element ends its character range, and an
  element that holds one with more of its own text after it (not
  counting end markers) is written around character ranges. The only
  sample is one stale pair of InDesign 7.5 (4 story elements, 3 of them
  in one element). There is no pair between 7.5 and 11.4.

**Evidence.** Over the 36 trustworthy pairs of the corpus of 2026-10
whose XML structure has more than the root element, an element *matches*
when the converter writes an `XMLElement` with the same `Self` whose
parent has the same tag, whose nearest enclosing `XMLElement` is the
same, and whose children (element names, and `Self` of child elements,
without `Properties` and `XMLAttribute`) are the same. Elements whose
content is the story in their own story file are counted as *story
wrappers*.

| Level in the reference | Pairs | Elements | Match |
|---|---|---|---|
| Around paragraph ranges | 6 | 323 | 205 |
| Around character ranges | 9 | 771 | 588 |
| Inside a character range | 33 | 1,966 | 1,729 |
| Story wrapper | 5 | 65 | 64 |
| All | 36 | 3,125 | 2,586 |

Without the one pair whose structure has table and cell elements (below),
643 of 647 match: 201 of 203 around paragraph ranges, 222 of 222 around
character ranges, 162 of 163 inside a character range, 58 of 59 story
wrappers. The other 4: in 2 pairs (one document saved twice) IDML writes
two paragraph ranges where the converter writes one, because the
paragraph formatting it reads is the same (below); one element has a
`Link` child that is not converted; one story wrapper holds a paragraph
range split the same way. In the pair with table and cell elements, 1,943
of 2,478 match: the 105 table and cell elements are not written, which
puts their 305 descendants in the wrong element. The other 125 are 26
elements that IDML writes around paragraph ranges and the converter
around character ranges, their 93 children and 4 parents, and 2 elements
with fewer paragraph ranges; the 2 of the 26 that were checked lie over
paragraphs split this way. The stale pair of InDesign 11.4 (6,401
elements) matches 6,322; the 79 others are story wrappers whose
paragraph ranges split differently.

Over the same 36 pairs, 95 of the 101 story files and backing stories
that have elements have the same tree of paragraph ranges, character
ranges, elements, text and line breaks as the reference. The text itself
does not depend on the elements: it is the same as when the elements
were left out.

## Not converted

- No node type other than the document node and elements is decoded:
  XML comments, processing instructions and DTDs are not known.
- Elements whose content is a table or a cell (`XMLContent` of a
  table or cell, `NoTextMarker="true"`): they have no markers, and the
  converter does not write them (105 elements in one trustworthy pair).
- IDML splits paragraph ranges where the paragraph formatting that the
  converter reads is the same: in the backing story of the pair above, 37
  of 72 pairs of adjacent paragraph ranges have the same attributes and
  properties. The converter writes such paragraphs in one range, so an
  element over them is written around character ranges instead of
  paragraph ranges.
- Content of other classes: `XMLContent` is left out, with a warning.
