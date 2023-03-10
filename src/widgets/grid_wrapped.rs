use std::fmt::Debug;

use egui::{Color32, Layout, RichText, Sense, Stroke, Ui, Vec2};
use log::{debug, info};

#[derive(Default)]
pub struct GridWrappedBuilder<T: Clone> {
    enabled: bool,
    show_search: bool,
    button_str: String,
    cell_size: Vec2,
    total_items: Vec<T>,
}

#[derive(Clone)]
pub struct GridWrapped<T: Clone> {
    sended: bool,
    enabled: bool,
    show_search: bool,
    search_text: String,
    selected: Option<usize>,
    button_str: String,
    cell_size: Vec2,
    total_items: Vec<T>,
}

impl<T> Default for GridWrapped<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            sended: false,
            enabled: false,
            show_search: true,
            selected: None,
            total_items: Vec::new(),
            search_text: String::new(),
            button_str: String::new(),
            cell_size: Vec2::new(20., 20.),
        }
    }
}

impl<T> GridWrappedBuilder<T>
where
    T: Clone,
{
    pub fn show_search(&mut self) -> &mut Self {
        self.show_search = true;
        self
    }

    pub fn set_enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = value;
        self
    }

    pub fn set_button_text(&mut self, text: &str) -> &mut Self {
        self.button_str = text.to_string();
        self
    }

    pub fn set_cell_size(&mut self, cell_size: impl Into<Vec2>) -> &mut Self {
        self.cell_size = cell_size.into();
        self
    }

    pub fn set_items(&mut self, items: Vec<T>) -> &mut Self {
        self.total_items = items;
        self
    }

    pub fn build(&self) -> GridWrapped<T> {
        info!("Building Wrapped: {}", self.total_items.len());
        GridWrapped {
            enabled: self.enabled,
            show_search: self.show_search || self.total_items.len() > 20,
            button_str: self.button_str.clone(),
            cell_size: self.cell_size,
            total_items: self.total_items.clone(),
            ..Default::default()
        }
    }
}

impl<T> GridWrapped<T>
where
    T: Clone + Debug,
{
    pub fn set_cell_size(&mut self, cell_size: impl Into<Vec2>) -> &mut Self {
        self.cell_size = cell_size.into();
        self
    }

    pub fn set_enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = value;
        self
    }

    pub fn set_items(&mut self, items: Vec<T>) -> &mut Self {
        self.total_items = items;
        self
    }

    pub fn reset(&mut self) {
        self.selected = None;
        self.sended = false;
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        on_btn_click: Option<impl FnOnce()>,
        filter: Option<impl FnMut(usize, &T, &str) -> bool>,
        draw_item: impl FnOnce(&mut Ui, usize, &T) + Copy,
        on_change: impl FnOnce(usize),
    ) {
        if !ui.is_rect_visible(ui.clip_rect()) {
            return;
        }
        ui.vertical(|ui| {
            if self.show_search {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.text_edit_singleline(&mut self.search_text);
                        ui.add_space(10.);
                        ui.label("Search:");
                    });
                });
            }
            let mut items = self.total_items.clone();
            if let Some(mut filter_fn) = filter {
                if !self.search_text.is_empty() {
                    debug!("Filtering items on grid wrapped");
                    items = items
                        .iter()
                        .enumerate()
                        .filter(|(i, item)| filter_fn(*i, *item, self.search_text.as_str()))
                        .map(|(_, i)| i.clone())
                        .collect();
                    info!(
                        "Text: {}\nTotal: {}\nFiltered: {}",
                        self.search_text,
                        self.total_items.len(),
                        items.len(),
                    );
                }
            }
            egui::ScrollArea::vertical()
                .min_scrolled_width(ui.available_width())
                .min_scrolled_height(ui.available_height())
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if let Some(on_btn_click) = on_btn_click {
                            let (rect, _resp) =
                                ui.allocate_at_least(self.cell_size, Sense::click());

                            ui.allocate_ui_at_rect(rect, |ui| {
                                let btn = ui.add_enabled(
                                    self.enabled,
                                    eframe::egui::Button::new(
                                        RichText::new(self.button_str.clone()).size(20.),
                                    )
                                    .wrap(true)
                                    .min_size(self.cell_size),
                                );
                                if btn.clicked() {
                                    on_btn_click();
                                }
                                ui.add_space(5.);
                            });
                        }

                        for (i, item) in items.iter().enumerate() {
                            let (rect, resp) = ui.allocate_at_least(self.cell_size, Sense::click());
                            let mut rect_margin = rect;
                            rect_margin.max.x += 5.;
                            rect_margin.max.y += 5.;

                            let color = if let Some(selected) = self.selected {
                                if selected == i && self.search_text.is_empty() {
                                    Color32::from_gray(64)
                                } else {
                                    Color32::TRANSPARENT
                                }
                            } else {
                                Color32::TRANSPARENT
                            };

                            ui.allocate_ui_at_rect(rect_margin, |ui| {
                                if resp.hovered() && self.enabled {
                                    if self.selected.is_some() && self.selected.unwrap() == i {
                                        ui.painter().rect_filled(rect, 5., color);
                                    }
                                    ui.painter().rect_stroke(
                                        rect,
                                        5.,
                                        Stroke::new(2., Color32::from_gray(64)),
                                    );
                                } else {
                                    ui.painter().rect_filled(rect, 5., color);
                                }
                                draw_item(ui, i, item);
                            });
                            if resp.clicked() && self.enabled {
                                self.selected = Some(i);
                                self.sended = false;
                            }
                        }
                    });
                });
        });
        if let Some(i) = self.selected {
            if !self.sended {
                on_change(i);
                self.sended = true;
            }
        }
    }
}
