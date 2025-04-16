use jni::{
    JNIEnv,
    objects::{JObject, JString},
    sys::{JNI_FALSE, JNI_TRUE, jboolean, jint, jlong},
};
use std::borrow::Cow;

use crate::{binder::*, events::KeyEvent, view::*};

pub const INPUT_TYPE_MASK_CLASS: u32 = 0x0000000f;
pub const INPUT_TYPE_MASK_VARIATION: u32 = 0x00000ff0;
pub const INPUT_TYPE_MASK_FLAGS: u32 = 0x00fff000;
pub const INPUT_TYPE_NULL: u32 = 0x00000000;
pub const INPUT_TYPE_CLASS_TEXT: u32 = 0x00000001;
pub const INPUT_TYPE_TEXT_FLAG_CAP_CHARACTERS: u32 = 0x00001000;
pub const INPUT_TYPE_TEXT_FLAG_CAP_WORDS: u32 = 0x00002000;
pub const INPUT_TYPE_TEXT_FLAG_CAP_SENTENCES: u32 = 0x00004000;
pub const INPUT_TYPE_TEXT_FLAG_AUTO_CORRECT: u32 = 0x00008000;
pub const INPUT_TYPE_TEXT_FLAG_AUTO_COMPLETE: u32 = 0x00010000;
pub const INPUT_TYPE_TEXT_FLAG_MULTI_LINE: u32 = 0x00020000;
pub const INPUT_TYPE_TEXT_FLAG_IME_MULTI_LINE: u32 = 0x00040000;
pub const INPUT_TYPE_TEXT_FLAG_NO_SUGGESTIONS: u32 = 0x00080000;
pub const INPUT_TYPE_TEXT_FLAG_ENABLE_TEXT_CONVERSION_SUGGESTIONS: u32 = 0x00100000;
pub const INPUT_TYPE_TEXT_VARIATION_NORMAL: u32 = 0x00000000;
pub const INPUT_TYPE_TEXT_VARIATION_URI: u32 = 0x00000010;
pub const INPUT_TYPE_TEXT_VARIATION_EMAIL_ADDRESS: u32 = 0x00000020;
pub const INPUT_TYPE_TEXT_VARIATION_EMAIL_SUBJECT: u32 = 0x00000030;
pub const INPUT_TYPE_TEXT_VARIATION_SHORT_MESSAGE: u32 = 0x00000040;
pub const INPUT_TYPE_TEXT_VARIATION_LONG_MESSAGE: u32 = 0x00000050;
pub const INPUT_TYPE_TEXT_VARIATION_PERSON_NAME: u32 = 0x00000060;
pub const INPUT_TYPE_TEXT_VARIATION_POSTAL_ADDRESS: u32 = 0x00000070;
pub const INPUT_TYPE_TEXT_VARIATION_PASSWORD: u32 = 0x00000080;
pub const INPUT_TYPE_TEXT_VARIATION_VISIBLE_PASSWORD: u32 = 0x00000090;
pub const INPUT_TYPE_TEXT_VARIATION_WEB_EDIT_TEXT: u32 = 0x000000a0;
pub const INPUT_TYPE_TEXT_VARIATION_FILTER: u32 = 0x000000b0;
pub const INPUT_TYPE_TEXT_VARIATION_PHONETIC: u32 = 0x000000c0;
pub const INPUT_TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS: u32 = 0x000000d0;
pub const INPUT_TYPE_TEXT_VARIATION_WEB_PASSWORD: u32 = 0x000000e0;
pub const INPUT_TYPE_CLASS_NUMBER: u32 = 0x00000002;
pub const INPUT_TYPE_NUMBER_FLAG_SIGNED: u32 = 0x00001000;
pub const INPUT_TYPE_NUMBER_FLAG_DECIMAL: u32 = 0x00002000;
pub const INPUT_TYPE_NUMBER_VARIATION_NORMAL: u32 = 0x00000000;
pub const INPUT_TYPE_NUMBER_VARIATION_PASSWORD: u32 = 0x00000010;
pub const INPUT_TYPE_CLASS_PHONE: u32 = 0x00000003;
pub const INPUT_TYPE_CLASS_DATETIME: u32 = 0x00000004;
pub const INPUT_TYPE_DATETIME_VARIATION_NORMAL: u32 = 0x00000000;
pub const INPUT_TYPE_DATETIME_VARIATION_DATE: u32 = 0x00000010;
pub const INPUT_TYPE_DATETIME_VARIATION_TIME: u32 = 0x00000020;

