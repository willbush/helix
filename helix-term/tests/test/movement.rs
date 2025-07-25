use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn insert_mode_cursor_position() -> anyhow::Result<()> {
    test(TestCase {
        in_text: String::new(),
        in_selection: Selection::single(0, 0),
        in_keys: "i".into(),
        out_text: String::new(),
        out_selection: Selection::single(0, 0),
        line_feed_handling: LineFeedHandling::AsIs,
    })
    .await?;

    test(("#[\n|]#", "i", "#[|\n]#")).await?;
    test(("#[\n|]#", "i<esc>", "#[|\n]#")).await?;
    test(("#[\n|]#", "i<esc>i", "#[|\n]#")).await?;

    Ok(())
}

/// Range direction is preserved when escaping insert mode to normal
#[tokio::test(flavor = "multi_thread")]
async fn insert_to_normal_mode_cursor_position() -> anyhow::Result<()> {
    test(("#[f|]#oo\n", "vll<A-;><esc>", "#[|foo]#\n")).await?;
    test((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "vll<A-;><esc>",
        indoc! {"\
                #[|foo]#
                #(|bar)#"
        },
    ))
    .await?;

    test((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "a",
        indoc! {"\
                #[fo|]#o
                #(ba|)#r"
        },
    ))
    .await?;

    test((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "a<esc>",
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_by_character() -> anyhow::Result<()> {
    // Only pairs matching the passed character count
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi{",
        "(so [many {#[good|]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi[",
        "(so [#[many {good} text|]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi(",
        "(#[so [many {good} text] here|]#)",
    ))
    .await?;

    // Works with characters that aren't pairs too
    test((
        "'so 'many 'go#[o|]#d' text' here'",
        "mi'",
        "'so 'many '#[good|]#' text' here'",
    ))
    .await?;
    test((
        "'so 'many 'go#[o|]#d' text' here'",
        "2mi'",
        "'so '#[many 'good' text|]#' here'",
    ))
    .await?;
    test((
        "'so \"many 'go#[o|]#d' text\" here'",
        "mi\"",
        "'so \"#[many 'good' text|]#\" here'",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mi{",
        "(so [many {#[|good]#} text] here)",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_inside_pair() -> anyhow::Result<()> {
    // Works at first character of buffer
    // TODO: Adjust test when opening pair failure is fixed
    test(("#[(|]#something)", "mim", "#[(|]#something)")).await?;

    // Inside a valid pair selects pair
    test(("some (#[t|]#ext) here", "mim", "some (#[text|]#) here")).await?;

    // On pair character selects pair
    // TODO: Opening pair character is a known failure case that needs addressing
    // test(("some #[(|]#text) here", "mim", "some (#[text|]#) here")).await?;
    test(("some (text#[)|]# here", "mim", "some (#[text|]#) here")).await?;

    // No valid pair does nothing
    test(("so#[m|]#e (text) here", "mim", "so#[m|]#e (text) here")).await?;

    // Count skips to outer pairs
    test((
        "(so (many (go#[o|]#d) text) here)",
        "1mim",
        "(so (many (#[good|]#) text) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "2mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "3mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;

    // Matching pairs outside selection don't match
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "mim",
        "((so)(#[(many) good (text)|]#)(here))",
    ))
    .await?;
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "2mim",
        "(#[(so)((many) good (text))(here)|]#)",
    ))
    .await?;

    // Works with mixed braces
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mim",
        "(so [many {#[good|]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "2mim",
        "(so [#[many {good} text|]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "3mim",
        "(#[so [many {good} text] here|]#)",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mim",
        "(so [many {#[|good]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "2mim",
        "(so [#[|many {good} text]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "3mim",
        "(#[|so [many {good} text] here]#)",
    ))
    .await?;

    // Only pairs outside of full selection range are considered
    test((
        "(so (many (go#[od) |]#text) here)",
        "mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so (many#[ (go|]#od) text) here)",
        "mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so#[ (many (go|]#od) text) here)",
        "mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;
    test((
        "(so (many (go#[od) text) |]#here)",
        "mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;

    // Works with multiple cursors
    test((
        "(so (many (good) text) #[he|]#re\nso (many (good) text) #(|he)#re)",
        "mim",
        "(#[so (many (good) text) here\nso (many (good) text) here|]#)",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_around_pair() -> anyhow::Result<()> {
    // Works at first character of buffer
    // TODO: Adjust test when opening pair failure is fixed
    test(("#[(|]#something)", "mam", "#[(|]#something)")).await?;

    // Inside a valid pair selects pair
    test(("some (#[t|]#ext) here", "mam", "some #[(text)|]# here")).await?;

    // On pair character selects pair
    // TODO: Opening pair character is a known failure case that needs addressing
    // test(("some #[(|]#text) here", "mam", "some #[(text)|]# here")).await?;
    test(("some (text#[)|]# here", "mam", "some #[(text)|]# here")).await?;

    // No valid pair does nothing
    test(("so#[m|]#e (text) here", "mam", "so#[m|]#e (text) here")).await?;

    // Count skips to outer pairs
    test((
        "(so (many (go#[o|]#d) text) here)",
        "1mam",
        "(so (many #[(good)|]# text) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "2mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "3mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;

    // Matching pairs outside selection don't match
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "mam",
        "((so)#[((many) good (text))|]#(here))",
    ))
    .await?;
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "2mam",
        "#[((so)((many) good (text))(here))|]#",
    ))
    .await?;

    // Works with mixed braces
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mam",
        "(so [many #[{good}|]# text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "2mam",
        "(so #[[many {good} text]|]# here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "3mam",
        "#[(so [many {good} text] here)|]#",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mam",
        "(so [many #[|{good}]# text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "2mam",
        "(so #[|[many {good} text]]# here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "3mam",
        "#[|(so [many {good} text] here)]#",
    ))
    .await?;

    // Only pairs outside of full selection range are considered
    test((
        "(so (many (go#[od) |]#text) here)",
        "mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so (many#[ (go|]#od) text) here)",
        "mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so#[ (many (go|]#od) text) here)",
        "mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;
    test((
        "(so (many (go#[od) text) |]#here)",
        "mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;

    // Works with multiple cursors
    test((
        "(so (many (good) text) #[he|]#re\nso (many (good) text) #(|he)#re)",
        "mam",
        "#[(so (many (good) text) here\nso (many (good) text) here)|]#",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn match_around_closest_ts() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r#"fn main() {testing!{"f#[|oo]#)"};}"#,
            "mam",
            r#"fn main() {testing!{#[|"foo)"]#};}"#,
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r##"fn main() { let _ = ("#[|1]#23", "#(|1)#23"); } "##,
            "3mam",
            r##"fn main() #[|{ let _ = ("123", "123"); }]# "##,
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r##" fn main() { let _ = ("12#[|3", "12]#3"); } "##,
            "1mam",
            r##" fn main() { let _ = #[|("123", "123")]#; } "##,
        ),
    )
    .await?;

    Ok(())
}

/// Ensure the very initial cursor in an opened file is the width of
/// the first grapheme
#[tokio::test(flavor = "multi_thread")]
async fn cursor_position_newly_opened_file() -> anyhow::Result<()> {
    let test = |content: &str, expected_sel: Selection| -> anyhow::Result<()> {
        let file = helpers::temp_file_with_contents(content)?;
        let mut app = helpers::AppBuilder::new()
            .with_file(file.path(), None)
            .build()?;

        let (view, doc) = helix_view::current!(app.editor);
        let sel = doc.selection(view.id).clone();
        assert_eq!(expected_sel, sel);

        Ok(())
    };

    test("foo", Selection::single(0, 1))?;
    test("👨‍👩‍👧‍👦 foo", Selection::single(0, 7))?;
    test("", Selection::single(0, 0))?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn cursor_position_append_eof() -> anyhow::Result<()> {
    // Selection is forwards
    test(("#[foo|]#", "abar<esc>", "#[foobar|]#\n")).await?;

    // Selection is backwards
    test(("#[|foo]#", "abar<esc>", "#[foobar|]#\n")).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_next_function_is_union_of_objects() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                #[/|]#// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
            "},
            "]fv]f",
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }|]#
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_prev_function_unselects_object() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }|]#
            "},
            "v[f",
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }|]#
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_prev_function_goes_backwards_to_object() -> anyhow::Result<()> {
    // Note: the anchor stays put and the head moves back.
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                #[fn ident(x: usize) -> usize { x }|]#
            "},
            "v[f",
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                #[|fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                ]#fn ident(x: usize) -> usize { x }
            "},
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                #[fn ident(x: usize) -> usize { x }|]#
            "},
            "v[f[f",
            indoc! {"\
                /// Increments
                #[|fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                ]#fn ident(x: usize) -> usize { x }
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn find_char_line_ending() -> anyhow::Result<()> {
    test((
        indoc! {
            "\
            one
            #[|t]#wo
            three"
        },
        "T<ret>gll2f<ret>",
        indoc! {
            "\
            one
            two#[
            |]#three"
        },
    ))
    .await?;

    test((
        indoc! {
            "\
            #[|o]#ne
            two
            three"
        },
        "f<ret>2t<ret>ghT<ret>F<ret>",
        indoc! {
            "\
            one#[|
            t]#wo
            three"
        },
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_surround_replace() -> anyhow::Result<()> {
    test((
        indoc! {"\
            (#[|a]#)
            "},
        "mrm{",
        indoc! {"\
            {#[|a]#}
            "},
    ))
    .await?;

    test((
        indoc! {"\
            (#[a|]#)
            "},
        "mrm{",
        indoc! {"\
            {#[a|]#}
            "},
    ))
    .await?;

    test((
        indoc! {"\
            {{

            #(}|)#
            #[}|]#
            "},
        "mrm)",
        indoc! {"\
            ((

            #()|)#
            #[)|]#
            "},
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_surround_delete() -> anyhow::Result<()> {
    test((
        indoc! {"\
            (#[|a]#)
            "},
        "mdm",
        indoc! {"\
            #[|a]#
            "},
    ))
    .await?;

    test((
        indoc! {"\
            (#[a|]#)
            "},
        "mdm",
        indoc! {"\
            #[a|]#
            "},
    ))
    .await?;

    test((
        indoc! {"\
            {{

            #(}|)#
            #[}|]#
            "},
        "mdm",
        "\n\n#(\n|)##[\n|]#",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tree_sitter_motions_work_across_injections() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "<A-o>",
            "<script>let #[|x = 1]#;</script>",
        ),
    )
    .await?;

    // When the full injected layer is selected, expand_selection jumps to
    // a more shallow layer.
    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>#[|let x = 1;]#</script>",
            "<A-o>",
            "#[|<script>let x = 1;</script>]#",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x = 1]#;</script>",
            "<A-i>",
            "<script>let #[|x]# = 1;</script>",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "<A-n>",
            "<script>let x #[=|]# 1;</script>",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "<A-p>",
            "<script>#[|let]# x = 1;</script>",
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_find_repeat_motion_forward() -> anyhow::Result<()> {
    // Single repeat
    test(("#[|f]#oo bar baz bax", "fb<A-.>", "foo #[bar b|]#az bax")).await?;

    // Double repeat
    test((
        "#[|f]#oo bar baz bax",
        "fb<A-.><A-.>",
        "foo bar #[baz b|]#ax",
    ))
    .await?;
    // Same test, but using count prefix.
    test(("#[|f]#oo bar baz bax", "fb2<A-.>", "foo bar #[baz b|]#ax")).await?;

    // Start in normal mode, find, switch to select mode, then repeat
    test(("#[|f]#oo bar baz bax", "fbv<A-.>", "#[foo bar b|]#az bax")).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_find_repeat_motion_reverse() -> anyhow::Result<()> {
    // Single reverse after double forward
    test((
        "#[|f]#oo bar baz bax",
        "fb2<A-.><A-gt>",
        "foo bar #[|baz b]#ax",
    ))
    .await?;
    // Double reverse after double forward
    test((
        "#[|f]#oo bar baz bax",
        "fb<A-.><A-.><A-gt><A-gt>",
        "foo #[|bar b]#az bax",
    ))
    .await?;
    // Same as above but using counts
    test((
        "#[|f]#oo bar baz bax",
        "fb2<A-.>2<A-gt>",
        "foo #[|bar b]#az bax",
    ))
    .await?;

    // Start in normal mode, find, switch to select mode, then repeat twice, then reverse.
    test((
        "#[|f]#oo bar baz bax",
        "fbv2<A-.><A-gt>",
        "#[foo bar b|]#az bax",
    ))
    .await?;

    Ok(())
}

/// The `find` / `till` on `<ret>` has its special handling so test it specifically.
#[tokio::test(flavor = "multi_thread")]
async fn test_find_till_return_key() -> anyhow::Result<()> {
    test(("#[|h]#ello\nworld\n", "f<ret>", "#[hello\n|]#world\n")).await?;
    test(("#[|h]#ello\nworld\n", "t<ret>", "#[hello|]#\nworld\n")).await?;

    // with one repeat
    test(("#[|h]#ello\nworld\n", "f<ret><A-.>", "hello#[\nworld\n|]#")).await?;
    test(("#[|h]#ello\nworld\n", "t<ret><A-.>", "hell#[o\nworld|]#\n")).await?;

    // test transition from normal to select mode.
    test((
        "#[|0]#0\n11\n22\n33\n",
        "f<ret><A-.>v<A-.>",
        "00#[\n11\n22\n|]#33\n",
    ))
    .await?;
    Ok(())
}
