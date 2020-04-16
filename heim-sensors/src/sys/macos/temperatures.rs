use heim_common::prelude::*;
use std::iter;

use crate::TemperatureSensor;

pub fn temperatures() -> Result<impl Iterator<Item = Result<TemperatureSensor>>> {
    iter::empty()
}