pub const IME_FLAG_NO_PERSONALIZED_LEARNING: u32 = 0x1000000;
pub const IME_FLAG_NO_FULLSCREEN: u32 = 0x2000000;
pub const IME_FLAG_NAVIGATE_PREVIOUS: u32 = 0x4000000;
pub const IME_FLAG_NAVIGATE_NEXT: u32 = 0x8000000;
pub const IME_FLAG_NO_EXTRACT_UI: u32 = 0x10000000;
pub const IME_FLAG_NO_ACCESSORY_ACTION: u32 = 0x20000000;
pub const IME_FLAG_NO_ENTER_ACTION: u32 = 0x40000000;
pub const IME_FLAG_FORCE_ASCII: u32 = 0x80000000;

pub const CAP_MODE_CHARACTERS: u32 = INPUT_TYPE_TEXT_FLAG_CAP_CHARACTERS;
pub const CAP_MODE_WORDS: u32 = INPUT_TYPE_TEXT_FLAG_CAP_WORDS;
pub const CAP_MODE_SENTENCES: u32 = INPUT_TYPE_TEXT_FLAG_CAP_SENTENCES;

#[repr(transparent)]
pub struct InputMethodManager<'local>(pub JObject<'local>);

impl<'local> InputMethodManager<'local> {
    pub fn show_soft_input(
        &self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        flags: jint,
    ) -> bool {
        env.call_method(
            &self.0,
            "showSoftInput",
            "(Landroid/view/View;I)Z",
            &[(&view.0).into(), flags.into()],
        )
        .unwrap()
        .z()
        .unwrap()
    }

    pub fn hide_soft_input_from_window(
        &self,
        env: &mut JNIEnv<'local>,
        window_token: &IBinder<'local>,
        flags: jint,
    ) -> bool {
        env.call_method(
            &self.0,
            "hideSoftInputFromWindow",
            "(Landroid/os/IBinder;I)Z",
            &[(&window_token.0).into(), flags.into()],
        )
        .unwrap()
        .z()
        .unwrap()
    }

    pub fn restart_input(&self, env: &mut JNIEnv<'local>, view: &View<'local>) {
        env.call_method(
            &self.0,
            "restartInput",
            "(Landroid/view/View;)V",
            &[(&view.0).into()],
        )
        .unwrap()
        .v()
        .unwrap();
    }

    pub fn update_selection(
        &self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        sel_start: jint,
        sel_end: jint,
        candidates_start: jint,
        candidates_end: jint,
    ) {
        env.call_method(
            &self.0,
            "updateSelection",
            "(Landroid/view/View;IIII)V",
            &[
                (&view.0).into(),
                sel_start.into(),
                sel_end.into(),
                candidates_start.into(),
                candidates_end.into(),
            ],
        )
        .unwrap()
        .v()
        .unwrap();
    }
}

#[repr(transparent)]
pub struct EditorInfo<'local>(pub JObject<'local>);

impl<'local> EditorInfo<'local> {
    pub fn set_input_type(&self, env: &mut JNIEnv<'local>, value: u32) {
        env.set_field(&self.0, "inputType", "I", (value as jint).into())
            .unwrap();
    }

    pub fn set_ime_options(&self, env: &mut JNIEnv<'local>, value: u32) {
        env.set_field(&self.0, "imeOptions", "I", (value as jint).into())
            .unwrap();
    }

