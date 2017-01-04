//! 'Annotated' Graph
//! A graph structure with arbitrary data types associated with nodes and edges
//! Purposefully limited for now (no deletion, etc.)

type NodeId = usize;

struct Node<N> {
	val: N,
	neighbours: Vec<NodeId>
}

pub struct Edge<E> {
	val: E,
	end_ids: (NodeId, NodeId)
}

impl<E: Clone> Clone for Edge<E> {
	fn clone(&self) -> Self {
		Edge {
			val: self.val.clone(),
			.. *self
		}
	}
}

pub struct AnnGraph<N, E> {
	nodes: Vec<Node<N>>,
	edges: Vec<Edge<E>>
}

impl<N, E> AnnGraph<N, E> {
	pub fn new() -> AnnGraph<N, E> {
		AnnGraph {
			nodes: Vec::new(),
			edges: Vec::new()
		}
	}

	pub fn add_node<I: IntoIterator<Item=(NodeId, E)>>(&mut self, node: N, neighbours: I) -> NodeId {
		let id = self.nodes.len();
		let mut neighbour_ids = Vec::new();

		for (n,e) in neighbours {
			self.nodes[n].neighbours.push(id);
			self.edges.push(Edge { val: e, end_ids: (id, n) });
			neighbour_ids.push(n);
		}
		self.nodes.push(Node { val: node, neighbours: neighbour_ids });

		id
	}

	pub fn edges(&self) -> &[Edge<E>] {
		self.edges.as_slice()
	}
	
	pub fn nodes(&self) -> Vec<&N> {
		self.nodes.iter().map(|n| { &n.val }).collect()
	}

	pub fn ends(&self, e: &Edge<E>) -> (&N, &N) {
		(self.get_node(e.end_ids.0), self.get_node(e.end_ids.1))
	}

	pub fn get_node(&self, id: NodeId) -> &N {
		&self.nodes[id].val
	}

	pub fn neighbour_ids(&self, id: NodeId) -> Vec<NodeId> {
		self.nodes[id].neighbours.clone()
	}

	pub fn neighbours(&self, id: NodeId) -> Vec<&N> {
		self.neighbour_ids(id).into_iter().map(|_id| { self.get_node(_id) }).collect()
	}
}

impl<N, E: Clone> AnnGraph<N, E> {
	pub fn modify_nodes<F>(&self, f: F) -> AnnGraph<N, E>
			where F: Fn(NodeId, &N) -> N {
		let new_nodes = self.nodes.iter().enumerate().map(|(id, n)| {
			Node {
				val: f(id, &n.val),
				neighbours: n.neighbours.clone()
			}
		}).collect();
		AnnGraph {
			nodes: new_nodes,
			edges: self.edges.clone()
		}
	}
}
