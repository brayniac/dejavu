//! Library providing DejaVu font family for embedding

/// Includes the DejaVu Sans variants
pub mod sans {
    /// DejaVu Sans Regular
    pub fn regular() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSans.ttf")
    }

    /// DejaVu Sans Bold
    pub fn bold() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSans-Bold.ttf")
    }

    /// DejaVu Sans Bold Oblique
    pub fn bold_oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSans-BoldOblique.ttf")
    }

    /// DejaVu Sans Extra Light
    pub fn extra_light() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSans-ExtraLight.ttf")
    }

    /// DejaVu Sans Oblique
    pub fn oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSans-Oblique.ttf")
    }
}

/// Includes the DejaVu Sans Condensed variants
pub mod sans_condensed {
    /// DejaVu Sans Condensed Regular
    pub fn regular() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansCondensed.ttf")
    }

    /// DejaVu Sans Condensed Bold
    pub fn bold() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansCondensed-Bold.ttf")
    }

    /// DejaVu Sans Condensed Bold Oblique
    pub fn bold_oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansCondensed-BoldOblique.ttf")
    }

    /// DejaVu Sans Condensed Oblique
    pub fn oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansCondensed-Oblique.ttf")
    }
}

/// Includes the DejaVu Sans Mono variants
pub mod sans_mono {
    /// DejaVu Sans Mono Regular
    pub fn regular() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono.ttf")
    }

    /// DejaVu Sans Mono  Bold
    pub fn bold() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-Bold.ttf")
    }

    /// DejaVu Sans Mono Bold Oblique
    pub fn bold_oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-BoldOblique.ttf")
    }

    /// DejaVu Sans Mono Oblique
    pub fn oblique() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-Oblique.ttf")
    }
}

/// Includes the DejaVu Serif variants
pub mod serif {
    /// DejaVu Serif Regular
    pub fn regular() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerif.ttf")
    }

    /// DejaVu Serif Bold
    pub fn bold() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerif-Bold.ttf")
    }

    /// DejaVu Serif Bold Italic
    pub fn bold_italic() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerif-BoldItalic.ttf")
    }

    /// DejaVu Serif Italic
    pub fn italic() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerif-Italic.ttf")
    }
}

/// Includes the DejaVu Serif Condensed variants
pub mod serif_condensed {
    /// DejaVu Serif Condensed Regular
    pub fn regular() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerifCondensed.ttf")
    }

    /// DejaVu Serif Condensed Bold
    pub fn bold() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerifCondensed-Bold.ttf")
    }

    /// DejaVu Serif Condensed Bold Italic
    pub fn bold_italic() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerifCondensed-BoldItalic.ttf")
    }

    /// DejaVu Serif Condensed Italic
    pub fn italic() -> &'static [u8] {
        include_bytes!("../assets/dejavu-fonts-ttf-2.37/ttf/DejaVuSerifCondensed-Italic.ttf")
    }
}
