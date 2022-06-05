pub trait Tyndex : Copy {
    fn from_index(i: usize) -> Self;
    fn to_index(self) -> usize;
}

pub struct TyVec<I: Tyndex, T> {
    pub raw: Vec<T>,
    _marker: std::marker::PhantomData<fn(&I)>,
}

impl<I: Tyndex, T> TyVec<I, T> {
    pub fn from_raw(raw: Vec<T>) -> Self {
        Self {
            raw,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> I {
        I::from_index(self.raw.len())
    }

    pub fn push_and_idx(&mut self, v: T) -> I {
        let i = self.len();
        self.raw.push(v);
        i
    }

    pub fn enum_ref(&self) -> impl Iterator<Item=(I, &T)> {
        self.raw.iter().enumerate().map(|(i, v)| (I::from_index(i), v))
    }
}

impl<I: Tyndex, T> std::ops::Index<I> for TyVec<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.raw[index.to_index()]
    }
}

impl<I: Tyndex, T> std::ops::IndexMut<I> for TyVec<I, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.raw[index.to_index()]
    }
}

impl<I: Tyndex, T: std::fmt::Debug> std::fmt::Debug for TyVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl<I: Tyndex, T: serde::Serialize> serde::Serialize for TyVec<I, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.raw.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I: Tyndex, T: serde::Deserialize<'de>> serde::Deserialize<'de> for TyVec<I, T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::deserialize(deserializer).map(Self::from_raw)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
