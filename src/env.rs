use jni::objects::JValue;
use jni::{AttachGuard, InitArgsBuilder, JNIVersion, JavaVM};
use std::sync::{Arc, Once};

#[derive(Debug, Clone, Copy)]
pub struct Signature<'a>(pub &'a str, pub &'a str, pub &'a str);

pub fn jvm() -> &'static Arc<JavaVM> {
    static mut JVM: Option<Arc<JavaVM>> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .build()
            .unwrap_or_else(|e| panic!("{:#?}", e));

        let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

        unsafe {
            JVM = Some(Arc::new(jvm));
        }
    });

    unsafe { JVM.as_ref().unwrap() }
}

pub fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("failed to attach jvm thread")
}

// #[derive(Debug, Clone)]
pub struct Env {
    use_main_thread: bool,
}

// We can allow the host to define the stdlib classes/methods
// that can get loaded as regular libraries.
pub trait CoreFeatures {
    fn invoke_static(sig: Signature, args: &[JValue]) -> Option<JValue<'static, 'static>>;
}

impl Default for Env {
    fn default() -> Self {
        Self {
            use_main_thread: true,
        }
    }
}

impl CoreFeatures for Env {
    fn invoke_static(sig: Signature, args: &[JValue]) -> Option<JValue<'static, 'static>> {
        let mut env: AttachGuard<'_> = attach_current_thread();
        let Signature(class, method, signature) = sig;
        Some(
            env.call_static_method(class, method, signature, args)
                .unwrap()
                .borrow()
        )
    }
}
