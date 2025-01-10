import React, { useState } from "react";
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
import Options from "../../pages/modals/Options/Options";
import Modal from "./Modal";
import UserManagement from "../../pages/modals/UserManagement/UserManagement";
import UploadNewGame from "../../pages/modals/UploadNewGame/UploadNewGame";
import { useModal } from "../../context/ModalContext";

export default function Layout({ children }: { children: React.ReactNode }) {
  const { isAuthenticated } = useAuth();
  const [showOptions, setShowOptions] = useState(false);
  const [showUserManagement, setShowUserManagement] = useState(false);
  const { showUploadModal, updateShowUploadModal } = useModal();

  const publicMenuItems = [
    {
      label: "Options",
      icon: SettingsIcon,
      onClick: () => setShowOptions(true),
      isButton: true,
    },
    { label: "Offline", to: "/offline-emulator", icon: Gamepad },
  ];

  const privateMenuItems = [
    { label: "Board", to: "/", icon: HomeIcon },
    { label: "Library", to: "/library", icon: LibraryIcon },
    {
      label: "User",
      onClick: () => setShowUserManagement(true),
      icon: UserIcon,
      isButton: true,
    },
  ];

  const menuItems = isAuthenticated
    ? [...privateMenuItems, ...publicMenuItems]
    : publicMenuItems;

  return (
    <div className="h-screen w-full flex flex-col">
      <div className="mb-16 md:mb-0 flex-1 overflow-y-auto bg-base-background">
        <div className="py-4 px-2 min-h-full md:ml-36 mx-auto">{children}</div>
      </div>
      <Navbar
        menuItems={menuItems}
        footer={{
          githubLink: "https://github.com/SirGram/SabiBoy",
          version: packageJson.version,
        }}
      />
      {showOptions && (
        <Modal isOpen={showOptions} onClose={() => setShowOptions(false)}>
          <Options />
        </Modal>
      )}
      {showUserManagement && (
        <Modal
          isOpen={showUserManagement}
          onClose={() => setShowUserManagement(false)}
        >
          <UserManagement />
        </Modal>
      )}
      {showUploadModal && (
        <Modal
          isOpen={showUploadModal}
          onClose={() => updateShowUploadModal(false)}
        >
          <UploadNewGame />
        </Modal>
      )}
    </div>
  );
}

type NavbarProps = {
  menuItems: {
    label: string;
    to?: string;
    icon: React.ForwardRefExoticComponent<
      Omit<LucideProps, "ref"> & React.RefAttributes<SVGSVGElement>
    >;
    onClick?: () => void;
    isButton?: boolean;
  }[];
  footer: {
    githubLink: string;
    version: string;
  };
};
function Navbar({ menuItems, footer }: NavbarProps) {
  return (
    <nav className="fixed bottom-0 left-0 right-0 h-16 md:my-2 md:h-screen md:left-0 md:top-0 md:max-w-36 bg-base-background/95 backdrop-blur-md flex md:flex-col items-center border-t md:border-r md:border-t-0 border-base-border shadow-xl z-10">
      <div className="md:px-2 flex md:flex-col items-center w-full h-full md:h-auto md:flex-1 justify-around md:justify-start md:pt-0">
        {menuItems.map((item, index) => (
          <>
            <NavItem key={item.label} {...item} />
            {index !== menuItems.length - 1 && (
              <div className="hidden md:flex border-t border-base-border border-1 w-full h-0.5"></div>
            )}
          </>
        ))}
      </div>
      {/* Footer Section */}
      <div className="hidden md:flex  items-center flex-col space-y-4 mb-6 w-full px-2 opacity-50 hover:opacity-100 ">
        <a
          href={footer.githubLink}
          target="_blank"
          rel="noopener noreferrer"
          className="group relative hover:bg-muted/20 rounded-lg py-2 px-3 transition-all duration-300 flex flex-col items-center justify-center w-full"
        >
          <Github className="w-6 h-6 transition-all duration-300 transform group-hover:scale-110 text-lg-foreground " />
          <span className="text-xs md:text-sm font-medium text-muted-foreground group-hover:text-base-foreground">
            v{footer.version}
          </span>
        </a>
      </div>
    </nav>
  );
}

type NavItemProps = {
  label: string;
  to?: string;
  icon: React.ForwardRefExoticComponent<
    Omit<LucideProps, "ref"> & React.RefAttributes<SVGSVGElement>
  >;
  onClick?: () => void;
  isButton?: boolean;
};

function NavItem({ label, to, icon: Icon, onClick, isButton }: NavItemProps) {
  const { pathname } = useLocation();
  const isActive = to && pathname === to;

  const baseClassName = `text-center group flex flex-col md:flex-row md:gap-2 relative hover:bg-muted/20 
    py-2 md:py-4 px-3 w-full rounded-lg transition-all duration-300 
    justify-center items-center md:justify-start 
    ${isActive ? "pointer-events-none" : "opacity-50 hover:opacity-100"}
    hover:shadow-md`;

  if (isButton) {
    return (
      <button onClick={onClick} className={baseClassName}>
        <Icon className="w-6 h-6 md:w-6 md:h-6 transition-all duration-300 transform group-hover:scale-110 text-lg-foreground group-hover:text-base-foreground" />
        <span className="text-xs md:text-lg mt-1 md:mt-0 font-medium text-muted-foreground group-hover:text-base-foreground">
          {label}
        </span>
      </button>
    );
  }

  return (
    <Link to={to!} className={baseClassName}>
      <Icon
        className={`w-6 h-6 md:w-6 md:h-6 transition-all duration-300 transform group-hover:scale-110 ${
          isActive
            ? "text-primary"
            : "text-lg-foreground group-hover:text-base-foreground"
        }`}
      />
      <span
        className={`text-xs md:text-lg mt-1 md:mt-0 font-medium ${
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
