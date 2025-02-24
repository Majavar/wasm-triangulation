use std::ops::Deref;

use super::{
    geometry::Point,
    graph_datastructure::{Graph, GraphFace, GraphEdge, GraphVertex, graph},
};

#[derive(Debug)]
pub struct Delaunay(Graph);

impl Deref for Delaunay {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn bbox_center(points: &[Point]) -> Option<Point> {
    let mut iter = points.iter();

    iter.next()
        .map(|first| {
            iter.fold(
                (first.x, first.x, first.y, first.y),
                |(min_x, max_x, min_y, max_y), point| {
                    (
                        min_x.min(point.x),
                        max_x.max(point.x),
                        min_y.min(point.y),
                        max_y.max(point.y),
                    )
                },
            )
        })
        .map(|(min_x, max_x, min_y, max_y)| Point {
            x: (min_x + max_x) / 2.0,
            y: (min_y + max_y) / 2.0,
        })
}

fn find_closest_to_position(points: &[Point], point: Point) -> Option<usize> {
    let mut iter = points
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.distance_squared(&point)));

    iter.next()
        .map(|first| {
            iter.fold(
                first,
                |min, current| if min.1 < current.1 { min } else { current },
            )
        })
        .map(|(i, _)| i)
}

fn find_closest_to_vertex(points: &[Point], index: usize) -> Option<(usize, usize)> {
    let mut iter = points
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(i, p)| (i, p.distance_squared(&points[index])));

    iter.next()
        .map(|first| {
            iter.fold(
                first,
                |min, current| if min.1 < current.1 { min } else { current },
            )
        })
        .map(|(i, _)| (index, i))
}

fn find_delaunay_triangle(
    points: &[Point],
    v0: usize,
    v1: usize,
) -> Option<(usize, usize, usize)> {
    let mut iter = points
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != v0 && *i != v1)
        .map(|(i, p)| (i, Point::square_circumradius(&points[v0], &points[v1], p)));

    iter.next()
        .map(|first| {
            iter.fold(
                first,
                |min, current| if min.1 < current.1 { min } else { current },
            )
        })
        .map(|(i, _)| (v0, v1, i))
}

fn find_seed_triangle(points: &[Point]) -> Option<(usize, usize, usize)> {
    bbox_center(points)
        .and_then(|center| find_closest_to_position(points, center))
        .and_then(|v0| find_closest_to_vertex(points, v0))
        .and_then(|(v0, v1)| find_delaunay_triangle(points, v0, v1))
        .map(|(v0, v1, v2)| {
            if Point::is_ccw(&points[v0], &points[v1], &points[v2]) {
                (v0, v1, v2)
            } else {
                (v0, v2, v1)
            }
        })
}

fn add_seed_triangle(
    edges: &mut Vec<GraphEdge>,
    faces: &mut Vec<GraphFace>,
    vertices: &mut Vec<GraphVertex>,
    i0: usize,
    i1: usize,
    i2: usize,
) {
    vertices.push(GraphVertex::new(None, 3));
    vertices.push(GraphVertex::new(Some(i2), 0));
    vertices.push(GraphVertex::new(Some(i1), 7));
    vertices.push(GraphVertex::new(Some(i0), 1));

    faces.push(GraphFace::new(0));
    faces.push(GraphFace::new(1));
    faces.push(GraphFace::new(6));
    faces.push(GraphFace::new(8));

    edges.push(GraphEdge::new(3, 2, 1));
    edges.push(GraphEdge::new(1, 6, 0));
    edges.push(GraphEdge::new(0, 9, 0));
    edges.push(GraphEdge::new(1, 4, 3));
    edges.push(GraphEdge::new(3, 10, 0));
    edges.push(GraphEdge::new(0, 1, 2));
    edges.push(GraphEdge::new(2, 5, 1));
    edges.push(GraphEdge::new(3, 8, 2));
    edges.push(GraphEdge::new(1, 11, 1));
    edges.push(GraphEdge::new(2, 0, 3));
    edges.push(GraphEdge::new(2, 3, 2));
    edges.push(GraphEdge::new(0, 7, 3));
}

