use std::{hint::black_box, time::Instant};

use rand::Rng;


#[derive(Debug, Clone)]
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub height: usize,
    pub parent: Option<usize>,
}


#[derive(Debug, Clone)]
pub struct Tree<K: Ord, V> {
    pub nodes: Vec<Node<K, V>>,
    pub root: usize
}


impl<K: Ord + Clone + std::fmt::Debug, V: Clone + std::fmt::Debug> Tree<K,V> {
    pub fn insert(&mut self, key: K, value: V) -> bool {
        let mut cur_node = self.root;

        let insert_left;
        let insert_on;
        let mut prev_nodes = Vec::new();
        loop {
            let node = self.nodes.get(cur_node).unwrap();
            prev_nodes.push(cur_node);
            match node.key.cmp(&key) {
                std::cmp::Ordering::Greater => {
                    if let Some(left) = node.left {
                        cur_node = left;
                    } else {
                        insert_left = true;
                        insert_on = cur_node;
                        break;
                    }

                },
                std::cmp::Ordering::Equal => return false,
                std::cmp::Ordering::Less => {
                    if let Some(right) = node.right {
                        cur_node = right;
                    } else {
                        insert_left = false;
                        insert_on = cur_node;
                        break;
                    }
                }
            }
        }
        let len = self.nodes.len();
        let n = self.nodes.get_mut(insert_on).unwrap();
        if insert_left {
            n.left = Some(len);
        } else {
            n.right = Some(len);
        }
        self.nodes.push(Node {
            key: key.clone(),
            value: value.clone(),
            left: None,
            right: None,
            height: 1,
            parent: Some(insert_on)
        });
        for node in prev_nodes.iter().rev() {
            self.update_height(*node);
            self.rebalance(*node);
        }
        true
    }
    fn left_height(&self, node: usize) -> Option<usize> {
        let node = self.nodes.get(node)?;
        if let Some(left) = node.left {
            Some(self.nodes.get(left).unwrap().height)
        }else{
            Some(0)
        }
    }
    fn right_height(&self, node: usize) -> Option<usize> {
        let node = self.nodes.get(node)?;
        if let Some(right) = node.right {
            Some(self.nodes.get(right).unwrap().height)
        }else{
            Some(0)
        }

    }
    fn update_height(&mut self, node: usize) {
        let left_height = self.left_height(node).unwrap();
        let right_height = self.right_height(node).unwrap();
        let node_ref = self.nodes.get_mut(node).unwrap();
        node_ref.height = 1 + usize::max(left_height, right_height);
    }
    pub fn balance_factor(&self, node: usize) -> Option<i8> {
        let left_height = self.left_height(node).unwrap_or(0);
        let right_height = self.right_height(node).unwrap_or(0);
        if left_height >= right_height {
            Some((left_height - right_height) as i8)
        } else {
            Some(-((right_height - left_height) as i8))
        }
    }

    fn rotate_right(&mut self, rotation_root: usize) {
        let root_node = self.nodes.get(rotation_root).unwrap();
        
        if root_node.left.is_none() {
            return;
        }

        let left_node_index = root_node.left.unwrap(); // b
        let left_node = self.nodes.get(left_node_index).unwrap();
        let left_right_node = left_node.right; 
        let root_node_parent = root_node.parent;

        if let Some(lrn) = left_right_node {
            let left_right_node_mut = self.nodes.get_mut(lrn).unwrap();
            left_right_node_mut.parent = Some(rotation_root);
        }

        let left_node_mut = self.nodes.get_mut(left_node_index).unwrap();
        left_node_mut.right = Some(rotation_root);
        left_node_mut.parent = root_node_parent;
        

        let root_node_mut = self.nodes.get_mut(rotation_root).unwrap();
        root_node_mut.left = left_right_node;
        root_node_mut.parent = Some(left_node_index);



        
        
        
        if rotation_root == self.root {
            self.root = left_node_index;
        } else {
            let parent_mut = self.nodes.get_mut(root_node_parent.unwrap()).unwrap();
            if parent_mut.left == Some(rotation_root) {
                parent_mut.left = Some(left_node_index);
            } else if parent_mut.right == Some(rotation_root) {
                parent_mut.right = Some(left_node_index);
            }
        }

        self.update_height(rotation_root);
        self.update_height(left_node_index);


    }

