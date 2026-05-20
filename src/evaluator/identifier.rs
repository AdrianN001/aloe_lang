use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::identifier::Identifier,
    object::{
        Object, ObjectRef,
        built_in::BuiltIn,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl Identifier {
    fn get_builtin_from_identifier(&self) -> Option<BuiltIn> {
        match self.value.as_str() {
            "len" => Some(BuiltIn::Len),

            "rest" => Some(BuiltIn::Rest),
            "first" => Some(BuiltIn::First),
            "last" => Some(BuiltIn::Last),
            "push" => Some(BuiltIn::Push),

            "print" => Some(BuiltIn::Print),
            "println" => Some(BuiltIn::Println),
            "__input" => Some(BuiltIn::Read),
            "__input_async" => Some(BuiltIn::ARead),

            "type" => Some(BuiltIn::Type),
            "inspect" => Some(BuiltIn::Inspect),

            "range" => Some(BuiltIn::Range),
            "__random" => Some(BuiltIn::Random),

            "__err" => Some(BuiltIn::Err),

            "__open" => Some(BuiltIn::Open),
            "__path" => Some(BuiltIn::Path),
            "__tcp_bind" => Some(BuiltIn::TCPBind),
            "__tcp_connect" => Some(BuiltIn::TCPConnect),
            "__atcp_bind" => Some(BuiltIn::ATCPBind),
            "__atcp_connect" => Some(BuiltIn::ATCPConnect),

            "__sleep" => Some(BuiltIn::Sleep),
            "__sleep2" => Some(BuiltIn::Sleep2),
            "__time" => Some(BuiltIn::Time),

            "__spawn" => Some(BuiltIn::Spawn),

            "id" => Some(BuiltIn::Id),
            "__ref_n" => Some(BuiltIn::RefNumber),
            "__sizeof" => Some(BuiltIn::Size),
            "ceil" => Some(BuiltIn::Ceil),
            "floor" => Some(BuiltIn::Floor),
            "trunc" => Some(BuiltIn::Trunc),
            "fabs" => Some(BuiltIn::Fabs),
            "sqrt" => Some(BuiltIn::Sqrt),
            "exp" => Some(BuiltIn::Exp),
            "expm1" => Some(BuiltIn::Expm1),
            "log" => Some(BuiltIn::Log),
            "log10" => Some(BuiltIn::Log10),
            "log1p" => Some(BuiltIn::Log1p),
            "pow" => Some(BuiltIn::Pow),
            "fmod" => Some(BuiltIn::Fmod),
            "hypot" => Some(BuiltIn::Hypot),
            "copysign" => Some(BuiltIn::Copysign),
            "degrees" => Some(BuiltIn::Degrees),
            "radians" => Some(BuiltIn::Radians),
            "sin" => Some(BuiltIn::Sin),
            "cos" => Some(BuiltIn::Cos),
            "tan" => Some(BuiltIn::Tan),
            "asin" => Some(BuiltIn::Asin),
            "acos" => Some(BuiltIn::Acos),
            "atan" => Some(BuiltIn::Atan),
            "atan2" => Some(BuiltIn::Atan2),
            "sinh" => Some(BuiltIn::Sinh),
            "cosh" => Some(BuiltIn::Cosh),
            "tanh" => Some(BuiltIn::Tanh),
            "asinh" => Some(BuiltIn::Asinh),
            "acosh" => Some(BuiltIn::Acosh),
            "atanh" => Some(BuiltIn::Atanh),
            "ldexp" => Some(BuiltIn::Ldexp),
            "frexp" => Some(BuiltIn::Frexp),
            "modf" => Some(BuiltIn::Modf),
            "gcd" => Some(BuiltIn::Gcd),
            "lcm" => Some(BuiltIn::Lcm),
            "factorial" => Some(BuiltIn::Factorial),
            _ => None,
        }
    }

    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        match environ.borrow().get(&self.value) {
            Some(obj) => Ok(obj.clone()),
            None => {
                if let Some(built_in) = self.get_builtin_from_identifier() {
                    return Ok(Rc::new(RefCell::new(Object::BuiltIn(built_in))));
                }

                Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::UnknownIdentifier,
                    format!("unknown identifier: {}", &self.value),
                    state.clone(),
                )))
            }
        }
    }
}
