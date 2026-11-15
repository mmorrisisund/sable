#[derive(Debug, Default, PartialEq)]
pub struct Deg(f32);

impl Deg {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
}

impl From<Deg> for Rad {
    fn from(value: Deg) -> Self {
        Rad(value.0.to_radians())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Rad(f32);

impl Rad {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
}

impl From<Rad> for Deg {
    fn from(value: Rad) -> Self {
        Deg(value.0.to_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deg_to_rad() {
        let deg = Deg(35.0);
        let rad: Rad = deg.into();

        assert!(rad.0 == 0.61086524, "left: {}, right: 0.61086524", rad.0);
    }

    #[test]
    fn rad_to_deg() {
        let rad = Rad(4.0);
        let deg: Deg = rad.into();

        assert!(deg.0 == 229.183_12, "left: {}, right: 229.183_12", deg.0);
    }
}
