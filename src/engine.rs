use std::collections::HashMap;

pub struct Attribute<T>
where
    T: Clone,
{
    pub id: u8,
    pub value: T,
    pub observers: HashMap<u8, fn(value: &T)>,
    pub modifiers: HashMap<u8, Modifier<T>>,
}

impl<T> Attribute<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self {
            id: 1,
            value,
            observers: HashMap::new(),
            modifiers: HashMap::new(),
        }
    }
}

impl<T> Entity for Attribute<T>
where
    T: Clone,
{
    fn get_id(&self) -> &u8 {
        &self.id
    }
}

impl<T> Subject<T> for Attribute<T>
where
    T: Clone,
{
    fn register_observer(&mut self, id: u8, observer: fn(value: &T)) {
        self.observers.insert(id.to_owned(), observer);
    }

    fn register_modifier(&mut self, id: u8, priority: u8, modifier: fn(value: &T) -> T) {
        self.modifiers.insert(
            id.to_owned(),
            Modifier {
                priority,
                calculate: modifier,
            },
        );
    }

    fn remove_observer(&mut self, id: u8) {
        self.observers.remove(&id);
    }

    fn remove_modifier(&mut self, id: u8) {
        self.modifiers.remove(&id);
    }

    fn notify_observers(&self) {
        for observer in self.observers.values() {
            observer(&self.get_value());
        }
    }

    fn get_value(&self) -> T {
        let mut modifiers: Vec<_> = self.modifiers.values().collect();
        modifiers.sort_by(|a, b| a.priority.cmp(&b.priority));
        let mut final_value: T = self.value.clone();
        for modifier in modifiers.iter() {
            final_value = (modifier.calculate)(&final_value);
        }
        final_value
    }
}

pub trait Entity {
    fn get_id(&self) -> &u8;
}

pub trait Observer<T>: Entity
where
    T: Clone,
{
    fn on_notify(&self, value: &T);
}

pub trait Subject<T>: Entity
where
    T: Clone,
{
    fn register_observer(&mut self, id: u8, observer: fn(value: &T));
    fn register_modifier(&mut self, id: u8, priority: u8, modifier: fn(value: &T) -> T);
    fn remove_observer(&mut self, id: u8);
    fn remove_modifier(&mut self, id: u8);
    fn notify_observers(&self);
    fn get_value(&self) -> T;
}

pub struct Modifier<T> {
    pub priority: u8,
    pub calculate: fn(value: &T) -> T,
}

pub mod util {
    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::engine::{Attribute, Subject};

    pub fn convert<K, T>(origin: &HashMap<K, Attribute<T>>) -> HashMap<K, T>
    where
        K: Clone + Eq + Hash,
        T: Clone,
    {
        origin
            .into_iter()
            .map(|(key, value)| (key.clone(), value.get_value().clone()))
            .collect()
    }

    pub fn restore<K, T>(origin: &HashMap<K, T>) -> HashMap<K, Attribute<T>>
    where
        K: Clone + Eq + Hash,
        T: Clone,
    {
        origin
            .into_iter()
            .map(|(key, value)| (key.clone(), Attribute::new(value.clone())))
            .collect()
    }
}
