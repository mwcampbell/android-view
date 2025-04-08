// Derived from vello_editor
// Copyright 2024 the Parley Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use accesskit::{Node, TreeUpdate};
use android_view::{KeyEvent, jni::JNIEnv, ndk::event::Keycode};
use core::default::Default;
use parley::{GenericFamily, StyleProperty, editor::SplitString, layout::PositionedLayoutItem};
use std::time::{Duration, Instant};
use vello::{
    Scene,
    kurbo::{Affine, Line, Stroke},
    peniko::color::palette,
    peniko::{Brush, Fill},
};

pub use parley::layout::editor::Generation;
use parley::{FontContext, LayoutContext, PlainEditor, PlainEditorDriver};

use crate::access_ids::next_node_id;

pub const INSET: f32 = 32.0;

pub struct Editor {
    font_cx: FontContext,
    layout_cx: LayoutContext<Brush>,
    editor: PlainEditor<Brush>,
    last_click_time: Option<Instant>,
    click_count: u32,
    pointer_down: bool,
    cursor_pos: (f32, f32),
    cursor_visible: bool,
    //modifiers: Option<Modifiers>, TODO: restore this state if needed
    start_time: Option<Instant>,
    blink_period: Duration,
}

impl Editor {
    pub fn new(text: &str) -> Self {
        let mut editor = PlainEditor::new(32.0);
        editor.set_text(text);
        editor.set_scale(1.0);
        let styles = editor.edit_styles();
        styles.insert(StyleProperty::LineHeight(1.2));
        styles.insert(GenericFamily::SystemUi.into());
        styles.insert(StyleProperty::Brush(palette::css::WHITE.into()));
        Self {
            font_cx: Default::default(),
            layout_cx: Default::default(),
            editor,
            last_click_time: Default::default(),
            click_count: Default::default(),
            pointer_down: Default::default(),
            cursor_pos: Default::default(),
            cursor_visible: Default::default(),
            //modifiers: Default::default(), TODO: restore if needed
            start_time: Default::default(),
            blink_period: Default::default(),
        }
    }

