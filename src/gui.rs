use crate::PestCompParameters;
use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use egui::*;
use egui_baseview::*;
use std::sync::Arc;
use vst::editor::Editor;

const WINDOW_HEIGHT: usize = 240;
const WINDOW_WIDTH: usize = 860;

pub struct PestCompEditor {
    pub params: Arc<PestCompParameters>,
    pub window_handle: Option<WindowParent>,
    pub is_open: bool,
}

impl Editor for PestCompEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
        if let Some(mut window_handle) = self.window_handle.take() {
            (window_handle.0).close();
        }
    }

    fn open(&mut self, parent: *mut std::os::raw::c_void) -> bool {
        match self.is_open {
            true => false,
            false => {
                self.is_open = true;
                let settings = Settings {
                    window: WindowOpenOptions {
                        title: "PestComp".to_string(),
                        size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
                        scale: WindowScalePolicy::ScaleFactor(2.0),
                    },
                    render_settings: RenderSettings::default(),
                };

                let mut font_defs = FontDefinitions::default();
                font_defs.font_data.insert(
                    "olde_english".to_owned(),
                    std::borrow::Cow::Borrowed(include_bytes!("OldeEnglish.ttf")),
                );
                font_defs
                    .fonts_for_family
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "olde_english".to_owned());
                font_defs
                    .family_and_size
                    .insert(TextStyle::Heading, (FontFamily::Proportional, 70.0));
                font_defs
                    .family_and_size
                    .insert(TextStyle::Body, (FontFamily::Proportional, 15.0));
                font_defs
                    .family_and_size
                    .insert(TextStyle::Button, (FontFamily::Proportional, 30.0));

                let window_handle = EguiWindow::open_parented(
                    &VstParent(parent),
                    settings,
                    self.params.clone(),
                    |_, _, _| {},
                    move |ctx: &CtxRef, _, state: &mut Arc<PestCompParameters>| {
                        ctx.set_fonts(font_defs.clone());
                        CentralPanel::default().show(ctx, |ui| {
                            ui.style_mut().spacing.item_spacing.y = 20.0;
                            ui.style_mut().spacing.item_spacing.x = 40.0;
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.add(
                                        Label::new("PestCömp")
                                            .heading()
                                            .text_color(Color32::from_rgb(65, 54, 104)),
                                    );
                                    ui.label("2022 © Przychlast Records Inc.");
                                });
                                let val = state.engage.get();
                                let mut checked: bool = if val < 0.5 { false } else { true };
                                ui.add(
                                    Checkbox::new(&mut checked, "Engage")
                                        .text_style(TextStyle::Button),
                                );
                                state.engage.set(if checked { 1.0 } else { 0.0 });
                            });
                        });
                    },
                );

                self.window_handle = Some(WindowParent(window_handle));
                true
            }
        }
    }
}

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

struct VstParent(*mut std::ffi::c_void);
unsafe impl Send for VstParent {}

pub struct WindowParent(WindowHandle);
unsafe impl Send for WindowParent {}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;
        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}
