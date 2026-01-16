# 📊 Analyseur de Logs Rust

Un outil en ligne de commande léger et rapide pour analyser des fichiers de logs et en extraire des statistiques utiles.

## ✨ Fonctionnalités

- 📈 **Statistiques globales** : Nombre total de lignes analysées
- 🏷️ **Répartition par niveau** : INFO, WARN, ERROR, DEBUG avec pourcentages
- 🌐 **Codes HTTP** : Distribution des codes de statut (200, 404, 500, etc.)
- 🔝 **Top 10 des IP** : Adresses IP les plus actives
- ❌ **Dernières erreurs** : Affichage des 5 erreurs les plus récentes
- ⚠️ **Derniers warnings** : Affichage des 5 avertissements les plus récents

## 🚀 Installation

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+)

### Compilation

```bash
# Cloner ou créer le projet
cargo new log-analyzer
cd log-analyzer

# Copier le code dans src/main.rs

# Compiler
cargo build --release

# L'exécutable sera dans target/release/log-analyzer
```

## 📖 Utilisation

### Commande de base

```bash
cargo run -- <chemin_vers_fichier.log>
```

### Exemples

```bash
# Analyser un fichier local
cargo run -- test.log

# Analyser des logs système (Linux)
cargo run -- /var/log/nginx/access.log
cargo run -- /var/log/apache2/error.log

# Avec l'exécutable compilé
./target/release/log-analyzer /var/log/app.log
```

## 📝 Format de logs supportés

L'analyseur détecte automatiquement :

- **Niveaux de log** : INFO, WARN, WARNING, ERROR, DEBUG (insensible à la casse)
- **Adresses IPv4** : Format standard (ex: 192.168.1.1)
- **Codes HTTP** : Nombres entre 100-599 (200, 404, 500, etc.)

### Exemple de fichier log

```
2024-01-15 10:30:45 INFO Server started on port 8080
192.168.1.100 - - [15/Jan/2024:10:31:02] "GET /index.html HTTP/1.1" 200 1234
192.168.1.101 - - [15/Jan/2024:10:31:10] "GET /missing.html HTTP/1.1" 404 234
2024-01-15 10:32:00 ERROR Database connection failed
2024-01-15 10:32:05 WARN Retrying connection (attempt 1/3)
192.168.1.100 - - [15/Jan/2024:10:32:15] "POST /api/login HTTP/1.1" 500 89
```

## 📊 Exemple de sortie

```
╔══════════════════════════════════════╗
║   RAPPORT D'ANALYSE DES LOGS         ║
╚══════════════════════════════════════╝

📊 Total de lignes analysées : 13

📋 Répartition par niveau :
   INFO : 3 (23.1%)
   ERROR : 2 (15.4%)
   WARN : 1 (7.7%)
   DEBUG : 1 (7.7%)

🌐 Codes de statut HTTP :
   200 : 4
   201 : 1
   403 : 1
   404 : 1
   500 : 1

🔝 Top 10 des adresses IP :
   192.168.1.100 : 5 requêtes
   192.168.1.101 : 1 requêtes
   192.168.1.102 : 1 requêtes

❌ Dernières erreurs (2) :
   2024-01-15 10:34:00 ERROR Failed to write to cache
   2024-01-15 10:32:00 ERROR Database connection failed

⚠️  Derniers avertissements (1) :
   2024-01-15 10:32:05 WARN Retrying connection (attempt 1/3)
```

## 🧪 Tests

```bash
# Exécuter les tests unitaires
cargo test

# Tests avec sortie détaillée
cargo test -- --nocapture
```

## 🛠️ Structure du projet

```
log-analyzer/
├── Cargo.toml          # Configuration du projet
├── README.md           # Ce fichier
├── src/
│   └── main.rs        # Code source principal
└── test.log           # Fichier de test (optionnel)
```

## 🔧 Développement

### Ajouter des fonctionnalités

Le code est organisé en plusieurs parties :

- `LogStats` : Structure de données pour les statistiques
- `analyze_line()` : Analyse ligne par ligne
- `extract_ip()` : Extraction des adresses IP
- `extract_status_code()` : Extraction des codes HTTP
- `print_report()` : Affichage formaté des résultats

### Idées d'améliorations

- [ ] Support des adresses IPv6
- [ ] Filtrage par plage de dates
- [ ] Export en JSON/CSV
- [ ] Détection de patterns suspects (brute force, etc.)
- [ ] Mode interactif avec filtres en temps réel
- [ ] Support de regex personnalisées
- [ ] Graphiques avec `plotters`
- [ ] Analyse en streaming pour gros fichiers

## 📄 Licence

Ce projet est libre d'utilisation pour un usage personnel et éducatif.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésite pas à :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/amelioration`)
3. Commit tes changements (`git commit -am 'Ajout fonctionnalité'`)
4. Push vers la branche (`git push origin feature/amelioration`)
5. Ouvrir une Pull Request

## 📞 Support

Pour toute question ou suggestion, ouvre une issue sur le dépôt du projet.

---

**Fait avec ❤️ et 🦀 Rust**
