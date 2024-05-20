mod create_node {
    use es_02::node::{Node, NodeFunction};

    #[test]
    fn generator_off() {
        let node: Node = "G gen1 - off".into();
        assert_eq!(
            node,
            Node::new(
                "gen1".to_string(),
                NodeFunction::Generator(false),
                None,
                [None, None]
            )
        )
    }

    #[test]
    fn generator_on() {
        let node: Node = "G gen1 - on".into();
        assert_eq!(
            node,
            Node::new(
                "gen1".to_string(),
                NodeFunction::Generator(true),
                None,
                [None, None]
            )
        )
    }

    #[test]
    fn switch_off() {
        let node: Node = "S sw01 - off".into();
        assert_eq!(
            node,
            Node::new(
                "sw01".to_string(),
                NodeFunction::Switch(false),
                None,
                [None, None]
            )
        )
    }

    #[test]
    fn switch_on() {
        let node: Node = "S sw01 gen1 on".into();
        assert_eq!(
            node,
            Node::new(
                "sw01".to_string(),
                NodeFunction::Switch(true),
                None,
                [None, None]
            )
        )
    }

    #[test]
    fn light() {
        let node: Node = "L l01 sw01".into();
        assert_eq!(
            node,
            Node::new("l01".to_string(), NodeFunction::Light, None, [None, None])
        )
    }
}

mod switch_node {
    use es_02::node::Node;

    #[test]
    fn generator_off_to_on() {
        let mut node: Node = "G g1 - off".into();
        assert!(node.switch().is_ok());
        assert_eq!(node.get_status(), Some(true));
    }

    #[test]
    fn generator_on_to_off() {
        let mut node: Node = "G g1 - on".into();
        assert!(node.switch().is_ok());
        assert_eq!(node.get_status(), Some(false));
    }

    #[test]
    fn switch_off_to_on() {
        let mut node: Node = "S s1 - off".into();
        assert!(node.switch().is_ok());
        assert_eq!(node.get_status(), Some(true));
    }

    #[test]
    fn switch_on_to_off() {
        let mut node: Node = "S s1 g1 on".into();
        assert!(node.switch().is_ok());
        assert_eq!(node.get_status(), Some(false));
    }

    #[test]
    fn light() {
        let mut node: Node = "L l1 g1".into();
        let prev_status = node.get_status();
        assert!(node.switch().is_err());
        assert_eq!(node.get_status(), prev_status);
    }
}

mod node_outs {
    use std::{cell::RefCell, rc::Rc};

    use es_02::node::Node;

    #[test]
    fn get_no_out() {
        let node: Node = "G g1 - off".into();
        assert!(node.get_out(0).is_none());
        assert!(node.get_out(1).is_none());
    }

    #[test]
    fn get_wrong_index() {
        let node: Node = "G g1 - off".into();
        assert!(node.get_out(2).is_none());
    }

    #[test]
    fn add_one() {
        let mut node: Node = "G g1 - off".into();
        let ch1: Node = "S s1 g1 off".into();
        node.add_out(Some(Rc::new(RefCell::new(ch1))));
        assert!(node.get_out(0).is_some());
    }

    #[test]
    fn add_two() {
        let mut node: Node = "G g1 - off".into();
        let ch1: Node = "S s1 g1 off".into();
        let ch2: Node = "S s2 g1 off".into();
        node.add_out(Some(Rc::new(RefCell::new(ch1))));
        node.add_out(Some(Rc::new(RefCell::new(ch2))));
        assert!(node.get_out(0).is_some());
        assert!(node.get_out(1).is_some());
    }
}

mod create_tree {
    use std::collections::{HashMap, HashSet};

    use es_02::{
        circuit_tree::CircuitTree,
        node::{Node, NodeFunction},
    };

    #[test]
    fn empty() {
        let tree: CircuitTree = "".into();
        assert_eq!(tree, CircuitTree::with_values(None, HashMap::new()))
    }

    #[test]
    fn generator_only() {
        let s = "G g1 - off";
        let tree: CircuitTree = s.into();
        let vec = vec!["g1".to_string()];
        let set: HashSet<String> = HashSet::from_iter(vec.iter().map(|s| s.clone()));
        assert_eq!(tree.get_node_names(), set)
    }

    #[test]
    fn generator_switch() {
        let s = "G g1 - off\nS s1 g1 off";
        let tree: CircuitTree = s.into();
        let vec: Vec<String> = vec!["g1".to_string(), "s1".to_string()];
        let set: HashSet<String> = HashSet::from_iter(vec.iter().map(|s| s.clone()));
        assert_eq!(tree.get_node_names(), set)
    }

    #[test]
    fn generator_light() {
        let s = "G g1 - off\nL l1 g1";
        let tree: CircuitTree = s.into();
        let vec: Vec<String> = vec!["g1".to_string(), "l1".to_string()];
        let set: HashSet<String> = HashSet::from_iter(vec.iter().map(|s| s.clone()));
        assert_eq!(tree.get_node_names(), set)
    }

