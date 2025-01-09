import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import Library from "./pages/Library/Library";
import Emulator from "./pages/Game/Emulator";
import { GameboyProvider } from "./context/GameboyContext";
import { OptionsProvider } from "./context/OptionsContext";
import Board from "./pages/Board/Board";
import OfflineEmulator from "./pages/Offline/OfflineEmulator";
import Login from "./pages/Login/Login";
import { AuthProvider } from "./context/AuthContext";
import { useAuth } from "./context/AuthContext";
import { ReactNode } from "react";
import LoadingScreen from "./components/LoadingScreen";

function RequireAuth({ children }: { children: ReactNode }) {
  const { isAuthenticated, isLoading } = useAuth();
  if (isLoading) {
    return <LoadingScreen />;
  }

  if (!isAuthenticated && !isLoading) {
    return <Navigate to="/login" />;
  }

  return <>{children}</>;
}

function App() {
  return (
    <AuthProvider>
      <OptionsProvider>
        <GameboyProvider>
          <Router>
            <Routes>
              {/* Public routes */}
              <Route path="/login" element={<Login />} />
              <Route path="/offline-emulator" element={<OfflineEmulator />} />

              {/* Protected routes */}
              <Route
                path="/"
                element={
                  <RequireAuth>
                    <Board />
                  </RequireAuth>
                }
              />
              <Route
                path="/board"
                element={
                  <RequireAuth>
                    <Board />
                  </RequireAuth>
                }
              />
              <Route
                path="/library"
                element={
                  <RequireAuth>
                    <Library />
                  </RequireAuth>
                }
              />
              <Route
                path="/emulator"
                element={
                  <RequireAuth>
                    <Emulator />
                  </RequireAuth>
                }
              />
            </Routes>
          </Router>
        </GameboyProvider>
      </OptionsProvider>
    </AuthProvider>
  );
}

export default App;
