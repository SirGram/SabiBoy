import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Library from "./pages/Library/Library";
import Emulator from "./pages/Game/Emulator";
import { GameboyProvider } from "./context/GameboyContext";
import Options from "./pages/Options/Options";
import { OptionsProvider } from "./context/OptionsContext";
import Board from "./pages/Board/Board";

function App() {
  return (
    <OptionsProvider>
      <GameboyProvider>
        <Router>
          <Routes>
            <Route path="/" element={<Board />} />
            <Route path="/library" element={<Library />} />
            <Route path="/options" element={<Options />} />
            <Route path="/emulator" element={<Emulator />} />
          </Routes>
        </Router>
      </GameboyProvider>
    </OptionsProvider>
  );
}

export default App;
