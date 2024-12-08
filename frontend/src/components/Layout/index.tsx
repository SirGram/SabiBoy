import React from "react";
import { Link, useLocation } from "react-router-dom";
import {
  Github,
  HomeIcon,
  LibraryIcon,
  SettingsIcon,
  Gamepad,
  LucideProps,
} from "lucide-react";
import packageJson from "../../../package.json";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="h-screen w-full flex flex-col md:flex-row">
      <Navbar
        menuItems={[
          { label: "Board", to: "/", icon: HomeIcon },
          { label: "Library", to: "/library", icon: LibraryIcon },
          { label: "Offline", to: "/offline-emulator", icon: Gamepad },
          { label: "Options", to: "/options", icon: SettingsIcon },
        ]}
        footer={{
          githubLink: "https://github.com/SirGram/SabiBoy",
          version: packageJson.version,
        }}
      />
      <div className="w-full h-full bg-base-background">{children}</div>
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
      className="fixed bottom-0 left-0 right-0 bg-base-background/90 
      md:static md:w-24 md:h-full 
      flex flex-row md:flex-col 
      justify-between items-center 
      border-t md:border-r border-base-border 
      shadow-lg transition-all z-10"
    >
      {/* Top Menu Section */}
      <div className="flex flex-row md:flex-col items-center w-full justify-around md:justify-center">
        {menuItems.map((item) => (
          <NavItem
            key={item.to}
            label={item.label}
            to={item.to}
            Icon={item.icon}
          />
        ))}
      </div>

      {/* Footer Section */}
      <div className="hidden md:flex flex-col items-center space-y-1 mb-4 w-full">
        <a
          href={footer.githubLink}
          target="_blank"
          rel="noopener noreferrer"
          className="group relative hover:bg-muted/20 rounded-md py-3 px-5 transition-colors duration-300"
        >
          <Github className="text-base-foreground group-hover:text-primary w-7 h-7 transition-all duration-300 transform group-hover:scale-110" />
          <span className="text-muted-foreground text-xs font-semibold">
            {footer.version}
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
      className={`text-center group flex flex-col relative hover:bg-muted/20 
        py-3 md:py-5 px-3 w-full rounded-md transition-colors duration-300 
        justify-center items-center
        ${isActive ? "pointer-events-none" : ""}
        `}
    >
      <Icon
        className={`w-6 h-6 md:w-7 md:h-7 transition-all duration-300 transform group-hover:scale-110 ${
          isActive
            ? "text-blue-500"
            : "text-base-foreground group-hover:text-primary"
        }`}
      />
      <span className="text-xs md:text-sm text-muted-foreground hidden md:block">
        {label}
      </span>
    </Link>
  );
}
