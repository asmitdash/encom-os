# Encom OS

> An operating system where the AI is the layer, not an app.

Encom OS is a Linux distribution built around the [Encom](https://github.com/asmitdash/encom) agent runtime. The agent isn't an icon you launch — it's the shell, the launcher, the notification surface, and the system service that ties them together. Local-first when you want it; cloud models when you don't.

```
┌──────────────────────────────────────────────────────────┐
│  Encom Shell  (Wayland compositor + agent surface)       │
├──────────────────────────────────────────────────────────┤
│  Encom Daemon  (encom-core, the AI layer)                │
│  · Memory   · Skills   · Model routing   · Scheduling    │
├──────────────────────────────────────────────────────────┤
│  Arch Linux base  +  Ollama  +  systemd  +  PipeWire     │
└──────────────────────────────────────────────────────────┘
```

## Two install modes

At first boot the installer asks you which one you want.

**Portable (Tails-style live USB)**
- Boots from USB, runs entirely in RAM, no disk writes.
- Cloud models only — you supply API keys for OpenAI, Anthropic, xAI/Grok, Google Gemini, Mistral, Cohere, Groq, DeepSeek, or any of the other built-in adapters.
- Optional encrypted persistent volume on the same USB for keys + skill state.
- Use case: borrowed hardware, kiosks, ephemeral work.

**Full install**
- Encom OS becomes your daily driver.
- All cloud-model options, **plus** Ollama for local models.
- During install you pick a default local model (Llama 3.x, Qwen, Mistral, Gemma, Hermes, …); the installer downloads it into the image so first boot has a usable agent without internet.
- Default model is switchable any time in **Settings → AI Layer**.

## What "AI is the layer" means in practice

- **Launcher.** `Super` opens an agent prompt, not a grid. Apps, files, contacts, settings — all addressable in natural language. Type `find that PDF Vidhi sent last week` and it does.
- **Notifications.** The agent triages them. You see the three that matter; the rest go into a digest the agent reads to you on demand.
- **System actions are skills.** "Mute everything for an hour," "back up Documents to my external drive," "what's eating my battery" — all skills, sandboxed, declarative permissions.
- **Local first.** Inference, memory, and skill execution stay on the box. Cloud only when you opt a specific provider in.
- **No telemetry.** None. No phone-home. No usage analytics. Period.

## Status

**Phase 0** — scaffolds the repo, the archiso profile, the Wayland shell crate, the first-boot wizard skeleton, and the Docker-based ISO build. Won't boot to a usable desktop yet.

| Phase | Ships |
|-------|-------|
| 0 | Repo, archiso profile skeleton, shell crate, first-boot wizard skeleton, Docker ISO build |
| 1 | Encom daemon integration (depends on `asmitdash/encom` Phase 1) |
| 2 | Wayland compositor + chat surface + launcher (smithay) |
| 3 | First-boot wizard — portable vs full, API keys, Ollama model picker |
| 4 | Notification ownership, system skills, settings panel |
| 5 | Theming, branding, install ISO published as a GitHub Release |

## Building the ISO (Docker, any host)

```bash
./build/iso.sh
# produces: out/encom-os-<version>-x86_64.iso
```

Requires Docker. The build script runs `archlinux:base-devel` in a privileged container with `archiso`. No Linux host required — works from Windows or macOS as long as Docker can run privileged containers.

## License

MIT. See [LICENSE](LICENSE).

The Arch base, the kernel, systemd, Mesa, PipeWire, and every other upstream package retain their own licenses. Encom OS only relicenses the things this repo writes.

## Disclaimer

Encom OS is **not affiliated with Disney, Tron, or any prior fictional or real "ENCOM" entity.** The name is a homage; the visual identity, code, and trademarks are unrelated and original.

## Author

[asmitdash](https://github.com/asmitdash) — also maintains the [Encom](https://github.com/asmitdash/encom) runtime that this OS is built on.
