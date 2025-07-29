use std::{collections::HashMap, iter::Map};

use strum::IntoEnumIterator;

pub struct Meal {
    name: String,
    ingredients: Vec<Ingredient>,
}

pub struct Ingredient {
    name: String,
    amount: Amount,
}

pub struct Amount {
    unit: Unit,
    value: f32,
}

pub enum Unit {
    Count,
    Grams,
    Kilograms,
    Millilitres,
    Litres,
}

/*
 * Instead of having the days we do have meals.
 * Have every day, then populate that with multiple meals.
 *
 * Calendar {
 *      months: Vec<Month>,
 * }
 *
 * Month {
 *      days: Vec<Day>,
 * }
 *
 * Day {
 *      number: 1..31, // validate for each day, prob want a crate for this
 *      name: DayName
 * }
 *
 * DayName {
 *      Monday,
 *      Tuesday,
 *      Wednesday,
 *      Thursday,
 *      Friday,
 *      Saturday,
 *      Sunday,
 * }
 */

pub struct Calendar {
    meals: Map<Date, Meal>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Date {
    day: Day,
    month: Month,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(strum::EnumIter, Eq, Hash, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn as_vec() -> Vec<Self> {
        Self::iter().collect::<Vec<Self>>()
    }
}
