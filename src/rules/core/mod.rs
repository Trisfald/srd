//! Core components used by the rules.

pub mod action;

pub mod statistic;
pub use self::statistic::{
    Statistic, StatisticChange, StatisticId, StatisticInitializer, StatisticsSeed,
};
