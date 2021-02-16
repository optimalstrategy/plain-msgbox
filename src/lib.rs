//! A crate for generating plain message boxes like the one below (formatting is manual):
//!
//! ```bash
//! ╭────────────────────────────────╮
//! │ Call stack size:     1024      │
//! │ Interning threshold: 20        │
//! │ Optimization level:  1         │
//! │ Optimizations:                 │
//! │   Constant Folding:       true │
//! │   Peephole Optimizations: true │
//! │   Tail Call Optimization: true │
//! │   Dead Code Elimination:  true │
//! │ (misc): cfg_export = false     │
//! │ (misc): caching    = true      │
//! <Config>─────────────────────────╯
//! ```
//!
//! # Usage
//! Add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! plain_msgbox = { git = "https://github.com/optimalstrategy/plain-msgbox" }
//! ```
//!
//! Then generate some boxes:
//!
//! ```rust
//! use plain_msgbox::generate_box;
//!
//! let msg = generate_box(&[
//!     format!("A vec:    {:?}", vec![1, 2, 3]),
//!     format!("A tuple:  {:?}", (1, 2, 3)),
//!     format!("A string: {}", "abcdefghi"),
//! ]);
//! assert_eq!(msg, "\
//! ╭─────────────────────╮
//! │ A vec:    [1, 2, 3] │
//! │ A tuple:  (1, 2, 3) │
//! │ A string: abcdefghi │
//! ╰─────────────────────╯");
//! ```

/// Generate a new message box using the provided lines.
///
/// ```
/// # use plain_msgbox::*;
///  let msg = generate_box(&[
///      format!("2015 is {:#b} in binary!", 2015),
///      format!("2018 is {:#o} in octal!", 2018),
///      format!("2021 is {:#x} in hex!", 2021),
///  ]);
///
///  assert_eq!(msg, "\
/// ╭──────────────────────────────────╮
/// │ 2015 is 0b11111011111 in binary! │
/// │ 2018 is 0o3742 in octal!         │
/// │ 2021 is 0x7e5 in hex!            │
/// ╰──────────────────────────────────╯");
/// ```
#[inline]
pub fn generate_box(lines: &[String]) -> String {
    generate_with_config(lines, Default::default())
}

/// Generate a new message box with the given caption on the last line.
///
/// ```
/// # use plain_msgbox::*;
///  let msg = generate_with_caption(&[
///      format!("2015 is {:#b} in binary!", 2015),
///      format!("2018 is {:#o} in octal!", 2018),
///      format!("2021 is {:#x} in hex!", 2021),
///  ], "Rust Editions");
///
///  assert_eq!(msg, "\
/// ╭──────────────────────────────────╮
/// │ 2015 is 0b11111011111 in binary! │
/// │ 2018 is 0o3742 in octal!         │
/// │ 2021 is 0x7e5 in hex!            │
/// <Rust Editions>────────────────────╯");
/// ```
#[inline]
pub fn generate_with_caption(lines: &[String], last_line_caption: &str) -> String {
    generate_with_config(
        lines,
        TextBoxConfig {
            last_line_caption: Some(last_line_caption),
            ..Default::default()
        },
    )
}

