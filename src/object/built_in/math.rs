use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::panic_type::PanicType,
    float_obj::FloatObj,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

fn make_int(value: i64) -> ObjectRef {
    new_objectref(Object::Int(Integer { value }))
}

fn make_float(value: f64) -> ObjectRef {
    new_objectref(Object::FloatObj(FloatObj { val: value }))
}

fn make_array(items: Vec<ObjectRef>) -> ObjectRef {
    new_objectref(Object::Array(Box::new(Array { items })))
}

fn float_from_object(obj: &Object, state: StateRef) -> Result<f64, RuntimeSignal> {
    match obj {
        Object::FloatObj(float) => Ok(float.val),
        Object::Int(integer) => Ok(integer.value as f64),
        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            "expected a numeric value".into(),
            state,
        ))),
    }
}

fn int_from_object(obj: &Object, state: StateRef) -> Result<i64, RuntimeSignal> {
    match obj {
        Object::Int(integer) => Ok(integer.value),
        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            "expected an integer value".into(),
            state,
        ))),
    }
}

fn float_argument(args: &[ObjectRef], index: usize, state: StateRef) -> Result<f64, RuntimeSignal> {
    args.get(index)
        .ok_or_else(|| {
            RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!("missing argument {}", index + 1),
                state.clone(),
            ))
        })
        .and_then(|obj| float_from_object(&*obj.borrow(), state))
}

fn integer_argument(
    args: &[ObjectRef],
    index: usize,
    state: StateRef,
) -> Result<i64, RuntimeSignal> {
    args.get(index)
        .ok_or_else(|| {
            RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!("missing argument {}", index + 1),
                state.clone(),
            ))
        })
        .and_then(|obj| int_from_object(&*obj.borrow(), state))
}

fn wrong_argument_count(
    expected: &str,
    got: usize,
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    Err(RuntimeSignal::Panic(PanicObj::new(
        PanicType::WrongArgumentCount,
        format!("expected {}, got {}", expected, got),
        state,
    )))
}

pub fn ceil_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    let value = float_argument(args, 0, state)?;
    Ok(make_int(value.ceil() as i64))
}

pub fn floor_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    let value = float_argument(args, 0, state)?;
    Ok(make_int(value.floor() as i64))
}

pub fn trunc_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    let value = float_argument(args, 0, state)?;
    Ok(make_int(value.trunc() as i64))
}

pub fn fabs_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    let value = float_argument(args, 0, state)?;
    Ok(make_float(value.abs()))
}

pub fn sqrt_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    let value = float_argument(args, 0, state)?;
    Ok(make_float(value.sqrt()))
}

pub fn exp_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    Ok(make_float(float_argument(args, 0, state)?.exp()))
}

pub fn expm1_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }

    Ok(make_float(float_argument(args, 0, state)?.exp_m1()))
}

pub fn log_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    match args.len() {
        1 => {
            let value = float_argument(args, 0, state)?;
            Ok(make_float(value.ln()))
        }
        2 => {
            let value = float_argument(args, 0, state.clone())?;
            let base = float_argument(args, 1, state.clone())?;
            Ok(make_float(value.log(base)))
        }
        _ => wrong_argument_count("1 or 2 values", args.len(), state),
    }
}

pub fn log10_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.log10()))
}

pub fn log1p_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.ln_1p()))
}

pub fn pow_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    let base = float_argument(args, 0, state.clone())?;
    let exponent = float_argument(args, 1, state.clone())?;
    Ok(make_float(base.powf(exponent)))
}

pub fn fmod_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    let x = float_argument(args, 0, state.clone())?;
    let y = float_argument(args, 1, state.clone())?;
    Ok(make_float(x % y))
}

pub fn hypot_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    let x = float_argument(args, 0, state.clone())?;
    let y = float_argument(args, 1, state.clone())?;
    Ok(make_float(x.hypot(y)))
}

pub fn copysign_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    let x = float_argument(args, 0, state.clone())?;
    let y = float_argument(args, 1, state)?;
    Ok(make_float(x.copysign(y)))
}

pub fn degrees_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.to_degrees()))
}

pub fn radians_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.to_radians()))
}

pub fn sin_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.sin()))
}

pub fn cos_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.cos()))
}

pub fn tan_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.tan()))
}

pub fn asin_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.asin()))
}

pub fn acos_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.acos()))
}

pub fn atan_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.atan()))
}

pub fn atan2_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    Ok(make_float(
        float_argument(args, 0, state.clone())?.atan2(float_argument(args, 1, state.clone())?),
    ))
}

pub fn sinh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.sinh()))
}

pub fn cosh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.cosh()))
}

pub fn tanh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.tanh()))
}

pub fn asinh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.asinh()))
}

pub fn acosh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.acosh()))
}

pub fn atanh_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    Ok(make_float(float_argument(args, 0, state)?.atanh()))
}

pub fn ldexp_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return wrong_argument_count("2 values", args.len(), state);
    }
    let float = float_argument(args, 0, state.clone())?;
    let exponent = integer_argument(args, 1, state)?;
    Ok(make_float(float * 2f64.powi(exponent as i32)))
}

fn frexp(value: f64) -> (f64, i64) {
    if value == 0.0 {
        return (0.0, 0);
    }

    let sign = if value.is_sign_negative() { -1.0 } else { 1.0 };
    let mut mantissa = value.abs();
    let mut exponent = 0i64;

    while mantissa < 0.5 {
        mantissa *= 2.0;
        exponent -= 1;
    }
    while mantissa >= 1.0 {
        mantissa *= 0.5;
        exponent += 1;
    }

    (sign * mantissa, exponent)
}

pub fn frexp_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    let value = float_argument(args, 0, state.clone())?;
    let (mantissa, exponent) = frexp(value);
    Ok(make_array(vec![
        make_float(mantissa),
        make_int(exponent as i64),
    ]))
}

pub fn modf_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    let value = float_argument(args, 0, state)?;
    Ok(make_array(vec![
        make_float(value.fract()),
        make_float(value.trunc()),
    ]))
}

fn integer_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() < 2 {
        return wrong_argument_count("at least 2 values", args.len(), state);
    }
    let mut result = integer_argument(args, 0, state.clone())?;
    for index in 1..args.len() {
        result = integer_gcd(result, integer_argument(args, index, state.clone())?);
    }
    Ok(make_int(result))
}

pub fn lcm_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() < 2 {
        return wrong_argument_count("at least 2 values", args.len(), state);
    }
    let mut result = integer_argument(args, 0, state.clone())?;
    for index in 1..args.len() {
        let next = integer_argument(args, index, state.clone())?;
        let gcd = integer_gcd(result, next);
        if gcd == 0 {
            result = 0;
        } else {
            result = (result / gcd).abs() * next.abs();
        }
    }
    Ok(make_int(result))
}

pub fn factorial_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return wrong_argument_count("1 value", args.len(), state);
    }
    let value = integer_argument(args, 0, state.clone())?;
    if value < 0 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            "factorial() not defined for negative values".into(),
            state,
        )));
    }
    let mut result = 1i64;
    for i in 1..=value {
        result = result.checked_mul(i).ok_or_else(|| {
            RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                "integer overflow in factorial".into(),
                state.clone(),
            ))
        })?;
    }
    Ok(make_int(result))
}
