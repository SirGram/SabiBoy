import { useEffect, useState } from "react";
import { Trash2, UserPlus } from "lucide-react";
import { useAuth } from "../../../context/AuthContext";
import { useNavigate } from "react-router-dom";
import api from "../../../api/client";
import { AxiosError } from "axios";
import CollapsibleList from "../../../components/CollapsibleList";

// Types for user and form states
interface UserFormData {
  email: string;
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
}

interface UserListEntry {
  _id: string;
  email: string;
  role: "normal" | "superuser";
}

export default function UserManagement() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  const [passwordForm, setPasswordForm] = useState<UserFormData>({
    email: user?.email || "",
    currentPassword: "",
    newPassword: "",
    confirmPassword: "",
  });

  const [users, setUsers] = useState<UserListEntry[]>([]);
  const [showUserList, setShowUserList] = useState(false);
  const [newUserForm, setNewUserForm] = useState({
    email: "",
    password: "",
    role: "normal",
  });

  const handlePasswordChange = async (e: React.FormEvent) => {
    e.preventDefault();

    if (passwordForm.newPassword !== passwordForm.confirmPassword) {
      alert("New passwords do not match");
      return;
    }

    try {
      await api.patch("/api/users/change-password", {
        currentPassword: passwordForm.currentPassword,
        newPassword: passwordForm.newPassword,
      });

      alert("Password changed successfully");
      setPasswordForm((prev) => ({
        ...prev,
        currentPassword: "",
        newPassword: "",
        confirmPassword: "",
      }));
    } catch (error) {
      const axiosError = error as AxiosError<{ message: string }>;
      const message =
        axiosError.response?.data?.message || "Failed to change password";
      console.error("Password change error:", error);
      alert(message);
    }
  };

  const fetchUsers = async () => {
    try {
      const response = await api.get("/api/users");

      const userData = await response.data;
      setUsers(userData);
      setShowUserList(true);
    } catch (error) {
      console.error("Failed to fetch users:", error);
    }
  };

  const handleCreateUser = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.post("/api/users", newUserForm);
      alert("User created successfully");
      setNewUserForm({ email: "", password: "", role: "normal" });
      fetchUsers();
    } catch (error) {
      console.error("User creation error:", error);
      alert("An error occurred while creating user");
    }
  };

  const handleDeleteAccount = async () => {
    if (
      !window.confirm(
        "Are you sure you want to delete your account? This action cannot be undone."
      )
    )
      return;

    try {
      await api.delete(`/api/users/${user?.id}`);
      logout();
      navigate("/login");
    } catch (error) {
      console.error("Account deletion error:", error);
      alert("An error occurred while deleting account");
    }
  };

  const handleDeleteUser = async (userId: string) => {
    if (!window.confirm("Are you sure you want to delete this user?")) return;

    try {
      await api.delete(`/api/users/${userId}`);
      fetchUsers();
    } catch (error) {
      console.error("User deletion error:", error);
      alert("An error occurred while deleting user");
    }
  };
  const handleLogOut = () => {
    logout();
    navigate("/login");
  };
  const handleDeleteAllGames = async () => {
    if (
      !window.confirm(
        "Are you sure you want to delete all the games from the common Library? This action cannot be undone."
      )
    )
      return;
    try {
      await api.delete(`/api/games`);
      alert("All games have been deleted successfully");
    } catch (error) {
      console.error("Game deletion error:", error);
      alert("An error occurred while deleting games");
    }
  };
  useEffect(() => {
    fetchUsers();
  }, []);

  return (
    <div className="flex flex-col gap-6 h-full items-center max-w-md mx-auto ">
      <CollapsibleList title="Account Details">
        <div className="w-full space-y-4">
          <div className="flex justify-between items-center">
            <span>Email</span>
            <span className="text-base-foreground/60">{user?.email}</span>
          </div>
          <button
            onClick={() => handleLogOut()}
            className="bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition w-full"
          >
            Log Out
          </button>
        </div>
        <h4>Change Password</h4>
        <form
          onSubmit={handlePasswordChange}
          className="space-y-4 w-full flex flex-col"
        >
          <input
            type="password"
            placeholder="Current Password"
            value={passwordForm.currentPassword}
            onChange={(e) =>
              setPasswordForm((prev) => ({
                ...prev,
                currentPassword: e.target.value,
              }))
            }
            className="w-full px-3 py-2 border rounded bg-transparent"
            required
          />
          <input
            type="password"
            placeholder="New Password"
            value={passwordForm.newPassword}
            onChange={(e) =>
              setPasswordForm((prev) => ({
                ...prev,
                newPassword: e.target.value,
              }))
            }
            className="w-full px-3 py-2 border rounded bg-transparent"
            required
          />
          <input
            type="password"
            placeholder="Confirm New Password"
            value={passwordForm.confirmPassword}
            onChange={(e) =>
              setPasswordForm((prev) => ({
                ...prev,
                confirmPassword: e.target.value,
              }))
            }
            className="w-full px-3 py-2 border rounded bg-transparent"
            required
          />
          <button
            type="submit"
            className="w-full bg-primary hover:bg-primary-hover py-2 rounded"
          >
            Change Password
          </button>
        </form>
        <h4>Account Actions</h4>
        <button
          onClick={handleDeleteAccount}
          className="w-full bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition"
        >
          <Trash2 className="mr-2 inline" /> Delete Account
        </button>
        {user?.role === "superuser" && (
          <button
            onClick={handleDeleteAllGames}
            className="w-full bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition"
          >
            <Trash2 className="mr-2 inline" /> Delete Library
          </button>
        )}
      </CollapsibleList>

      {user?.role === "superuser" && (
        <>
          <CollapsibleList title="Create New User">
            <form
              onSubmit={handleCreateUser}
              className="space-y-4 w-full flex flex-col"
            >
              <input
                type="email"
                placeholder="Email"
                value={newUserForm.email}
                onChange={(e) =>
                  setNewUserForm((prev) => ({
                    ...prev,
                    email: e.target.value,
                  }))
                }
                className="w-full px-3 py-2 border rounded bg-transparent"
                required
              />
              <input
                type="password"
                placeholder="Password"
                value={newUserForm.password}
                onChange={(e) =>
                  setNewUserForm((prev) => ({
                    ...prev,
                    password: e.target.value,
                  }))
                }
                className="w-full px-3 py-2 border rounded bg-transparent"
                required
              />
              <select
                value={newUserForm.role}
                onChange={(e) =>
                  setNewUserForm((prev) => ({
                    ...prev,
                    role: e.target.value as "normal" | "superuser",
                  }))
                }
                className="w-full px-3 py-2 border rounded bg-base-background"
              >
                <option value="normal">Normal User</option>
                <option value="superuser">Superuser</option>
              </select>
              <button
                type="submit"
                className="w-full bg-primary hover:bg-primary-hover py-2 rounded flex items-center justify-center"
              >
                <UserPlus className="mr-2" /> Create User
              </button>
            </form>
          </CollapsibleList>

          <CollapsibleList title="User List">
            <div className="w-full">
              {showUserList && (
                <div className="w-full overflow-x-auto">
                  <table className="w-full border border-base-border rounded-md overflow-hidden">
                    <thead>
                      <tr className="bg-muted">
                        <th className="border p-2">Email</th>
                        <th className="border p-2">Role</th>
                        <th className="border p-2">Actions</th>
                      </tr>
                    </thead>
                    <tbody>
                      {users.map((userEntry) => (
                        <tr key={userEntry._id} className="hover:bg-muted/50">
                          <td className="border p-2">{userEntry.email}</td>
                          <td className="border p-2">
                            {userEntry.role.toUpperCase()}
                          </td>
                          <td className="border p-2 text-center">
                            {userEntry.role === "normal" && (
                              <button
                                onClick={() => handleDeleteUser(userEntry._id)}
                                className="text-destructive hover:bg-destructive/10 p-1 rounded"
                              >
                                <Trash2 size={16} />
                              </button>
                            )}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          </CollapsibleList>
        </>
      )}
    </div>
  );
}
