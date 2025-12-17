// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Stylesheet

use simplecss::*;

#[test]
fn style_01() {
    let style = StyleSheet::parse("");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_02() {
    let style = StyleSheet::parse("a {}");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_03() {
    let style = StyleSheet::parse("a { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_04() {
    let style = StyleSheet::parse("/**/");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_05() {
    let style = StyleSheet::parse("a { color:red } /**/");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_06() {
    let style = StyleSheet::parse("a, b { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }\nb { color:red; }");
}

#[test]
fn style_07() {
    let style = StyleSheet::parse("a, { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_08() {
    let style = StyleSheet::parse("a,, { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_09() {
    let style = StyleSheet::parse("a,,b { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }\nb { color:red; }");
}

#[test]
fn style_10() {
    let style = StyleSheet::parse(",a { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_11() {
    let style = StyleSheet::parse("@import \"subs.css\";\na { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_12() {
    let style = StyleSheet::parse(
        "\
@media screen {
    p:before { content: 'Hello'; }
}
a { color:red }",
    );
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_13() {
    let style = StyleSheet::parse("a > { color:red }");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_14() {
    let style = StyleSheet::parse("p { color:green; color }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_15() {
    let style = StyleSheet::parse("p { color; color:green }");
    assert_eq!(style.to_string(), ""); // TODO: should be 'p { color:green; }'
}

#[test]
fn style_16() {
    let style = StyleSheet::parse("p { color:green; color: }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_17() {
    let style = StyleSheet::parse("p { color:green; color:; color:red; }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_18() {
    let style = StyleSheet::parse("p { color:green; color{;color:maroon} }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_19() {
    let style = StyleSheet::parse("p { color{;color:maroon} color:green; }");
    assert_eq!(style.to_string(), ""); // TODO: should be 'p { color:green; }'
}

#[test]
fn style_20() {
    let style = StyleSheet::parse(
        "\
        h1 { color: green }
        h2 & h3 { color: red }
        h4 { color: black }
    ",
    );
    assert_eq!(
        style.to_string(),
        "h1 { color:green; }\nh4 { color:black; }"
    );
}

#[test]
fn style_21() {
    let style = StyleSheet::parse(":le>*");
    assert_eq!(style.to_string(), "");
}

#[test]
fn font_face_01() {
    let style = StyleSheet::parse(
        "@font-face { font-family: 'Noto Serif'; src: url(NotoSerif.woff2) format('woff2'); }",
    );

    assert_eq!(style.rules.len(), 0);
    assert_eq!(style.font_faces.len(), 1);

    let ff = &style.font_faces[0];
    assert_eq!(
        ff.declarations,
        vec![
            Declaration {
                name: "font-family",
                value: "'Noto Serif'",
                important: false,
            },
            Declaration {
                name: "src",
                value: "url(NotoSerif.woff2) format('woff2')",
                important: false,
            },
        ]
    );
}

#[test]
fn font_face_02_mixed_with_rules() {
    let style = StyleSheet::parse(
        "@font-face { font-family: 'MyFont'; src: url(https://foo.com/my.woff2); font-weight: normal; } div { color: red; }",
    );

    assert_eq!(style.rules.len(), 1);
    assert_eq!(style.font_faces.len(), 1);

    assert_eq!(style.rules[0].selector.to_string(), "div");
    assert_eq!(style.rules[0].declarations.len(), 1);
    assert_eq!(style.rules[0].declarations[0].name, "color");
    assert_eq!(style.rules[0].declarations[0].value, "red");

    assert_eq!(style.font_faces[0].declarations[0].name, "font-family");
    assert_eq!(style.font_faces[0].declarations[0].value, "'MyFont'");
    assert_eq!(style.font_faces[0].declarations[1].name, "src");
    assert_eq!(
        style.font_faces[0].declarations[1].value,
        "url(https://foo.com/my.woff2)"
    );
    assert_eq!(style.font_faces[0].declarations[2].name, "font-weight");
    assert_eq!(style.font_faces[0].declarations[2].value, "normal");
}

#[test]
fn font_face_03_mixed_with_rules() {
    let style = StyleSheet::parse(
        "@font-palette-values --identifier { font-family: Bixa; override-colors: 0 green, 1 #999; } div { color: red; } @font-face { font-family: 'MyFont'; src: local('Airal'); font-weight: 200 800; }",
    );

    assert_eq!(style.rules.len(), 1);
    assert_eq!(style.font_faces.len(), 1);

    assert_eq!(style.rules[0].selector.to_string(), "div");
    assert_eq!(style.rules[0].declarations.len(), 1);
    assert_eq!(style.rules[0].declarations[0].name, "color");
    assert_eq!(style.rules[0].declarations[0].value, "red");

    assert_eq!(style.font_faces[0].declarations[0].name, "font-family");
    assert_eq!(style.font_faces[0].declarations[0].value, "'MyFont'");
    assert_eq!(style.font_faces[0].declarations[1].name, "src");
    assert_eq!(style.font_faces[0].declarations[1].value, "local('Airal')");
    assert_eq!(style.font_faces[0].declarations[2].name, "font-weight");
    assert_eq!(style.font_faces[0].declarations[2].value, "200 800");
}