/// Generate a new message box according the given config.
///
/// ```
/// # use plain_msgbox::*;
///  let msg = generate_with_config(&[
///      String::from("Function Name: generate_with_config"),
///      format!("Address: {:p}", 0x55e7d53f0860 as *const ()),
///  ], TextBoxConfig::dos().with_caption("Fn Info"));
///
///  assert_eq!(msg, "\
/// ╔═════════════════════════════════════╗
/// ║ Function Name: generate_with_config ║
/// ║ Address: 0x55e7d53f0860             ║
/// <Fn Info>═════════════════════════════╝");
/// ```
pub fn generate_with_config(lines: &[String], config: TextBoxConfig<'_>) -> String {
    let longest_line = lines.iter().map(|r| r.len()).max().unwrap_or(0);

    let longest_line = config
        .last_line_caption
        .map(str::len)
        .unwrap_or(0)
        .max(longest_line);

    let mut result = vec![
        config.left_top_corner.to_owned()
            + &config.horizontal_bar.repeat(longest_line + 2)
            + config.right_top_corner,
    ];
    result.reserve(lines.len() + 1);

    for line in lines {
        let spaces = " ".repeat(longest_line - line.len());
        result.push(format!(
            "{} {}{} {}",
            config.vertical_bar, line, spaces, config.vertical_bar
        ));
    }

    if let Some(caption) = config.last_line_caption {
        result.push(format!(
            "<{}>{}{}",
            caption,
            config
                .horizontal_bar
                .repeat(longest_line - caption.len() + 1),
            config.right_bottom_corner
        ));
    } else {
        result.push(
            config.left_bottom_corner.to_owned()
                + &config.horizontal_bar.repeat(longest_line + 2)
                + config.right_bottom_corner,
        );
    }

    result.join("\n")
}

/// Configure the last line caption and the box drawing characters.
#[derive(Debug, Clone)]
pub struct TextBoxConfig<'a> {
    /// The character to use as the horizontal bar.
    pub horizontal_bar: &'a str,
    /// The character to use as the vertical bor.
    pub vertical_bar: &'a str,
    /// The character to use for the left top corner.
    pub left_top_corner: &'a str,
    /// The character to use for the left bottom corner.
    pub left_bottom_corner: &'a str,
    /// The character to use for the right top corner.
    pub right_top_corner: &'a str,
    /// The character to use for the right bottom corner.
    pub right_bottom_corner: &'a str,
    /// The caption displayed on the last line of the box.
    pub last_line_caption: Option<&'a str>,
}

impl<'a> TextBoxConfig<'a> {
    /// Create a DOS-styled text box config.
    pub fn dos() -> Self {
        Self {
            horizontal_bar: dos::DOS_HORIZONTAL_BAR,
            vertical_bar: dos::DOS_VERTICAL_BAR,
            left_top_corner: dos::DOS_LEFT_TOP_CORNER,
            left_bottom_corner: dos::DOS_LEFT_BOTTOM_CORNER,
            right_top_corner: dos::DOS_RIGHT_TOP_CORNER,
            right_bottom_corner: dos::DOS_RIGHT_BOTTOM_CORNER,
            last_line_caption: None,
        }
    }

    /// Add the given caption to the current text box configuration.
    pub fn with_caption(self, caption: &'a str) -> Self {
        Self {
            last_line_caption: Some(caption),
            ..self
        }
    }
}

impl<'a> Default for TextBoxConfig<'a> {
    fn default() -> Self {
        Self {
            horizontal_bar: default::DEFAULT_HORIZONTAL_BAR,
            vertical_bar: default::DEFAULT_VERTICAL_BAR,
            left_top_corner: default::DEFAULT_LEFT_TOP_CORNER,
            left_bottom_corner: default::DEFAULT_LEFT_BOTTOM_CORNER,
            right_top_corner: default::DEFAULT_RIGHT_TOP_CORNER,
            right_bottom_corner: default::DEFAULT_RIGHT_BOTTOM_CORNER,
            last_line_caption: None,
        }
    }
}

/// Contains the default box characters.
pub mod default {
    /// The default horizontal bar character `─`.
    pub static DEFAULT_HORIZONTAL_BAR: &str = "─";
    /// The default vertical bar character `│`.
    pub static DEFAULT_VERTICAL_BAR: &str = "│";
    /// The default left top corner character `╭`.
    pub static DEFAULT_LEFT_TOP_CORNER: &str = "╭";
    /// The default left bottom corner character `╰`.
    pub static DEFAULT_LEFT_BOTTOM_CORNER: &str = "╰";
    /// The default right top corner character `╮`.
    pub static DEFAULT_RIGHT_TOP_CORNER: &str = "╮";
    /// The default right bottom corner character `╯`.
    pub static DEFAULT_RIGHT_BOTTOM_CORNER: &str = "╯";
}

