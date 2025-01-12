import { ChevronDown, ChevronRight } from "lucide-react";
import { useState } from "react";

export default function CollapsibleList({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  const [isOpen, setIsOpen] = useState(true);

  const toggleCollapse = () => {
    setIsOpen(!isOpen);
  };
  return (
    <div className="flex flex-col gap-4 w-full rounded-lg p-2">
      <div
        className="flex items-center justify-start cursor-pointer w-fit text-muted-foreground"
        onClick={toggleCollapse}
      >
        {isOpen ? <ChevronDown /> : <ChevronRight />}
        <h2 className="text-xl font-semibold">{title}</h2>

      </div>

      {isOpen && (
        <div className="px-1 flex items-center gap-4 pb-4  overflow-x-auto whitespace-nowrap md:flex-wrap md:overflow-x-hidden w-full">
         {children}
        </div>
      )}
    </div>
  );
}
