import { useState } from "react";
import { Trash2, UserPlus } from "lucide-react";
import Layout from "../../components/Layout/MainLayout";
import { useAuth } from "../../context/AuthContext";
import { useNavigate } from "react-router-dom";

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
  const { user, logout, fetchWithAuth } = useAuth();
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
      const response = await fetchWithAuth(`/api/users/change-password`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          currentPassword: passwordForm.currentPassword,
          newPassword: passwordForm.newPassword,
        }),
      });

      if (response.ok) {
        alert("Password changed successfully");
        setPasswordForm((prev) => ({
          ...prev,
          currentPassword: "",
          newPassword: "",
          confirmPassword: "",
        }));
      } else {
        const errorData = await response.json();
        alert(errorData.message || "Failed to change password");
      }
    } catch (error) {
      console.error("Password change error:", error);
      alert("An error occurred while changing password");
    }
  };

  const fetchUsers = async () => {
    try {
      const response = await fetchWithAuth("/api/users");
      if (response.ok) {
        const userData = await response.json();
        setUsers(userData);
        setShowUserList(true);
      }
    } catch (error) {
      console.error("Failed to fetch users:", error);
    }
  };

  const handleCreateUser = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      const response = await fetchWithAuth("/api/users", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(newUserForm),
      });

      if (response.ok) {
        alert("User created successfully");
        setNewUserForm({ email: "", password: "", role: "normal" });
        fetchUsers(); // Refresh user list
      } else {
        const errorData = await response.json();
        alert(errorData.message || "Failed to create user");
      }
    } catch (error) {
      console.error("User creation error:", error);
      alert("An error occurred while creating user");
    }
  };

  const handleDeleteAccount = async () => {
    const confirmDelete = window.confirm(
      "Are you sure you want to delete your account? This action cannot be undone."
    );

    if (confirmDelete) {
      try {
        const response = await fetchWithAuth(`/api/users/${user?.id}`, {
          method: "DELETE",
        });

        if (response.ok) {
          logout();
          navigate("/login");
        } else {
          const errorData = await response.json();
          alert(errorData.message || "Failed to delete account");
        }
      } catch (error) {
        console.error("Account deletion error:", error);
        alert("An error occurred while deleting account");
      }
    }
  };

  const handleDeleteUser = async (userId: string) => {
    const confirmDelete = window.confirm(
      "Are you sure you want to delete this user?"
    );

    if (confirmDelete) {
      try {
        const response = await fetchWithAuth(`/api/users/${userId}`, {
          method: "DELETE",
        });

        if (response.ok) {
          fetchUsers();
        } else {
          const errorData = await response.json();
          alert(errorData.message || "Failed to delete user");
        }
      } catch (error) {
        console.error("User deletion error:", error);
        alert("An error occurred while deleting user");
      }
    }
  };

  const handleLogOut = () => {
    logout();
    navigate("/login");
  };

  return (
    <Layout>
      <div className="flex flex-col gap-4 h-full items-center  max-w-md mx-auto">
        <h1 className="text-2xl font-bold mb-6">User Management</h1>

        {/* Account Details */}
        <div className="w-full border-base-border border-b  mb-4">
          <h2 className="text-xl font-semibold mb-3 flex flex-col ">
            Account Details
          </h2>
          <div className="flex flex-col justify-between gap-4 ">
            <div className="flex justify-between items-center ">
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
        </div>

        {/* Password Change */}
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-4">Change Password</h2>
          <form onSubmit={handlePasswordChange} className="space-y-4">
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
        </div>

        {/* Account Deletion */}
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-3">Account Actions</h2>
          <button
            onClick={handleDeleteAccount}
            className="w-full bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition"
          >
            <Trash2 className="mr-2 inline" /> Delete Account
          </button>
        </div>

        {/* Superuser Sections */}
        {user?.role === "superuser" && (
          <>
            {/* Create New User */}
            <div className="w-full">
              <h2 className="text-xl font-semibold mb-4">Create New User</h2>
              <form onSubmit={handleCreateUser} className="space-y-4">
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
                      role: e.target.value,
                    }))
                  }
                  className="w-full px-3 py-2 border rounded"
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
            </div>

            {/* User List */}
            <div className="w-full">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-semibold">User List</h2>
                <button
                  onClick={fetchUsers}
                  className="bg-secondary hover:bg-secondary-hover px-3 py-1 rounded"
                >
                  Refresh Users
                </button>
              </div>
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
                            {userEntry.role == "normal" && (
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
          </>
        )}
      </div>
    </Layout>
  );
}