    fn rotate_left(&mut self, rotation_root: usize) {
        let root_node = self.nodes.get(rotation_root).unwrap();
        if root_node.right.is_none() {
            return;
        }

        let right_node_index = root_node.right.unwrap();
        
        let right_node = self.nodes.get(right_node_index).unwrap();
        let right_left_node = right_node.left;
        let root_node_parent: Option<usize> = root_node.parent;
        if let Some(rln) = right_left_node {
            let right_left_node_mut = self.nodes.get_mut(rln).unwrap();
            right_left_node_mut.parent = Some(rotation_root);
        }

        let right_node_mut = self.nodes.get_mut(right_node_index).unwrap();
        right_node_mut.left = Some(rotation_root);
        right_node_mut.parent = root_node_parent;

        let root_node_mut = self.nodes.get_mut(rotation_root).unwrap();
        root_node_mut.right = right_left_node;
        root_node_mut.parent = Some(right_node_index);

        
        

        if rotation_root == self.root {
            self.root = right_node_index;
        } else {
            let parent_mut = self.nodes.get_mut(root_node_parent.unwrap()).unwrap();
            if parent_mut.left == Some(rotation_root) {
                parent_mut.left = Some(right_node_index);
            } else if parent_mut.right == Some(rotation_root) {
                parent_mut.right = Some(right_node_index);
            }
        }
        self.update_height(rotation_root);
        self.update_height(right_node_index);

        

    }

    fn rebalance(&mut self, node: usize) {
        match self.balance_factor(node).unwrap() {
            -2 => {
                let right_node = self.nodes.get(node).unwrap().right.unwrap();
                if self.balance_factor(right_node).unwrap() == -1 {
                    self.rotate_right(right_node);
                }
                self.rotate_left(node);
                
            },
            2 => {
                                let left_node = self.nodes.get(node).unwrap().left.unwrap();
                if self.balance_factor(left_node).unwrap() == 1 {
                    self.rotate_left(left_node);
                }
                self.rotate_right(node);
            },
            _ => {}
        }
    } 

    
}

pub struct TreeIter<'a, K: 'a + Ord + Clone, V: 'a + Clone> {
    prev_nodes: Vec<usize>,
    current_node: usize,
    cur_emitted: bool,
    going_left: bool,
    tree: &'a Tree<K, V>
}


impl<'a, K: 'a + Ord + Clone, V: 'a + Clone> Iterator for TreeIter<'a, K, V> {
    type Item = &'a Node<K,V>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cn = self.tree.nodes.get(self.current_node);
            if let Some(node) = cn {
                if node.left.is_some() && self.going_left {
                    self.prev_nodes.push(self.current_node);
                    self.cur_emitted = false;
                    self.current_node = node.left.unwrap();
                    continue;
                } else if !(self.cur_emitted) {
                    self.cur_emitted = true;
                    return Some(node);
                } else if let Some(right) = node.right {
                    self.current_node = right;
                    self.going_left = true;
                    self.cur_emitted = false;
                    continue;
                }else {
                     self.cur_emitted = false;
                     let nn = self.prev_nodes.pop();
                     if let Some(nnn) = nn {
                         self.current_node = nnn;
                         self.going_left = false;
                     }else {
                         return None;
                     }
                }
            }
        }
    }
}

