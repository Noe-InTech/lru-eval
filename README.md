# 🚀 Cache LRU en Rust

Ce projet implémente un **cache LRU (Least Recently Used)** en Rust. Un cache LRU est une structure de données efficace permettant de gérer une quantité limitée d'éléments en mémoire, tout en optimisant les accès récents.

---

## ✨ Fonctionnalités principales

- 🔑 **Insertion** : Ajoutez des paires clé-valeur dans le cache.
- 🔄 **Mise à jour** : Modifiez la valeur associée à une clé existante.
- ⚡ **Récupération rapide** : Accédez rapidement à une valeur via sa clé.
- 🗑️ **Éviction automatique** : Lorsque la capacité maximale est atteinte, les éléments les moins récemment utilisés sont supprimés.
- 📏 **Capacité personnalisable** : Configurez la taille maximale du cache à l'initialisation.


---

## 🤔 Comportement du cache

### Insertion et éviction
1. Lorsque le cache atteint sa capacité maximale :
   - La clé la moins récemment utilisée est supprimée.
2. Si une clé déjà existante est insérée à nouveau :
   - Sa valeur est mise à jour.
   - Elle devient la clé la plus récemment utilisée.

### Ordre des clés
Les clés sont gérées dynamiquement en fonction des accès. Les clés les plus récentes sont toujours positionnées à la fin.

---

## 🧪 Tests

Pour valider le bon fonctionnement du cache, des tests unitaires et d'intégration sont inclus.

### Lancer tous les tests :
`
Pour un test normal : cargo test
Pour un test avec quelques détails : cargo test -- --nocapture
