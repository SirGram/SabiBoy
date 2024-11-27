import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Library from "./pages/Library/Library";
import Game from "./pages/Game";
import Layout from "./components/Layout";

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Library />} />
          <Route path="/:itemId" element={<Game />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;