impl<'a, K: 'a + Ord + Clone, V: 'a + Clone> Tree<K, V> {
    fn iter_nodes(&'a self) -> TreeIter<'a, K, V> {
        TreeIter {
            prev_nodes: vec![],
            current_node: self.root,
            tree: self,
            cur_emitted: false,
            going_left: true
        } 
    }
    fn iter(&'a self) -> impl Iterator<Item = (&'a K, &'a V) > + 'a {
        self.iter_nodes().map(|node| (&node.key, &node.value)) 
    }
}

#[derive(Clone, Debug)]
pub struct NodeForRendering<K: std::fmt::Debug,V: std::fmt::Debug> {
    pub key: K,
    pub value: V,
    pub name: String,
    pub children: Vec<NodeForRendering<K, V>>,
}
impl<K: std::fmt::Debug, V: std::fmt::Debug> NodeForRendering<K,V> {
    pub fn new(key: K, value: V, children: Vec<NodeForRendering<K, V>>) -> NodeForRendering<K,V> {
        NodeForRendering {
            name: format!("{:?}: {:?}", key, value),
            key,
            value,
            children
        }
    } 
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> render_as_tree::Node for NodeForRendering<K, V>{
    type Iter<'a> = std::slice::Iter<'a, Self>
        where 
        V: 'a,
        K: 'a;
    
    fn name(&self) -> &str {
        &self.name
    }
    fn children(&self) -> Self::Iter<'_> {
        self.children.iter()
    }

}




pub struct Marker {

    directions: Vec<usize>, // 0 left, 1 right
    node_corresponding: usize
}

pub fn basic_tree() -> Tree<i64, i8> {
    return Tree {
        nodes: vec![
            Node {
                key: 4,
                value: 2,
                left: None, 
                right: None,
                height: 0,
                parent: None,
            },
        ],
        root: 0,
    };
}

pub fn time_test(starting_size: usize, insertions: usize, tree: &mut Tree<i64, i8>) -> (f64, f64){
    let mut randoms = vec![];
    let mut rng = rand::rng();
    if starting_size > tree.nodes.len() {
        for _ in 0..starting_size - tree.nodes.len() {
            randoms.push((rng.random_range(-1000000000..10000000000), rng.random_range(-10..10)));
        }
        for r in randoms {
            tree.insert(r.0, r.1);
        }
    }
    let mut randoms = vec![];
    let mut nanos_insertion = 0;
    let mut nanos_minimumfinding = 0;
    for _ in 0..insertions {
        randoms.push((rng.random_range(-1000000000..10000000000), rng.random_range(-10..10)));
    }
    
    for r in randoms {
        let time = Instant::now();
        tree.insert(r.0, r.1);
        nanos_insertion += time.elapsed().as_nanos();


        let time = Instant::now();
        let _min = tree.iter().next();
        black_box(_min);
        nanos_minimumfinding += time.elapsed().as_nanos();
    }
    println!("\n------- TIME TAKEN FOR {} INSERTIONS (WITH A TREE SIZE {}) ------", insertions, starting_size);
    println!("{:?} micros", nanos_insertion as f64/1000.0);

    println!("\n------- TIME TAKEN FOR {} MIN FINDINGS (WITH A TREE SIZE {}) ------", insertions, starting_size);
    println!("{:?} micros", nanos_minimumfinding as f64/1000.0);

    return (nanos_insertion as f64/1000.0, nanos_minimumfinding as f64/1000.0);

}

pub fn display_tree<K: std::fmt::Debug + Ord + Clone, V: std::fmt::Debug + Clone>(tree: Tree<K, V>) {
    let root = tree.nodes.get(tree.root).unwrap();
    let mut root_node = NodeForRendering::new(root.key.clone(), root.value.clone(), vec![]);
    let mut stack = vec![Marker {
        directions: vec![],
        node_corresponding: tree.root
    }];

    while let Some(cur_task) = stack.pop() {
        let mut rnode = &mut root_node;
        for direction in cur_task.directions.iter() {
            rnode = rnode.children.get_mut(*direction).unwrap();
        }
        let corresponding_node = tree.nodes.get(cur_task.node_corresponding).unwrap();
        if let Some(left) = corresponding_node.left {
            let leftnode = tree.nodes.get(left).unwrap();
            let new_node = NodeForRendering::new(leftnode.key.clone(), leftnode.value.clone(), vec![]);
            rnode.children.push(new_node);
            let mut new_directions = cur_task.directions.clone();
            new_directions.extend(vec![
                rnode.children.len() - 1
            ]);
            stack.push(Marker {
                node_corresponding: left,
                directions: new_directions
            });
        }
        if let Some(right) = corresponding_node.right {
            let rightnode = tree.nodes.get(right).unwrap();
            let new_node = NodeForRendering::new(rightnode.key.clone(), rightnode.value.clone(), vec![]);
            rnode.children.push(new_node);
            let mut new_directions = cur_task.directions.clone();
            new_directions.extend(vec![
                rnode.children.len() - 1
            ]);
            stack.push(Marker {
                node_corresponding: right,
                directions: new_directions
            });
        }
    }
    for line in render_as_tree::render(&root_node).iter() {
        println!("{}", line);
    }

}


fn main() {
    let mut tree = basic_tree();
    let sizes = [
        0,
        1000,
        2000,
        4000,
        8000,
        16000,
        32000,
        64000,
        128000,
        256000,
        512000,
        1024000,
        2048000,
        4096000,
        8192000,
        16384000
    ];

    let mut times = vec![];

    for size in sizes.iter() {
        times.push(time_test(*size, 10, &mut tree));
    }

    let (mut insertions, mut mins): (Vec<f64>, Vec<f64>) = (vec![], vec![]);

    for time in times.iter() {
        insertions.push(time.0);
        mins.push(time.1);
    }

    println!("{:?} {:?}", insertions, mins);





    let mut rng = rand::rng();
    let mut randoms = vec![];
    for _ in 0..30000 {
        randoms.push((rng.random_range(-1000000000..10000000000), rng.random_range(-10..10)));
    }
    

    
    let mut tree: Tree<i64, i64> = Tree {
        root: 0,
        nodes: vec![
            Node {
                key: randoms[0].0,
                value: randoms[0].1,
                left: None,
                right: None,
                height: 0,
                parent: None
            }
        ]
    };

    
    let time = Instant::now();
    for (i, r) in randoms.iter().enumerate() {
        if i == 0 {continue};
        tree.insert(r.0, r.1);
    }
    let sorted: Vec<i64> = tree.iter().map(|x| x.0.clone()).collect();
    let time_taken_by_btree = time.elapsed().as_millis();

    let time = Instant::now();
    let mut sorted_2: Vec<i64> = randoms.clone().iter().map(|x| x.0).collect();
    sorted_2.sort();
    let time_taken_by_normal = time.elapsed().as_nanos() as f64/1000_000.0;

    assert_eq!(sorted, sorted_2);

    println!("------ AVL TREE TIME TAKEN ------");
    println!("{} ms", time_taken_by_btree);
    println!("------ RUST SORT TIME TAKEN ------");
    println!("{} ms", time_taken_by_normal);



}
