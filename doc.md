# Documentation du projet LRU Cache en Rust

## Introduction
Ce projet implémente un cache LRU en Rust, permettant de stocker des données avec une capacité limitée tout en garantissant que les entrées les moins récemment utilisées sont évincées en premier lorsque le cache atteint sa capacité maximale. 

## Fonctionnalités principales
1. **Mise en cache avec gestion LRU** :
   - Conservation des données sous forme de paires clé-valeur dans un `HashMap`.
   - Utilisation d'une liste de clés pour suivre l'ordre des accès et identifier les entrées à évincer.
2. **Capacité définie par l'utilisateur** :
   - L'utilisateur peut définir la capacité maximale du cache lors de son initialisation.
3. **Eviction automatique** :
   - Lorsqu'une nouvelle entrée est ajoutée et que le cache est plein, l'entrée la moins récemment utilisée est supprimée.

## Structure du projet

### Modules principaux
1. **`cache.rs`** :
   - Contient la structure générique `Cache<K, V>` qui implémente la logique LRU.
   - Les méthodes principales incluent :
     - `new(capacity: usize)`: Initialise un nouveau cache avec une capacité donnée.
     - `get(&mut self, key: &K) -> Option<&V>`: Récupère la valeur associée à une clé et met à jour son ordre dans le cache.
     - `put(&mut self, key: K, value: V)`: Ajoute ou met à jour une entrée dans le cache.
2. **`lib.rs`** :
   - Fournit une abstraction de haut niveau via la structure `LruCache`.
   - Simplifie l'utilisation du cache pour des paires de types spécifiques (`char` pour les clés et `u32` pour les valeurs).

### Tests d'intégration
Les tests d'intégration dans `integration_test.rs` valident les fonctionnalités du cache :
- **Mise à jour d'une clé existante** : Vérifie que les valeurs sont correctement mises à jour.
- **Insertion et récupération réussies** : Garantit que les éléments insérés sont récupérables.
- **Eviction LRU** : Teste que les entrées les moins récemment utilisées sont correctement évincées.
- **Respect de la capacité** : Vérifie que la capacité maximale du cache est respectée.

## Utilisation

### Initialisation du cache
Pour initialiser un cache avec une capacité spécifique :
```rust
let mut cache = Cache::new(3); // Capacité de 3
```

### Ajouter des éléments
Pour insérer ou mettre à jour des éléments :
```rust
cache.put('a', 1);
cache.put('b', 2);
```

### Accéder à des éléments
Pour accéder à une valeur :
```rust
if let Some(value) = cache.get(&'a') {
    println!("Valeur associée : {}", value);
}
```

### Gestion de l'éviction
Si la capacité est atteinte, la clé la moins récemment utilisée sera évincée :
```rust
cache.put('c', 3);
cache.put('d', 4); // 'a' sera évincée
```

## Exemples de tests
- Test d'éviction LRU :
```rust
#[test]
fn test_lru_cache_eviction() {
    let mut cache = Cache::new(2);
    cache.put('x', 10);
    cache.put('y', 20);
    cache.put('z', 30); // 'x' sera évincée
    assert_eq!(cache.get(&'x'), None);
}
```

