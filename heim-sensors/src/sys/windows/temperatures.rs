use std::iter;

use heim_common::prelude::*;

use crate::TemperatureSensor;

pub fn temperatures() -> Result<impl Iterator<Item = Result<TemperatureSensor>>> {
    iter::empty()
}
