#[derive(Debug, Clone, Default)]
pub struct VecMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}
impl<K: Ord, V> VecMap<K, V> {
    pub fn new() -> Self
    where
        K: PartialEq,
    {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self
    where
        K: PartialEq,
    {
        VecMap {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.keys.capacity().min(self.values.capacity())
    }

    #[inline]
    // fn position<Q: PartialEq<K>>(&self, key: &Q) -> Option<usize> {
    //     self.keys.iter().position(|k| key == k)
    // }
    fn position(&self, key: &K) -> Option<usize> {
        let mut i = 0;
        let res: Option<usize> = None;
        while i < self.len() {
            if self.keys[i] == *key { return Some(i) }
            i +=1;
        }
        return None;
    }
}

#[derive(Debug, Clone)]
pub struct Node<T, V> {
    pub children: VecMap<T, u32>, // array to hold transitions for all alphanumeric characters
    val: Option<V>, // value
}

impl <T: std::cmp::Ord, V> Node<T, V> {
    pub fn new() -> Node<T, V> {
        Node {
            children: VecMap::new(),
            val: None
        }
    }
}
pub struct Trie<T, V> {
    pub nodes: Vec<Node<T, V>>
}

impl <T: std::cmp::Ord + std::clone::Clone, V: std::clone::Clone> Trie<T, V> {
    pub fn new() -> Trie<T, V> {
        Trie {nodes: [Node::new()].to_vec()}
    }
    #[inline]
    fn transition(&self, node_pos: usize, step_to: &T) -> Option<usize> {
        self.nodes[node_pos].children.position(step_to).map(|p| self.nodes[node_pos].children.values[p] as usize)
    }

    pub fn insert(&mut self, input: Vec<T>, val: Option<V>) {
        let mut node_pos: usize = 0;
		let mut temp_node_pos: usize;
        for ele in input {
            match self.transition(node_pos, &ele) {
                None => {          
                    temp_node_pos = self.nodes.len();         
                    self.nodes.push(Node::new());
                    self.nodes[node_pos].children.keys.push(ele);
                    self.nodes[node_pos].children.values.push(temp_node_pos as u32);
                    node_pos = temp_node_pos;
                }
                Some(pos) => {
                    node_pos = pos;
                }
            }
        }
        self.nodes[node_pos].val = val;
    }
    pub fn get(&self, input: &Vec<T>) -> Option<&V> {
        let mut node_pos: usize = 0;
        for ele in input {
            match self.transition(node_pos, ele) {
                None => {return None},
                Some(index) => {
                    node_pos = index;
                }
            }
        }
        return self.nodes[node_pos].val.as_ref();
    }
    pub fn longest_common_prefix(&self, input: &Vec<T>) -> Option<(usize, &V)> {
        let mut node_pos: usize = 0;
        let mut value: Option<(usize, &V)> = None;
        for (i, ele) in input.iter().enumerate() {
            match self.transition(node_pos, ele) {
                None => {
                    return value;
                },
                Some(index) => {
                    node_pos = index;
                    value = self.nodes[node_pos].val.as_ref().map(|s| (i, s));
                }
            }
        }
        return value;
    }
}

