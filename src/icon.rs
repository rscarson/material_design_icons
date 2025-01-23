/// Macro implementing the `Icon` features for all the auto-extracted font data
macro_rules! impl_icon {
    ($name:ident, $font_name:literal) => {
        impl From<$name> for char {
            fn from(value: $name) -> Self {
                // Safety: All codepoints are google-provided and should be valid
                std::char::from_u32(value as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", char::from(*self))
            }
        }

        #[cfg(feature = "iced")]
        impl<'a, Message> From<$name> for iced::Element<'a, Message> {
            fn from(value: $name) -> Self {
                value
                    .into_text(iced::settings::Settings::default().default_text_size)
                    .into()
            }
        }

        impl $name {
            /// Convert the icon to an iced Text widget
            #[cfg(feature = "iced")]
            pub fn into_text<'a, Theme>(
                self,
                font_size: impl Into<iced::Pixels>,
            ) -> iced::widget::Text<'a, Theme>
            where
                Theme: iced::widget::text::Catalog,
            {
                const ICON_FONT: iced::font::Family = iced::font::Family::Name($font_name);
                iced::widget::Text::new(char::from(self))
                    .font(iced::font::Font {
                        family: ICON_FONT,
                        ..Default::default()
                    })
                    .size(font_size)
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::sharp::Icon;

    #[test]
    #[cfg(feature = "iced")]
    fn test_into_text() {
        let icon = Icon::Add;
        let _: iced::widget::Text = icon.into_text(24);
    }

    #[test]
    fn test_icon() {
        let icon = Icon::Add;
        let _ = char::from(icon);
        let _ = icon as u32;
        icon.to_string();
    }
}
