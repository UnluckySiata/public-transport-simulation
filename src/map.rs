use egui::{Color32, Pos2};

use crate::node::TransportVariant;

pub struct MapNode {
    pub position: Pos2,
    pub color: Color32,
    pub text: String,
}

impl MapNode {
    pub fn new(
        position: Pos2,
        jammed: bool,
        transport_variant: TransportVariant,
        vehicle_line_left: u32,
        vehicle_line_right: u32,
    ) -> Self {
        let color = if jammed {
            Color32::ORANGE
        } else {
            match transport_variant {
                TransportVariant::Bus => Color32::BLUE,
                TransportVariant::Tram => Color32::GREEN,
            }
        };

        let text = match (vehicle_line_left, vehicle_line_right) {
            (0, 0) => "_ | _".to_owned(),
            (0, v) => format!("_ | {v}"),
            (v, 0) => format!("{v} | _"),
            (v1, v2) => format!("{v1} | {v2}"),
        };

        Self {
            position,
            color,
            text,
        }
    }
}

pub struct Visuals {
    pub nodes: Vec<MapNode>
}
