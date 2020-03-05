use petgraph::{
    *,
    graph::*,
    graphmap::*,
    dot::*,
    visit::*,
};

use std::collections::{HashSet, HashMap};
use std::fmt::{self, Debug, Display, Formatter};
use petgraph::visit::{EdgeRef as PetgraphEdgeRef};
use std::hash::{self, Hash, Hasher};
use crate::graph::*;


pub type EdgeRef<'a> = EdgeReference<'a, TextGraphEdgeWeight>;

#[derive(Clone, Copy, PartialEq)]
pub struct GraphEdge<'a>  {
    edge: EdgeRef<'a>,
}

impl<'a> GraphEdge<'a>  {
    pub fn contains_weight(&self, w: &usize) -> bool {
        self.weight().contains(w)
    }
    pub fn max_weight(&'a self) -> Option<&'a usize> {
        self.weight().iter().max()
    }
}
impl<'a> Debug for GraphEdge<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl<'a> Display for GraphEdge<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?} --{:?}--> {:?}",
               self.edge.source(),
               self.edge.weight(),
               self.edge.target())
    }
}
impl<'a> Hash for GraphEdge<'a> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id().hash(h);
    }
}
impl<'a> std::ops::Deref for GraphEdge<'a> {
    type Target = EdgeRef<'a>;
    fn deref(&self) -> &Self::Target {
        &self.edge
    }
}
impl<'a> From<EdgeRef<'a>> for GraphEdge<'a>  {
    fn from(edge: EdgeRef<'a>) -> Self {
        Self {
            edge
        }
    }
}
impl<'a> Eq for GraphEdge<'a> {
}
impl<'a> petgraph::visit::EdgeRef for GraphEdge<'a> {
    type NodeId = <EdgeRef<'a> as petgraph::visit::EdgeRef>::NodeId;
    type EdgeId = <EdgeRef<'a> as petgraph::visit::EdgeRef>::EdgeId;
    type Weight = <EdgeRef<'a> as petgraph::visit::EdgeRef>::Weight;
    fn source(&self) -> Self::NodeId {
        self.edge.source()
    }
    fn target(&self) -> Self::NodeId {
        self.edge.target()
    }
    fn weight(&self) -> &Self::Weight {
        self.edge.weight()
    }
    fn id(&self) -> Self::EdgeId {
        self.edge.id()
    }
}
