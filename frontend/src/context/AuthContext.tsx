import React, { createContext, useState, useContext, ReactNode, useEffect } from "react";
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
  isLoading: boolean; // Add loading state
}

const AuthContext = createContext<AuthContextType>({
  user: null,
  login: async () => {},
  logout: () => {},
  isAuthenticated: false,
  isLoading: true, 
});

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Monitor auth state changes
  useEffect(() => {
    console.log("Auth state updated:", {
      user,
      isAuthenticated: !!user,
      isLoading
    });
  }, [user, isLoading]);

  // Check for existing token and verify it on app load
  useEffect(() => {
    const verifyAuth = async () => {
      try {
        const token = localStorage.getItem("access_token");
        console.log("Token", token);
        if (!token) {
          setIsLoading(false);
          return;
        }

        const response = await api.get("/api/auth/profile");
        console.log("Response", response);
        setUser(response.data);
      } catch (error) {
        console.error("Token verification failed:", error);
        logout();
      } finally {
        setIsLoading(false);
      }
    };

    verifyAuth();
  }, []);

  const login = async (email: string, password: string) => {
    try {
      const response = await api.post("/api/auth/login", {
        email,
        password,
      });

      const { access_token, user } = response.data;

      // Store token and configure api client
      localStorage.setItem("access_token", access_token);
      api.defaults.headers.common["Authorization"] = `Bearer ${access_token}`;

      setUser(user);
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  };

  const logout = () => {
    localStorage.removeItem("access_token");
    delete api.defaults.headers.common["Authorization"];
    setUser(null);
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        logout,
        isAuthenticated: !!user,
        isLoading,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);