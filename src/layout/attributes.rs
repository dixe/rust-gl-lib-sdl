/*
Size of fx a button

We want to specify:
 * exact size
 * fit content
 * fill portion of available space


Along with a max and or min, These max and min should also be specified by on of the above


 */

#[derive(Debug,Clone,Copy)]
pub struct Size {
    pub width: LengthAttrib,
    pub height: LengthAttrib
}

#[derive(Debug,Clone,Copy)]
pub enum LengthAttrib {
    No(Length),
    Max(Length),
    Min(Length),
    MinMax(Length, Length)
}

#[derive(Debug,Clone,Copy)]
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



pub struct Attributes {
    width: LengthAttrib,
    height: LengthAttrib,
    align: Alignment,
}


pub enum Alignment {
    Left,
    Right,
    Center
}
