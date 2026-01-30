use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemState {
    Pending,
    Processing,
    Completed,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub path: PathBuf,
    pub state: ItemState,
    pub selected: bool,
}

impl QueueItem {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: ItemState::Pending,
            selected: false,
        }
    }

    pub fn is_locked(&self) -> bool {
        matches!(self.state, ItemState::Processing)
    }

    pub fn filename(&self) -> String {
        self.path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[derive(Debug, Default)]
pub struct Queue {
    items: Vec<QueueItem>,
}

impl Queue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, path: PathBuf) {
        self.items.push(QueueItem::new(path));
    }

    pub fn add_multiple(&mut self, paths: Vec<PathBuf>) {
        for path in paths {
            self.add(path);
        }
    }

    pub fn remove_selected(&mut self) {
        self.items.retain(|item| !item.selected || item.is_locked());
    }

    pub fn clear_selection(&mut self) {
        for item in &mut self.items {
            item.selected = false;
        }
    }

    pub fn select(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            if !item.is_locked() {
                item.selected = true;
            }
        }
    }

    pub fn toggle_select(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            if !item.is_locked() {
                item.selected = !item.selected;
            }
        }
    }

    pub fn select_range(&mut self, start: usize, end: usize) {
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };

        for i in start..=end {
            if let Some(item) = self.items.get_mut(i) {
                if !item.is_locked() {
                    item.selected = true;
                }
            }
        }
    }

    pub fn move_selected(&mut self, target_index: usize) {
        // Extract selected items (excluding locked ones)
        let mut selected = Vec::new();
        let mut i = 0;
        while i < self.items.len() {
            if self.items[i].selected && !self.items[i].is_locked() {
                selected.push(self.items.remove(i));
            } else {
                i += 1;
            }
        }

        // Insert at target position
        let insert_pos = target_index.min(self.items.len());
        for (offset, item) in selected.into_iter().enumerate() {
            self.items.insert(insert_pos + offset, item);
        }
    }

    pub fn remove_completed(&mut self) {
        self.items
            .retain(|item| !matches!(item.state, ItemState::Completed));
    }

    pub fn get_next_pending(&self) -> Option<usize> {
        self.items
            .iter()
            .position(|item| matches!(item.state, ItemState::Pending))
    }

    pub fn set_state(&mut self, index: usize, state: ItemState) {
        if let Some(item) = self.items.get_mut(index) {
            item.state = state;
        }
    }

    pub fn items(&self) -> &[QueueItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [QueueItem] {
        &mut self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
