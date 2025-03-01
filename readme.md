<!-- cargo-rdme start -->

# Google Material Design Icons
## Wrapper for the Google Material Design Icons font

[![Crates.io](https://img.shields.io/crates/v/material_design_icons.svg)](https://crates.io/crates/material_design_icons/)
[![Build Status](https://github.com/rscarson/material_design_icons/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/material_design_icons/actions?query=branch%3Amaster)
[![docs.rs](https://img.shields.io/docsrs/material_design_icons)](https://docs.rs/material_design_icons/latest/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/material_design_icons/master/LICENSE)

Each icon has a preview link to the [Google Material Design Icons](https://fonts.google.com/icons) website.

The icons are available in the following styles:
- Sharp: [`material_design_icons::sharp`]
- Outlined: [`material_design_icons::outlined`]
- Rounded: [`material_design_icons::rounded`]

## Usage
Load the font using the [`ICON_FONT`] constant.  
Create an [`Icon`] object with the desired icon name.

[`Icon`] can be converted to `char` or [`String`] using the `From` trait.

If the feature `iced` is enabled, [`Icon`] also implements the `Into<iced::Element>` trait.  
- You will need to include `.font(ICON_FONT)` when creating your iced application.

## Examples

```rust
use material_design_icons::sharp::Icon;
let icon = Icon::Add;
let char = char::from(icon);
let codepoint = icon as u32;
let string = icon.to_string();
```

If you use the `parser` feature, you can load the font and extract the icon data.
```rust
use material_design_icons::sharp::Icon;
use material_design_icons::font::Font;
let font = Font::new_sharp().unwrap();

let index = font.index_of(Icon::Add as u32).unwrap();
let name = font.glyph_name(index).unwrap();
let bitmap = font.bitmap_for(index).unwrap();
```

If you are using `iced`, you can convert the icon to a `Text` widget.
```rust
use material_design_icons::sharp::Icon;

let icon = Icon::Add;
let text = icon.into_text(24);
```

<!-- cargo-rdme end -->
