import React from "react";
import { Link } from "react-router-dom";
import {
  Github,
  HomeIcon,
  LibraryIcon,
  SettingsIcon,
  Gamepad,
  LucideProps,
} from "lucide-react";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="h-screen flex">
      <Navbar
        menuItems={[
          { label: "Board", to: "/", icon: HomeIcon },
          { label: "Library", to: "/library", icon: LibraryIcon },
          { label: "Emulator", to: "/emulator", icon: Gamepad },
          { label: "Options", to: "/options", icon: SettingsIcon },
        ]}
        footer={{
          githubLink: "https://github.com/SirGram/SabiBoy",
          version: "v0.0",
        }}
      />
      <div className="ml-24 w-full h-full">{children}</div>
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
    <nav className="fixed  bg-base-background w-24 flex flex-col justify-between items-center py-6 h-full border-r border-base-border shadow-lg transition-all">
      {/* Top Menu Section */}
      <div className="flex flex-col items-center ">
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
      <div className="flex flex-col items-center space-y-1 mb-4 w-full">
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
  return (
    <Link
      to={to}
      className="group flex flex-col relative hover:bg-muted/20 py-5 px-3  w-full rounded-md transition-colors duration-300 justify-center items-center"
    >
      <Icon className="text-base-foreground group-hover:text-primary w-7 h-7 transition-all duration-300 transform group-hover:scale-110" />
      <span className="text-sm text-muted-foreground">{label}</span>
    </Link>
  );
}
