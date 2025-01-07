import React, {
  createContext,
  useState,
  useContext,
  ReactNode,
  useEffect,
} from "react";
import api from "../api/client";

interface User {
  id: string;
  email: string;
  role: "normal" | "superuser";
}

interface AuthContextType {
  user: User | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType>({
  user: null,
  login: async () => {},
  logout: () => {},
  isAuthenticated: false,
});

export const AuthProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [user, setUser] = useState<User | null>(null);

  // Check for existing token on app load
  useEffect(() => {
    const token = localStorage.getItem("access_token");
    console.log(token);
    if (token) {
      verifyToken(token);
    }
  }, []);

  const verifyToken = async (token: string) => {
    try {
      const response = await fetch("/auth/profile", {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error("Token verification failed");
      }

      const data = await response.json();
      setUser(data);
    } catch {
      logout();
    }
  };

  const login = async (email: string, password: string) => {
    try {
      const response = await api.post("/api/auth/login", {
        email,
        password,
      });
      console.log(response);

      const { access_token, user } = response.data;

      localStorage.setItem("access_token", access_token);

      // Set user
      setUser(user);
    } catch (error) {
      console.error("Login failed", error);
      throw error;
    }
  };

  const logout = () => {
    localStorage.removeItem("access_token");
    setUser(null);
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        logout,
        isAuthenticated: !!user,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);
