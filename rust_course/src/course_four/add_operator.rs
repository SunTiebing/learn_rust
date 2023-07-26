use std::ops::Add;

#[derive(Debug)]
pub(crate) struct TypeOne(pub i32);
#[derive(Debug)]
pub(crate) struct TypeTwo(pub i32);

impl Add for TypeOne {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TypeOne(self.0 + other.0)
    }
}

impl Add for TypeTwo {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TypeTwo(self.0 + other.0)
    }
}

pub(crate) trait AddTrait {
    fn add_trait(self, other: Self) -> Self;
}

impl AddTrait for TypeOne {
    fn add_trait(self, other: Self) -> Self {
        self + other
    }
}

impl AddTrait for TypeTwo {
    fn add_trait(self, other: Self) -> Self {
        self + other
    }
}

pub(crate) fn call_add_trait<T: AddTrait>(item1: T, item2: T) -> T {
    item1.add_trait(item2)
}