    pub fn set_initial_sel_start(&self, env: &mut JNIEnv<'local>, value: jint) {
        env.set_field(&self.0, "initialSelStart", "I", value.into())
            .unwrap();
    }

    pub fn set_initial_sel_end(&self, env: &mut JNIEnv<'local>, value: jint) {
        env.set_field(&self.0, "initialSelEnd", "I", value.into())
            .unwrap();
    }

    pub fn set_initial_caps_mode(&self, env: &mut JNIEnv<'local>, value: u32) {
        env.set_field(&self.0, "initialCapsMode", "I", (value as jint).into())
            .unwrap();
    }
}

#[allow(unused_variables)]
pub trait InputConnection {
    fn text_before_cursor<'slf, 'local>(
        &'slf mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        n: jint,
    ) -> Option<Cow<'slf, str>>;
    // TODO: styled version

    fn text_after_cursor<'slf, 'local>(
        &'slf mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        n: jint,
    ) -> Option<Cow<'slf, str>>;
    // TODO: styled version

    fn selected_text<'slf, 'local>(
        &'slf mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
    ) -> Option<Cow<'slf, str>>;
    // TODO: styled version

    fn cursor_caps_mode<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        req_modes: u32,
    ) -> u32;

    // TODO: Do we need to bind getExtractedText? Gio's InputConnection
    // just returns null.

    fn delete_surrounding_text<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        before_length: jint,
        after_length: jint,
    ) -> bool;

    fn delete_surrounding_text_in_code_points<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        before_length: jint,
        after_length: jint,
    ) -> bool;

    fn set_composing_text<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        text: &str,
        new_cursor_position: jint,
    ) -> bool;
    // TODO: styled version

    fn set_composing_region<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        start: jint,
        end: jint,
    ) -> bool;

    fn finish_composing_text<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
    ) -> bool;

    fn commit_text<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        text: &str,
        new_cursor_position: jint,
    ) -> bool {
        self.set_composing_text(env, view, text, new_cursor_position)
            && self.finish_composing_text(env, view)
    }
    // TODO: styled version

    // TODO: Do we need to bind commitCompletion or commitCoorrection?
    // Gio's InputConnection just returns false for both.

    fn set_selection<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        start: jint,
        end: jint,
    ) -> bool;

    fn perform_editor_action<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        editor_action: jint,
    ) -> bool;

    fn perform_context_menu_action<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        id: jint,
    ) -> bool {
        false
    }

    fn begin_batch_edit<'local>(&mut self, env: &mut JNIEnv<'local>, view: &View<'local>) -> bool;

    fn end_batch_edit<'local>(&mut self, env: &mut JNIEnv<'local>, view: &View<'local>) -> bool;

    fn send_key_event<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        event: &KeyEvent<'local>,
    ) -> bool;

    fn clear_meta_key_states<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        states: jint,
    ) -> bool {
        false
    }

    fn report_fullscreen_mode<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        enabled: bool,
    ) -> bool {
        false
    }

    // TODO: Do we need to bind performPrivateCommand? Gio's InputConnection
    // just returns false.

    fn request_cursor_updates<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        view: &View<'local>,
        cursor_update_mode: jint,
    ) -> bool;

    fn close_connection<'local>(&mut self, env: &mut JNIEnv<'local>, view: &View<'local>) {}

    // TODO: Do we need to bind commitContent? Gio's InputConnection
    // just returns false.
}

fn with_input_connection_and_default<F, T>(id: jlong, default: T, f: F) -> T
where
    F: FnOnce(&mut dyn InputConnection) -> T,
{
    with_peer_and_default(id, default, |peer| f(peer.as_input_connection()))
}

fn with_input_connection<F, T: Default>(id: jlong, f: F) -> T
where
    F: FnOnce(&mut dyn InputConnection) -> T,
{
    with_input_connection_and_default(id, T::default(), f)
}

