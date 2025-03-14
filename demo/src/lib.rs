#![deny(unsafe_op_in_unsafe_fn)]

use android_view::{
    jni::{
        JNIEnv, JavaVM,
        sys::{JNI_VERSION_1_6, JavaVM as RawJavaVM, jint, jlong},
    },
    *,
};
use log::LevelFilter;
use std::ffi::c_void;

struct DemoViewPeer;

impl ViewPeer for DemoViewPeer {
    fn on_hover_event<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        _view: &View,
        event: &MotionEvent<'local>,
    ) -> bool {
        log::trace!("hover {} {}", event.x(env), event.y(env));
        false
    }

    // TODO
}

extern "system" fn new_view_peer<'local>(
    _env: JNIEnv<'local>,
    _view: View<'local>,
    _context: Context<'local>,
) -> jlong {
    log::trace!("new demo view");
    register_view_peer(DemoViewPeer)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn JNI_OnLoad(vm: *mut RawJavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("android-view-demo"),
    );

    let vm = unsafe { JavaVM::from_raw(vm) }.unwrap();
    let mut env = vm.get_env().unwrap();
    register_view_class(
        &mut env,
        "org/linebender/android/viewdemo/DemoView",
        new_view_peer,
    );
    JNI_VERSION_1_6
}
