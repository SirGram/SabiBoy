# SabiBoy

**SabiBoy** is an online Game Boy library integrated with a high-performance emulator. Built with modern web technologies, it lets you play, manage, and customize your favorite retro games directly in your browser.

## 🏗Technical Architecture

1. **Emulator**

   - Written in **Rust** for speed and accuracy.
   - Compiled to **WebAssembly** using `wasm-pack` and `wasm-bindgen`.
   - Desktop-only lib with *Minifb* for testing.
2. **Frontend**

   - Built with **React + TypeScript**.
   - Features:
     - Responsive UI
     - Real-time emulator interaction
     - Customizable settings
3. **Backend**

   - Powered by **NestJS**.
   - Features:

     - RESTful API endpoints
     - User authentication with **JWT**
     - Save state management
     - MongoDB for user and save state data.
     - Static ROM storage served from the backend.
     - API integration for ROM management.

## 🎮 Emulator

### ✨Features

- DMG-based
- MBC 1,3,5
- Debug Mode

  - Comprehensive debugging tools
  - Real-time CPU registers and memory inspection
  - APU channel and PPU fetching viewer
  - Step-through execution
- Options

  - Custom color palettes
  - Configurable key bindings
  - Screen scaling
  - Audio settings

  ### 🧪 Testing


  | **Test**       | **Status** |
  | -------------- | ---------- |
  | **sm83**       | ✅         |
  | **Blargg CPU** | ✅         |
  | **DMG-ACID2**  | ❌         |
  | **Mooneye**    | ❌         |

## 🛠 Setting Up

```
# 1. Clone the repository
# 2. Fill environment variables for DB connection
# 3. Install Dependencies
cd frontend
npm i
cd ../backend
npm i
cd ../emulator/wasm
cargo i
# 4. Build WebAssembly module
wasm-pack build
cp ./pkg ../frontend/src/wasm
# 5. Launch the app
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📋 License

SabiBoy uses the **Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)** license. This means:

- You can use and modify the project for personal or educational purposes.
- You must credit the original creator.
- Commercial use is not allowed without permission.

**Note**: Always respect copyright laws when using ROMs.
