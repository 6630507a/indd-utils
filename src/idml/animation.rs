//! `MotionPreset`, `TimingSetting` and `AnimationSetting`.
//!
//! Evidence: `docs/format/objects.md`, animation and timing. Values with
//! one value in every sample and no stored field found (`PlaysLoop`,
//! `ReverseAnimation`, `InitiallyHidden`, `HiddenAfter`, the preset
//! locks, `PathOpen`) are written as that value.

use super::*;
use crate::model::{Animation, MotionPreset, TimingList};

/// `TriggerEvent` codes of timing lists.
const TRIGGERS: [(u32, &str); 4] = [
    (1, "OnPageLoad"),
    (5, "OnClick"),
    (6, "OnRelease"),
    (7, "OnRollover"),
];
const DESIGN_OPTIONS: [(u32, &str); 2] = [(0, "FromCurrentAppearance"), (2, "ToCurrentLocation")];
const EASE_TYPES: [(u32, &str); 2] = [(0, "NoEase"), (4, "CustomEase")];
/// `Properties` children of an animation by stored property name, in
/// IDML order.
const PROPERTY_ARRAYS: [(&str, &str); 4] = [
    ("Alpha_Amount", "OpacityArray"),
    ("Rotation_Z", "RotationArray"),
    ("Scale_X", "ScaleXArray"),
    ("Scale_Y", "ScaleYArray"),
];

