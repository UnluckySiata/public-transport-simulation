use eframe;
use egui::{Align2, CentralPanel, Context, Grid, SidePanel, Slider, Vec2};

use crate::Simulation;

const NODE_RADIUS: f32 = 20.0;

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt) as f64;

        if !self.paused && !self.done {
            let elapsed_time = dt * self.speed;
            self.done = self.time.increase(elapsed_time);

            self.graph.simulation_iter(elapsed_time, &self.time);

            ctx.request_repaint();
        }

        SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Simulation");

            let button_text = if self.paused { "Run" } else { "Stop" };

            if ui.button(button_text).clicked() {
                self.paused = !self.paused
            }

            ui.add(Slider::new(&mut self.speed, 0.1..=100.0).text("Speed"));
            ui.label(format!("Time: {}", self.time));
            Grid::new("stats").show(ui, |_| {});
        });

        CentralPanel::default().show(ctx, |ui| {
            let visuals = self.graph.generate_visuals();
            let painter = ui.painter_at(ui.max_rect());

            for node in visuals.nodes {
                painter.circle_filled(node.position, NODE_RADIUS, node.color);
                painter.text(
                    node.position,
                    Align2::CENTER_CENTER,
                    &node.text,
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );

                painter.text(
                    node.position + Vec2 { x: 0.0, y: -40.0 },
                    Align2::CENTER_CENTER,
                    &node.label,
                    egui::FontId::proportional(16.0),
                    egui::Color32::WHITE,
                );
            }
            // for edge in &self.edges {
            //     for offset in &edge.parallel_offsets {
            //         let p1 = edge.from.pos + *offset;
            //         let p2 = edge.to.pos + *offset;
            //         painter.line_segment([p1, p2], (2.0, edge.color));
            //     }
            // }
        });
    }
}
