import React from "react";
import { Link, useLocation } from "react-router-dom";
import {
  Github,
  HomeIcon,
  LibraryIcon,
  SettingsIcon,
  Gamepad,
  LucideProps,
  UserIcon,
} from "lucide-react";
import packageJson from "../../../package.json";
import { useAuth } from "../../context/AuthContext";

export default function Layout({ children }: { children: React.ReactNode }) {
  const { isAuthenticated } = useAuth();

  const publicMenuItems = [
    { label: "Offline", to: "/offline-emulator", icon: Gamepad },
    { label: "Options", to: "/options", icon: SettingsIcon },
  ];

  const privateMenuItems = [
    { label: "Board", to: "/", icon: HomeIcon },
    { label: "Library", to: "/library", icon: LibraryIcon },
    { label: "User", to: "/user", icon: UserIcon },
  ];

  // Combine menu items based on authentication status
  const menuItems = isAuthenticated
    ? [...privateMenuItems, ...publicMenuItems]
    : publicMenuItems;

  return (
    <div className="h-screen w-full flex flex-col">
      <div className="mb-16 md:mb-0 flex-1 overflow-y-auto bg-base-background">
        <div className="py-4 px-2 min-h-full md:ml-36  mx-auto">{children}</div>
      </div>
      <Navbar
        menuItems={menuItems}
        footer={{
          githubLink: "https://github.com/SirGram/SabiBoy",
          version: packageJson.version,
        }}
      />
    </div>
  );
}

type NavbarProps = {
  menuItems: {
    label: string;
    to: string;
    icon: React.ForwardRefExoticComponent<
      Omit<LucideProps, "ref"> & React.RefAttributes<SVGSVGElement>
    >;
  }[];
  footer: {
    githubLink: string;
    version: string;
  };
};

function Navbar({ menuItems, footer }: NavbarProps) {
  return (
    <nav
      className="fixed bottom-0 left-0 right-0 h-16 
             md:my-2 md:h-screen md:left-0 md:top-0 md:max-w-36
             bg-base-background/95 backdrop-blur-md
             flex md:flex-col items-center 
             border-t md:border-r md:border-t-0 border-base-border 
             shadow-xl z-10"
    >
      {/* Menu Items */}
      <div className="md:px-2 flex md:flex-col items-center w-full h-full md:h-auto md:flex-1 justify-around md:justify-start md:pt-0 ">
        {menuItems.map((item) => (
          <NavItem
            key={item.to}
            label={item.label}
            to={item.to}
            Icon={item.icon}
          />
        ))}
      </div>

      {/* Footer - Only visible on desktop */}
      <div className="hidden md:flex  items-center space-y-2 mb-6 w-full">
        <a
          href={footer.githubLink}
          target="_blank"
          rel="noopener noreferrer"
          className="group relative hover:bg-muted/20 rounded-lg py-3 px-5 transition-colors duration-300 flex items-center gap-2"
        >
          <Github className="text-lg-foreground group-hover:text-primary w-7 h-7 transition-all duration-300 transform group-hover:scale-110" />
          <span className="text-muted-foreground text-xs md:text-lg font-medium mt-1 block">
            v{footer.version}
          </span>
        </a>
      </div>
    </nav>
  );
}

type NavItemProps = {
  label: string;
  to: string;
  Icon: React.ForwardRefExoticComponent<
    Omit<LucideProps, "ref"> & React.RefAttributes<SVGSVGElement>
  >;
};

function NavItem({ label, to, Icon }: NavItemProps) {
  const { pathname } = useLocation();
  const isActive = pathname === to;

  return (
    <Link
      to={to}
      className={`text-center group flex flex-col md:flex-row md:gap-2 relative hover:bg-muted/20 
        py-2  md:py-4 px-3 w-full rounded-lg transition-all duration-300 
        justify-center  items-center md:justify-start 
        ${isActive ? "pointer-events-none" : "opacity-50 hover:opacity-100"}
        hover:shadow-md`}
    >
      <Icon
        className={`w-6 h-6 md:w-6 md:h-6 transition-all duration-300 transform group-hover:scale-110 ${
          isActive
            ? "text-primary"
            : "text-lg-foreground group-hover:text-base-foreground"
        }`}
      />
      <span
        className={`text-xs md:text-lg mt-1 md:mt-0 font-medium 
           ${
             isActive
               ? "text-primary"
               : "text-muted-foreground group-hover:text-base-foreground"
           }`}
      >
        {label}
      </span>
    </Link>
  );
}
