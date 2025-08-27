use std::collections::BTreeSet;
use std::cmp::Ord;
use svg::Document;
use svg::node::element::{Ellipse, Circle, Text};
use svg::node::Node;
use num::FromPrimitive;
use num::Signed;
use num::rational::Rational64;

fn main() {
    fn p ((x, y): (Rational64, Rational64)) -> bool {
        (x.recip() - y.recip()).fract().abs() < Rational64::from_f64(0.00001).unwrap()
    }
    let relation = Relation {
        domain: into_set(vec![-3., -2., -1., 1., 2., 3.]),
        codomain: into_set(vec![-3., -2., -1., 1., 2., 3.]),
        p: &p
    };

    let draw_domain = render_set(relation.domain, 1.5, 0.5);
    let draw_codomain = render_set(relation.codomain, 4.5, 0.5);


    let mut document = Document::new()
        .set("width", "6in")
        .set("height", "5in");

    for item in draw_domain.into_iter().chain(draw_codomain.into_iter()) {
        document = document.add(item)
    }

    svg::save("diagram.svg", &document).unwrap();

}

/// Set, and starting coordinates in inches
fn render_set(set: BTreeSet<Rational64>, x: f64, y: f64) -> Vec<Box<dyn Node>> {
    let mut to_draw: Vec<Box<dyn Node>> = Vec::new();

    const SPACING: f64 = 0.70; // space in inches between bullet points
    let center_y: f64 = SPACING * set.len() as f64 / 2.0 + y;

    let elipse = Ellipse::new()
        .set("cx", format!("{}in", x))
        .set("cy", format!("{}in", center_y))
        .set("rx", "1in")
        .set("ry", format!("{}in", center_y - 0.2))
        .set("stroke", "black")
        .set("fill", "white")
        .set("stroke-width", "0.05in");
    to_draw.push(Box::new(elipse));

    // The median node should be at the center of the ellipse
    let center = set.len() as f64 / 2.0 - 0.5;
    for (i, element) in set.iter().enumerate() {
        let pos_y = (i as f64 - center) * SPACING + center_y;
        let dot = Circle::new()
            .set("cy", format!("{}in", pos_y))
            .set("cx", format!("{}in", x))
            .set("r", "0.1in")
            .set("fill", "black");

        let text = Text::new(format!("{}", element))
            .set("x", format!("{}in", x))
            .set("y", format!("{}in", pos_y - 0.2))
            .set("font-size", "30")
            .set("text-anchor", "middle");
        to_draw.push(Box::new(dot));
        to_draw.push(Box::new(text));
    }

    to_draw
}


fn into_set(vec: Vec<f64>) -> BTreeSet<Rational64> {
    let mut set = BTreeSet::new();
    for item in vec {
        set.insert(Rational64::from_f64(item).unwrap());
    }
    set
}
        

struct Relation<'p, T> {
    domain: BTreeSet<T>,
    codomain: BTreeSet<T>,
    p: &'p dyn Fn((T, T)) -> bool
}

impl<T> Relation<'_, T> {
    fn cartesian_product(&self) -> Vec<(&T, &T)> {
        let mut product = Vec::with_capacity(self.domain.len() * self.codomain.len());
        for x in self.domain.iter() {
            for y in self.codomain.iter() {
                product.push((x, y));
            }
        }
        product
    }
}
