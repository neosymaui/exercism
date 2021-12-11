pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub val: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(val: &str) -> Self {
                    let n = Node {
                        val: val.to_string(),
                        attrs: HashMap::new(),
                    };
                    n
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut n = self.clone();
                    for attr in attrs.iter() {
                        n.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    n
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    let res: &str = self.attrs.get(key).unwrap();
                    Some(res)
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                srt: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(srt: &str, end: &str) -> Self {
                    let e = Edge {
                        srt: srt.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    };
                    e
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut e = self.clone();
                    for attr in attrs.iter() {
                        e.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    e
                }
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            let g = Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            };
            g
        }

        pub fn with_nodes(&self, nodes: &Vec<graph_items::node::Node>) -> Self {
            let mut g = self.clone();
            g.nodes = nodes.clone();
            g
        }

        pub fn with_edges(&self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            let mut g = self.clone();
            g.edges = edges.clone();
            g
        }

        pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
            let mut g = self.clone();
            for attr in attrs.iter() {
                g.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            g
        }

        pub fn get_node(&self, val: &str) -> Result<graph_items::node::Node, &str> {
            for node in self.nodes.iter() {
                if node.val == val {
                    return Ok(node.clone());
                }
            }
            panic!("node must be stored");
        }
    }
}
