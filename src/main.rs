use crate::env::{Env, CoreFeatures, Signature};
use jni::objects::JValue;

mod env;

fn main() {
    Env::invoke_static(
        Signature("java/lang/Math", "abs", "(I)I"),
        &[JValue::from(10)]
    );
}
