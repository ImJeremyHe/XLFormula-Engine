//!# XLFormula Engine
//!XLFormula Engine is a Rust crate for parsing and evaluating Excel formulas. It currently works with f32 types.
//!
//!## Features
//!It supports:
//!
//!* Any numbers, negative and positive, as float or integer;
//!* Arithmetic operations +, -, /, *, ^;
//!* Logical operations AND(), OR(), NOT(), XOR();
//!* Comparison operations =, >, >=, <, <=, <>;
//!* String operation & (concatenation);
//!* Build-in variables TRUE, FALSE;
//!* Excel functions ABS(), SUM(), PRODUCT(), AVERAGE();
//!* Operations on lists of values (one dimensional range);
//! * Add or subtract dates and excel funtion DAYS().
//!
//!## Installation
//!
//!Add the corresponding entry to your Cargo.toml dependency list:
//!```toml
//![dependencies]
//!xlformula_engine = "0.1.8"
//!```
//!and add this to your crate root:
//!```rust
//!extern crate xlformula_engine;
//!```
//!
//!## Examples
//!
//!Here are simple examples of parsing an Excel formula string and evaluating to a result:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1+2");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1+3/0"); // error (#DIV/0!)
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!The last string is evaluated to #DIV/0!.
//!
//!Concatenating strings:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=\"Hello \" & \" World!\"");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1 + \"Hello\""); // error (#CAST!)
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!Concatenating number and string results in a #CAST! error.
//!
//!Constants ( i.e. a string without '=' ):
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"1.2");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"Hello World");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Excel functions:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Logical expressions:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=2>=1");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=AND(\"test\",\"True\", 1, true) ");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!References:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!
//!let data_function = |s: String| match s.as_str() {
//!"A" => types::Value::Text("=1+B".to_string()),
//!"B" => types::Value::Number(3.0),
//!_ => types::Value::Error(types::Error::Value),
//!};
//!let formula = parse_formula::parse_string_to_formula(&"=A+B");
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!List:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoFormula;
//!
//!let formula = parse_formula::parse_string_to_formula(&"={1,2,3}+{1,2,3}");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));    
//!
//!let formula = parse_formula::parse_string_to_formula(&"=XOR({0,0,0})");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=AVERAGE({1,2,3},1,2,3)");
//!let result = calculate::calculate_formula(formula, None::<NoFormula>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Date:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!use chrono::format::ParseError;
//!use chrono::{DateTime, FixedOffset};
//!
//!fn main() -> Result<(), ParseError> {
//!let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
//!let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
//!let data_function = |s: String| match s.as_str() {
//!"start" => types::Value::Date(start),
//!"end" => types::Value::Date(end),
//!_ => types::Value::Error(types::Error::Value),
//!};
//!
//!let formula = parse_formula::parse_string_to_formula(&"=DAYS(end, start)");
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=start+1");
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=end-3");
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//! Ok(())
//!}
//!```

#[macro_use]
extern crate pest_derive;

/// Evaluates a formula.
pub mod calculate;

/// The Structs and Enums for the calculation.
pub mod types;

/// Parses a string using `pest` and `pest::prec_climber`.
pub mod parse_formula;

pub type NoFormula<'a> = &'a fn(String) -> types::Value;