fn find_visible_edge(edges: &[GraphEdge], vertices: &[GraphVertex], points: &[Point], position: usize) -> Option<(usize, bool)> {
    let initial = vertices[0].edge;
    let mut current = initial;
    let mut current_position = vertices[edges[current].vertex].position.unwrap();

    loop {
        let next = edges[current].next;
        let next_position = vertices[edges[next].vertex].position.unwrap();

        if !Point::is_ccw(&points[position], &points[current_position], &points[next_position]) {
            break Some((current, current == initial));
        }

        current = next;
        current_position = next_position;

        if current == initial {
            break None;
        }
    }
}

fn add_triangle(
    edges: &mut Vec<GraphEdge>,
    faces: &mut Vec<GraphFace>,
    vertices: &mut [GraphVertex],
    vertex: usize,
    current_edge: usize,
    next_edge: usize,
) -> usize {
    let current_vertex = edges[current_edge].vertex;
    let next_vertex = edges[next_edge].vertex;
    let face = edges[next_edge].face;
    let opposite_edge = edges[next_edge ^ 1].next;

    let new_current_face = faces.len();
    let new_next_face = new_current_face + 1;

    let edge = edges.len();

    faces.push(GraphFace::new(edge));
    faces.push(GraphFace::new(edge + 4));

    edges.push(GraphEdge::new(vertex, current_edge ^ 1, face));
    edges.push(GraphEdge::new(current_vertex, edge + 4, new_current_face));
    edges.push(GraphEdge::new(0, edge + 1, new_next_face));
    edges.push(GraphEdge::new(vertex, next_edge, new_current_face));
    edges.push(GraphEdge::new(next_vertex, edge + 2, face));
    edges.push(GraphEdge::new(vertex, opposite_edge, new_next_face));

    edges[next_edge ^ 1].next = edge + 5;
    edges[current_edge].next = edge + 3;
    edges[opposite_edge ^ 1].next = edge;

    edges[current_edge ^1].face = new_current_face;
    edges[next_edge].face  =new_next_face;

    vertices[vertex].edge = edge + 1;
    vertices[0].edge = edge + 3;

    opposite_edge
}

fn legalize(
    edges: &mut Vec<GraphEdge>,
    faces: &mut Vec<GraphFace>,
    vertices: &mut Vec<GraphVertex>,
    points: &[Point],
    t0e0: usize,
) {
    let t1e1 = edges[t0e0].next;
    let p = edges[t1e1].vertex;

    if p != 0 {
        let t1e0 = t0e0 ^ 1;
        let t0e1 = edges[t1e0].next;
        let t0e2 = edges[t0e1 ^ 1].next;

        let va = edges[t0e0].vertex;
        let vb = edges[t1e0].vertex;
        let v0 = edges[t0e1].vertex;
        let v1 = edges[t1e1].vertex;

        let p0 = vertices[v0].position.unwrap();
        let pa = vertices[va].position.unwrap();
        let pb = vertices[vb].position.unwrap();
        let p1 = vertices[v1].position.unwrap();

        if points[p0].in_circle(&points[pa], &points[pb], &points[p1]) {
            let t1e2 = edges[t1e1 ^ 1].next;
            let t0 = edges[t0e2].face;
            let t1 = edges[t1e2].face;
            vertices[va].edge = t0e1;
            vertices[vb].edge = t1e1;
            edges[t0e2 ^ 1].next = t1e1;
            edges[t1e2 ^ 1].next = t0e1;
            edges[t0e0].vertex = edges[t1e1].vertex;
            edges[t1e0].vertex = edges[t0e1].vertex;
            edges[t0e0].next = t0e2;
            edges[t1e0].next = t1e2;
            edges[t0e1 ^ 1].next = t0e0;
            edges[t1e1 ^ 1].next = t1e0;
            edges[t0e2].face = t1;
            edges[t1e2].face = t0;
            faces[t0].edge = t0e1 ^ 1;
            faces[t1].edge = t1e1 ^ 1;
            legalize(edges, faces, vertices, points, t1e1);
            legalize(edges, faces, vertices, points, t1e2);
        }
    }
}

