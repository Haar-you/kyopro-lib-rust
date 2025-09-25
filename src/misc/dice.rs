//! 6面サイコロ

/// 6面サイコロ
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dice<T> {
    /// 上の面
    pub top: T,
    /// 下の面
    pub bottom: T,
    /// 前の面
    pub front: T,
    /// 後ろの面
    pub back: T,
    /// 右の面
    pub right: T,
    /// 左の面
    pub left: T,
}

impl<T> Dice<T>
where
    T: Clone,
{
    /// [`Dice<T>`]を生成する。
    pub fn new(top: T, bottom: T, front: T, back: T, right: T, left: T) -> Self {
        Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        }
    }

    /// サイコロを左の面が下になるように回転する。
    pub fn rot_left(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(right, left, front, back, bottom, top)
    }

    /// サイコロを右の面が下になるように回転する。
    pub fn rot_right(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(left, right, front, back, top, bottom)
    }

    /// サイコロを前の面が下になるように回転する。
    pub fn rot_front(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(back, front, top, bottom, right, left)
    }

    /// サイコロを後ろの面が下になるように回転する。
    pub fn rot_back(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(front, back, bottom, top, right, left)
    }

    /// サイコロを上からみて時計回りに回転する。
    pub fn rot_clockwise(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(top, bottom, right, left, back, front)
    }

    /// サイコロを上からみて反時計回りに回転する。
    pub fn rot_counterclockwise(&self) -> Self {
        let Self {
            top,
            bottom,
            front,
            back,
            right,
            left,
        } = self.clone();
        Self::new(top, bottom, left, right, front, back)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dice = Dice::new(1, 6, 2, 5, 3, 4);

        assert_eq!(dice.rot_right(), dice.rot_left().rot_left().rot_left());
        assert_eq!(dice.rot_front(), dice.rot_back().rot_back().rot_back());
        assert_eq!(
            dice.rot_clockwise(),
            dice.rot_counterclockwise()
                .rot_counterclockwise()
                .rot_counterclockwise()
        )
    }
}