    #[test]
    fn generator_switch_light() {
        let s = "G g1 - off\nS s1 g1 off\nL l1 s1";
        let tree: CircuitTree = s.into();
        let vec: Vec<String> = vec!["g1".to_string(), "s1".to_string(), "l1".to_string()];
        let set: HashSet<String> = HashSet::from_iter(vec.iter().map(|s| s.clone()));
        assert_eq!(tree.get_node_names(), set)
    }

    #[test]
    fn generator_switch_two_lights() {
        let s = "G g1 - off\nS s1 g1 off\nL l1 s1\nL l2 g1";
        let tree: CircuitTree = s.into();
        let vec: Vec<String> = vec![
            "g1".to_string(),
            "s1".to_string(),
            "l1".to_string(),
            "l2".to_string(),
        ];
        let set: HashSet<String> = HashSet::from_iter(vec.iter().map(|s| s.clone()));
        assert_eq!(tree.get_node_names(), set)
    }

    #[test]
    fn parent_connected() {
        let tree: CircuitTree = "G g1 - off\nL l1 g1".into();

        let light = tree.get("l1").unwrap();
        let light = light.as_ref().borrow();

        let light_parent = light.get_parent().unwrap();
        let light_parent = light_parent.as_ref().borrow();

        let fake_root = Node::new(
            "g1".to_string(),
            NodeFunction::Generator(false),
            None,
            [None, None],
        );

        assert_eq!(*light_parent, fake_root);
    }

    #[test]
    fn add_root() {
        let mut tree = CircuitTree::new();
        let node = Node::without_links("test".to_string(), NodeFunction::Generator(false));
        tree.add("-", node);
        assert!(tree.get("test").is_some());
    }

    #[test]
    fn add_non_root() {
        let mut tree: CircuitTree = "G g1 - off".into();
        let node = Node::without_links("test".to_string(), NodeFunction::Switch(false));
        tree.add("g1", node);
        assert!(tree.get("test").is_some());
    }
}

mod get {
    use std::{cell::RefCell, rc::Rc};

    use es_02::{
        circuit_tree::CircuitTree,
        node::{Node, NodeFunction},
    };

    #[test]
    fn node_exists() {
        let tree: CircuitTree = "G g1 - off\nL l1 g1".into();
        let res = tree.get("g1");
        let should_be = Node::without_links("g1".to_string(), NodeFunction::Generator(false));
        assert_eq!(res, Some(Rc::new(RefCell::new(should_be))));
    }

    #[test]
    fn node_not_exists() {
        let tree: CircuitTree = "G g1 - off\nL l1 g1".into();
        let res = tree.get("g2");
        assert_eq!(res, None);
    }
}

mod tree_light_status {
    use es_02::circuit_tree::CircuitTree;

    #[test]
    fn light_is_on() {
        let tree: CircuitTree = "G g1 - on\nS s1 g1 on\nL l1 s1".into();
        assert!(tree.light_status("l1").is_ok_and(|val| val));
    }

    #[test]
    fn light_is_off_due_to_switch() {
        let tree: CircuitTree = "G g1 - on\nS s1 g1 off\nL l1 s1".into();
        assert!(tree.light_status("l1").is_ok_and(|val| !val));
    }

    #[test]
    fn light_is_off_due_to_generator() {
        let tree: CircuitTree = "G g1 - off\nS s1 g1 on\nL l1 s1".into();
        assert!(tree.light_status("l1").is_ok_and(|val| !val));
    }

    #[test]
    fn not_a_light() {
        let tree: CircuitTree = "G g1 - on\nS s1 g1 on\nL l1 s1".into();
        assert!(tree.light_status("g1").is_err());
    }
}

mod turn_light_on {
    use es_02::circuit_tree::CircuitTree;

    #[test]
    fn only_generator() {
        let tree: CircuitTree = "G g1 - off\nL l1 g1".into();
        tree.turn_light_on("l1");
        assert!(tree.light_status("l1").is_ok_and(|val| val));
    }

    #[test]
    fn generator_and_switch() {
        let tree: CircuitTree = "G g1 - off\nS sw1 g1 off\nL l1 sw1".into();
        tree.turn_light_on("l1");
        assert!(tree.light_status("l1").is_ok_and(|val| val));
    }

    #[test]
    fn already_on() {
        let tree: CircuitTree = "G g1 - on\nS sw1 g1 on\nL l1 sw1".into();
        tree.turn_light_on("l1");
        assert!(tree.light_status("l1").is_ok_and(|val| val));
    }

    #[test]
    fn not_a_light() {
        let tree: CircuitTree = "G g1 - off\nS sw1 g1 on\nL l1 sw1".into();
        tree.turn_light_on("sw1");
        assert!(tree.light_status("l1").is_ok_and(|val| !val));
    }
}
