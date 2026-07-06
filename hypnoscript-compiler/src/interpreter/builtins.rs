//! Builtin function dispatch: bridges interpreter values to the
//! `hypnoscript-runtime` builtin modules (math, string, array, file, ...).

use super::Interpreter;
use super::error::InterpreterError;
use super::value::Value;
use hypnoscript_runtime::{
    ArrayBuiltins, CoreBuiltins, FileBuiltins, HashingBuiltins, MathBuiltins, StatisticsBuiltins,
    StringBuiltins, SystemBuiltins, TimeBuiltins, ValidationBuiltins,
};

impl Interpreter {
    pub(crate) fn call_builtin(
        &mut self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        if let Some(result) = self.call_math_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_string_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_array_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_core_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_file_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_hashing_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_statistics_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_system_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_time_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_validation_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_async_builtin(name, args)? {
            return Ok(Some(result));
        }

        Ok(None)
    }

    /// Simulated-async builtins operating on [`Value::Promise`].
    ///
    /// The interpreter is single-threaded; promises carry a simulated delay
    /// that elapses when they are awaited (scaled via `HYPNO_TIME_SCALE`,
    /// exactly like `drift`).
    pub(crate) fn call_async_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        use super::value::Promise;
        use std::cell::RefCell;
        use std::rc::Rc;

