Size of fx a button

    We want to specify:
* exact size
    * fit content
    * fill portion of available space


    Along with a max and or min, These max and min should also be specified by on of the above



    #[derive(Debug,Clone,Copy)]
pub enum LengthContraint {
    No(Length),
    Max(Length),
    Min(Length),
    MinMax(Length, Length)
}

#[derive(Debug,Clone,Copy)]
pub enum Length {
    /// Length equal to given number of pixels
    Px(u32),

    /// Fill all the avialable space
    Fill,

    /// Fill a portion [0;1.0] of the available space
    FillPortion(f32),

    /// Fit the content
    FitContent
}
