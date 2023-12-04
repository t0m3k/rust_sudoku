use crate::sudoku::board;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    board: board::Board,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            board: board::Board::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("WASM Sudoku");

            let mut focus: Option<(usize, usize)> = None;

            if ui.button("New game").clicked() {
                self.board.generate_puzzle();
            }

            if ui.button("Solve").clicked() {
                self.board.solve();
            }

            if self.board.is_solved() {
                ui.label("Solved!");
            } else {
                ui.label("Not solved");
            }

            ui.add(egui::Separator::default());

            ui.heading("Sudoku");

            egui::Grid::new("sudoku_grid")
                .striped(true)
                .min_row_height(30.0)
                .show(ui, |ui| {
                    for row in 0..9 {
                        ui.horizontal(|ui| {
                            ui.add_space(1.0);
                        });
                        for col in 0..9 {
                            let cell = self.board.get(row, col);
                            let mut text = cell.value.to_string();
                            if cell.value == 0 {
                                text = "".to_string();
                            }

                            let colour = if !self.board.is_safe(row, col, cell.value) {
                                egui::Color32::from_rgb(255, 0, 0)
                            } else {
                                ui.style().visuals.text_color()
                            };

                            if cell.locked {
                                ui.centered_and_justified(|ui| {
                                    ui.colored_label(colour, &text);
                                });
                            } else {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(6.0);
                                    let response = ui.add(
                                        egui::TextEdit::singleline(&mut text)
                                            .desired_width(20.0)
                                            .text_color(colour)
                                            .char_limit(1),
                                    );
                                    if response.changed() {
                                        let parsed = text.parse::<u8>();

                                        if let Ok(value) = parsed {
                                            self.board.set(row, col, value);
                                        } else {
                                            self.board.set(row, col, 0);
                                        }
                                    }
                                    if response.has_focus() {
                                        focus.replace((row, col));
                                    }
                                });
                            }
                            if (col + 1) % 3 == 0 {
                                ui.horizontal(|ui| {
                                    ui.add_space(1.0);
                                });
                            }
                        }
                        ui.end_row();
                        if row % 3 == 2 {
                            ui.vertical(|ui| {
                                ui.add_space(2.0);
                            });
                            ui.end_row();
                        }
                    }
                });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Tomasz Tracz");
    });
}
