/*
Size of fx a button

We want to specify:
 * exact size
 * fit content
 * fill portion of available space


Along with a max and or min, These max and min should also be specified by on of the above


 */




#[derive(Debug, Clone, Copy, Default)]
pub struct Attributes {
    pub size: Size,
    pub align: Align,
    pub padding: Padding,
    pub spacing: Spacing,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: LengthAttrib,
    pub height: LengthAttrib
}

#[derive(Debug, Clone, Copy)]
pub enum LengthAttrib {
    No(Length),
    Max(Length),
    Min(Length),
    MinMax(Length, Length)
}

impl Default for LengthAttrib {
    fn default() -> Self {
        LengthAttrib::No(Default::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    /// Length equal to given number of pixels
    Px(f32),

    /// Fill all the avialable space
    /// Is equivalent to FillPortion 1
    Fill,

    /// Fill a portion
    /// If 1 child has FillPortion 1 and another has FillPortion 3
    /// Than the first child will have 1/4 of the space and the other child the
    /// remainin 3/4
    FillPortion(u32),

    /// Fit the content
    FitContent
}

impl Default for Length {
    fn default() -> Self {
        Length::Fill
    }
}



/// Attributes can be used to specify how an element renders
#[derive(Debug,Clone,Copy)]
pub enum Attribute {
    Width(LengthAttrib),
    Height(LengthAttrib),
    Padding(f32),
    PaddingXY(f32,f32),
    PaddingEach(Padding),
    Spacing(f32),
    SpacingXY(f32, f32),

}

#[derive(Debug, Clone, Copy, Default)]
pub struct Align {
    pub x: Alignment,
    pub y: Alignment,
}

#[derive(Debug,Clone,Copy)]
pub enum Alignment {
    Left,
    Right,
    Center
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Center
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Spacing {
    pub x: f32,
    pub y: f32,
}
