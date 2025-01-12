import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../../context/AuthContext";
import { Eye, EyeOff } from "lucide-react";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError("");

    try {
      await login(email, password);
      navigate("/");
    } catch (err) {
      setError("Invalid email or password. Please try again.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-primary/30 via-accent/20 to-primary/30 px-4">
      {/* Logo Card */}
      <div className="w-fit max-w-md mb-6 p-8 bg-base-background/80 backdrop-blur-sm rounded-xl shadow-xl text-center">
        <div className="flex flex-col items-center gap-2">
          <img
            src="/icon.svg"
            alt="SabiBoy Logo"
            className="w-20 h-20 transition-transform hover:scale-105"
          />
          <h1 className="text-xl  text-base-foreground font-mono">SabiBoy</h1>
          <h2 className="text-thin text-sm text-base-foreground/50">
            Your Gameboy Library
          </h2>
        </div>
      </div>

      {/* Login Card */}
      <div className="w-full max-w-md p-6  bg-base-background/80 rounded-lg shadow-lg">
        <h2 className="text-2xl font-bold text-center text-gray-200 mb-6">
          Login to Your Account
        </h2>

        {error && (
          <div className="mb-4 p-3 text-sm text-destructive text-center bg-destructive/10 rounded-md">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label
              htmlFor="email"
              className="block text-sm font-medium text-gray-300 mb-1"
            >
              Email Address
            </label>
            <input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full px-4 py-2 border bg-base-background border-muted text-base-foreground rounded-lg focus:ring-2 focus:ring-secondary focus:border-secondary transition-colors"
              placeholder="Enter your email"
              required
            />
          </div>

          <div>
            <label
              htmlFor="password"
              className="block text-sm font-medium text-gray-300 mb-1"
            >
              Password
            </label>
            <div className="relative">
              <input
                id="password"
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="w-full px-4 py-2 border bg-base-background border-muted text-base-foreground rounded-lg focus:ring-2 focus:ring-secondary focus:border-secondary transition-colors pr-10"
                placeholder="Enter your password"
                required
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300 transition-colors"
                aria-label={showPassword ? "Hide password" : "Show password"}
              >
                {showPassword ? (
                  <EyeOff className="h-5 w-5" />
                ) : (
                  <Eye className="h-5 w-5" />
                )}
              </button>
            </div>
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className="w-full py-2 px-4 bg-primary text-white font-semibold rounded-lg hover:bg-primary-hover focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? "Signing in..." : "Sign In"}
          </button>

          <div className="relative my-6">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-t border-gray-300"></div>
            </div>
            <div className="relative flex justify-center text-sm items-center">
              <span className="px-2 bg-base-background text-gray-200">or</span>
            </div>
          </div>

          <Link
            to="/offline-emulator"
            className="block w-full py-2 px-4 bg-secondary/50 text-white font-semibold rounded-lg hover:bg-secondary-hover/50 focus:outline-none focus:ring-2 focus:ring-secondary focus:ring-offset-2 transition-colors text-center"
          >
            Continue Offline
          </Link>
        </form>
      </div>
    </div>
  );
}
