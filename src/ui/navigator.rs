use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
}

pub struct Navigator {
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
    pub forward_stack: Vec<PathBuf>,
    pub view_mode: ViewMode,
    pub show_hidden: bool,
}

pub type NavState = Rc<RefCell<Navigator>>;

impl Navigator {
    pub fn new(start_path: PathBuf) -> NavState {
        Rc::new(RefCell::new(Navigator {
            current_path: start_path,
            history: Vec::new(),
            forward_stack: Vec::new(),
            view_mode: ViewMode::List,
            show_hidden: false,
        }))
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        if path == self.current_path {
            return;
        }
        let old = self.current_path.clone();
        self.history.push(old);
        if self.history.len() > 50 {
            self.history.remove(0);
        }
        self.forward_stack.clear();
        self.current_path = path;
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            let current = self.current_path.clone();
            self.forward_stack.push(current);
            self.current_path = prev;
            true
        } else {
            false
        }
    }

    pub fn go_forward(&mut self) -> bool {
        if let Some(next) = self.forward_stack.pop() {
            let current = self.current_path.clone();
            self.history.push(current);
            self.current_path = next;
            true
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::List => ViewMode::Grid,
            ViewMode::Grid => ViewMode::List,
        };
    }

    pub fn current_dir_name(&self) -> String {
        self.current_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string())
    }
}
