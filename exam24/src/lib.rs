use std::fmt;
use chrono::{DateTime, Utc, TimeZone};

// Define the TreeNode struct
pub struct TreeNode {
    timestamp: u64,
    heart_rate: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(timestamp: u64, heart_rate: u32) -> Self {
        TreeNode {
            timestamp,
            heart_rate,
            left: None,
            right: None,
        }
    }
}

// Define the HeartRateTree struct
pub struct HeartRateTree {
    root: Option<Box<TreeNode>>,
}

impl HeartRateTree {
    pub fn new() -> Self {
        HeartRateTree { root: None }
    }

    pub fn insert(&mut self, timestamp: u64, value: u32) {
        let new_node = Box::new(TreeNode::new(timestamp, value));
        if let Some(ref mut root_node) = self.root {
            Self::insert_node(root_node, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_node(current: &mut Box<TreeNode>, new_node: Box<TreeNode>) {
        if new_node.timestamp < current.timestamp {
            if let Some(ref mut left) = current.left {
                Self::insert_node(left, new_node);
            } else {
                current.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = current.right {
                Self::insert_node(right, new_node);
            } else {
                current.right = Some(new_node);
            }
        }
    }

    pub fn average_last_minute(&self, current_time: u64) -> f32 {
        let start = current_time.saturating_sub(60);
        let (sum, count) = Self::sum_and_count(self.root.as_ref(), start, current_time);
        if count == 0 {
            0.0
        } else {
            sum as f32 / count as f32
        }
    }

    fn sum_and_count(node: Option<&Box<TreeNode>>, start: u64, end: u64) -> (u32, u32) {
        if let Some(n) = node {
            if n.timestamp < start {
                Self::sum_and_count(n.right.as_ref(), start, end)
            } else if n.timestamp > end {
                Self::sum_and_count(n.left.as_ref(), start, end)
            } else {
                let (left_sum, left_count) = Self::sum_and_count(n.left.as_ref(), start, end);
                let (right_sum, right_count) = Self::sum_and_count(n.right.as_ref(), start, end);
                (
                    left_sum + n.heart_rate + right_sum,
                    left_count + 1 + right_count,
                )
            }
        } else {
            (0, 0)
        }
    }

    // In-order traversal helper (public wrapper)
    pub fn inorder_traversal(&self, result: &mut Vec<(u64, u32)>) {
        self.inorder_traversal_helper(&self.root, result);
    }

    fn inorder_traversal_helper(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<(u64, u32)>) {
        if let Some(n) = node {
            self.inorder_traversal_helper(&n.left, result);
            result.push((n.timestamp, n.heart_rate));
            self.inorder_traversal_helper(&n.right, result);
        }
    }

    fn display_node(&self, node: &Option<Box<TreeNode>>, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(n) = node {
            self.display_node(&n.left, f)?;
            let dt: DateTime<Utc> = Utc.timestamp_opt(n.timestamp as i64, 0).unwrap();
            writeln!(f, "{} - {}", dt.to_rfc3339(), n.heart_rate)?;
            self.display_node(&n.right, f)?;
        }
        Ok(())
    }
}

// Display for TreeNode (optional)
impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dt: DateTime<Utc> = Utc.timestamp_opt(self.timestamp as i64, 0).unwrap();
        write!(f, "{} - {}", dt.to_rfc3339(), self.heart_rate)
    }
}

// Display for HeartRateTree
impl fmt::Display for HeartRateTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display_node(&self.root, f)
    }
}