    pub fn driver(&mut self) -> PlainEditorDriver<'_, Brush> {
        self.editor.driver(&mut self.font_cx, &mut self.layout_cx)
    }

    pub fn editor(&self) -> &PlainEditor<Brush> {
        &self.editor
    }

    pub fn editor_mut(&mut self) -> &mut PlainEditor<Brush> {
        &mut self.editor
    }

    pub fn text(&self) -> SplitString<'_> {
        self.editor.text()
    }

    pub fn utf8_to_utf16_index(&self, utf8_index: usize) -> usize {
        let mut utf16_len_so_far = 0usize;
        let mut utf8_len_so_far = 0usize;
        for c in self.editor.raw_text().chars() {
            if utf8_len_so_far >= utf8_index {
                break;
            }
            utf16_len_so_far += c.len_utf16();
            utf8_len_so_far += c.len_utf8();
        }
        utf16_len_so_far
    }

    pub fn utf16_to_utf8_index(&self, utf16_index: usize) -> usize {
        let mut utf16_len_so_far = 0usize;
        let mut utf8_len_so_far = 0usize;
        for c in self.editor.raw_text().chars() {
            if utf16_len_so_far >= utf16_index {
                break;
            }
            utf16_len_so_far += c.len_utf16();
            utf8_len_so_far += c.len_utf8();
        }
        utf8_len_so_far
    }

    pub fn utf8_to_usv_index(&self, utf8_index: usize) -> usize {
        let mut usv_len_so_far = 0usize;
        let mut utf8_len_so_far = 0usize;
        for c in self.editor.raw_text().chars() {
            if utf8_len_so_far >= utf8_index {
                break;
            }
            usv_len_so_far += 1;
            utf8_len_so_far += c.len_utf8();
        }
        usv_len_so_far
    }

    pub fn usv_to_utf8_index(&self, usv_index: usize) -> usize {
        let mut usv_len_so_far = 0usize;
        let mut utf8_len_so_far = 0usize;
        for c in self.editor.raw_text().chars() {
            if usv_len_so_far >= usv_index {
                break;
            }
            usv_len_so_far += 1;
            utf8_len_so_far += c.len_utf8();
        }
        utf8_len_so_far
    }

    pub fn cursor_reset(&mut self) {
        self.start_time = Some(Instant::now());
        // TODO: for real world use, this should be reading from the system settings
        self.blink_period = Duration::from_millis(500);
        self.cursor_visible = true;
    }

    pub fn disable_blink(&mut self) {
        self.start_time = None;
    }

    pub fn next_blink_time(&self) -> Option<Instant> {
        self.start_time.map(|start_time| {
            let phase = Instant::now().duration_since(start_time);

            start_time
                + Duration::from_nanos(
                    ((phase.as_nanos() / self.blink_period.as_nanos() + 1)
                        * self.blink_period.as_nanos()) as u64,
                )
        })
    }

    pub fn cursor_blink(&mut self) {
        self.cursor_visible = self.start_time.is_some_and(|start_time| {
            let elapsed = Instant::now().duration_since(start_time);
            (elapsed.as_millis() / self.blink_period.as_millis()) % 2 == 0
        });
    }

    pub fn on_key_down<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
        key_code: Keycode,
        event: &KeyEvent<'local>,
    ) -> bool {
        self.cursor_reset();
        let mut drv = self.editor.driver(&mut self.font_cx, &mut self.layout_cx);
        let meta_state = event.meta_state(env);
        let shift = meta_state.shift_on();
        let action_mod = meta_state.ctrl_on();

        match key_code {
            // TODO: clipboard commands?
            Keycode::A if action_mod => {
                if shift {
                    drv.collapse_selection();
                } else {
                    drv.select_all();
                }
            }
            Keycode::DpadLeft => {
                if action_mod {
                    if shift {
                        drv.select_word_left();
                    } else {
                        drv.move_word_left();
                    }
                } else if shift {
                    drv.select_left();
                } else {
                    drv.move_left();
                }
            }
            Keycode::DpadRight => {
                if action_mod {
                    if shift {
                        drv.select_word_right();
                    } else {
                        drv.move_word_right();
                    }
                } else if shift {
                    drv.select_right();
                } else {
                    drv.move_right();
                }
            }
            Keycode::DpadUp => {
                if shift {
                    drv.select_up();
                } else {
                    drv.move_up();
                }
            }
            Keycode::DpadDown => {
                if shift {
                    drv.select_down();
                } else {
                    drv.move_down();
                }
            }
            Keycode::MoveHome => {
                if action_mod {
                    if shift {
                        drv.select_to_text_start();
                    } else {
                        drv.move_to_text_start();
                    }
                } else if shift {
                    drv.select_to_line_start();
                } else {
                    drv.move_to_line_start();
                }
            }
            Keycode::MoveEnd => {
                let this = &mut *self;
                let mut drv = this.driver();

                if action_mod {
                    if shift {
                        drv.select_to_text_end();
                    } else {
                        drv.move_to_text_end();
                    }
                } else if shift {
                    drv.select_to_line_end();
                } else {
                    drv.move_to_line_end();
                }
            }
            Keycode::ForwardDel => {
                if action_mod {
                    drv.delete_word();
                } else {
                    drv.delete();
                }
            }
            Keycode::Del => {
                if action_mod {
                    drv.backdelete_word();
                } else {
                    drv.backdelete();
                }
            }
            Keycode::Enter | Keycode::NumpadEnter => {
                drv.insert_or_replace_selection("\n");
            }
            Keycode::Space => {
                drv.insert_or_replace_selection(" ");
            }
            _ => {
                if let Some(c) = event.unicode_char(env) {
                    let mut b = [0u8; 4];
                    let s = c.encode_utf8(&mut b);
                    drv.insert_or_replace_selection(s);
                    return true;
                }
                return false;
            }
        }
        true
    }

    // TODO: motion events

    pub fn handle_accesskit_action_request(&mut self, req: &accesskit::ActionRequest) {
        if req.action == accesskit::Action::SetTextSelection {
            if let Some(accesskit::ActionData::SetTextSelection(selection)) = &req.data {
                self.driver().select_from_accesskit(selection);
            }
        }
    }

    /// Return the current `Generation` of the layout.
    pub fn generation(&self) -> Generation {
        self.editor.generation()
    }

    /// Draw into scene.
    ///
    /// Returns drawn `Generation`.
    pub fn draw(&mut self, scene: &mut Scene) -> Generation {
        let transform = Affine::translate((INSET as f64, INSET as f64));
        self.editor.selection_geometry_with(|rect, _| {
            scene.fill(
                Fill::NonZero,
                transform,
                palette::css::STEEL_BLUE,
                None,
                &rect,
            );
        });
        if self.cursor_visible {
            if let Some(cursor) = self.editor.cursor_geometry(1.5) {
                scene.fill(Fill::NonZero, transform, palette::css::WHITE, None, &cursor);
            }
        }
        let layout = self.editor.layout(&mut self.font_cx, &mut self.layout_cx);
        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let style = glyph_run.style();
                // We draw underlines under the text, then the strikethrough on top, following:
                // https://drafts.csswg.org/css-text-decor/#painting-order
                if let Some(underline) = &style.underline {
                    let underline_brush = &style.brush;
                    let run_metrics = glyph_run.run().metrics();
                    let offset = match underline.offset {
                        Some(offset) => offset,
                        None => run_metrics.underline_offset,
                    };
                    let width = match underline.size {
                        Some(size) => size,
                        None => run_metrics.underline_size,
                    };
                    // The `offset` is the distance from the baseline to the top of the underline
                    // so we move the line down by half the width
                    // Remember that we are using a y-down coordinate system
                    // If there's a custom width, because this is an underline, we want the custom
                    // width to go down from the default expectation
                    let y = glyph_run.baseline() - offset + width / 2.;

                    let line = Line::new(
                        (glyph_run.offset() as f64, y as f64),
                        ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                    );
                    scene.stroke(
                        &Stroke::new(width.into()),
                        transform,
                        underline_brush,
                        None,
                        &line,
                    );
                }
                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let font_size = run.font_size();
                let synthesis = run.synthesis();
                let glyph_xform = synthesis
                    .skew()
                    .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                scene
                    .draw_glyphs(font)
                    .brush(&style.brush)
                    .hint(true)
                    .transform(transform)
                    .glyph_transform(glyph_xform)
                    .font_size(font_size)
                    .normalized_coords(run.normalized_coords())
                    .draw(
                        Fill::NonZero,
                        glyph_run.glyphs().map(|glyph| {
                            let gx = x + glyph.x;
                            let gy = y - glyph.y;
                            x += glyph.advance;
                            vello::Glyph {
                                id: glyph.id as _,
                                x: gx,
                                y: gy,
                            }
                        }),
                    );
                if let Some(strikethrough) = &style.strikethrough {
                    let strikethrough_brush = &style.brush;
                    let run_metrics = glyph_run.run().metrics();
                    let offset = match strikethrough.offset {
                        Some(offset) => offset,
                        None => run_metrics.strikethrough_offset,
                    };
                    let width = match strikethrough.size {
                        Some(size) => size,
                        None => run_metrics.strikethrough_size,
                    };
                    // The `offset` is the distance from the baseline to the *top* of the strikethrough
                    // so we calculate the middle y-position of the strikethrough based on the font's
                    // standard strikethrough width.
                    // Remember that we are using a y-down coordinate system
                    let y = glyph_run.baseline() - offset + run_metrics.strikethrough_size / 2.;

                    let line = Line::new(
                        (glyph_run.offset() as f64, y as f64),
                        ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                    );
                    scene.stroke(
                        &Stroke::new(width.into()),
                        transform,
                        strikethrough_brush,
                        None,
                        &line,
                    );
                }
            }
        }
        self.editor.generation()
    }

    pub fn accessibility(&mut self, update: &mut TreeUpdate, node: &mut Node) {
        let mut drv = self.editor.driver(&mut self.font_cx, &mut self.layout_cx);
        drv.accessibility(update, node, next_node_id, INSET.into(), INSET.into());
    }
}

pub const LOREM: &str = r" Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi cursus mi sed euismod euismod. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam placerat efficitur tellus at semper. Morbi ac risus magna. Donec ut cursus ex. Etiam quis posuere tellus. Mauris posuere dui et turpis mollis, vitae luctus tellus consectetur. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur eu facilisis nisl.

Phasellus in viverra dolor, vitae facilisis est. Maecenas malesuada massa vel ultricies feugiat. Vivamus venenatis et gהתעשייה בנושא האינטרנטa nibh nec pharetra. Phasellus vestibulum elit enim, nec scelerisque orci faucibus id. Vivamus consequat purus sit amet orci egestas, non iaculis massa porttitor. Vestibulum ut eros leo. In fermentum convallis magna in finibus. Donec justo leo, maximus ac laoreet id, volutpat ut elit. Mauris sed leo non neque laoreet faucibus. Aliquam orci arcu, faucibus in molestie eget, ornare non dui. Donec volutpat nulla in fringilla elementum. Aliquam vitae ante egestas ligula tempus vestibulum sit amet sed ante. ";
