use jni::{AttachGuard, JavaVM, InitArgsBuilder, JNIVersion};
use std::sync::{Arc, Once};

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
pub struct Env<'a> {
  use_main_thread: bool,
  attach_guard: Option<&'a mut AttachGuard<'a>>
}

// We can allow the host to define the stdlib classes/methods
// that can 
pub trait CoreFeatures {
  fn attach(&mut self);
  fn invoke_static();
}

impl<'a> Default for Env<'a> {
  fn default() -> Self {
      Self {
        use_main_thread: true,
        attach_guard: None
      }
  }
}

impl<'a> CoreFeatures for Env<'a> {
  fn attach(&mut self){
    let mut env: AttachGuard<'a> = attach_current_thread();
    self.attach_guard = Some(&mut env);
  }

  fn invoke_static() {
      unimplemented!()
  }
}
