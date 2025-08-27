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

    
    let mut to_draw: Vec<Box<dyn Node>> = Vec::new();

    const SPACING: f64 = 0.70; // space in inches between bullet points
    const CENTER_Y: f64 = 2.5;


    let left_elipse = Ellipse::new()
        .set("cx", "1.5in")
        .set("cy", format!("{}in", CENTER_Y))
        .set("rx", "1in")
        .set("ry", "2in")
        .set("stroke", "black")
        .set("fill", "white")
        .set("stroke-width", "0.05in");

    // The median node should be at the center of the ellipse
    let center = relation.domain.len() as f64 / 2.0 - 0.5;
    for (i, element) in relation.domain.iter().enumerate() {
        let pos_y = (i as f64 - center) * SPACING + CENTER_Y;
        let dot = Circle::new()
            .set("cy", format!("{}in", pos_y))
            .set("cx", "1.5in")
            .set("r", "0.1in")
            .set("fill", "black");
        let text = Text::new("text")
            .set("x", "1.5in")
            .set("y", format!("{}in", pos_y - 0.2))
            .set("font-size", "20");
        to_draw.push(Box::new(dot));
        to_draw.push(Box::new(text));
    }


    let mut document = Document::new()
        .set("width", "5in")
        .set("height", "5in")
        .add(left_elipse);

    for item in to_draw { document = document.add(item) }

    svg::save("diagram.svg", &document).unwrap();

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
