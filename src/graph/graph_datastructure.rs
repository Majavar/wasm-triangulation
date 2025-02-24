use super::Point;
use std::iter::successors;

#[derive(Debug)]
pub struct GraphEdge {
    pub vertex: usize,
    pub next: usize,
    pub face: usize,
}

impl GraphEdge {
    pub fn new(vertex: usize, next: usize, face: usize) -> GraphEdge {
        GraphEdge { vertex, next, face }
    }
}

#[derive(Debug)]
pub struct GraphFace {
    pub edge: usize,
}

impl GraphFace {
    pub fn new(edge: usize) -> GraphFace {
        GraphFace { edge }
    }
}

#[derive(Debug)]
pub struct GraphVertex {
    pub position: Option<usize>,
    pub edge: usize,
}

impl GraphVertex {
    pub fn new(position: Option<usize>, edge: usize) -> GraphVertex {
        GraphVertex { position, edge }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge<'a> {
    graph: &'a Graph,
    index: usize,
}

impl<'a> Edge<'a> {
    pub fn id(&self) -> usize {
        self.index / 2
    }

    pub fn vertices(&self) -> (Vertex<'a>, Vertex<'a>) {
        let id = self.index;

        let left = Vertex {
            graph: self.graph,
            index: self.graph.edges[id].vertex,
        };

        let right = Vertex {
            graph: self.graph,
            index: self.graph.edges[id ^ 1].vertex,
        };

        (left, right)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Face<'a> {
    graph: &'a Graph,
    index: usize,
}

impl<'a> Face<'a> {
    pub fn id(&self) -> usize {
        self.index
    }

    pub fn edges(&self) -> impl Iterator<Item = Edge<'a>> {
        let start = self.graph.faces[self.index].edge;

        successors(Some(self.graph.edge(start)), move |e| {
            let next = self.graph.edges[e.index].next ^ 1;
            if next == start {
                None
            } else {
                Some(self.graph.edge(next))
            }
        })
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex<'a>> {
        self.edges().map(|edge| edge.vertices().0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex<'a> {
    graph: &'a Graph,
    index: usize,
}

impl Vertex<'_> {
    pub fn id(&self) -> usize {
        self.index
    }

    pub fn position(&self) -> Option<Point> {
        self.graph.vertices[self.index]
            .position
            .map(|i| self.graph.points[i])
    }
}

#[derive(Debug)]
pub struct Graph {
    points: Box<[Point]>,
    edges: Box<[GraphEdge]>,
    faces: Box<[GraphFace]>,
    vertices: Box<[GraphVertex]>,
}

impl Graph {
    pub fn edge_count(&self) -> usize {
        self.edges.len() / 2
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge(&self, index: usize) -> Edge {
        Edge { graph: self, index }
    }

    pub fn face(&self, index: usize) -> Face {
        Face { graph: self, index }
    }

    pub fn vertex(&self, index: usize) -> Vertex {
        Vertex { graph: self, index }
    }

    pub fn edges(&self) -> impl Iterator<Item = Edge> {
        (0..self.edge_count()).map(move |i| self.edge(i * 2))
    }

    pub fn faces(&self) -> impl Iterator<Item = Face> {
        (0..self.face_count()).map(move |i| self.face(i))
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex> {
        (0..self.vertex_count()).map(move |i| self.vertex(i))
    }
}

pub fn graph(
    points: Box<[Point]>,
    edges: Box<[GraphEdge]>,
    faces: Box<[GraphFace]>,
    vertices: Box<[GraphVertex]>,
) -> Graph {
    Graph {
        points,
        edges,
        faces,
        vertices,
    }
}
