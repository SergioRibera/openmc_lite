use egui::{Color32, Layout, RichText};

#[derive(Clone, PartialEq, Eq)]
pub struct Tabs<T: Clone> {
    tabs: Vec<(String, T)>,
    changed: bool,
    font_size: u32,
    text_color: Color32,
    selected: usize,
}

impl<T> Tabs<T>
where
    T: Clone,
{
    pub fn new(
        tabs: &[(&str, T)],
        default_selected: usize,
        font_size: u32,
        text_color: Color32,
    ) -> Self {
        Self {
            font_size,
            text_color,
            changed: false,
            selected: default_selected,
            tabs: tabs
                .iter()
                .map(|t| (t.0.to_string(), t.1.clone()))
                .collect(),
        }
    }

    pub fn add_tab(&mut self, pos: usize, name: &str, tab: T) {
        self.tabs.insert(pos, (name.to_string(), tab));
    }

    pub fn remove_tab(&mut self, pos: usize) {
        self.tabs.remove(pos);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> T {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                ui.style_mut().visuals.button_frame = false;
                for (i, (tab, _)) in self.tabs.iter().enumerate() {
                    let text = if i == self.selected {
                        RichText::new(tab).size(self.font_size as f32).underline()
                    } else {
                        RichText::new(tab).size(self.font_size as f32)
                    };
                    let label = ui.button(text);
                    ui.add_space(10.);
                    if label.clicked() {
                        self.selected = i;
                    }
                }
            });
        });
        self.tabs[self.selected].1.clone()
    }
}
