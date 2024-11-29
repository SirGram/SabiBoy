import { Link } from "react-router-dom";
import { 
  Home as HomeIcon, 
  Library as LibraryIcon, 
  Settings as SettingsIcon 
} from 'lucide-react';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="bg-base-background h-screen flex">
      <Navbar />
      <div className="w-full h-full">{children}</div>
    </div>
  );
}

function Navbar() {
  return (
    <nav className="bg-base-background w-20 flex flex-col justify-between items-center py-6 h-full border-r border-base-border">
      <div className="flex flex-col items-center space-y-6">
        <Link 
          to="/" 
          className="hover:bg-muted/20 p-3 rounded-lg transition-colors group"
        >
          <HomeIcon 
            className="text-base-foreground group-hover:text-primary w-6 h-6" 
          />
        </Link>
        <Link 
          to="/library" 
          className="hover:bg-muted/20 p-3 rounded-lg transition-colors group"
        >
          <LibraryIcon 
            className="text-base-foreground group-hover:text-primary w-6 h-6" 
          />
        </Link>
      </div>
      
      <div className="flex flex-col items-center space-y-4">
        <Link 
          to="/options" 
          className="hover:bg-muted/20 p-3 rounded-lg transition-colors group"
        >
          <SettingsIcon 
            className="text-base-foreground group-hover:text-primary w-6 h-6" 
          />
        </Link>
        <div className="text-muted-foreground text-xs">v0.0</div>
      </div>
    </nav>
  );
}