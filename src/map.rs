use egui::{Color32, Pos2};

use crate::node::{NodeVariant, TransportVariant};

pub struct MapNode {
    pub position: Pos2,
    pub color: Color32,
    pub text: String,
    pub label: String,
}

impl MapNode {
    pub fn new(
        position: Pos2,
        jammed: bool,
        transport_variant: TransportVariant,
        node_variant: NodeVariant,
        vehicle_line_left: u32,
        vehicle_line_right: u32,
        label: String,
    ) -> Self {
        let color = if jammed {
            Color32::ORANGE
        } else {
            match (transport_variant, node_variant) {
                (TransportVariant::Bus, NodeVariant::Stop) => Color32::BROWN,
                (TransportVariant::Bus, NodeVariant::Regular) => Color32::BLUE,
                (TransportVariant::Tram, _) => Color32::GREEN,
                _ => Color32::LIGHT_GRAY,
            }
        };

        let text = match (vehicle_line_left, vehicle_line_right) {
            (0, 0) => "X | X".to_owned(),
            (0, v) => format!("X | {v}"),
            (v, 0) => format!("{v} | X"),
            (v1, v2) => format!("{v1} | {v2}"),
        };

        Self {
            position,
            color,
            text,
            label,
        }
    }
}

pub struct Visuals {
    pub nodes: Vec<MapNode>
}
