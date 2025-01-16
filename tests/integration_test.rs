use lru_eval_noe::cache::Cache;

#[test]
fn test_lru_cache_update_existing_key() {
    println!("\n=== Test : Mise à jour d'une clé existante ===");

    let mut cache = Cache::new(3);

    // Insertion de 3 éléments dans le cache
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("État initial du cache : {:?}", cache.keys());
    
    // Mise à jour de la clé 'b'
    cache.put('b', 20);
    println!("Après mise à jour de 'b' : {:?}", cache.keys());

    // Vérification de la mise à jour
    assert_eq!(cache.get(&'b'), Some(&20), "La clé 'b' aurait dû être mise à jour.");
    println!("=== Fin du test ===");
}

#[test]
fn test_lru_cache_success() {
    println!("\n=== Test : Insertion et récupération réussies ===");

    let mut cache = Cache::new(3);

    // Insertion de 3 éléments dans le cache
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("État du cache après trois insertions : {:?}", cache.keys());

    // Vérification de la récupération des éléments
    assert_eq!(cache.get(&'a'), Some(&1), "La clé 'a' doit être dans le cache.");
    assert_eq!(cache.get(&'b'), Some(&2), "La clé 'b' doit être dans le cache.");
    assert_eq!(cache.get(&'c'), Some(&3), "La clé 'c' doit être dans le cache.");

    println!("=== Fin du test ===");
}

#[test]
fn test_lru_cache_eviction() {
    println!("\n=== Test : Éviction LRU ===");

    let mut cache = Cache::new(3);

    // Insertion de 3 éléments dans le cache
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("État initial du cache : {:?}", cache.keys());

    // Insertion d'un 4e élément pour provoquer une éviction
    cache.put('d', 4);
    println!("État du cache après éviction : {:?}", cache.keys());

    // Vérification des évictions
    assert_eq!(cache.get(&'a'), None, "La clé 'a' doit avoir été évincée.");
    assert_eq!(cache.get(&'b'), Some(&2), "La clé 'b' doit être dans le cache.");
    assert_eq!(cache.get(&'c'), Some(&3), "La clé 'c' doit être dans le cache.");
    assert_eq!(cache.get(&'d'), Some(&4), "La clé 'd' doit être dans le cache.");

    println!("=== Fin du test ===");
}

#[test]
fn test_lru_cache_capacity() {
    println!("\n=== Test : Respect de la capacité maximale ===");

    let mut cache = Cache::new(3);

    // Insertion de 3 éléments dans le cache
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("État du cache après trois insertions : {:?}", cache.keys());

    // Insertion d'un 4e élément
    cache.put('d', 4);
    println!("État du cache après insertion de 'd' : {:?}", cache.keys());

    // Vérification des évictions
    assert_eq!(cache.get(&'a'), None, "La clé 'a' doit avoir été évincée.");
    assert_eq!(cache.get(&'b'), Some(&2), "La clé 'b' doit être dans le cache.");
    assert_eq!(cache.get(&'c'), Some(&3), "La clé 'c' doit être dans le cache.");
    assert_eq!(cache.get(&'d'), Some(&4), "La clé 'd' doit être dans le cache.");

    println!("=== Fin du test ===");
}
