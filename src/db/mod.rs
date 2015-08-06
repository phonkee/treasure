/*

Value enum

Represents all available POD types.

Every dialect must have possibility to return this kind of value. Then every column has trait from_value and to_value.
This is the point where treasure will communicate with all dialects
 */

pub mod row;
pub mod value;