fn code(table: &[(u32, &'static str)], v: u32) -> Option<&'static str> {
    table.iter().find(|(c, _)| *c == v).map(|(_, n)| *n)
}

/// A number rounded to 6 significant digits, as IDML writes
/// `TransformOffsets`.
fn six_digits(v: f64) -> String {
    num(format!("{v:.5e}").parse().unwrap_or(v))
}

fn point(x: f64, y: f64) -> String {
    format!("{} {}", num(x), num(y))
}

/// CDATA sections holding `s`, split inside any `]]>`.
fn cdata_text(x: &mut Xml, s: &str) {
    let s: String = s.chars().filter(|&c| xml::xml_char(c)).collect();
    let mut parts = s.split("]]>").peekable();
    while let Some(p) = parts.next() {
        if parts.peek().is_some() {
            x.cdata(&format!("{p}]]"), CDATA_SECTION);
            x.cdata(">", CDATA_SECTION);
        } else if !p.is_empty() {
            x.cdata(p, CDATA_SECTION);
        }
    }
}

impl Writer<'_> {
    /// A `MotionPreset` of designmap.xml.
    pub(super) fn motion_preset(x: &mut Xml, p: &MotionPreset) {
        x.start("MotionPreset")
            .attr("Self", uref(Some(p.uid)))
            .attr("Name", p.name.idml())
            .attr("EditLocked", "false")
            .attr("DeleteLocked", "false")
            .attr("NameLocked", "false");
        x.start("Properties").start("Contents");
        cdata_text(x, &p.contents);
        x.end().end().end();
    }

    fn warn(&self, msg: String) {
        self.warnings.borrow_mut().push(msg);
    }

    /// The `TimingSetting` of a spread or button whose `Self` is `owner`;
    /// nothing without timing lists.
    pub(super) fn timing_setting(&self, x: &mut Xml, owner: &str, lists: &[TimingList]) {
        if lists.is_empty() {
            return;
        }
        x.start("TimingSetting");
        for (i, l) in lists.iter().enumerate() {
            let list = format!("{owner}TimingSetting1TimingList{i}");
            x.start("TimingList").attr("Self", &list);
            match code(&TRIGGERS, l.trigger) {
                Some(t) => {
                    x.attr("TriggerEvent", t);
                }
                None => self.warn(format!(
                    "{owner}: trigger code {} is not known; TriggerEvent left out",
                    l.trigger
                )),
            }
            for (j, g) in l.groups.iter().enumerate() {
                let group = format!("{list}TimingGroup{j}");
                x.start("TimingGroup").attr("Self", &group);
                if (1..=100).contains(&g.plays) {
                    x.attr("Plays", g.plays.to_string());
                } else {
                    self.warn(format!("{group}: Plays {} left out", g.plays));
                }
                x.attr("PlaysLoop", "false");
                for (k, t) in g.targets.iter().enumerate() {
                    let target = format!("{group}i{:x}", t.uid);
                    x.start("TimingTarget")
                        .attr("Self", &target)
                        .attr("DynamicTarget", uref(Some(t.uid)));
                    if (0.0..=60.0).contains(&t.delay) {
                        x.attr("DelaySeconds", num(t.delay));
                    } else {
                        self.warn(format!("{target}: DelaySeconds {} left out", t.delay));
                    }
                    x.attr("ReverseAnimation", "false")
                        .attr("TargetRole", t.role.to_string())
                        .attr("TargetAction", t.action.to_string())
                        .attr("Placement", k.to_string())
                        .end();
                }
                x.end();
            }
            x.end();
        }
        x.end();
    }

    /// The `AnimationSetting` of the page item `uid`.
    pub(super) fn animation_setting(&self, x: &mut Xml, uid: u32, a: &Animation) {
        x.start("AnimationSetting").attr(
            "TransformOffsets",
            format!("{} {}", six_digits(a.offsets[0]), six_digits(a.offsets[1])),
        );
        if (0.125..=60.0).contains(&a.duration) {
            x.attr("Duration", num(a.duration));
        } else {
            self.warn(format!(
                "item {uid}: animation Duration {} left out",
                a.duration
            ));
        }
        match code(&DESIGN_OPTIONS, a.design) {
            Some(d) => {
                x.attr("DesignOption", d);
            }
            None => self.warn(format!(
                "item {uid}: animation design option {} is not known; left out",
                a.design
            )),
        }
        match code(&EASE_TYPES, a.ease) {
            Some(e) => {
                x.attr("EaseType", e);
            }
            None => self.warn(format!(
                "item {uid}: animation ease code {} is not known; left out",
                a.ease
            )),
        }
        if (1..=100).contains(&a.plays) {
            x.attr("Plays", a.plays.to_string());
        } else {
            self.warn(format!("item {uid}: animation Plays {} left out", a.plays));
        }
        x.attr("PlaysLoop", "false")
            .attr("InitiallyHidden", "false")
            .attr("HiddenAfter", "false")
            .attr("HasCustomSettings", a.custom.to_string());
        x.start("Properties");
        if a.preset != 0 {
            x.start("Preset")
                .attr("type", "object")
                .text(&uref(Some(a.preset)))
                .end();
        }
        if !a.path.is_empty() {
            x.start("MotionPath");
            for &(frame, px, py) in &a.path {
                let p = point(px, py);
                x.start("AnimationDataPathKeyFrameType")
                    .attr("KeyFrame", frame.to_string());
                x.empty(
                    "PathPoint",
                    &[
                        ("Anchor", p.clone()),
                        ("LeftDirection", p.clone()),
                        ("RightDirection", p),
                    ],
                );
                x.end();
            }
            x.end();
        } else if !a.points.is_empty() {
            x.start("MotionPathPoints")
                .attr("PathOpen", "true")
                .start("PathPointArray");
            for &(px, py) in &a.points {
                let p = point(px, py);
                x.empty(
                    "PathPointType",
                    &[
                        ("Anchor", p.clone()),
                        ("LeftDirection", p.clone()),
                        ("RightDirection", p),
                    ],
                );
            }
            x.end().end();
        }
        for (name, tag) in PROPERTY_ARRAYS {
            let Some((_, keys)) = a.properties.iter().find(|(n, _)| n == name) else {
                continue;
            };
            if keys.is_empty() {
                continue;
            }
            x.start(tag);
            for &(frame, v) in keys {
                x.empty(
                    "AnimationDataKeyFrameType",
                    &[("KeyFrame", frame.to_string()), ("Value", num(v))],
                );
            }
            x.end();
        }
        x.end().end();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{TimingGroup, TimingTarget};

    #[test]
    fn writes_timing_and_animation() {
        let doc = Document::default();
        let w = Writer::for_test(&doc);
        let lists = [TimingList {
            trigger: 1,
            groups: vec![
                TimingGroup {
                    plays: 1,
                    targets: vec![
                        TimingTarget {
                            uid: 0xFE,
                            role: 132355,
                            action: 132353,
                            delay: 0.0,
                        },
                        TimingTarget {
                            uid: 0x608,
                            role: 132355,
                            action: 132353,
                            delay: 0.0,
                        },
                    ],
                },
                TimingGroup {
                    plays: 2,
                    targets: vec![TimingTarget {
                        uid: 0x119,
                        role: 132355,
                        action: 132353,
                        delay: 0.25,
                    }],
                },
            ],
        }];
        let mut x = Xml::new();
        w.timing_setting(&mut x, "uff", &lists);
        let out: String = x.finish().lines().map(str::trim).collect();
        assert!(out.contains(
            "<TimingList Self=\"uffTimingSetting1TimingList0\" TriggerEvent=\"OnPageLoad\">\
             <TimingGroup Self=\"uffTimingSetting1TimingList0TimingGroup0\" Plays=\"1\" PlaysLoop=\"false\">\
             <TimingTarget Self=\"uffTimingSetting1TimingList0TimingGroup0ife\" DynamicTarget=\"ufe\" \
             DelaySeconds=\"0\" ReverseAnimation=\"false\" TargetRole=\"132355\" TargetAction=\"132353\" \
             Placement=\"0\" />"
        ));
        assert!(out.contains(
            "<TimingTarget Self=\"uffTimingSetting1TimingList0TimingGroup1i119\" DynamicTarget=\"u119\" \
             DelaySeconds=\"0.25\" ReverseAnimation=\"false\" TargetRole=\"132355\" TargetAction=\"132353\" \
             Placement=\"0\" />"
        ));

        // One key frame and one property with two keys, as stored.
        let mut d = Vec::new();
        d.extend_from_slice(&1u16.to_le_bytes());
        d.extend_from_slice(&0u16.to_le_bytes());
        d.extend_from_slice(&0x58Eu32.to_le_bytes());
        d.extend_from_slice(&1f64.to_le_bytes());
        for v in [24u32, 24, 0, 0, 1] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        d.extend_from_slice(&0u16.to_le_bytes());
        d.extend_from_slice(&4u32.to_le_bytes());
        d.extend_from_slice(&0.49992400000001f64.to_le_bytes());
        d.extend_from_slice(&0.5f64.to_le_bytes());
        for v in [1u32, 0, 2] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        d.extend_from_slice(&[0; 16]);
        d.extend_from_slice(&0u32.to_le_bytes());
        d.extend_from_slice(&1u32.to_le_bytes());
        for _ in 0..2 {
            d.extend_from_slice(&12u32.to_le_bytes());
            d.extend_from_slice(&[12, 0x40]);
            d.extend_from_slice(b"Alpha_Amount");
        }
        d.extend_from_slice(&2u32.to_le_bytes());
        for (frame, v) in [(0u32, 0f64), (23, 100.0)] {
            d.extend_from_slice(&frame.to_le_bytes());
            d.extend_from_slice(&6u32.to_le_bytes());
            d.extend_from_slice(&v.to_le_bytes());
        }
        let a = Animation::read(&mut crate::Encoding::default().cursor(&d)).unwrap();
        let mut x = Xml::new();
        w.animation_setting(&mut x, 1, &a);
        let out: String = x.finish().lines().map(str::trim).collect();
        assert_eq!(
            out.split_once("?>").map_or("", |(_, b)| b),
            "<AnimationSetting TransformOffsets=\"0.499924 0.5\" Duration=\"1\" \
             DesignOption=\"FromCurrentAppearance\" EaseType=\"CustomEase\" Plays=\"1\" \
             PlaysLoop=\"false\" InitiallyHidden=\"false\" HiddenAfter=\"false\" \
             HasCustomSettings=\"false\"><Properties><Preset type=\"object\">u58e</Preset>\
             <MotionPath><AnimationDataPathKeyFrameType KeyFrame=\"0\">\
             <PathPoint Anchor=\"0 0\" LeftDirection=\"0 0\" RightDirection=\"0 0\" />\
             </AnimationDataPathKeyFrameType></MotionPath><OpacityArray>\
             <AnimationDataKeyFrameType KeyFrame=\"0\" Value=\"0\" />\
             <AnimationDataKeyFrameType KeyFrame=\"23\" Value=\"100\" /></OpacityArray>\
             </Properties></AnimationSetting>"
        );
        // Truncated: no animation.
        for n in 0..d.len() {
            assert!(Animation::read(&mut crate::Encoding::default().cursor(&d[..n])).is_err());
        }
    }
}
