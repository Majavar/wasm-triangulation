use crate::{
    graph::{Delaunay, Point},
    settings::Settings,
};
use dioxus::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    settings: Settings,
}

#[component]
pub fn Content(props: Props) -> Element {
    let mut rng = StdRng::seed_from_u64(props.settings.seed);
    let graph = Delaunay::from(
        (0..props.settings.num_seeds)
            .map(|_| Point {
                x: rng.random_range(0.0..=1.0),
                y: rng.random_range(0.0..=1.0),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
    .unwrap();

    rsx! {
        div {
            class: "grow flex justify-center overflow-hidden",
            svg {
                class: "max-w-full max-h-full",
                view_box: "0 0 1 1",
                for e in graph.edges() {
                    if let (Some(v1), Some(v2)) = (e.vertices().0.position(), e.vertices().1.position()) {
                        line {
                            x1: v1.x,
                            y1: v1.y,
                            x2: v2.x,
                            y2: v2.y,
                            stroke: "red",
                            "stroke-width": "0.003"
                        }
                    }
                }
                for v in graph.vertices() {
                    if let Some(p) = v.position() {
                        circle {
                            cx: p.x,
                            cy: p.y,
                            r: "0.001",
                            stroke: "black",
                            "stroke-width": "0.003",
                            fill: "black"
                        }
                    }
                }
            }
        }
    }
}