/// Contains DOS-styled box characters.
pub mod dos {
    /// The DOS horizontal bar character `═`.
    pub static DOS_HORIZONTAL_BAR: &str = "═";
    /// The DOS vertical bar character `║`.
    pub static DOS_VERTICAL_BAR: &str = "║";
    /// The DOS left top corner character `╔`.
    pub static DOS_LEFT_TOP_CORNER: &str = "╔";
    /// The DOS left bottom corner character `╚`.
    pub static DOS_LEFT_BOTTOM_CORNER: &str = "╚";
    /// The DOS right top corner character `╗`.
    pub static DOS_RIGHT_TOP_CORNER: &str = "╗";
    /// The DOS right bottom corner character `╝`.
    pub static DOS_RIGHT_BOTTOM_CORNER: &str = "╝";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_boxes() {
        let completely_empty = generate_box(&[]);
        assert_eq!(completely_empty, "╭──╮\n╰──╯");

        let empty_with_caption = generate_with_caption(&[], "a super long caption");
        assert_eq!(
            empty_with_caption,
            "\
╭──────────────────────╮
<a super long caption>─╯"
        );

        let msgbox1 = generate_box(&[String::new()]);
        assert_eq!(
            msgbox1,
            "\
╭──╮
│  │
╰──╯"
        );

        let msgbox2 = generate_box(&[
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]);

        assert_eq!(
            msgbox2,
            "\
╭──╮
│  │
│  │
│  │
│  │
│  │
╰──╯"
        );
    }

    #[test]
    fn test_message_box() {
        let msgbox = generate_box(&[
            "Line 1:              ".to_string(),
            "                       line 2".to_string(),
            "abc".to_string(),
            "".to_string(),
            "42".to_string(),
        ]);

        assert_eq!(
            msgbox,
            "\
╭───────────────────────────────╮
│ Line 1:                       │
│                        line 2 │
│ abc                           │
│                               │
│ 42                            │
╰───────────────────────────────╯"
        );
    }

    #[test]
    fn test_message_box_caption() {
        let msgbox = generate_with_caption(
            &[
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit".to_string(),
                "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua".to_string(),
                "Ut enim ad minim veniam".to_string(),
                "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat".to_string(),
                "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur".to_string(),
            ],
            "Lorem ipsum",
        );

        println!("{}", msgbox);

        assert_eq!(
            msgbox,
            "\
╭───────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ Lorem ipsum dolor sit amet, consectetur adipiscing elit                                               │
│ sed do eiusmod tempor incididunt ut labore et dolore magna aliqua                                     │
│ Ut enim ad minim veniam                                                                               │
│ quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat                     │
│ Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur │
<Lorem ipsum>───────────────────────────────────────────────────────────────────────────────────────────╯"
        );
    }

    #[test]
    fn test_message_box_config() {
        let msgbox = generate_with_config(
            &[
                "The Rustonomicon. The Dark Arts of Unsafe Rust.".to_string(),
                "THE KNOWLEDGE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND OR IMPLIED, INCLUDING BUT NOT LIMITED TO".to_string(),
                "THE WARRANTIES OF UNLEASHING INDESCRIBABLE HORRORS THAT SHATTER YOUR PSYCHE AND SET YOUR MIND ADRIFT IN ".to_string(),
                "THE UNKNOWABLY INFINITE COSMOS.".to_string(),
            ],
            TextBoxConfig::dos(),
        );

        assert_eq!(msgbox,
            "\
╔══════════════════════════════════════════════════════════════════════════════════════════════════════════╗
║ The Rustonomicon. The Dark Arts of Unsafe Rust.                                                          ║
║ THE KNOWLEDGE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND OR IMPLIED, INCLUDING BUT NOT LIMITED TO ║
║ THE WARRANTIES OF UNLEASHING INDESCRIBABLE HORRORS THAT SHATTER YOUR PSYCHE AND SET YOUR MIND ADRIFT IN  ║
║ THE UNKNOWABLY INFINITE COSMOS.                                                                          ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════════════╝"
)
    }
}