        let result = match name {
            // delayedValue(ms, value) -> promise resolving to `value` after `ms`
            "delayedValue" => {
                let delay = self.number_arg(args, 0, name)?.max(0.0) as u64;
                let value = self.arg(args, 1, name)?.clone();
                Some(Value::Promise(Rc::new(RefCell::new(Promise::delayed(
                    delay, value,
                )))))
            }
            // instantPromise(value) -> already resolved promise
            "instantPromise" => {
                let value = self.arg(args, 0, name)?.clone();
                Some(Value::Promise(Rc::new(RefCell::new(Promise::resolve(
                    value,
                )))))
            }
            // promiseAll([p1, p2, ...]) -> waits for the *longest* delay
            // (simulated concurrency) and returns the array of results.
            "promiseAll" => {
                let promises = self.array_arg(args, 0, name)?;
                let max_delay = promises
                    .iter()
                    .filter_map(|p| match p {
                        Value::Promise(p) => p.borrow().pending_delay_ms(),
                        _ => None,
                    })
                    .max()
                    .unwrap_or(0);
                CoreBuiltins::drift(max_delay);
                let results = promises
                    .iter()
                    .map(|p| match p {
                        Value::Promise(p) => p.borrow_mut().mark_resolved(),
                        other => other.clone(),
                    })
                    .collect();
                Some(Value::array(results))
            }
            // promiseRace([p1, p2, ...]) -> waits for the *shortest* delay and
            // returns that promise's value.
            "promiseRace" => {
                let promises = self.array_arg(args, 0, name)?;
                if promises.is_empty() {
                    return Err(InterpreterError::Runtime(format!(
                        "Builtin '{}' requires a non-empty array",
                        name
                    )));
                }
                let winner = promises
                    .iter()
                    .min_by_key(|p| match p {
                        Value::Promise(p) => p.borrow().pending_delay_ms().unwrap_or(0),
                        _ => 0,
                    })
                    .expect("non-empty array always has a minimum");
                let value = match winner {
                    Value::Promise(p) => {
                        CoreBuiltins::drift(p.borrow().pending_delay_ms().unwrap_or(0));
                        p.borrow_mut().mark_resolved()
                    }
                    other => other.clone(),
                };
                Some(value)
            }
            // isPromiseResolved(p) -> whether awaiting would complete instantly
            "isPromiseResolved" => match self.arg(args, 0, name)? {
                Value::Promise(p) => Some(Value::Boolean(p.borrow().is_resolved())),
                _ => Some(Value::Boolean(true)),
            },
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_math_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Sin" => Some(Value::Number(MathBuiltins::sin(
                self.number_arg(args, 0, name)?,
            ))),
            "Cos" => Some(Value::Number(MathBuiltins::cos(
                self.number_arg(args, 0, name)?,
            ))),
            "Tan" => Some(Value::Number(MathBuiltins::tan(
                self.number_arg(args, 0, name)?,
            ))),
            "Sqrt" => Some(Value::Number(MathBuiltins::sqrt(
                self.number_arg(args, 0, name)?,
            ))),
            "Log" => Some(Value::Number(MathBuiltins::log(
                self.number_arg(args, 0, name)?,
            ))),
            "Log10" => Some(Value::Number(MathBuiltins::log10(
                self.number_arg(args, 0, name)?,
            ))),
            "Abs" => Some(Value::Number(MathBuiltins::abs(
                self.number_arg(args, 0, name)?,
            ))),
            "Floor" => Some(Value::Number(MathBuiltins::floor(
                self.number_arg(args, 0, name)?,
            ))),
            "Ceil" => Some(Value::Number(MathBuiltins::ceil(
                self.number_arg(args, 0, name)?,
            ))),
            "Round" => Some(Value::Number(MathBuiltins::round(
                self.number_arg(args, 0, name)?,
            ))),
            "Min" => Some(Value::Number(MathBuiltins::min(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Max" => Some(Value::Number(MathBuiltins::max(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Pow" => Some(Value::Number(MathBuiltins::pow(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Factorial" => Some(Value::Number(MathBuiltins::factorial(
                self.integer_arg(args, 0, name)?,
            ) as f64)),
            "Gcd" => Some(Value::Number(MathBuiltins::gcd(
                self.integer_arg(args, 0, name)?,
                self.integer_arg(args, 1, name)?,
            ) as f64)),
            "Lcm" => Some(Value::Number(MathBuiltins::lcm(
                self.integer_arg(args, 0, name)?,
                self.integer_arg(args, 1, name)?,
            ) as f64)),
            "IsPrime" => Some(Value::Boolean(MathBuiltins::is_prime(
                self.integer_arg(args, 0, name)?,
            ))),
            "Fibonacci" => Some(Value::Number(MathBuiltins::fibonacci(
                self.integer_arg(args, 0, name)?,
            ) as f64)),
            "Clamp" => Some(Value::Number(MathBuiltins::clamp(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
                self.number_arg(args, 2, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_string_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Length" => {
                // Length works for both strings and arrays
                if args.is_empty() {
                    return Err(InterpreterError::Runtime(format!(
                        "Function '{}' requires at least 1 argument",
                        name
                    )));
                }
                match &args[0] {
                    Value::String(s) => Some(Value::Number(s.len() as f64)),
                    Value::Array(arr) => Some(Value::Number(arr.len() as f64)),
                    _ => {
                        return Err(InterpreterError::TypeError(format!(
                            "Function 'Length' expects string or array argument, got {}",
                            args[0]
                        )));
                    }
                }
            }
            "ToUpper" => Some(Value::String(StringBuiltins::to_upper(
                &self.string_arg(args, 0, name)?,
            ))),
            "ToLower" => Some(Value::String(StringBuiltins::to_lower(
                &self.string_arg(args, 0, name)?,
            ))),
            "Trim" => Some(Value::String(StringBuiltins::trim(
                &self.string_arg(args, 0, name)?,
            ))),
            "IndexOf" => Some(Value::Number(StringBuiltins::index_of(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ) as f64)),
            "Replace" => Some(Value::String(StringBuiltins::replace(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
                &self.string_arg(args, 2, name)?,
            ))),
            "Reverse" => Some(Value::String(StringBuiltins::reverse(
                &self.string_arg(args, 0, name)?,
            ))),
            "Capitalize" => Some(Value::String(StringBuiltins::capitalize(
                &self.string_arg(args, 0, name)?,
            ))),
            "StartsWith" => Some(Value::Boolean(StringBuiltins::starts_with(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "EndsWith" => Some(Value::Boolean(StringBuiltins::ends_with(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "Contains" => Some(Value::Boolean(StringBuiltins::contains(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "Split" => {
                let items = StringBuiltins::split(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .into_iter()
                .map(Value::String)
                .collect();
                Some(Value::array(items))
            }
            "Substring" => Some(Value::String(StringBuiltins::substring(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.usize_arg(args, 2, name)?,
            ))),
            "Repeat" => Some(Value::String(StringBuiltins::repeat(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
            ))),
            "PadLeft" => Some(Value::String(StringBuiltins::pad_left(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.char_arg(args, 2, name)?,
            ))),
            "PadRight" => Some(Value::String(StringBuiltins::pad_right(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.char_arg(args, 2, name)?,
            ))),
            "IsEmpty" => Some(Value::Boolean(StringBuiltins::is_empty(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsWhitespace" => Some(Value::Boolean(StringBuiltins::is_whitespace(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_array_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "ArrayLength" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Number(ArrayBuiltins::length(&array) as f64))
            }
            "ArrayIsEmpty" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Boolean(ArrayBuiltins::is_empty(&array)))
            }
            "ArrayGet" => {
                let array = self.array_arg(args, 0, name)?;
                let index = self.usize_arg(args, 1, name)?;
                let value = ArrayBuiltins::get(&array, index).unwrap_or(Value::Null);
                Some(value)
            }
            "ArrayIndexOf" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Number(
                    ArrayBuiltins::index_of(&array, &target) as f64
                ))
            }
            "ArrayContains" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Boolean(ArrayBuiltins::contains(&array, &target)))
            }
            "ArrayReverse" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::array(ArrayBuiltins::reverse(&array)))
            }
            "ArraySum" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::sum(&numbers)))
            }
            "ArrayAverage" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::average(&numbers)))
            }
            "ArrayMin" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::min(&numbers)))
            }
            "ArrayMax" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::max(&numbers)))
            }
            "ArraySort" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                let sorted = ArrayBuiltins::sort(&numbers)
                    .into_iter()
                    .map(Value::Number)
                    .collect();
                Some(Value::array(sorted))
            }
            "ArrayFirst" => {
                let array = self.array_arg(args, 0, name)?;
                Some(ArrayBuiltins::first(&array).unwrap_or(Value::Null))
            }
            "ArrayLast" => {
                let array = self.array_arg(args, 0, name)?;
                Some(ArrayBuiltins::last(&array).unwrap_or(Value::Null))
            }
            "ArrayTake" => {
                let array = self.array_arg(args, 0, name)?;
                let count = self.usize_arg(args, 1, name)?;
                Some(Value::array(ArrayBuiltins::take(&array, count)))
            }
            "ArraySkip" => {
                let array = self.array_arg(args, 0, name)?;
                let count = self.usize_arg(args, 1, name)?;
                Some(Value::array(ArrayBuiltins::skip(&array, count)))
            }
            "ArraySlice" => {
                let array = self.array_arg(args, 0, name)?;
                let start = self.usize_arg(args, 1, name)?;
                let end = self.usize_arg(args, 2, name)?;
                Some(Value::array(ArrayBuiltins::slice(&array, start, end)))
            }
            "ArrayJoin" => {
                let array = self.array_arg(args, 0, name)?;
                let separator = self.string_arg(args, 1, name)?;
                Some(Value::String(ArrayBuiltins::join(&array, &separator)))
            }
            "ArrayCount" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Number(ArrayBuiltins::count(&array, &target) as f64))
            }
            "ArrayDistinct" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::array(ArrayBuiltins::distinct(&array)))
            }
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_core_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Observe" => {
                let message = self.arg(args, 0, name)?.to_string();
                CoreBuiltins::observe(&message);
                Some(Value::Null)
            }
            "Drift" => {
                let duration = self.number_arg(args, 0, name)?;
                CoreBuiltins::drift(duration.max(0.0) as u64);
                Some(Value::Null)
            }
            "DeepTrance" => {
                let duration = self.number_arg(args, 0, name)?;
                CoreBuiltins::deep_trance(duration.max(0.0) as u64);
                Some(Value::Null)
            }
            "HypnoticCountdown" => {
                CoreBuiltins::hypnotic_countdown(self.integer_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "TranceInduction" => {
                CoreBuiltins::trance_induction(&self.string_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "HypnoticVisualization" => {
                CoreBuiltins::hypnotic_visualization(&self.string_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "ToInt" => Some(Value::Number(
                CoreBuiltins::to_int(self.number_arg(args, 0, name)?) as f64,
            )),
            "ToDouble" => Some(Value::Number(
                CoreBuiltins::to_double(&self.string_arg(args, 0, name)?)
                    .map_err(InterpreterError::Runtime)?,
            )),
            "ToString" => Some(Value::String(
                args.first()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "null".to_string()),
            )),
            "ToBoolean" => Some(Value::Boolean(CoreBuiltins::to_boolean(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_file_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "ReadFile" => Some(Value::String(
                FileBuiltins::read_file(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?,
            )),
            "WriteFile" => {
                FileBuiltins::write_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "AppendFile" => {
                FileBuiltins::append_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "FileExists" => Some(Value::Boolean(FileBuiltins::file_exists(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsFile" => Some(Value::Boolean(FileBuiltins::is_file(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsDirectory" => Some(Value::Boolean(FileBuiltins::is_directory(
                &self.string_arg(args, 0, name)?,
            ))),
            "DeleteFile" => {
                FileBuiltins::delete_file(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "CreateDirectory" => {
                FileBuiltins::create_directory(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "ListDirectory" => {
                let files = FileBuiltins::list_directory(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?
                    .into_iter()
                    .map(Value::String)
                    .collect();
                Some(Value::array(files))
            }
            "GetFileSize" => Some(Value::Number(
                FileBuiltins::get_file_size(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))? as f64,
            )),
            "CopyFile" => Some(Value::Number(
                FileBuiltins::copy_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))? as f64,
            )),
            "RenameFile" => {
                FileBuiltins::rename_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "GetFileExtension" => Some(self.option_string_to_value(
                FileBuiltins::get_file_extension(&self.string_arg(args, 0, name)?),
            )),
            "GetFileName" => Some(self.option_string_to_value(FileBuiltins::get_file_name(
                &self.string_arg(args, 0, name)?,
            ))),
            "GetParentDirectory" => Some(self.option_string_to_value(
                FileBuiltins::get_parent_directory(&self.string_arg(args, 0, name)?),
            )),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_hashing_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "HashString" => Some(Value::Number(HashingBuiltins::hash_string(
                &self.string_arg(args, 0, name)?,
            ) as f64)),
            "HashNumber" => Some(Value::Number(HashingBuiltins::hash_number(
                self.number_arg(args, 0, name)?,
            ) as f64)),
            "SimpleRandom" => Some(Value::Number(HashingBuiltins::simple_random(
                self.u64_arg(args, 0, name)?,
            ) as f64)),
            "AreAnagrams" => Some(Value::Boolean(HashingBuiltins::are_anagrams(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "IsPalindrome" => Some(Value::Boolean(HashingBuiltins::is_palindrome(
                &self.string_arg(args, 0, name)?,
            ))),
            "CountOccurrences" => Some(Value::Number(HashingBuiltins::count_occurrences(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ) as f64)),
            "RemoveDuplicates" => Some(Value::String(HashingBuiltins::remove_duplicates(
                &self.string_arg(args, 0, name)?,
            ))),
            "UniqueCharacters" => Some(Value::String(HashingBuiltins::unique_characters(
                &self.string_arg(args, 0, name)?,
            ))),
            "ReverseWords" => Some(Value::String(HashingBuiltins::reverse_words(
                &self.string_arg(args, 0, name)?,
            ))),
            "TitleCase" => Some(Value::String(HashingBuiltins::title_case(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_statistics_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let numbers_primary = |this: &Self| -> Result<Vec<f64>, InterpreterError> {
            let array = this.array_arg(args, 0, name)?;
            this.values_to_numbers(&array, name)
        };

        let result = match name {
            "Mean" => Some(Value::Number(StatisticsBuiltins::calculate_mean(
                &numbers_primary(self)?,
            ))),
            "Median" => Some(Value::Number(StatisticsBuiltins::calculate_median(
                &numbers_primary(self)?,
            ))),
            "Mode" => Some(Value::Number(StatisticsBuiltins::calculate_mode(
                &numbers_primary(self)?,
            ))),
            "StandardDeviation" => Some(Value::Number(
                StatisticsBuiltins::calculate_standard_deviation(&numbers_primary(self)?),
            )),
            "Variance" => Some(Value::Number(StatisticsBuiltins::calculate_variance(
                &numbers_primary(self)?,
            ))),
            "Range" => Some(Value::Number(StatisticsBuiltins::calculate_range(
                &numbers_primary(self)?,
            ))),
            "Percentile" => Some(Value::Number(StatisticsBuiltins::calculate_percentile(
                &numbers_primary(self)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Correlation" => {
                let x = self.values_to_numbers(&self.array_arg(args, 0, name)?, name)?;
                let y = self.values_to_numbers(&self.array_arg(args, 1, name)?, name)?;
                Some(Value::Number(StatisticsBuiltins::calculate_correlation(
                    &x, &y,
                )))
            }
            "LinearRegression" => {
                let x = self.values_to_numbers(&self.array_arg(args, 0, name)?, name)?;
                let y = self.values_to_numbers(&self.array_arg(args, 1, name)?, name)?;
                let (slope, intercept) = StatisticsBuiltins::linear_regression(&x, &y);
                Some(Value::array(vec![
                    Value::Number(slope),
                    Value::Number(intercept),
                ]))
            }
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_system_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "GetCurrentDirectory" => Some(Value::String(SystemBuiltins::get_current_directory())),
            "GetEnv" => Some(self.option_string_to_value(SystemBuiltins::get_env_var(
                &self.string_arg(args, 0, name)?,
            ))),
            "SetEnv" => {
                SystemBuiltins::set_env_var(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(InterpreterError::Runtime)?;
                Some(Value::Null)
            }
            "GetOperatingSystem" => Some(Value::String(SystemBuiltins::get_operating_system())),
            "GetArchitecture" => Some(Value::String(SystemBuiltins::get_architecture())),
            "GetCpuCount" => Some(Value::Number(SystemBuiltins::get_cpu_count() as f64)),
            "GetHostname" => Some(Value::String(SystemBuiltins::get_hostname())),
            "GetUsername" => Some(Value::String(SystemBuiltins::get_username())),
            "GetHomeDirectory" => Some(Value::String(SystemBuiltins::get_home_directory())),
            "GetTempDirectory" => Some(Value::String(SystemBuiltins::get_temp_directory())),
            "GetArgs" => Some(Value::array(
                SystemBuiltins::get_args()
                    .into_iter()
                    .map(Value::String)
                    .collect(),
            )),
            "Exit" => {
                // Exit mirrors the legacy runtime behavior by terminating the host process immediately.
                SystemBuiltins::exit(self.integer_arg(args, 0, name)? as i32);
            }
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_time_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "CurrentTimestamp" => Some(Value::Number(TimeBuiltins::get_current_time() as f64)),
            "CurrentDate" => Some(Value::String(TimeBuiltins::get_current_date())),
            "CurrentTime" => Some(Value::String(TimeBuiltins::get_current_time_string())),
            "CurrentDateTime" => Some(Value::String(TimeBuiltins::get_current_date_time())),
            "FormatDateTime" => Some(Value::String(TimeBuiltins::format_date_time(
                &self.string_arg(args, 0, name)?,
            ))),
            "DayOfWeek" => Some(Value::Number(TimeBuiltins::get_day_of_week() as f64)),
            "DayOfYear" => Some(Value::Number(TimeBuiltins::get_day_of_year() as f64)),
            "IsLeapYear" => Some(Value::Boolean(TimeBuiltins::is_leap_year(
                self.integer_arg(args, 0, name)? as i32,
            ))),
            "DaysInMonth" => Some(self.option_u32_to_value(TimeBuiltins::get_days_in_month(
                self.integer_arg(args, 0, name)? as i32,
                self.usize_arg(args, 1, name)? as u32,
            ))),
            "CurrentYear" => Some(Value::Number(TimeBuiltins::get_year() as f64)),
            "CurrentMonth" => Some(Value::Number(TimeBuiltins::get_month() as f64)),
            "CurrentDay" => Some(Value::Number(TimeBuiltins::get_day() as f64)),
            "CurrentHour" => Some(Value::Number(TimeBuiltins::get_hour() as f64)),
            "CurrentMinute" => Some(Value::Number(TimeBuiltins::get_minute() as f64)),
            "CurrentSecond" => Some(Value::Number(TimeBuiltins::get_second() as f64)),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn call_validation_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "IsValidEmail" => Some(Value::Boolean(ValidationBuiltins::is_valid_email(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsValidUrl" => Some(Value::Boolean(ValidationBuiltins::is_valid_url(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsValidPhoneNumber" => Some(Value::Boolean(
                ValidationBuiltins::is_valid_phone_number(&self.string_arg(args, 0, name)?),
            )),
            "IsAlphanumeric" => Some(Value::Boolean(ValidationBuiltins::is_alphanumeric(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsAlphabetic" => Some(Value::Boolean(ValidationBuiltins::is_alphabetic(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsNumeric" => Some(Value::Boolean(ValidationBuiltins::is_numeric(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsLowercase" => Some(Value::Boolean(ValidationBuiltins::is_lowercase(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsUppercase" => Some(Value::Boolean(ValidationBuiltins::is_uppercase(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsInRange" => Some(Value::Boolean(ValidationBuiltins::is_in_range(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
                self.number_arg(args, 2, name)?,
            ))),
            "MatchesPattern" => Some(Value::Boolean(ValidationBuiltins::matches_pattern(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn arg<'a>(
        &self,
        args: &'a [Value],
        index: usize,
        name: &str,
    ) -> Result<&'a Value, InterpreterError> {
        args.get(index).ok_or_else(|| {
            InterpreterError::Runtime(format!(
                "Builtin '{}' expected argument at position {}",
                name,
                index + 1
            ))
        })
    }

    pub(crate) fn number_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<f64, InterpreterError> {
        self.arg(args, index, name)?.to_number().map_err(|_| {
            InterpreterError::TypeError(format!(
                "Builtin '{}' expected numeric argument at position {}",
                name,
                index + 1
            ))
        })
    }

    pub(crate) fn integer_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<i64, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        Ok(value.round() as i64)
    }

    pub(crate) fn u64_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<u64, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        if value < 0.0 {
            return Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-negative number at position {}",
                name,
                index + 1
            )));
        }
        Ok(value.round() as u64)
    }

    pub(crate) fn usize_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<usize, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        if value < 0.0 {
            return Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-negative number at position {}",
                name,
                index + 1
            )));
        }
        Ok(value.round() as usize)
    }

    pub(crate) fn string_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<String, InterpreterError> {
        match self.arg(args, index, name)? {
            Value::String(s) => Ok(s.clone()),
            other => Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected string argument at position {}, got {:?}",
                name,
                index + 1,
                other
            ))),
        }
    }

    pub(crate) fn char_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<char, InterpreterError> {
        let text = self.string_arg(args, index, name)?;
        text.chars().next().ok_or_else(|| {
            InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-empty string to derive character at position {}",
                name,
                index + 1
            ))
        })
    }

    /// Returns the array argument as a shared handle (O(1), no deep copy).
    pub(crate) fn array_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<std::rc::Rc<Vec<Value>>, InterpreterError> {
        match self.arg(args, index, name)? {
            Value::Array(items) => Ok(std::rc::Rc::clone(items)),
            other => Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected array argument at position {}, got {:?}",
                name,
                index + 1,
                other
            ))),
        }
    }

    pub(crate) fn option_string_to_value(&self, input: Option<String>) -> Value {
        input.map(Value::String).unwrap_or(Value::Null)
    }

    pub(crate) fn option_u32_to_value(&self, input: Option<u32>) -> Value {
        input
            .map(|v| Value::Number(v as f64))
            .unwrap_or(Value::Null)
    }

    pub(crate) fn values_to_numbers(
        &self,
        values: &[Value],
        name: &str,
    ) -> Result<Vec<f64>, InterpreterError> {
        values
            .iter()
            .enumerate()
            .map(|(i, value)| {
                value.to_number().map_err(|_| {
                    InterpreterError::TypeError(format!(
                        "Builtin '{}' expected numeric array element at position {}",
                        name,
                        i + 1
                    ))
                })
            })
            .collect()
    }
}
