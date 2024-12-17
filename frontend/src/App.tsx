import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Library from "./pages/Library/Library";
import Emulator from "./pages/Game/Emulator";
import { GameboyProvider } from "./context/GameboyContext";
import Options from "./pages/Options/Options";
import { OptionsProvider } from "./context/OptionsContext";
import Board from "./pages/Board/Board";
import OfflineEmulator from "./pages/Offline/OfflineEmulator";
import Login from "./pages/Login/Login";
import { AuthProvider } from "./context/AuthContext";

function App() {
  return (
    <AuthProvider>
      <OptionsProvider>
        <GameboyProvider>
          <Router>
            <Routes>
              <Route path="/login" element={<Login />} />
              <Route path="/" element={<Board />} />
              <Route path="/library" element={<Library />} />
              <Route path="/options" element={<Options />} />
              <Route path="/emulator" element={<Emulator />} />
              <Route path="/offline-emulator" element={<OfflineEmulator />} />
            </Routes>
          </Router>
        </GameboyProvider>
      </OptionsProvider>
    </AuthProvider>
  );
}

export default App;
