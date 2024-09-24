use bmbp_util::{BmbpTree, BmbpTreeUtil};

#[test]
fn test_tree() {
    #[derive(Clone)]
    struct Dmeo {
        code: Option<String>,
        parent_id: Option<String>,
        name: Option<String>,
        order: usize,
        children: Option<Vec<Dmeo>>,
    }
    impl BmbpTree<Dmeo> for Dmeo {
        fn get_code(&self) -> &Option<String> {
            &self.code
        }
        fn set_code(&mut self, code: Option<String>) -> &mut Self {
            self.code = code;
            self
        }
        fn get_parent_code(&self) -> &Option<String> {
            &self.parent_id
        }
        fn set_parent_code(&mut self, parent_code: Option<String>) -> &mut Self {
            self.parent_id = parent_code;
            self
        }
        fn get_children(&self) -> &Option<Vec<Dmeo>> {
            &self.children
        }
        fn get_children_mut(&mut self) -> &mut Option<Vec<Dmeo>> {
            &mut self.children
        }
        fn set_children(&mut self, children: Option<Vec<Dmeo>>) -> &mut Self {
            self.children = children;
            self
        }

        fn get_order(&self) -> usize {
            0usize
        }
    }

    let root = Dmeo {
        code: Some("1".to_string()),
        parent_id: Some("#".to_string()),
        name: Some("1".to_string()),
        order: 2usize,
        children: None,
    };
    let demo_tree = vec![root];
    let tree = BmbpTreeUtil::build_tree::<Dmeo>(demo_tree);
    assert_eq!(tree.len(), 1)
}