impl Delaunay {
    pub fn from(points: Box<[Point]>) -> Result<Self, ()> {
        let len = points.len() + 1;

        let mut edges = Vec::with_capacity(len * 6);
        let mut faces = Vec::with_capacity(len * 2);
        let mut vertices = Vec::with_capacity(len);

        let (i0, i1, i2) = find_seed_triangle(&points).ok_or(())?;
        add_seed_triangle(&mut edges, &mut faces, &mut vertices, i0, i1, i2);
        let center = Point::circumcenter(&points[i0], &points[i1], &points[i2]);

        let mut dists = points.iter().enumerate().map(|(i, p)| (i, center.distance_squared(p))).collect::<Vec<_>>();
        dists.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for new_point in dists.iter().skip(3).map(|d| d.0) {
            if new_point == i0 || new_point == i1 || new_point == i2 {
                continue;
            }

            if let Some((edge, walk_back)) = find_visible_edge(&edges, &vertices, &points, new_point) {
                let vertex = vertices.len();
                vertices.push(GraphVertex::new(Some(new_point), 0));

                let mut current = edge;
                let mut current_position;

                let mut next = edges[current].next;
                let mut next_vertex = edges[next].vertex;
                let mut next_position = vertices[next_vertex].position.unwrap();

                let mut previous = edges[edges[current ^ 1].next ^ 1].next ^ 1;
                let e = add_triangle(&mut edges, &mut faces, &mut vertices, vertex, current, next);
                legalize(&mut edges, &mut faces, &mut vertices, &points, e);

                let new_edge = vertices[0].edge;

                loop {
                    current = next;
                    current_position = next_position;

                    next = edges[current].next;
                    next_vertex = edges[next].vertex;
                    next_position = vertices[next_vertex].position.unwrap();

                    if Point::is_ccw(&points[new_point], &points[current_position], &points[next_position]) {
                        break;
                    }

                    let edge_1 = edges[next ^ 1].next;
                    let edge_2 = edges[current ^ 1].next;
                    let face_1 = edges[next].face;
                    let face_2 = edges[current].face;

                    edges[new_edge].next = next;

                    edges[edge_2].face = face_1;
                    faces[face_2].edge = current ^ 1;

                    edges[current].vertex = vertex;
                    edges[current ^ 1].vertex = edges[next].vertex;

                    edges[current].next = edge_1;
                    edges[current ^ 1].next = new_edge ^ 1;

                    edges[next].face = face_2;
                    edges[next ^ 1].next = current;
                    edges[edge_2 ^ 1].next = current ^ 1;
                    edges[edge_1 ^ 1].next = edge_2;

                    legalize(&mut edges, &mut faces, &mut vertices, &points, edge_1);
                }

                if walk_back {
                    let mut current = edge;
                    let current_vertex = edges[current].vertex;
                    let mut current_position = vertices[current_vertex].position.unwrap();

                    let mut previous_vertex = edges[previous].vertex;
                    let mut previous_position = vertices[previous_vertex].position.unwrap();

                    while !Point::is_ccw(&points[new_point], &points[previous_position], &points[current_position]) {
                        let edge_1 = edges[new_edge ^ 1].next;
                        let edge_2 = edges[current ^ 1].next;
                        let face_1 = edges[new_edge].face;
                        let face_2 = edges[current].face;

                        edges[previous].next = new_edge;

                        edges[edge_2].face = face_1;
                        faces[face_2].edge = current ^ 1;

                        edges[current].vertex = edges[previous].vertex;
                        edges[current ^ 1].vertex = vertex;

                        edges[current].next = edge_1;
                        edges[current ^ 1].next = previous ^ 1;

                        edges[new_edge].face = face_2;
                        edges[new_edge ^ 1].next = current;
                        edges[edge_2 ^ 1].next = current ^ 1;
                        edges[edge_1 ^ 1].next = edge_2;

                        legalize(&mut edges, &mut faces, &mut vertices, &points, edge_2);

                        current = previous;
                        current_position = previous_position;

                        previous = edges[edges[current ^ 1].next ^ 1].next ^ 1;
                        previous_vertex = edges[previous].vertex;
                        previous_position = vertices[previous_vertex].position.unwrap();
                    }
                }
            }
        }

        Ok(Delaunay(graph(
            points,
            edges.into_boxed_slice(),
            faces.into_boxed_slice(),
            vertices.into_boxed_slice(),
        )))
    }
}

impl FromIterator<Point> for Delaunay {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let points = iter.into_iter().collect::<Vec<_>>().into_boxed_slice();
        Delaunay::from(points).unwrap()
    }
}
