import { Link } from "react-router-dom";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="bg-black h-screen">
      <Navbar />
      {children}
    </div>
  );
}
function Navbar() {
  return (
    <nav className="flex justify-between items-center h-16 px-4 ">
      <Link to="/">
        <h1>SabiBoy</h1>
      </Link>
    </nav>
  );
}
