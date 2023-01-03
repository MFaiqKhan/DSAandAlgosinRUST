// use fnv to create a HashMap with a faster hash function because HashMap uses a slower hash function by default
// fnv is a crate that provides a fast hash function for small integers


use std::collections::HashMap; // we need to import the HashMap struct from the collections module

// here E= () means that E is a generic type, and we are setting the default type to () which is the unit type
// the unit type is a type with no data, it is used when we don't need to store any data in a variable
// the unit type is represented by (), and we can set it to i32 or any other type
// e.g 
// let x: () = (); // x is a variable of type () and it is set to ()
// let x: i32 = (); // x is a variable of type i32 and it is set to ()
// let x: i32 = 5; // x is a variable of type i32 and it is set to 5
// let x: () = 5; // x is a variable of type () and it is set to 5 // means that we can set a variable of type () to any type



pub struct Graph<VId, E = (), V= ()> {  // Graph is a struct with 3 generic types, VId, E and V
    vertices: HashMap<VId, V>, // vertices is a HashMap with keys of type VId and values of type V
    adjacency: HashMap<VId, Vec<(VId, E)>>, // adjacency is a HashMap with keys of type VId and values of type Vec<(VId, E)>
    // Vec<(VId, E)> is a vector of tuples, where the first element of the tuple is of type VId and the second element is of type E
}

// VId is the type of the vertex id, E is the type of the edge, and V is the type of the vertex
// what it can do is create a graph with vertices of type VId, edges of type E and vertex data of type V
// the graph is represented by a HashMap of vertices and a HashMap of edges
// the vertices HashMap has keys of type VId and values of type V
// the edges HashMap has keys of type VId and values of type Vec<(VId, E)>
// adjacency in graph means the edges that connect the vertices

impl <VId, E, V> Graph<VId, E, V> // impl here means implementation, we are implementing the Graph struct in the std::collections module
where  // where is used to specify the constraints on the generic types
    VId: Eq + Hash,  // why ? because we need to be able to use the VId as a key in the HashMap, Eq is used to compare the values and Hash is used to hash the values
    V: Hash,  // why ? because we need to be able to use the V as a key in the HashMap, Hash is used to hash the values
    {
        // initialize a new graph
        pub fn new() -> Graph<VId, E, V> {
            Graph { vertices: HashMap::new(), adjacency: HashMap::new() } // we create a new Graph struct with empty HashMaps for vertices and adjacency
        }

        pub fn push_vertex(self: &mut Graph<VId, E, V>, vid: VId, vertex: V) { // self is a mutable reference to the Graph struct, vid is the vertex id, and vertex is the vertex data
            self.vertices.insert(vid, vertex); // insert the vertex id and data into the vertices HashMap
        }

        pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) { // self is a mutable reference to the Graph struct, from is the vertex id of the source vertex, to is the vertex id of the destination vertex, and edge is the edge data
           let adjacent_to_from = self.adjacency.entry(from).or_default(); // we get the value of the key from in the adjacency HashMap, if the key doesn't exist, we insert a new key with an empty vector as the value
           adjacent_to_from.push((to, edge)); // we push the destination vertex id and edge data into the vector
        }

        pub fn push_undirected_edge(
            self: &mut Self, // self is a mutable reference to the Graph struct
            from: VId, // from is the vertex id of the source vertex
            to: VId,
            edge: E,
        ) { // self is a mutable reference to the Graph struct, from is the vertex id of the source vertex, to is the vertex id of the destination vertex, and edge is the edge data
            self.push_edge(from.clone(), to.clone(), edge.clone()); // we push the edge from the source vertex to the destination vertex, we use the clone method to clone the values of the parameters why ? 
            //because we need to use the values again in the next line and clone help us to do that by creating a copy of the values otherwise we would have moved the values and we wouldn't be able to use them again and get an error
            self.push_edge(to, from, edge); // we push the edge from the destination vertex to the source vertex
        }
    }


    // draw the graph
    /* 
    pub fn draw(self: &mut Self) {
        let mut graph = Graph::new();
        graph.push_vertex(1, "A");
        graph.push_vertex(2, "B");
        graph.push_vertex(3, "C");
        graph.push_vertex(4, "D");
        graph.push_vertex(5, "E");
        graph.push_vertex(6, "F");
        graph.push_vertex(7, "G");
        graph.push_vertex(8, "H");
        graph.push_vertex(9, "I");
        graph.push_vertex(10, "J");
        graph.push_vertex(11, "K");
        graph.push_vertex(12, "L");
        graph.push_vertex(13, "M");
        graph.push_vertex(14, "N");
        graph.push_vertex(15, "O");
        graph.push_vertex(16, "P");
        graph.push_vertex(17, "Q");
        graph.push_vertex(18, "R");
        graph.push_vertex(19, "S");
        graph.push_vertex(20, "T");
        graph.push_vertex(21, "U");
        graph.push_vertex(22, "V");
        graph.push_vertex(23, "W");
        graph.push_vertex(24, "X");
        graph.push_vertex(25, "Y");
        graph.push_vertex(26, "Z");
        graph.push_edge(1, 2, 1);
        graph.push_edge(1, 3, 1);
        graph.push_edge(1, 4, 1);
        graph.push_edge(1, 5, 1);
        graph.push_edge(1, 6, 1);
        graph.push_edge(1, 7, 1);
        graph.push_edge(1, 8, 1);
        graph.push_edge(1, 9, 1);
        graph.push_edge(1, 10, 1);
        graph.push_edge(1, 11, 1);
        graph.push_edge(1, 12, 1);
        graph.push_edge(1, 13, 1);
        graph.push_edge(1, 14, 1);
        graph.push_edge(1, 15, 1);
        graph.push_edge(1, 16, 1);
        graph.push_edge(1, 17, 1);
        graph.push_edge(1, 18, 1);

        graph.push_edge(2, 1, 1);
        graph.push_edge(2, 3, 1);
        graph.push_edge(2, 4, 1);
        graph.push_edge(2, 5, 1);
        graph.push_edge(2, 6, 1);
        graph.push_edge(2, 7, 1);
        graph.push_edge(2, 8, 1);

        now finish 

        graph.push_edge(1, 2, 1);
        graph.push_edge(1, 3, 1);
        graph.push_edge(1, 4, 1);
        graph.push_edge(1, 5, 1);

        graph.push_edge(2, 1, 1);
        graph.push_edge(2, 3, 1);
        graph.push_edge(2, 4, 1);


        graph.push_edge(3, 1, 1);
        graph.push_edge(3, 2, 1);

        graph.push_edge(4, 1, 1);
        graph.push_edge(4, 2, 1);

        graph.push_edge(5, 1, 1);
        graph.push_edge(5, 2, 1);

        graph.push_edge(6, 1, 1);

        graph.push_edge(7, 1, 1);

        graph.push_edge(8, 1, 1);

        graph.push_edge(9, 1, 1);

        graph.push_edge(10, 1, 1);

        graph.push_edge(11, 1, 1);


        graph.push_edge(12, 1, 1);


    */


// why we are comparing in VId ?
// hwo Eq will compare the values ?  by using the == operator
// example: 1 == 1 will return true, 1 == 2 will return false


// hwo self is a mutable reference to the Graph struct ?
// we can use the self keyword to access the fields of the struct
// example: self.vertices will access the vertices field of the Graph struct