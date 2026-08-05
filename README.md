# jarvis

Rust workspace для JarVIS Core — реалізація за мотивами архітектурної
специфікації `jarvis-core-architecture-spec.md`.

## Структура

```
jarvis/
│
├── Cargo.toml            — workspace-маніфест
│
├── crates/
│   ├── jarvis-core/       — бібліотека: домени, трейти, події (без I/O з зовнішнім світом)
│   ├── jarvis-cli/        — тонкий CLI-виконавець над jarvis-core
│   └── jarvis-app/        — фоновий оркестратор/демон над jarvis-core
│
└── README.md
```

## Статус

**TASK 001** — створено лише каркас workspace: три крейти компілюються,
`jarvis-core` експортує тимчасову функцію-маркер `version()`, жодної
бізнес-логіки ще не реалізовано. Наступні задачі додаватимуть модулі
відповідно до розділів архітектурної специфікації (§2): audio, listener,
stt, intent, commands, voices, ipc, db, i18n.

## Збірка

```bash
cargo build
cargo test
```
