use std::{collections::HashSet, hash::Hash};

///A Pulse is used to track dependency ids and tell this Pulse owner which ones use this Pulse.
#[derive(Debug)]
pub struct Pulse<T, K: Clone + Eq + Hash> {
    data: T,
    sender: flume::Sender<K>,
    dependents: HashSet<K>,
}
impl<T, K: Clone + Eq + Hash> Pulse<T, K> {
    pub fn new(data: T, sender: flume::Sender<K>) -> Self {
        Self {
            sender,
            data,
            dependents: HashSet::new(),
        }
    }

    ///Adds the given dependency on this pulse, so when it modifies, the dependency with the given `dep` value is sent.
    ///The `dep` is intended to be the ID of the thing that depends on this Pulse
    pub fn add_dependency(&mut self, dep: K) {
        self.dependents.insert(dep);
    }

    ///Checks weather the given `dep` is listed by this Pulse
    pub fn has_dependency(&self, dep: &K) -> bool {
        self.dependents.contains(dep)
    }

    ///Tries to remove the given `dep` dependency, returns weather it was successfully removed or not.
    ///If this returns false, it means this pulse hadn't it
    pub fn delete_dependency(&mut self, dep: &K) -> bool {
        self.dependents.remove(dep)
    }

    ///Reads the current value of this pulse
    pub fn get(&self) -> &T {
        &self.data
    }

    ///Writes the given `data` and returns the old value.
    ///After so, emits to the owner all the dependencies
    pub fn write(&mut self, data: T) -> T {
        let out = std::mem::replace(&mut self.data, data);
        self.emit();
        out
    }

    ///Writes this Pulse data using the given function, then, emits the dependencies for the owner
    pub fn write_with<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.data);
        self.emit();
    }

    ///Emits to the owner all the dependencies that require this Pulse and returns the amount sent
    pub fn emit(&self) -> usize {
        for dep in self.dependents.iter() {
            self.sender.send(dep.clone()).unwrap();
        }
        self.dependents.len()
    }
}

///An Observed Pulse is as a normal Pulse but it follows the observer pattern, so the dependencies are functions and there is no specific 'owner'
pub struct ObservedPulse<T> {
    dependents: Vec<Box<dyn Fn(&T)>>,
    data: T,
}

impl<T> ObservedPulse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            dependents: Vec::new(),
        }
    }

    ///Adds the given dependency on this pulse, so when it modifies, the dependency with the given `dep` value is sent.
    ///The `dep` is intended to be the ID of the thing that depends on this Pulse
    pub fn add_dependency<F: Fn(&T) + 'static>(&mut self, dep: F) {
        self.dependents.push(Box::new(dep));
    }

    ///Tries to remove the dependency at the given `index`, note that this is O(n), might not cause much impact on
    ///pulses with low amount of functions, but if the amount increases and the order doesnt matter, use `delete_dependency_unordered` instead
    pub fn delete_dependency(&mut self, index: usize) -> Box<dyn FnOnce(&T)> {
        self.dependents.remove(index)
    }
    ///Removes the dependency at the given index by swaping it to the end and poping, so this is O(1), but the indices won't match 100% with the order they were
    ///inserted. For keeping them, use `delete_dependency` instead
    #[inline]
    pub fn delete_dependency_unordered(&mut self, index: usize) -> Box<dyn FnOnce(&T)> {
        self.dependents.swap_remove(index)
    }

    #[inline]
    ///Reads the current value of this pulse
    pub fn get(&self) -> &T {
        &self.data
    }

    ///Writes the given `data` and returns the old value.
    ///After so, emits to the owner all the dependencies
    pub fn write(&mut self, data: T) -> T {
        let out = std::mem::replace(&mut self.data, data);
        self.emit();
        out
    }

    ///Writes this Pulse data using the given function, then, emits the dependencies for the owner
    pub fn write_with<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.data);
        self.emit();
    }

    ///Emits to the owner all the dependencies that require this Pulse and returns the amount sent
    pub fn emit(&self) -> usize {
        for dep in self.dependents.iter() {
            dep(&self.data);
        }
        self.dependents.len()
    }
}
