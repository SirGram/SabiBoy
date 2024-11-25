import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import GameboyDisplay from "./components/GameboyDisplay";
import Library from "./pages/Library/Library";

function App() {
  return (
    <Router>
      
        <Routes>
          <Route path="/" element={< Library/>} />
          <Route path="/:itemId" element={<GameboyDisplay />} />
        </Routes>
    </Router>
  );
}

export default App;
