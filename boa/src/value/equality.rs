use super::*;
use crate::builtins::Number;

impl Value {
    /// Strict equality comparison. Always.
    ///
    /// (fork)
    ///
    /// For more information, check <https://tc39.es/ecma262/#sec-strict-equality-comparison>.
    pub fn equals(&self, other: &Self) -> bool {
        // 1. If Type(x) is different from Type(y), return false.
        if self.get_type() != other.get_type() {
            return false;
        }

        match (self, other) {
            // 2. If Type(x) is Number or BigInt, then
            //    a. Return ! Type(x)::equal(x, y).
            (Self::BigInt(x), Self::BigInt(y)) => BigInt::equal(x, y),
            (Self::Rational(x), Self::Rational(y)) => Number::equal(*x, *y),
            (Self::Rational(x), Self::Integer(y)) => Number::equal(*x, f64::from(*y)),
            (Self::Integer(x), Self::Rational(y)) => Number::equal(f64::from(*x), *y),
            (Self::Integer(x), Self::Integer(y)) => x == y,

            //Null has to be handled specially because "typeof null" returns object and if we managed
            //this without a special case we would compare self and other as if they were actually
            //objects which unfortunately fails
            //Specification Link: https://tc39.es/ecma262/#sec-typeof-operator
            (Self::Null, Self::Null) => true,

            // 3. Return ! SameValueNonNumeric(x, y).
            (_, _) => same_value_non_numeric(self, other),
        }
    }
}

/// This function takes a string and conversts it to BigInt type.
///
/// If the result is `NaN` than `None` is returned.
///
/// More information:
///  - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-stringtobigint
pub fn string_to_bigint(string: &str) -> Option<BigInt> {
    if string.is_empty() {
        return Some(BigInt::from(0));
    }

    BigInt::from_str(string)
}

/// The internal comparison abstract operation SameValue(x, y),
/// where x and y are ECMAScript language values, produces true or false.
///
/// More information:
///  - [ECMAScript][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-samevalue
pub fn same_value(x: &Value, y: &Value) -> bool {
    // 1. If Type(x) is different from Type(y), return false.
    if x.get_type() != y.get_type() {
        return false;
    }

    match (x, y) {
        // 2. If Type(x) is Number or BigInt, then
        //    a. Return ! Type(x)::SameValue(x, y).
        (Value::BigInt(x), Value::BigInt(y)) => BigInt::same_value(x, y),
        (Value::Rational(x), Value::Rational(y)) => Number::same_value(*x, *y),
        (Value::Rational(x), Value::Integer(y)) => Number::same_value(*x, f64::from(*y)),
        (Value::Integer(x), Value::Rational(y)) => Number::same_value(f64::from(*x), *y),
        (Value::Integer(x), Value::Integer(y)) => x == y,

        // 3. Return ! SameValueNonNumeric(x, y).
        (_, _) => same_value_non_numeric(x, y),
    }
}

/// The internal comparison abstract operation `SameValueZero(x, y)`,
/// where `x` and `y` are ECMAScript language values, produces `true` or `false`.
///
/// `SameValueZero` differs from SameValue only in its treatment of `+0` and `-0`.
///
/// More information:
///  - [ECMAScript][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-samevaluezero
pub fn same_value_zero(x: &Value, y: &Value) -> bool {
    if x.get_type() != y.get_type() {
        return false;
    }

    match (x, y) {
        // 2. If Type(x) is Number or BigInt, then
        //    a. Return ! Type(x)::SameValueZero(x, y).
        (Value::BigInt(x), Value::BigInt(y)) => BigInt::same_value_zero(x, y),

        (Value::Rational(x), Value::Rational(y)) => Number::same_value_zero(*x, *y),
        (Value::Rational(x), Value::Integer(y)) => Number::same_value_zero(*x, f64::from(*y)),
        (Value::Integer(x), Value::Rational(y)) => Number::same_value_zero(f64::from(*x), *y),
        (Value::Integer(x), Value::Integer(y)) => x == y,

        // 3. Return ! SameValueNonNumeric(x, y).
        (_, _) => same_value_non_numeric(x, y),
    }
}

fn same_value_non_numeric(x: &Value, y: &Value) -> bool {
    debug_assert!(x.get_type() == y.get_type());
    match (x, y) {
        (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
        (Value::String(ref x), Value::String(ref y)) => x == y,
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::Object(ref x), Value::Object(ref y)) => GcObject::equals(x, y),
        _ => false,
    }
}
