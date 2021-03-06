// Copyright 2020 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 96-bit XYZ color space.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::Cmyk;
use crate::Hsl;
use crate::Hsv;
use crate::Rgb;
use crate::utility::cerp_f32;
use crate::utility::clamped;
use crate::utility::lerp_f32;

// External library imports.
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use tracing::Level;
use tracing::span;


// Standard library imports.
use std::convert::From;
use std::fmt;
use std::f32;



////////////////////////////////////////////////////////////////////////////////
// Xyz
////////////////////////////////////////////////////////////////////////////////
/// The encoded XYZ color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Xyz {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
    /// The z component.
    pub z: f32,
}


impl Xyz {
    /// Constructs a new `Xyz` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Xyz::new(0.24, 0.68, 0.91);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let mut xyz = Xyz {x: 0.0, y: 0.0, z: 0.0};
        xyz.set_x(x);
        xyz.set_y(y);
        xyz.set_z(z);
        xyz
    }

    /// Returns the x component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// assert_eq!(color.x(), 0.24);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// assert_eq!(color.y(), 0.68);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Returns the z component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// assert_eq!(color.z(), 0.91);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Sets the x component as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// color.set_x(0.55);
    /// 
    /// assert_eq!(color.x(), 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_x(&mut self, x: f32) {
        self.x = clamped(x, 0.0, 1.0);
    }

    /// Sets the y component as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// color.set_y(0.55);
    /// 
    /// assert_eq!(color.y(), 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    pub fn set_y(&mut self, y: f32) {
        self.y = clamped(y, 0.0, 1.0);
    }

    /// Sets the z component as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Xyz::new(0.24, 0.68, 0.91);
    /// 
    /// color.set_z(0.55);
    /// 
    /// assert_eq!(color.z(), 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    pub fn set_z(&mut self, z: f32) {
        self.z = clamped(z, 0.0, 1.0);
    }

    /// Returns an array containing the `[X, Y, Z]` components.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Xyz::new(0.24, 0.68, 0.91);
    ///
    /// let components = color.components();
    ///
    /// assert_eq!(components, [0.24, 0.68, 0.91]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn components(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Performs a component-wise linear interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Xyz::new(0.24, 0.68, 0.91);
    /// let color_b = Xyz::new(0.84, 0.228, 0.455);
    ///
    /// let lerp_color = Xyz::linear_interpolate(color_a, color_b, 0.19);
    ///
    /// assert_eq!(lerp_color, Xyz::new(0.35399997, 0.59412, 0.82355));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn linear_interpolate<C, D>(start: C, end: D, amount: f32) -> Self 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        Xyz {
            x: lerp_f32(s.x, e.x, amount),
            y: lerp_f32(s.y, e.y, amount),
            z: lerp_f32(s.z, e.z, amount),
        }
    }

    /// Performs a component-wise cubic interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0. The interpolation function will be consistent
    /// with the slopes given by `start_slope` and `end_slope`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Xyz::new(0.24, 0.68, 0.91);
    /// let color_b = Xyz::new(0.84, 0.228, 0.455);
    ///
    /// let lerp_color = Xyz::cubic_interpolate(
    ///     color_a, color_b, 0.0, 0.0, 0.19);
    ///
    /// assert_eq!(lerp_color, Xyz::new(0.29674917, 0.63724893, 0.8669652));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cubic_interpolate<C, D>(
        start: C,
        end: D,
        start_slope: f32,
        end_slope: f32,
        amount: f32) -> Self 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        Xyz {
            x: cerp_f32(s.x, e.x, start_slope, end_slope, amount),
            y: cerp_f32(s.y, e.y, start_slope, end_slope, amount),
            z: cerp_f32(s.z, e.z, start_slope, end_slope, amount),
        }
    }

    /// Returns the distance between the given colors in `Xyz` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Xyz;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Xyz::new(0.24, 0.68, 0.91);
    /// let color_b = Xyz::new(0.84, 0.228, 0.455);
    ///
    /// assert_eq!(Xyz::distance(color_a, color_b), 0.8782534);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn distance<C, D>(start: C, end: D) -> f32 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        
        let x = s.x - e.x;
        let y = s.y - e.y;
        let z = s.z - e.z;

        (x*x + y*y + z*z).sqrt()
    }
}


impl fmt::Display for Xyz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Xyz conversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Xyz {
    fn from(components: [f32; 3]) -> Self {
        let span = span!(Level::DEBUG, "Xyz::from<[f32; 3]>");
        let _enter = span.enter();
        
        Xyz {
            x: components[0],
            y: components[1],
            z: components[2],
        }
    }
}


impl From<Cmyk> for Xyz {
    fn from(cmyk: Cmyk) -> Self {
        let span = span!(Level::DEBUG, "Xyz::from<Cmyk>");
        let _enter = span.enter();
        
        Xyz::from(Rgb::from(cmyk))
    }
}

impl From<Hsl> for Xyz {
    fn from(hsl: Hsl) -> Self {
        let span = span!(Level::DEBUG, "Xyz::from<Hsl>");
        let _enter = span.enter();
        
        Xyz::from(Rgb::from(hsl))
    }
}

impl From<Hsv> for Xyz {
    fn from(hsv: Hsv) -> Self {
        let span = span!(Level::DEBUG, "Xyz::from<Hsv>");
        let _enter = span.enter();
        
        Xyz::from(Rgb::from(hsv))
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        let span = span!(Level::DEBUG, "Xyz::from<Rgb>");
        let _enter = span.enter();
        
        let m = rgb.ratios(); 

        Xyz {
            x: m[0] * 0.4124564 + m[1] * 0.3575761 + m[2] * 0.1804375,
            y: m[0] * 0.2126729 + m[1] * 0.7151522 + m[2] * 0.0721750,
            z: m[0] * 0.0193339 + m[1] * 0.1191920 + m[2] * 0.9503041,
        }
    }
}

