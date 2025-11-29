use std::collections::HashMap;

use uuid::Uuid;

pub struct Attribute<T> {
    pub id: u8,
    pub value: T,
    pub observers: HashMap<u8, Box<dyn Observer<T>>>,
    pub modifiers: HashMap<u8, Box<dyn Modifier<T>>>,
}

impl<T> Attribute<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: 1,
            value,
            observers: HashMap::new(),
            modifiers: HashMap::new(),
        }
    }
}

impl<T> Entity for Attribute<T> {
    fn get_id(&self) -> &u8 {
        &self.id
    }
}

impl<T> Subject<T> for Attribute<T> {
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers
            .insert(observer.get_id().to_owned(), observer);
    }

    fn remove_observer(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers.remove(&observer.get_id());
    }

    fn notify_observers(&self) {
        for observer in self.observers.values() {
            observer.on_notify(&self.get_value());
        }
    }

    fn get_value(&self) -> &T {
        let mut modifiers: Vec<_> = self.modifiers.values().collect();
        modifiers.sort_by(|a, b| a.get_priority().cmp(&b.get_priority()));
        let mut final_value: &T = &self.value;
        for modifier in modifiers.iter() {
            final_value = modifier.calculate(final_value);
        }
        final_value
    }
}

pub trait Entity {
    fn get_id(&self) -> &u8;
}

pub trait Observer<T>: Entity {
    fn on_notify(&self, value: &T);
}

pub trait Subject<T>: Entity {
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>);
    fn remove_observer(&mut self, observer: Box<dyn Observer<T>>);
    fn notify_observers(&self);
    fn get_value(&self) -> &T;
}

pub trait Modifier<T> {
    fn get_priority(&self) -> u8;
    fn calculate(&self, value: &T) -> &T;
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
