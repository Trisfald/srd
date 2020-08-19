//! Module for numerical values.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// A numerical value bounded between the default(usually 0) and a maximum.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PositiveBoundedValue<T> {
    value: T,
    max: T,
}

impl<T: Default + Copy + Ord> PositiveBoundedValue<T> {
    fn capped_value(value: T, max: T) -> T {
        if value < T::default() {
            T::default()
        } else {
            std::cmp::min(value, max)
        }
    }

    /// Constructs a new `PositiveBoundedValue`.
    pub fn new(value: T, max: T) -> Self {
        Self {
            value: Self::capped_value(value, max),
            max,
        }
    }

    /// Returns the current value.
    pub fn value(&self) -> T {
        self.value
    }

    /// Sets a new value.
    pub fn set_value(&mut self, value: T) {
        self.value = value
    }

    /// Returns the maximum value.
    pub fn max(&self) -> T {
        self.max
    }

    /// Sets a new maximum.
    pub fn set_max(&mut self, max: T) {
        self.max = Self::capped_value(max, max);
        self.value = Self::capped_value(self.value, self.max);
    }
}

impl<T: Default + Add + Copy + Ord + Add<T, Output = T>> PositiveBoundedValue<T> {
    /// Adds `n` to the current value.
    pub fn add(&mut self, n: T) {
        self.value = Self::capped_value(self.value + n, self.max);
    }
}

impl<T: Default + Sub + Copy + Ord + Sub<T, Output = T>> PositiveBoundedValue<T> {
    /// Subtracts `n` to the current value.
    pub fn subtract(&mut self, n: T) {
        if n > self.value {
            self.value = T::default();
        } else {
            self.value = Self::capped_value(self.value - n, self.max);
        }
    }
}

impl<T: Default + Mul + Copy + Ord + Mul<T, Output = T>> PositiveBoundedValue<T> {
    /// Multiplies the current value by `n`.
    pub fn multiply(&mut self, n: T) {
        self.value = Self::capped_value(self.value * n, self.max);
    }
}

impl<T: Default + Div + Copy + Ord + Div<T, Output = T>> PositiveBoundedValue<T> {
    /// Divides the current value by `n`.
    pub fn divide(&mut self, n: T) {
        self.value = Self::capped_value(self.value / n, self.max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_bounded_new() {
        assert_eq!(PositiveBoundedValue::new(4, 6).value(), 4);
        assert_eq!(PositiveBoundedValue::new(8, 6).value(), 6);
        assert_eq!(PositiveBoundedValue::new(-8, 6).value(), 0);
    }

    #[test]
    fn positive_bounded_add() {
        let mut v = PositiveBoundedValue::new(5, 10);
        v.add(2);
        assert_eq!(v.value(), 7);
        v.add(10);
        assert_eq!(v.value(), 10);
        v.add(-100);
        assert_eq!(v.value(), 0);
    }

    #[test]
    fn positive_bounded_sub() {
        let mut v = PositiveBoundedValue::new(5, 10);
        v.subtract(2);
        assert_eq!(v.value(), 3);
        v.subtract(10);
        assert_eq!(v.value(), 0);
        v.subtract(-100);
        assert_eq!(v.value(), 10);

        let mut v = PositiveBoundedValue::new(5u8, 10u8);
        v.subtract(6u8);
        assert_eq!(v.value(), 0);
    }

    #[test]
    fn positive_bounded_mul() {
        let mut v = PositiveBoundedValue::new(2, 10);
        v.multiply(2);
        assert_eq!(v.value(), 4);
        v.multiply(5);
        assert_eq!(v.value(), 10);
        v.multiply(-50);
        assert_eq!(v.value(), 0);
    }

    #[test]
    fn positive_bounded_div() {
        let mut v = PositiveBoundedValue::new(6, 10);
        v.divide(2);
        assert_eq!(v.value(), 3);
        v.divide(2);
        assert_eq!(v.value(), 1);
        v.divide(-1);
        assert_eq!(v.value(), 0);
    }

    #[test]
    fn positive_bounded_change_max() {
        let mut v = PositiveBoundedValue::new(6, 10);
        v.set_max(15);
        assert_eq!(v.max(), 15);
        assert_eq!(v.value(), 6);
        v.set_max(5);
        assert_eq!(v.max(), 5);
        assert_eq!(v.value(), 5);
        v.set_max(-1);
        assert_eq!(v.max(), 0);
        assert_eq!(v.value(), 0);
    }
}
