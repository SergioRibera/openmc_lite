#![allow(unused)]
use std::cell::RefCell;

use egui::{Button, Layout, RichText, Ui};

use crate::{
    data::config_path,
    resources::{icon::Icon, ResourceLoader},
    settings::LauncherSettings,
    widgets::{GridWrapped, GridWrappedBuilder, ImageButton},
    MainState,
};

#[derive(Debug)]
pub enum AccountType {
    Mojang,
    Minecraft,
    Local,
}

enum AccountStep {
    SelectFace,
    Loading,
    View,
}

impl From<String> for AccountType {
    fn from(v: String) -> Self {
        match v.as_str() {
            "MINECRAFT" => Self::Minecraft,
            "MOJANG" | "MICROSOFT" => Self::Mojang,
            _ => Self::Local,
        }
    }
}

pub struct Account {
    account_type: Option<AccountType>,
    is_logged: bool,
    curr_step: AccountStep,
    faces: Vec<(String, Icon)>,
    face_grid: GridWrapped<u8>,
    selected_icon: Option<Icon>,
}

impl Account {
    pub fn new(account_type: Option<AccountType>, cfg: &LauncherSettings) -> Self {
        let is_logged = account_type.is_some();
        let path_icons = config_path("faces");
        let items = path_icons
            .read_dir()
            .unwrap()
            .flatten()
            .filter(|f| f.file_name().to_str().unwrap().ends_with(".png"))
            .flat_map(|f| {
                let path = f.path();
                let path = path.to_str().unwrap();
                let path = path.to_string();
                let icon = Icon::image_from_path(
                    f.file_name().to_str().unwrap(),
                    path.as_str(),
                    egui_extras::image::FitTo::Size(50, 50),
                );
                match icon {
                    Ok(icon) => Ok((path, icon)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<(String, Icon)>>();
        let faces_len = items.len() as u8;

        Self {
            account_type,
            is_logged,
            faces: items,
            curr_step: AccountStep::View,
            selected_icon: if cfg.session.face_img.is_empty() {
                None
            } else {
                Some(
                    Icon::image_from_path(
                        cfg.session.face_img.as_str(),
                        cfg.session.face_img.as_str(),
                        egui_extras::image::FitTo::Size(50, 50),
                    )
                    .unwrap(),
                )
            },
            face_grid: GridWrappedBuilder::default()
                .show_search()
                .set_enabled(true)
                .set_cell_size((70., 70.))
                .set_items((0u8..faces_len).collect::<Vec<u8>>())
                .build(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        res: &ResourceLoader,
        state: &mut MainState,
        cfg: &mut LauncherSettings,
    ) {
        if self.account_type.is_none() {
            self.show_list(ui, res);
        } else {
            let account_type = self.account_type.as_ref().unwrap();
            match self.curr_step {
                AccountStep::SelectFace => self.select_face_view(ui, cfg, state),
                AccountStep::Loading => todo!(),
                AccountStep::View => match account_type {
                    AccountType::Mojang => {}
                    AccountType::Minecraft => {}
                    AccountType::Local => self.show_local(ui, cfg),
                },
                _ => {}
            }
        }
    }

    fn show_list(&mut self, ui: &mut Ui, res: &ResourceLoader) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Choose your Login Type").size(40.).strong());
            ui.add_space(50.);
            let mc_btn = ui
                .add(
                    ImageButton::new(res.mc_btn.texture_id(ui.ctx()), (250., 80.))
                        .set_enabled(false),
                )
                .on_hover_text_at_pointer("Not Implemented!");
            ui.add_space(10.);
            let ms_btn = ui
                .add(
                    ImageButton::new(res.ms_btn.texture_id(ui.ctx()), (250., 80.))
                        .set_enabled(false),
                )
                .on_hover_text_at_pointer("Not Implemented!");
            ui.add_space(10.);
            let lc_btn = ui.add(ImageButton::new(
                res.lc_btn.texture_id(ui.ctx()),
                (250., 80.),
            ));

            if mc_btn.clicked() {
                log::info!("Minecraft Clicked");
                self.curr_step = AccountStep::View;
                self.account_type.replace(AccountType::Minecraft);
            }
            if ms_btn.clicked() {
                log::info!("Microsoft Clicked");
                self.curr_step = AccountStep::View;
                self.account_type.replace(AccountType::Mojang);
            }
            if lc_btn.clicked() {
                log::info!("Local Clicked");
                self.curr_step = AccountStep::View;
                self.account_type.replace(AccountType::Local);
            }
        });
    }

    fn show_local(&mut self, ui: &mut Ui, cfg: &mut LauncherSettings) {
        ui.vertical(|ui| {
            ui.style_mut().spacing.item_spacing = (10., 15.).into();
            self.widget_select_face(ui, cfg);
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.label("User Name:");
                    ui.text_edit_singleline(&mut cfg.session.name);
                });
            });
        });
    }

    fn widget_select_face(&mut self, ui: &mut Ui, cfg: &mut LauncherSettings) {
        ui.vertical_centered(|ui| {
            let resp = if cfg.session.face_img.is_empty() {
                ui.add_sized((200., 200.), Button::new("Add Face"))
            } else {
                let icon = self.selected_icon.as_ref().unwrap();
                ui.add(ImageButton::new(icon.id(ui.ctx()), (200., 200.)))
                    .on_hover_text_at_pointer("Select Face")
            };

            if resp.clicked() {
                self.curr_step = AccountStep::SelectFace;
            }
        });
    }

    fn select_face_view(&mut self, ui: &mut Ui, cfg: &mut LauncherSettings, state: &mut MainState) {
        ui.vertical_centered(|ui| {
            ui.heading(
                RichText::new("Choose your favourite face")
                    .size(40.)
                    .strong(),
            );
            ui.add_space(50.);

            let mut grid = self.face_grid.clone();
            let selected = RefCell::new(String::new());
            grid.show(
                ui,
                None::<fn()>,
                Some(|i: usize, _: &u8, search: &str| {
                    self.faces[i]
                        .0
                        .to_lowercase()
                        .contains(&search.to_lowercase())
                }),
                |ui, _, v| {
                    ui.centered_and_justified(|ui| {
                        ui.image(self.faces[*v as usize].1.id(ui.ctx()), (50., 50.));
                    });
                },
                |s: usize| {
                    selected.replace(self.faces[s].0.clone());
                },
            );
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add_space(10.);
                if ui.button("Cancel").clicked() || ui.button("Accept").clicked() {
                    self.curr_step = AccountStep::View;
                }
            });

            let selected = selected.borrow();
            if !selected.is_empty() {
                cfg.session.face_img = selected.clone();
                self.selected_icon = Some(
                    Icon::image_from_path(
                        selected.as_str(),
                        selected.as_str(),
                        egui_extras::image::FitTo::Size(50, 50),
                    )
                    .unwrap(),
                );
                state.changed_face = true;
            }
            self.face_grid = grid;
        });
    }
}
