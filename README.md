# Apprentissage Rust 🦀

> Regroupement de projets pratiques pour apprendre Rust, en suivant le [Rust Book officiel](https://doc.rust-lang.org/book/).

---

## Table des matières

1. [Vue d'ensemble](#vue-densemble)
2. [Structure du workspace](#structure-du-workspace)
3. [Projets](#projets)
   - [Hello World!](#hello-world) — *Chapitre 1 — Premier programme*
   - [variables](#variables) — *Chapitre 3 — Types et mutabilité*
   - [function](#function) — *Chapitre 4 — Fonctions et slices* ⚠️
   - [guessing_game](#guessing_game) — *Chapitre 2 — Projet interactif*
   - [struct_enum](#struct_enum) — *Chapitre 5 — Structs*
   - [rectangles](#rectangles) — *Chapitre 5 — Méthodes sur structs*
   - [enum_ressource](#enum_ressource) — *Chapitre 6 — Enums et pattern matching* ⚠️
   - [restaurant](#restaurant) — *Chapitre 7 — Modules et visibilité*
4. [État des projets](#état-des-projets)
5. [Prérequis](#prérequis)
6. [Lancer un projet](#lancer-un-projet)
7. [Ressources](#ressources)

---

## Vue d'ensemble

Ce dépôt est un **workspace Cargo** regroupant plusieurs mini-projets indépendants, chacun explorant un concept fondamental du langage Rust. Les projets suivent globalement la progression du Rust Book, du "Hello World" jusqu'aux modules et enums avancés.

---

## Structure du workspace

```
Rust/
├── Cargo.toml             # Workspace racine (liste tous les membres)
├── Cargo.lock             # Lockfile des dépendances
├── README.md              # Ce fichier
│
├── Hello World!/          # Premier programme (hors workspace)
├── variables/             # Types, mutabilité, tuples, tableaux
├── function/              # Fonctions, slices, références         ⚠️ WIP
├── guessing_game/         # Projet interactif avec I/O et rand
├── struct_enum/           # Structs nommées et tuple-structs
├── rectangles/            # Structs avec méthodes (impl)
├── enum_ressource/        # Enums, pattern matching, Option       ⚠️ WIP
└── restaurant/            # Modules, lib.rs, visibilité
```

---

## Projets

### Hello World!
> Chapitre 1 du Rust Book

Introduction au compilateur Rust et à la macro `println!`.
- Premier programme en Rust
- Compilation avec `rustc` / `cargo run`

---

### variables
> Chapitre 3 — Variables et types de données

Exploration des types primitifs et des structures de données de base.
- Mutabilité avec `mut` vs immutabilité par défaut
- Types scalaires : entiers (`i32`, `f32`, `f64`), booléens, `char`
- Types composés : tuples `(i32, f64, u8)` et tableaux `[i32; 5]`
- Destructuration de tuple
- ⚠️ Point à corriger : point-virgule manquant après `let remainder = 43 % 5`

---

### function
> Chapitre 4 — Fonctions et ownership (slices)

Exploration des fonctions et de la gestion mémoire par référence.
- Déclaration et appel de fonctions avec paramètres
- Références et emprunt (`&String`)
- Slices de chaînes (`&str`) et la fonction `first_word`
- ⚠️ WIP : `another_function` retourne `5` sans type de retour déclaré (`-> i32`), et `let mut five = another_function` est incomplet (manque les parenthèses d'appel)

---

### guessing_game
> Chapitre 2 — Projet guidé

Premier projet complet et interactif du Rust Book.
- Lecture depuis l'entrée standard avec `io::stdin()`
- Génération de nombre aléatoire avec la crate externe `rand`
- Gestion d'erreurs avec `match` et `Result`
- Boucle `loop` avec condition de sortie (`break`)
- Pattern matching avec `Ordering`

---

### struct_enum
> Chapitre 5 — Structs

Définition et instanciation des différentes formes de structs.
- Structs nommées avec champs typés (`User`)
- Syntaxe de mise à jour de struct (`..user1`)
- Tuple-structs (`Color`, `Point`)
- Unit-like struct (`AlwaysEqual`)
- Fonction constructeur `build_user`

---

### rectangles
> Chapitre 5 — Méthodes sur les structs

Utilisation de `impl` pour associer des comportements à une struct.
- Struct `Rectangle` avec `width` et `height`
- Méthode `area(&self)` — calcul de surface
- Méthode `can_hold(&self, other: &Rectangle)` — comparaison
- Dérivation du trait `Debug` (`#[derive(Debug)]`)
- Macros de débogage `println!("{:#?}")` et `dbg!()`

---

### enum_ressource
> Chapitre 6 — Enums et pattern matching

Exploration approfondie des enums et du pattern matching.
- Enums avec données associées (`Coin`, `IpAddr`)
- `impl` sur un enum (`UsState::existed_in`)
- Pattern matching exhaustif avec `match`
- `if let` pour le matching simplifié
- `let-else` pour le dépaquetage conditionnel
- Retour de `Option<String>`
- ⚠️ WIP : variable `coin` non déclarée dans `main` — le `main` est à compléter pour relier les fonctions définies

---

### restaurant
> Chapitre 7 — Modules et packages

Organisation du code en modules avec une bibliothèque (`lib.rs`).
- Structure en `mod` imbriqués (`front_of_house > hosting / serving`)
- Utilisation de `lib.rs` au lieu de `main.rs` (crate de type library)
- Séparation des responsabilités : `add_to_waitlist`, `serve_order`, `take_payment`
- 💡 Prochaine étape : ajouter `pub` sur les modules et fonctions pour explorer la visibilité (thème central du chapitre 7)

---

## État des projets

| Projet          | Chapitre | Compile | Statut       |
|-----------------|----------|---------|--------------|
| Hello World!    | 1        | ✅      | Complet      |
| variables       | 3        | ⚠️      | Point-virgule manquant |
| function        | 4        | ❌      | WIP          |
| guessing_game   | 2        | ✅      | Complet      |
| struct_enum     | 5        | ✅      | Complet      |
| rectangles      | 5        | ✅      | Complet      |
| enum_ressource  | 6        | ❌      | WIP          |
| restaurant      | 7        | ✅      | En cours     |

---

## Prérequis

- [Rust & Cargo](https://www.rust-lang.org/tools/install) installés via `rustup`

```bash
rustup --version
cargo --version
```

---

## Lancer un projet

Depuis la racine du workspace, cibler un membre spécifique :

```bash
# Compiler et exécuter un projet
cargo run -p guessing_game

# Compiler seulement
cargo build -p rectangles

# Vérifier sans compiler
cargo check -p variables
```

---

## Ressources

- 📖 [The Rust Programming Language (Rust Book)](https://doc.rust-lang.org/book/)
- 🔧 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 📦 [crates.io — registre des crates](https://crates.io/)
- 🔍 [docs.rs — documentation des crates](https://docs.rs/)
