use std::{cell::RefCell, collections::HashSet, rc::Rc, vec};

#[derive(Debug)]
struct Circuit {
    connections: Vec<Rc<JunctionBox>>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Edge(Rc<JunctionBox>, Rc<JunctionBox>);

impl Circuit {
    fn contains(&self, target: &Rc<JunctionBox>) -> bool {
        self.connections.contains(target)
    }
}

impl Edge {
    fn weight(&self) -> f64 {
        let &JunctionBox {
            x: p1,
            y: p2,
            z: p3,
        } = &*self.0;

        let &JunctionBox {
            x: q1,
            y: q2,
            z: q3,
        } = &*self.1;

        (((p1 - q1).pow(2) + (p2 - q2).pow(2) + (p3 - q3).pow(2)) as f64).sqrt()
    }
}

fn parse_inputs(input: &str) -> anyhow::Result<Vec<Rc<JunctionBox>>> {
    input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            if let &[x, y, z] = s
                .splitn(3, ',')
                .map(|s| Ok(s.parse()?))
                .collect::<anyhow::Result<Vec<_>>>()?
                .as_slice()
            {
                Ok(Rc::new(JunctionBox { x, y, z }))
            } else {
                Err(anyhow::anyhow!("Invalid coordinates: `{s}`"))
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const PAIRS: usize = 1000;
    let jboxes = parse_inputs(input)?;

    // Create edges for all junction boxes
    let mut edges = HashSet::with_capacity(jboxes.len() * (jboxes.len() - 1) / 2);

    for jbox in &jboxes {
        for jbox2 in &jboxes {
            if jbox != jbox2 {
                let (u, v) = if jbox < jbox2 {
                    (jbox, jbox2)
                } else {
                    (jbox2, jbox)
                };

                let edge = Edge(Rc::clone(u), Rc::clone(v));
                edges.insert(edge);
            }
        }
    }

    // Kruskal's algorithm
    let mut edges = edges.into_iter().collect::<Vec<_>>();
    edges.sort_by(|a, b| a.weight().partial_cmp(&b.weight()).unwrap());

    let mut circuits: Vec<Rc<RefCell<Circuit>>> = Vec::new();

    for edge in edges.into_iter().take(PAIRS) {
        let a = edge.0;
        let b = edge.1;

        let mut circuit_a: Option<Rc<RefCell<Circuit>>> = None;
        let mut circuit_a_index = None;

        let mut circuit_b: Option<Rc<RefCell<Circuit>>> = None;
        let mut circuit_b_index = None;

        for (index, circuit) in circuits.iter().enumerate() {
            if circuit_a.is_some() && circuit_b.is_some() {
                break;
            }

            if circuit_a.is_none() && circuit.borrow().contains(&a) {
                circuit_a = Some(Rc::clone(circuit));
                circuit_a_index = Some(index);
            }

            if circuit_b.is_none() && circuit.borrow().contains(&b) {
                circuit_b = Some(Rc::clone(circuit));
                circuit_b_index = Some(index);
            }
        }

        match (circuit_a, circuit_b) {
            (None, None) => circuits.push(Rc::new(RefCell::new(Circuit {
                connections: vec![a, b],
            }))),
            (Some(circuit_a), None) => circuit_a.borrow_mut().connections.push(b),
            (None, Some(circuit_b)) => circuit_b.borrow_mut().connections.push(a),
            (Some(circuit_a), Some(circuit_b)) => {
                if Rc::ptr_eq(&circuit_a, &circuit_b) {
                    // Cycle detected
                    continue;
                }

                let (larger, (smaller, smaller_index)) = if circuit_a.borrow().connections.len()
                    < circuit_b.borrow().connections.len()
                {
                    (circuit_b, (circuit_a, circuit_a_index.unwrap()))
                } else {
                    (circuit_a, (circuit_b, circuit_b_index.unwrap()))
                };

                // Put smaller's boxes in larger
                larger
                    .borrow_mut()
                    .connections
                    .extend(smaller.borrow().connections.iter().cloned());

                // Remove the smaller one from the list
                circuits.swap_remove(smaller_index);
            }
        }
    }

    // Sort to get the top 3
    circuits.sort_by_key(|e| e.borrow().connections.len());

    let product = circuits
        .iter()
        .rev()
        .take(3)
        .map(|e| e.borrow().connections.len())
        .product::<usize>();

    println!("Answer: {}", product);

    Ok(())
}
