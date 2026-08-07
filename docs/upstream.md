# Mantener MANDA sincronizado con Kaku (upstream)

MANDA es un fork renombrado de [tw93/Kaku](https://github.com/tw93/Kaku)
(que a su vez es un fork de WezTerm). Este documento explica cómo recibir
los cambios de Kaku sin romper las diferencias propias de MANDA.

## Configuración (una sola vez)

```bash
cd ~/MANDA
git remote add upstream https://github.com/tw93/Kaku.git
git fetch upstream
```

## Sincronizar con Kaku (cada 2 semanas aprox.)

```bash
git fetch upstream
git merge upstream/main
```

> Si sale "Already up to date", ya estás al día.

### ¿Por qué ahora funciona limpio?

MANDA se creó como un repo **nuevo** (sin la historia de Kaku), por lo que
un `git merge` directo daba 133 conflictos `add/add` (git no tenía ancestro
común). Se hizo un **re-root**: el árbol actual de MANDA se montó sobre
`upstream/main` como commit puente, preservando toda la historia de MANDA
como segundo padre. Desde entonces, `git merge upstream/main` es limpio
porque git ya conoce la relación.

### Reglas para mantener el merge limpio

1. **No renombres directorios/crates de MANDA** (`manda`, `manda-gui`,
   `crates/manda-*`). El sync depende de que esos nombres sean estables.
2. **Los archivos que son 100% de MANDA** (`web/`, `install/`,
   `crates/manda-ai-utils/src/providers.rs`, `docs/upstream.md`) no existen
   en Kaku → git los respeta automáticamente en cada merge.
3. **Los archivos compartidos** (código Rust, config) deben seguir el flujo
   normal de merge. Si un commit de Kaku cambia algo que MANDA modificó,
   revisa el conflicto y conserva la versión de MANDA (los presets de
   proveedores de IA son tuyos).
4. **`git pull` de origin** sigue siendo con tu `origin`, no con upstream.

## Flujo completo recomendado

```bash
# 1) Traer lo nuevo de Kaku
git fetch upstream

# 2) Ver qué llegó
git log --oneline HEAD..upstream/main

# 3) Fusionar
git merge upstream/main

# 4) Si hay conflictos, resuélvelos (regla 3) y:
git add .
git commit

# 5) Publicar
git push origin main
```

## Si algún día quieres volver a re-rootear (no recomendado)

```bash
NEWROOT=$(git commit-tree HEAD^{tree} -p upstream/main -p HEAD \
  -m 'Re-root: MANDA tree merged onto Kaku main')
git reset --hard $NEWROOT
```

> Esto reescribe la historia; úsalo solo si Kaku rompe el ancestro común
> (p. ej. si un día se reestructura el repo).
