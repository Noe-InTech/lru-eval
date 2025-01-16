use std::collections::HashMap;
use std::hash::Hash;

// Le trait Serializable est défini pour s'assurer que les types K et V peuvent être sérialisés.
pub trait Serializable {}

impl Serializable for char {}
impl Serializable for u32 {}

pub struct Cache<K, V> {
    capacity: usize, // Capacité maximale du cache
    map: HashMap<K, V>, // Stockage des données dans un HashMap
    keys: Vec<K>, // Liste des clés pour gérer l'ordre LRU
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Serializable, // K doit être hashable, égal et clonable
    V: Clone + Serializable, // V doit être clonable
{
    pub fn new(capacity: usize) -> Self {
        // On crée un nouveau cache avec la capacité spécifiée
        Self {
            capacity,
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn keys(&self) -> Vec<K> {
        // Retourne une copie des clés
        self.keys.clone()
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // On vérifie si la clé existe dans le cache
        if let Some(value) = self.map.get(key) {
            // Si elle existe, on déplace la clé à la fin de la liste pour marquer comme récemment utilisé
            self.keys.retain(|k| k != key); // Retirer la clé de sa position actuelle
            self.keys.push(key.clone()); // Ajouter la clé à la fin de la liste
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        // Si la clé existe déjà, on la met à jour et on la déplace à la fin
        if self.map.contains_key(&key) {
            self.keys.retain(|k| k != &key); // Retirer la clé existante de la liste
        } else if self.keys.len() >= self.capacity {
            // Si la capacité est atteinte, on évince la première clé
            if let Some(old_key) = self.keys.first().cloned() {
                self.map.remove(&old_key); // Retirer la clé la plus ancienne
                self.keys.remove(0); // Retirer la clé de la liste
            }
        }

        // Ajouter la nouvelle clé à la fin de la liste et dans le HashMap
        self.keys.push(key.clone());
        self.map.insert(key, value);
    }
}
