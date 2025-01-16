pub mod cache; 
use crate::cache::Cache; 

pub struct LruCache {
    cache: Cache<char, u32>, // Cache qui utilise des caractères comme clés et des entiers comme valeurs
}

impl LruCache {
    pub fn new(cache_capacity: usize) -> Self {
        // On crée une nouvelle instance de LruCache avec une capacité donnée
        Self {
            cache: Cache::new(cache_capacity),
        }
    }

    pub fn access(&mut self, letter: char, position: u32) -> Option<u32> {
        // Si la lettre existe déjà dans le cache, on la met à jour
        if let Some(&current_position) = self.cache.get(&letter) {
            // On met à jour la position de la lettre dans le cache
            self.cache.put(letter, position);
            Some(current_position) // On retourne l'ancienne position
        } else {
            // Sinon, on l'ajoute au cache
            self.cache.put(letter, position);
            None // On retourne None car la lettre n'était pas dans le cache avant
        }
    }



}
