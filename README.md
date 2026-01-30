# Speak Please

[![Download PPTX]](speak-please.pptx?raw=true)

This is my project for `RAF Challenge` and `Neumann informatikaverseny` competitions.

### Project description

Speak Please is a gamified social networking platform designed to enhance interpersonal skills and boost self-esteem. The application encourages users to engage in real-world social interactions and personal growth challenges through a structured reward system and peer-to-peer connectivity.

### Tech stack

This is app written in Rust/Tauri(React + TailwindCSS + Typescript) for frontend and pure Rust for backend(axum + sea-orm).

### Implementated and ideas

1. Auth system: Implemented with JWT and refresh tokens, using Argon2 for crypto-safe hashing.
2. Lobby chat and Arena: Currently in progress (~80% complete).
3. Quest system: Fully implemented.
4. Quest archive: Allows users to track and review their progress.
5. Multi-platform: Works on Android and Linux (tested on Arch). I HAVE NO IDEA ABOUT WINDOWS; I don't use that piece of crap software. Technically, Tauri is cross-platform, so it should compile on Windows, but I refuse to touch that OS for testing. --iOS;
6. Tauri Specta: Used for connecting the frontend and backend with shared TypeScript interfaces (extremely useful).
7. Database: Powered by SeaORM (Rust ORM).
8. Performance: Everything is async with solid error handling (not in every single line yet, but that's in the plan).

## Build & Setup

If you want -\_('-')_\-

Since the primary target for this app is **Android**, follow these steps:

### 1. Backend (Server) Build

The backend is a standalone Rust binary.

```bash
cd backend # or your backend directory
cargo build --release
# The binary will be in target/release/
```

You need .env file in server folder:

```env
AWS_ACCESS_KEY_ID="euFxA8Zsny22K0bHI4ipEDJ3dliggs85"
AWS_SECRET_ACCESS_KEY="gPCsF50o3rb9r9LCo8poCcUa93Qm5nr5"
AWS_REGION="eu-004"
AWS_ENDPOINT="https://eu-004.s3.synologyc2.net"
```

Note: This is my free Synology C2 S3 storage; feel free to use it or set up your own.

And .env file in shared folder:

```env
JWT_SECRET_KEY=a94b7ccb13fa9cd18ae8f1be737c6fb5050b9e5b5f7dbb9093b08e5ccc1d1ce0
REFRESH_TOKEN_SALT="super-secret-static-salt-123"
```

You can use this

For frontend, you must add BACKEND_URL:

```env
BACKEND_URL=https://varicose-desiredly-fransisca.ngrok-free.dev
# I managed to make server works, but if I can't, im sorry
```

### 2. Frontend & Android Build

You need the Android SDK, NDK, and the Rust target for your phone's architecture (usually `aarch64-linux-android`).

**System Dependencies** (**Arch Linux**):

```bash
sudo pacman -S --needed base-devel curl wget openssl glue libsoup3 webkit2gtk-4.1 desktop-file-utils jdk17-openjdk
```

**Setup Android Target**:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

**Build the APK**:

```bash
pnpm install
```

### 3. Build for Android

```bash
pnpm run tauri android build
```

The resulting .apk or .aab will be located in src-tauri/gen/android/.

## Personal note

To be honest, I hate almost everything about both frontend and backend development, but working with Rust/Tauri was actually pretty fun. Personally, I see this project as an experiment in using such a powerful programming language for a "simple" idea (which, as it turns out, isn't that simple at all). I don't care about marketing, yes, it's important as hell, but I love to code and I technology, I don't love to work with people.

Specta is a lifesaver. I hate TypeScript's loose type checking (where you can just throw :any at everything and it "works"), but at least with Specta, the bridge to Rust is solid. I wasted a lot of time on JWT tokens, but it was worth it — I learned a lot. I also picked up some new Rust tricks along the way.

Tailwind isn't that bad (actually, it is, but I'll manage).

PS. Specta is rust library which make bindings to Rust structures and Tauri commands
