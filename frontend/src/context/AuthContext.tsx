import React, {
  createContext,
  useState,
  useContext,
  ReactNode,
  useEffect,
} from "react";

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
  fetchWithAuth: (input: RequestInfo, init?: RequestInit) => Promise<Response>;
}

const AuthContext = createContext<AuthContextType>({
  user: null,
  login: async () => {},
  logout: () => {},
  isAuthenticated: false,
  fetchWithAuth: async () => new Response(),
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
      const response = await fetch("/api/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });
      console.log(response);

      if (!response.ok) {
        throw new Error("Login failed");
      }

      const data = await response.json();
      const { access_token, user } = data;

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

  // Attach token to outgoing requests
  const fetchWithAuth = async (input: RequestInfo, init: RequestInit = {}) => {
    const token = localStorage.getItem("access_token");

    // Create headers, ensuring a new Headers object
    const headers = new Headers(init.headers || {});

    if (token) {
      headers.set("Authorization", `Bearer ${token}`);
    }

    // Explicitly set Content-Type if not already set
    if (!headers.get("Content-Type")) {
      headers.set("Content-Type", "application/json");
    }


    return fetch(input, {
      ...init,
      headers,
    });
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        logout,
        isAuthenticated: !!user,
        fetchWithAuth,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);