pub(crate) extern "system" fn get_text_before_cursor<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    n: jint,
) -> JString<'local> {
    with_input_connection(peer, |ic| {
        if let Some(result) = ic.text_before_cursor(&mut env, &view, n) {
            env.new_string(result).unwrap()
        } else {
            JObject::null().into()
        }
    })
}

pub(crate) extern "system" fn get_text_after_cursor<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    n: jint,
) -> JString<'local> {
    with_input_connection(peer, |ic| {
        if let Some(result) = ic.text_after_cursor(&mut env, &view, n) {
            env.new_string(result).unwrap()
        } else {
            JObject::null().into()
        }
    })
}

pub(crate) extern "system" fn get_selected_text<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
) -> JString<'local> {
    with_input_connection(peer, |ic| {
        if let Some(result) = ic.selected_text(&mut env, &view) {
            env.new_string(result).unwrap()
        } else {
            JObject::null().into()
        }
    })
}

pub(crate) extern "system" fn get_cursor_caps_mode<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    req_modes: jint,
) -> jint {
    with_input_connection(peer, |ic| {
        ic.cursor_caps_mode(&mut env, &view, req_modes as u32) as jint
    })
}

pub(crate) extern "system" fn delete_surrounding_text<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    before_length: jint,
    after_length: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.delete_surrounding_text(&mut env, &view, before_length, after_length) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn delete_surrounding_text_in_code_points<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    before_length: jint,
    after_length: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.delete_surrounding_text_in_code_points(&mut env, &view, before_length, after_length) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn set_composing_text<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    text: JString<'local>,
    new_cursor_position: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        let text = env.get_string(&text).unwrap();
        let text = Cow::from(&text);
        if ic.set_composing_text(&mut env, &view, &text, new_cursor_position) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn set_composing_region<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    start: jint,
    end: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.set_composing_region(&mut env, &view, start, end) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn finish_composing_text<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.finish_composing_text(&mut env, &view) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn commit_text<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    text: JString<'local>,
    new_cursor_position: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        let text = env.get_string(&text).unwrap();
        let text = Cow::from(&text);
        if ic.commit_text(&mut env, &view, &text, new_cursor_position) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn set_selection<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    start: jint,
    end: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.set_selection(&mut env, &view, start, end) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn perform_editor_action<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    editor_action: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.perform_editor_action(&mut env, &view, editor_action) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn perform_context_menu_action<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    id: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.perform_context_menu_action(&mut env, &view, id) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn begin_batch_edit<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.begin_batch_edit(&mut env, &view) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn end_batch_edit<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.end_batch_edit(&mut env, &view) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn input_connection_send_key_event<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    event: KeyEvent<'local>,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.send_key_event(&mut env, &view, &event) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn input_connection_clear_meta_key_states<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    states: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.clear_meta_key_states(&mut env, &view, states) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn input_connection_report_fullscreen_mode<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    enabled: jboolean,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.report_fullscreen_mode(&mut env, &view, enabled == JNI_TRUE) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn request_cursor_updates<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
    cursor_update_mode: jint,
) -> jboolean {
    with_input_connection(peer, |ic| {
        if ic.request_cursor_updates(&mut env, &view, cursor_update_mode) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    })
}

pub(crate) extern "system" fn close_input_connection<'local>(
    mut env: JNIEnv<'local>,
    view: View<'local>,
    peer: jlong,
) {
    with_input_connection(peer, |ic| {
        ic.close_connection(&mut env, &view);
    })
}

pub fn caps_mode(env: &mut JNIEnv, text: &str, off: usize, req_modes: u32) -> u32 {
    let text = env.new_string(text).unwrap();
    env.call_static_method(
        "android/text/TextUtils",
        "getCapsMode",
        "(Ljava/lang/CharSequence;II)I",
        &[
            (&text).into(),
            (off as jint).into(),
            (req_modes as jint).into(),
        ],
    )
    .unwrap()
    .i()
    .unwrap() as u32
}
