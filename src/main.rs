use std::collections::{BTreeSet, BTreeMap};
use std::cmp::Ord;
use std::iter::once;
use std::vec;
use svg::Document;
use svg::node::element::{Ellipse, Circle, Text, Line, Definitions, Path, Marker};
use svg::node::Node;
use num::FromPrimitive;
use num::Signed;
use num::rational::Rational64;

fn main() {
    fn p ((x, y): (&Rational64, &Rational64)) -> bool {
        (x.recip() - y.recip()).fract().abs() < Rational64::from_f64(0.00001).unwrap()
    }
    let relation = Relation {
        domain: into_set(vec![-3., -2., -1., 1., 2., 3.]),
        codomain: into_set(vec![-3., -2., -1., 1., 2., 3.]),
        p: &p
    };

    let (draw_domain, domain_loc) = render_set(&relation.domain, 1.5, 0.25);
    let (draw_codomain, codomain_loc) = render_set(&relation.codomain, 4.5, 0.25);

    let one = Rational64::from_f64(1.0).unwrap();
    println!("{:?}", domain_loc.get(&one).unwrap());

    let draw_arrows = relation.cartesian_product()
        .into_iter().filter(|tup| (relation.p)(*tup)).map(|(x, y)| {
        // the |(x, y)| are elements of the domain & co-domain, not co-ords
        let (x1, y1) = domain_loc.get(x).unwrap();
        let (x2, y2) = codomain_loc.get(y).unwrap();
        
        // TODO: make offset a distance rather than percentage
        const OFFSET: f64 = 0.22;
        let (center_x, center_y) = ((x1+x2)/2.0, (y1+y2)/2.0);
        // let x1 = x1*(1.0-OFFSET) + center_x*OFFSET;
        let x2 = x2*(1.0-OFFSET) + center_x*OFFSET;
        // let y1 = y1*(1.0-OFFSET) + center_y*OFFSET;
        let y2 = y2*(1.0-OFFSET) + center_y*OFFSET;

        Box::new(Line::new()
            .set("x1", format!("{}in", x1))
            .set("y1", format!("{}in", y1))
            .set("x2", format!("{}in", x2))
            .set("y2", format!("{}in", y2))
            .set("marker-end", "url(#arrow)")
            .set("stroke", "black")
            .set("stroke-width", "0.03in")) as Box<dyn Node>
    });

    let arrow_heads = Marker::new()
        .set("id", "arrow")
        .set("refY", "2")
        .set("markerUnits", "strokeWidth")
        .set("markerWidth", "4")
        .set("markerHeight", "4")
        .set("orient", "auto")
        .add(Path::new().set("d", "M 0 0 L 4 2 L 0 4 z").set("fill", "black"));

    let mut document = Document::new()
        .set("width", "6in") // hardcoded doc size is not ideal
        .set("height", "6in")
        .add(Definitions::new().add(arrow_heads));

    let to_draw = draw_domain
        .chain(draw_codomain)
        .chain(draw_arrows);

    for item in to_draw {
        document = document.add(item)
    }

    svg::save("diagram.svg", &document).unwrap();

}

/// Set, and starting coordinates in inches
fn render_set(set: &BTreeSet<Rational64>, x: f64, y: f64) ->
(vec::IntoIter<Box<dyn Node>>, BTreeMap<Rational64, (f64, f64)>) {
    // The elements to be rendered
    let mut to_draw: Vec<Box<dyn Node>> = Vec::new();
    // So that we know where each number is
    let mut elements = BTreeMap::new();

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
        let pos_y = (i as f64 - center) * SPACING + center_y + 0.2;
        let dot = Circle::new()
            .set("cx", format!("{}in", x))
            .set("cy", format!("{}in", pos_y))
            .set("r", "0.1in")
            .set("fill", "black");

        let text = Text::new(format!("{}", element))
            .set("x", format!("{}in", x))
            .set("y", format!("{}in", pos_y - 0.2))
            .set("font-size", "30")
            .set("text-anchor", "middle");
        to_draw.push(Box::new(dot));
        to_draw.push(Box::new(text));
        elements.insert(*element, (x, pos_y));
    }

    (to_draw.into_iter(), elements)
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
    p: &'p dyn Fn((&T, &T)) -> bool
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
