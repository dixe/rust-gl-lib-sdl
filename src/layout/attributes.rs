

#[derive(Debug, Clone, Copy, Default)]
pub struct Attributes {
    pub width: Length,
    pub width_contraint: LengthConstraint,
    pub height: Length,
    pub height_contraint: LengthConstraint,
    pub align: Align,
    pub padding: Padding,
    pub spacing: Spacing
}


#[derive(Debug, Clone, Copy)]
pub enum LengthConstraint {
    Unbound,
    Max(u32),
    Min(u32),
    MinMax(u32, u32)
}

impl LengthConstraint {
    pub fn max(&self, default: f32) -> f32 {
        match self {
            LengthConstraint::Unbound => default,
            LengthConstraint::Max(max) => *max as f32,
            LengthConstraint::MinMax(_, max) => *max as f32,
            LengthConstraint::Min(_) => default
        }
    }

    pub fn min(&self, default: f32) -> f32 {
        match self {
            LengthConstraint::Unbound => default,
            LengthConstraint::Max(_) => default,
            LengthConstraint::MinMax(min,_) => *min as f32,
            LengthConstraint::Min(min) => *min as f32
        }
    }
}


impl Default for LengthConstraint {
    fn default() -> Self {
        LengthConstraint::Unbound
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    /// Length equal to given number of pixels
    Px(u32),

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
        Length::FitContent
    }
}



/// Attributes can be used to specify how an element renders
#[derive(Debug,Clone,Copy)]
pub enum Attribute {
    Width(Length),
    WidthConstraint(LengthConstraint),
    Height(Length),
    HeightConstraint(LengthConstraint),
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
