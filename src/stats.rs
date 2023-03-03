use crate::{
    settings::{LauncherInstance, LauncherSettings},
    MainState,
};

pub fn show_windows(ctx: &egui::Context, state: &MainState, cfg: &mut LauncherSettings) {
    egui::Window::new("Stats")
        .title_bar(true)
        .movable(true)
        .resizable(true)
        .collapsible(true)
        .default_open(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.collapsing("State", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Subtitle:");
                        ui.add_space(10.);
                        ui.label(state.sub_title.clone());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Create Instance:");
                        ui.add_space(10.);
                        ui.label(state.create_instance.to_string());
                    });
                });

                ui.collapsing("Launcher Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        ui.add_space(10.);
                        ui.label(format!("{:?}", cfg.theme));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Exists Icons:");
                        ui.add_space(10.);
                        ui.label(cfg.exists_icons.to_string());
                    });
                    if let Some(i) = cfg.last_launched.clone() {
                        let mut instance = i.clone();
                        ui.horizontal(|ui| {
                            ui.label("Last Instance Launched:");
                            ui.add_space(10.);
                            render_instance(ui, &mut instance);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Last Instance Launched:");
                            ui.add_space(10.);
                            ui.label("None");
                        });
                    }
                    if cfg.instances.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label("Instances:");
                            ui.add_space(10.);
                            ui.label("None");
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Instances:");
                            ui.add_space(10.);
                            ui.vertical(|ui| {
                                for i in cfg.instances.iter_mut() {
                                    render_instance(ui, i);
                                    ui.add_space(10.);
                                }
                            });
                        });
                    }
                });
            });
        });
}

fn render_instance(ui: &mut egui::Ui, instance: &mut LauncherInstance) {
    let mut downloaded = instance.downloaded;
    ui.collapsing(instance.name.clone(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.add_space(10.);
            ui.label(instance.name.clone());
        });
        ui.horizontal(|ui| {
            ui.label("Version:");
            ui.add_space(10.);
            ui.label(format!("{:?}", instance.version));
        });
        ui.horizontal(|ui| {
            ui.label("Downloaded:");
            ui.add_space(10.);
            ui.checkbox(&mut downloaded, instance.downloaded.to_string())
            // ui.label(instance.downloaded.to_string());
        });
        ui.horizontal(|ui| {
            ui.label("Downloading:");
            ui.add_space(10.);
            ui.label(instance.downloading.to_string());
        });
        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.add_space(10.);
            ui.label(instance.path.clone());
        });
    });
    instance.downloaded = downloaded;
}
